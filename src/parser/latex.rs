//! LaTeX → MathML parser.
//!
//! Accepts the LaTeX MathCAT emits from `Rules/Braille/LaTeX` (commands, short
//! names, `~` letters, `\begin`…`\end` matrices). Incomplete input is kept.
//! A command argument that was not started is an empty slot (`\frac` is an
//! empty fraction). The same is true of a script: `2^` and `2^{` are a
//! superscript with an empty exponent. An opened group keeps what was typed.

#![allow(clippy::needless_return)]

use super::Expr;
use crate::errors::Result;
use crate::latex_symbols::{self, Out};

struct Parser {
    input: String,
    pos: usize,
}

enum Stop {
    Top,
    Group,
    /// Matrix cell: also stop at `&`, `\\`, and `\end`.
    Cell,
    /// `\sqrt[index]`.
    Bracket,
    /// `\left` … `\right`.
    Right,
}

impl Parser {
    fn new(input: &str) -> Self {
        Self {
            input: input.to_string(),
            pos: 0,
        }
    }

    fn rest(&self) -> &str {
        &self.input[self.pos..]
    }

    fn advance(&mut self, n: usize) {
        self.pos += n;
    }

    fn skip_ws(&mut self) {
        let n: usize = self
            .rest()
            .chars()
            .take_while(|c| c.is_whitespace())
            .map(|c| c.len_utf8())
            .sum();
        self.advance(n);
    }

    fn eat(&mut self, s: &str) -> bool {
        if self.rest().starts_with(s) {
            self.advance(s.len());
            true
        } else {
            false
        }
    }

    fn eof(&self) -> bool {
        self.rest().is_empty()
    }

    fn peek(&self) -> Option<char> {
        self.rest().chars().next()
    }

    fn at_row_break(&self) -> bool {
        let rest = self.rest();
        rest.starts_with("\\\\") &&
            rest[2..].chars().next().is_none_or(|c| !c.is_ascii_alphabetic())
    }

    fn at_cmd(&self, name: &str) -> bool {
        let rest = self.rest();
        let head = format!("\\{name}");
        rest.starts_with(&head) &&
            rest[head.len()..].chars().next().is_none_or(|c| !c.is_ascii_alphabetic())
    }

    fn at_stop(&self, stop: &Stop) -> bool {
        match stop {
            Stop::Top => false,
            Stop::Group => self.peek() == Some('}'),
            Stop::Bracket => self.peek() == Some(']'),
            Stop::Right => self.at_cmd("right"),
            Stop::Cell => {
                self.peek() == Some('}') ||
                    self.peek() == Some('&') ||
                    self.at_row_break() ||
                    self.at_cmd("end")
            }
        }
    }

    fn parse_list(&mut self, stop: Stop) -> Vec<Expr> {
        let mut items = Vec::new();
        loop {
            self.skip_ws();
            if self.eof() || self.at_stop(&stop) {
                break;
            }
            if items.is_empty() && matches!(self.peek(), Some('^' | '_')) {
                items.push(self.parse_leading_scripts());
                continue;
            }
            let atom = self.parse_atom();
            // A line break outside a script is not an item. A stray closer is.
            if atom.is_empty_row() {
                continue;
            }
            items.push(self.parse_postscripts(atom));
        }
        items
    }

    fn parse_leading_scripts(&mut self) -> Expr {
        let mut sub = None;
        let mut sup = None;
        loop {
            self.skip_ws();
            if self.eat("_") {
                sub = Some(self.script_body());
            } else if self.eat("^") {
                sup = Some(self.script_body());
            } else {
                break;
            }
        }
        let base = Expr::Row(vec![]);
        match (sub, sup) {
            (Some(sub), Some(sup)) => Expr::SubSup(Box::new(base), Box::new(sub), Box::new(sup)),
            (Some(sub), None) => Expr::Sub(Box::new(base), Box::new(sub)),
            (None, Some(sup)) => Expr::Sup(Box::new(base), Box::new(sup)),
            (None, None) => base,
        }
    }

