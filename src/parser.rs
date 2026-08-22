//! Braille technical material → MathML parser.
//!
//! Currently implements UEB (Unified English Braille) based on the ICEB
//! *Guidelines for Technical Material*, covering the core constructs used in
//! MathCAT's MathML→UEB tests: numbers, letters, operators, brackets,
//! numeric/general fractions, superscripts/subscripts, radicals, arrows,
//! over/underscripts (GTM 12), chemistry patterns, and linearized matrices
//! (GTM 15 / `mtable` on one line for refreshable displays).
//!
//! Parsing is done in two phases:
//! 1. A mode-aware lexer turns Unicode braille cells into [`Token`]s
//!    (symbols are loaded lazily from UEB `unicode.yaml` and `unicode-full.yaml`).
//! 2. A recursive-descent parser builds an [`Expr`] AST, then emits MathML.

#![allow(clippy::needless_return)]
#![allow(non_snake_case)]

use crate::errors::{bail, Result};

// --- public API -------------------------------------------------------------

/// Parse a Unicode braille string into a MathML fragment wrapped in `<math>`.
///
/// `braille_code` selects the braille system (e.g. `"UEB"`). Additional codes
/// will be added over time.
pub fn Braille_to_MathML(braille: &str, braille_code: &str) -> Result<String> {
    let expr = Braille_to_Expr(braille, braille_code)?;
    Ok(format!("<math>{}</math>", expr.to_mathml()))
}

/// Parse Unicode braille into an expression AST (useful for tests / tooling).
///
/// `braille_code` selects the braille system (e.g. `"UEB"`).
pub fn Braille_to_Expr(braille: &str, braille_code: &str) -> Result<Expr> {
    match braille_code {
        "UEB" => {
            let tokens = tokenize(braille)?;
            parse_tokens(&tokens)
        }
        other => bail!("Braille_to_Expr: unsupported braille code '{other}'"),
    }
}

// --- AST --------------------------------------------------------------------

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Expr {
    Number(String),
    /// Identifier text (Latin or Greek letter(s), possibly multi-letter).
    Identifier(String),
    /// Operator / punctuation character(s) for `<mo>`.
    Operator(String),
    /// Horizontal list (implicit `<mrow>` when length ≠ 1).
    Row(Vec<Expr>),
    /// Braille grouping `⠣…⠜` (not printed as fences). Scripts/accents bind to the
    /// whole group rather than peeling the last row child.
    Group(Box<Expr>),
    Sup(Box<Expr>, Box<Expr>),
    Sub(Box<Expr>, Box<Expr>),
    SubSup(Box<Expr>, Box<Expr>, Box<Expr>),
    Frac(Box<Expr>, Box<Expr>),
    /// Binomial / line-less fraction: `<mfrac linethickness="0">`
    BinomFrac(Box<Expr>, Box<Expr>),
    Sqrt(Box<Expr>),
    Root(Box<Expr>, Box<Expr>), // index, base
    Fenced {
        open: String,
        /// `None` when the close fence was not typed yet → emit `<mtext>&#xFFFD;</mtext>`.
        close: Option<String>,
        body: Box<Expr>,
    },
    /// Linearized matrix/determinant or equation-line `mtable`.
    /// `open`/`close` are `Some` for GTM-15 fenced matrices; both `None` for bare `<mtable>`.
    Table {
        open: Option<String>,
        /// `None` when the close fence was not typed yet (fenced) or for bare tables.
        close: Option<String>,
        rows: Vec<Vec<Expr>>,
    },
    /// `<mover>` — accent / limit above
    Over(Box<Expr>, Box<Expr>),
    /// `<munder>` — accent / limit below
    Under(Box<Expr>, Box<Expr>),
    /// `<munderover>`
    UnderOver(Box<Expr>, Box<Expr>, Box<Expr>),
    /// Left-displaced scripts (approximate `<mmultiscripts>`)
    Prescripts {
        base: Box<Expr>,
        sub: Option<Box<Expr>>,
        sup: Option<Box<Expr>>,
    },
    /// Post-scripts as `<mmultiscripts>` (chemistry ions / oxidation states).
    MultiScripts {
        base: Box<Expr>,
        sub: Option<Box<Expr>>,
        sup: Option<Box<Expr>>,
    },
    Space,
    Text(String),
    /// Required item/closer not yet typed — `<mtext>&#xFFFD;</mtext>`.
    Missing,
}

impl Expr {
    fn row(mut parts: Vec<Expr>) -> Expr {
        parts.retain(|e| !matches!(e, Expr::Row(v) if v.is_empty()));
        // Bare ° after a number is degree-as-superscript (ICEB BANA 5a), not a
        // free-standing operator that canonicalize would merge with C/F into ℃/℉.
        parts = fold_degree_superscripts(parts);
        parts = fold_double_tildes(parts);
        parts = flatten_juxtaposed_letter_rows(parts);
        match parts.len() {
            0 => Expr::Row(vec![]),
            1 => parts.pop().unwrap(),
            _ => Expr::Row(parts),
        }
    }

    fn is_empty_row(&self) -> bool {
        matches!(self, Expr::Row(v) if v.is_empty())
    }
}

/// Emit row children as MathML, turning braille spaces into `<mo>&#xA0;</mo>` unless they
/// match UEB `default`/`mo` `AddSpaces` (spaces auto-inserted around comparison operators,
/// or around all operators when `UseSpacesAroundAllOperators` is true).
fn row_parts_to_mathml(parts: &[Expr]) -> String {
    let mut s = String::new();
    for (i, p) in parts.iter().enumerate() {
        if matches!(p, Expr::Space) {
            if space_is_auto_added_by_mo_rule(parts, i) {
                continue;
            }
            s.push_str("<mo>&#xA0;</mo>");
        } else {
            s.push_str(&p.to_mathml());
        }
    }
    s
}

/// True when this braille space would have been inserted by UEB_Rules `mo`/`default` AddSpaces
/// (also under/over/underover whose base is a comparison operator).
fn space_is_auto_added_by_mo_rule(parts: &[Expr], space_idx: usize) -> bool {
    let neighbor_is_auto = |e: Option<&Expr>| match e {
        Some(Expr::Operator(op)) => operator_gets_ueb_auto_spaces(op),
        Some(Expr::Under(base, _) | Expr::Over(base, _) | Expr::UnderOver(base, _, _)) => {
            match base.as_ref() {
                Expr::Operator(op) => operator_gets_ueb_auto_spaces(op),
                _ => false,
            }
        }
        _ => false,
    };
    neighbor_is_auto(space_idx.checked_sub(1).and_then(|j| parts.get(j)))
        || neighbor_is_auto(parts.get(space_idx + 1))
}

/// Mirrors UEB_Rules `AddSpaces` / `braille::is_operator_that_adds_whitespace`.
fn operator_gets_ueb_auto_spaces(op: &str) -> bool {
    // Ratio colon is explicitly excluded from AddSpaces in UEB_Rules.
    if op == "∶" {
        return false;
    }
    if use_spaces_around_all_operators() {
        return true;
    }
    is_braille_comparison_operator(op)
}

fn use_spaces_around_all_operators() -> bool {
    use crate::prefs::PreferenceManager;
    PreferenceManager::get()
        .borrow()
        .pref_to_string("UseSpacesAroundAllOperators")
        == "true"
}

fn is_braille_comparison_operator(op: &str) -> bool {
    use crate::definitions::BRAILLE_DEFINITIONS;
    BRAILLE_DEFINITIONS.with(|definitions| {
        let definitions = definitions.borrow();
        if let Some(set) = definitions.get_hashset("NemethComparisonOperators") {
            return set.contains(op);
        }
        if let Some(set) = definitions.get_hashset("ComparisonOperators") {
            return set.contains(op);
        }
        // Defs may be unloaded in isolated parser unit tests — cover common equals/relations.
        matches!(
            op,
            "=" | "≠"
                | "≈"
                | "<"
                | ">"
                | "≤"
                | "≥"
                | "∈"
                | "∋"
                | "→"
                | "←"
                | "↔"
                | "≡"
                | "≢"
                | "≦"
                | "≧"
                | "≲"
                | "≳"
        )
    })
}

impl Expr {
    pub fn to_mathml(&self) -> String {
        match self {
            Expr::Number(n) => format!("<mn>{}</mn>", xml_escape(n)),
            Expr::Identifier(s) => format!("<mi>{}</mi>", xml_escape(s)),
            Expr::Operator(s) => format!("<mo>{}</mo>", xml_escape(s)),
            Expr::Text(s) => format!("<mtext>{}</mtext>", xml_escape(s)),
            Expr::Missing => "<mtext>&#xFFFD;</mtext>".to_string(),
            // Non-automatic braille spaces → explicit nbsp (see `row_parts_to_mathml`).
            Expr::Space => "<mo>&#xA0;</mo>".to_string(),
            Expr::Row(parts) => {
                format!("<mrow>{}</mrow>", row_parts_to_mathml(parts))
            }
            // Grouping indicators are not MathML fences — just emit the body.
            Expr::Group(body) => body.to_mathml(),
            Expr::Sup(base, exp) => {
                format!("<msup>{}{}</msup>", base.to_mathml(), exp.to_mathml())
            }
            Expr::Sub(base, sub) => {
                format!("<msub>{}{}</msub>", base.to_mathml(), sub.to_mathml())
            }
            Expr::SubSup(base, sub, sup) => {
                format!(
                    "<msubsup>{}{}{}</msubsup>",
                    base.to_mathml(),
                    sub.to_mathml(),
                    sup.to_mathml()
                )
            }
            Expr::Frac(num, den) => {
                format!("<mfrac>{}{}</mfrac>", num.to_mathml(), den.to_mathml())
            }
            Expr::BinomFrac(num, den) => {
                format!(
                    "<mfrac linethickness=\"0\">{}{}</mfrac>",
                    num.to_mathml(),
                    den.to_mathml()
                )
            }
            Expr::Sqrt(body) => format!("<msqrt>{}</msqrt>", body.to_mathml()),
            Expr::Root(index, base) => {
                format!("<mroot>{}{}</mroot>", base.to_mathml(), index.to_mathml())
            }
            Expr::Fenced { open, close, body } => {
                let open_xml = format!("<mo>{}</mo>", xml_escape(open));
                let body_xml = body.to_mathml();
                match close {
                    Some(c) => format!(
                        "<mrow>{open_xml}{body_xml}<mo>{}</mo></mrow>",
                        xml_escape(c)
                    ),
                    // Incomplete fence (e.g. mid-typing): open (+ body) only — no � placeholder.
                    None if body.is_empty_row() || body_xml.is_empty() => open_xml,
                    None => format!("<mrow>{open_xml}{body_xml}</mrow>"),
                }
            }
            Expr::Table { open, close, rows } => {
                let mut s = String::new();
                let fenced = open.is_some();
                if fenced {
                    s.push_str("<mrow>");
                    if let Some(o) = open {
                        s.push_str(&format!("<mo>{}</mo>", xml_escape(o)));
                    }
                }
                s.push_str("<mtable>");
                for row in rows {
                    s.push_str("<mtr>");
                    for cell in row {
                        s.push_str("<mtd>");
                        s.push_str(&cell.to_mathml());
                        s.push_str("</mtd>");
                    }
                    s.push_str("</mtr>");
                }
                s.push_str("</mtable>");
                if fenced {
                    if let Some(c) = close {
                        s.push_str(&format!("<mo>{}</mo>", xml_escape(c)));
                    }
                    s.push_str("</mrow>");
                }
                s
            }
            Expr::Over(base, over) => {
                format!("<mover>{}{}</mover>", base.to_mathml(), over.to_mathml())
            }
            Expr::Under(base, under) => {
                format!("<munder>{}{}</munder>", base.to_mathml(), under.to_mathml())
            }
            Expr::UnderOver(base, under, over) => {
                format!(
                    "<munderover>{}{}{}</munderover>",
                    base.to_mathml(),
                    under.to_mathml(),
                    over.to_mathml()
                )
            }
            Expr::Prescripts { base, sub, sup } => {
                let mut s = format!("<mmultiscripts>{}", base.to_mathml());
                s.push_str("<mprescripts/>");
                s.push_str(&sub.as_ref().map(|e| e.to_mathml()).unwrap_or_else(|| "<none/>".into()));
                s.push_str(&sup.as_ref().map(|e| e.to_mathml()).unwrap_or_else(|| "<none/>".into()));
                s.push_str("</mmultiscripts>");
                s
            }
            Expr::MultiScripts { base, sub, sup } => {
                let mut s = format!("<mmultiscripts>{}", base.to_mathml());
                s.push_str(&sub.as_ref().map(|e| e.to_mathml()).unwrap_or_else(|| "<none/>".into()));
                s.push_str(&sup.as_ref().map(|e| e.to_mathml()).unwrap_or_else(|| "<none/>".into()));
                s.push_str("</mmultiscripts>");
                s
            }
        }
    }
}

fn xml_escape(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    for ch in s.chars() {
        match ch {
            '&' => out.push_str("&amp;"),
            '<' => out.push_str("&lt;"),
            '>' => out.push_str("&gt;"),
            '"' => out.push_str("&quot;"),
            // Prefer numeric character references for non-ASCII ops so tests are stable.
            c if c as u32 > 127 => out.push_str(&format!("&#x{:X};", c as u32)),
            c => out.push(c),
        }
    }
    out
}

// --- tokens -----------------------------------------------------------------

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Token {
    Number(String),
    /// One letter (Latin or Greek). Consecutive letters may be merged later.
    Letter {
        ch: char,
        capital: bool,
    },
    Op(String),
    Space,
    /// `(` `[` `{` `|` or braille-grouping open
    Open(char),
    /// `)` `]` `}` `|` or braille-grouping close
    Close(char),
    /// Enlarged / multi-line fence (dot-6 + grouping): GTM 15 matrices on one line.
    /// Open and close use the print character (`(` `)` `[` `]` `{` `}` `|`).
    EnlargedFence(char),
    /// UEB_Rules multi-row `mtr` marker: dots 456 + blank (`⠸⠀`).
    TableRowStart,
    /// Equation-line table bound when the `mtable` is not alone: dot-6 + space (`⠠⠀`).
    TableBound,
    /// Vertical bar `|` (⠸⠳) — open/close share one cell.
    VertBar,
    /// Level-change up (superscript)
    LevelUp,
    /// Level-change down (subscript)
    LevelDown,
    /// Directly-above indicator (⠨⠔) — followed by an item (GTM 7.9 / 12)
    Above,
    /// Directly-below indicator (⠨⠢)
    Below,
    /// Simple overscript modifier applied to the preceding item (bar, dot, arrow, …)
    SimpleOver(String),
    /// Simple underscript modifier applied to the preceding item
    SimpleUnder(String),
    FracOpen,
    FracLine,
    FracClose,
    SqrtOpen,
    SqrtClose,
    /// Unrecognized or incomplete braille cell(s), preserved as literary `<mtext>`
    /// (soft recovery for mid-typing prefixes such as a lone `⠐`).
    BrailleText(String),
    /// Grade-1 / capital indicators are consumed by the lexer; these mark passage boundaries if needed.
    Grade1PassageEnd,
}

// --- lexer ------------------------------------------------------------------

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum CapMode {
    Off,
    /// Next letter only
    Symbol,
    /// Until space / terminator
    Word,
    /// Until passage terminator
    Passage,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Grade1Mode {
    Off,
    /// Next symbols that need G1 (symbol indicator consumed for one “symbol use”)
    Symbol,
    Word,
    Passage,
}

/// Pending symbol-level typeform (GTM 2.7) applied to the next letter/number run.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum PendingTypeform {
    None,
    /// Fraktur symbol indicator ⠈⠆ — map Latin letters to fraktur print forms when possible.
    Fraktur,
    /// Bold symbol indicator ⠘⠂ — next number/letter only.
    BoldSymbol,
    /// Bold word indicator ⠘⠆ — until bold terminator ⠘⠄ or space.
    BoldWord,
}

struct Lexer<'a> {
    rest: &'a str,
    numeric: bool,
    /// Grade 1 set by numeric indicator until space/hyphen/dash
    numeric_grade1: bool,
    grade1: Grade1Mode,
    capital: CapMode,
    pending_typeform: PendingTypeform,
    /// Open radical nesting — only then is ⠬ a radical closer (else omission).
    radical_depth: i32,
    tokens: Vec<Token>,
}

