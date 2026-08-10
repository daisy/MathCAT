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
        close: String,
        body: Box<Expr>,
    },
    /// Bracketed linearized matrix/determinant (`mtable` / `mtr` / `mtd`).
    Table {
        open: String,
        close: String,
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
    Space,
    Text(String),
}

impl Expr {
    fn row(mut parts: Vec<Expr>) -> Expr {
        parts.retain(|e| !matches!(e, Expr::Row(v) if v.is_empty()));
        match parts.len() {
            0 => Expr::Row(vec![]),
            1 => parts.pop().unwrap(),
            _ => Expr::Row(parts),
        }
    }

    pub fn to_mathml(&self) -> String {
        match self {
            Expr::Number(n) => format!("<mn>{}</mn>", xml_escape(n)),
            Expr::Identifier(s) => format!("<mi>{}</mi>", xml_escape(s)),
            Expr::Operator(s) => format!("<mo>{}</mo>", xml_escape(s)),
            Expr::Text(s) => format!("<mtext>{}</mtext>", xml_escape(s)),
            Expr::Space => "<mtext>&#xA0;</mtext>".to_string(),
            Expr::Row(parts) => {
                let inner: String = parts.iter().map(Expr::to_mathml).collect();
                format!("<mrow>{inner}</mrow>")
            }
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
                format!(
                    "<mrow><mo>{}</mo>{}<mo>{}</mo></mrow>",
                    xml_escape(open),
                    body.to_mathml(),
                    xml_escape(close)
                )
            }
            Expr::Table { open, close, rows } => {
                let mut s = format!("<mrow><mo>{}</mo><mtable>", xml_escape(open));
                for row in rows {
                    s.push_str("<mtr>");
                    for cell in row {
                        s.push_str("<mtd>");
                        s.push_str(&cell.to_mathml());
                        s.push_str("</mtd>");
                    }
                    s.push_str("</mtr>");
                }
                s.push_str(&format!(
                    "</mtable><mo>{}</mo></mrow>",
                    xml_escape(close)
                ));
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
            // Unknown / literary leftover
            let ch = self.peek_char().unwrap();
            bail!(
                "UEB lexer: unrecognized braille cell '{}' (U+{:04X}) near {:?}"
                , ch, ch as u32, &self.rest[..self.rest.len().min(12)]
            );
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
        // Fraktur symbol is tracked so the next letter can map to ℜ / etc.
        // Other typeforms need mathvariant (or bold digit chars) — skipped for now.
        if self.eat("⠈⠆") {
            self.pending_typeform = PendingTypeform::Fraktur;
            return Ok(true);
        }
        if self.eat("⠘⠆")
            || self.eat("⠘⠂")
            || self.eat("⠘⠄")
            || self.eat("⠨⠂")
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
                || self.rest.starts_with('⠀');
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
                    // decimal point
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

        match den {
            Some(d) => {
                // Emit as fraction tokens via a Frac AST later — use Number with slash? Better emit dedicated.
                // Push a synthetic frac: we'll use Op and numbers… Simplest: push Number for mixed later.
                // Represent simple numeric fraction as a single special Number "p/q" parsed later? Or Frac tokens.
                self.tokens.push(Token::Number(format!("FRAC:{num}/{d}")));
            }
            None => {
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
    is_blackboard_letter(s)
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
    let (rest, expr) = parse_expr(tokens)?;
    let rest = skip_noise(rest);
    if !rest.is_empty() {
        bail!("UEB parse: trailing tokens: {:?}", &rest[..rest.len().min(5)]);
    }
    Ok(expr)
}

fn skip_noise(input: Toks<'_>) -> Toks<'_> {
    let mut i = input;
    while let Some(Token::Grade1PassageEnd) = i.first() {
        i = &i[1..];
    }
    i
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
        return Ok((input, spaces_before));
    }

    if let Some(Token::Op(s)) = input.first() {
        // Arrows / large ops may take under/over (e.g. Haber process: → with scripts)
        if matches!(
            input.get(1),
            Some(
                Token::Above
                    | Token::Below
                    | Token::LevelUp
                    | Token::LevelDown
                    | Token::SimpleOver(_)
                    | Token::SimpleUnder(_)
            )
        ) {
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

    let (input, atom) = parse_scripted(input)?;
    // Simple / binomial fraction line between items: num ⠻ den → mfrac linethickness=0
    if matches!(input.first(), Some(Token::FracLine)) {
        let input = &input[1..];
        let (input, den) = parse_scripted(input)?;
        let mut v = spaces_before;
        v.push(Expr::BinomFrac(Box::new(atom), Box::new(den)));
        return Ok((input, v));
    }
    let mut v = spaces_before;
    v.push(atom);
    Ok((input, v))
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
                let (rest, script) = parse_item(input)?;
                input = rest;
                pre_sub = Some(script);
            }
            Some(Token::LevelUp) => {
                input = &input[1..];
                let (rest, script) = parse_item(input)?;
                input = rest;
                pre_sup = Some(script);
            }
            _ => break,
        }
    }

    let (mut input, mut base) = parse_atom(input)?;

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
                let (rest, script) = parse_item(input)?;
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
                let (rest, script) = parse_item(input)?;
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
                let (rest, script) = parse_item(input)?;
                input = rest;
                over = Some(normalize_overscript(script));
            }
            Some(Token::Below) => {
                input = &input[1..];
                let (rest, script) = parse_item(input)?;
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
    base = match (sub, sup, first_post_was_sup) {
        // x^2_k written as LevelUp then LevelDown → nested msub(msup) (GTM 7.7.2)
        (Some(sub), Some(sup), Some(true)) => {
            Expr::Sub(Box::new(Expr::Sup(Box::new(base), Box::new(sup))), Box::new(sub))
        }
        (Some(sub), Some(sup), _) => Expr::SubSup(Box::new(base), Box::new(sub), Box::new(sup)),
        (Some(sub), None, _) => Expr::Sub(Box::new(base), Box::new(sub)),
        (None, Some(sup), _) => Expr::Sup(Box::new(base), Box::new(sup)),
        (None, None, _) => base,
    };
    let expr = match (under, over) {
        (Some(u), Some(o)) => Expr::UnderOver(Box::new(base), Box::new(u), Box::new(o)),
        (Some(u), None) => Expr::Under(Box::new(base), Box::new(u)),
        (None, Some(o)) => Expr::Over(Box::new(base), Box::new(o)),
        (None, None) => base,
    };

    // Chemistry / combinatorics left-superscript on a letter base:
    // emit empty-base `<msup>` then the base (with post-scripts), matching MathJax/GTM tests.
    let expr = if pre_sub.is_none() {
        if let Some(pre_sup) = pre_sup {
            let letterish = match &expr {
                Expr::Identifier(_) => true,
                Expr::Sub(b, _) | Expr::Sup(b, _) => matches!(b.as_ref(), Expr::Identifier(_)),
                Expr::SubSup(b, _, _) => matches!(b.as_ref(), Expr::Identifier(_)),
                _ => false,
            };
            if letterish {
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
        _ => parse_atom(input),
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
        Token::EnlargedFence(_) => parse_enlarged_matrix(input),
        Token::VertBar => parse_fenced(input, '|'),
        Token::Open(c) => parse_fenced(input, *c),
        // Operators as atoms (degree, chemistry charge ±, arrows used as items, etc.)
        Token::Op(s) => Ok((&input[1..], Expr::Operator(s.clone()))),
        _ => Err(parse_err(input, "expected atom")),
    }
}

fn parse_identifier(
    input: Toks<'_>,
) -> std::result::Result<(Toks<'_>, Expr), crate::errors::Error> {
    let mut s = String::new();
    let mut i = 0;
    while let Some(Token::Letter { ch, .. }) = input.get(i) {
        // If a modifier / script follows this letter, leave it for parse_scripted
        // so accents bind to the last letter only (GTM 12: ÂBC-style hat on B).
        let next_is_modifier = matches!(
            input.get(i + 1),
            Some(
                Token::LevelUp
                    | Token::LevelDown
                    | Token::Above
                    | Token::Below
                    | Token::SimpleOver(_)
                    | Token::SimpleUnder(_)
            )
        );
        if next_is_modifier {
            if s.is_empty() {
                s.push(*ch);
                i += 1;
            }
            break;
        }
        s.push(*ch);
        i += 1;
    }
    if s.is_empty() {
        return Err(parse_err(input, "expected letter"));
    }
    Ok((&input[i..], Expr::Identifier(s)))
}

fn parse_general_fraction(
    input: Toks<'_>,
) -> std::result::Result<(Toks<'_>, Expr), crate::errors::Error> {
    let input = expect_token(input, &Token::FracOpen)?;
    let (input, num) = parse_until(input, |t| matches!(t, Token::FracLine))?;
    let input = expect_token(input, &Token::FracLine)?;
    let (input, den) = parse_until(input, |t| matches!(t, Token::FracClose))?;
    let input = expect_token(input, &Token::FracClose)?;
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
        let (input, idx) = parse_item(input)?;
        (input, Some(idx))
    } else {
        (input, None)
    };
    let (input, base) = parse_until(input, |t| matches!(t, Token::SqrtClose))?;
    let input = expect_token(input, &Token::SqrtClose)?;
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
            _ => return Err(parse_err(input, "expected braille group close")),
        };
        return Ok((input, body));
    }

    let is_close = |t: &Token| match t {
        Token::Close(c) if *c == close => true,
        Token::VertBar if open == '|' => true,
        _ => false,
    };

    let (input, body) = parse_until(input, is_close)?;
    let input = match input.first() {
        Some(t) if is_close(t) => &input[1..],
        _ => return Err(parse_err(input, "expected close fence")),
    };

    // Single-row linearized matrix: space-separated cells inside ordinary fences
    // (UEB_Rules default-mtr without enlarged markers when there is only one row).
    if let Some(cells) = matrix_cells_from_spaced_body(&body) {
        return Ok((
            input,
            Expr::Table {
                open: open.to_string(),
                close: close.to_string(),
                rows: vec![cells],
            },
        ));
    }

    Ok((
        input,
        Expr::Fenced {
            open: open.to_string(),
            close: close.to_string(),
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
        input = match input.first() {
            Some(Token::EnlargedFence(c)) if *c == close => &input[1..],
            _ => return Err(parse_err(input, "expected enlarged close fence")),
        };

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
            open: open.to_string(),
            close: close.to_string(),
            rows,
        },
    ))
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
                | Token::Space
                | Token::Grade1PassageEnd
        )
    ) {
        return Ok((input, vec![]));
    }