    fn parse_postscripts(&mut self, mut base: Expr) -> Expr {
        loop {
            self.skip_ws();
            let large = is_large(&base);
            if self.peek() == Some('\'') {
                let mut primes = String::new();
                while self.eat("'") {
                    primes.push('\'');
                }
                base = Expr::Sup(Box::new(base), Box::new(Expr::Operator(primes)));
                continue;
            }
            if self.eat("_") {
                let sub = self.script_body();
                self.skip_ws();
                if self.eat("^") {
                    let sup = self.script_body();
                    base = scripted(base, Some(sub), Some(sup), large);
                } else {
                    base = scripted(base, Some(sub), None, large);
                }
            } else if self.eat("^") {
                let sup = self.script_body();
                self.skip_ws();
                if self.eat("_") {
                    let sub = self.script_body();
                    base = scripted(base, Some(sub), Some(sup), large);
                } else {
                    base = scripted(base, None, Some(sup), large);
                }
            } else {
                break;
            }
        }
        base
    }

    /// The item after `^` or `_`. Nothing typed, or only an opening `{`, is an
    /// empty slot rather than a "missing char" placeholder.
    fn script_body(&mut self) -> Expr {
        self.skip_ws();
        if self.eof() ||
            matches!(self.peek(), Some('}' | '&' | ']')) ||
            self.at_row_break()
        {
            return Expr::Row(vec![]);
        }
        if self.peek() == Some('{') {
            self.group()
        } else {
            self.parse_atom()
        }
    }

    /// A `{...}` argument. A missing `{` is an empty slot (a fraction template),
    /// not a failure. An opened group that never closes keeps what was typed.
    fn group(&mut self) -> Expr {
        self.skip_ws();
        if !self.eat("{") {
            return Expr::Row(vec![]);
        }
        let items = self.parse_list(Stop::Group);
        self.eat("}");
        list(items)
    }

    fn parse_atom(&mut self) -> Expr {
        self.skip_ws();
        if self.eof() {
            return Expr::Row(vec![]);
        }
        // A line break here is outside a matrix cell. Drop it instead of stalling.
        if self.at_row_break() {
            self.advance(2);
            return Expr::Row(vec![]);
        }
        // A closer the current list did not claim. Keep it and move on.
        if matches!(self.peek(), Some('}' | '&' | ']')) {
            let ch = self.peek().unwrap();
            self.advance(ch.len_utf8());
            return Expr::Operator(ch.to_string());
        }
        if self.eat("{") {
            let items = self.parse_list(Stop::Group);
            self.eat("}");
            return list(items);
        }
        if self.rest().starts_with('\\') {
            return self.parse_backslash();
        }
        if self.peek() == Some('.') {
            let mut dots = String::new();
            while self.eat(".") {
                dots.push('.');
            }
            return Expr::Operator(dots);
        }
        if let Some((out, n)) = latex_symbols::match_prefix(self.rest()) {
            self.advance(n);
            return emit(&out);
        }
        if let Some(n) = self.number_len() {
            let text = self.rest()[..n].to_string();
            self.advance(n);
            return Expr::Number(text);
        }
        if let Some(name) = self.match_func() {
            self.advance(name.len());
            return func_expr(name);
        }
        let ch = self.peek().unwrap();
        self.advance(ch.len_utf8());
        if ch.is_alphabetic() {
            Expr::Identifier(ch.to_string())
        } else {
            Expr::Operator(ch.to_string())
        }
    }

    fn number_len(&self) -> Option<usize> {
        let bytes = self.rest().as_bytes();
        if bytes.first().is_none_or(|b| !b.is_ascii_digit()) {
            return None;
        }
        let mut i = 0usize;
        while i < bytes.len() && bytes[i].is_ascii_digit() {
            i += 1;
        }
        // `0,1` is one number. `1, 2` is a list: a space follows the comma.
        while i + 1 < bytes.len() &&
            (bytes[i] == b',' || bytes[i] == b'.') &&
            bytes[i + 1].is_ascii_digit()
        {
            i += 1;
            while i < bytes.len() && bytes[i].is_ascii_digit() {
                i += 1;
            }
        }
        Some(i)
    }

