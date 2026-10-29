//! ASCIIMath symbol table.
//!
//! Built-in names follow the AsciiMath grammar (including TeX alternatives such
//! as `succ` / `succeq`). Additional names are loaded from
//! `Rules/Braille/ASCIIMath/unicode.yaml`, `unicode-full.yaml`, and the LaTeX
//! unicode files. Matching is always longest-prefix: `succeq` wins over `succ`,
//! and `\partialmeetcontraction` wins over `\partial`.
//!
//! The `𝐖` / `𝐰` markers in those YAML templates are word-boundary flags from
//! the braille rules, not part of the name a user types.

#![allow(clippy::needless_return)]

use crate::errors::Result;
use crate::parser::symbols;
use crate::shim_filesystem::read_to_string_shim;
use anyhow::Context;
use std::cell::RefCell;
use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum Kind {
    Const,
    Unary,
    UnaryUnderOver,
    Binary,
    Infix,
    Left,
    Right,
    LeftRight,
    Text,
    UnderOver,
    Space,
    Definition,
}

#[derive(Debug, Clone)]
pub(crate) struct Sym {
    /// Canonical behavior name (`bar`, not the `overline` alias).
    pub name: String,
    pub output: String,
    pub kind: Kind,
    /// Emit `<mi>` rather than `<mo>` for constant symbols.
    pub ident: bool,
    pub func: bool,
    pub accent: bool,
    pub variant: Option<&'static str>,
    pub fences: Option<(&'static str, &'static str)>,
    pub invisible: bool,
    /// Built-in grammar tokens keep their kind when YAML adds the same spelling.
    locked: bool,
    rank: i32,
}

#[derive(Clone)]
struct Trie {
    children: HashMap<char, Trie>,
    sym: Option<Sym>,
}

impl Trie {
    fn new() -> Self {
        Self {
            children: HashMap::new(),
            sym: None,
        }
    }

    fn insert(&mut self, key: &str, sym: Sym, overwrite_output: bool) {
        let mut node = self;
        for ch in key.chars() {
            node = node.children.entry(ch).or_insert_with(Trie::new);
        }
        match &mut node.sym {
            Some(existing) if existing.locked && !overwrite_output => {}
            Some(existing) if existing.locked && overwrite_output => {
                if sym.rank >= existing.rank && sym.output != existing.output {
                    existing.output = sym.output;
                    existing.rank = sym.rank;
                }
            }
            Some(existing) => {
                if sym.rank >= existing.rank {
                    let locked = existing.locked;
                    let kind = existing.kind;
                    let name = existing.name.clone();
                    *existing = sym;
                    if locked {
                        existing.locked = true;
                        existing.kind = kind;
                        existing.name = name;
                    }
                }
            }
            None => node.sym = Some(sym),
        }
    }

    fn longest<'a>(&'a self, input: &str) -> Option<(&'a Sym, usize)> {
        let mut node = self;
        let mut best: Option<(&'a Sym, usize)> = None;
        let mut bytes = 0usize;
        for ch in input.chars() {
            match node.children.get(&ch) {
                Some(next) => {
                    bytes += ch.len_utf8();
                    node = next;
                    if let Some(sym) = node.sym.as_ref() {
                        best = Some((sym, bytes));
                    }
                }
                None => break,
            }
        }
        best
    }
}

thread_local! {
    static SYMBOLS: RefCell<Option<std::result::Result<Trie, String>>> = const { RefCell::new(None) };
}

pub fn ensure_asciimath_symbols_loaded() -> Result<()> {
    SYMBOLS.with(|cell| symbols::ensure(cell, load_symbols, "ASCIIMath symbol table"))
}

/// Longest symbol match at the start of `input`.
pub(crate) fn match_prefix(input: &str) -> Option<(Sym, usize)> {
    let _ = ensure_asciimath_symbols_loaded();
    SYMBOLS.with(|cell| {
        symbols::with_ok(cell, |trie| {
            trie.longest(input).map(|(sym, n)| (sym.clone(), n))
        })
        .flatten()
    })
}

pub fn clear_asciimath_symbol_cache() {
    SYMBOLS.with(symbols::clear);
}