impl<'a> Lexer<'a> {
    fn new(input: &'a str) -> Self {
        Self {
            rest: input,
            numeric: false,
            numeric_grade1: false,
            grade1: Grade1Mode::Off,
            capital: CapMode::Off,
            pending_typeform: PendingTypeform::None,
            radical_depth: 0,
            tokens: Vec::new(),
        }
    }

    fn in_grade1(&self) -> bool {
        self.numeric_grade1 || !matches!(self.grade1, Grade1Mode::Off)
    }

    fn eat(&mut self, prefix: &str) -> bool {
        if self.rest.starts_with(prefix) {
            self.rest = &self.rest[prefix.len()..];
            true
        } else {
            false
        }
    }

    fn peek_char(&self) -> Option<char> {
        self.rest.chars().next()
    }

    fn end_numeric_modes_for_space(&mut self) {
        self.numeric = false;
        self.numeric_grade1 = false;
        if matches!(self.grade1, Grade1Mode::Word) {
            self.grade1 = Grade1Mode::Off;
        }
        if matches!(self.capital, CapMode::Word) {
            self.capital = CapMode::Off;
        }
        if matches!(self.pending_typeform, PendingTypeform::BoldWord) {
            self.pending_typeform = PendingTypeform::None;
        }
    }

    fn consume_cap_for_letter(&mut self) -> bool {
        match self.capital {
            CapMode::Off => false,
            CapMode::Symbol => {
                self.capital = CapMode::Off;
                true
            }
            CapMode::Word | CapMode::Passage => true,
        }
    }

    fn consume_grade1_symbol_use(&mut self) {
        if matches!(self.grade1, Grade1Mode::Symbol) {
            self.grade1 = Grade1Mode::Off;
        }
    }

    fn lex(mut self) -> Result<Vec<Token>> {
        while !self.rest.is_empty() {
            if self.try_indicators()? {
                continue;
            }
            if self.try_structure()? {
                continue;
            }
            if self.try_number()? {
                continue;
            }
            if self.try_operator()? {
                continue;
            }
            if self.try_letter()? {
                continue;
            }
            if self.try_contraction()? {
                continue;
            }
            // Soft recovery: keep unknown cells as braille mtext (e.g. trailing prefix ⠐).
            let ch = self.peek_char().unwrap();
            self.rest = &self.rest[ch.len_utf8()..];
            self.numeric = false;
            self.tokens.push(Token::BrailleText(ch.to_string()));
        }
        Ok(self.tokens)
    }

    /// Grade 1, capital, numeric indicators.
    fn try_indicators(&mut self) -> Result<bool> {
        // Grade 1 passage terminator ⠰⠄
        if self.eat("⠰⠄") {
            self.grade1 = Grade1Mode::Off;
            self.tokens.push(Token::Grade1PassageEnd);
            return Ok(true);
        }
        // Capital passage terminator ⠠⠄
        if self.eat("⠠⠄") {
            self.capital = CapMode::Off;
            return Ok(true);
        }
        // Under-bar / under-arrow are capital-dot patterns used as modifiers (GTM 12),
        // not capitalisation indicators — handled in try_structure / try_modifier.
        if self.rest.starts_with("⠠⠱") || self.rest.starts_with("⠠⠘⠱") {
            return Ok(false);
        }
        // Equation-line table bound `⠠⠀` — before treating ⠠ as capital.
        if self.rest.starts_with("⠠⠀") {
            return Ok(false);
        }
        // Enlarged (multi-line) grouping for linearized matrices (GTM 15) — must be
        // recognized before treating ⠠ as a capital indicator.
        if self.try_enlarged_fence()? {
            return Ok(true);
        }
        // Grade 1 passage / word / symbol (longest first)
        if self.eat("⠰⠰⠰") {
            self.grade1 = Grade1Mode::Passage;
            return Ok(true);
        }
        if self.eat("⠰⠰") {
            // Don't downgrade an active passage to word mode
            if !matches!(self.grade1, Grade1Mode::Passage) {
                self.grade1 = Grade1Mode::Word;
            }
            return Ok(true);
        }
        if self.eat("⠰") {
            // Nested ⠰ inside passage/word must not exit that mode (GTM: indicators
            // are often repeated before items like ⠻ inside a G1 passage).
            if matches!(self.grade1, Grade1Mode::Off) {
                self.grade1 = Grade1Mode::Symbol;
            }
            return Ok(true);
        }
        // Capital passage / word / symbol
        if self.eat("⠠⠠⠠") {
            self.capital = CapMode::Passage;
            return Ok(true);
        }
        if self.eat("⠠⠠") {
            self.capital = CapMode::Word;
            return Ok(true);
        }
        if self.eat("⠠") {
            self.capital = CapMode::Symbol;
            return Ok(true);
        }
        // Angstrom: ⠘⠫ + a → Å (UEB chemical/physics unit)
        if self.rest.starts_with("⠘⠫") {
            let after = self.rest.chars().nth(2);
            if after.and_then(braille_cell_to_latin) == Some('a') {
                self.eat("⠘⠫");
                self.eat_char(); // a
                let capital = self.consume_cap_for_letter();
                let ch = if capital { 'Å' } else { 'Å' };
                self.tokens.push(Token::Letter {
                    ch,
                    capital: true,
                });
                return Ok(true);
            }
            // Unknown decorated shape — skip prefix
            self.eat("⠘⠫");
            return Ok(true);
        }
        // Numeric indicator
        if self.eat("⠼") {
            self.numeric = true;
            self.numeric_grade1 = true;
            return Ok(true);
        }
        // Bold / typeform symbol/word/passage indicators (GTM 2.7).
        if self.eat("⠈⠆") {
            self.pending_typeform = PendingTypeform::Fraktur;
            return Ok(true);
        }
        if self.eat("⠘⠆") {
            self.pending_typeform = PendingTypeform::BoldWord;
            return Ok(true);
        }
        if self.eat("⠘⠂") {
            self.pending_typeform = PendingTypeform::BoldSymbol;
            return Ok(true);
        }
        if self.eat("⠘⠄") {
            self.pending_typeform = PendingTypeform::None;
            return Ok(true);
        }
        if self.eat("⠨⠂")
            || self.eat("⠨⠆")
            || self.eat("⠸⠂")
            || self.eat("⠸⠆")
            || self.eat("⠈⠂")
            || self.eat("⠈⠄")
        {
            return Ok(true);
        }
        Ok(false)
    }

    fn try_structure(&mut self) -> Result<bool> {
        // Equation-line mtable bound (dot-6 + space) when table is not alone.
        if self.eat("⠠⠀") {
            self.end_numeric_modes_for_space();
            self.tokens.push(Token::TableBound);
            return Ok(true);
        }
        // Multi-row mtable row start (UEB_Rules `mtr` when >1 row): before bare space.
        if self.eat("⠸⠀") {
            self.end_numeric_modes_for_space();
            self.tokens.push(Token::TableRowStart);
            return Ok(true);
        }
        // Space
        if self.eat("⠀") {
            self.end_numeric_modes_for_space();
            self.tokens.push(Token::Space);
            return Ok(true);
        }

        // Simple over/under modifiers (GTM 12) — may appear with or without G1
        if self.try_modifier()? {
            return Ok(true);
        }

        // Arrows (GTM 13): arrow indicator ⠳ …
        if self.try_arrow()? {
            return Ok(true);
        }

        // Shape indicator ⠫ … (minimal: consume shape + following number/letter run)
        if self.try_shape()? {
            return Ok(true);
        }

        // Level changes — require grade 1 meaning (G1 mode or explicit G1 already consumed)
        if self.in_grade1() {
            if self.eat("⠔") {
                self.consume_grade1_symbol_use();
                self.numeric = false;
                self.tokens.push(Token::LevelUp);
                return Ok(true);
            }
            if self.eat("⠢") {
                self.consume_grade1_symbol_use();
                self.numeric = false;
                self.tokens.push(Token::LevelDown);
                return Ok(true);
            }
            if self.eat("⠨⠔") {
                self.consume_grade1_symbol_use();
                self.tokens.push(Token::Above);
                return Ok(true);
            }
            if self.eat("⠨⠢") {
                self.consume_grade1_symbol_use();
                self.tokens.push(Token::Below);
                return Ok(true);
            }
            // General fraction indicators (G1)
            if self.eat("⠷") {
                self.consume_grade1_symbol_use();
                self.numeric = false;
                self.tokens.push(Token::FracOpen);
                return Ok(true);
            }
            if self.eat("⠨⠌") {
                self.consume_grade1_symbol_use();
                self.numeric = false;
                self.tokens.push(Token::FracLine);
                return Ok(true);
            }
            if self.eat("⠾") {
                self.consume_grade1_symbol_use();
                self.numeric = false;
                self.tokens.push(Token::FracClose);
                return Ok(true);
            }
            // Radical
            if self.eat("⠩") {
                self.consume_grade1_symbol_use();
                self.numeric = false;
                self.radical_depth += 1;
                self.tokens.push(Token::SqrtOpen);
                return Ok(true);
            }
            if self.radical_depth > 0 && self.eat("⠬") {
                self.consume_grade1_symbol_use();
                self.numeric = false;
                // Empty radical is written ⠩⠬⠬ (GTM 3.6.7): a filler ⠬ before the closer.
                // Consume the filler without emitting tokens so the body stays empty.
                if self.rest.starts_with('⠬')
                    && matches!(self.tokens.last(), Some(Token::SqrtOpen))
                {
                    return Ok(true);
                }
                self.radical_depth -= 1;
                self.tokens.push(Token::SqrtClose);
                return Ok(true);
            }
            // Omission blank (GTM 3.6) — bare ⠬ outside a radical.
            // Forward UEB regenerates ⠬ from two consecutive wide nbsp `<mo>`s.
            if self.eat("⠬") {
                self.consume_grade1_symbol_use();
                self.numeric = false;
                self.tokens.push(Token::Op("\u{A0}".into()));
                self.tokens.push(Token::Op("\u{A0}".into()));
                return Ok(true);
            }
            // Braille grouping ⠣ ⠜
            if self.eat("⠣") {
                self.consume_grade1_symbol_use();
                self.numeric = false;
                self.tokens.push(Token::Open('⟦'));
                return Ok(true);
            }
            if self.eat("⠜") {
                self.consume_grade1_symbol_use();
                self.numeric = false;
                self.tokens.push(Token::Close('⟧'));
                return Ok(true);
            }
            // Omission / visible blank "?" (GTM 3.6) — bare ⠦ in G1.
            // Do not steal multi-cell symbols that start with ⠦ (e.g. ≟ = ⠦⠻⠐⠶).
            if self.rest.starts_with('⠦') {
                let longer_symbol = crate::ueb_symbols::match_ueb_symbol(self.rest)
                    .is_some_and(|(b, _)| b.starts_with('⠦') && b != "⠦");
                if !longer_symbol && self.eat("⠦") {
                    self.consume_grade1_symbol_use();
                    self.numeric = false;
                    self.tokens.push(Token::Op("?".into()));
                    return Ok(true);
                }
            }
            // Simple / binomial fraction line ⠻ (GTM 6.4 / 14.3) — distinct from ⠨⠌
            if self.eat("⠻") {
                self.consume_grade1_symbol_use();
                self.numeric = false;
                self.tokens.push(Token::FracLine);
                return Ok(true);
            }
            // Integral (G1)
            if self.eat("⠮") {
                self.consume_grade1_symbol_use();
                self.numeric = false;
                self.tokens.push(Token::Op("∫".into()));
                return Ok(true);
            }
        }

        // Print brackets
        if self.eat("⠐⠣") {
            self.numeric = false;
            self.tokens.push(Token::Open('('));
            return Ok(true);
        }
        if self.eat("⠐⠜") {
            self.numeric = false;
            self.tokens.push(Token::Close(')'));
            return Ok(true);
        }
        if self.eat("⠨⠣") {
            self.numeric = false;
            self.tokens.push(Token::Open('['));
            return Ok(true);
        }
        if self.eat("⠨⠜") {
            self.numeric = false;
            self.tokens.push(Token::Close(']'));
            return Ok(true);
        }
        if self.eat("⠸⠣") {
            self.numeric = false;
            self.tokens.push(Token::Open('{'));
            return Ok(true);
        }
        if self.eat("⠸⠜") {
            self.numeric = false;
            self.tokens.push(Token::Close('}'));
            return Ok(true);
        }
        // Vertical bar (absolute value / determinant fence) — same cell for open and close.
        // Spaced form ⠀⠸⠳⠀ is the “such that” comparison (unicode.yaml), not a fence.
        if self.eat("⠸⠳") {
            self.numeric = false;
            let spaced = matches!(self.tokens.last(), Some(Token::Space))
                && self.rest.starts_with('⠀');
            if spaced {
                self.tokens.push(Token::Op("|".into()));
            } else {
                self.tokens.push(Token::VertBar);
            }
            return Ok(true);
        }

        // Omission outside explicit G1 (numeric G1 may already have ended)
        if self.radical_depth == 0 && self.eat("⠬") {
            self.numeric = false;
            self.tokens.push(Token::Op("\u{A0}".into()));
            self.tokens.push(Token::Op("\u{A0}".into()));
            return Ok(true);
        }

        Ok(false)
    }

    /// GTM 15 enlarged fences: dot 6 + normal grouping (one-line matrices).
    fn try_enlarged_fence(&mut self) -> Result<bool> {
        let pairs: &[(&str, char)] = &[
            ("⠠⠐⠣", '('),
            ("⠠⠐⠜", ')'),
            ("⠠⠨⠣", '['),
            ("⠠⠨⠜", ']'),
            ("⠠⠸⠣", '{'),
            ("⠠⠸⠜", '}'),
            ("⠠⠸⠳", '|'),
        ];
        for &(braille, ch) in pairs {
            if self.eat(braille) {
                self.numeric = false;
                self.numeric_grade1 = false;
                self.tokens.push(Token::EnlargedFence(ch));
                return Ok(true);
            }
        }
        Ok(false)
    }

    /// GTM 12 simple modifiers placed after an item.
    fn try_modifier(&mut self) -> Result<bool> {
        // Longer under forms first (include leading ⠠)
        if self.eat("⠠⠘⠱") {
            self.consume_grade1_symbol_use();
            self.numeric = false;
            self.tokens.push(Token::SimpleUnder("→".into()));
            return Ok(true);
        }
        if self.eat("⠠⠱") {
            self.consume_grade1_symbol_use();
            self.numeric = false;
            self.tokens.push(Token::SimpleUnder("_".into()));
            return Ok(true);
        }
        // Over forms — typically need G1 meaning of ⠱ / compounds
        if self.in_grade1() || self.rest.starts_with("⠘") || self.rest.starts_with("⠐⠱") || self.rest.starts_with("⠸⠱") || self.rest.starts_with("⠨⠸⠱") {
            if self.eat("⠨⠸⠱") {
                self.consume_grade1_symbol_use();
                self.numeric = false;
                self.tokens.push(Token::SimpleOver("⌒".into()));
                return Ok(true);
            }
            if self.eat("⠘⠱") {
                self.consume_grade1_symbol_use();
                self.numeric = false;
                self.tokens.push(Token::SimpleOver("→".into()));
                return Ok(true);
            }
            if self.eat("⠘⠲") {
                self.consume_grade1_symbol_use();
                self.numeric = false;
                self.tokens.push(Token::SimpleOver("˙".into()));
                return Ok(true);
            }
            // hat — yaml sometimes has trailing space in mapping; braille is ⠐⠱
            if self.eat("⠐⠱") {
                self.consume_grade1_symbol_use();
                self.numeric = false;
                self.tokens.push(Token::SimpleOver("^".into()));
                return Ok(true);
            }
            if self.eat("⠸⠱") {
                self.consume_grade1_symbol_use();
                self.numeric = false;
                self.tokens.push(Token::SimpleOver("~".into()));
                return Ok(true);
            }
            if self.in_grade1() && self.eat("⠱") {
                self.consume_grade1_symbol_use();
                self.numeric = false;
                self.tokens.push(Token::SimpleOver("_".into()));
                return Ok(true);
            }
        }
        Ok(false)
    }