    fn match_func(&self) -> Option<&'static str> {
        let rest = self.rest();
        for name in FUNC_NAMES {
            if rest.starts_with(name) &&
                rest[name.len()..].chars().next().is_none_or(|c| !c.is_ascii_alphabetic())
            {
                return Some(name);
            }
        }
        None
    }

    fn parse_backslash(&mut self) -> Expr {
        if self.at_row_break() {
            return Expr::Missing;
        }
        if let Some((out, n)) = latex_symbols::match_prefix(self.rest()) {
            let name = cmd_name_of(&self.rest()[..n]);
            let after = self.rest()[n..].chars().next();
            if !(is_structural(&name) && matches!(after, Some('{' | '['))) {
                self.advance(n);
                return emit(&out);
            }
        }
        let name = self.read_cmd_name();
        if is_structural(&name) {
            self.parse_structural(&name)
        } else if name.is_empty() {
            Expr::Operator("\\".into())
        } else {
            Expr::Identifier(name)
        }
    }

    fn read_cmd_name(&mut self) -> String {
        if !self.eat("\\") {
            return String::new();
        }
        let mut name = String::new();
        if let Some(c) = self.peek() && c.is_ascii_alphabetic() {
            while let Some(c) = self.peek() && c.is_ascii_alphabetic() {
                name.push(c);
                self.advance(c.len_utf8());
            }
        } else if let Some(c) = self.peek() && c != '\\' {
            name.push(c);
            self.advance(c.len_utf8());
        }
        name
    }

    fn parse_structural(&mut self, name: &str) -> Expr {
        match name {
            "frac" | "f" => {
                let num = self.group();
                let den = self.group();
                Expr::Frac(Box::new(num), Box::new(den))
            }
            "sqrt" | "s" => {
                self.skip_ws();
                if self.eat("[") {
                    let index = list(self.parse_list(Stop::Bracket));
                    self.eat("]");
                    Expr::Root(Box::new(index), Box::new(self.group()))
                } else {
                    Expr::Sqrt(Box::new(self.group()))
                }
            }
            "binom" => {
                let n = self.group();
                let k = self.group();
                Expr::Row(vec![
                    Expr::Operator("(".into()),
                    Expr::BinomFrac(Box::new(n), Box::new(k)),
                    Expr::Operator(")".into()),
                ])
            }
            "overline" | "ol" => Expr::Over(Box::new(self.group()), Box::new(bar())),
            "underline" => Expr::Under(Box::new(self.group()), Box::new(bar())),
            "vec" => Expr::Over(Box::new(self.group()), Box::new(Expr::Operator("→".into()))),
            "hat" => Expr::Over(Box::new(self.group()), Box::new(Expr::Operator("^".into()))),
            "dot" => Expr::Over(Box::new(self.group()), Box::new(Expr::Operator(".".into()))),
            "ddot" => Expr::Over(Box::new(self.group()), Box::new(Expr::Operator("..".into()))),
            "tilde" => Expr::Over(Box::new(self.group()), Box::new(Expr::Operator("~".into()))),
            "overbrace" => Expr::Over(Box::new(self.group()), Box::new(Expr::Operator("⏞".into()))),
            "underbrace" => Expr::Under(Box::new(self.group()), Box::new(Expr::Operator("⏟".into()))),
            "overset" => {
                let over = self.group();
                let base = self.group();
                Expr::Over(Box::new(base), Box::new(over))
            }
            "underset" => {
                let under = self.group();
                let base = self.group();
                Expr::Under(Box::new(base), Box::new(under))
            }
            "boxed" => wrap_outer("box", self.group()),
            "circle" => wrap_outer("circle", self.group()),
            "overwrite" => {
                let body = self.group();
                let sym = self.brace_command();
                wrap_inner(body, &overwrite_notation(&sym))
            }
            "begin" => self.parse_matrix(),
            "left" => self.parse_left(),
            "text" => Expr::Text(self.group_text()),
            _ => Expr::Identifier(name.to_string()),
        }
    }

    fn parse_matrix(&mut self) -> Expr {
        let env = self.brace_word();
        // `\begin{array}{cc}` — the group after the name is a column spec.
        if env == "array" || env == "array*" {
            self.skip_ws();
            if self.peek() == Some('{') {
                let _ = self.group();
            }
        }
        let (open, close) = matrix_fences(&env);
        let mut rows = Vec::new();
        loop {
            self.skip_ws();
            if self.eof() || self.at_cmd("end") {
                break;
            }
            let mut cells = Vec::new();
            let mut broke = false;
            loop {
                cells.push(list(self.parse_list(Stop::Cell)));
                self.skip_ws();
                if self.eat("&") {
                    continue;
                }
                if self.at_row_break() {
                    self.advance(2);
                    broke = true;
                }
                break;
            }
            rows.push(cells);
            // `\\` at the end of what was typed starts the next row, still empty.
            if broke {
                self.skip_ws();
                if self.eof() || self.at_cmd("end") {
                    rows.push(vec![Expr::Row(vec![])]);
                }
            }
        }
        if self.at_cmd("end") {
            let _ = self.read_cmd_name();
            let _ = self.brace_word();
        }
        if rows.is_empty() {
            rows.push(vec![Expr::Row(vec![])]);
        }
        Expr::Table { open, close, rows }
    }

    fn parse_left(&mut self) -> Expr {
        self.skip_ws();
        let open = self.read_fence();
        let items = self.parse_list(Stop::Right);
        let close = if self.at_cmd("right") {
            let _ = self.read_cmd_name();
            Some(self.read_fence())
        } else {
            None
        };
        Expr::Fenced {
            open,
            close,
            body: Box::new(list(items)),
        }
    }

    fn read_fence(&mut self) -> String {
        self.skip_ws();
        if self.rest().starts_with('\\') {
            if let Some((out, n)) = latex_symbols::match_prefix(self.rest()) {
                self.advance(n);
                return out.text;
            }
            return self.read_cmd_name();
        }
        match self.peek() {
            Some(c) => {
                self.advance(c.len_utf8());
                c.to_string()
            }
            None => String::new(),
        }
    }

    fn brace_word(&mut self) -> String {
        self.skip_ws();
        if !self.eat("{") {
            return String::new();
        }
        let mut s = String::new();
        while let Some(c) = self.peek() && c != '}' && !c.is_whitespace() {
            s.push(c);
            self.advance(c.len_utf8());
        }
        self.skip_ws();
        self.eat("}");
        s
    }

    fn brace_command(&mut self) -> String {
        self.skip_ws();
        if !self.eat("{") {
            return String::new();
        }
        self.skip_ws();
        let name = if self.rest().starts_with('\\') {
            self.read_cmd_name()
        } else {
            String::new()
        };
        while !self.eof() && self.peek() != Some('}') {
            let ch = self.peek().unwrap();
            self.advance(ch.len_utf8());
        }
        self.eat("}");
        name
    }

    fn group_text(&mut self) -> String {
        self.skip_ws();
        if !self.eat("{") {
            return String::new();
        }
        let mut s = String::new();
        let mut depth = 1i32;
        while let Some(c) = self.peek() && depth > 0 {
            self.advance(c.len_utf8());
            match c {
                '{' => {
                    depth += 1;
                    if depth > 1 {
                        s.push(c);
                    }
                }
                '}' => {
                    depth -= 1;
                    if depth > 0 {
                        s.push(c);
                    }
                }
                _ => s.push(c),
            }
        }
        s
    }
}