fn load_symbols() -> Result<Trie> {
    let mut trie = Trie::new();
    insert_builtins(&mut trie);

    let rules = symbols::rules_dir()?;
    // Weaker files first so the short unicode.yaml spelling wins ties.
    for (rel, overwrite) in [
        ("Braille/LaTeX/unicode-full.yaml", false),
        ("Braille/LaTeX/unicode.yaml", false),
        ("Braille/ASCIIMath/unicode-full.yaml", false),
        ("Braille/ASCIIMath/unicode.yaml", true),
    ] {
        let path = rules.join(rel);
        let text = read_to_string_shim(&path)
            .with_context(|| format!("cannot read '{}'", path.display()))?;
        for (ch, template) in collect_templates(&text)? {
            let Some(name) = token_name(&template) else {
                continue;
            };
            let rank = char_rank(&ch);
            let ident = is_ident_output(&ch);
            trie.insert(
                &name,
                Sym {
                    name: name.clone(),
                    output: ch,
                    kind: Kind::Const,
                    ident,
                    func: false,
                    accent: false,
                    variant: None,
                    fences: None,
                    invisible: false,
                    locked: false,
                    rank,
                },
                overwrite,
            );
        }
    }
    Ok(trie)
}

fn is_ident_output(s: &str) -> bool {
    let mut chars = s.chars();
    match (chars.next(), chars.next()) {
        (Some(c), None) => c.is_alphabetic(),
        _ => false,
    }
}

/// Prefer BMP operators and letters over Mathematical Alphanumeric lookalikes.
fn char_rank(s: &str) -> i32 {
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
    if c.is_alphabetic() {
        return 6;
    }
    4
}

/// Turn a YAML `t:` template into the token a user types.
///
/// `𝐖` / `𝐰` are word-boundary markers. Composite templates (spaces, grouped
/// arguments) are not single names. Apostrophe runs are derivative marks, not
/// prime-character tokens. Single ASCII letters and digits stay with the
/// identifier / number lexer.
fn token_name(template: &str) -> Option<String> {
    if template.is_empty() {
        return None;
    }
    let spaced = template.replace(['𝐖', '𝐰'], " ");
    let keep_backslash_space = spaced.contains("\\ ");
    let trimmed = spaced.trim();
    if trimmed.is_empty() {
        return None;
    }
    let name = if keep_backslash_space && trimmed == "\\" {
        "\\ ".to_string()
    } else if trimmed.chars().any(char::is_whitespace) {
        return None;
    } else {
        trimmed.to_string()
    };
    if (name.contains('{') || name.contains('}')) && name != "\\{" && name != "\\}" {
        return None;
    }
    if name.chars().all(|c| c == '\'') {
        return None;
    }
    let mut chars = name.chars();
    if let (Some(c), None) = (chars.next(), chars.next())
        && c.is_ascii_alphanumeric()
    {
        return None;
    }
    Some(name)
}

fn collect_templates(text: &str) -> Result<Vec<(String, String)>> {
    // Multi-character keys (`𝐀-𝐙`) are font alphabets, not symbol names.
    symbols::collect_templates(text, None, |_, _, _| String::new())
}

struct B {
    n: &'static str,
    o: &'static str,
    k: Kind,
    ident: bool,
    func: bool,
    acc: bool,
    tex: Option<&'static str>,
    var: Option<&'static str>,
    fence: Option<(&'static str, &'static str)>,
    invis: bool,
}

impl B {
    fn new(n: &'static str, o: &'static str, k: Kind) -> Self {
        Self {
            n,
            o,
            k,
            ident: false,
            func: false,
            acc: false,
            tex: None,
            var: None,
            fence: None,
            invis: false,
        }
    }
    fn ident(mut self) -> Self {
        self.ident = true;
        self
    }
    fn func(mut self) -> Self {
        self.func = true;
        self.ident = true;
        self
    }
    fn acc(mut self) -> Self {
        self.acc = true;
        self
    }
    fn tex(mut self, t: &'static str) -> Self {
        self.tex = Some(t);
        self
    }
    fn var(mut self, v: &'static str) -> Self {
        self.var = Some(v);
        self
    }
    fn fence(mut self, open: &'static str, close: &'static str) -> Self {
        self.fence = Some((open, close));
        self
    }
    fn invis(mut self) -> Self {
        self.invis = true;
        self
    }
}