    if let Some(Token::Op(s)) = input.first() {
        if matches!(
            input.get(1),
            Some(
                Token::Above
                    | Token::Below
                    | Token::LevelUp
                    | Token::LevelDown
                    | Token::SimpleOver(_)
                    | Token::SimpleUnder(_)
            )
        ) {
            let (input, atom) = parse_scripted(input)?;
            return Ok((input, vec![atom]));
        }
        let op = Expr::Operator(s.clone());
        return Ok((&input[1..], vec![op]));
    }

    let (input, atom) = parse_scripted(input)?;
    if matches!(input.first(), Some(Token::FracLine)) {
        let input = &input[1..];
        let (input, den) = parse_scripted(input)?;
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
    Some(cells)
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
    }

    #[test]
    fn chem_h_plus() {
        let mml = Braille_to_MathML("⠠⠓⠰⠔⠐⠖", "UEB").unwrap();
        assert!(mml.contains("<msup>"), "{mml}");
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
        let mml = Braille_to_MathML("⠠⠐⠣⠼⠁⠀⠼⠚⠠⠐⠜⠠⠐⠣⠼⠚⠀⠼⠁⠠⠐⠜", "UEB").unwrap();
        assert!(mml.contains("<mtable>"), "{mml}");
        assert!(mml.contains("<mtr>"), "{mml}");
        assert!(mml.contains("<mtd>"), "{mml}");
        assert!(mml.matches("<mtr>").count() == 2, "{mml}");
        assert!(mml.contains("<mo>(</mo>"), "{mml}");
    }

    #[test]
    fn matrix_1x3_spaced_parens() {
        let mml = Braille_to_MathML("⠐⠣⠼⠁⠀⠼⠃⠀⠼⠉⠐⠜", "UEB").unwrap();
        assert!(mml.contains("<mtable>"), "{mml}");
        assert!(mml.matches("<mtd>").count() == 3, "{mml}");
    }

    #[test]
    fn determinant_2x2_enlarged() {
        let mml = Braille_to_MathML("⠠⠸⠳⠼⠁⠀⠼⠃⠠⠸⠳⠠⠸⠳⠼⠉⠀⠼⠙⠠⠸⠳", "UEB").unwrap();
        assert!(mml.contains("<mtable>"), "{mml}");
        assert!(mml.contains("<mo>|</mo>"), "{mml}");
    }
}
