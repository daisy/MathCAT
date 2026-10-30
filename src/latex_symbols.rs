//! LaTeX command → print character map.
//!
//! Built from `Rules/Braille/LaTeX/unicode.yaml` and `unicode-full.yaml`.
//! Every template MathCAT can emit as one token is accepted (`\alpha`, `\pm`,
//! `\mathbb Z`, `~p`, `<=`, `\not=`). Matching is longest-prefix, and a
//! backslash command that ends in a letter does not match inside a longer
//! command (`\in` does not consume `\infty`).

#![allow(clippy::needless_return)]

use crate::errors::Result;
use crate::parser::symbols;
use crate::shim_filesystem::read_to_string_shim;
use anyhow::Context;
use std::cell::RefCell;
use std::collections::HashMap;

#[derive(Clone)]
pub(crate) struct Out {
    pub text: String,
    pub ident: bool,
}

struct Node {
    children: HashMap<char, Node>,
    out: Option<Out>,
}

impl Node {
    fn new() -> Self {
        Self {
            children: HashMap::new(),
            out: None,
        }
    }

    fn insert(&mut self, key: &str, out: Out, overwrite: bool) {
        let mut node = self;
        for ch in key.chars() {
            node = node.children.entry(ch).or_insert_with(Node::new);
        }
        match &node.out {
            Some(prev) if !overwrite && rank(&out.text) < rank(&prev.text) => {}
            _ => node.out = Some(out),
        }
    }

    /// Insert only when this key is not already a token.
    fn insert_missing(&mut self, key: &str, out: Out) {
        let mut node = self;
        for ch in key.chars() {
            node = node.children.entry(ch).or_insert_with(Node::new);
        }
        if node.out.is_none() {
            node.out = Some(out);
        }
    }

    /// Longest key that is a complete token at the start of `input`.
    fn longest<'a>(&'a self, input: &str) -> Option<(&'a Out, usize)> {
        let mut node = self;
        let mut best: Option<(&Out, usize)> = None;
        let mut bytes = 0usize;
        let mut saw_backslash = false;
        let mut chars = input.chars().peekable();
        while let Some(ch) = chars.next() {
            if ch == '\\' {
                saw_backslash = true;
            }
            match node.children.get(&ch) {
                Some(next) => {
                    bytes += ch.len_utf8();
                    node = next;
                    if let Some(out) = node.out.as_ref() &&
                        token_boundary(saw_backslash, ch, chars.peek().copied())
                    {
                        best = Some((out, bytes));
                    }
                }
                None => break,
            }
        }
        best
    }
}

/// `\in` must not win inside `\infty`. `~a` and `<=` are not backslash commands.
fn token_boundary(saw_backslash: bool, last: char, next: Option<char>) -> bool {
    if saw_backslash && last.is_ascii_alphabetic() {
        return next.is_none_or(|c| !c.is_ascii_alphabetic());
    }
    true
}

fn rank(s: &str) -> i32 {
    let mut chars = s.chars();
    let Some(c) = chars.next() else {
        return -1;
    };
    if chars.next().is_some() {
        return 1;
    }
    let o = c as u32;
    if (0x1D400..=0x1D7FF).contains(&o) {
        return 2;
    }
    if c.is_alphabetic() || c == '∞' || c == '△' {
        return 6;
    }
    4
}

fn is_ident_char(c: char) -> bool {
    c.is_alphabetic() || c == '∞' || c == '△'
}

fn out_for(text: &str) -> Out {
    let mut chars = text.chars();
    let ident = matches!((chars.next(), chars.next()), (Some(c), None) if is_ident_char(c));
    Out {
        text: text.to_string(),
        ident,
    }
}

thread_local! {
    static SYMBOLS: RefCell<Option<std::result::Result<Node, String>>> = const { RefCell::new(None) };
}

pub fn ensure_latex_symbols_loaded() -> Result<()> {
    SYMBOLS.with(|cell| symbols::ensure(cell, load_symbols, "LaTeX symbol table"))
}

pub fn clear_latex_symbol_cache() {
    SYMBOLS.with(symbols::clear);
}

pub(crate) fn match_prefix(input: &str) -> Option<(Out, usize)> {
    let _ = ensure_latex_symbols_loaded();
    SYMBOLS.with(|cell| {
        symbols::with_ok(cell, |trie| trie.longest(input).map(|(out, n)| (out.clone(), n))).flatten()
    })
}

