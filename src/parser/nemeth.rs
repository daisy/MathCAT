//! Nemeth Code → MathML parser (BANA 2022).
//!
//! Lexer is mode-aware (numeric indicator vs fraction closer, English-letter
//! indicator vs subscript). Print symbols other than letters/digits come from
//! [`crate::nemeth_symbols`].

#![allow(clippy::needless_return)]
#![allow(non_snake_case)]

use super::Expr;
use crate::errors::Result;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum FracKind {
    Simple,
    Mixed,
    /// Number of `⠠` prefixes (1 = complex, 2 = hypercomplex, …).
    Nested(u8),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ScriptDir {
    Up,
    Down,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum PendingTypeform {
    None,
    /// `⠸⠼…` bold digits
    Bold,
    /// `⠈⠼…` script digits
    Script,
    /// `⠠⠸⠼…` double-struck digits
    DoubleStruck,
    /// `⠠⠨⠼…` sans-serif digits
    SansSerif,
    /// `⠠⠨⠸⠼…` sans-serif bold digits
    SansSerifBold,
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum Token {
    Number {
        text: String,
        /// MathML `mathvariant` when Unicode styled digits are unavailable (script).
        mathvariant: Option<&'static str>,
    },
    Letter {
        ch: char,
        capital: bool,
    },
    Op(String),
    Space,
    Open(char),
    Close(char),
    VertBar,
    /// Superscript/subscript indicator (baseline-relative components).
    Level(Vec<ScriptDir>),
    /// Baseline / multipurpose `⠐` (parser decides which).
    Baseline,
    FracOpen(FracKind),
    FracLine {
        bevelled: bool,
    },
    FracClose(FracKind),
    /// Index of radical `⠣` (before radicand opener).
    RadicalIndex,
    SqrtOpen,
    SqrtClose,
    /// Directly-over indicator `⠣` after a modified expression.
    Above,
    /// Directly-under / binomial line `⠩`.
    Below,
    /// Modifier terminator `⠻` when not closing a radical.
    ModEnd,
    /// Subscript comma (Rule 78) / cancellation opener `⠪`.
    ScriptComma,
    /// Linearized matrix row separator `⣍`.
    TableRowSep,
    /// Enlarged fence (`⠠` + grouping) for linearized matrices.
    EnlargedFence(char),
    Omission,
    /// Capital-word / multi-letter identifier (`⠠⠠II`).
    Word(String),
    BrailleText(String),
}

struct Lexer<'a> {
    rest: &'a str,
    tokens: Vec<Token>,
    frac_stack: Vec<FracKind>,
    frac_saw_line: Vec<bool>,
    radical_depth: u32,
    pending_typeform: PendingTypeform,
}

impl<'a> Lexer<'a> {
    fn new(input: &'a str) -> Self {
        Self {
            rest: input,
            tokens: Vec::new(),
            frac_stack: Vec::new(),
            frac_saw_line: Vec::new(),
            radical_depth: 0,
            pending_typeform: PendingTypeform::None,
        }
    }

    fn eat(&mut self, s: &str) -> bool {
        if self.rest.starts_with(s) {
            self.rest = &self.rest[s.len()..];
            true
        } else {
            false
        }
    }

    fn peek_char(&self) -> Option<char> {
        self.rest.chars().next()
    }

    fn eat_char(&mut self) -> Option<char> {
        let mut chars = self.rest.chars();
        let ch = chars.next()?;
        self.rest = chars.as_str();
        Some(ch)
    }

    fn lex(mut self) -> Result<Vec<Token>> {
        crate::nemeth_symbols::ensure_nemeth_symbols_loaded()?;
        while !self.rest.is_empty() {
            if self.try_space()
                || self.try_structure()
                || self.try_typeform_before_number()
                || self.try_number()
                || self.try_symbol()
                || self.try_letter()
            {
                continue;
            }
            if let Some(ch) = self.eat_char() {
                self.tokens.push(Token::BrailleText(ch.to_string()));
            }
        }
        Ok(self.tokens)
    }

    /// Typeform indicator immediately before a numeric indicator (`⠸⠼`, `⠠⠸⠼`, …).
    fn try_typeform_before_number(&mut self) -> bool {
        if self.pending_typeform != PendingTypeform::None {
            return false;
        }
        // Longest prefixes first.
        let prefixes: &[(&str, PendingTypeform)] = &[
            ("⠠⠨⠸", PendingTypeform::SansSerifBold),
            ("⠠⠸", PendingTypeform::DoubleStruck),
            ("⠠⠨", PendingTypeform::SansSerif),
            ("⠸", PendingTypeform::Bold),
            ("⠈", PendingTypeform::Script),
        ];
        for &(prefix, tf) in prefixes {
            if self.rest.starts_with(prefix) {
                let after = &self.rest[prefix.len()..];
                if after.starts_with('⠼') {
                    self.rest = after;
                    self.pending_typeform = tf;
                    return true;
                }
            }
        }
        false
    }

    fn try_space(&mut self) -> bool {
        if self.eat("⠀") {
            self.tokens.push(Token::Space);
            true
        } else {
            false
        }
    }

    fn try_structure(&mut self) -> bool {
        // Mixed / nested fraction openers and closers (longest first).
        if self.eat("⠸⠹") {
            self.push_frac_open(FracKind::Mixed);
            return true;
        }
        if self.rest.starts_with("⠸⠼") && matches!(self.frac_stack.last(), Some(FracKind::Mixed)) {
            self.eat("⠸⠼");
            self.push_frac_close(FracKind::Mixed);
            return true;
        }
        if self.eat("⠸⠌") {
            self.mark_frac_line();
            self.tokens.push(Token::FracLine { bevelled: true });
            return true;
        }

        // Complex / hypercomplex: one or more ⠠ then ⠹ / ⠌ / ⠼
        if let Some(n) = self.try_nested_frac_marker() {
            return n;
        }

        if self.eat("⠹") {
            self.push_frac_open(FracKind::Simple);
            return true;
        }
        if self.eat("⠌") && !self.frac_stack.is_empty() {
            self.mark_frac_line();
            self.tokens.push(Token::FracLine { bevelled: false });
            return true;
        }
        if self.peek_char() == Some('⠼') && self.can_close_simple_frac() {
            self.eat("⠼");
            self.push_frac_close(FracKind::Simple);
            return true;
        }

        // Nested radicals: ⠨…⠜ / ⠨…⠻
        if let Some(ok) = self.try_nested_radical() {
            return ok;
        }

        // Radicals: ⠜ / ⠻; index ⠣ (unless ⠣ is an over-indicator after an atom)
        if self.eat("⠻") {
            if self.radical_depth > 0 {
                self.radical_depth -= 1;
                self.tokens.push(Token::SqrtClose);
            } else {
                self.tokens.push(Token::ModEnd);
            }
            return true;
        }
        if self.eat("⠣") {
            if self.last_is_atom_like()
                || matches!(
                    self.tokens.last(),
                    Some(Token::Baseline | Token::Above)
                )
            {
                self.tokens.push(Token::Above);
            } else {
                self.tokens.push(Token::RadicalIndex);
            }
            return true;
        }
        if self.eat("⠩") {
            self.tokens.push(Token::Below);
            return true;
        }
        if self.eat("⠜") {
            self.radical_depth += 1;
            self.tokens.push(Token::SqrtOpen);
            return true;
        }

        if self.eat("⣍") {
            self.tokens.push(Token::TableRowSep);
            return true;
        }
        if self.eat("⠪") {
            self.tokens.push(Token::ScriptComma);
            return true;
        }

        // Enlarged fences for linearized matrices
        if self.eat("⠠⠳") {
            self.tokens.push(Token::EnlargedFence('|'));
            return true;
        }
        if self.eat("⠠⠷") {
            self.tokens.push(Token::EnlargedFence('('));
            return true;
        }
        if self.eat("⠠⠾") {
            self.tokens.push(Token::EnlargedFence(')'));
            return true;
        }
        if self.eat("⠠⠈⠷") {
            self.tokens.push(Token::EnlargedFence('['));
            return true;
        }
        if self.eat("⠠⠈⠾") {
            self.tokens.push(Token::EnlargedFence(']'));
            return true;
        }

        // Grouping
        if self.eat("⠈⠷") {
            self.tokens.push(Token::Open('['));
            return true;
        }
        if self.eat("⠈⠾") {
            self.tokens.push(Token::Close(']'));
            return true;
        }
        if self.eat("⠨⠷") {
            self.tokens.push(Token::Open('{'));
            return true;
        }
        if self.eat("⠨⠾") {
            self.tokens.push(Token::Close('}'));
            return true;
        }
        if self.eat("⠷") {
            self.tokens.push(Token::Open('('));
            return true;
        }
        if self.eat("⠾") {
            self.tokens.push(Token::Close(')'));
            return true;
        }
        if self.eat("⠳") {
            self.tokens.push(Token::VertBar);
            return true;
        }

        // Level indicators (runs of ⠘ / ⠰). ⠰+letter after a non-atom is ELI — handled in try_letter.
        if self.peek_char() == Some('⠘') {
            let dirs = self.eat_level_run();
            self.tokens.push(Token::Level(dirs));
            return true;
        }
        if self.peek_char() == Some('⠰') && !self.looks_like_eli() {
            let dirs = self.eat_level_run();
            self.tokens.push(Token::Level(dirs));
            return true;
        }

        if self.eat("⠿") {
            self.tokens.push(Token::Omission);
            return true;
        }
        // ASCII hyphen is omitted from the reverse YAML table; ⠤ is minus/hyphen.
        if self.eat("⠤") {
            self.tokens.push(Token::Op("-".into()));
            return true;
        }
        false
    }

    fn try_nested_frac_marker(&mut self) -> Option<bool> {
        // ⠠⠹ / ⠠⠠⠹ … and matching ⠠⠌ / ⠠⠼
        let mut n = 0u8;
        let mut idx = 0;
        for ch in self.rest.chars() {
            if ch == '⠠' {
                n += 1;
                idx += ch.len_utf8();
            } else {
                break;
            }
        }
        if n == 0 {
            return None;
        }
        let rest = &self.rest[idx..];
        if rest.starts_with('⠹') {
            self.rest = &rest['⠹'.len_utf8()..];
            self.push_frac_open(FracKind::Nested(n));
            return Some(true);
        }
        if rest.starts_with('⠌') && matches!(self.frac_stack.last(), Some(FracKind::Nested(m)) if *m == n)
        {
            self.rest = &rest['⠌'.len_utf8()..];
            self.mark_frac_line();
            self.tokens.push(Token::FracLine { bevelled: false });
            return Some(true);
        }
        if rest.starts_with("⠸⠌")
            && matches!(self.frac_stack.last(), Some(FracKind::Nested(m)) if *m == n)
        {
            self.rest = &rest["⠸⠌".len()..];
            self.mark_frac_line();
            self.tokens.push(Token::FracLine { bevelled: true });
            return Some(true);
        }
        if rest.starts_with('⠼') && matches!(self.frac_stack.last(), Some(FracKind::Nested(m)) if *m == n)
            && self.frac_saw_line.last().copied() == Some(true) {
                self.rest = &rest['⠼'.len_utf8()..];
                self.push_frac_close(FracKind::Nested(n));
                return Some(true);
            }
        None
    }

    fn try_nested_radical(&mut self) -> Option<bool> {
        let mut n = 0u8;
        let mut idx = 0;
        for ch in self.rest.chars() {
            if ch == '⠨' {
                n += 1;
                idx += ch.len_utf8();
            } else {
                break;
            }
        }
        if n == 0 {
            return None;
        }
        let rest = &self.rest[idx..];
        if rest.starts_with('⠜') && self.radical_depth > 0 {
            self.rest = &rest['⠜'.len_utf8()..];
            self.radical_depth += 1;
            self.tokens.push(Token::SqrtOpen);
            return Some(true);
        }
        if rest.starts_with('⠻') && self.radical_depth > 0 {
            self.rest = &rest['⠻'.len_utf8()..];
            self.radical_depth -= 1;
            self.tokens.push(Token::SqrtClose);
            return Some(true);
        }
        if rest.starts_with('⠣') && self.radical_depth > 0 {
            self.rest = &rest['⠣'.len_utf8()..];
            self.tokens.push(Token::RadicalIndex);
            return Some(true);
        }
        None
    }

    fn last_is_atom_like(&self) -> bool {
        matches!(
            self.tokens.last(),
            Some(
                Token::Letter { .. }
                    | Token::Word(_)
                    | Token::Number { .. }
                    | Token::Close(_)
                    | Token::FracClose(_)
                    | Token::SqrtClose
                    | Token::Op(_)
                    | Token::VertBar
            )
        )
    }

    fn push_frac_open(&mut self, kind: FracKind) {
        self.frac_stack.push(kind);
        self.frac_saw_line.push(false);
        self.tokens.push(Token::FracOpen(kind));
    }

    fn push_frac_close(&mut self, kind: FracKind) {
        self.frac_stack.pop();
        self.frac_saw_line.pop();
        self.tokens.push(Token::FracClose(kind));
    }

    fn mark_frac_line(&mut self) {
        if let Some(f) = self.frac_saw_line.last_mut() {
            *f = true;
        }
    }

    fn can_close_simple_frac(&self) -> bool {
        matches!(self.frac_stack.last(), Some(FracKind::Simple))
            && self.frac_saw_line.last().copied() == Some(true)
    }

    fn eat_level_run(&mut self) -> Vec<ScriptDir> {
        let mut dirs = Vec::new();
        loop {
            if self.eat("⠘") {
                dirs.push(ScriptDir::Up);
            } else if self.eat("⠰") {
                dirs.push(ScriptDir::Down);
            } else {
                break;
            }
        }
        dirs
    }

    /// English-letter indicator: ⠰ before a letter, not after a complete atom.
    fn looks_like_eli(&self) -> bool {
        let mut chars = self.rest.chars();
        if chars.next() != Some('⠰') {
            return false;
        }
        let next = chars.next();
        let letter_next = next.is_some_and(|c| braille_cell_to_latin(c).is_some() || c == '⠠');
        if !letter_next {
            return false;
        }
        // Left scripts write ⠰item⠐base; that is not an English-letter indicator.
        if self.looks_like_prescript() {
            return false;
        }
        match self.tokens.last() {
            None | Some(Token::Space) | Some(Token::Open(_))
            | Some(Token::FracOpen(_)) | Some(Token::FracLine { .. }) | Some(Token::Baseline) => {
                true
            }
            Some(Token::Op(s)) if !is_prime_op(s) => true,
            _ => false,
        }
    }

    /// `⠰⠁⠐⠭` / `⠰⠁⠘⠝⠐⠭`: left script(s) then multipurpose then a letter base.
    fn looks_like_prescript(&self) -> bool {
        let mut after_mp = false;
        for c in self.rest.chars() {
            if after_mp {
                return braille_cell_to_latin(c).is_some()
                    || c == '⠠'
                    || c == '⠷'
                    || c == '⠈'
                    || c == '⠨';
            }
            if c == '⠀' {
                return false;
            }
            if c == '⠐' {
                after_mp = true;
            }
        }
        false
    }

    fn try_number(&mut self) -> bool {
        let start_marked = self.rest.starts_with('⠼');
        let unmarked_ok = self.allow_unmarked_digits();
        if !start_marked && !unmarked_ok {
            return false;
        }
        if start_marked {
            // Don't consume ⠼ if it is a simple-fraction closer (already handled).
            self.eat("⠼");
        } else if self.peek_char() == Some('⠨')
            && matches!(self.tokens.last(), Some(Token::Letter { .. } | Token::Word(_)))
        {
            // After a letter, ⠨ is Greek/italic, not an unmarked decimal.
            return false;
        } else if self.peek_char().and_then(braille_cell_to_digit).is_none()
            && self.peek_char() != Some('⠨')
        {
            return false;
        }

        // Unmarked leading ⠨ is a decimal only when a digit cell follows.
        // Otherwise leave ⠨⠡ (°/∘), ⠨⠅ (=), ⠨⠁ (Greek), … for try_symbol.
        if !start_marked && self.peek_char() == Some('⠨') {
            let after_dot = self.rest.chars().nth(1);
            if after_dot.and_then(braille_cell_to_digit).is_none() {
                return false;
            }
        }

        let mut num = String::new();
        // Leading decimal
        if self.eat("⠨") {
            num.push('.');
        }
        let mut got_digit = false;
        loop {
            if let Some(ch) = self.peek_char().and_then(braille_cell_to_digit) {
                self.eat_char();
                num.push(ch);
                got_digit = true;
                continue;
            }
            // Interior comma ⠠ (not ⠠⠀ punctuation comma)
            if self.rest.starts_with('⠠')
                && !self.rest.starts_with("⠠⠀")
                && self.rest['⠠'.len_utf8()..]
                    .chars()
                    .next()
                    .and_then(braille_cell_to_digit)
                    .is_some()
            {
                self.eat("⠠");
                num.push(',');
                continue;
            }
            if self.rest.starts_with('⠨')
                && self.rest.chars().nth(1).and_then(braille_cell_to_digit).is_some()
            {
                self.eat("⠨");
                num.push('.');
                continue;
            }
            break;
        }
        if !got_digit && num != "." {
            // Lone ⠼ with nothing after — treat as braille text / incomplete
            if start_marked && num.is_empty() {
                self.tokens.push(Token::BrailleText("⠼".into()));
                return true;
            }
            return false;
        }
        if num == "." {
            self.tokens.push(Token::Op(".".into()));
            return true;
        }
        let (text, mathvariant) = apply_pending_typeform(num, self.pending_typeform);
        self.pending_typeform = PendingTypeform::None;
        // Rule 9e: after a typeformed digit run, a fresh ⠼ continues the same
        // number in the default typeform (`⠸⠼⠲⠼⠒⠢` → 𝟒35).
        if mathvariant.is_none()
            && text.chars().all(|c| c.is_ascii_digit() || c == '.' || c == ',')
            && let Some(Token::Number {
                text: prev,
                mathvariant: prev_var,
            }) = self.tokens.last_mut()
            {
                let styled = prev_var.is_some()
                    || prev.chars().any(|c| {
                        let o = c as u32;
                        (0x1D7CE..=0x1D7FF).contains(&o)
                    });
                if styled {
                    prev.push_str(&text);
                    return true;
                }
            }
        self.tokens.push(Token::Number { text, mathvariant });
        true
    }

    fn allow_unmarked_digits(&self) -> bool {
        if self.peek_char().and_then(braille_cell_to_digit).is_none()
            && self.peek_char() != Some('⠨')
        {
            return false;
        }
        match self.tokens.last() {
            Some(Token::FracOpen(_)) | Some(Token::FracLine { .. }) | Some(Token::Open(_))
            | Some(Token::Level(_)) | Some(Token::Letter { .. }) | Some(Token::Word(_))
            | Some(Token::RadicalIndex)
            | Some(Token::SqrtOpen) | Some(Token::EnlargedFence(_)) | Some(Token::TableRowSep)
            | Some(Token::Above) | Some(Token::Below) => true,
            Some(Token::Op(s)) if !is_prime_op(s) => true,
            Some(Token::Baseline) => true,
            Some(Token::Space) => {
                // Enclosed list: space after comma inside fences
                matches!(
                    self.tokens.iter().rev().nth(1),
                    Some(Token::Op(s)) if s == ","
                )
            }
            _ => false,
        }
    }

    fn try_symbol(&mut self) -> bool {
        // Baseline ⠐ is 1 cell; YAML may have longer ⠐… operators (e.g. ⠐⠅ = <).
        if let Some((braille, print)) = crate::nemeth_symbols::match_nemeth_symbol(self.rest) {
            if should_leave_for_lexer(&braille, self.rest) {
                // Fall through.
            } else {
            // Don't steal ⠐ alone if YAML mapped a space/empty; require non-empty print.
            if braille == "⠐" {
                // Fall through to baseline unless YAML has a 1-cell ⠐ operator.
                // Equals-like ops are 2+ cells.
                if print.chars().all(|c| c.is_whitespace()) {
                    return false;
                }
            }
            self.rest = &self.rest[braille.len()..];
            if print == "√" {
                self.radical_depth += 1;
                self.tokens.push(Token::SqrtOpen);
                return true;
            }
            if is_identifier_symbol(&print) {
                let mut chars = print.chars();
                if let Some(ch) = chars.next() {
                    self.tokens.push(Token::Letter {
                        ch,
                        capital: ch.is_uppercase(),
                    });
                    return true;
                }
            }
            if print == "|" {
                self.tokens.push(Token::VertBar);
                return true;
            }
            if print == "(" {
                self.tokens.push(Token::Open('('));
                return true;
            }
            if print == ")" {
                self.tokens.push(Token::Close(')'));
                return true;
            }
            if is_quote_char(&print) {
                self.tokens.push(Token::BrailleText(print));
                return true;
            }
            self.tokens.push(Token::Op(print));
            return true;
            }
        }
        // Open quote ⠦ is also digit 8; YAML skips the cell so numeric mode can own it.
        if self.peek_char() == Some('⠦') {
            self.eat("⠦");
            self.tokens.push(Token::BrailleText("“".into()));
            return true;
        }
        if self.eat("⠐") {
            self.tokens.push(Token::Baseline);
            return true;
        }
        false
    }

    fn try_letter(&mut self) -> bool {
        if self.rest.starts_with('⠰') {
            let after = &self.rest['⠰'.len_utf8()..];
            let next = after.chars().next();
            if next == Some('⠠') || next.and_then(braille_cell_to_latin).is_some() {
                self.eat("⠰");
            } else {
                return false;
            }
        }
        // Capital word indicator ⠠⠠ + two or more Latin letters (not Hebrew ⠠⠠ + one).
        if self.rest.starts_with("⠠⠠") && !self.rest.starts_with("⠠⠠⠠") {
            let after = &self.rest["⠠⠠".len()..];
            let mut word = String::new();
            let mut consumed = 0usize;
            for c in after.chars() {
                if let Some(latin) = braille_cell_to_latin(c) {
                    word.push(latin.to_ascii_uppercase());
                    consumed += c.len_utf8();
                } else {
                    break;
                }
            }
            if word.chars().count() >= 2 {
                self.eat("⠠⠠");
                self.rest = &self.rest[consumed..];
                self.tokens.push(Token::Word(word));
                return true;
            }
        }
        let capital = if self.rest.starts_with('⠠') {
            let after = &self.rest['⠠'.len_utf8()..];
            if after.chars().next().and_then(braille_cell_to_latin).is_some() {
                self.eat("⠠");
                true
            } else {
                false
            }
        } else {
            false
        };
        let Some(cell) = self.peek_char() else {
            return false;
        };
        let Some(latin) = braille_cell_to_latin(cell) else {
            return false;
        };
        self.eat_char();
        let ch = if capital {
            latin.to_ascii_uppercase()
        } else {
            latin
        };
        self.tokens.push(Token::Letter { ch, capital });
        true
    }
}

fn braille_cell_to_digit(cell: char) -> Option<char> {
    Some(match cell {
        '⠴' => '0',
        '⠂' => '1',
        '⠆' => '2',
        '⠒' => '3',
        '⠲' => '4',
        '⠢' => '5',
        '⠖' => '6',
        '⠶' => '7',
        '⠦' => '8',
        '⠔' => '9',
        _ => return None,
    })
}

fn apply_pending_typeform(
    ascii_num: String,
    tf: PendingTypeform,
) -> (String, Option<&'static str>) {
    match tf {
        PendingTypeform::None => (ascii_num, None),
        PendingTypeform::Script => (ascii_num, Some("script")),
        PendingTypeform::Bold => (map_digits(&ascii_num, |d| 0x1D7CE + d), None),
        PendingTypeform::DoubleStruck => (map_digits(&ascii_num, |d| 0x1D7D8 + d), None),
        PendingTypeform::SansSerif => (map_digits(&ascii_num, |d| 0x1D7E2 + d), None),
        PendingTypeform::SansSerifBold => (map_digits(&ascii_num, |d| 0x1D7EC + d), None),
    }
}

fn map_digits(s: &str, style: impl Fn(u32) -> u32) -> String {
    s.chars()
        .map(|c| {
            if c.is_ascii_digit() {
                char::from_u32(style(c as u32 - '0' as u32)).unwrap_or(c)
            } else {
                c
            }
        })
        .collect()
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

fn is_identifier_symbol(s: &str) -> bool {
    let mut chars = s.chars();
    match (chars.next(), chars.next()) {
        (Some(c), None) => c.is_alphabetic(),
        _ => false,
    }
}

fn is_prime_op(s: &str) -> bool {
    matches!(s, "'" | "′" | "″" | "‴" | "⁗")
}

fn is_quote_char(s: &str) -> bool {
    matches!(s, "“" | "”" | "\"")
}

/// YAML matches that the mode-aware lexer must keep.
fn should_leave_for_lexer(braille: &str, rest: &str) -> bool {
    // ⠐ + digit / decimal / plus is multipurpose, not a ⠐-prefixed operator.
    if braille.starts_with('⠐') && braille.chars().count() > 1
        && let Some(c) = braille.chars().nth(1)
            && (braille_cell_to_digit(c).is_some() || c == '⠨' || c == '⠬') {
                return true;
            }
    // ⠠⠠ + one letter with more letters after is a capital word, not Hebrew.
    if braille.starts_with("⠠⠠") && braille.chars().count() == 3 {
        let after = rest.get(braille.len()..).unwrap_or("");
        if after.chars().next().and_then(braille_cell_to_latin).is_some() {
            return true;
        }
    }
    false
}

fn tokenize(input: &str) -> Result<Vec<Token>> {
    Lexer::new(input).lex()
}

pub(super) fn braille_to_expr(braille: &str) -> Result<Expr> {
    let tokens = tokenize(braille)?;
    parse_tokens(&tokens)
}

type Toks<'a> = &'a [Token];

fn parse_tokens(tokens: &[Token]) -> Result<Expr> {
    if tokens.is_empty() {
        return Ok(Expr::Row(vec![]));
    }
    if let Some(expr) = try_parse_matrix(tokens) {
        return Ok(expr);
    }
    let (rest, expr) = parse_expr(tokens)?;
    if rest.iter().all(|t| matches!(t, Token::Space)) {
        return Ok(expr);
    }
    let mut parts = match expr {
        Expr::Row(p) => p,
        e => vec![e],
    };
    let (_, extra) = parse_expr(rest).unwrap_or((&[], Expr::Row(vec![])));
    match extra {
        Expr::Row(p) => parts.extend(p),
        e => parts.push(e),
    }
    Ok(Expr::row(parts))
}

fn parse_expr(input: Toks<'_>) -> Result<(Toks<'_>, Expr)> {
    let mut input = skip_space(input);
    let mut parts = Vec::new();
    while !input.is_empty() {
        if is_stop_token(input.first()) {
            break;
        }
        match input.first() {
            Some(Token::Space) => {
                input = &input[1..];
                parts.push(Expr::Space);
            }
            Some(Token::Baseline) if looks_like_modifier_start(input) => {
                let (rest, e) = parse_modified(input)?;
                input = rest;
                parts.push(e);
            }
            Some(Token::Below) => {
                input = &input[1..];
                if let Some(left) = parts.pop() {
                    let (rest, right) = parse_modifier_material(input);
                    input = rest;
                    if is_simple_bar(&right)
                        && !matches!(input.first(), Some(Token::Above | Token::ModEnd))
                    {
                        parts.push(Expr::Under(Box::new(left), Box::new(right)));
                    } else {
                        parts.push(Expr::BinomFrac(Box::new(left), Box::new(right)));
                    }
                }
            }
            Some(Token::Above) => {
                // Contracted overscript on the preceding item (no leading ⠐).
                input = &input[1..];
                let (rest, m) = parse_modifier_material(input);
                input = rest;
                if let Some(left) = parts.pop() {
                    parts.push(Expr::Over(Box::new(left), Box::new(m)));
                } else {
                    parts.push(Expr::Over(Box::new(Expr::Missing), Box::new(m)));
                }
            }
            Some(Token::ModEnd) => {
                // Stray closer from a chemistry bond / unopened modifier.
                input = &input[1..];
            }
            Some(Token::ScriptComma) => {
                input = &input[1..];
                parts.push(Expr::Operator(",".into()));
            }
            _ => {
                let (rest, part) = parse_scripted(input)?;
                input = rest;
                parts.push(part);
            }
        }
    }
    Ok((input, Expr::row(parts)))
}

fn looks_like_modifier_start(input: Toks<'_>) -> bool {
    if !matches!(input.first(), Some(Token::Baseline)) {
        return false;
    }
    // ⠐ + digit is multipurpose juxtaposition, not Rule 15.
    if matches!(input.get(1), Some(Token::Number { .. })) {
        return false;
    }
    for t in input.iter().skip(1) {
        match t {
            Token::Above | Token::Below | Token::ModEnd => return true,
            Token::Baseline | Token::Space => return false,
            _ => {}
        }
    }
    false
}

fn parse_modified(input: Toks<'_>) -> Result<(Toks<'_>, Expr)> {
    let mut input = &input[1..]; // consume ⠐
    let mut parts = Vec::new();
    let mut under: Option<Expr> = None;
    let mut over: Option<Expr> = None;
    while !input.is_empty() {
        if is_stop_token(input.first()) {
            break;
        }
        match input.first() {
            Some(Token::Above) => {
                while matches!(input.first(), Some(Token::Above)) {
                    input = &input[1..];
                }
                let (rest, m) = parse_modifier_material(input);
                input = rest;
                over = Some(match over.take() {
                    Some(prev) => Expr::Over(Box::new(prev), Box::new(m)),
                    None => m,
                });
            }
            Some(Token::Below) => {
                while matches!(input.first(), Some(Token::Below)) {
                    input = &input[1..];
                }
                let (rest, m) = parse_modifier_material(input);
                input = rest;
                under = Some(match under.take() {
                    Some(prev) => Expr::Under(Box::new(prev), Box::new(m)),
                    None => m,
                });
            }
            Some(Token::ModEnd) => {
                input = &input[1..];
                break;
            }
            Some(Token::Baseline) => {
                // Extra ⠐ after a scripted base before ⠣/⠩ (Rule 15).
                input = &input[1..];
            }
            Some(Token::Space) => break,
            _ => {
                let (rest, part) = parse_scripted(input)?;
                input = rest;
                parts.push(part);
            }
        }
    }
    let expr = Expr::row(parts);
    let expr = match (under, over) {
        (Some(u), Some(o)) => Expr::UnderOver(Box::new(expr), Box::new(u), Box::new(o)),
        (Some(u), None) => Expr::Under(Box::new(expr), Box::new(u)),
        (None, Some(o)) => Expr::Over(Box::new(expr), Box::new(o)),
        (None, None) => expr,
    };
    Ok((input, expr))
}

fn parse_modifier_material(input: Toks<'_>) -> (Toks<'_>, Expr) {
    let mut input = input;
    let mut parts = Vec::new();
    while !input.is_empty() {
        match input.first() {
            Some(Token::ModEnd | Token::Above | Token::Below) | None => break,
            t if is_stop_token(t) => break,
            Some(Token::Space) => {
                input = &input[1..];
                parts.push(Expr::Space);
            }
            _ => match parse_scripted(input) {
                Ok((rest, e)) => {
                    input = rest;
                    parts.push(e);
                }
                Err(_) => break,
            },
        }
    }
    (input, Expr::row(parts))
}

fn is_simple_bar(e: &Expr) -> bool {
    matches!(e, Expr::Operator(s) if s == "¯" || s == "_" || s == "\u{AF}" || s == "‾")
}

fn skip_space(input: Toks<'_>) -> Toks<'_> {
    input
}

fn is_stop_token(t: Option<&Token>) -> bool {
    matches!(
        t,
        Some(
            Token::FracLine { .. }
                | Token::FracClose(_)
                | Token::SqrtClose
                | Token::Close(_)
                | Token::TableRowSep
                | Token::EnlargedFence(_)
        )
    )
}

fn parse_scripted(input: Toks<'_>) -> Result<(Toks<'_>, Expr)> {
    let mut input = input;
    let mut pre_sub = None;
    let mut pre_sup = None;
    while let Some(Token::Level(dirs)) = input.first() {
        let dirs = dirs.clone();
        input = &input[1..];
        let (rest, script) = parse_item_or_missing(input);
        input = rest;
        apply_level_dirs(&mut pre_sub, &mut pre_sup, &dirs, script);
        if matches!(input.first(), Some(Token::Baseline)) {
            input = &input[1..];
        }
        if input.first().is_some_and(|t| {
            matches!(
                t,
                Token::Letter { .. } | Token::Word(_) | Token::Number { .. } | Token::Open(_) | Token::SqrtOpen
            )
        }) {
            break;
        }
    }

    let (mut input, mut base) = match parse_atom(input) {
        Ok(x) => x,
        Err(_) if pre_sub.is_some() || pre_sup.is_some() => (input, Expr::Missing),
        Err(e) => return Err(e),
    };

    let mut sub: Option<Expr> = None;
    let mut sup: Option<Expr> = None;

    // Primes attach as superscripts (Rule 83).
    while let Some(Token::Op(s)) = input.first() {
        if !is_prime_op(s) {
            break;
        }
        let p = Expr::Operator(s.clone());
        input = &input[1..];
        sup = Some(match sup.take() {
            Some(prev) => Expr::row(vec![prev, p]),
            None => p,
        });
    }

    // Implicit numeric subscript: letter / function / chem element, then unmarked number.
    // A row of letters (FeCl) attaches the digit to the last eligible identifier (Cl₂).
    if let Some(Token::Number { .. }) = input.first()
        && sub.is_none() && eligible_numeric_sub_base(&base) {
            let num_expr = number_token_to_expr(&input[0]);
            input = &input[1..];
            if matches!(&base, Expr::Row(v) if !v.is_empty()) {
                if let Expr::Row(mut parts) = std::mem::replace(&mut base, Expr::Row(vec![])) {
                    let last = parts.pop().expect("eligible row is non-empty");
                    parts.push(Expr::Sub(Box::new(last), Box::new(num_expr)));
                    base = Expr::row(parts);
                }
            } else {
                sub = Some(num_expr);
            }
        }

    loop {
        match input.first() {
            Some(Token::Level(dirs)) => {
                let dirs = dirs.clone();
                input = &input[1..];
                let (rest, script) = parse_script_content(input);
                input = rest;
                apply_level_dirs(&mut sub, &mut sup, &dirs, script);
            }
            Some(Token::Op(s)) if is_prime_op(s) => {
                let p = Expr::Operator(s.clone());
                input = &input[1..];
                sup = Some(match sup.take() {
                    Some(prev) => Expr::row(vec![prev, p]),
                    None => p,
                });
            }
            Some(Token::ScriptComma) if sub.is_some() => {
                // Comma inside an already-open numeric/letter subscript (Rule 78).
                input = &input[1..];
                let (rest, more) = parse_item_or_missing(input);
                input = rest;
                if let Some(prev) = sub.take() {
                    sub = Some(Expr::row(vec![prev, Expr::Operator(",".into()), more]));
                }
            }
            Some(Token::Baseline) => {
                match input.get(1) {
                    Some(Token::Number { text: n, .. }) if sub.is_none() && sup.is_none() => {
                        // Rule 177: ⠐ between letter and digit is juxtaposition, not a subscript.
                        if let Expr::Identifier(s) = &base
                            && !n.starts_with('.') {
                                base = Expr::Identifier(format!("{s}{n}"));
                                input = &input[2..];
                                continue;
                            }
                        if let Expr::Row(parts) = &mut base
                            && let Some(Expr::Identifier(s)) = parts.last_mut()
                                && !n.starts_with('.') {
                                    s.push_str(n);
                                    input = &input[2..];
                                    continue;
                                }
                        input = &input[1..];
                        break;
                    }
                    Some(Token::Level(_)) => {
                        // Staggered scripts: finish this layer, then read the next.
                        base = combine_scripts(base, sub.take(), sup.take());
                        input = &input[1..];
                    }
                    _ => {
                        input = &input[1..];
                        break;
                    }
                }
            }
            Some(Token::Space) => {
                // Space after a simple script → baseline (`⠉⠕⠎⠘⠆⠀⠭`, `⠼⠆⠘⠭⠀⠐⠅`).
                // Space after a nested/complex script stays in that script (Rule 79d).
                let open = match (&sub, &sup) {
                    (_, Some(s)) => Some(s),
                    (Some(s), None) => Some(s),
                    (None, None) => None,
                };
                if open.is_some_and(|s| !is_simple_script(s)) {
                    input = &input[1..];
                    let (rest, more) = parse_script_content(input);
                    input = rest;
                    if let Some(slot) = if sup.is_some() {
                        sup.as_mut()
                    } else {
                        sub.as_mut()
                    } {
                        let prev = std::mem::replace(slot, Expr::Missing);
                        *slot = Expr::row(vec![prev, Expr::Space, more]);
                    }
                } else {
                    break;
                }
            }
            _ => break,
        }
    }

    if pre_sub.is_some() || pre_sup.is_some() {
        base = Expr::Prescripts {
            base: Box::new(base),
            sub: pre_sub.map(Box::new),
            sup: pre_sup.map(Box::new),
        };
    }

    let expr = combine_scripts(base, sub, sup);
    let (input, expr) = fold_simple_over(input, expr);
    Ok((input, expr))
}

fn combine_scripts(base: Expr, sub: Option<Expr>, sup: Option<Expr>) -> Expr {
    match (sub, sup) {
        (None, None) => base,
        (Some(s), None) => Expr::Sub(Box::new(base), Box::new(s)),
        (None, Some(s)) => Expr::Sup(Box::new(base), Box::new(s)),
        (Some(lo), Some(hi)) => Expr::SubSup(Box::new(base), Box::new(lo), Box::new(hi)),
    }
}

fn parse_script_content(input: Toks<'_>) -> (Toks<'_>, Expr) {
    let mut input = input;
    let mut parts = Vec::new();
    loop {
        match input.first() {
            None
            | Some(Token::Level(_))
            | Some(Token::Baseline)
            | Some(Token::FracLine { .. })
            | Some(Token::FracClose(_))
            | Some(Token::SqrtClose)
            | Some(Token::Close(_))
            | Some(Token::ModEnd)
            | Some(Token::Above)
            | Some(Token::Below)
            | Some(Token::TableRowSep)
            | Some(Token::EnlargedFence(_)) => break,
            Some(Token::Space) => {
                // Digit-group spaces stay in the script (Rule 79e). Other spaces
                // are handled by `parse_scripted` (baseline vs continued script).
                if matches!(input.get(1), Some(Token::Number { .. })) {
                    input = &input[1..];
                    parts.push(Expr::Space);
                } else {
                    break;
                }
            }
            Some(Token::ScriptComma) => {
                input = &input[1..];
                parts.push(Expr::Operator(",".into()));
            }
            _ => match parse_atom(input) {
                Ok((rest, e)) => {
                    let stop_after = is_degree_like(&e);
                    input = rest;
                    parts.push(e);
                    // `⠘⠨⠡` is a complete degree superscript; following punct /
                    // atoms are baseline (`⠼⠂⠘⠨⠡⠠⠀⠎⠊⠝`).
                    if stop_after {
                        break;
                    }
                }
                Err(_) => break,
            },
        }
    }
    (input, Expr::row(parts))
}

fn is_degree_like(e: &Expr) -> bool {
    matches!(
        e,
        Expr::Operator(s) if matches!(s.as_str(), "°" | "∘" | "\u{25E6}" | "\u{02DA}")
    )
}

/// Single number / letter / degree — space after these returns to baseline.
/// Nested or multi-part scripts keep going (Rule 79d: `⠑⠘⠉⠕⠎⠘⠘⠆⠀⠭`).
fn is_simple_script(e: &Expr) -> bool {
    match e {
        Expr::Number(_) | Expr::StyledNumber { .. } | Expr::Identifier(_) => true,
        Expr::Operator(_) if is_degree_like(e) => true,
        Expr::Row(v) if v.len() == 1 => is_simple_script(&v[0]),
        Expr::Row(v) if v.is_empty() => true,
        _ => false,
    }
}

fn fold_simple_over(input: Toks<'_>, expr: Expr) -> (Toks<'_>, Expr) {
    match input.first() {
        Some(Token::Op(s)) if s == "¯" || s == "‾" || s == "˙" || s == "^" => {
            (
                &input[1..],
                Expr::Over(Box::new(expr), Box::new(Expr::Operator(s.clone()))),
            )
        }
        _ => (input, expr),
    }
}

fn apply_level_dirs(
    sub: &mut Option<Expr>,
    sup: &mut Option<Expr>,
    dirs: &[ScriptDir],
    script: Expr,
) {
    if dirs.is_empty() {
        return;
    }
    if dirs.len() == 1 {
        match dirs[0] {
            ScriptDir::Up => nest_or_set(sup, script),
            ScriptDir::Down => nest_or_set(sub, script),
        }
        return;
    }
    // ⠘⠘ y after a first-order superscript x → x^y (Rule 14.3, baseline-relative).
    let slot = match dirs[0] {
        ScriptDir::Up => sup,
        ScriptDir::Down => sub,
    };
    let mut acc = slot.take().unwrap_or(Expr::Missing);
    for dir in dirs.iter().skip(1) {
        acc = match dir {
            ScriptDir::Up => Expr::Sup(Box::new(acc), Box::new(script.clone())),
            ScriptDir::Down => Expr::Sub(Box::new(acc), Box::new(script.clone())),
        };
    }
    *slot = Some(acc);
}

fn nest_or_set(slot: &mut Option<Expr>, script: Expr) {
    if let Some(prev) = slot.take() {
        if is_prime_expr(&prev) {
            *slot = Some(Expr::row(vec![prev, script]));
        } else {
            *slot = Some(Expr::Sup(Box::new(prev), Box::new(script)));
        }
    } else {
        *slot = Some(script);
    }
}

fn is_prime_expr(e: &Expr) -> bool {
    match e {
        Expr::Operator(s) if is_prime_op(s) => true,
        Expr::Row(v) if !v.is_empty() && v.iter().all(is_prime_expr) => true,
        _ => false,
    }
}

fn eligible_numeric_sub_base(base: &Expr) -> bool {
    match base {
        Expr::Identifier(s) => identifier_ok_for_numeric_sub(s),
        Expr::Operator(s) if matches!(s.as_str(), "∑" | "∏" | "∫") => true,
        Expr::Row(v) => v.last().is_some_and(eligible_numeric_sub_base),
        _ => false,
    }
}

fn identifier_ok_for_numeric_sub(s: &str) -> bool {
    // Match Nemeth_Rules `numeric-sub` / BaseOkForNumericScript: single letter,
    // function name, or a chemical element (Cl, Na, Fe, …) so Cl₂ is msub not Cl2.
    (s.chars().count() == 1 && s.chars().all(|c| c.is_alphabetic()))
        || is_function_name(s)
        || crate::chemistry::is_element_symbol(s)
}

fn number_token_to_expr(t: &Token) -> Expr {
    match t {
        Token::Number {
            text,
            mathvariant: Some(v),
        } => Expr::StyledNumber {
            text: text.clone(),
            mathvariant: (*v).to_string(),
        },
        Token::Number { text, .. } => Expr::Number(text.clone()),
        _ => unreachable!("number_token_to_expr"),
    }
}

fn parse_item_or_missing(input: Toks<'_>) -> (Toks<'_>, Expr) {
    match parse_atom(input) {
        Ok(x) => x,
        Err(_) => (input, Expr::Missing),
    }
}

fn parse_atom(input: Toks<'_>) -> Result<(Toks<'_>, Expr)> {
    let input = match input {
        [Token::Space, rest @ ..] => rest,
        other => other,
    };
    match input.first() {
        Some(Token::Number { .. }) => Ok((&input[1..], number_token_to_expr(&input[0]))),
        Some(Token::Letter { .. }) => parse_identifier(input),
        Some(Token::Word(s)) => Ok((&input[1..], Expr::Identifier(s.clone()))),
        Some(Token::Op(s)) => Ok((&input[1..], Expr::Operator(s.clone()))),
        Some(Token::Omission) => Ok((&input[1..], Expr::Operator("?".into()))),
        Some(Token::BrailleText(s)) => Ok((&input[1..], Expr::Text(s.clone()))),
        Some(Token::Open(c)) => parse_fenced(input, *c),
        Some(Token::VertBar) => parse_vertbar(input),
        Some(Token::FracOpen(_)) => parse_fraction(input),
        Some(Token::RadicalIndex) => parse_root(input),
        Some(Token::SqrtOpen) => parse_sqrt(input),
        Some(Token::EnlargedFence(c)) if is_open_fence(*c) => parse_enlarged_matrix(input),
        Some(Token::Baseline) => {
            // Multipurpose between atoms — skip (juxtaposition).
            parse_atom(&input[1..])
        }
        Some(Token::ModEnd) | Some(Token::Level(_)) | Some(Token::Above) | Some(Token::Below) => {
            // Soft recovery: leftover structure tokens after an incomplete construct.
            parse_atom(&input[1..])
        }
        None => Err(anyhow::anyhow!("unexpected end of input")),
        Some(other) => Err(anyhow::anyhow!("unexpected token {other:?}")),
    }
}

fn parse_identifier(input: Toks<'_>) -> Result<(Toks<'_>, Expr)> {
    let mut i = 0;
    let mut letters = String::new();
    while let Some(Token::Letter { ch, .. }) = input.get(i) {
        letters.push(*ch);
        i += 1;
    }
    if letters.is_empty() {
        return Err(anyhow::anyhow!("expected letter"));
    }
    let rest = &input[i..];
    if is_function_name(&letters) {
        return Ok((rest, Expr::Identifier(letters)));
    }
    // Split known function prefix (sin x written unspaced: ⠎⠊⠝⠭)
    if let Some(n) = longest_function_prefix(&letters) {
        let func = letters[..n].to_string();
        let rest_letters = letters[n..].to_string();
        if rest_letters.is_empty() {
            return Ok((rest, Expr::Identifier(func)));
        }
        return Ok((
            rest,
            Expr::row(vec![
                Expr::Identifier(func),
                Expr::Identifier(rest_letters),
            ]),
        ));
    }
    if letters.chars().count() == 1 {
        Ok((rest, Expr::Identifier(letters)))
    } else if letters.chars().all(|c| c.is_ascii_lowercase()) {
        // Keep as one mi if it looks like a word/abbrev.
        Ok((rest, Expr::Identifier(letters)))
    } else {
        // Split mixed/capital runs into chem-element tokens (Cl, Na, Fe) or
        // single capitals (C then H), so Cl₂ can take an implicit numeric subscript.
        Ok((rest, Expr::row(split_letter_run(&letters))))
    }
}

/// Greedy chem-style split: longest ASCII element symbol, else one capital, else a
/// lowercase ASCII run. Non-ASCII letters (Greek) stay one identifier per character.
fn split_letter_run(s: &str) -> Vec<Expr> {
    let chars: Vec<char> = s.chars().collect();
    let mut i = 0;
    let mut parts = Vec::new();
    while i < chars.len() {
        if chars[i].is_ascii_uppercase() {
            let mut taken = 1;
            for len in [3, 2] {
                if i + len <= chars.len() {
                    let cand: String = chars[i..i + len].iter().collect();
                    if crate::chemistry::is_element_symbol(&cand) {
                        taken = len;
                        break;
                    }
                }
            }
            parts.push(Expr::Identifier(chars[i..i + taken].iter().collect()));
            i += taken;
        } else if chars[i].is_ascii_lowercase() {
            let start = i;
            while i < chars.len() && chars[i].is_ascii_lowercase() {
                i += 1;
            }
            parts.push(Expr::Identifier(chars[start..i].iter().collect()));
        } else {
            parts.push(Expr::Identifier(chars[i].to_string()));
            i += 1;
        }
    }
    parts
}

const FUNCTION_NAMES: &[&str] = &[
    "arcsin", "arccos", "arctan", "sinh", "cosh", "tanh", "sin", "cos", "tan", "cot", "sec",
    "csc", "log", "ln", "exp", "lim", "max", "min", "gcd", "lcm", "det", "arg",
];

fn is_function_name(s: &str) -> bool {
    FUNCTION_NAMES.contains(&s)
}

fn longest_function_prefix(s: &str) -> Option<usize> {
    let mut best = None;
    for name in FUNCTION_NAMES {
        if s.starts_with(name) && name.len() < s.len() {
            best = Some(best.map_or(name.len(), |b: usize| b.max(name.len())));
        }
    }
    best
}

fn parse_fenced(input: Toks<'_>, open: char) -> Result<(Toks<'_>, Expr)> {
    let input = &input[1..];
    let (input, body) = parse_expr(input)?;
    let (input, close) = match input.first() {
        Some(Token::Close(c)) => (&input[1..], Some(c.to_string())),
        Some(Token::VertBar) if open == '|' => (&input[1..], Some("|".into())),
        _ => (input, None),
    };
    Ok((
        input,
        Expr::Fenced {
            open: open.to_string(),
            close,
            body: Box::new(body),
        },
    ))
}

fn parse_vertbar(input: Toks<'_>) -> Result<(Toks<'_>, Expr)> {
    parse_fenced(input, '|')
}

fn parse_fraction(input: Toks<'_>) -> Result<(Toks<'_>, Expr)> {
    let kind = match input.first() {
        Some(Token::FracOpen(k)) => *k,
        _ => return Err(anyhow::anyhow!("expected fraction open")),
    };
    let input = &input[1..];
    let (input, num) = parse_expr(input)?;
    let (input, bevelled) = match input.first() {
        Some(Token::FracLine { bevelled }) => (&input[1..], *bevelled),
        _ => (input, false),
    };
    let (input, den) = parse_expr(input)?;
    let input = match input.first() {
        Some(Token::FracClose(k)) if *k == kind => &input[1..],
        _ => input, // incomplete
    };
    let _ = bevelled;
    let num = if num.is_empty_row() {
        Expr::Missing
    } else {
        num
    };
    let den = if den.is_empty_row() {
        Expr::Missing
    } else {
        den
    };
    if bevelled {
        return Ok((
            input,
            Expr::row(vec![num, Expr::Operator("/".into()), den]),
        ));
    }
    Ok((input, Expr::Frac(Box::new(num), Box::new(den))))
}

fn parse_sqrt(input: Toks<'_>) -> Result<(Toks<'_>, Expr)> {
    let input = &input[1..];
    let (input, body) = parse_expr(input)?;
    let input = match input.first() {
        Some(Token::SqrtClose) => &input[1..],
        _ => input,
    };
    Ok((input, Expr::Sqrt(Box::new(body))))
}

fn parse_root(input: Toks<'_>) -> Result<(Toks<'_>, Expr)> {
    // ⠣ index ⠜ radicand ⠻
    let mut input = &input[1..];
    let mut index_parts = Vec::new();
    while !input.is_empty() && !matches!(input.first(), Some(Token::SqrtOpen) | Some(Token::SqrtClose)) {
        if is_stop_token(input.first()) {
            break;
        }
        if matches!(input.first(), Some(Token::Space)) {
            input = &input[1..];
            index_parts.push(Expr::Space);
            continue;
        }
        let (rest, part) = parse_scripted(input)?;
        input = rest;
        index_parts.push(part);
    }
    let index = Expr::row(index_parts);
    let input = match input.first() {
        Some(Token::SqrtOpen) => &input[1..],
        _ => input,
    };
    let (input, base) = parse_expr(input)?;
    let input = match input.first() {
        Some(Token::SqrtClose) => &input[1..],
        _ => input,
    };
    Ok((input, Expr::Root(Box::new(index), Box::new(base))))
}

fn is_open_fence(c: char) -> bool {
    matches!(c, '(' | '[' | '{' | '|')
}

fn matching_close(open: char) -> char {
    match open {
        '(' => ')',
        '[' => ']',
        '{' => '}',
        '|' => '|',
        _ => open,
    }
}

fn try_parse_matrix(tokens: &[Token]) -> Option<Expr> {
    if !matches!(tokens.first(), Some(Token::EnlargedFence(c)) if is_open_fence(*c)) {
        return None;
    }
    if !tokens.iter().any(|t| matches!(t, Token::TableRowSep)) {
        return None;
    }
    parse_enlarged_matrix(tokens).ok().map(|(_, e)| e)
}

fn parse_enlarged_matrix(input: Toks<'_>) -> Result<(Toks<'_>, Expr)> {
    let open = match input.first() {
        Some(Token::EnlargedFence(c)) => *c,
        _ => return Err(anyhow::anyhow!("expected enlarged fence")),
    };
    let close_ch = matching_close(open);
    let mut input = &input[1..];
    let mut rows = Vec::new();
    loop {
        let (rest, cells) = parse_matrix_row(input)?;
        input = rest;
        rows.push(cells);
        match input.first() {
            Some(Token::TableRowSep) => input = &input[1..],
            Some(Token::EnlargedFence(c)) if *c == close_ch => {
                input = &input[1..];
                break;
            }
            _ => break,
        }
    }
    Ok((
        input,
        Expr::Table {
            open: Some(open.to_string()),
            close: Some(close_ch.to_string()),
            rows,
        },
    ))
}

fn parse_matrix_row(input: Toks<'_>) -> Result<(Toks<'_>, Vec<Expr>)> {
    let mut input = input;
    let mut cells = Vec::new();
    let mut parts = Vec::new();
    while !input.is_empty() {
        match input.first() {
            Some(Token::TableRowSep) | Some(Token::EnlargedFence(_)) => break,
            Some(Token::Space) => {
                input = &input[1..];
                if !parts.is_empty() {
                    cells.push(Expr::row(std::mem::take(&mut parts)));
                }
            }
            _ => {
                let (rest, part) = parse_scripted(input)?;
                input = rest;
                parts.push(part);
            }
        }
    }
    if !parts.is_empty() {
        cells.push(Expr::row(parts));
    }
    Ok((input, cells))
}

#[cfg(test)]
mod tests {
    use crate::parser::Braille_to_MathML;

    fn assert_nemeth(braille: &str, expect_contains: &[&str]) {
        let mml = Braille_to_MathML(braille, "Nemeth")
            .unwrap_or_else(|e| panic!("parse failed for {braille:?}: {e}"));
        for frag in expect_contains {
            assert!(
                mml.contains(frag),
                "expected {frag:?} in MathML\nbraille: {braille}\ngot: {mml}"
            );
        }
    }

    #[test]
    fn number_27() {
        assert_nemeth("⠼⠆⠶", &["<mn>27</mn>"]);
    }

    #[test]
    fn minus_one() {
        assert_nemeth("⠤⠼⠂", &["<mn>1</mn>"]);
    }

    #[test]
    fn decimal() {
        assert_nemeth("⠤⠼⠨⠒", &["<mn>.3</mn>"]);
    }

    #[test]
    fn degree_not_decimal() {
        assert_nemeth("⠼⠔⠴⠘⠨⠡", &["<msup>", "<mn>90</mn>", "<mo>&#xB0;</mo>"]);
    }

    #[test]
    fn degree_script_does_not_swallow_equals() {
        // Space after ° returns to baseline before =.
        assert_nemeth(
            "⠼⠔⠴⠘⠨⠡⠀⠨⠅⠀⠼⠂⠦⠴",
            &["<msup>", "<mn>90</mn>", "<mo>&#xB0;</mo>", "<mo>=</mo>", "<mn>180</mn>"],
        );
    }

    #[test]
    fn simple_sup_space_returns_to_baseline() {
        // cos² x — space after numeric superscript is baseline (Rule 79d).
        assert_nemeth(
            "⠉⠕⠎⠘⠆⠀⠭",
            &["<msup>", "<mi>cos</mi>", "<mn>2</mn>", "<mi>x</mi>"],
        );
    }

    #[test]
    fn nested_sup_space_stays_in_script() {
        // e^{cos² x} — space after nested superscript stays in the outer script.
        assert_nemeth(
            "⠑⠘⠉⠕⠎⠘⠘⠆⠀⠭",
            &["<msup>", "<mi>e</mi>", "<msup>", "<mi>cos</mi>", "<mn>2</mn>", "<mi>x</mi>"],
        );
    }

    #[test]
    fn mixed_bold_then_plain_digits() {
        // Rule 9e: ⠸⠼⠲⠼⠒⠢ → single mn 𝟒35.
        assert_nemeth("⠸⠼⠲⠼⠒⠢", &["<mn>&#x1D7D2;35</mn>"]);
    }

    #[test]
    fn equals_after_multipurpose_not_decimal() {
        assert_nemeth("⠐⠨⠅⠣⠸⠦⠻", &["<mover>", "<mo>=</mo>"]);
    }

    #[test]
    fn bold_digits() {
        assert_nemeth("⠸⠼⠒⠲⠢", &["<mn>&#x1D7D1;&#x1D7D2;&#x1D7D3;</mn>"]);
    }

    #[test]
    fn script_digit() {
        assert_nemeth("⠈⠼⠆", &["mathvariant=\"script\"", "<mn", ">2</mn>"]);
    }

    #[test]
    fn double_struck_digit() {
        assert_nemeth("⠠⠸⠼⠂", &["<mn>&#x1D7D9;</mn>"]);
    }

    #[test]
    fn y_equals_2_sin_x() {
        assert_nemeth(
            "⠽⠀⠨⠅⠀⠼⠆⠎⠊⠝⠀⠭",
            &["<mi>y</mi>", "<mo>=</mo>", "<mn>2</mn>", "<mi>sin</mi>", "<mi>x</mi>"],
        );
    }

    #[test]
    fn simple_fraction() {
        assert_nemeth("⠹⠁⠬⠃⠌⠉⠼", &["<mfrac>", "<mi>a</mi>", "<mi>c</mi>"]);
    }

    #[test]
    fn superscript_x2() {
        assert_nemeth("⠭⠘⠆", &["<msup>", "<mi>x</mi>", "<mn>2</mn>"]);
    }

    #[test]
    fn numeric_subscript() {
        assert_nemeth("⠭⠂", &["<msub>", "<mi>x</mi>", "<mn>1</mn>"]);
    }

    #[test]
    fn sqrt_x_plus_y() {
        assert_nemeth("⠜⠭⠬⠽⠻", &["<msqrt>", "<mi>x</mi>", "<mi>y</mi>"]);
    }

    #[test]
    fn parens() {
        assert_nemeth("⠷⠭⠬⠽⠾", &["<mo>(</mo>", "<mi>x</mi>", "<mo>)</mo>"]);
    }

    #[test]
    fn mixed_number() {
        assert_nemeth("⠼⠲⠸⠹⠒⠌⠦⠸⠼", &["<mn>4</mn>", "<mfrac>", "<mn>3</mn>", "<mn>8</mn>"]);
    }

    #[test]
    fn cubed_root() {
        assert_nemeth("⠣⠒⠜⠆⠻", &["<mroot>", "<mn>2</mn>", "<mn>3</mn>"]);
    }

    #[test]
    fn complex_fraction() {
        assert_nemeth("⠠⠹⠹⠒⠌⠦⠼⠠⠌⠢⠠⠼", &["<mfrac>", "<mn>3</mn>", "<mn>8</mn>", "<mn>5</mn>"]);
    }

    #[test]
    fn nested_superscript() {
        assert_nemeth("⠝⠘⠭⠘⠘⠽", &["<msup>", "<mi>n</mi>", "<mi>x</mi>", "<mi>y</mi>"]);
    }

    #[test]
    fn binomial() {
        assert_nemeth("⠷⠝⠩⠅⠾", &["linethickness=\"0\"", "<mi>n</mi>", "<mi>k</mi>"]);
    }

    #[test]
    fn nested_sqrt() {
        assert_nemeth("⠜⠭⠬⠨⠜⠭⠬⠽⠨⠻⠬⠵⠻", &["<msqrt>"]);
    }

    #[test]
    fn overbar_x() {
        assert_nemeth("⠐⠭⠣⠱⠻", &["<mover>", "<mi>x</mi>"]);
    }

    #[test]
    fn capital_h_is_ascii() {
        assert_nemeth("⠠⠓", &["<mi>H</mi>"]);
    }

    #[test]
    fn prime_on_x() {
        assert_nemeth("⠭⠄", &["<msup>", "<mi>x</mi>"]);
    }

    #[test]
    fn implicit_sub_then_sup() {
        assert_nemeth("⠭⠂⠘⠆", &["<msubsup>", "<mi>x</mi>", "<mn>1</mn>", "<mn>2</mn>"]);
    }

    #[test]
    fn implicit_sub_cl2() {
        assert_nemeth("⠠⠉⠇⠆", &["<msub>", "<mi>Cl</mi>", "<mn>2</mn>"]);
    }

    #[test]
    fn chem_element_be_not_split() {
        assert_nemeth("⠠⠃⠑", &["<mi>Be</mi>"]);
    }

    #[test]
    fn implicit_sub_ch2_is_c_then_h2() {
        let mml = Braille_to_MathML("⠠⠉⠠⠓⠆", "Nemeth").unwrap();
        assert!(mml.contains("<mi>C</mi>"), "got {mml}");
        assert!(mml.contains("<msub>"), "got {mml}");
        assert!(mml.contains("<mi>H</mi>"), "got {mml}");
        assert!(mml.contains("<mn>2</mn>"), "got {mml}");
        assert!(!mml.contains("<mi>CH</mi>"), "got {mml}");
    }

    #[test]
    fn implicit_sub_fecl3() {
        let mml = Braille_to_MathML("⠠⠋⠑⠠⠉⠇⠒", "Nemeth").unwrap();
        assert!(mml.contains("<mi>Fe</mi>"), "got {mml}");
        assert!(mml.contains("<msub>"), "got {mml}");
        assert!(mml.contains("<mi>Cl</mi>"), "got {mml}");
        assert!(mml.contains("<mn>3</mn>"), "got {mml}");
    }

    #[test]
    fn multipurpose_not_subscript() {
        assert_nemeth("⠭⠐⠢", &["<mi>x5</mi>"]);
    }

    #[test]
    fn unmarked_digit_after_plus() {
        assert_nemeth("⠷⠭⠂⠬⠂⠾", &["<msub>", "<mi>x</mi>", "<mn>1</mn>", "<mn>1</mn>"]);
    }

    #[test]
    fn greek_alpha_beta() {
        assert_nemeth("⠨⠁⠨⠃", &["<mi>&#x3B1;</mi>", "<mi>&#x3B2;</mi>"]);
    }

    #[test]
    fn baseline_plus_not_in_script() {
        assert_nemeth("⠭⠰⠁⠐⠬⠽⠘⠆", &["<msub>", "<mi>x</mi>", "<mi>a</mi>", "<msup>", "<mi>y</mi>"]);
    }

    #[test]
    fn subscript_comma_in_parens() {
        assert_nemeth("⠭⠰⠷⠁⠪⠃⠾", &["<msub>", "<mi>x</mi>", "<mi>a</mi>", "<mi>b</mi>"]);
    }

    #[test]
    fn matrix_2x2() {
        assert_nemeth("⠠⠳⠼⠂⠀⠼⠆⠀⣍⠤⠒⠀⠤⠼⠲⠠⠳", &["<mtable>", "<mtr>"]);
    }
}