fn list(items: Vec<Expr>) -> Expr {
    match items.len() {
        0 => Expr::Row(vec![]),
        1 => items.into_iter().next().unwrap(),
        _ => Expr::Row(items),
    }
}

fn emit(out: &Out) -> Expr {
    if out.ident {
        Expr::Identifier(out.text.clone())
    } else {
        Expr::Operator(out.text.clone())
    }
}

fn func_expr(name: &str) -> Expr {
    if latex_symbols::is_limit_name(name) {
        Expr::Operator(name.to_string())
    } else {
        Expr::Identifier(name.to_string())
    }
}

fn scripted(base: Expr, sub: Option<Expr>, sup: Option<Expr>, large: bool) -> Expr {
    match (sub, sup, large) {
        (Some(sub), Some(sup), true) => Expr::UnderOver(Box::new(base), Box::new(sub), Box::new(sup)),
        (Some(sub), None, true) => Expr::Under(Box::new(base), Box::new(sub)),
        (None, Some(sup), true) => Expr::Over(Box::new(base), Box::new(sup)),
        (Some(sub), Some(sup), false) => Expr::SubSup(Box::new(base), Box::new(sub), Box::new(sup)),
        (Some(sub), None, false) => Expr::Sub(Box::new(base), Box::new(sub)),
        (None, Some(sup), false) => Expr::Sup(Box::new(base), Box::new(sup)),
        (None, None, _) => base,
    }
}

fn bar() -> Expr {
    Expr::Operator("\u{2015}".into())
}

fn is_large(expr: &Expr) -> bool {
    match expr {
        Expr::Operator(s) if latex_symbols::is_limit_name(s) => true,
        Expr::Operator(s) => {
            s.chars().count() == 1 &&
                matches!(
                    s.chars().next().unwrap(),
                    '∑' | '∏' | '∐' | '∫' | '∬' | '∭' | '∮' | '∯' | '∰' | '⋀' | '⋁' | '⋂' | '⋃'
                        | '⨁' | '⨂' | '⨀'
                )
        }
        _ => false,
    }
}

