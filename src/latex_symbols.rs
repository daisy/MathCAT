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
    Ok(root)
}

pub(crate) fn is_limit_name(name: &str) -> bool {
    matches!(
        name,
        "lim" | "liminf" | "limsup" | "max" | "min" | "sup" | "inf"
    )
}

const FUNC_NAMES: &[&str] = &[
    "arccsch", "arcsech", "arccoth", "arcsinh", "arccosh", "arctanh", "arcsin", "arccos",
    "arctan", "arccsc", "arcsec", "arccot", "liminf", "limsup", "sinh", "cosh", "tanh",
    "csch", "sech", "coth", "sin", "cos", "tan", "csc", "sec", "cot", "deg", "det", "dim",
    "exp", "gcd", "hom", "inf", "ker", "lg", "lim", "ln", "log", "max", "min", "Pr", "sup",
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