    /// GTM 13 arrows. Indicator ⠳ then tip/shaft encoding.
    fn try_arrow(&mut self) -> Result<bool> {
        if !self.rest.starts_with('⠳') {
            return Ok(false);
        }
        // Need G1 for ⠳ (grade-2 contraction otherwise); accept when in G1 or after symbol G1.
        if !self.in_grade1() {
            return Ok(false);
        }
        let arrows: &[(&str, &str)] = &[
            ("⠳⠺⠶⠗⠕", "⇔"),
            ("⠳⠺⠗⠕", "↔"),
            ("⠳⠺⠗⠬", "↕"),
            ("⠳⠒⠒⠒⠕", "⟶"),
            ("⠳⠒⠒⠒⠪", "⟵"),
            ("⠳⠶⠕", "⇒"),
            ("⠳⠶⠪", "⇐"),
            ("⠳⠶⠬", "⇑"),
            ("⠳⠶⠩", "⇓"),
            ("⠳⠪", "←"),
            ("⠳⠕", "→"),
            ("⠳⠬", "↑"),
            ("⠳⠩", "↓"),
            ("⠳⠱", "↖"),
            ("⠳⠎", "↗"),
            ("⠳⠣", "↘"),
            ("⠳⠜", "↙"),
        ];
        for &(braille, print) in arrows {
            if self.rest.starts_with(braille) {
                self.rest = &self.rest[braille.len()..];
                self.consume_grade1_symbol_use();
                self.numeric = false;
                self.tokens.push(Token::Op(print.into()));
                return Ok(true);
            }
        }
        // Bare ⠳ — treat as incomplete arrow start; consume and emit → as fallback
        if self.eat("⠳") {
            self.consume_grade1_symbol_use();
            self.numeric = false;
            self.tokens.push(Token::Op("→".into()));
            return Ok(true);
        }
        Ok(false)
    }

    fn try_shape(&mut self) -> Result<bool> {
        // Prefer full reverse-map match so typeformed shapes (⠨⠫⠿⠱ → ◍, ⠸⠫⠼⠉⠱ → ▲)
        // emit the Unicode characters forward UEB already uses.
        if let Some((braille, print)) = crate::ueb_symbols::match_ueb_symbol(self.rest) {
            if braille.contains('⠫') {
                let typeform_prefix = braille.starts_with('⠨')
                    || braille.starts_with('⠘')
                    || braille.starts_with('⠸')
                    || braille.starts_with('⠈');
                if !self.in_grade1() && !typeform_prefix && !braille.starts_with('⠰') {
                    // fall through to manual parse / G1 requirement
                } else {
                    self.rest = &self.rest[braille.len()..];
                    self.consume_grade1_symbol_use();
                    self.numeric = false;
                    self.tokens.push(Token::Op(print));
                    return Ok(true);
                }
            }
        }

        // Optional typeform prefix before shape (e.g. ⠨⠫ for italicised shape)
        let typeform_prefixes = ["⠨", "⠘", "⠸", "⠈"];
        let mut prefix_len = 0;
        for p in typeform_prefixes {
            if self.rest.starts_with(p) && self.rest[p.len()..].starts_with('⠫') {
                prefix_len = p.len();
                break;
            }
        }
        let after_prefix = &self.rest[prefix_len..];
        if !after_prefix.starts_with('⠫') {
            return Ok(false);
        }
        // Shape needs G1, unless a typeform prefix makes the shape indicator unambiguous
        if !self.in_grade1() && prefix_len == 0 {
            return Ok(false);
        }
        let prefix = if prefix_len > 0 {
            let p = &self.rest[..prefix_len];
            self.rest = &self.rest[prefix_len..];
            p.to_string()
        } else {
            String::new()
        };
        self.eat("⠫");
        self.consume_grade1_symbol_use();
        self.numeric = false;
        // Optional following number (shape code), e.g. ⠼⠉ for triangle
        let mut code = String::new();
        if self.eat("⠼") {
            self.numeric = true;
            self.numeric_grade1 = true;
            while let Some(ch) = self.peek_char() {
                if let Some(d) = braille_cell_to_digit(ch) {
                    self.eat_char();
                    code.push(d);
                } else {
                    break;
                }
            }
            self.numeric = false;
        } else if self.eat("⠿") {
            code.push('●');
        }
        // Optional shape terminator ⠱
        let _ = self.eat("⠱");
        let print = match (prefix.as_str(), code.as_str()) {
            ("⠸", "3") => "▲",
            ("⠸", "4") => "■",
            ("⠨", "●") => "◍",
            ("⠨", "4") => "▧",
            (_, "3") => "△",
            (_, "4") => "□",
            (_, "●") => "○",
            _ => "◇",
        };
        self.tokens.push(Token::Op(print.into()));
        Ok(true)
    }

    fn try_number(&mut self) -> Result<bool> {
        self.lex_numeric_item()
    }

    fn lex_numeric_item(&mut self) -> Result<bool> {
        if !self.numeric {
            return Ok(false);
        }
        let start = self.rest;
        let mut num = String::new();
        let mut den: Option<String> = None;

        loop {
            let ch = match self.peek_char() {
                Some(c) => c,
                None => break,
            };
            if let Some(digit) = braille_cell_to_digit(ch) {
                self.eat_char();
                if let Some(ref mut d) = den {
                    d.push(digit);
                } else {
                    num.push(digit);
                }
                continue;
            }
            match ch {
                '⠲' => {
                    // Decimal point when more of the number follows (digit, or grouped
                    // repeating digits `0.⟨3⟩`). Trailing `⠲` after a number is a period.
                    let after = self.rest.chars().nth(1);
                    let is_decimal = match after {
                        Some(c) if braille_cell_to_digit(c).is_some() => true,
                        Some('⠣') => true, // braille grouping for following digits
                        _ => false,
                    };
                    if !is_decimal {
                        break;
                    }
                    self.eat_char();
                    if let Some(ref mut d) = den {
                        d.push('.');
                    } else {
                        num.push('.');
                    }
                }
                '⠂' => {
                    // thousands comma — but comma + space is literary list comma (end number)
                    let after = self.rest.chars().nth(1);
                    if after == Some('⠀') {
                        break;
                    }
                    self.eat_char();
                    if let Some(ref mut d) = den {
                        d.push(',');
                    } else {
                        num.push(',');
                    }
                }
                '⠌' if den.is_none() => {
                    // simple numeric fraction line
                    self.eat_char();
                    den = Some(String::new());
                }
                '⠐' => {
                    // numeric space if followed by a digit
                    let after = self.rest.chars().nth(1);
                    if after.and_then(braille_cell_to_digit).is_some() {
                        self.eat_char(); // ⠐
                        if let Some(ref mut d) = den {
                            d.push(' ');
                        } else {
                            num.push(' ');
                        }
                    } else {
                        break;
                    }
                }
                _ => break,
            }
        }

        if self.rest.as_ptr() == start.as_ptr() && self.rest.len() == start.len() {
            // numeric indicator with nothing following — leave indicator effect, no token
            return Ok(false);
        }
        if num.is_empty() && den.is_none() {
            return Ok(false);
        }

        let bold = matches!(
            self.pending_typeform,
            PendingTypeform::BoldSymbol | PendingTypeform::BoldWord
        );
        if matches!(self.pending_typeform, PendingTypeform::BoldSymbol) {
            self.pending_typeform = PendingTypeform::None;
        }

        match den {
            Some(d) => {
                let (num, d) = if bold {
                    (to_bold_digits(&num), to_bold_digits(&d))
                } else {
                    (num, d)
                };
                self.tokens.push(Token::Number(format!("FRAC:{num}/{d}")));
            }
            None => {
                let num = if bold { to_bold_digits(&num) } else { num };
                self.tokens.push(Token::Number(num));
            }
        }
        // Numeric mode continues until a non-numeric symbol; we've stopped at such a boundary.
        // Keep numeric=true only if we ended mid-number (we didn't). GTM: mode ends on other symbols.
        // After finishing a number token, if next isn't numeric-continuation, clear numeric.
        if !matches!(
            self.peek_char(),
            Some(c) if braille_cell_to_digit(c).is_some()
                || c == '⠲'
                || c == '⠂'
                || c == '⠌'
                || c == '⠐'
        ) {
            self.numeric = false;
        }
        Ok(true)
    }

    fn eat_char(&mut self) {
        if let Some(c) = self.peek_char() {
            self.rest = &self.rest[c.len_utf8()..];
        }
    }

    fn try_operator(&mut self) -> Result<bool> {
        // Full symbol table from Rules/Braille/UEB/unicode.yaml + unicode-full.yaml.
        if let Some((braille, print)) = crate::ueb_symbols::match_ueb_symbol(self.rest) {
            // Without grade 1, ⠔ / ⠢ are grade-2 contractions ("in" / "en"), not the
            // start of scripted unicode superscripts (¹ ² ³ from stripped `⠰⠔⠼…`).
            if !self.in_grade1() {
                let first = braille.chars().next();
                if matches!(first, Some('⠔' | '⠢')) {
                    return Ok(false);
                }
            }
            // Pending capital must apply via the letter lexer (⠠⠨⠎ → ∑/Σ, not σ + Cap left on).
            if !matches!(self.capital, CapMode::Off) && is_greek_letter_char(&print) {
                return Ok(false);
            }
            // ℃ / ℉ share the degree prefix ⠘⠚ + capital letter. Prefer degree + letter
            // so reverse matches `<msup>…<mo>°</mo></msup><mi>F</mi>` style MathML.
            if print == "℉" || print == "℃" {
                if self.eat("⠘⠚") {
                    self.numeric = false;
                    self.tokens.push(Token::Op("°".into()));
                    return Ok(true);
                }
            }
            // Double/triple prime as repeated ′ (forward often uses adjacent primes).
            if print == "″" || print == "‴" {
                self.rest = &self.rest[braille.len()..];
                self.numeric = false;
                let n = if print == "″" { 2 } else { 3 };
                for _ in 0..n {
                    self.tokens.push(Token::Op("′".into()));
                }
                return Ok(true);
            }
            self.rest = &self.rest[braille.len()..];
            self.numeric = false;
            // Hyphen (bare ⠤) ends numeric-initiated grade 1; minus ⠐⠤ does not (GTM 1.2.2).
            if braille == "⠤" {
                self.numeric_grade1 = false;
            }
            // Letters / blackboard / large ops → identifiers so scripts can attach
            if is_identifier_symbol(&print) {
                let ch = print.chars().next().unwrap();
                self.tokens.push(Token::Letter {
                    ch,
                    capital: ch.is_uppercase(),
                });
            } else {
                self.tokens.push(Token::Op(print));
            }
            return Ok(true);
        }
        Ok(false)
    }

    fn try_letter(&mut self) -> Result<bool> {
        let ch = match self.peek_char() {
            Some(c) => c,
            None => return Ok(false),
        };

        // Greek letter indicator ⠨ (optional capital ⠠ before the letter cell)
        if ch == '⠨' {
            let mut chars = self.rest.chars();
            chars.next(); // ⠨
            let mut capital = self.consume_cap_for_letter();
            let mut letter_cell = chars.next();
            if letter_cell == Some('⠠') {
                capital = true;
                letter_cell = chars.next();
                // consume ⠨ ⠠ letter
                self.eat_char();
                self.eat_char();
                self.eat_char();
            } else if let Some(cell) = letter_cell {
                if braille_cell_to_latin(cell).is_some() || braille_greek_only_cell(cell).is_some() {
                    self.eat_char(); // ⠨
                    self.eat_char(); // letter
                } else {
                    return Ok(false);
                }
            } else {
                return Ok(false);
            }
            let cell = letter_cell.unwrap();
            let base = braille_greek_only_cell(cell)
                .or_else(|| braille_cell_to_latin(cell))
                .unwrap();
            let greek = latin_to_greek(base, capital);
            // Cap+Greek s is the summation sign in UEB math (not variable Σ).
            let greek = if capital && greek == 'Σ' { '∑' } else { greek };
            self.numeric = false;
            self.tokens.push(Token::Letter {
                ch: greek,
                capital: false,
            });
            return Ok(true);
        }

        if let Some(latin) = braille_cell_to_latin(ch) {
            // In numeric mode, a-j cells are digits — already handled. If numeric still true
            // with a-j, they'd be digits. k-z can appear? Unusual. If numeric true and a-j,
            // try_number should have taken them. So if we see a-j here with numeric, treat as digit restart.
            if self.numeric {
                if braille_cell_to_digit(ch).is_some() {
                    return self.lex_numeric_item();
                }
            }
            // Letters a-j immediately after a number need G1 (numeric_grade1 still on).
            // If we get here without G1 for a-j after number, still accept as letter (tests use ⠰).
            self.eat_char();
            let capital = self.consume_cap_for_letter();
            let mut out = if capital {
                latin.to_ascii_uppercase()
            } else {
                latin
            };
            if self.pending_typeform == PendingTypeform::Fraktur {
                out = latin_to_fraktur(out);
                self.pending_typeform = PendingTypeform::None;
            }
            self.numeric = false;
            self.tokens.push(Token::Letter {
                ch: out,
                capital,
            });
            // Standing-alone G1 symbol was for this letter
            if matches!(self.grade1, Grade1Mode::Symbol) {
                // letter itself consumed the symbol-mode purpose
                self.grade1 = Grade1Mode::Off;
            }
            return Ok(true);
        }

        Ok(false)
    }

    /// Minimal Grade-2 contractions used in function names (sech, cosh, …).
    fn try_contraction(&mut self) -> Result<bool> {
        if self.in_grade1() {
            return Ok(false);
        }
        let ch = match self.peek_char() {
            Some(c) => c,
            None => return Ok(false),
        };
        // Only expand unambiguous single-cell groups common in UEB math function names
        let expansion = match ch {
            '⠡' => Some("ch"),
            '⠩' => Some("sh"),
            '⠹' => Some("th"),
            '⠻' => Some("er"),
            '⠜' => Some("ar"),
            '⠔' => Some("in"),
            '⠢' => Some("en"),
            '⠫' => Some("ed"),
            '⠯' => Some("and"),
            '⠆' => Some("bb"),
            '⠂' => Some("ea"),
            _ => None,
        };
        let Some(exp) = expansion else {
            return Ok(false);
        };
        self.eat_char();
        let capital_first = self.consume_cap_for_letter();
        for (i, c) in exp.chars().enumerate() {
            let ch = if i == 0 && capital_first {
                c.to_ascii_uppercase()
            } else {
                c
            };
            self.tokens.push(Token::Letter {
                ch,
                capital: i == 0 && capital_first,
            });
        }
        Ok(true)
    }
}

fn tokenize(input: &str) -> Result<Vec<Token>> {
    crate::ueb_symbols::ensure_ueb_symbols_loaded()?;
    Lexer::new(input).lex()
}

fn is_blackboard_letter(s: &str) -> bool {
    let mut chars = s.chars();
    let Some(c) = chars.next() else {
        return false;
    };
    if chars.next().is_some() {
        return false;
    }
    if matches!(c, 'ℂ' | 'ℍ' | 'ℕ' | 'ℙ' | 'ℚ' | 'ℝ' | 'ℤ' | '𝕆') {
        return true;
    }
    let o = c as u32;
    (0x1D538..=0x1D56B).contains(&o) || (0x1D7D8..=0x1D7E1).contains(&o)
}

/// Symbols that should tokenize as identifiers (letters / large ops), not bare `<mo>`.
fn is_identifier_symbol(s: &str) -> bool {
    // Omission blank ⠨⠤ is recovered as an identifier (ICEB 3.6.2), not `<mo>`.
    s == "_"
        || is_blackboard_letter(s)
        || matches!(s, "∑" | "∏" | "∫" | "∐" | "∮" | "⋀" | "⋁" | "⋂" | "⋃")
        || {
            let mut chars = s.chars();
            match (chars.next(), chars.next()) {
                (Some(c), None) => c.is_alphabetic(),
                _ => false,
            }
        }
}

