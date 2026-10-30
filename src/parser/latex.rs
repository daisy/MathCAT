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
    /// `\mathop` makes the following scripts under/over. `\limits` / `\nolimits` override it.
    force_limits: Option<bool>,
    /// Declaration fonts (`\bf`, `\rm`, …). The last face applies to new letters.
    fonts: Vec<Face>,
    /// `fonts.len()` at each `{`, so `}` restores the outer face.
    font_marks: Vec<usize>,
    /// `@>>>` and the other amscd arrows are arrows only inside `\begin{CD}`.
    in_cd: bool,
}

#[derive(Clone, Copy)]
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

#[derive(Clone, Copy)]
enum Infix {
    Over,
    Atop,
    Choose,
    Brace,
    Brack,
}

impl Parser {
    fn new(input: &str) -> Self {
        Self {
            input: input.to_string(),
            pos: 0,
            force_limits: None,
            fonts: Vec::new(),
            font_marks: Vec::new(),
            in_cd: false,
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
        (rest.starts_with("\\\\") &&
            rest[2..].chars().next().is_none_or(|c| !c.is_ascii_alphabetic())) ||
            self.at_cmd("cr")
    }

    fn eat_row_break(&mut self) -> bool {
        if self.rest().starts_with("\\\\") &&
            self.rest()[2..].chars().next().is_none_or(|c| !c.is_ascii_alphabetic())
        {
            self.advance(2);
            true
        } else if self.at_cmd("cr") {
            let _ = self.read_cmd_name();
            true
        } else {
            false
        }
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
        self.parse_scoped(stop, false).0
    }

    /// `until_over` is the top of `\buildrel`. The `\over` there ends the top
    /// instead of starting another fraction, and that signal propagates out.
    fn parse_scoped(&mut self, stop: Stop, until_over: bool) -> (Vec<Expr>, bool) {
        let mut items = Vec::new();
        loop {
            self.skip_ws();
            if self.eof() || self.at_stop(&stop) {
                return (items, false);
            }
            if until_over && self.at_cmd("over") {
                let _ = self.read_cmd_name();
                return (items, true);
            }
            if self.at_cmd("buildrel") {
                let _ = self.read_cmd_name();
                let (top, saw) = self.parse_scoped(stop, true);
                if !saw {
                    items.push(Expr::Over(Box::new(Expr::Row(vec![])), Box::new(list(top))));
                    return (items, false);
                }
                let (base, base_saw) = self.parse_scoped(stop, until_over);
                items.push(Expr::Over(Box::new(list(base)), Box::new(list(top))));
                return (items, base_saw);
            }
            if let Some(kind) = self.infix_kind() {
                let _ = self.read_cmd_name();
                let (den, saw) = self.parse_scoped(stop, until_over);
                let frac = infix_frac(kind, list(std::mem::take(&mut items)), list(den));
                return (vec![frac], saw);
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
    }

    fn infix_kind(&self) -> Option<Infix> {
        if self.at_cmd("over") {
            Some(Infix::Over)
        } else if self.at_cmd("atop") {
            Some(Infix::Atop)
        } else if self.at_cmd("choose") {
            Some(Infix::Choose)
        } else if self.at_cmd("brace") {
            Some(Infix::Brace)
        } else if self.at_cmd("brack") {
            Some(Infix::Brack)
        } else {
            None
        }
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
        let mut forced = self.force_limits.take();
        loop {
            self.skip_ws();
            if self.at_cmd("limits") {
                let _ = self.read_cmd_name();
                forced = Some(true);
                continue;
            }
            if self.at_cmd("nolimits") {
                let _ = self.read_cmd_name();
                forced = Some(false);
                continue;
            }
            let large = forced.unwrap_or_else(|| is_large(&base));
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
        self.braced()
    }

    /// The body after `{` has been consumed. `}` restores declaration fonts.
    fn braced(&mut self) -> Expr {
        self.font_marks.push(self.fonts.len());
        let items = self.parse_list(Stop::Group);
        self.eat("}");
        if let Some(mark) = self.font_marks.pop() {
            self.fonts.truncate(mark);
        }
        list(items)
    }

    /// An explicit font command (`\mathbf`, `\mathrm`) ignores declaration faces
    /// such as `\it`, then applies its own face to the argument.
    fn plain_group(&mut self) -> Expr {
        let saved = std::mem::take(&mut self.fonts);
        let body = self.group();
        self.fonts = saved;
        body
    }

    /// `[l]` / `[r]` on `\cfrac`. The alignment is not stored.
    fn skip_optional_bracket(&mut self) {
        self.skip_ws();
        if !self.eat("[") {
            return;
        }
        while !self.eof() && self.peek() != Some(']') {
            let ch = self.peek().unwrap();
            self.advance(ch.len_utf8());
        }
        self.eat("]");
    }

    fn parse_atom(&mut self) -> Expr {
        self.skip_ws();
        if self.eof() {
            return Expr::Row(vec![]);
        }
        // A line break here is outside a matrix cell. Drop it instead of stalling.
        if self.at_row_break() {
            self.eat_row_break();
            return Expr::Row(vec![]);
        }
        // `[\![` / `]\!]` before a stray `]` is taken on its own.
        if let Some(ch) = self.take_double_bracket() {
            return self.oper(ch.to_string());
        }
        if let Some(ch) = self.take_cd_arrow() {
            return Expr::Operator(ch.to_string());
        }
        // A closer the current list did not claim. Keep it and move on.
        if matches!(self.peek(), Some('}' | '&' | ']')) {
            let ch = self.peek().unwrap();
            self.advance(ch.len_utf8());
            return self.oper(ch.to_string());
        }
        if self.eat("{") {
            return self.braced();
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
            return self.emit_out(&out);
        }
        if let Some(n) = self.number_len() {
            let text = self.rest()[..n].to_string();
            self.advance(n);
            return self.number(text);
        }
        if let Some(name) = self.match_func() {
            self.advance(name.len());
            return self.func_leaf(name);
        }
        // LaTeX `~` is a non-breaking space. A short name such as `~p` is a longer token
        // and was already taken by the symbol table.
        if self.eat("~") {
            return Expr::Operator("\u{00A0}".into());
        }
        let ch = self.peek().unwrap();
        self.advance(ch.len_utf8());
        if ch.is_alphabetic() {
            self.ident(ch.to_string())
        } else {
            self.oper(ch.to_string())
        }
    }

    fn ident(&self, text: String) -> Expr {
        match self.fonts.last().copied() {
            Some(face) => apply_face(Expr::Identifier(text), face),
            None => Expr::Identifier(text),
        }
    }

    fn number(&self, text: String) -> Expr {
        match self.fonts.last().copied() {
            Some(face) => apply_face(Expr::Number(text), face),
            None => Expr::Number(text),
        }
    }

    fn oper(&self, text: String) -> Expr {
        match self.fonts.last().copied() {
            Some(face) => apply_face(Expr::Operator(text), face),
            None => Expr::Operator(text),
        }
    }

    fn emit_out(&self, out: &Out) -> Expr {
        if out.ident {
            self.ident(out.text.clone())
        } else {
            self.oper(out.text.clone())
        }
    }

    fn func_leaf(&self, name: &str) -> Expr {
        // Limit names stay plain operators so `_` / `^` become under/over.
        if latex_symbols::is_limit_name(name) {
            Expr::Operator(name.to_string())
        } else {
            self.ident(name.to_string())
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
            self.eat_row_break();
            return Expr::Row(vec![]);
        }
        // `\lbrack\!\lbrack` before `\!` becomes a zero-width space.
        if let Some(ch) = self.take_double_bracket() {
            return self.oper(ch.to_string());
        }
        if let Some(space) = self.explicit_space() {
            return space;
        }
        if let Some((out, n)) = latex_symbols::match_prefix(self.rest()) {
            let name = cmd_name_of(&self.rest()[..n]);
            let after = self.rest()[n..].chars().next();
            if !(is_structural(&name) && matches!(after, Some('{' | '['))) {
                self.advance(n);
                return self.emit_out(&out);
            }
        }
        let name = self.read_cmd_name();
        if is_ignored_command(&name) {
            return Expr::Row(vec![]);
        }
        if is_ignored_with_arg(&name) {
            self.skip_ws();
            let _ = self.eat("*");
            if self.peek() == Some('{') {
                let _ = self.group();
            }
            return Expr::Row(vec![]);
        }
        // `\bigl(` is the fence character. The size grade is not representable.
        if is_big_delim(&name) || name == "limits" || name == "nolimits" {
            return Expr::Row(vec![]);
        }
        if let Some(face) = decl_face(&name) {
            self.fonts.push(face);
            return Expr::Row(vec![]);
        }
        if matches!(name.as_str(), "mkern" | "mskip") {
            return mu_to_space(self.read_mu());
        }
        if is_structural(&name) {
            self.parse_structural(&name)
        } else if name.is_empty() {
            Expr::Operator("\\".into())
        } else {
            self.ident(name)
        }
    }

    /// `\!` `\,` `\:` `\;` and `\ ` (backslash space). A longer symbol that starts
    /// with the same characters keeps the symbol (`\,,` is still the low double quote).
    fn explicit_space(&mut self) -> Option<Expr> {
        let rest = self.rest();
        let mut chars = rest.chars();
        if chars.next() != Some('\\') {
            return None;
        }
        let (ch, n) = match chars.next() {
            Some('!') => ('\u{200B}', 2usize),
            Some(',') => ('\u{2009}', 2),
            Some(':') => ('\u{205F}', 2),
            Some(';') => ('\u{2004}', 2),
            Some(' ') => (' ', 2),
            _ => return None,
        };
        if let Some((_, len)) = latex_symbols::match_prefix(rest) &&
            len > n
        {
            return None;
        }
        self.advance(n);
        Some(Expr::Operator(ch.to_string()))
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
            "frac" | "f" | "dfrac" | "tfrac" | "cfrac" => {
                if name == "cfrac" {
                    self.skip_optional_bracket();
                }
                let num = self.group();
                let den = self.group();
                Expr::Frac(Box::new(num), Box::new(den))
            }
            "ifrac" => {
                let num = self.group();
                let den = self.group();
                Expr::BevelledFrac(Box::new(num), Box::new(den))
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
            "binom" | "dbinom" | "tbinom" => {
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
            "widehat" => accent_over("^", self.group()),
            "widetilde" => accent_over("~", self.group()),
            "check" => accent_over("\u{02C7}", self.group()),
            "acute" => accent_over("\u{00B4}", self.group()),
            "grave" => accent_over("`", self.group()),
            "breve" => accent_over("\u{02D8}", self.group()),
            "bar" => accent_over("\u{00AF}", self.group()),
            "mathring" => accent_over("\u{02DA}", self.group()),
            "dddot" => accent_over("...", self.group()),
            "ddddot" => accent_over("....", self.group()),
            "overrightarrow" => accent_over("\u{2192}", self.group()),
            "overleftarrow" => accent_over("\u{2190}", self.group()),
            "overleftrightarrow" => accent_over("\u{2194}", self.group()),
            "underrightarrow" => accent_under("\u{2192}", self.group()),
            "underleftarrow" => accent_under("\u{2190}", self.group()),
            "underleftrightarrow" => accent_under("\u{2194}", self.group()),
            "stackrel" => {
                let over = self.group();
                let base = self.group();
                Expr::Over(Box::new(base), Box::new(over))
            }
            "substack" => self.parse_substack(),
            "operatorname" => {
                let name = self.group_text();
                self.func_leaf(name.trim())
            }
            "bmod" => self.ident("mod".into()),
            "pmod" => Expr::Row(vec![
                Expr::Operator("(".into()),
                self.ident("mod".into()),
                Expr::Operator("\u{00A0}".into()),
                self.group(),
                Expr::Operator(")".into()),
            ]),
            "mod" => Expr::Row(vec![
                self.ident("mod".into()),
                Expr::Operator("\u{00A0}".into()),
                self.group(),
            ]),
            "phantom" => enclose("phantom", self.group()),
            "hphantom" => enclose("hphantom", self.group()),
            "vphantom" => enclose("vphantom", self.group()),
            "cancel" => enclose("updiagonalstrike", self.group()),
            "bcancel" => enclose("downdiagonalstrike", self.group()),
            "xcancel" => enclose("updiagonalstrike downdiagonalstrike", self.group()),
            "fbox" | "framebox" => wrap_outer("box", self.group()),
            "middle" => Expr::Operator(self.read_fence()),
            "mathrel" => self.group(),
            "mathop" => {
                let body = self.group();
                self.force_limits = Some(true);
                self.parse_postscripts(body)
            }
            "mathrm" | "textrm" | "textnormal" => apply_upright(self.plain_group()),
            "mathbf" | "textbf" => apply_alpha(self.plain_group(), 0x1D400, 0x1D7CE, 0x1D6A8),
            "mathit" | "textit" | "emph" => apply_alpha(self.plain_group(), 0x1D434, 0, 0x1D6E2),
            "mathsf" | "textsf" => apply_alpha(self.plain_group(), 0x1D5A0, 0x1D7E2, 0),
            "mathtt" | "texttt" => apply_alpha(self.plain_group(), 0x1D670, 0x1D7F6, 0),
            "mathbb" | "Bbb" => apply_alpha(self.plain_group(), 0x1D538, 0x1D7D8, 0),
            "mathcal" | "mathscr" => apply_alpha(self.plain_group(), 0x1D49C, 0, 0),
            "mathfrak" => apply_alpha(self.plain_group(), 0x1D504, 0, 0),
            "boldsymbol" | "bm" => apply_alpha(self.plain_group(), 0x1D468, 0x1D7CE, 0x1D71C),
            "mbox" | "hbox" => Expr::Text(self.group_text()),
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
            "hspace" => self.parse_hspace(),
            "bra" => Expr::Fenced {
                open: "⟨".into(),
                close: Some("∣".into()),
                body: Box::new(self.group()),
            },
            "ket" => Expr::Fenced {
                open: "∣".into(),
                close: Some("⟩".into()),
                body: Box::new(self.group()),
            },
            "xrightarrow" | "xleftarrow" => self.parse_extensible_arrow(name),
            "genfrac" => self.parse_genfrac(),
            "root" => self.parse_root_of(),
            "eqalign" => self.parse_substack(),
            "color" | "textcolor" => self.parse_color(),
            _ => self.ident(name.to_string()),
        }
    }

    fn parse_matrix(&mut self) -> Expr {
        let env = self.brace_word();
        // `\begin{array}{cc}` and `\begin{alignat}{2}` — the group is a column spec.
        if takes_column_spec(&env) {
            self.skip_ws();
            if self.peek() == Some('{') {
                let _ = self.group();
            }
        }
        let (open, close) = matrix_fences(&env);
        let saved_cd = self.in_cd;
        if env == "CD" {
            self.in_cd = true;
        }
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
                if self.eat_row_break() {
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
        self.in_cd = saved_cd;
        let table = Expr::Table { open, close, rows };
        if env == "equation" || env == "equation*" {
            unwrap_equation(table)
        } else {
            table
        }
    }

    /// `\substack{a \\ b}` — one column, rows split on `\\`.
    fn parse_substack(&mut self) -> Expr {
        self.skip_ws();
        let opened = self.eat("{");
        let mut rows = Vec::new();
        if opened {
            loop {
                self.skip_ws();
                if self.eof() || self.peek() == Some('}') {
                    break;
                }
                let mut cells = Vec::new();
                let mut broke = false;
                loop {
                    cells.push(list(self.parse_list(Stop::Cell)));
                    self.skip_ws();
                    if self.eof() || self.peek() == Some('}') {
                        break;
                    }
                    if self.eat("&") {
                        continue;
                    }
                    if self.eat_row_break() {
                        broke = true;
                    }
                    break;
                }
                rows.push(cells);
                if broke {
                    self.skip_ws();
                    if self.eof() || self.peek() == Some('}') {
                        rows.push(vec![Expr::Row(vec![])]);
                    }
                }
            }
            self.eat("}");
        }
        if rows.is_empty() {
            rows.push(vec![Expr::Row(vec![])]);
        }
        Expr::Table {
            open: None,
            close: None,
            rows,
        }
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
        if let Some(ch) = self.take_double_bracket() {
            return ch.to_string();
        }
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

    /// `[\![`, `]\!]`, `\lbrack\!\lbrack`, `\rbrack\!\rbrack` → U+27E6 / U+27E7.
    fn take_double_bracket(&mut self) -> Option<char> {
        let rest = self.rest();
        let (ch, n) = if rest.starts_with("\\lbrack\\!\\lbrack") {
            ('\u{27E6}', 16)
        } else if rest.starts_with("\\rbrack\\!\\rbrack") {
            ('\u{27E7}', 16)
        } else if rest.starts_with("[\\![") {
            ('\u{27E6}', 4)
        } else if rest.starts_with("]\\!]") {
            ('\u{27E7}', 4)
        } else {
            return None;
        };
        self.advance(n);
        Some(ch)
    }

    /// `@>>>` `@<<<` `@VVV` `@AAA`, three or more marks. Only while `in_cd` is set.
    fn take_cd_arrow(&mut self) -> Option<char> {
        if !self.in_cd {
            return None;
        }
        let rest = self.rest();
        let mut chars = rest.chars();
        if chars.next() != Some('@') {
            return None;
        }
        let kind = chars.next()?;
        if !matches!(kind, '>' | '<' | 'V' | 'A') {
            return None;
        }
        let mut count = 1usize;
        let mut n = 2usize;
        for c in chars {
            if c != kind {
                break;
            }
            count += 1;
            n += c.len_utf8();
        }
        if count < 3 {
            return None;
        }
        self.advance(n);
        Some(match kind {
            '>' => '\u{2192}',
            '<' => '\u{2190}',
            'V' => '\u{2193}',
            _ => '\u{2191}',
        })
    }

    /// Sign, decimal, and `mu` / `em` / `ex` / `pt`. Missing pieces are zero mu.
    fn read_mu(&mut self) -> f64 {
        self.skip_ws();
        let neg = self.eat("-");
        if !neg {
            let _ = self.eat("+");
        }
        let mut num = String::new();
        while let Some(c) = self.peek() &&
            (c.is_ascii_digit() || (c == '.' && !num.contains('.')))
        {
            num.push(c);
            self.advance(c.len_utf8());
        }
        let mag: f64 = num.parse().unwrap_or(0.0);
        self.skip_ws();
        let scale = if self.eat("mu") {
            1.0
        } else if self.eat("em") {
            18.0
        } else if self.eat("ex") {
            9.0
        } else if self.eat("pt") {
            1.8
        } else {
            1.0
        };
        if neg { -mag * scale } else { mag * scale }
    }

    fn parse_hspace(&mut self) -> Expr {
        self.skip_ws();
        let braced = self.eat("{");
        let mu = self.read_mu();
        if braced {
            self.skip_ws();
            while !self.eof() && self.peek() != Some('}') {
                let c = self.peek().unwrap();
                self.advance(c.len_utf8());
            }
            self.eat("}");
        }
        mu_to_space(mu)
    }

    fn parse_extensible_arrow(&mut self, name: &str) -> Expr {
        let arrow = if name == "xrightarrow" { "→" } else { "←" };
        self.skip_ws();
        let below = if self.eat("[") {
            let under = list(self.parse_list(Stop::Bracket));
            self.eat("]");
            Some(under)
        } else {
            None
        };
        let above = self.group();
        let base = Expr::Operator(arrow.into());
        match below {
            Some(under) if !under.is_empty_row() => {
                if above.is_empty_row() {
                    Expr::Under(Box::new(base), Box::new(under))
                } else {
                    Expr::UnderOver(Box::new(base), Box::new(under), Box::new(above))
                }
            }
            _ => {
                if above.is_empty_row() {
                    base
                } else {
                    Expr::Over(Box::new(base), Box::new(above))
                }
            }
        }
    }

    fn parse_genfrac(&mut self) -> Expr {
        let left = fence_spec(&self.group_text());
        let right = fence_spec(&self.group_text());
        let thick = self.group_text();
        let _style = self.group_text();
        let num = self.group();
        let den = self.group();
        let frac = if thickness_is_zero(&thick) {
            Expr::BinomFrac(Box::new(num), Box::new(den))
        } else {
            Expr::Frac(Box::new(num), Box::new(den))
        };
        fence_frac(frac, left, right)
    }

    /// `\root n \of expr` — the index runs up to `\of`, then one group or atom.
    fn parse_root_of(&mut self) -> Expr {
        let mut items = Vec::new();
        loop {
            self.skip_ws();
            if self.eof() ||
                self.at_cmd("of") ||
                self.peek() == Some('}') ||
                self.at_row_break() ||
                self.peek() == Some('&')
            {
                break;
            }
            let atom = self.parse_atom();
            if atom.is_empty_row() {
                continue;
            }
            items.push(self.parse_postscripts(atom));
        }
        if self.at_cmd("of") {
            let _ = self.read_cmd_name();
        }
        Expr::Root(Box::new(list(items)), Box::new(self.script_body()))
    }

    /// Color is dropped. A following group, when present, is the math.
    fn parse_color(&mut self) -> Expr {
        self.skip_ws();
        if self.peek() == Some('[') {
            self.skip_optional_bracket();
        }
        self.skip_ws();
        if self.peek() == Some('{') {
            let _ = self.group_text();
        }
        self.skip_ws();
        if self.peek() == Some('{') {
            self.group()
        } else {
            Expr::Row(vec![])
        }
    }
}

fn list(items: Vec<Expr>) -> Expr {
    match items.len() {
        0 => Expr::Row(vec![]),
        1 => items.into_iter().next().unwrap(),
        _ => Expr::Row(items),
    }
}

fn infix_frac(kind: Infix, num: Expr, den: Expr) -> Expr {
    let barless = matches!(kind, Infix::Atop | Infix::Choose | Infix::Brace | Infix::Brack);
    let frac = if barless {
        Expr::BinomFrac(Box::new(num), Box::new(den))
    } else {
        Expr::Frac(Box::new(num), Box::new(den))
    };
    let (open, close) = match kind {
        Infix::Choose => ("(", ")"),
        Infix::Brace => ("{", "}"),
        Infix::Brack => ("[", "]"),
        Infix::Over | Infix::Atop => return frac,
    };
    Expr::Row(vec![
        Expr::Operator(open.into()),
        frac,
        Expr::Operator(close.into()),
    ])
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

fn accent_over(op: &str, body: Expr) -> Expr {
    Expr::Over(Box::new(body), Box::new(Expr::Operator(op.into())))
}

fn accent_under(op: &str, body: Expr) -> Expr {
    Expr::Under(Box::new(body), Box::new(Expr::Operator(op.into())))
}

fn enclose(notation: &str, body: Expr) -> Expr {
    Expr::Enclose {
        notation: notation.to_string(),
        body: Box::new(body),
    }
}

#[derive(Clone, Copy)]
enum Face {
    Upright,
    Alpha { letters: u32, digits: u32, greek: u32 },
}

fn apply_upright(expr: Expr) -> Expr {
    apply_face(expr, Face::Upright)
}

fn apply_alpha(expr: Expr, letters: u32, digits: u32, greek: u32) -> Expr {
    apply_face(
        expr,
        Face::Alpha {
            letters,
            digits,
            greek,
        },
    )
}

fn apply_face(expr: Expr, face: Face) -> Expr {
    match expr {
        Expr::Identifier(text) => match face {
            Face::Upright => Expr::StyledIdent {
                text,
                mathvariant: "normal".into(),
            },
            Face::Alpha {
                letters,
                digits,
                greek,
            } => Expr::Identifier(map_text(&text, letters, digits, greek)),
        },
        Expr::StyledIdent { text, .. } => match face {
            Face::Upright => Expr::StyledIdent {
                text,
                mathvariant: "normal".into(),
            },
            Face::Alpha {
                letters,
                digits,
                greek,
            } => Expr::Identifier(map_text(&text, letters, digits, greek)),
        },
        Expr::Number(text) => Expr::Number(map_text_face(&text, face)),
        Expr::StyledNumber { text, mathvariant } => Expr::StyledNumber {
            text: map_text_face(&text, face),
            mathvariant,
        },
        Expr::Operator(text) => Expr::Operator(map_text_face(&text, face)),
        Expr::Text(text) => Expr::Text(map_text_face(&text, face)),
        Expr::Row(v) => Expr::Row(v.into_iter().map(|e| apply_face(e, face)).collect()),
        Expr::Group(e) => Expr::Group(Box::new(apply_face(*e, face))),
        Expr::Sup(a, b) => Expr::Sup(Box::new(apply_face(*a, face)), Box::new(apply_face(*b, face))),
        Expr::Sub(a, b) => Expr::Sub(Box::new(apply_face(*a, face)), Box::new(apply_face(*b, face))),
        Expr::SubSup(a, b, c) => Expr::SubSup(
            Box::new(apply_face(*a, face)),
            Box::new(apply_face(*b, face)),
            Box::new(apply_face(*c, face)),
        ),
        Expr::Frac(a, b) => {
            Expr::Frac(Box::new(apply_face(*a, face)), Box::new(apply_face(*b, face)))
        }
        Expr::BinomFrac(a, b) => Expr::BinomFrac(
            Box::new(apply_face(*a, face)),
            Box::new(apply_face(*b, face)),
        ),
        Expr::BevelledFrac(a, b) => Expr::BevelledFrac(
            Box::new(apply_face(*a, face)),
            Box::new(apply_face(*b, face)),
        ),
        Expr::Sqrt(a) => Expr::Sqrt(Box::new(apply_face(*a, face))),
        Expr::Root(a, b) => {
            Expr::Root(Box::new(apply_face(*a, face)), Box::new(apply_face(*b, face)))
        }
        Expr::Fenced { open, close, body } => Expr::Fenced {
            open: map_text_face(&open, face),
            close: close.map(|c| map_text_face(&c, face)),
            body: Box::new(apply_face(*body, face)),
        },
        Expr::Table { open, close, rows } => Expr::Table {
            open,
            close,
            rows: rows
                .into_iter()
                .map(|r| r.into_iter().map(|c| apply_face(c, face)).collect())
                .collect(),
        },
        Expr::Over(a, b) => {
            Expr::Over(Box::new(apply_face(*a, face)), Box::new(apply_face(*b, face)))
        }
        Expr::Under(a, b) => {
            Expr::Under(Box::new(apply_face(*a, face)), Box::new(apply_face(*b, face)))
        }
        Expr::UnderOver(a, b, c) => Expr::UnderOver(
            Box::new(apply_face(*a, face)),
            Box::new(apply_face(*b, face)),
            Box::new(apply_face(*c, face)),
        ),
        Expr::Enclose { notation, body } => Expr::Enclose {
            notation,
            body: Box::new(apply_face(*body, face)),
        },
        Expr::Prescripts { base, sub, sup } => Expr::Prescripts {
            base: Box::new(apply_face(*base, face)),
            sub: sub.map(|e| Box::new(apply_face(*e, face))),
            sup: sup.map(|e| Box::new(apply_face(*e, face))),
        },
        Expr::MultiScripts { base, sub, sup } => Expr::MultiScripts {
            base: Box::new(apply_face(*base, face)),
            sub: sub.map(|e| Box::new(apply_face(*e, face))),
            sup: sup.map(|e| Box::new(apply_face(*e, face))),
        },
        other => other,
    }
}

fn map_text_face(s: &str, face: Face) -> String {
    match face {
        Face::Upright => s.to_string(),
        Face::Alpha {
            letters,
            digits,
            greek,
        } => map_text(s, letters, digits, greek),
    }
}

fn map_text(s: &str, letters: u32, digits: u32, greek: u32) -> String {
    s.chars()
        .map(|c| map_char(c, letters, digits, greek))
        .collect()
}

fn map_char(c: char, letters: u32, digits: u32, greek: u32) -> char {
    let cp = if c.is_ascii_uppercase() && letters != 0 {
        letters + (c as u32 - 'A' as u32)
    } else if c.is_ascii_lowercase() && letters != 0 {
        letters + 26 + (c as u32 - 'a' as u32)
    } else if c.is_ascii_digit() && digits != 0 {
        digits + (c as u32 - '0' as u32)
    } else if greek != 0 &&
        let Some(slot) = greek_slot(c)
    {
        greek + slot
    } else {
        return c;
    };
    hole_char(cp)
}

/// Index in the Mathematical Alphanumeric Greek run (bold capital alpha is 0).
fn greek_slot(c: char) -> Option<u32> {
    Some(match c {
        'Α' => 0,
        'Β' => 1,
        'Γ' => 2,
        'Δ' => 3,
        'Ε' => 4,
        'Ζ' => 5,
        'Η' => 6,
        'Θ' => 7,
        'Ι' => 8,
        'Κ' => 9,
        'Λ' => 10,
        'Μ' => 11,
        'Ν' => 12,
        'Ξ' => 13,
        'Ο' => 14,
        'Π' => 15,
        'Ρ' => 16,
        'ϴ' => 17,
        'Σ' => 18,
        'Τ' => 19,
        'Υ' => 20,
        'Φ' => 21,
        'Χ' => 22,
        'Ψ' => 23,
        'Ω' => 24,
        '∇' => 25,
        'α' => 26,
        'β' => 27,
        'γ' => 28,
        'δ' => 29,
        'ε' => 30,
        'ζ' => 31,
        'η' => 32,
        'θ' => 33,
        'ι' => 34,
        'κ' => 35,
        'λ' => 36,
        'μ' => 37,
        'ν' => 38,
        'ξ' => 39,
        'ο' => 40,
        'π' => 41,
        'ρ' => 42,
        'ς' => 43,
        'σ' => 44,
        'τ' => 45,
        'υ' => 46,
        'φ' => 47,
        'χ' => 48,
        'ψ' => 49,
        'ω' => 50,
        '∂' => 51,
        'ϵ' => 52,
        'ϑ' => 53,
        'ϰ' => 54,
        'ϕ' => 55,
        'ϱ' => 56,
        'ϖ' => 57,
        _ => return None,
    })
}

/// Holes in the Mathematical Alphanumeric Symbols block point at legacy BMP letters.
fn hole_char(cp: u32) -> char {
    let mapped = match cp {
        0x1D455 => 0x210E,
        0x1D49D => 0x212C,
        0x1D4A0 => 0x2130,
        0x1D4A1 => 0x2131,
        0x1D4A3 => 0x210B,
        0x1D4A4 => 0x2110,
        0x1D4A7 => 0x2112,
        0x1D4A8 => 0x2133,
        0x1D4AD => 0x211B,
        0x1D4BA => 0x212F,
        0x1D4BC => 0x210A,
        0x1D4C4 => 0x2134,
        0x1D506 => 0x212D,
        0x1D50B => 0x210C,
        0x1D50C => 0x2111,
        0x1D515 => 0x211C,
        0x1D51D => 0x2128,
        0x1D53A => 0x2102,
        0x1D53F => 0x210D,
        0x1D545 => 0x2115,
        0x1D547 => 0x2119,
        0x1D548 => 0x211A,
        0x1D549 => 0x211D,
        0x1D551 => 0x2124,
        other => other,
    };
    char::from_u32(mapped).unwrap_or(char::REPLACEMENT_CHARACTER)
}

fn decl_face(name: &str) -> Option<Face> {
    Some(match name {
        "rm" => Face::Upright,
        "bf" => Face::Alpha {
            letters: 0x1D400,
            digits: 0x1D7CE,
            greek: 0x1D6A8,
        },
        "it" => Face::Alpha {
            letters: 0x1D434,
            digits: 0,
            greek: 0x1D6E2,
        },
        "cal" => Face::Alpha {
            letters: 0x1D49C,
            digits: 0,
            greek: 0,
        },
        "frak" => Face::Alpha {
            letters: 0x1D504,
            digits: 0,
            greek: 0,
        },
        "sf" => Face::Alpha {
            letters: 0x1D5A0,
            digits: 0x1D7E2,
            greek: 0,
        },
        "tt" => Face::Alpha {
            letters: 0x1D670,
            digits: 0x1D7F6,
            greek: 0,
        },
        _ => return None,
    })
}

fn space_for_mu(mu: f64) -> char {
    if mu >= 14.0 {
        '\u{2003}'
    } else if mu >= 9.0 {
        '\u{2002}'
    } else if mu >= 4.5 {
        '\u{2004}'
    } else if mu >= 3.5 {
        '\u{205F}'
    } else if mu >= 2.0 {
        '\u{2009}'
    } else {
        '\u{200B}'
    }
}

fn mu_to_space(mu: f64) -> Expr {
    if mu < 2.0 {
        return Expr::Operator("\u{200B}".into());
    }
    let mu = mu.min(2160.0);
    if mu < 18.0 {
        return Expr::Operator(space_for_mu(mu).to_string());
    }
    let ems = (mu / 18.0).floor() as usize;
    let rem = mu - (ems as f64) * 18.0;
    let mut s = "\u{2003}".repeat(ems);
    if rem >= 2.0 {
        s.push(space_for_mu(rem));
    }
    Expr::Operator(s)
}

fn fence_spec(spec: &str) -> Option<String> {
    let t = spec.trim();
    if t.is_empty() || t == "." {
        return None;
    }
    Some(match t {
        "(" | ")" | "[" | "]" | "{" | "}" | "|" => t.to_string(),
        "\\{" | "\\lbrace" => "{".into(),
        "\\}" | "\\rbrace" => "}".into(),
        "\\lbrack" => "[".into(),
        "\\rbrack" => "]".into(),
        "\\vert" | "\\lvert" | "\\rvert" => "|".into(),
        "\\|" | "\\Vert" | "\\lVert" | "\\rVert" => "‖".into(),
        "\\langle" => "⟨".into(),
        "\\rangle" => "⟩".into(),
        other => other.trim_start_matches('\\').to_string(),
    })
}

fn thickness_is_zero(spec: &str) -> bool {
    let t = spec.trim();
    if t.is_empty() {
        return false;
    }
    let bytes = t.as_bytes();
    let mut i = 0;
    if bytes.first() == Some(&b'+') || bytes.first() == Some(&b'-') {
        i = 1;
    }
    let start = i;
    let mut saw_digit = false;
    let mut saw_dot = false;
    while i < bytes.len() && (bytes[i].is_ascii_digit() || (bytes[i] == b'.' && !saw_dot)) {
        if bytes[i].is_ascii_digit() {
            saw_digit = true;
        }
        if bytes[i] == b'.' {
            saw_dot = true;
        }
        i += 1;
    }
    saw_digit && t[start..i].parse::<f64>().unwrap_or(1.0) == 0.0
}

fn fence_frac(frac: Expr, left: Option<String>, right: Option<String>) -> Expr {
    match (left, right) {
        (None, None) => frac,
        (Some(open), close) => Expr::Fenced {
            open,
            close,
            body: Box::new(frac),
        },
        (None, Some(close)) => Expr::Row(vec![frac, Expr::Operator(close)]),
    }
}

fn takes_column_spec(env: &str) -> bool {
    matches!(
        env,
        "array" | "array*" | "subarray" | "alignat" | "alignat*" | "alignedat"
    )
}

fn unwrap_equation(expr: Expr) -> Expr {
    match expr {
        Expr::Table {
            open: None,
            close: None,
            mut rows,
        } if rows.len() == 1 && rows[0].len() == 1 => rows.pop().unwrap().pop().unwrap(),
        other => other,
    }
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
        "frac" | "f" | "dfrac" | "tfrac" | "cfrac" | "ifrac"
            | "sqrt" | "s"
            | "binom" | "dbinom" | "tbinom"
            | "overline" | "ol"
            | "underline"
            | "vec"
            | "hat"
            | "dot"
            | "ddot"
            | "tilde"
            | "widehat"
            | "widetilde"
            | "check"
            | "acute"
            | "grave"
            | "breve"
            | "bar"
            | "mathring"
            | "dddot"
            | "ddddot"
            | "overrightarrow"
            | "overleftarrow"
            | "overleftrightarrow"
            | "underrightarrow"
            | "underleftarrow"
            | "underleftrightarrow"
            | "overbrace"
            | "underbrace"
            | "overset"
            | "underset"
            | "stackrel"
            | "boxed"
            | "circle"
            | "overwrite"
            | "begin"
            | "left"
            | "text"
            | "substack"
            | "operatorname"
            | "bmod"
            | "pmod"
            | "mod"
            | "phantom"
            | "hphantom"
            | "vphantom"
            | "cancel"
            | "bcancel"
            | "xcancel"
            | "fbox"
            | "framebox"
            | "middle"
            | "mathrel"
            | "mathop"
            | "mathrm"
            | "mathbf"
            | "mathit"
            | "mathsf"
            | "mathtt"
            | "mathbb"
            | "Bbb"
            | "mathcal"
            | "mathscr"
            | "mathfrak"
            | "boldsymbol"
            | "bm"
            | "textrm"
            | "textnormal"
            | "textbf"
            | "textit"
            | "emph"
            | "textsf"
            | "texttt"
            | "mbox"
            | "hbox"
            | "hspace"
            | "bra"
            | "ket"
            | "xrightarrow"
            | "xleftarrow"
            | "genfrac"
            | "root"
            | "eqalign"
            | "color"
            | "textcolor"
    )
}

fn is_ignored_command(name: &str) -> bool {
    matches!(
        name,
        "displaystyle" | "textstyle" | "scriptstyle" | "scriptscriptstyle"
            | "tiny" | "scriptsize" | "footnotesize" | "small" | "normalsize"
            | "large" | "Large" | "LARGE" | "huge" | "Huge"
            | "notag" | "nonumber"
    )
}

fn is_ignored_with_arg(name: &str) -> bool {
    matches!(name, "tag" | "label" | "ref" | "eqref")
}

fn is_big_delim(name: &str) -> bool {
    matches!(
        name,
        "big" | "Big" | "bigg" | "Bigg"
            | "bigl" | "bigr" | "bigm"
            | "Bigl" | "Bigr" | "Bigm"
            | "biggl" | "biggr" | "biggm"
            | "Biggl" | "Biggr" | "Biggm"
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
        "cases" => (Some("{".into()), None),
        "rcases" => (None, Some("}".into())),
        // `matrix`, `array`, `smallmatrix`, `split`, `align`, and `equation` are bare.
        _ => (None, None),
    }
}

const FUNC_NAMES: &[&str] = &[
    "arccsch", "arcsech", "arccoth", "arcsinh", "arccosh", "arctanh", "arcsin", "arccos",
    "arctan", "arccsc", "arcsec", "arccot", "varinjlim", "varprojlim", "varliminf",
    "varlimsup", "injlim", "projlim", "liminf", "limsup", "sinh", "cosh", "tanh", "csch",
    "sech", "coth", "sin", "cos", "tan", "csc", "sec", "cot", "deg", "det", "dim", "exp",
    "gcd", "hom", "inf", "ker", "lg", "lim", "ln", "log", "max", "min", "Pr", "sup", "arg",
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

    #[test]
    fn office_symbol_aliases_reuse_existing_characters() {
        // Names from the Microsoft 365 list that spell a character MathCAT already emits
        // under another command. `\jmath` is the one new character (U+0237).
        let cases = [
            (r"\neq", "&#x2260;"),
            (r"\ne", "&#x2260;"),
            (r"\emptyset", "&#x2205;"),
            (r"\neg", "&#xAC;"),
            (r"\bot", "&#x22A5;"),
            (r"\prime", "&#x2032;"),
            (r"\omicron", "&#x3BF;"),
            (r"\varGamma", "&#x393;"),
            (r"\varOmega", "&#x3A9;"),
            (r"\diamond", "&#x22C4;"),
            (r"\Diamond", "&#x25C7;"),
            (r"\lozenge", "&#x25CA;"),
            (r"\blacklozenge", "&#x29EB;"),
            (r"\bigcirc", "&#x25EF;"),
            (r"\dagger", "&#x2020;"),
            (r"\ddagger", "&#x2021;"),
            (r"\hbar", "&#x210F;"),
            (r"\circlearrowleft", "&#x21BA;"),
            (r"\dashrightarrow", "&#x21E2;"),
            (r"\iddots", "&#x22F0;"),
            (r"\pvint", "&#x2A0D;"),
            (r"\dots", "&#x2026;"),
            (r"\dotsb", "&#x22EF;"),
            (r"\dotsc", "&#x2026;"),
            (r"\nleq", "&#x2270;"),
            (r"\ntriangleleft", "&#x22EA;"),
            (r"\doteqdot", "&#x2251;"),
            (r"\quad", "&#x2003;"),
            (r"\enspace", "&#x2002;"),
            (r"\medspace", "&#x205F;"),
            (r"\jmath", "&#x237;"),
            (r"\lbrace", "{"),
            (r"\lvert", "|"),
            (r"\lVert", "&#x2016;"),
        ];
        for (src, needle) in cases {
            let out = mml(src);
            assert!(out.contains(needle), "{src} -> {out}");
        }
        // `\neq` must not swallow the letters of a longer name, and `\not=` stays ≠.
        assert_eq!(mml(r"\neq"), mml(r"\not="));
        assert!(mml(r"\infty").contains("&#x221E;"), "{}", mml(r"\infty"));
    }

    #[test]
    fn spacing_commands_and_short_names() {
        // `\,` is a thin space here. The forward template for U+201A stays `\,`.
        let thin = mml(r"a\,b");
        assert!(thin.contains("&#x2009;"), "{thin}");
        assert!(mml(r"a\!b").contains("&#x200B;"));
        assert!(mml(r"a\:b").contains("&#x205F;"));
        assert!(mml(r"a\;b").contains("&#x2004;"));
        assert!(mml("~").contains("&#xA0;"));
        // `~p` is still the short name for pi, not a space plus p.
        assert_eq!(mml("~p"), mml(r"\pi"));
        assert!(mml(r"\qquad").contains("&#x2003;&#x2003;"));
        assert!(mml(r"\arg").contains("<mi>arg</mi>"));
        let lim = mml(r"\varinjlim_n");
        assert!(lim.contains("<munder>"), "{lim}");
        assert!(lim.contains("varinjlim"), "{lim}");
    }

    #[test]
    fn ignored_commands_leave_the_math() {
        // Style, size, tags, and delimiter grades are accepted and dropped.
        assert_eq!(mml(r"\displaystyle x"), mml("x"));
        assert_eq!(mml(r"\Large x"), mml("x"));
        assert_eq!(mml(r"\tag{1}x"), mml("x"));
        assert_eq!(mml(r"\label{eq}x"), mml("x"));
        assert_eq!(mml(r"\notag x"), mml("x"));
        assert_eq!(mml(r"\bigl("), mml("("));
        assert_eq!(mml(r"\Biggr)"), mml(")"));
    }

    #[test]
    fn fraction_accent_and_font_commands() {
        assert_eq!(mml(r"\dfrac{1}{2}"), mml(r"\frac{1}{2}"));
        assert_eq!(mml(r"\cfrac[l]{1}{2}"), mml(r"\frac{1}{2}"));
        assert_eq!(mml(r"\tbinom{n}{k}"), mml(r"\binom{n}{k}"));
        let skewed = mml(r"\ifrac{1}{2}");
        assert!(skewed.contains("bevelled=\"true\""), "{skewed}");
        assert!(skewed.contains("<mn>1</mn>"), "{skewed}");
        let hat = mml(r"\widehat{x}");
        assert!(hat.contains("<mover>"), "{hat}");
        assert!(hat.contains("<mo>^</mo>"), "{hat}");
        assert_eq!(mml(r"\stackrel{a}{b}"), mml(r"\overset{a}{b}"));
        let under = mml(r"\underrightarrow{x}");
        assert!(under.contains("<munder>"), "{under}");
        // Bold x is U+1D431. Upright x keeps ASCII and mathvariant.
        assert!(mml(r"\mathbf{x}").contains("&#x1D431;"));
        assert!(mml(r"\mathrm{x}").contains("mathvariant=\"normal\">x</mi>"));
        // Script B is the BMP hole U+212C, not the unassigned slot.
        assert!(mml(r"\mathcal{B}").contains("&#x212C;"));
        assert!(mml(r"\mathbb{R}").contains("&#x211D;"));
        assert!(mml(r"\mathfrak{g}").contains("&#x1D524;"));
        assert!(mml(r"\mathbf{\alpha}").contains("&#x1D6C2;"));
        assert!(mml(r"\mbox{hi}").contains("<mtext>hi</mtext>"));
    }

    #[test]
    fn enclose_limits_mod_and_substack() {
        assert!(mml(r"\cancel{x}").contains("notation=\"updiagonalstrike\""));
        assert!(mml(r"\bcancel{x}").contains("notation=\"downdiagonalstrike\""));
        let cross = mml(r"\xcancel{x}");
        assert!(
            cross.contains("notation=\"updiagonalstrike downdiagonalstrike\""),
            "{cross}"
        );
        assert!(mml(r"\phantom{x}").contains("notation=\"phantom\""));
        assert!(mml(r"\fbox{x}").contains("notation=\"box\""));
        let nolimits = mml(r"\sum\nolimits_i");
        assert!(nolimits.contains("<msub>"), "{nolimits}");
        assert!(!nolimits.contains("<munder>"), "{nolimits}");
        let forced = mml(r"x\limits_i");
        assert!(forced.contains("<munder>"), "{forced}");
        let op = mml(r"\mathop{x}_i^n");
        assert!(op.contains("<munderover>"), "{op}");
        assert_eq!(mml(r"\mathrel{=}"), mml("="));
        assert!(mml(r"\operatorname{sin}").contains("<mi>sin</mi>"));
        let lim = mml(r"\operatorname{lim}_n");
        assert!(lim.contains("<munder>"), "{lim}");
        let pmod = mml(r"\pmod{2}");
        assert!(pmod.contains("<mo>(</mo>"), "{pmod}");
        assert!(pmod.contains(">mod</mi>"), "{pmod}");
        assert!(pmod.contains("<mn>2</mn>"), "{pmod}");
        assert!(mml(r"a \bmod b").contains(">mod</mi>"));
        let stack = mml(r"\substack{a \\ b}");
        assert!(stack.contains("<mtable>"), "{stack}");
        assert_eq!(stack.matches("<mtr>").count(), 2, "{stack}");
        let sub = mml(r"\begin{subarray}{c} a \\ b \end{subarray}");
        assert_eq!(sub.matches("<mtr>").count(), 2, "{sub}");
        let mid = mml(r"\left( a \middle| b \right)");
        assert!(mid.contains("<mo>(</mo>"), "{mid}");
        assert!(mid.contains("<mo>|</mo>"), "{mid}");
        assert!(mid.contains("<mo>)</mo>"), "{mid}");
    }

    #[test]
    fn ms_alias_does_not_eat_a_longer_command() {
        // A shorter alias must stop at a letter boundary. `\ne` is ≠, `\neg` is ¬,
        // and `\nleq` is not the existing `\nless`.
        assert_eq!(mml(r"\neg"), "<math><mo>&#xAC;</mo></math>");
        assert_eq!(mml(r"\nleq"), "<math><mo>&#x2270;</mo></math>");
        assert_eq!(mml(r"\ngeq"), "<math><mo>&#x2271;</mo></math>");
        assert_eq!(mml(r"\nsim"), "<math><mo>&#x2241;</mo></math>");
        assert_eq!(mml(r"\ncong"), "<math><mo>&#x2247;</mo></math>");
        assert_eq!(mml(r"\dots"), "<math><mo>&#x2026;</mo></math>");
        assert_eq!(mml(r"\dotsb"), "<math><mo>&#x22EF;</mo></math>");
        assert_eq!(mml(r"\dotsm"), "<math><mo>&#x22EF;</mo></math>");
        assert_eq!(mml(r"\dotso"), "<math><mo>&#x2026;</mo></math>");
        assert_eq!(mml(r"\quad"), "<math><mo>&#x2003;</mo></math>");
        assert_eq!(
            mml(r"\qquad"),
            "<math><mo>&#x2003;&#x2003;</mo></math>"
        );
        assert_eq!(mml(r"\thickspace"), "<math><mo>&#x2004;</mo></math>");
        assert_eq!(mml(r"\jmath"), "<math><mi>&#x237;</mi></math>");
    }

    #[test]
    fn ms_fences_and_greek_aliases() {
        // Fence names are the same glyphs as the ASCII fences, including inside `\left`.
        assert_eq!(
            mml(r"\left\lbrace x \right\rbrace"),
            "<math><mrow><mo>{</mo><mi>x</mi><mo>}</mo></mrow></math>"
        );
        assert_eq!(
            mml(r"\left\lvert x \right\rvert"),
            "<math><mrow><mo>|</mo><mi>x</mi><mo>|</mo></mrow></math>"
        );
        assert_eq!(
            mml(r"\left\lVert x \right\|"),
            "<math><mrow><mo>&#x2016;</mo><mi>x</mi><mo>&#x2016;</mo></mrow></math>"
        );
        assert_eq!(mml(r"\lbrack"), "<math><mo>[</mo></math>");
        assert_eq!(mml(r"\rbrack"), "<math><mo>]</mo></math>");
        assert_eq!(mml(r"\varDelta"), "<math><mi>&#x394;</mi></math>");
        assert_eq!(mml(r"\varPhi"), "<math><mi>&#x3A6;</mi></math>");
        assert_eq!(mml(r"\circlearrowright"), "<math><mo>&#x21BB;</mo></math>");
        assert_eq!(mml(r"\dashleftarrow"), "<math><mo>&#x21E0;</mo></math>");
        assert_eq!(mml(r"\ntriangleright"), "<math><mo>&#x22EB;</mo></math>");
        assert_eq!(mml(r"\ntrianglelefteq"), "<math><mo>&#x22EC;</mo></math>");
        assert_eq!(mml(r"\ntrianglerighteq"), "<math><mo>&#x22ED;</mo></math>");
    }

    #[test]
    fn ms_spacing_and_dropped_commands() {
        // Punctuation spaces and the commands Office parses and then discards.
        assert_eq!(
            mml(r"a\ b"),
            "<math><mrow><mi>a</mi><mo> </mo><mi>b</mi></mrow></math>"
        );
        assert_eq!(mml(r"a\space b"), mml(r"a\ b"));
        assert_eq!(mml(r"\textstyle x"), "<math><mi>x</mi></math>");
        assert_eq!(mml(r"\scriptstyle x"), "<math><mi>x</mi></math>");
        assert_eq!(mml(r"\scriptsize x"), "<math><mi>x</mi></math>");
        assert_eq!(mml(r"\Huge x"), "<math><mi>x</mi></math>");
        assert_eq!(mml(r"\eqref{1}x"), "<math><mi>x</mi></math>");
        assert_eq!(mml(r"\ref{1}x"), "<math><mi>x</mi></math>");
        assert_eq!(mml(r"\nonumber x"), "<math><mi>x</mi></math>");
        assert_eq!(mml(r"\bigm|"), "<math><mo>|</mo></math>");
        assert_eq!(mml(r"\Bigl("), "<math><mo>(</mo></math>");
    }

    #[test]
    fn ms_fraction_aliases_match_frac_and_binom() {
        // Size and alignment on these names are not stored. `\ifrac` is bevelled.
        let frac = "<math><mfrac><mn>1</mn><mn>2</mn></mfrac></math>";
        assert_eq!(mml(r"\tfrac{1}{2}"), frac);
        assert_eq!(mml(r"\cfrac[r]{1}{2}"), frac);
        assert_eq!(
            mml(r"\dfrac"),
            "<math><mfrac><mrow></mrow><mrow></mrow></mfrac></math>"
        );
        assert_eq!(mml(r"\dbinom{n}{k}"), mml(r"\binom{n}{k}"));
        assert_eq!(
            mml(r"\ifrac{1}{2}"),
            "<math><mfrac bevelled=\"true\"><mn>1</mn><mn>2</mn></mfrac></math>"
        );
    }

    #[test]
    fn ms_accents_are_over_or_under() {
        // One argument, same shape as `\hat` and `\overline`.
        assert_eq!(
            mml(r"\widetilde{x}"),
            "<math><mover><mi>x</mi><mo>~</mo></mover></math>"
        );
        assert_eq!(
            mml(r"\check{x}"),
            "<math><mover><mi>x</mi><mo>&#x2C7;</mo></mover></math>"
        );
        assert_eq!(
            mml(r"\acute{x}"),
            "<math><mover><mi>x</mi><mo>&#xB4;</mo></mover></math>"
        );
        assert_eq!(
            mml(r"\grave{x}"),
            "<math><mover><mi>x</mi><mo>`</mo></mover></math>"
        );
        assert_eq!(
            mml(r"\breve{x}"),
            "<math><mover><mi>x</mi><mo>&#x2D8;</mo></mover></math>"
        );
        assert_eq!(
            mml(r"\bar{x}"),
            "<math><mover><mi>x</mi><mo>&#xAF;</mo></mover></math>"
        );
        assert_eq!(
            mml(r"\mathring{x}"),
            "<math><mover><mi>x</mi><mo>&#x2DA;</mo></mover></math>"
        );
        assert_eq!(
            mml(r"\dddot{x}"),
            "<math><mover><mi>x</mi><mo>...</mo></mover></math>"
        );
        assert_eq!(
            mml(r"\ddddot{x}"),
            "<math><mover><mi>x</mi><mo>....</mo></mover></math>"
        );
        assert_eq!(
            mml(r"\overrightarrow{x}"),
            "<math><mover><mi>x</mi><mo>&#x2192;</mo></mover></math>"
        );
        assert_eq!(
            mml(r"\overleftarrow{x}"),
            "<math><mover><mi>x</mi><mo>&#x2190;</mo></mover></math>"
        );
        assert_eq!(
            mml(r"\overleftrightarrow{x}"),
            "<math><mover><mi>x</mi><mo>&#x2194;</mo></mover></math>"
        );
        assert_eq!(
            mml(r"\underleftarrow{x}"),
            "<math><munder><mi>x</mi><mo>&#x2190;</mo></munder></math>"
        );
        assert_eq!(
            mml(r"\underleftrightarrow{x}"),
            "<math><munder><mi>x</mi><mo>&#x2194;</mo></munder></math>"
        );
    }

    #[test]
    fn ms_font_commands_map_letters() {
        // Letters move into the math alphanumeric block. Upright roman stays ASCII.
        // Script B and double-struck Z use the BMP holes.
        assert_eq!(mml(r"\mathit{x}"), "<math><mi>&#x1D465;</mi></math>");
        assert_eq!(mml(r"\mathsf{A}"), "<math><mi>&#x1D5A0;</mi></math>");
        assert_eq!(mml(r"\mathtt{A}"), "<math><mi>&#x1D670;</mi></math>");
        assert_eq!(mml(r"\Bbb{Z}"), "<math><mi>&#x2124;</mi></math>");
        assert_eq!(mml(r"\mathscr{B}"), "<math><mi>&#x212C;</mi></math>");
        assert_eq!(mml(r"\boldsymbol{x}"), "<math><mi>&#x1D499;</mi></math>");
        assert_eq!(mml(r"\bm{x}"), mml(r"\boldsymbol{x}"));
        assert_eq!(
            mml(r"\mathrm{sin}"),
            "<math><mi mathvariant=\"normal\">sin</mi></math>"
        );
        assert_eq!(mml(r"\textrm{x}"), mml(r"\mathrm{x}"));
        assert_eq!(mml(r"\textnormal{x}"), mml(r"\mathrm{x}"));
        assert_eq!(mml(r"\textbf{x}"), mml(r"\mathbf{x}"));
        assert_eq!(mml(r"\textit{x}"), mml(r"\mathit{x}"));
        assert_eq!(mml(r"\emph{x}"), mml(r"\mathit{x}"));
        assert_eq!(mml(r"\textsf{A}"), mml(r"\mathsf{A}"));
        assert_eq!(mml(r"\texttt{A}"), mml(r"\mathtt{A}"));
        assert_eq!(mml(r"\hbox{hi}"), "<math><mtext>hi</mtext></math>");
    }

    #[test]
    fn ms_operators_limits_and_enclose() {
        // New operator names. Limit names take the script below the name.
        assert_eq!(
            mml(r"\injlim_n"),
            "<math><munder><mo>injlim</mo><mi>n</mi></munder></math>"
        );
        assert_eq!(
            mml(r"\projlim_n"),
            "<math><munder><mo>projlim</mo><mi>n</mi></munder></math>"
        );
        assert_eq!(
            mml(r"\varprojlim_n"),
            "<math><munder><mo>varprojlim</mo><mi>n</mi></munder></math>"
        );
        assert_eq!(
            mml(r"\varliminf_n"),
            "<math><munder><mo>varliminf</mo><mi>n</mi></munder></math>"
        );
        assert_eq!(
            mml(r"\varlimsup_n"),
            "<math><munder><mo>varlimsup</mo><mi>n</mi></munder></math>"
        );
        assert_eq!(
            mml(r"\mod{2}"),
            "<math><mrow><mi>mod</mi><mo>&#xA0;</mo><mn>2</mn></mrow></math>"
        );
        assert_eq!(
            mml(r"\hphantom{x}"),
            "<math><menclose notation=\"hphantom\"><mi>x</mi></menclose></math>"
        );
        assert_eq!(
            mml(r"\vphantom{x}"),
            "<math><menclose notation=\"vphantom\"><mi>x</mi></menclose></math>"
        );
        assert_eq!(mml(r"\framebox{x}"), mml(r"\boxed{x}"));
        assert_eq!(
            mml(r"\mathop{\sum}_i"),
            "<math><munder><mo>&#x2211;</mo><mi>i</mi></munder></math>"
        );
    }

    #[test]
    fn group1_lengths_map_to_unicode_spaces() {
        // 1 em = 18 mu. Whole ems are U+2003; the remainder uses the same spaces as `\,`.
        assert_eq!(mml(r"\mkern18mu"), "<math><mo>&#x2003;</mo></math>");
        assert_eq!(mml(r"\mskip 20mu"), "<math><mo>&#x2003;&#x2009;</mo></math>");
        assert_eq!(mml(r"\hspace{0.5em}"), "<math><mo>&#x2002;</mo></math>");
        assert_eq!(mml(r"\mkern5mu"), "<math><mo>&#x2004;</mo></math>");
        assert_eq!(mml(r"\mkern4mu"), "<math><mo>&#x205F;</mo></math>");
        assert_eq!(mml(r"\mkern3mu"), "<math><mo>&#x2009;</mo></math>");
        assert_eq!(mml(r"\mkern0mu"), "<math><mo>&#x200B;</mo></math>");
        assert_eq!(mml(r"\mkern-2mu"), "<math><mo>&#x200B;</mo></math>");
        assert_eq!(
            mml(r"a\mkern18mu b"),
            "<math><mrow><mi>a</mi><mo>&#x2003;</mo><mi>b</mi></mrow></math>"
        );
    }

    #[test]
    fn group1_declaration_fonts_last_until_the_group_ends() {
        // `\bf` / `\it` apply to later letters. An explicit `\mathbf` ignores the outer face.
        assert_eq!(mml(r"{\bf x}"), "<math><mi>&#x1D431;</mi></math>");
        assert_eq!(mml(r"{\rm x}"), "<math><mi mathvariant=\"normal\">x</mi></math>");
        assert_eq!(mml(r"\cal B"), "<math><mi>&#x212C;</mi></math>");
        assert_eq!(mml(r"\frak A"), "<math><mi>&#x1D504;</mi></math>");
        assert_eq!(mml(r"\sf A"), "<math><mi>&#x1D5A0;</mi></math>");
        assert_eq!(mml(r"\tt A"), "<math><mi>&#x1D670;</mi></math>");
        assert_eq!(mml(r"{\bf 2}"), "<math><mn>&#x1D7D0;</mn></math>");
        assert_eq!(
            mml(r"{\bf x\it y}"),
            "<math><mrow><mi>&#x1D431;</mi><mi>&#x1D466;</mi></mrow></math>"
        );
        assert_eq!(
            mml(r"{{\bf x}y}"),
            "<math><mrow><mi>&#x1D431;</mi><mi>y</mi></mrow></math>"
        );
        assert_eq!(mml(r"{\it \mathbf{x}}"), "<math><mi>&#x1D431;</mi></math>");
        assert_eq!(
            mml(r"{\bf \mathrm{x}}"),
            "<math><mi mathvariant=\"normal\">x</mi></math>"
        );
    }

    #[test]
    fn group1_fences_tables_arrows_and_roots() {
        // Bra/ket, double brackets, case fences, equation as a single cell, and `\root`.
        assert_eq!(
            mml(r"\bra{x}"),
            "<math><mrow><mo>&#x27E8;</mo><mi>x</mi><mo>&#x2223;</mo></mrow></math>"
        );
        assert_eq!(
            mml(r"\ket{x}"),
            "<math><mrow><mo>&#x2223;</mo><mi>x</mi><mo>&#x27E9;</mo></mrow></math>"
        );
        assert_eq!(mml(r"[\!["), "<math><mo>&#x27E6;</mo></math>");
        assert_eq!(mml(r"]\!]"), "<math><mo>&#x27E7;</mo></math>");
        assert_eq!(mml(r"\lbrack\!\lbrack"), mml(r"[\!["));
        assert_eq!(
            mml(r"\left[\![ x \right]\!]"),
            "<math><mrow><mo>&#x27E6;</mo><mi>x</mi><mo>&#x27E7;</mo></mrow></math>"
        );
        assert_eq!(
            mml(r"\begin{matrix} a \cr b \end{matrix}"),
            "<math><mtable><mtr><mtd><mi>a</mi></mtd></mtr><mtr><mtd><mi>b</mi></mtd></mtr></mtable></math>"
        );
        assert_eq!(
            mml(r"\begin{cases} a & b \end{cases}"),
            "<math><mrow><mo>{</mo><mtable><mtr><mtd><mi>a</mi></mtd><mtd><mi>b</mi></mtd></mtr></mtable></mrow></math>"
        );
        assert_eq!(
            mml(r"\begin{rcases} a \end{rcases}"),
            "<math><mrow><mtable><mtr><mtd><mi>a</mi></mtd></mtr></mtable><mo>}</mo></mrow></math>"
        );
        assert_eq!(mml(r"\begin{equation} x \end{equation}"), "<math><mi>x</mi></math>");
        assert_eq!(mml(r"\begin{equation*} x \end{equation*}"), mml("x"));
        assert_eq!(
            mml(r"\begin{alignat}{2} a & b \end{alignat}"),
            "<math><mtable><mtr><mtd><mi>a</mi></mtd><mtd><mi>b</mi></mtd></mtr></mtable></math>"
        );
        assert_eq!(
            mml(r"\begin{smallmatrix} a \end{smallmatrix}"),
            "<math><mtable><mtr><mtd><mi>a</mi></mtd></mtr></mtable></math>"
        );
        assert_eq!(
            mml(r"\eqalign{a & b \cr c}"),
            "<math><mtable><mtr><mtd><mi>a</mi></mtd><mtd><mi>b</mi></mtd></mtr><mtr><mtd><mi>c</mi></mtd></mtr></mtable></math>"
        );
        assert_eq!(
            mml(r"\xrightarrow{n}"),
            "<math><mover><mo>&#x2192;</mo><mi>n</mi></mover></math>"
        );
        assert_eq!(
            mml(r"\xrightarrow[a]{b}"),
            "<math><munderover><mo>&#x2192;</mo><mi>a</mi><mi>b</mi></munderover></math>"
        );
        assert_eq!(
            mml(r"\xleftarrow[a]{}"),
            "<math><munder><mo>&#x2190;</mo><mi>a</mi></munder></math>"
        );
        assert_eq!(
            mml(r"\genfrac{(}{)}{0pt}{}{n}{k}"),
            "<math><mrow><mo>(</mo><mfrac linethickness=\"0\"><mi>n</mi><mi>k</mi></mfrac><mo>)</mo></mrow></math>"
        );
        assert_eq!(
            mml(r"\genfrac{}{}{}{}{1}{2}"),
            "<math><mfrac><mn>1</mn><mn>2</mn></mfrac></math>"
        );
        assert_eq!(
            mml(r"\root 3 \of {x}"),
            "<math><mroot><mi>x</mi><mn>3</mn></mroot></math>"
        );
        assert_eq!(mml(r"\color{red}{x}"), mml("x"));
        assert_eq!(mml(r"\textcolor{#FF0000}{x+1}"), mml("x+1"));
        assert_eq!(mml(r"\color{blue}x"), mml("x"));
    }

    #[test]
    fn infix_fractions_are_scoped_to_the_group() {
        // The keyword splits the group. A second keyword splits the denominator.
        assert_eq!(
            mml(r"{a \over b}"),
            "<math><mfrac><mi>a</mi><mi>b</mi></mfrac></math>"
        );
        assert_eq!(
            mml(r"a \over b"),
            "<math><mfrac><mi>a</mi><mi>b</mi></mfrac></math>"
        );
        assert_eq!(mml(r"{a \choose b}"), mml(r"\binom{a}{b}"));
        assert_eq!(
            mml(r"{a \choose b}c"),
            "<math><mrow><mrow><mo>(</mo><mfrac linethickness=\"0\"><mi>a</mi><mi>b</mi></mfrac><mo>)</mo></mrow><mi>c</mi></mrow></math>"
        );
        assert_eq!(
            mml(r"{a \choose bc}"),
            "<math><mrow><mo>(</mo><mfrac linethickness=\"0\"><mi>a</mi><mrow><mi>b</mi><mi>c</mi></mrow></mfrac><mo>)</mo></mrow></math>"
        );
        assert_eq!(
            mml(r"{a \atop b}"),
            "<math><mfrac linethickness=\"0\"><mi>a</mi><mi>b</mi></mfrac></math>"
        );
        assert_eq!(
            mml(r"{n \brace k}"),
            "<math><mrow><mo>{</mo><mfrac linethickness=\"0\"><mi>n</mi><mi>k</mi></mfrac><mo>}</mo></mrow></math>"
        );
        assert_eq!(
            mml(r"{n \brack k}"),
            "<math><mrow><mo>[</mo><mfrac linethickness=\"0\"><mi>n</mi><mi>k</mi></mfrac><mo>]</mo></mrow></math>"
        );
        assert_eq!(mml(r"{a \over b \over c}"), mml(r"{a \over {b \over c}}"));
        assert_eq!(
            mml(r"\buildrel a \over x"),
            "<math><mover><mi>x</mi><mi>a</mi></mover></math>"
        );
        assert_eq!(
            mml(r"{\buildrel a \over x}c"),
            "<math><mrow><mover><mi>x</mi><mi>a</mi></mover><mi>c</mi></mrow></math>"
        );
        // `\overline` is still one accent, not `\over` plus `line`.
        assert_eq!(
            mml(r"\overline{x}"),
            "<math><mover><mi>x</mi><mo>&#x2015;</mo></mover></math>"
        );
    }

    #[test]
    fn cd_arrows_are_only_inside_the_environment() {
        // `@>>>` is an arrow in `CD`. The same text in a matrix is ordinary characters.
        assert_eq!(
            mml(r"\begin{CD} A @>>> B \end{CD}"),
            "<math><mtable><mtr><mtd><mrow><mi>A</mi><mo>&#x2192;</mo><mi>B</mi></mrow></mtd></mtr></mtable></math>"
        );
        assert_eq!(
            mml(r"\begin{CD} A & @>>> & B \\ @VVV & @AAA & @<<< \end{CD}"),
            "<math><mtable><mtr><mtd><mi>A</mi></mtd><mtd><mo>&#x2192;</mo></mtd><mtd><mi>B</mi></mtd></mtr><mtr><mtd><mo>&#x2193;</mo></mtd><mtd><mo>&#x2191;</mo></mtd><mtd><mo>&#x2190;</mo></mtd></mtr></mtable></math>"
        );
        assert_eq!(
            mml(r"\begin{CD} @>>>> \end{CD}"),
            "<math><mtable><mtr><mtd><mo>&#x2192;</mo></mtd></mtr></mtable></math>"
        );
        assert!(!mml(r"@>>>").contains("&#x2192;"), "{}", mml(r"@>>>"));
        assert!(!mml(r"\begin{matrix} @>>> \end{matrix}").contains("&#x2192;"));
    }
}