fn is_structural(name: &str) -> bool {
    matches!(
        name,
        "frac" | "f"
            | "sqrt" | "s"
            | "binom"
            | "overline" | "ol"
            | "underline"
            | "vec"
            | "hat"
            | "dot"
            | "ddot"
            | "tilde"
            | "overbrace"
            | "underbrace"
            | "overset"
            | "underset"
            | "boxed"
            | "circle"
            | "overwrite"
            | "begin"
            | "left"
            | "text"
    )
}

fn cmd_name_of(token: &str) -> String {
    let rest = token.strip_prefix('\\').unwrap_or(token);
    rest.chars().take_while(|c| c.is_ascii_alphabetic()).collect()
}

fn wrap_outer(outer: &str, body: Expr) -> Expr {
    match body {
        Expr::Enclose { notation, body } => Expr::Enclose {
            notation: format!("{outer} {notation}"),
            body,
        },
        other => Expr::Enclose {
            notation: outer.to_string(),
            body: Box::new(other),
        },
    }
}

fn wrap_inner(body: Expr, piece: &str) -> Expr {
    match body {
        Expr::Enclose { notation, body } => Expr::Enclose {
            notation: format!("{notation} {piece}"),
            body,
        },
        other => Expr::Enclose {
            notation: piece.to_string(),
            body: Box::new(other),
        },
    }
}

fn overwrite_notation(cmd: &str) -> String {
    match cmd {
        "senwline" => "updiagonalstrike",
        "neswline" => "downdiagonalstrike",
        "vline" => "verticalstrike",
        "hline" => "horizontalstrike",
        "nearrow" => "northeastarrow",
        "searrow" => "southeastarrow",
        "swarrow" => "southwestarrow",
        "nwarrow" => "northwestarrow",
        "uparrow" => "uparrow",
        "downarrow" => "downarrow",
        "longleftarrow" => "leftarrow",
        "longrightarrow" => "rightarrow",
        "updownarrow" => "updownarrow",
        "longleftrightarrow" => "leftrightarrow",
        "neswarrow" => "northeastsouthwestarrow",
        "nwsearrow" => "northwestsoutheastarrow",
        other => other,
    }
    .to_string()
}

fn matrix_fences(env: &str) -> (Option<String>, Option<String>) {
    match env {
        "pmatrix" | "bpm" => (Some("(".into()), Some(")".into())),
        "bmatrix" | "bbm" => (Some("[".into()), Some("]".into())),
        "vmatrix" | "bvm" => (Some("|".into()), Some("|".into())),
        "Vmatrix" | "bVm" => (Some("‖".into()), Some("‖".into())),
        "Bmatrix" | "bBm" => (Some("{".into()), Some("}".into())),
        // `matrix` and `array` are bare tables. An unknown or unfinished name is too.
        _ => (None, None),
    }
}

const FUNC_NAMES: &[&str] = &[
    "arccsch", "arcsech", "arccoth", "arcsinh", "arccosh", "arctanh", "arcsin", "arccos",
    "arctan", "arccsc", "arcsec", "arccot", "liminf", "limsup", "sinh", "cosh", "tanh",
    "csch", "sech", "coth", "sin", "cos", "tan", "csc", "sec", "cot", "deg", "det", "dim",
    "exp", "gcd", "hom", "inf", "ker", "lg", "lim", "ln", "log", "max", "min", "Pr", "sup",
];

pub(super) fn braille_to_expr(input: &str) -> Result<Expr> {
    latex_symbols::ensure_latex_symbols_loaded()?;
    let mut parser = Parser::new(input);
    Ok(list(parser.parse_list(Stop::Top)))
}

#[cfg(test)]
mod tests {
    use super::super::Braille_to_MathML;

    fn mml(src: &str) -> String {
        Braille_to_MathML(src, "LaTeX").unwrap_or_else(|e| panic!("parse failed for {src:?}: {e}"))
    }

    #[test]
    fn longest_command_not_prefix() {
        // `\infty` is one symbol, not `\in` followed by `fty`.
        let out = mml(r"\infty");
        assert!(out.contains("&#x221E;") || out.contains("∞"), "{out}");
        assert!(!out.contains(">fty<"), "{out}");
    }

    #[test]
    fn not_equals_is_one_symbol() {
        let out = mml(r"\not=");
        assert!(out.contains("&#x2260;") || out.contains("≠"), "{out}");
    }