fn braille_cell_to_digit(cell: char) -> Option<char> {
    Some(match cell {
        '⠁' => '1',
        '⠃' => '2',
        '⠉' => '3',
        '⠙' => '4',
        '⠑' => '5',
        '⠋' => '6',
        '⠛' => '7',
        '⠓' => '8',
        '⠊' => '9',
        '⠚' => '0',
        _ => return None,
    })
}

fn braille_cell_to_latin(cell: char) -> Option<char> {
    Some(match cell {
        '⠁' => 'a',
        '⠃' => 'b',
        '⠉' => 'c',
        '⠙' => 'd',
        '⠑' => 'e',
        '⠋' => 'f',
        '⠛' => 'g',
        '⠓' => 'h',
        '⠊' => 'i',
        '⠚' => 'j',
        '⠅' => 'k',
        '⠇' => 'l',
        '⠍' => 'm',
        '⠝' => 'n',
        '⠕' => 'o',
        '⠏' => 'p',
        '⠟' => 'q',
        '⠗' => 'r',
        '⠎' => 's',
        '⠞' => 't',
        '⠥' => 'u',
        '⠧' => 'v',
        '⠺' => 'w',
        '⠭' => 'x',
        '⠽' => 'y',
        '⠵' => 'z',
        _ => return None,
    })
}

/// Braille cells used for Greek letters that are not in a–z (η θ χ).
fn braille_greek_only_cell(cell: char) -> Option<char> {
    match cell {
        '⠱' => Some('η'),
        '⠹' => Some('θ'),
        '⠯' => Some('χ'),
        _ => None,
    }
}

/// Map Latin letters to common fraktur / black-letter print forms used by UEB.
fn latin_to_fraktur(ch: char) -> char {
    match ch {
        'C' => 'ℭ',
        'H' => 'ℌ',
        'I' => 'ℑ',
        'R' => 'ℜ',
        'Z' => 'ℨ',
        // Mathematical Fraktur capitals U+1D504–1D51C (skipping C,H,I,R,Z holes)
        'A' => '\u{1D504}',
        'B' => '\u{1D505}',
        'D' => '\u{1D507}',
        'E' => '\u{1D508}',
        'F' => '\u{1D509}',
        'G' => '\u{1D50A}',
        'J' => '\u{1D50D}',
        'K' => '\u{1D50E}',
        'L' => '\u{1D50F}',
        'M' => '\u{1D510}',
        'N' => '\u{1D511}',
        'O' => '\u{1D512}',
        'P' => '\u{1D513}',
        'Q' => '\u{1D514}',
        'S' => '\u{1D516}',
        'T' => '\u{1D517}',
        'U' => '\u{1D518}',
        'V' => '\u{1D519}',
        'W' => '\u{1D51A}',
        'X' => '\u{1D51B}',
        'Y' => '\u{1D51C}',
        // Mathematical Fraktur small letters U+1D51E–1D537
        c @ 'a'..='z' => char::from_u32(0x1D51E + (c as u32 - 'a' as u32)).unwrap_or(c),
        c => c,
    }
}

fn is_greek_letter_char(s: &str) -> bool {
    let mut chars = s.chars();
    match (chars.next(), chars.next()) {
        (Some(c), None) => {
            let o = c as u32;
            (0x0370..=0x03FF).contains(&o) || c == '∑' || c == '∏'
        }
        _ => false,
    }
}

fn op_takes_post_scripts(s: &str) -> bool {
    matches!(
        s,
        "∑" | "∏" | "∫" | "∐" | "∮" | "⋀" | "⋁" | "⋂" | "⋃" | "σ" | "Σ"
    ) || is_identifier_symbol(s)
}

fn latin_to_greek(latin_or_special: char, capital: bool) -> char {
    let g = match latin_or_special {
        'a' => 'α',
        'b' => 'β',
        'g' => 'γ',
        'd' => 'δ',
        'e' => 'ε',
        'z' => 'ζ',
        'η' => 'η',
        'θ' => 'θ',
        'i' => 'ι',
        'k' => 'κ',
        'l' => 'λ',
        'm' => 'μ',
        'n' => 'ν',
        'x' => 'ξ',
        'o' => 'ο',
        'p' => 'π',
        'r' => 'ρ',
        's' => 'σ',
        't' => 'τ',
        'u' => 'υ',
        'f' => 'φ',
        'χ' => 'χ',
        'y' => 'ψ',
        'w' => 'ω',
        // ⠹ maps through braille_cell_to_latin as θ already
        c if c == 'θ' || c == 'η' || c == 'χ' => c,
        c => c,
    };
    if capital {
        match g {
            'α' => 'Α',
            'β' => 'Β',
            'γ' => 'Γ',
            'δ' => 'Δ',
            'ε' => 'Ε',
            'ζ' => 'Ζ',
            'η' => 'Η',
            'θ' => 'Θ',
            'ι' => 'Ι',
            'κ' => 'Κ',
            'λ' => 'Λ',
            'μ' => 'Μ',
            'ν' => 'Ν',
            'ξ' => 'Ξ',
            'ο' => 'Ο',
            'π' => 'Π',
            'ρ' => 'Ρ',
            'σ' => 'Σ',
            'τ' => 'Τ',
            'υ' => 'Υ',
            'φ' => 'Φ',
            'χ' => 'Χ',
            'ψ' => 'Ψ',
            'ω' => 'Ω',
            c => c,
        }
    } else {
        g
    }
}

// --- parser (on tokens) -----------------------------------------------------

type Toks<'a> = &'a [Token];

fn parse_tokens(tokens: &[Token]) -> Result<Expr> {
    if tokens.is_empty() {
        return Ok(Expr::Row(vec![]));
    }
    // Bare equation-line mtable: row breaks only, no ⠠⠀ bounds, no fences.
    if let Some(expr) = try_parse_unbracketed_mtable(tokens) {
        return Ok(expr);
    }
    let mut parts = Vec::new();
    let mut input = tokens;
    loop {
        input = skip_noise(input);
        if input.is_empty() {
            break;
        }
        // Top-level unmatched closers (mid-typing / missing open) → print operators,
        // not leftover braille mtext. Braille grouping closers are not printed.
        match input.first() {
            Some(Token::Close('⟧')) => {
                input = &input[1..];
                continue;
            }
            Some(Token::Close(c)) => {
                parts.push(Expr::Operator(c.to_string()));
                input = &input[1..];
                continue;
            }
            Some(Token::EnlargedFence(c)) if !is_open_fence_char(*c) => {
                parts.push(Expr::Operator(c.to_string()));
                input = &input[1..];
                continue;
            }
            Some(Token::TableBound) => {
                // Marked equation-line table, or literary comma when no row markers follow.
                if input.iter().any(|t| matches!(t, Token::TableRowStart)) {
                    let (rest, expr) = parse_marked_equation_lines(input)?;
                    parts.push(expr);
                    input = rest;
                } else {
                    parts.push(Expr::Operator(",".into()));
                    input = &input[1..];
                }
                continue;
            }
            Some(Token::TableRowStart) => {
                let (rest, expr) = parse_row_marked_matrix(input)?;
                parts.push(expr);
                input = rest;
                continue;
            }
            // Multi-row linearized matrix: enlarged or normal open … enlarged close `⠸⠀` …
            Some(Token::EnlargedFence(c))
                if is_open_fence_char(*c) && looks_like_row_marked_matrix(input) =>
            {
                let (rest, expr) = parse_row_marked_matrix(input)?;
                parts.push(expr);
                input = rest;
                continue;
            }
            Some(Token::Open(_)) | Some(Token::VertBar) if looks_like_row_marked_matrix(input) => {
                let (rest, expr) = parse_row_marked_matrix(input)?;
                parts.push(expr);
                input = rest;
                continue;
            }
            _ => {}
        }

        let (rest, expr) = parse_expr(input)?;
        if rest.len() == input.len() {
            // No progress — keep remaining cells visible as braille mtext.
            let trail = input.iter().map(token_to_braille).collect::<String>();
            if !trail.is_empty() {
                parts.push(Expr::Text(trail));
            }
            break;
        }
        match expr {
            Expr::Row(v) => parts.extend(v),
            e => parts.push(e),
        }
        input = rest;
    }
    Ok(Expr::row(parts))
}

/// Best-effort reverse of a token to UEB cells for leftover trailing mtext.
fn token_to_braille(t: &Token) -> String {
    match t {
        Token::Space => "⠀".into(),
        Token::LevelUp => "⠔".into(),
        Token::LevelDown => "⠢".into(),
        Token::Above => "⠨⠔".into(),
        Token::Below => "⠨⠢".into(),
        Token::FracOpen => "⠷".into(),
        Token::FracLine => "⠨⠌".into(),
        Token::FracClose => "⠾".into(),
        Token::SqrtOpen => "⠩".into(),
        Token::SqrtClose => "⠬".into(),
        Token::Open('(') => "⠐⠣".into(),
        Token::Close(')') => "⠐⠜".into(),
        Token::Open('[') => "⠨⠣".into(),
        Token::Close(']') => "⠨⠜".into(),
        Token::Open('{') => "⠸⠣".into(),
        Token::Close('}') => "⠸⠜".into(),
        Token::Open('⟦') => "⠣".into(),
        Token::Close('⟧') => "⠜".into(),
        Token::Open(c) | Token::Close(c) => c.to_string(),
        Token::VertBar => "⠸⠳".into(),
        Token::TableRowStart => "⠸⠀".into(),
        Token::TableBound => "⠠⠀".into(),
        Token::EnlargedFence(c) => match c {
            '(' => "⠠⠐⠣".into(),
            ')' => "⠠⠐⠜".into(),
            '[' => "⠠⠨⠣".into(),
            ']' => "⠠⠨⠜".into(),
            '{' => "⠠⠸⠣".into(),
            '}' => "⠠⠸⠜".into(),
            '|' => "⠠⠸⠳".into(),
            other => other.to_string(),
        },
        Token::Grade1PassageEnd => "⠰⠄".into(),
        Token::Letter { ch, capital } => {
            let lower = ch.to_ascii_lowercase();
            let cell = latin_to_braille_cell(lower).unwrap_or(*ch);
            if *capital {
                format!("⠠{cell}")
            } else {
                cell.to_string()
            }
        }
        Token::Number(n) => {
            if let Some((num, den)) = n.strip_prefix("FRAC:").and_then(|s| s.split_once('/')) {
                format!(
                    "⠼{}⠌{}",
                    digits_to_braille(num),
                    digits_to_braille(den)
                )
            } else {
                format!("⠼{}", digits_to_braille(n))
            }
        }
        Token::Op(s) => s.clone(),
        Token::SimpleOver(s) | Token::SimpleUnder(s) => s.clone(),
        Token::BrailleText(s) => s.clone(),
    }
}

fn latin_to_braille_cell(ch: char) -> Option<char> {
    Some(match ch {
        'a' => '⠁',
        'b' => '⠃',
        'c' => '⠉',
        'd' => '⠙',
        'e' => '⠑',
        'f' => '⠋',
        'g' => '⠛',
        'h' => '⠓',
        'i' => '⠊',
        'j' => '⠚',
        'k' => '⠅',
        'l' => '⠇',
        'm' => '⠍',
        'n' => '⠝',
        'o' => '⠕',
        'p' => '⠏',
        'q' => '⠟',
        'r' => '⠗',
        's' => '⠎',
        't' => '⠞',
        'u' => '⠥',
        'v' => '⠧',
        'w' => '⠺',
        'x' => '⠭',
        'y' => '⠽',
        'z' => '⠵',
        _ => return None,
    })
}

fn digits_to_braille(digits: &str) -> String {
    digits
        .chars()
        .map(|d| match d {
            '1' => '⠁',
            '2' => '⠃',
            '3' => '⠉',
            '4' => '⠙',
            '5' => '⠑',
            '6' => '⠋',
            '7' => '⠛',
            '8' => '⠓',
            '9' => '⠊',
            '0' => '⠚',
            c => c,
        })
        .collect()
}