fn load_symbols() -> Result<Node> {
    let mut root = Node::new();
    let rules = symbols::rules_dir()?;
    for (rel, overwrite) in [
        ("Braille/LaTeX/unicode-full.yaml", false),
        ("Braille/LaTeX/unicode.yaml", true),
    ] {
        let path = rules.join(rel);
        let text = read_to_string_shim(&path)
            .with_context(|| format!("cannot read '{}'", path.display()))?;
        for (ch, template) in symbols::collect_templates(&text, None, |_, _, _| String::new())? {
            let Some(key) = command_token(&template) else {
                continue;
            };
            root.insert(&key, out_for(&ch), overwrite);
        }
    }
    // Function names the rules emit as `\sin` / `sin`, not as unicode.yaml characters.
    for name in FUNC_NAMES {
        let ident = !is_limit_name(name);
        let key = format!("\\{name}");
        root.insert(
            &key,
            Out {
                text: (*name).to_string(),
                ident,
            },
            false,
        );
    }
    // Reverse-only spellings. A key that YAML already owns is left alone, so the
    // forward template (for example `\not=` for ≠) does not change.
    for (key, text) in ALIASES {
        root.insert_missing(key, out_for(text));
    }
    Ok(root)
}

pub(crate) fn is_limit_name(name: &str) -> bool {
    matches!(
        name,
        "lim" | "liminf" | "limsup"
            | "injlim" | "projlim"
            | "varinjlim" | "varprojlim"
            | "varliminf" | "varlimsup"
            | "max" | "min" | "sup" | "inf"
    )
}

const FUNC_NAMES: &[&str] = &[
    "arccsch", "arcsech", "arccoth", "arcsinh", "arccosh", "arctanh", "arcsin", "arccos",
    "arctan", "arccsc", "arcsec", "arccot", "varinjlim", "varprojlim", "varliminf",
    "varlimsup", "injlim", "projlim", "liminf", "limsup", "sinh", "cosh", "tanh", "csch",
    "sech", "coth", "sin", "cos", "tan", "csc", "sec", "cot", "deg", "det", "dim", "exp",
    "gcd", "hom", "inf", "ker", "lg", "lim", "ln", "log", "max", "min", "Pr", "sup", "arg",
];