fn insert_builtins(trie: &mut Trie) {
    let mut syms = Vec::new();
    let c = |n, o| B::new(n, o, Kind::Const);
    let ci = |n, o| B::new(n, o, Kind::Const).ident();

    // Greek. phi/varphi follow MathCAT's ASCIIMath unicode.yaml (φ / ϕ).
    for (n, o) in [
        ("alpha", "α"),
        ("beta", "β"),
        ("chi", "χ"),
        ("delta", "δ"),
        ("epsilon", "ε"),
        ("varepsilon", "ɛ"),
        ("eta", "η"),
        ("gamma", "γ"),
        ("iota", "ι"),
        ("kappa", "κ"),
        ("lambda", "λ"),
        ("lamda", "λ"),
        ("mu", "μ"),
        ("nu", "ν"),
        ("omega", "ω"),
        ("phi", "φ"),
        ("varphi", "ϕ"),
        ("pi", "π"),
        ("psi", "ψ"),
        ("rho", "ρ"),
        ("sigma", "σ"),
        ("tau", "τ"),
        ("theta", "θ"),
        ("vartheta", "ϑ"),
        ("upsilon", "υ"),
        ("xi", "ξ"),
        ("zeta", "ζ"),
    ] {
        syms.push(ci(n, o));
    }
    syms.push(ci("epsi", "ε").tex("epsilon"));
    for (n, o) in [
        ("Delta", "Δ"),
        ("Gamma", "Γ"),
        ("Lambda", "Λ"),
        ("Lamda", "Λ"),
        ("Omega", "Ω"),
        ("Phi", "Φ"),
        ("Pi", "Π"),
        ("Psi", "Ψ"),
        ("Sigma", "Σ"),
        ("Theta", "Θ"),
        ("Xi", "Ξ"),
    ] {
        syms.push(ci(n, o));
    }

    syms.extend([
        c("*", "⋅").tex("cdot"),
        c("**", "∗").tex("ast"),
        c("***", "⋆").tex("star"),
        c("//", "/"),
        c("\\\\", "\\").tex("backslash"),
        c("setminus", "\\"),
        c("xx", "×").tex("times"),
        c("|><", "⋉").tex("ltimes"),
        c("><|", "⋊").tex("rtimes"),
        c("|><|", "⋈").tex("bowtie"),
        c("-:", "÷").tex("div"),
        B::new("divide", "-:", Kind::Definition),
        c("@", "∘").tex("circ"),
        c("o+", "⊕").tex("oplus"),
        c("o-", "⊖").tex("ominus"),
        c("ox", "⊗").tex("otimes"),
        c("o.", "⊙").tex("odot"),
        B::new("sum", "∑", Kind::UnderOver),
        B::new("prod", "∏", Kind::UnderOver),
        c("^^", "∧").tex("wedge"),
        B::new("^^^", "⋀", Kind::UnderOver).tex("bigwedge"),
        c("vv", "∨").tex("vee"),
        B::new("vvv", "⋁", Kind::UnderOver).tex("bigvee"),
        c("nn", "∩").tex("cap"),
        B::new("nnn", "⋂", Kind::UnderOver).tex("bigcap"),
        c("uu", "∪").tex("cup"),
        B::new("uuu", "⋃", Kind::UnderOver).tex("bigcup"),
        c("dag", "†").tex("dagger"),
        c("ddag", "‡").tex("ddagger"),
        c("!=", "≠").tex("ne"),
        c(":=", ":="),
        c("lt", "<"),
        c("<=", "≤").tex("le"),
        c("lt=", "≤").tex("leq"),
        c("gt", ">"),
        c("mlt", "≪").tex("ll"),
        c(">=", "≥").tex("ge"),
        c("gt=", "≥").tex("geq"),
        c("mgt", "≫").tex("gg"),
        c("-<", "≺").tex("prec"),
        c("-lt", "≺"),
        c(">-", "≻").tex("succ"),
        c("-<=", "⪯").tex("preceq"),
        c(">-=", "⪰").tex("succeq"),
        c("in", "∈"),
        c("!in", "∉").tex("notin"),
        c("sub", "⊂").tex("subset"),
        c("!sub", "⊄"),
        B::new("notsubset", "!sub", Kind::Definition),
        c("sup", "⊃").tex("supset"),
        c("!sup", "⊅"),
        B::new("notsupset", "!sup", Kind::Definition),
        c("sube", "⊆").tex("subseteq"),
        c("!sube", "⊈"),
        B::new("notsubseteq", "!sube", Kind::Definition),
        c("supe", "⊇").tex("supseteq"),
        c("!supe", "⊉"),
        B::new("notsupseteq", "!supe", Kind::Definition),
        c("-=", "≡").tex("equiv"),
        c("!-=", "≢"),
        B::new("notequiv", "!-=", Kind::Definition),
        c("~=", "≅").tex("cong"),
        c("~~", "≈").tex("approx"),
        c("~", "∼").tex("sim"),
        c("prop", "∝").tex("propto"),
        B::new("and", "and", Kind::Space),
        B::new("or", "or", Kind::Space),
        c("not", "¬").tex("neg"),
        c("=>", "⇒").tex("implies"),
        B::new("if", "if", Kind::Space),
        c("<=>", "⇔").tex("iff"),
        c("AA", "∀").tex("forall"),
        c("EE", "∃").tex("exists"),
        c("_|_", "⊥").tex("bot"),
        c("TT", "⊤").tex("top"),
        c("|--", "⊢").tex("vdash"),
        c("|==", "⊨").tex("models"),
        B::new("(", "(", Kind::Left),
        B::new(")", ")", Kind::Right),
        B::new("[", "[", Kind::Left),
        B::new("]", "]", Kind::Right),
        B::new("{", "{", Kind::Left),
        B::new("}", "}", Kind::Right),
        B::new("|", "|", Kind::LeftRight),
        c(":|:", "|"),
        B::new("|:", "|", Kind::Left),
        B::new(":|", "|", Kind::Right),
        B::new("(:", "\u{2329}", Kind::Left).tex("langle"),
        B::new(":)", "\u{232A}", Kind::Right).tex("rangle"),
        B::new("<<", "\u{2329}", Kind::Left),
        B::new(">>", "\u{232A}", Kind::Right),
        B::new("{:", "{:", Kind::Left).invis(),
        B::new(":}", ":}", Kind::Right).invis(),
        c("int", "∫"),
        B::new("dx", "{:d x:}", Kind::Definition),
        B::new("dy", "{:d y:}", Kind::Definition),
        B::new("dz", "{:d z:}", Kind::Definition),
        B::new("dt", "{:d t:}", Kind::Definition),
        c("oint", "∮"),
        c("del", "∂").tex("partial"),
        c("grad", "∇").tex("nabla"),
        c("+-", "±").tex("pm"),
        c("-+", "∓").tex("mp"),
        c("O/", "∅").tex("emptyset"),
        c("oo", "∞").tex("infty"),
        c("aleph", "ℵ"),
        c("...", "...").tex("ldots"),
        c(":.", "∴").tex("therefore"),
        c(":'", "∵").tex("because"),
        c("/_", "∠").tex("angle"),
        c("/_\\", "△").tex("triangle"),
        c("\\ ", "\u{00A0}"),
        c("frown", "⌢"),
        c("quad", "\u{00A0}"),
        c("qquad", "\u{00A0}"),
        c("cdots", "⋯"),
        c("vdots", "⋮"),
        c("ddots", "⋱"),
        c("diamond", "⋄"),
        c("square", "□"),
        c("|__", "⌊").tex("lfloor"),
        c("__|", "⌋").tex("rfloor"),
        c("|~", "⌈").tex("lceiling"),
        c("~|", "⌉").tex("rceiling"),
        ci("CC", "ℂ"),
        ci("NN", "ℕ"),
        ci("QQ", "ℚ"),
        ci("RR", "ℝ"),
        ci("ZZ", "ℤ"),
        ci("lg", "lg"),
        B::new("f", "f", Kind::Unary).func(),
        B::new("g", "g", Kind::Unary).func(),
        c("hbar", "ℏ"),
        B::new("lim", "lim", Kind::UnderOver),
        B::new("Lim", "Lim", Kind::UnderOver),
        c(",", ","),
        c("+", "+"),
        // Canonicalize rewrites U+2212 back to ASCII hyphen.
        c("-", "−"),
        c("=", "="),
    ]);

    for name in [
        "sin", "cos", "tan", "sinh", "cosh", "tanh", "cot", "sec", "csc", "arcsin", "arccos",
        "arctan", "arcsec", "arccsc", "arccot", "coth", "sech", "csch", "exp", "log", "ln", "det",
        "gcd", "lcm", "Sin", "Cos", "Tan", "Arcsin", "Arccos", "Arctan", "Sinh", "Cosh", "Tanh",
        "Cot", "Sec", "Csc", "Log", "Ln",
    ] {
        syms.push(B::new(name, name, Kind::Unary).func());
    }
    syms.extend([
        B::new("abs", "abs", Kind::Unary).fence("|", "|"),
        B::new("norm", "norm", Kind::Unary).fence("\u{2225}", "\u{2225}"),
        B::new("floor", "floor", Kind::Unary).fence("⌊", "⌋"),
        B::new("ceil", "ceil", Kind::Unary).fence("⌈", "⌉"),
        ci("dim", "dim"),
        ci("mod", "mod"),
        ci("lub", "lub"),
        ci("glb", "glb"),
        B::new("min", "min", Kind::UnderOver),
        B::new("max", "max", Kind::UnderOver),
    ]);

    for (n, o, tex) in [
        ("uarr", "↑", "uparrow"),
        ("darr", "↓", "downarrow"),
        ("rarr", "→", "rightarrow"),
        ("->", "→", "to"),
        (">->", "↣", "rightarrowtail"),
        ("->>", "↠", "twoheadrightarrow"),
        (">->>", "⤖", "twoheadrightarrowtail"),
        ("|->", "↦", "mapsto"),
        ("larr", "←", "leftarrow"),
        ("harr", "↔", "leftrightarrow"),
        ("rArr", "⇒", "Rightarrow"),
        ("lArr", "⇐", "Leftarrow"),
        ("dArr", "⇓", "Downarrow"),
        ("hArr", "⇔", "Leftrightarrow"),
    ] {
        syms.push(c(n, o).tex(tex));
    }
    syms.push(c("rightleftharpoons", "⇌"));

    syms.extend([
        B::new("sqrt", "sqrt", Kind::Unary),
        B::new("root", "root", Kind::Binary),
        B::new("frac", "/", Kind::Binary),
        B::new("/", "/", Kind::Infix),
        B::new("stackrel", "stackrel", Kind::Binary),
        B::new("overset", "stackrel", Kind::Binary),
        B::new("underset", "stackrel", Kind::Binary),
        B::new("_", "_", Kind::Infix),
        B::new("^", "^", Kind::Infix),
        B::new("hat", "^", Kind::Unary).acc(),
        B::new("bar", "¯", Kind::Unary).acc().tex("overline"),
        B::new("vec", "→", Kind::Unary).acc(),
        B::new("dot", ".", Kind::Unary).acc(),
        B::new("ddot", "..", Kind::Unary).acc(),
        B::new("overarc", "⏜", Kind::Unary).acc().tex("overparen"),
        B::new("ul", "̲", Kind::Unary).acc().tex("underline"),
        B::new("ubrace", "⏟", Kind::UnaryUnderOver).acc().tex("underbrace"),
        B::new("obrace", "⏞", Kind::UnaryUnderOver).acc().tex("overbrace"),
        B::new("text", "text", Kind::Text),
        B::new("mbox", "mbox", Kind::Text),
        B::new("\"", "\"", Kind::Text),
        B::new("color", "color", Kind::Binary),
        B::new("id", "id", Kind::Binary),
        B::new("class", "class", Kind::Binary),
        B::new("cancel", "cancel", Kind::Unary),
        B::new("bb", "bb", Kind::Unary).var("bold").tex("mathbf"),
        B::new("sf", "sf", Kind::Unary).var("sans-serif").tex("mathsf"),
        B::new("sfit", "sfit", Kind::Unary).var("sans-serif-italic"),
        B::new("bbsf", "bbsf", Kind::Unary).var("bold-sans-serif"),
        B::new("bbb", "bbb", Kind::Unary).var("double-struck").tex("mathbb"),
        B::new("cc", "cc", Kind::Unary).var("script").tex("mathcal"),
        B::new("bbcc", "bbcc", Kind::Unary).var("bold-script"),
        B::new("tt", "tt", Kind::Unary).var("monospace").tex("mathtt"),
        B::new("fr", "fr", Kind::Unary).var("fraktur").tex("mathfrak"),
        B::new("bbfr", "bbfr", Kind::Unary).var("bold-fraktur"),
        B::new("bbit", "bbit", Kind::Unary).var("bold-italic"),
        B::new("bii", "bii", Kind::Unary).var("bold-italic"),
        B::new("bbsfit", "bbsfit", Kind::Unary).var("sans-serif-bold-italic"),
        B::new("bold", "bold", Kind::Unary).var("bold"),
        B::new("italic", "italic", Kind::Unary).var("italic").tex("mathit"),
    ]);

    for b in syms {
        let sym = Sym {
            name: b.n.to_string(),
            output: b.o.to_string(),
            kind: b.k,
            ident: b.ident,
            func: b.func,
            accent: b.acc,
            variant: b.var,
            fences: b.fence,
            invisible: b.invis,
            locked: true,
            rank: 100,
        };
        trie.insert(b.n, sym.clone(), false);
        if let Some(tex) = b.tex
            && tex.chars().all(|c| c.is_ascii_alphabetic())
            && tex != b.n
        {
            trie.insert(tex, sym, false);
        }
    }
}