fn skip_noise(input: Toks<'_>) -> Toks<'_> {
    let mut i = input;
    while let Some(Token::Grade1PassageEnd) = i.first() {
        i = &i[1..];
    }
    i
}

fn digit_to_bold(c: char) -> char {
    match c {
        '0'..='9' => char::from_u32(0x1D7CE + (c as u32 - '0' as u32)).unwrap_or(c),
        _ => c,
    }
}

fn to_bold_digits(s: &str) -> String {
    s.chars().map(digit_to_bold).collect()
}

/// `100` + `°` → `<msup><mn>100</mn><mo>°</mo></msup>` so C/F stay separate units.
fn fold_degree_superscripts(parts: Vec<Expr>) -> Vec<Expr> {
    let mut out = Vec::with_capacity(parts.len());
    let mut iter = parts.into_iter().peekable();
    while let Some(part) = iter.next() {
        if matches!(part, Expr::Number(_))
            && matches!(iter.peek(), Some(Expr::Operator(s)) if s == "°")
        {
            let deg = iter.next().unwrap();
            out.push(Expr::Sup(Box::new(part), Box::new(deg)));
        } else {
            out.push(part);
        }
    }
    out
}

/// `~` `~` → single `~~` (MathJax double-tilde encoding; UEB repeats ⠈⠔).
fn fold_double_tildes(parts: Vec<Expr>) -> Vec<Expr> {
    let mut out = Vec::with_capacity(parts.len());
    let mut iter = parts.into_iter().peekable();
    while let Some(part) = iter.next() {
        let is_tilde =
            |e: &Expr| matches!(e, Expr::Operator(s) if s == "~" || s == "∼" || s == "˜");
        if is_tilde(&part) && iter.peek().is_some_and(is_tilde) {
            let _ = iter.next();
            out.push(Expr::Operator("~~".into()));
        } else {
            out.push(part);
        }
    }
    out
}

/// Flatten nested rows of letters / scripted letters so `A` `B̂` `C` stays one mrow.
fn flatten_juxtaposed_letter_rows(parts: Vec<Expr>) -> Vec<Expr> {
    let mut out = Vec::with_capacity(parts.len());
    for part in parts {
        match part {
            Expr::Row(inner) if !inner.is_empty() && inner.iter().all(is_letterish_factor) => {
                out.extend(inner);
            }
            other => out.push(other),
        }
    }
    out
}

fn is_letterish_factor(e: &Expr) -> bool {
    match e {
        Expr::Identifier(_) => true,
        Expr::Over(b, _) | Expr::Under(b, _) | Expr::Sup(b, _) | Expr::Sub(b, _) => {
            matches!(b.as_ref(), Expr::Identifier(_))
        }
        Expr::SubSup(b, _, _) | Expr::UnderOver(b, _, _) => {
            matches!(b.as_ref(), Expr::Identifier(_))
        }
        _ => false,
    }
}

fn parse_err(input: Toks<'_>, msg: &str) -> crate::errors::Error {
    crate::errors::anyhow!("{msg}; next={:?}", input.first())
}

fn parse_expr(input: Toks<'_>) -> std::result::Result<(Toks<'_>, Expr), crate::errors::Error> {
    let mut input = input;
    let mut parts = Vec::new();
    while !input.is_empty() && !matches!(input.first(), Some(Token::Grade1PassageEnd)) {
        // Stop at closers / fraction line when called as a sub-parse — those are handled by caller
        if matches!(
            input.first(),
            Some(
                Token::FracLine
                    | Token::FracClose
                    | Token::SqrtClose
                    | Token::Close(_)
            )
        ) {
            break;
        }
        let (rest, chunk) = parse_expr_part(input)?;
        if chunk.is_empty() && rest.as_ptr() == input.as_ptr() {
            break;
        }
        parts.extend(chunk);
        input = rest;
    }
    Ok((input, Expr::row(parts)))
}

/// One spaced “chunk”: optional spaces + atom-with-scripts or operator.
fn parse_expr_part(
    input: Toks<'_>,
) -> std::result::Result<(Toks<'_>, Vec<Expr>), crate::errors::Error> {
    let input_before = input;
    let mut input = input;
    let mut spaces_before = Vec::new();
    while matches!(input.first(), Some(Token::Space)) {
        spaces_before.push(Expr::Space);
        input = &input[1..];
    }
    if input.is_empty() {
        return Ok((input, spaces_before));
    }
    if matches!(
        input.first(),
        Some(
            Token::FracLine
                | Token::FracClose
                | Token::SqrtClose
                | Token::Close(_)
                | Token::Grade1PassageEnd
        )
    ) {
        // Closers / fraction line belong to the caller — don't consume spaces alone.
        if spaces_before.is_empty() {
            return Ok((input, vec![]));
        }
        return Ok((input_before, vec![]));
    }

    if let Some(Token::Op(s)) = input.first() {
        // Arrows / large ops may take under/over (e.g. Haber process: → with scripts).
        // LevelUp/LevelDown after binary +/−/= are left-superscripts on the next atom
        // (GTM 7.8), but ∫/∑/… still take ordinary post-scripts.
        let limit = matches!(
            input.get(1),
            Some(Token::Above | Token::Below | Token::SimpleOver(_) | Token::SimpleUnder(_))
        );
        let large_op_script = matches!(
            input.get(1),
            Some(Token::LevelUp | Token::LevelDown)
        ) && op_takes_post_scripts(s);
        if limit || large_op_script {
            let (input, atom) = parse_scripted(input)?;
            let mut v = spaces_before;
            v.push(atom);
            return Ok((input, v));
        }
        let op = Expr::Operator(s.clone());
        input = &input[1..];
        let mut spaces_after = Vec::new();
        while matches!(input.first(), Some(Token::Space)) {
            spaces_after.push(Expr::Space);
            input = &input[1..];
        }
        let mut v = spaces_before;
        v.push(op);
        v.extend(spaces_after);
        return Ok((input, v));
    }

    if let Some(Token::BrailleText(s)) = input.first() {
        let text = Expr::Text(s.clone());
        input = &input[1..];
        let mut v = spaces_before;
        v.push(text);
        return Ok((input, v));
    }

    let (input, atom) = match parse_scripted(input) {
        Ok(x) => x,
        Err(_) => return Ok((input_before, vec![])),
    };
    // Simple / binomial fraction line between items: num ⠻ den → mfrac linethickness=0
    if matches!(input.first(), Some(Token::FracLine)) {
        let input = &input[1..];
        let (input, den) = parse_scripted_or_missing(input);
        let mut v = spaces_before;
        v.push(Expr::BinomFrac(Box::new(atom), Box::new(den)));
        return Ok((input, v));
    }
    let mut v = spaces_before;
    v.push(atom);
    Ok((input, v))
}

fn parse_scripted_or_missing(input: Toks<'_>) -> (Toks<'_>, Expr) {
    if input.is_empty() || is_expr_stop_token(input.first()) {
        return (input, Expr::Missing);
    }
    match parse_scripted(input) {
        Ok(x) => x,
        Err(_) => (input, Expr::Missing),
    }
}

fn is_expr_stop_token(t: Option<&Token>) -> bool {
    matches!(
        t,
        Some(
            Token::FracLine
                | Token::FracClose
                | Token::SqrtClose
                | Token::Close(_)
                | Token::Grade1PassageEnd
                | Token::TableRowStart
                | Token::TableBound
        )
    )
}

fn parse_scripted(
    input: Toks<'_>,
) -> std::result::Result<(Toks<'_>, Expr), crate::errors::Error> {
    let mut input = input;
    // Optional left-displaced scripts (GTM 7.8) before the base
    let mut pre_sub: Option<Expr> = None;
    let mut pre_sup: Option<Expr> = None;
    loop {
        match input.first() {
            Some(Token::LevelDown) => {
                input = &input[1..];
                let (rest, script) = parse_item_or_missing(input);
                input = rest;
                pre_sub = Some(script);
            }
            Some(Token::LevelUp) => {
                input = &input[1..];
                let (rest, script) = parse_item_or_missing(input);
                input = rest;
                pre_sup = Some(script);
            }
            _ => break,
        }
    }

    let (mut input, base) = match parse_atom(input) {
        Ok(x) => x,
        Err(_) if pre_sub.is_some() || pre_sup.is_some() => (input, Expr::Missing),
        Err(e) => return Err(e),
    };

    let mut sub: Option<Expr> = None;
    let mut sup: Option<Expr> = None;
    let mut under: Option<Expr> = None;
    let mut over: Option<Expr> = None;
    // Track whether superscript was attached before subscript (msup-then-msub nesting).
    let mut first_post_was_sup: Option<bool> = None;
    loop {
        match input.first() {
            Some(Token::LevelUp) => {
                input = &input[1..];
                let (rest, script) = parse_item_or_missing(input);
                input = rest;
                if first_post_was_sup.is_none() {
                    first_post_was_sup = Some(true);
                }
                if let Some(prev) = sup.take() {
                    sup = Some(Expr::Sup(Box::new(prev), Box::new(script)));
                } else {
                    sup = Some(script);
                }
            }
            Some(Token::LevelDown) => {
                input = &input[1..];
                let (rest, script) = parse_item_or_missing(input);
                input = rest;
                if first_post_was_sup.is_none() {
                    first_post_was_sup = Some(false);
                }
                if let Some(prev) = sub.take() {
                    sub = Some(Expr::Sub(Box::new(prev), Box::new(script)));
                } else {
                    sub = Some(script);
                }
            }
            Some(Token::Above) => {
                input = &input[1..];
                let (rest, script) = parse_item_or_missing(input);
                input = rest;
                over = Some(normalize_overscript(script));
            }
            Some(Token::Below) => {
                input = &input[1..];
                let (rest, script) = parse_item_or_missing(input);
                input = rest;
                under = Some(script);
            }
            Some(Token::SimpleOver(sym)) => {
                let s = sym.clone();
                input = &input[1..];
                over = Some(Expr::Operator(s));
            }
            Some(Token::SimpleUnder(sym)) => {
                let s = sym.clone();
                input = &input[1..];
                under = Some(Expr::Operator(s));
            }
            _ => break,
        }
    }
    // Multi-letter identifier rows (e.g. a b̂): scripts/accents bind to the last chunk only.
    let expr = map_script_base(base, |base| {
        let base = match (sub, sup, first_post_was_sup) {
            // Chemistry ions / oxidation: prefer mmultiscripts (ICEB §16).
            (ref sub, Some(sup), _) if wants_chem_multiscripts(sub.as_ref(), &sup) => {
                Expr::MultiScripts {
                    base: Box::new(base),
                    sub: sub.as_ref().map(|e| Box::new(e.clone())),
                    sup: Some(Box::new(unwrap_chem_charge_group(sup))),
                }
            }
            // x^2_k written as LevelUp then LevelDown → nested msub(msup) (GTM 7.7.2)
            (Some(sub), Some(sup), Some(true)) => {
                Expr::Sub(Box::new(Expr::Sup(Box::new(base), Box::new(sup))), Box::new(sub))
            }
            (Some(sub), Some(sup), _) => Expr::SubSup(Box::new(base), Box::new(sub), Box::new(sup)),
            (Some(sub), None, _) => Expr::Sub(Box::new(base), Box::new(sub)),
            (None, Some(sup), _) => Expr::Sup(Box::new(base), Box::new(sup)),
            (None, None, _) => base,
        };
        match (under, over) {
            (Some(u), Some(o)) => Expr::UnderOver(Box::new(base), Box::new(u), Box::new(o)),
            (Some(u), None) => Expr::Under(Box::new(base), Box::new(u)),
            (None, Some(o)) => Expr::Over(Box::new(base), Box::new(o)),
            (None, None) => base,
        }
    });

    // Left-superscript on a letter/number base: empty-base `<msup>` then the base
    // (GTM 7.8 signed numbers / combinatorics), matching MathJax-style tests.
    let expr = if pre_sub.is_none() {
        if let Some(pre_sup) = pre_sup {
            let empty_msup_base = match &expr {
                Expr::Identifier(_) | Expr::Number(_) => true,
                Expr::Sub(b, _) | Expr::Sup(b, _) | Expr::MultiScripts { base: b, .. } => {
                    matches!(b.as_ref(), Expr::Identifier(_) | Expr::Number(_))
                }
                Expr::SubSup(b, _, _) => {
                    matches!(b.as_ref(), Expr::Identifier(_) | Expr::Number(_))
                }
                _ => false,
            };
            if empty_msup_base {
                Expr::row(vec![
                    Expr::Sup(Box::new(Expr::Row(vec![])), Box::new(pre_sup)),
                    expr,
                ])
            } else {
                Expr::Prescripts {
                    base: Box::new(expr),
                    sub: None,
                    sup: Some(Box::new(pre_sup)),
                }
            }
        } else {
            expr
        }
    } else {
        Expr::Prescripts {
            base: Box::new(expr),
            sub: pre_sub.map(Box::new),
            sup: pre_sup.map(Box::new),
        }
    };

    Ok((input, expr))
}

/// Map double-dot oversights from ‥ (two-dot leader) to diaeresis ¨ used by forward UEB.
fn normalize_overscript(script: Expr) -> Expr {
    let script = match script {
        Expr::Group(inner) => *inner,
        other => other,
    };
    match script {
        Expr::Operator(s) if s == "‥" || s == ".." || s == "¨" => Expr::Operator("¨".into()),
        Expr::Row(parts) if parts.len() == 2 => {
            let dots = parts.iter().all(|p| matches!(p, Expr::Operator(s) if s == "." || s == "˙" || s == "․"));
            if dots {
                Expr::Operator("¨".into())
            } else {
                Expr::Row(parts)
            }
        }
        other => other,
    }
}

/// GTM 7.1 “item” after a level-change / above / below indicator.
/// An item is a number, fraction, radical, arrow, bracketed/grouped expr, or else
/// the next individual symbol (not a multi-letter identifier run).
fn parse_item(input: Toks<'_>) -> std::result::Result<(Toks<'_>, Expr), crate::errors::Error> {
    if input.is_empty() {
        return Err(parse_err(input, "expected item"));
    }
    match input.first().unwrap() {
        Token::Number(_) | Token::FracOpen | Token::SqrtOpen | Token::Open(_) | Token::VertBar => {
            parse_atom(input)
        }
        Token::Letter { ch, .. } => Ok((&input[1..], Expr::Identifier(ch.to_string()))),
        Token::Op(s) => Ok((&input[1..], Expr::Operator(s.clone()))),
        Token::BrailleText(s) => Ok((&input[1..], Expr::Text(s.clone()))),
        _ => parse_atom(input),
    }
}

fn parse_item_or_missing(input: Toks<'_>) -> (Toks<'_>, Expr) {
    if input.is_empty() || is_expr_stop_token(input.first()) {
        return (input, Expr::Missing);
    }
    match parse_item(input) {
        Ok(x) => x,
        Err(_) => (input, Expr::Missing),
    }
}

fn parse_atom(input: Toks<'_>) -> std::result::Result<(Toks<'_>, Expr), crate::errors::Error> {
    if input.is_empty() {
        return Err(parse_err(input, "expected atom"));
    }
    match input.first().unwrap() {
        Token::Number(n) => {
            let rest = &input[1..];
            if let Some((num, den)) = n.strip_prefix("FRAC:").and_then(|s| s.split_once('/')) {
                Ok((
                    rest,
                    Expr::Frac(
                        Box::new(Expr::Number(num.to_string())),
                        Box::new(Expr::Number(den.to_string())),
                    ),
                ))
            } else {
                Ok((rest, Expr::Number(n.clone())))
            }
        }
        Token::Letter { .. } => parse_identifier(input),
        Token::FracOpen => parse_general_fraction(input),
        Token::SqrtOpen => parse_radical(input),
        Token::EnlargedFence(_) if looks_like_row_marked_matrix(input) => {
            parse_row_marked_matrix(input)
        }
        Token::EnlargedFence(_) => parse_enlarged_matrix(input),
        Token::VertBar => parse_fenced(input, '|'),
        Token::Open(c) => parse_fenced(input, *c),
        // Operators as atoms (degree, chemistry charge ±, arrows used as items, etc.)
        Token::Op(s) => Ok((&input[1..], Expr::Operator(s.clone()))),
        Token::BrailleText(s) => Ok((&input[1..], Expr::Text(s.clone()))),
        _ => Err(parse_err(input, "expected atom")),
    }
}

/// If `base` is a multi-chunk identifier row, bind scripts/accents to the last chunk only
/// (GTM 12: hat on the last letter of a run split into single-letter identifiers).
/// Braille grouping (`Expr::Group`) is atomic — accents apply to the whole group.
fn map_script_base(base: Expr, f: impl FnOnce(Expr) -> Expr) -> Expr {
    match base {
        Expr::Group(inner) => f(Expr::Group(inner)),
        Expr::Row(mut parts) if parts.len() > 1 => {
            let last = parts.pop().unwrap();
            parts.push(f(last));
            Expr::row(parts)
        }
        other => f(other),
    }
}

/// Multi-letter function names / units recognised as a single `<mi>` (longest match first).
const KNOWN_MULTI_LETTER_NAMES: &[&str] = &[
    // length 8+
    "argument",
    // length 6
    "arccos", "arcsin", "arctan",
    // length 4
    "sinh", "cosh", "tanh", "sech", "csch", "coth", "Real",
    // length 3
    "log", "lim", "sin", "cos", "tan", "sec", "csc", "cot", "max", "min", "gcd", "lcm",
    "exp", "det", "dim", "ker", "arg", "deg", "and", "erf", "Area",
    // length 2
    "ln", "Pr", "dx", "dy", "dz", "km", "cm", "mm", "kg", "mg", "ms", "ns", "pm", "am",
    "ft", "in", "or", "to",
];

/// Roman-numeral letter inventory (upper and lower).
fn is_roman_numeral_char(c: char) -> bool {
    matches!(
        c,
        'I' | 'V' | 'X' | 'L' | 'C' | 'D' | 'M' | 'i' | 'v' | 'x' | 'l' | 'c' | 'd' | 'm'
    )
}

fn is_roman_numeral_letters(s: &str) -> bool {
    !s.is_empty() && s.chars().all(is_roman_numeral_char)
}

/// Chemistry charge / oxidation-state superscripts (ICEB §16).
fn is_chem_charge_op(s: &str) -> bool {
    matches!(s, "+" | "-" | "−" | "±")
}

fn is_oxidation_roman(s: &str) -> bool {
    // Oxidation states are uppercase roman (II, III, …). Lowercase single letters
    // like n/m/i/x are ordinary math identifiers, not chem.
    !s.is_empty() && s.chars().all(|c| matches!(c, 'I' | 'V' | 'X' | 'L' | 'C' | 'D' | 'M'))
}

fn is_chem_charge_or_oxidation(expr: &Expr) -> bool {
    match expr {
        Expr::Operator(s) => is_chem_charge_op(s) || s == "++" || s == "--",
        Expr::Identifier(s) | Expr::Number(s) => is_oxidation_roman(s),
        Expr::Group(inner) | Expr::Fenced { body: inner, .. } => {
            is_chem_charge_or_oxidation(inner)
        }
        Expr::Row(parts) if parts.len() == 1 => is_chem_charge_or_oxidation(&parts[0]),
        Expr::Row(parts) => {
            !parts.is_empty()
                && parts
                    .iter()
                    .all(|p| matches!(p, Expr::Operator(s) if is_chem_charge_op(s)))
        }
        _ => false,
    }
}

/// Use `<mmultiscripts>` for chem ions with a subscript, multi-charge, or oxidation
/// state. Bare `+`/`−` alone stay as `<msup>` (math / simple ions in GTM examples).
fn wants_chem_multiscripts(sub: Option<&Expr>, sup: &Expr) -> bool {
    if !is_chem_charge_or_oxidation(sup) {
        return false;
    }
    if sub.is_some() {
        return true;
    }
    match sup {
        Expr::Identifier(s) | Expr::Number(s) => is_oxidation_roman(s),
        Expr::Group(inner) | Expr::Fenced { body: inner, .. } => {
            wants_chem_multiscripts(None, inner)
        }
        Expr::Row(parts) if parts.len() >= 2 => true,
        Expr::Operator(s) if s == "++" || s == "--" => true,
        _ => false,
    }
}

/// Drop script-grouping fences around charge rows: `(--)` → `--`.
fn unwrap_chem_charge_group(expr: Expr) -> Expr {
    match expr {
        Expr::Group(inner) if is_chem_charge_or_oxidation(inner.as_ref()) => {
            unwrap_chem_charge_group(*inner)
        }
        Expr::Fenced { body, .. } if is_chem_charge_or_oxidation(body.as_ref()) => {
            unwrap_chem_charge_group(*body)
        }
        other => other,
    }
}

/// Common two-letter chemical element symbols (matched only when the first letter is capital).
const CHEM_ELEMENTS_2: &[&str] = &[
    "He", "Li", "Be", "Ne", "Na", "Mg", "Al", "Si", "Cl", "Ar", "Ca", "Sc", "Ti", "Cr", "Mn",
    "Fe", "Co", "Ni", "Cu", "Zn", "Ga", "Ge", "As", "Se", "Br", "Kr", "Rb", "Sr", "Zr", "Nb",
    "Mo", "Tc", "Ru", "Rh", "Pd", "Ag", "Cd", "In", "Sn", "Sb", "Te", "Xe", "Cs", "Ba", "La",
    "Ce", "Pr", "Nd", "Pm", "Sm", "Eu", "Gd", "Tb", "Dy", "Ho", "Er", "Tm", "Yb", "Lu", "Hf",
    "Ta", "Re", "Os", "Ir", "Pt", "Au", "Hg", "Tl", "Pb", "Bi", "Po", "At", "Rn", "Fr", "Ra",
    "Ac", "Th", "Pa", "Np", "Pu", "Am", "Cm", "Bk", "Cf", "Es", "Fm", "Md", "No", "Lr", "Rf",
    "Db", "Sg", "Bh", "Hs", "Mt", "Ds", "Rg", "Cn", "Fl", "Lv", "Ts", "Og",
];

fn parse_identifier(
    input: Toks<'_>,
) -> std::result::Result<(Toks<'_>, Expr), crate::errors::Error> {
    // Collect consecutive letters; stop *before* a following script/accent token
    // (the letter that precedes the modifier is included so names like log/lim stay intact).
    // Also stop before switching between ASCII and non-ASCII letters (sinθ → sin + θ).
    let mut letters: Vec<(char, bool)> = Vec::new();
    let mut i = 0;
    while let Some(Token::Letter { ch, capital }) = input.get(i) {
        if let Some((prev, _)) = letters.last() {
            if prev.is_ascii_alphabetic() != ch.is_ascii_alphabetic() {
                break;
            }
        }
        letters.push((*ch, *capital));
        i += 1;
        if matches!(
            input.get(i),
            Some(
                Token::LevelUp
                    | Token::LevelDown
                    | Token::Above
                    | Token::Below
                    | Token::SimpleOver(_)
                    | Token::SimpleUnder(_)
            )
        ) {
            break;
        }
    }
    if letters.is_empty() {
        return Err(parse_err(input, "expected letter"));
    }
    let chunks = split_letter_run(&letters);
    Ok((&input[i..], Expr::row(chunks)))
}

fn split_letter_run(letters: &[(char, bool)]) -> Vec<Expr> {
    if letters.is_empty() {
        return vec![];
    }
    // Capital word/passage often marks juxtaposed single-letter variables (PQ+QR).
    // Keep an all-caps run together only when it is a roman numeral (II, CD, XIV).
    if letters.iter().all(|(_, capital)| *capital) {
        let s: String = letters.iter().map(|(ch, _)| *ch).collect();
        if is_roman_numeral_letters(&s) {
            return vec![Expr::Identifier(s)];
        }
    }
    // Lowercase ASCII runs of length ≥3 are literary / function-name remnants
    // (error, funcn, argut). Length-2 products like "kx" / "ab" stay split unless known.
    if letters.len() >= 3
        && letters
            .iter()
            .all(|(ch, capital)| !*capital && ch.is_ascii_alphabetic())
    {
        let s: String = letters.iter().map(|(ch, _)| *ch).collect();
        return vec![Expr::Identifier(s)];
    }
    // Proper names / labels in chem scripts: Haber, Newton (Cap + lowercase rest).
    if letters.len() >= 3
        && letters[0].1
        && letters[0].0.is_ascii_uppercase()
        && letters[1..]
            .iter()
            .all(|(ch, capital)| !*capital && ch.is_ascii_lowercase())
    {
        let s: String = letters.iter().map(|(ch, _)| *ch).collect();
        return vec![Expr::Identifier(s)];
    }
    let mut chunks = Vec::new();
    let mut i = 0;
    while i < letters.len() {
        if let Some(len) = longest_multi_letter_match(&letters[i..]) {
            let s: String = letters[i..i + len].iter().map(|(ch, _)| *ch).collect();
            chunks.push(Expr::Identifier(s));
            i += len;
        } else {
            chunks.push(Expr::Identifier(letters[i].0.to_string()));
            i += 1;
        }
    }
    chunks
}

fn longest_multi_letter_match(letters: &[(char, bool)]) -> Option<usize> {
    if letters.len() < 2 {
        return None;
    }
    let full: String = letters.iter().map(|(ch, _)| *ch).collect();
    // Function / unit names must consume the whole letter run (UEB usually spaces
    // args). Preferring prefixes would turn "argut" into arg+u+t.
    if full.len() <= 8
        && KNOWN_MULTI_LETTER_NAMES
            .iter()
            .any(|name| name.eq_ignore_ascii_case(&full))
    {
        return Some(letters.len());
    }
    // Known function/unit as prefix when the next letter is capital (SecA, sinA).
    // Lets "Sec" win over chem "Se" and keeps literary remnants like "argut" whole.
    if let Some(len) = known_name_prefix_before_capital(letters) {
        return Some(len);
    }
    // Maximal lowercase roman-numeral run (e.g. "vi", "xiv"). Capital roman
    // words (II, CD) are handled above; do not glue "CD" inside "ABCD".
    let roman_len = letters
        .iter()
        .take_while(|(ch, capital)| !*capital && is_roman_numeral_char(*ch))
        .count();
    if roman_len >= 2 && roman_len == letters.len() {
        return Some(roman_len);
    }
    // Two-letter chemical elements: capital then lowercase (Ca, not CA / IR).
    if letters.len() >= 2 && letters[0].1 && !letters[1].1 {
        let s: String = letters[..2].iter().map(|(ch, _)| *ch).collect();
        if CHEM_ELEMENTS_2
            .iter()
            .any(|el| el.eq_ignore_ascii_case(&s))
        {
            return Some(2);
        }
    }
    None
}

fn known_name_prefix_before_capital(letters: &[(char, bool)]) -> Option<usize> {
    let mut best = None;
    for &name in KNOWN_MULTI_LETTER_NAMES {
        let nlen = name.len();
        if nlen < 2 || nlen >= letters.len() {
            continue;
        }
        // Next letter starts a new capitalised atom.
        if !letters[nlen].1 {
            continue;
        }
        let s: String = letters[..nlen].iter().map(|(ch, _)| *ch).collect();
        if name.eq_ignore_ascii_case(&s) {
            best = Some(best.map_or(nlen, |b: usize| b.max(nlen)));
        }
    }
    best
}

fn parse_general_fraction(
    input: Toks<'_>,
) -> std::result::Result<(Toks<'_>, Expr), crate::errors::Error> {
    let input = expect_token(input, &Token::FracOpen)?;
    let (input, num_raw) =
        parse_until(input, |t| matches!(t, Token::FracLine | Token::FracClose))?;
    let has_line = matches!(input.first(), Some(Token::FracLine));
    let input = if has_line { &input[1..] } else { input };

    let (num, den, input) = if has_line {
        let (input, den_raw) = parse_until(input, |t| matches!(t, Token::FracClose))?;
        let input = match input.first() {
            Some(Token::FracClose) => &input[1..],
            _ => input,
        };
        let num = if num_raw.is_empty_row() {
            Expr::Row(vec![])
        } else {
            num_raw
        };
        let den = if den_raw.is_empty_row() {
            Expr::Row(vec![])
        } else {
            den_raw
        };
        (num, den, input)
    } else {
        let num = if num_raw.is_empty_row() {
            Expr::Missing
        } else {
            num_raw
        };
        let input = match input.first() {
            Some(Token::FracClose) => &input[1..],
            _ => input,
        };
        (num, Expr::Row(vec![]), input)
    };
    Ok((input, Expr::Frac(Box::new(num), Box::new(den))))
}

fn expect_token<'a>(
    input: Toks<'a>,
    expected: &Token,
) -> std::result::Result<Toks<'a>, crate::errors::Error> {
    match input.first() {
        Some(t) if t == expected => Ok(&input[1..]),
        _ => Err(parse_err(input, &format!("expected {expected:?}"))),
    }
}