/// Command spellings from the Microsoft 365 LaTeX set whose character MathCAT
/// already emits under another name, plus `\jmath` (U+0237), which has no key.
const ALIASES: &[(&str, &str)] = &[
    (r"\ne", "\u{2260}"),
    (r"\neq", "\u{2260}"),
    (r"\neg", "\u{00AC}"),
    (r"\bot", "\u{22A5}"),
    (r"\prime", "\u{2032}"),
    (r"\emptyset", "\u{2205}"),
    (r"\omicron", "\u{03BF}"),
    (r"\varGamma", "\u{0393}"),
    (r"\varDelta", "\u{0394}"),
    (r"\varTheta", "\u{0398}"),
    (r"\varLambda", "\u{039B}"),
    (r"\varXi", "\u{039E}"),
    (r"\varPi", "\u{03A0}"),
    (r"\varSigma", "\u{03A3}"),
    (r"\varUpsilon", "\u{03A5}"),
    (r"\varPhi", "\u{03A6}"),
    (r"\varPsi", "\u{03A8}"),
    (r"\varOmega", "\u{03A9}"),
    (r"\diamond", "\u{22C4}"),
    (r"\Diamond", "\u{25C7}"),
    (r"\lozenge", "\u{25CA}"),
    (r"\blacklozenge", "\u{29EB}"),
    (r"\bigcirc", "\u{25EF}"),
    (r"\dagger", "\u{2020}"),
    (r"\ddagger", "\u{2021}"),
    (r"\hbar", "\u{210F}"),
    (r"\circlearrowleft", "\u{21BA}"),
    (r"\circlearrowright", "\u{21BB}"),
    (r"\dashleftarrow", "\u{21E0}"),
    (r"\dashrightarrow", "\u{21E2}"),
    // Forward spelling of → is `\to`. These commented names still parse.
    (r"\rightarrow", "\u{2192}"),
    (r"\rarr", "\u{2192}"),
    (r"\ra", "\u{2192}"),
    // Forward spelling of ⟹ is `\implies`.
    (r"\Longrightarrow", "\u{27F9}"),
    (r"\iddots", "\u{22F0}"),
    (r"\pvint", "\u{2A0D}"),
    (r"\dots", "\u{2026}"),
    (r"\dotsb", "\u{22EF}"),
    (r"\dotsm", "\u{22EF}"),
    (r"\dotsc", "\u{2026}"),
    (r"\dotsi", "\u{2026}"),
    (r"\dotso", "\u{2026}"),
    (r"\nleq", "\u{2270}"),
    (r"\ngeq", "\u{2271}"),
    (r"\nsim", "\u{2241}"),
    (r"\ncong", "\u{2247}"),
    (r"\ntriangleleft", "\u{22EA}"),
    (r"\ntriangleright", "\u{22EB}"),
    (r"\ntrianglelefteq", "\u{22EC}"),
    (r"\ntrianglerighteq", "\u{22ED}"),
    (r"\doteqdot", "\u{2251}"),
    (r"\quad", "\u{2003}"),
    (r"\qquad", "\u{2003}\u{2003}"),
    (r"\enspace", "\u{2002}"),
    (r"\medspace", "\u{205F}"),
    (r"\thickspace", "\u{2004}"),
    (r"\jmath", "\u{0237}"),
    (r"\lbrace", "{"),
    (r"\rbrace", "}"),
    (r"\lbrack", "["),
    (r"\rbrack", "]"),
    (r"\lvert", "|"),
    (r"\rvert", "|"),
    (r"\vert", "|"),
    (r"\lVert", "\u{2016}"),
    (r"\rVert", "\u{2016}"),
    (r"\|", "\u{2016}"),
    // Commented spellings of a character that already has an active template.
    (r"\allequal", "\u{224C}"),
    (r"\Angle", "\u{299C}"),
    (r"\approxnotequal", "\u{2246}"),
    (r"\arrowwaveleft", "\u{219C}"),
    (r"\arrowwaveright", "\u{219D}"),
    (r"\Box", "\u{25A1}"),
    (r"\clockoint", "\u{2A0F}"),
    (r"\clwintegral", "\u{2231}"),
    (r"\Complex", "\u{2102}"),
    (r"\dblarrowupdown", "\u{21C5}"),
    (r"\dh", "\u{00F0}"),
    (r"\DownArrowBar", "\u{2913}"),
    (r"\DownArrowUpArrow", "\u{21F5}"),
    (r"\DownLeftRightVector", "\u{2950}"),
    (r"\DownLeftTeeVector", "\u{295E}"),
    (r"\DownLeftVectorBar", "\u{2956}"),
    (r"\DownRightTeeVector", "\u{295F}"),
    (r"\DownRightVectorBar", "\u{2957}"),
    (r"\downslopeellipsis", "\u{22F1}"),
    (r"\Elolarr", "\u{2940}"),
    (r"\ElOr", "\u{2A56}"),
    (r"\Elorarr", "\u{2941}"),
    (r"\Elroang", "\u{2986}"),
    (r"\Elxsqcup", "\u{2A06}"),
    (r"\Elxuplus", "\u{2A04}"),
    (r"\ElzAnd", "\u{2A53}"),
    (r"\ElzCint", "\u{2A0D}"),
    (r"\Elzcirfb", "\u{25D2}"),
    (r"\Elzcirfl", "\u{25D0}"),
    (r"\Elzcirfr", "\u{25D1}"),
    (r"\Elzddfnc", "\u{2999}"),
    (r"\Elzdefas", "\u{29CB}"),
    (r"\Elzdlcorn", "\u{23A3}"),
    (r"\Elzdshfnc", "\u{2506}"),
    (r"\ElzInf", "\u{2A07}"),
    (r"\ElzLap", "\u{29CA}"),
    (r"\Elzlpargt", "\u{29A0}"),
    (r"\Elzminhat", "\u{2A5F}"),
    (r"\ElzOr", "\u{2A54}"),
    (r"\Elzrarrx", "\u{2947}"),
    (r"\ElzRlarr", "\u{2942}"),
    (r"\ElzrLarr", "\u{2944}"),
    (r"\Elzrvbull", "\u{25D8}"),
    (r"\Elzsqfl", "\u{25E7}"),
    (r"\Elzsqfr", "\u{25E8}"),
    (r"\Elzsqfse", "\u{25EA}"),
    (r"\Elzsqspne", "\u{22E5}"),
    (r"\ElzSup", "\u{2A08}"),
    (r"\Elztdcol", "\u{2AF6}"),
    (r"\Elztfnc", "\u{2980}"),
    (r"\ElzThr", "\u{2A05}"),
    (r"\Elzvrecto", "\u{25AF}"),
    (r"\Equal", "\u{2A75}"),
    (r"\geq", "\u{2265}"),
    (r"\hermitconjmatrix", "\u{22B9}"),
    (r"\homothetic", "\u{223B}"),
    (r"\i", "\u{0131}"),
    (r"\image", "\u{22B7}"),
    (r"\land", "\u{2227}"),
    (r"\lazysinv", "\u{223E}"),
    (r"\LeftDownTeeVector", "\u{2961}"),
    (r"\LeftDownVectorBar", "\u{2959}"),
    (r"\LeftRightVector", "\u{294E}"),
    (r"\LeftTeeVector", "\u{295A}"),
    (r"\LeftTriangleBar", "\u{29CF}"),
    (r"\LeftUpDownVector", "\u{2951}"),
    (r"\LeftUpTeeVector", "\u{2960}"),
    (r"\LeftUpVectorBar", "\u{2958}"),
    (r"\LeftVectorBar", "\u{2952}"),
    (r"\leq", "\u{2264}"),
    (r"\lor", "\u{2228}"),
    (r"\NestedGreaterGreater", "\u{2AA2}"),
    (r"\NestedLessLess", "\u{2AA1}"),
    (r"\notgreaterless", "\u{2279}"),
    (r"\notlessgreater", "\u{2278}"),
    (r"\original", "\u{22B6}"),
    (r"\owns", "\u{220B}"),
    (r"\perspcorrespond", "\u{2A5E}"),
    (r"\precedesnotsimilar", "\u{22E8}"),
    (r"\r", "\u{030A}"),
    (r"\Reals", "\u{211D}"),
    (r"\ReverseUpEquilibrium", "\u{296F}"),
    (r"\rightanglearc", "\u{22BE}"),
    (r"\RightDownTeeVector", "\u{295D}"),
    (r"\RightDownVectorBar", "\u{2955}"),
    (r"\RightTeeVector", "\u{295B}"),
    (r"\RightTriangleBar", "\u{29D0}"),
    (r"\RightUpDownVector", "\u{294F}"),
    (r"\RightUpTeeVector", "\u{295C}"),
    (r"\RightUpVectorBar", "\u{2954}"),
    (r"\RightVectorBar", "\u{2953}"),
    (r"\RoundImplies", "\u{2970}"),
    (r"\RuleDelayed", "\u{29F4}"),
    (r"\smallsetminus", "\u{2216}"),
    (r"\sqrint", "\u{2A16}"),
    (r"\starequal", "\u{225B}"),
    (r"\textasciibreve", "\u{02D8}"),
    (r"\textcopyright", "\u{00A9}"),
    (r"\textdagger", "\u{2020}"),
    (r"\textdaggerdbl", "\u{2021}"),
    (r"\textparagraph", "\u{00B6}"),
    (r"\textregistered", "\u{00AE}"),
    (r"\textsection", "\u{00A7}"),
    (r"\textsterling", "\u{00A3}"),
    (r"\textvisiblespace", "\u{2423}"),
    (r"\textyen", "\u{00A5}"),
    (r"\tildetrpl", "\u{224B}"),
    (r"\truestate", "\u{22A7}"),
    (r"\UpArrowBar", "\u{2912}"),
    (r"\UpEquilibrium", "\u{296E}"),
    (r"\upslopeellipsis", "\u{22F0}"),
    (r"\v", "\u{030C}"),
    (r"\varperspcorrespond", "\u{2306}"),
    (r"\venus", "\u{2640}"),
    (r"\verymuchgreater", "\u{22D9}"),
    (r"\verymuchless", "\u{22D8}"),
];

/// A template MathCAT emits as one token. Grouped composites (`\acute{\eta}`) are
/// not tokens; the pieces are parsed on their own. Trailing `𝐖` is only a space.
fn command_token(template: &str) -> Option<String> {
    let spaced = template.replace(['𝐖', '𝐰'], " ");
    let trimmed = spaced.trim();
    if trimmed.is_empty() || trimmed.contains('{') || trimmed.contains('}') {
        return None;
    }
    if trimmed.chars().all(|c| c == '.') {
        return None;
    }
    let mut chars = trimmed.chars();
    if let (Some(c), None) = (chars.next(), chars.next()) &&
        (c.is_ascii_alphanumeric() || (c.is_ascii_punctuation() && c != '<' && c != '>' && c != '='))
    {
        return None;
    }
    Some(trimmed.to_string())
}