    #[test]
    fn unclosed_frac_does_not_fail() {
        // `\frac` is an empty fraction template, not two "missing char" slots.
        assert_eq!(
            mml(r"\frac"),
            "<math><mfrac><mrow></mrow><mrow></mrow></mfrac></math>"
        );
        // Numerator was typed; the open denominator is empty.
        assert_eq!(
            mml(r"\frac{x}{"),
            "<math><mfrac><mi>x</mi><mrow></mrow></mfrac></math>"
        );
        assert_eq!(
            mml(r"\frac{1"),
            "<math><mfrac><mn>1</mn><mrow></mrow></mfrac></math>"
        );
        // `2^` and `2^{` are a superscript with an empty exponent.
        assert_eq!(
            mml("2^"),
            "<math><msup><mn>2</mn><mrow></mrow></msup></math>"
        );
        assert_eq!(
            mml("2^{"),
            "<math><msup><mn>2</mn><mrow></mrow></msup></math>"
        );
        assert_eq!(
            mml("2_{"),
            "<math><msub><mn>2</mn><mrow></mrow></msub></math>"
        );
        assert_eq!(
            mml("2^{x"),
            "<math><msup><mn>2</mn><mi>x</mi></msup></math>"
        );
        crate::set_rules_dir(crate::abs_rules_dir_path()).unwrap();
        for src in [r"\frac", r"\frac{x}{"] {
            crate::set_mathml(&mml(src)).unwrap_or_else(|e| panic!("{src}: {e}"));
            let speech = crate::get_spoken_text().unwrap();
            assert!(
                !speech.to_lowercase().contains("missing"),
                "{src} speech={speech}"
            );
        }
    }

    #[test]
    fn incomplete_scripts_roots_and_arrays_stay_open() {
        // An unstarted script, root, binomial, accent, fence, or array is an
        // empty slot. Nothing here is a "missing char" placeholder.
        let cases = [
            ("2_", "<math><msub><mn>2</mn><mrow></mrow></msub></math>"),
            (r"\sqrt", "<math><msqrt><mrow></mrow></msqrt></math>"),
            (r"\sqrt{", "<math><msqrt><mrow></mrow></msqrt></math>"),
            (r"\sqrt{x", "<math><msqrt><mi>x</mi></msqrt></math>"),
            (
                r"\sqrt[",
                "<math><mroot><mrow></mrow><mrow></mrow></mroot></math>",
            ),
            (
                r"\sqrt[3",
                "<math><mroot><mrow></mrow><mn>3</mn></mroot></math>",
            ),
            (
                r"\sqrt[3]{",
                "<math><mroot><mrow></mrow><mn>3</mn></mroot></math>",
            ),
            (
                r"\sqrt[3]{x",
                "<math><mroot><mi>x</mi><mn>3</mn></mroot></math>",
            ),
            (
                r"\binom",
                "<math><mrow><mo>(</mo><mfrac linethickness=\"0\"><mrow></mrow><mrow></mrow></mfrac><mo>)</mo></mrow></math>",
            ),
            (
                r"\binom{n}{",
                "<math><mrow><mo>(</mo><mfrac linethickness=\"0\"><mi>n</mi><mrow></mrow></mfrac><mo>)</mo></mrow></math>",
            ),
            (
                r"\hat",
                "<math><mover><mrow></mrow><mo>^</mo></mover></math>",
            ),
            (r"\left(", "<math><mo>(</mo></math>"),
            (r"\left(x", "<math><mrow><mo>(</mo><mi>x</mi></mrow></math>"),
            (
                r"\begin{pmatrix}",
                "<math><mrow><mo>(</mo><mtable><mtr><mtd><mrow></mrow></mtd></mtr></mtable><mo>)</mo></mrow></math>",
            ),
            (
                r"\begin{pmatrix} a &",
                "<math><mrow><mo>(</mo><mtable><mtr><mtd><mi>a</mi></mtd><mtd><mrow></mrow></mtd></mtr></mtable><mo>)</mo></mrow></math>",
            ),
            (
                r"\begin{pmatrix} a \\",
                "<math><mrow><mo>(</mo><mtable><mtr><mtd><mi>a</mi></mtd></mtr><mtr><mtd><mrow></mrow></mtd></mtr></mtable><mo>)</mo></mrow></math>",
            ),
            (
                r"\begin{array}{cc} a &",
                "<math><mtable><mtr><mtd><mi>a</mi></mtd><mtd><mrow></mrow></mtd></mtr></mtable></math>",
            ),
        ];
        for (src, expect) in cases {
            let out = mml(src);
            assert_eq!(out, expect, "{src}");
            assert!(!out.contains("FFFD"), "{src} -> {out}");
        }
    }
}