fn parse_until<'a, F>(
    input: Toks<'a>,
    end: F,
) -> std::result::Result<(Toks<'a>, Expr), crate::errors::Error>
where
    F: Fn(&Token) -> bool,
{
    let mut depth = 0i32;
    let mut idx = 0;
    while let Some(t) = input.get(idx) {
        if depth == 0 && end(t) {
            break;
        }
        match t {
            Token::FracOpen | Token::SqrtOpen | Token::Open(_) => depth += 1,
            Token::FracClose | Token::SqrtClose | Token::Close(_) => depth -= 1,
            _ => {}
        }
        idx += 1;
    }
    let (inner, rest) = input.split_at(idx);
    let (_, expr) = parse_expr(inner)?;
    Ok((rest, expr))
}

fn parse_radical(
    input: Toks<'_>,
) -> std::result::Result<(Toks<'_>, Expr), crate::errors::Error> {
    let input = expect_token(input, &Token::SqrtOpen)?;
    let (input, index) = if matches!(input.first(), Some(Token::LevelUp)) {
        let input = &input[1..];
        let (input, idx) = parse_item_or_missing(input);
        (input, Some(idx))
    } else {
        (input, None)
    };
    let (input, base) = parse_until(input, |t| matches!(t, Token::SqrtClose))?;
    let input = match input.first() {
        Some(Token::SqrtClose) => &input[1..],
        _ => input,
    };
    let expr = match index {
        Some(idx) => Expr::Root(Box::new(idx), Box::new(base)),
        None => Expr::Sqrt(Box::new(base)),
    };
    Ok((input, expr))
}

fn parse_fenced(
    input: Toks<'_>,
    open: char,
) -> std::result::Result<(Toks<'_>, Expr), crate::errors::Error> {
    let close = matching_close(open);
    let input = match input.first() {
        Some(Token::Open(c)) if *c == open => &input[1..],
        Some(Token::VertBar) if open == '|' => &input[1..],
        _ => return Err(parse_err(input, "expected open fence")),
    };

    if open == '⟦' {
        let (input, body) = parse_until(input, |t| matches!(t, Token::Close('⟧')))?;
        let input = match input.first() {
            Some(Token::Close('⟧')) => &input[1..],
            _ => input, // incomplete grouping — keep body
        };
        return Ok((input, Expr::Group(Box::new(body))));
    }

    let is_close = |t: &Token| match t {
        Token::Close(c) if *c == close => true,
        Token::VertBar if open == '|' => true,
        _ => false,
    };

    let (input, body) = parse_until(input, is_close)?;
    let (input, close_tok) = match input.first() {
        Some(t) if is_close(t) => (&input[1..], Some(close.to_string())),
        _ => (input, None),
    };

    // Single-row linearized matrix: space-separated cells inside ordinary fences
    // (UEB_Rules default-mtr without enlarged markers when there is only one row).
    if close_tok.is_some() {
        if let Some(cells) = matrix_cells_from_spaced_body(&body) {
            return Ok((
                input,
                Expr::Table {
                    open: Some(open.to_string()),
                    close: close_tok,
                    rows: vec![cells],
                },
            ));
        }
    }

    Ok((
        input,
        Expr::Fenced {
            open: open.to_string(),
            close: close_tok,
            body: Box::new(body),
        },
    ))
}

/// GTM 15 / UEB_Rules: multi-row matrix as repeated enlarged fences on one line.
/// Pattern: `⠠(` cells `⠠)` `⠠(` cells `⠠)` …
fn parse_enlarged_matrix(
    input: Toks<'_>,
) -> std::result::Result<(Toks<'_>, Expr), crate::errors::Error> {
    let open = match input.first() {
        Some(Token::EnlargedFence(c)) if is_open_fence_char(*c) => *c,
        _ => return Err(parse_err(input, "expected enlarged open fence")),
    };
    let close = matching_close(open);
    let mut input = &input[1..];
    let mut rows = Vec::new();

    loop {
        let (rest, cells) = parse_matrix_row_cells(input, close)?;
        input = rest;
        rows.push(cells);

        // Expect enlarged close
        match input.first() {
            Some(Token::EnlargedFence(c)) if *c == close => {
                input = &input[1..];
            }
            _ => {
                return Ok((
                    input,
                    Expr::Table {
                        open: Some(open.to_string()),
                        close: None,
                        rows,
                    },
                ));
            }
        }

        // Another row?
        match input.first() {
            Some(Token::EnlargedFence(c)) if *c == open => {
                input = &input[1..];
                continue;
            }
            _ => break,
        }
    }

    Ok((
        input,
        Expr::Table {
            open: Some(open.to_string()),
            close: Some(close.to_string()),
            rows,
        },
    ))
}

/// True when tokens look like a multi-row fenced matrix: open … enlarged-close `⠸⠀` open …
fn looks_like_row_marked_matrix(input: Toks<'_>) -> bool {
    match input.first() {
        Some(Token::EnlargedFence(c)) if is_open_fence_char(*c) => {
            // Current UEB_Rules: both open and close are enlarged.
            enlarged_close_then_row_start(&input[1..], matching_close(*c))
        }
        Some(Token::TableRowStart) => {
            // Legacy encoding with a leading row marker.
            input.iter().skip(1).any(|t| {
                matches!(t, Token::Open(_) | Token::VertBar | Token::EnlargedFence(_))
            })
        }
        Some(Token::Open(c)) => {
            // Legacy: normal open … enlarged close `⠸⠀` — stop if a normal close appears first.
            let close = matching_close(*c);
            legacy_open_enlarged_close_row_start(input, *c, close)
        }
        Some(Token::VertBar) => {
            // `|P|` uses a second normal vert bar before any enlarged `⠠⠸⠳` matrix.
            legacy_vertbar_row_marked(input)
        }
        _ => false,
    }
}

fn enlarged_close_then_row_start(after_open: Toks<'_>, close: char) -> bool {
    for i in 0..after_open.len() {
        if matches!(after_open[i], Token::EnlargedFence(c) if c == close) {
            let mut j = i + 1;
            while j < after_open.len() && matches!(after_open[j], Token::Space) {
                j += 1;
            }
            return matches!(after_open.get(j), Some(Token::TableRowStart));
        }
    }
    false
}

fn legacy_open_enlarged_close_row_start(input: Toks<'_>, _open: char, close: char) -> bool {
    for i in 1..input.len() {
        match &input[i] {
            Token::Close(c) if *c == close => return false,
            Token::EnlargedFence(c) if *c == close => {
                let mut j = i + 1;
                while j < input.len() && matches!(input[j], Token::Space) {
                    j += 1;
                }
                return matches!(input.get(j), Some(Token::TableRowStart));
            }
            _ => {}
        }
    }
    false
}

fn legacy_vertbar_row_marked(input: Toks<'_>) -> bool {
    for i in 1..input.len() {
        match &input[i] {
            Token::VertBar => return false,
            Token::EnlargedFence('|') => {
                let mut j = i + 1;
                while j < input.len() && matches!(input[j], Token::Space) {
                    j += 1;
                }
                return matches!(input.get(j), Some(Token::TableRowStart));
            }
            _ => {}
        }
    }
    false
}

/// UEB_Rules multi-row fenced matrix/determinant.
/// Rows: enlarged (or normal) open + cells + enlarged close, with `⠸⠀` between rows
/// (not before row 1). Also accepts a legacy leading `⠸⠀`.
fn parse_row_marked_matrix(
    input: Toks<'_>,
) -> std::result::Result<(Toks<'_>, Expr), crate::errors::Error> {
    let mut input = input;
    if matches!(input.first(), Some(Token::TableRowStart)) {
        input = &input[1..];
    }
    let mut rows = Vec::new();
    let (rest, open, close, cells) = parse_matrix_row_segment(input)?;
    input = rest;
    rows.push(cells);

    while matches!(input.first(), Some(Token::TableRowStart)) {
        input = &input[1..];
        let (rest, next_open, next_close, cells) = parse_matrix_row_segment(input)?;
        if next_open != open || next_close != close {
            return Err(parse_err(input, "inconsistent matrix row fences"));
        }
        input = rest;
        rows.push(cells);
    }

    Ok((
        input,
        Expr::Table {
            open: Some(open),
            close: Some(close),
            rows,
        },
    ))
}

/// Bare multi-row equation-line `mtable`: `row ⠸⠀ row` with no fences and no `⠠⠀` bounds.
fn try_parse_unbracketed_mtable(tokens: &[Token]) -> Option<Expr> {
    if !tokens.iter().any(|t| matches!(t, Token::TableRowStart)) {
        return None;
    }
    if tokens.iter().any(|t| matches!(t, Token::TableBound)) {
        return None; // handled by parse_marked_equation_lines
    }
    if looks_like_row_marked_matrix(tokens) {
        return None;
    }
    // Require the row marker to appear as a top-level split (not only inside a fence body).
    if matches!(
        tokens.first(),
        Some(Token::Open(_) | Token::VertBar | Token::EnlargedFence(_) | Token::TableRowStart)
    ) {
        return None;
    }
    let mut rows = Vec::new();
    for segment in split_tokens_on(tokens, |t| matches!(t, Token::TableRowStart)) {
        let cells = parse_unbracketed_row_cells(segment).ok()?;
        if cells.is_empty() {
            return None;
        }
        rows.push(normalize_unbracketed_row_cells(cells));
    }
    if rows.len() < 2 {
        return None;
    }
    Some(Expr::Table {
        open: None,
        close: None,
        rows,
    })
}

/// Equation-line `mtable` with `⠠⠀` … `⠠⠀` bounds on each row (when not alone in the math).
fn parse_marked_equation_lines(
    input: Toks<'_>,
) -> std::result::Result<(Toks<'_>, Expr), crate::errors::Error> {
    let mut input = input;
    let mut rows = Vec::new();
    loop {
        input = skip_noise(input);
        if matches!(input.first(), Some(Token::TableBound)) {
            input = &input[1..];
        } else if rows.is_empty() {
            return Err(parse_err(input, "expected equation-line table bound"));
        } else {
            break;
        }
        let (rest, cells) = parse_unbracketed_row_until_bound(input)?;
        input = rest;
        rows.push(normalize_unbracketed_row_cells(cells));
        input = skip_noise(input);
        if matches!(input.first(), Some(Token::TableBound)) {
            input = &input[1..];
        }
        input = skip_noise(input);
        if matches!(input.first(), Some(Token::TableRowStart)) {
            input = &input[1..];
            continue;
        }
        break;
    }
    if rows.is_empty() {
        return Err(parse_err(input, "empty equation-line table"));
    }
    Ok((
        input,
        Expr::Table {
            open: None,
            close: None,
            rows,
        },
    ))
}

fn split_tokens_on<'a>(tokens: &'a [Token], pred: impl Fn(&Token) -> bool) -> Vec<&'a [Token]> {
    let mut out = Vec::new();
    let mut start = 0;
    for (i, t) in tokens.iter().enumerate() {
        if pred(t) {
            out.push(&tokens[start..i]);
            start = i + 1;
        }
    }
    out.push(&tokens[start..]);
    out
}

fn parse_unbracketed_row_until_bound(
    input: Toks<'_>,
) -> std::result::Result<(Toks<'_>, Vec<Expr>), crate::errors::Error> {
    let mut input = input;
    let mut cells = Vec::new();
    loop {
        while matches!(input.first(), Some(Token::Space)) {
            input = &input[1..];
        }
        if matches!(
            input.first(),
            Some(Token::TableBound | Token::TableRowStart) | None
        ) {
            break;
        }
        let mut parts = Vec::new();
        loop {
            if matches!(
                input.first(),
                Some(Token::Space | Token::TableBound | Token::TableRowStart) | None
            ) {
                break;
            }
            if is_expr_stop_token(input.first()) {
                break;
            }
            let (rest, part) = parse_expr_part_no_leading_space(input)?;
            if rest.len() == input.len() {
                break;
            }
            parts.extend(part);
            input = rest;
        }
        if !parts.is_empty() {
            cells.push(Expr::row(parts));
        } else {
            break;
        }
    }
    Ok((input, cells))
}

fn parse_unbracketed_row_cells(
    input: Toks<'_>,
) -> std::result::Result<Vec<Expr>, crate::errors::Error> {
    let (rest, cells) = parse_unbracketed_row_until_bound(input)?;
    if !rest.is_empty()
        && !matches!(rest.first(), Some(Token::TableBound | Token::TableRowStart))
    {
        // leftover non-boundary tokens — still accept what we parsed
    }
    Ok(cells)
}

/// Forward rules space around `=`, so `x = f(t)` becomes three space-cells; fold to two columns.
fn normalize_unbracketed_row_cells(cells: Vec<Expr>) -> Vec<Expr> {
    if cells.len() == 3 {
        let is_eq = |e: &Expr| matches!(e, Expr::Operator(s) if s == "=" || s == "＝");
        if is_eq(&cells[1]) {
            return vec![
                cells[0].clone(),
                Expr::row(vec![cells[1].clone(), cells[2].clone()]),
            ];
        }
    }
    if cells.len() >= 3 {
        let is_eq = |e: &Expr| matches!(e, Expr::Operator(s) if s == "=" || s == "＝");
        if is_eq(&cells[1]) {
            let mut rhs = vec![cells[1].clone()];
            rhs.extend(cells[2..].iter().cloned());
            return vec![cells[0].clone(), Expr::row(rhs)];
        }
    }
    cells
}

/// One linearized matrix row: open (normal or enlarged) + cells + enlarged close.
fn parse_matrix_row_segment(
    input: Toks<'_>,
) -> std::result::Result<(Toks<'_>, String, String, Vec<Expr>), crate::errors::Error> {
    let mut input = skip_noise(input);
    if input.is_empty() {
        return Err(parse_err(input, "expected matrix row open"));
    }
    let (open, close_ch) = match input.first().unwrap() {
        Token::Open(c) => {
            input = &input[1..];
            (*c, matching_close(*c))
        }
        Token::VertBar => {
            input = &input[1..];
            ('|', '|')
        }
        Token::EnlargedFence(c) if is_open_fence_char(*c) => {
            input = &input[1..];
            (*c, matching_close(*c))
        }
        _ => return Err(parse_err(input, "expected matrix row open")),
    };
    let (input, cells) = parse_matrix_row_cells(input, close_ch)?;
    // Enlarged close is optional when input ends mid-typing.
    let input = match input.first() {
        Some(Token::EnlargedFence(c)) if *c == close_ch => &input[1..],
        _ => input,
    };
    Ok((input, open.to_string(), close_ch.to_string(), cells))
}

fn is_open_fence_char(c: char) -> bool {
    matches!(c, '(' | '[' | '{' | '|')
}

/// Parse cells of one matrix row until the enlarged close fence (not consumed).
fn parse_matrix_row_cells(
    input: Toks<'_>,
    close: char,
) -> std::result::Result<(Toks<'_>, Vec<Expr>), crate::errors::Error> {
    let mut input = input;
    let mut cells = Vec::new();
    loop {
        while matches!(input.first(), Some(Token::Space)) {
            input = &input[1..];
        }
        if matches!(
            input.first(),
            Some(Token::EnlargedFence(c)) if *c == close
        ) || matches!(
            input.first(),
            Some(Token::TableRowStart | Token::TableBound)
        ) {
            break;
        }
        if input.is_empty() {
            break;
        }
        // One cell: atoms/ops until a column-separating space or the row closer.
        let mut parts = Vec::new();
        loop {
            if matches!(input.first(), Some(Token::Space))
                || matches!(input.first(), Some(Token::EnlargedFence(c)) if *c == close)
                || matches!(input.first(), Some(Token::TableRowStart | Token::TableBound))
                || input.is_empty()
            {
                break;
            }
            if matches!(
                input.first(),
                Some(
                    Token::FracLine
                        | Token::FracClose
                        | Token::SqrtClose
                        | Token::Close(_)
                        | Token::Grade1PassageEnd
                        | Token::TableRowStart
                )
            ) {
                break;
            }
            let (rest, chunk) = parse_expr_part_no_leading_space(input)?;
            if chunk.is_empty() && rest.as_ptr() == input.as_ptr() {
                break;
            }
            parts.extend(chunk);
            input = rest;
        }
        if parts.is_empty() {
            break;
        }
        cells.push(Expr::row(parts));
    }
    if cells.is_empty() {
        cells.push(Expr::Row(vec![]));
    }
    Ok((input, cells))
}

/// Like parse_expr_part but does not consume leading spaces (those are column separators).
fn parse_expr_part_no_leading_space(
    input: Toks<'_>,
) -> std::result::Result<(Toks<'_>, Vec<Expr>), crate::errors::Error> {
    if input.is_empty() {
        return Ok((input, vec![]));
    }
    if matches!(
        input.first(),
        Some(
            Token::FracLine
                | Token::FracClose
                | Token::SqrtClose
                | Token::Close(_)
                | Token::EnlargedFence(_)
                | Token::TableRowStart
                | Token::Space
                | Token::Grade1PassageEnd
        )
    ) {
        return Ok((input, vec![]));
    }

    if let Some(Token::Op(s)) = input.first() {
        let limit = matches!(
            input.get(1),
            Some(Token::Above | Token::Below | Token::SimpleOver(_) | Token::SimpleUnder(_))
        );
        let large_op_script = matches!(
            input.get(1),
            Some(Token::LevelUp | Token::LevelDown)
        ) && op_takes_post_scripts(s);
        if limit || large_op_script {
            let (input, atom) = parse_scripted(input)?;
            return Ok((input, vec![atom]));
        }
        let op = Expr::Operator(s.clone());
        return Ok((&input[1..], vec![op]));
    }

    if let Some(Token::BrailleText(s)) = input.first() {
        return Ok((&input[1..], vec![Expr::Text(s.clone())]));
    }

    let (input, atom) = match parse_scripted(input) {
        Ok(x) => x,
        Err(_) => return Ok((input, vec![])),
    };
    if matches!(input.first(), Some(Token::FracLine)) {
        let input = &input[1..];
        let (input, den) = parse_scripted_or_missing(input);
        return Ok((
            input,
            vec![Expr::BinomFrac(Box::new(atom), Box::new(den))],
        ));
    }
    Ok((input, vec![atom]))
}

/// If a fenced body is only space-separated cells (2+), treat as a one-row matrix.
fn matrix_cells_from_spaced_body(body: &Expr) -> Option<Vec<Expr>> {
    let parts = match body {
        Expr::Row(parts) => parts.as_slice(),
        _ => return None,
    };
    if !parts.iter().any(|p| matches!(p, Expr::Space)) {
        return None;
    }
    let mut cells = Vec::new();
    let mut cur = Vec::new();
    for p in parts {
        if matches!(p, Expr::Space) {
            if !cur.is_empty() {
                cells.push(Expr::row(std::mem::take(&mut cur)));
            }
        } else {
            cur.push(p.clone());
        }
    }
    if !cur.is_empty() {
        cells.push(Expr::row(cur));
    }
    if cells.len() < 2 {
        return None;
    }
    // Reject set-builder / equations: a cell that is only an operator (e.g. `|`, `=`)
    // is not a matrix entry.
    if cells.iter().any(|c| matches!(c, Expr::Operator(_))) {
        return None;
    }
    // Spaced words in ordinary parentheses (`(A and B)`) are not a matrix row.
    // Require at least one numeric / structural cell.
    if !cells.iter().any(cell_looks_matrix_entry) {
        return None;
    }
    Some(cells)
}

fn cell_looks_matrix_entry(cell: &Expr) -> bool {
    match cell {
        Expr::Number(_) | Expr::Frac(_, _) | Expr::BinomFrac(_, _) => true,
        Expr::Row(parts) => parts.iter().any(cell_looks_matrix_entry),
        Expr::Identifier(_) | Expr::Text(_) | Expr::Space | Expr::Operator(_) => false,
        // Scripted / fenced / radical cells still count as matrix entries.
        _ => true,
    }
}

fn matching_close(open: char) -> char {
    match open {
        '(' => ')',
        '[' => ']',
        '{' => '}',
        '|' => '|',
        '⟦' => '⟧',
        c => c,
    }
}

// --- tests ------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    fn strip_mrow_math(s: &str) -> String {
        s.replace("<mrow>", "")
            .replace("</mrow>", "")
            .replace("<math>", "")
            .replace("</math>", "")
    }

    /// Loose check: canonicalized-ish — ignore outer mrow wrapping differences.
    fn assert_ueb(braille: &str, expect_contains: &[&str]) {
        let mml = Braille_to_MathML(braille, "UEB").unwrap_or_else(|e| panic!("parse failed for {braille:?}: {e}"));
        for frag in expect_contains {
            assert!(
                mml.contains(frag),
                "expected {:?} in MathML\nbraille: {braille}\ngot: {mml}",
                frag
            );
        }
    }

    #[test]
    fn bana_5_1_simple_equation() {
        // x + y = 6
        let mml = Braille_to_MathML("⠭⠐⠖⠽⠀⠐⠶⠀⠼⠋", "UEB").unwrap();
        assert!(mml.contains("<mi>x</mi>"));
        assert!(mml.contains("<mo>+</mo>") || mml.contains("<mo>&#x2B;</mo>"));
        assert!(mml.contains("<mi>y</mi>"));
        assert!(mml.contains("<mo>=</mo>") || mml.contains("&#x3D;"));
        assert!(mml.contains("<mn>6</mn>"));
    }

    #[test]
    fn expr_3_1_1() {
        // 3 + 5 = 8
        assert_ueb("⠼⠉⠐⠖⠼⠑⠀⠐⠶⠀⠼⠓", &["<mn>3</mn>", "<mn>5</mn>", "<mn>8</mn>"]);
    }

    #[test]
    fn expr_3_1_2() {
        assert_ueb("⠼⠓⠐⠤⠼⠑⠀⠐⠶⠀⠼⠉", &["<mn>8</mn>", "<mn>5</mn>", "<mn>3</mn>"]);
    }

    #[test]
    fn expr_3_1_3_times() {
        assert_ueb(
            "⠼⠉⠐⠦⠼⠑⠀⠐⠶⠀⠼⠑⠐⠦⠼⠉⠀⠐⠶⠀⠼⠁⠑",
            &["<mn>3</mn>", "<mn>15</mn>"],
        );
    }

    #[test]
    fn simple_numeric_fraction() {
        // 5/8
        let mml = Braille_to_MathML("⠼⠑⠌⠓", "UEB").unwrap();
        assert!(mml.contains("<mfrac>"), "{mml}");
        assert!(mml.contains("<mn>5</mn>"), "{mml}");
        assert!(mml.contains("<mn>8</mn>"), "{mml}");
    }

    #[test]
    fn mixed_number() {
        // 2½
        let mml = Braille_to_MathML("⠼⠃⠼⠁⠌⠃", "UEB").unwrap();
        assert!(mml.contains("<mn>2</mn>"), "{mml}");
        assert!(mml.contains("<mfrac>"), "{mml}");
    }

    #[test]
    fn general_fraction_a_over_b() {
        // ⠰⠰⠷⠁⠨⠌⠃⠾
        let mml = Braille_to_MathML("⠰⠰⠷⠁⠨⠌⠃⠾", "UEB").unwrap();
        assert!(mml.contains("<mfrac>"), "{mml}");
        assert!(mml.contains("<mi>a</mi>"), "{mml}");
        assert!(mml.contains("<mi>b</mi>"), "{mml}");
    }

    #[test]
    fn bana_5_3_two_fractions() {
        let mml = Braille_to_MathML("⠰⠰⠷⠁⠨⠌⠃⠾⠐⠖⠷⠉⠨⠌⠙⠾", "UEB").unwrap();
        assert_eq!(mml.matches("<mfrac>").count(), 2);
        assert!(mml.contains("<mi>a</mi>") && mml.contains("<mi>d</mi>"));
    }

    #[test]
    fn superscript_x2() {
        // x² — ⠭⠰⠔⠼⠃
        let mml = Braille_to_MathML("⠭⠰⠔⠼⠃", "UEB").unwrap();
        assert!(mml.contains("<msup>"), "{mml}");
        assert!(mml.contains("<mi>x</mi>"), "{mml}");
        assert!(mml.contains("<mn>2</mn>"), "{mml}");
    }

    #[test]
    fn blackboard_n_and_partial() {
        // ℕ (DoubleStruck default ⠈) and ∂
        let n = Braille_to_MathML("⠈⠠⠝", "UEB").unwrap();
        assert!(n.contains("ℕ") || n.contains("&#x2115;"), "{n}");
        let p = Braille_to_MathML("⠈⠙", "UEB").unwrap();
        assert!(p.contains("∂") || p.contains("&#x2202;"), "{p}");
    }

    #[test]
    fn bana_5_2_x2_plus_y2() {
        let mml = Braille_to_MathML("⠭⠰⠔⠼⠃⠐⠖⠽⠔⠼⠃⠀⠐⠶⠀⠰⠠⠉", "UEB").unwrap();
        assert!(mml.contains("<msup>"), "{mml}");
        assert!(mml.contains("<mi>C</mi>") || mml.contains("<mi>c</mi>"), "{mml}");
    }

    #[test]
    fn grade1_passage_with_scripts() {
        // ⠰⠰⠰⠁⠔⠝⠐⠦⠁⠔⠍⠀⠐⠶⠀⠁⠔⠣⠝⠐⠖⠍⠜⠰⠄
        let mml = Braille_to_MathML("⠰⠰⠰⠁⠔⠝⠐⠦⠁⠔⠍⠀⠐⠶⠀⠁⠔⠣⠝⠐⠖⠍⠜⠰⠄", "UEB").unwrap();
        assert!(mml.contains("<msup>"), "{mml}");
        assert!(mml.contains("<mi>a</mi>"), "{mml}");
        assert!(mml.contains("<mi>n</mi>"), "{mml}");
    }

    #[test]
    fn sqrt_simple() {
        // ⠰⠰⠩⠐⠣⠽⠐⠤⠭⠔⠼⠃⠐⠜⠬
        let mml = Braille_to_MathML("⠰⠰⠩⠐⠣⠽⠐⠤⠭⠔⠼⠃⠐⠜⠬", "UEB").unwrap();
        assert!(mml.contains("<msqrt>"), "{mml}");
        assert!(mml.contains("<mi>y</mi>"), "{mml}");
    }

    #[test]
    fn capitals_word() {
        let mml = Braille_to_MathML("⠠⠠⠁⠃⠉⠙", "UEB").unwrap();
        assert!(
            mml.contains("<mi>ABCD</mi>") || (mml.contains("A") && mml.contains("D")),
            "{mml}"
        );
    }

    #[test]
    fn greek_theta_pi() {
        // 0 ≤ θ ≤ 2π
        let mml = Braille_to_MathML("⠼⠚⠀⠸⠈⠣⠀⠨⠹⠀⠸⠈⠣⠀⠼⠃⠨⠏", "UEB").unwrap();
        assert!(mml.contains("θ") || mml.contains("&#x3B8;"), "{mml}");
        assert!(mml.contains("π") || mml.contains("&#x3C0;"), "{mml}");
    }

    #[test]
    fn parentheses() {
        let mml = Braille_to_MathML("⠐⠣⠼⠁⠐⠖⠼⠃⠐⠜", "UEB").unwrap();
        assert!(mml.contains("<mo>(</mo>") || mml.contains("("), "{mml}");
        assert!(mml.contains("<mn>1</mn>") && mml.contains("<mn>2</mn>"), "{mml}");
    }

    #[test]
    fn trailing_incomplete_prefix_as_mtext() {
        // a+b⠐  — lone dots-5 after an expression (prefix of + etc.)
        let mml = Braille_to_MathML("⠁⠐⠖⠃⠐", "UEB").unwrap();
        let flat = strip_mrow_math(&mml);
        assert!(flat.contains("<mi>a</mi>"), "{mml}");
        assert!(flat.contains("<mo>+</mo>") || flat.contains("+"), "{mml}");
        assert!(flat.contains("<mi>b</mi>"), "{mml}");
        assert!(
            flat.contains("<mtext>&#x2810;</mtext>") || flat.contains("<mtext>⠐</mtext>"),
            "{mml}"
        );
    }

    #[test]
    fn bare_open_paren() {
        let mml = Braille_to_MathML("⠐⠣", "UEB").unwrap();
        assert_eq!(mml, "<math><mo>(</mo></math>");
    }

    #[test]
    fn stray_close_paren_after_expr() {
        // 2a+b)
        let mml = Braille_to_MathML("⠼⠃⠰⠁⠐⠖⠃⠐⠜", "UEB").unwrap();
        let flat = strip_mrow_math(&mml);
        assert!(flat.contains("<mn>2</mn>"), "{mml}");
        assert!(flat.contains("<mi>a</mi>"), "{mml}");
        assert!(flat.contains("<mo>+</mo>") || flat.contains("+"), "{mml}");
        assert!(flat.contains("<mi>b</mi>"), "{mml}");
        assert!(flat.ends_with("<mo>)</mo>") || flat.contains("<mo>)</mo>"), "{mml}");
        assert!(!mml.contains("<mtext>"), "{mml}");
    }

    #[test]
    fn vert_bar_unclosed_and_trailing() {
        // bare |
        assert_eq!(Braille_to_MathML("⠸⠳", "UEB").unwrap(), "<math><mo>|</mo></math>");
        // |x  (open, no close)
        let mml = Braille_to_MathML("⠸⠳⠭", "UEB").unwrap();
        assert!(mml.contains("<mo>|</mo>") && mml.contains("<mi>x</mi>"), "{mml}");
        assert!(!mml.contains("FFFD"), "{mml}");
        // 2a+b|  (trailing bar — same cell as open/close)
        let mml = Braille_to_MathML("⠼⠃⠰⠁⠐⠖⠃⠸⠳", "UEB").unwrap();
        let flat = strip_mrow_math(&mml);
        assert!(flat.contains("<mn>2</mn>") && flat.contains("<mi>a</mi>"), "{mml}");
        assert!(flat.contains("<mi>b</mi>") && flat.contains("<mo>|</mo>"), "{mml}");
        assert!(!mml.contains("FFFD") && !mml.contains("<mtext>"), "{mml}");
    }

    #[test]
    fn bare_unclosed_fences() {
        assert_eq!(Braille_to_MathML("⠨⠣", "UEB").unwrap(), "<math><mo>[</mo></math>");
        assert_eq!(Braille_to_MathML("⠸⠣", "UEB").unwrap(), "<math><mo>{</mo></math>");
        assert_eq!(Braille_to_MathML("⠸⠳", "UEB").unwrap(), "<math><mo>|</mo></math>");
        for br in ["⠐⠣⠭", "⠨⠣⠭", "⠸⠣⠭", "⠸⠳⠭"] {
            let mml = Braille_to_MathML(br, "UEB").unwrap();
            assert!(!mml.contains("FFFD"), "{br} -> {mml}");
            assert!(!mml.contains("<mo>)</mo>") && !mml.contains("<mo>]</mo>")
                && !mml.contains("<mo>}</mo>"), "{br} -> {mml}");
        }
        // Enlarged open: table without closer — no �
        let mml = Braille_to_MathML("⠠⠐⠣", "UEB").unwrap();
        assert!(!mml.contains("FFFD"), "{mml}");
        assert!(mml.contains("<mo>(</mo>"), "{mml}");
        // G1 grouping open alone (not a MathML fence): empty body, no �
        let mml = Braille_to_MathML("⠰⠣", "UEB").unwrap();
        assert!(!mml.contains("FFFD"), "{mml}");
    }

    #[test]
    fn decimal() {
        let mml = Braille_to_MathML("⠼⠉⠲⠊", "UEB").unwrap();
        assert!(mml.contains("<mn>3.9</mn>"), "{mml}");
    }

    #[test]
    fn not_equal() {
        let mml = Braille_to_MathML("⠼⠑⠐⠤⠼⠉⠀⠐⠶⠈⠱⠀⠼⠉⠐⠤⠼⠑", "UEB").unwrap();
        assert!(mml.contains("≠") || mml.contains("&#x2260;"), "{mml}");
    }

    #[test]
    fn alg_y_eq_x_plus_4() {
        let mml = Braille_to_MathML("⠰⠽⠀⠐⠶⠀⠭⠐⠖⠼⠙", "UEB").unwrap();
        let flat = strip_mrow_math(&mml);
        assert!(flat.contains("<mi>y</mi>"));
        assert!(flat.contains("<mi>x</mi>"));
        assert!(flat.contains("<mn>4</mn>"));
    }

    #[test]
    fn fraction_y_eq_x_over_2() {
        let mml = Braille_to_MathML("⠰⠰⠰⠽⠀⠐⠶⠀⠷⠭⠨⠌⠼⠃⠾⠰⠄", "UEB").unwrap();
        assert!(mml.contains("<mfrac>"), "{mml}");
        assert!(mml.contains("<mi>x</mi>") && mml.contains("<mn>2</mn>"), "{mml}");
    }

    #[test]
    fn subscript_log() {
        // ⠰⠰⠇⠕⠛⠢⠭⠽  → log_x y
        let mml = Braille_to_MathML("⠰⠰⠇⠕⠛⠢⠭⠽", "UEB").unwrap();
        assert!(mml.contains("<msub>") || mml.contains("log"), "{mml}");
    }

    #[test]
    fn bar_over_x() {
        let mml = Braille_to_MathML("⠰⠰⠭⠱", "UEB").unwrap();
        assert!(mml.contains("<mover>"), "{mml}");
        assert!(mml.contains("<mi>x</mi>"), "{mml}");
    }

    #[test]
    fn arrow_over_x() {
        let mml = Braille_to_MathML("⠭⠰⠘⠱", "UEB").unwrap();
        assert!(mml.contains("<mover>"), "{mml}");
        assert!(mml.contains("→") || mml.contains("&#x2192;"), "{mml}");
    }

    #[test]
    fn arrow_under_x() {
        let mml = Braille_to_MathML("⠭⠰⠠⠘⠱", "UEB").unwrap();
        assert!(mml.contains("<munder>"), "{mml}");
    }

    #[test]
    fn bar_under_grouped() {
        let mml = Braille_to_MathML("⠰⠰⠣⠭⠐⠖⠽⠜⠠⠱", "UEB").unwrap();
        assert!(mml.contains("<munder>"), "{mml}");
        // Grouping must keep the bar under x+y, not only y.
        assert!(
            mml.contains("<munder><mrow><mi>x</mi><mo>+</mo><mi>y</mi></mrow>")
                || mml.contains("<munder><mrow><mi>x</mi><mo>+</mo><mi>y</mi></mrow><mo"),
            "expected under on whole group, got {mml}"
        );
    }

    #[test]
    fn chem_h_plus() {
        let mml = Braille_to_MathML("⠠⠓⠰⠔⠐⠖", "UEB").unwrap();
        assert!(
            mml.contains("<mmultiscripts>") || mml.contains("<msup>"),
            "{mml}"
        );
        assert!(mml.contains("<mi>H</mi>"), "{mml}");
        assert!(mml.contains("<mo>+</mo>"), "{mml}");
    }

    #[test]
    fn chem_ca_oh_2() {
        let mml = Braille_to_MathML("⠰⠰⠠⠉⠁⠐⠣⠠⠕⠠⠓⠐⠜⠢⠼⠃", "UEB").unwrap();
        assert!(mml.contains("<msub>"), "{mml}");
        assert!(mml.contains("Ca") || mml.contains("<mi>C</mi>"), "{mml}");
    }

    #[test]
    fn lim_with_arrow_under() {
        let mml = Braille_to_MathML("⠰⠰⠇⠊⠍⠨⠢⠣⠭⠳⠕⠁⠜⠋⠐⠣⠭⠐⠜⠀⠐⠶⠀⠼⠁", "UEB").unwrap();
        assert!(mml.contains("<munder>") || mml.contains("lim"), "{mml}");
        assert!(mml.contains("→") || mml.contains("&#x2192;"), "{mml}");
    }

    #[test]
    fn questioned_equals_in_equation() {
        // 4 + 5 ≟ 12
        let mml = Braille_to_MathML("⠼⠙⠐⠖⠼⠑⠀⠰⠰⠦⠻⠐⠶⠀⠼⠁⠃", "UEB").unwrap();
        assert!(
            mml.contains("≟") || mml.contains("&#x225F;"),
            "expected ≟ in {mml}"
        );
    }

    #[test]
    fn standalone_arrow() {
        let mml = Braille_to_MathML("⠰⠳⠕", "UEB").unwrap();
        assert!(mml.contains("→") || mml.contains("&#x2192;"), "{mml}");
    }

    #[test]
    fn matrix_2x2_enlarged_parens() {
        // Current UEB_Rules: enlarged open + enlarged close, `⠸⠀` between rows.
        let mml = Braille_to_MathML(
            "⠠⠐⠣⠼⠁⠀⠼⠚⠠⠐⠜⠸⠀⠠⠐⠣⠼⠚⠀⠼⠁⠠⠐⠜",
            "UEB",
        )
        .unwrap();
        assert!(mml.contains("<mtable>"), "{mml}");
        assert!(mml.contains("<mtr>"), "{mml}");
        assert!(mml.contains("<mtd>"), "{mml}");
        assert!(mml.matches("<mtr>").count() == 2, "{mml}");
        assert!(mml.contains("<mo>(</mo>"), "{mml}");
    }

    #[test]
    fn matrix_1x3_spaced_parens() {
        // Single-row matrix also uses enlarged fences under current rules.
        let mml = Braille_to_MathML("⠠⠐⠣⠼⠁⠀⠼⠃⠀⠼⠉⠠⠐⠜", "UEB").unwrap();
        assert!(mml.contains("<mtable>"), "{mml}");
        assert!(mml.matches("<mtd>").count() == 3, "{mml}");
    }

    #[test]
    fn determinant_2x2_enlarged() {
        let mml = Braille_to_MathML(
            "⠠⠸⠳⠁⠀⠃⠠⠸⠳⠸⠀⠠⠸⠳⠉⠀⠙⠠⠸⠳",
            "UEB",
        )
        .unwrap();
        assert!(mml.contains("<mtable>"), "{mml}");
        assert!(mml.contains("<mo>|</mo>"), "{mml}");
    }

    #[test]
    fn unbracketed_mtable_multiple_lines() {
        let mml = Braille_to_MathML(
            "⠰⠭⠀⠐⠶⠀⠋⠐⠣⠞⠐⠜⠸⠀⠰⠽⠀⠐⠶⠀⠛⠐⠣⠞⠐⠜",
            "UEB",
        )
        .unwrap();
        assert!(mml.contains("<mtable>"), "{mml}");
        assert!(!mml.contains("<mrow><mo>(</mo><mtable>"), "{mml}");
        assert_eq!(mml.matches("<mtr>").count(), 2, "{mml}");
        assert!(mml.contains("<mi>x</mi>"), "{mml}");
        assert!(mml.contains("<mi>y</mi>"), "{mml}");
    }

    #[test]
    fn matrix_multiplication_two_tables() {
        // GTM 15.2 second example (linearized): two adjacent bracketed matrices.
        let mml = Braille_to_MathML(
            "⠠⠨⠣⠼⠁⠀⠼⠃⠀⠼⠉⠠⠨⠜⠸⠀⠠⠨⠣⠼⠙⠀⠼⠑⠀⠼⠋⠠⠨⠜⠠⠨⠣⠼⠁⠀⠼⠃⠠⠨⠜⠸⠀⠠⠨⠣⠐⠤⠼⠉⠀⠼⠙⠠⠨⠜⠸⠀⠠⠨⠣⠼⠑⠀⠐⠤⠼⠋⠠⠨⠜",
            "UEB",
        )
        .unwrap();
        assert_eq!(mml.matches("<mtable>").count(), 2, "{mml}");
        assert!(mml.contains("<mo>[</mo>"), "{mml}");
    }
}
