//! ASCIIMath → MathML parser.
//!
//! Follows the grammar published at <https://asciimath.org/> (the same
//! recursive descent as ASCIIMathML.js). Incomplete input is kept: a missing
//! argument or closer becomes [`Expr::Missing`] or an unclosed fence instead of
//! failing the parse. Symbol lookup is longest-match, including LaTeX command
//! names from the unicode tables (`\partial`, `\partialmeetcontraction`, …).

#![allow(clippy::needless_return)]

use super::Expr;
use crate::asciimath_symbols::{self, Kind, Sym};
use crate::errors::Result;
use crate::prefs::PreferenceManager;

struct Parsed {
    expr: Expr,
    /// Base was `sum` / `lim` / `ubrace` / … so scripts are under/over.
    underover: bool,
    func: bool,
    /// Character length of the function name (`f` is 1).
    func_chars: usize,
}

struct Parser {
    input: String,
    pos: usize,
    nesting: i32,
    /// Inside `|…|`, a second `|` ends the group.
    stop_leftright: bool,
    prev_kind: Kind,
    def_depth: u8,
    /// ASCII byte of the decimal separator (`.` or `,`).
    decimal: u8,
}

impl Parser {
    fn new(input: &str) -> Self {
        let decimal = decimal_sep();
        Self {
            input: input.to_string(),
            pos: 0,
            nesting: 0,
            stop_leftright: false,
            prev_kind: Kind::Const,
            def_depth: 0,
            decimal,
        }
    }

    fn rest(&self) -> &str {
        &self.input[self.pos..]
    }

    fn advance(&mut self, n: usize) {
        self.pos += n;
    }

    fn skip_ws(&mut self) {
        let n = self
            .rest()
            .chars()
            .take_while(|c| c.is_whitespace())
            .map(|c| c.len_utf8())
            .sum();
        self.advance(n);
    }

    fn lookup(&self) -> Option<(Sym, usize)> {
        let rest = self.rest();
        if rest.is_empty() {
            return None;
        }
        let direct = asciimath_symbols::match_prefix(rest);
        // `\cancel` must beat a shorter LaTeX command `\c`. Compare the direct
        // hit with the longest name after an optional backslash.
        let indirect = rest.strip_prefix('\\').and_then(|stripped| {
            if !stripped.chars().next().is_some_and(|c| c.is_ascii_alphabetic()) {
                return None;
            }
            let (sym, n) = asciimath_symbols::match_prefix(stripped)?;
            if sym.name.chars().count() > 1 {
                Some((sym, 1 + n))
            } else {
                None
            }
        });
        match (direct, indirect) {
            (Some((sym, n)), Some((sym2, n2))) => {
                if n2 > n {
                    Some((sym2, n2))
                } else {
                    Some((sym, n))
                }
            }
            (Some(hit), None) | (None, Some(hit)) => Some(hit),
            (None, None) => None,
        }
    }

    fn at_closer(&self) -> Option<(Sym, usize)> {
        if self.nesting <= 0 {
            return None;
        }
        let (sym, n) = self.lookup()?;
        if sym.kind == Kind::Right || (self.stop_leftright && sym.kind == Kind::LeftRight) {
            Some((sym, n))
        } else {
            None
        }
    }

    /// Digits with an optional decimal mark. A comma that is the decimal
    /// separator is not consumed when a space follows it (list separator).
    fn number_len(&self) -> Option<usize> {
        let bytes = self.rest().as_bytes();
        if bytes.first().is_none_or(|b| !b.is_ascii_digit()) {
            return None;
        }
        let mut i = 0usize;
        while i < bytes.len() && bytes[i].is_ascii_digit() {
            i += 1;
        }
        if i < bytes.len() && bytes[i] == self.decimal {
            let next = bytes.get(i + 1).copied();
            let list_space = self.decimal == b',' && next == Some(b' ');
            if !list_space && next.is_some_and(|c| c.is_ascii_digit()) {
                i += 1;
                while i < bytes.len() && bytes[i].is_ascii_digit() {
                    i += 1;
                }
            }
        }
        Some(i)
    }

    fn take_primes(&mut self) -> Option<String> {
        self.skip_ws();
        if !self.rest().starts_with('\'') || self.lookup().is_some() {
            return None;
        }
        let mut s = String::new();
        while self.rest().starts_with('\'') && self.lookup().is_none() {
            s.push('\'');
            self.advance(1);
        }
        if s.is_empty() { None } else { Some(s) }
    }

    fn expand_def(&mut self, sym: &Sym, n: usize) {
        let tail = self.input[self.pos + n..].to_string();
        let mut repl = sym.output.clone();
        repl.push_str(&tail);
        self.input.replace_range(self.pos.., &repl);
    }

    fn parse_expr(&mut self, stop_leftright: bool) -> (Vec<Expr>, Option<(Sym, usize)>) {
        let saved = self.stop_leftright;
        self.stop_leftright = stop_leftright;
        let mut items = Vec::new();
        let closer = loop {
            self.skip_ws();
            if self.rest().is_empty() {
                break None;
            }
            if let Some(closer) = self.at_closer() {
                break Some(closer);
            }
            let pos = self.pos;
            let Some(parsed) = self.parse_i() else {
                break None;
            };
            if self.pos == pos {
                // Never spin on a token we failed to consume.
                if let Some(ch) = self.rest().chars().next() {
                    self.advance(ch.len_utf8());
                    items.push(fallback_char(ch));
                }
                continue;
            }
            self.skip_ws();
            if let Some((sym, n)) = self.lookup()
                && sym.kind == Kind::Infix
                && sym.name == "/"
            {
                self.advance(n);
                self.prev_kind = Kind::Infix;
                let den = self
                    .parse_i()
                    .map(|p| unwrap_fence(p.expr))
                    .unwrap_or(Expr::Missing);
                items.push(Expr::Frac(
                    Box::new(unwrap_fence(parsed.expr)),
                    Box::new(den),
                ));
            } else {
                items.push(parsed.expr);
            }
        };
        self.stop_leftright = saved;
        (items, closer)
    }

    fn parse_i(&mut self) -> Option<Parsed> {
        self.skip_ws();
        if self.at_closer().is_some() || self.rest().is_empty() {
            return None;
        }
        let mut parsed = self.parse_s()?;
        self.skip_ws();
        if let Some((sym, n)) = self.lookup()
            && sym.kind == Kind::Infix
            && sym.name != "/"
        {
            self.advance(n);
            self.prev_kind = Kind::Infix;
            let script = self
                .parse_s()
                .map(|p| unwrap_fence(p.expr))
                .unwrap_or(Expr::Missing);
            let (sub, sup) = if sym.name == "_" {
                self.skip_ws();
                if let Some((sup_sym, n2)) = self.lookup()
                    && sup_sym.kind == Kind::Infix
                    && sup_sym.name == "^"
                {
                    self.advance(n2);
                    self.prev_kind = Kind::Infix;
                    let sup = self
                        .parse_s()
                        .map(|p| unwrap_fence(p.expr))
                        .unwrap_or(Expr::Missing);
                    (Some(script), Some(sup))
                } else {
                    (Some(script), None)
                }
            } else {
                (None, Some(script))
            };
            let base = parsed.expr;
            parsed.expr = script_expr(base, sub, sup, parsed.underover);
            self.skip_ws();
            if parsed.func &&
                self.func_wants_argument(parsed.func_chars) &&
                let Some(arg) = self.parse_i()
            {
                parsed.expr = Expr::row(vec![parsed.expr, arg.expr]);
            }
        }
        if let Some(primes) = self.take_primes() {
            parsed.expr = Expr::Sup(Box::new(parsed.expr), Box::new(Expr::Operator(primes)));
        }
        Some(parsed)
    }

    /// After a scripted function (`sin^2 x`), the argument follows unless the
    /// next token is another script, a closer, or (for `f`/`g`) a non-paren.
    fn func_wants_argument(&self, func_chars: usize) -> bool {
        if self.rest().is_empty() || self.at_closer().is_some() {
            return false;
        }
        if let Some((sym, _)) = self.lookup() {
            if sym.kind == Kind::Infix || sym.kind == Kind::Right {
                return false;
            }
            if func_chars == 1 && sym.kind != Kind::Left {
                return false;
            }
            return true;
        }
        // Plain letter or number. Single-letter `f`/`g` only take `(…)`.
        if func_chars == 1 {
            return false;
        }
        true
    }

    fn parse_s(&mut self) -> Option<Parsed> {
        self.skip_ws();
        if self.rest().is_empty() || self.at_closer().is_some() {
            return None;
        }
        if let Some(n) = self.number_len() {
            // A symbol that starts with a digit would already have matched;
            // numbers are what is left.
            if self.lookup().is_none() {
                let text = self.rest()[..n].to_string();
                self.advance(n);
                self.prev_kind = Kind::Const;
                return Some(leaf(Expr::Number(text)));
            }
        }
        let Some((sym, n)) = self.lookup() else {
            let ch = self.rest().chars().next()?;
            self.advance(ch.len_utf8());
            self.prev_kind = Kind::Const;
            return Some(leaf(fallback_char(ch)));
        };

        if sym.kind == Kind::Definition {
            if self.def_depth > 12 {
                self.advance(n);
                return Some(leaf(Expr::Identifier(sym.name.clone())));
            }
            self.expand_def(&sym, n);
            self.def_depth += 1;
            let parsed = self.parse_s();
            self.def_depth -= 1;
            return parsed;
        }

        // `|` immediately before a comma or closer is a bar, not an absolute value.
        if sym.kind == Kind::LeftRight && bar_is_separator(self.rest(), n) {
            self.advance(n);
            self.prev_kind = Kind::Const;
            return Some(leaf(Expr::Operator(sym.output.clone())));
        }

        match sym.kind {
            Kind::Const | Kind::UnderOver | Kind::Space => {
                // `/-b` keeps the minus with what follows an infix operator.
                let gobble_minus = sym.name == "-" && self.prev_kind == Kind::Infix;
                self.advance(n);
                if gobble_minus {
                    self.prev_kind = Kind::Const;
                    let minus = Expr::Operator(sym.output.clone());
                    self.skip_ws();
                    let expr = if self.rest().is_empty() || self.at_closer().is_some() {
                        minus
                    } else {
                        match self.parse_s() {
                            Some(arg) => Expr::row(vec![minus, arg.expr]),
                            None => minus,
                        }
                    };
                    return Some(leaf(expr));
                }
                self.prev_kind = sym.kind;
                let mut parsed = leaf(emit_const(&sym));
                parsed.underover = sym.kind == Kind::UnderOver;
                Some(parsed)
            }
            Kind::Infix if sym.name == "_" || sym.name == "^" => self.parse_prescript(&sym, n),
            Kind::Infix => {
                // Leading `/` (not a fraction) or a stray infix token.
                self.advance(n);
                self.prev_kind = Kind::Infix;
                Some(leaf(Expr::Operator(sym.output.clone())))
            }
            Kind::Left => self.parse_left(&sym, n),
            Kind::Right => {
                // Extra closer at the top level: keep it as an operator.
                self.advance(n);
                self.prev_kind = Kind::Right;
                Some(leaf(Expr::Operator(sym.output)))
            }
            Kind::LeftRight => self.parse_abs(&sym, n),
            Kind::Text => self.parse_text(&sym, n),
            Kind::Unary | Kind::UnaryUnderOver => self.parse_unary(&sym, n),
            Kind::Binary => self.parse_binary(&sym, n),
            Kind::Definition => None,
        }
    }

    fn parse_prescript(&mut self, sym: &Sym, n: usize) -> Option<Parsed> {
        self.advance(n);
        self.prev_kind = Kind::Infix;
        let first = self
            .parse_s()
            .map(|p| unwrap_fence(p.expr))
            .unwrap_or(Expr::Missing);
        let (sub, sup) = if sym.name == "_" {
            self.skip_ws();
            if let Some((s2, n2)) = self.lookup()
                && s2.kind == Kind::Infix
                && s2.name == "^"
            {
                self.advance(n2);
                let sup = self
                    .parse_s()
                    .map(|p| unwrap_fence(p.expr))
                    .unwrap_or(Expr::Missing);
                (Some(first), Some(sup))
            } else {
                (Some(first), None)
            }
        } else {
            (None, Some(first))
        };
        self.skip_ws();
        let base = if self.at_closer().is_some() || self.rest().is_empty() {
            Expr::Row(vec![])
        } else {
            self.parse_s().map(|p| p.expr).unwrap_or(Expr::Row(vec![]))
        };
        Some(leaf(Expr::Prescripts {
            base: Box::new(base),
            sub: sub.map(Box::new),
            sup: sup.map(Box::new),
        }))
    }

    fn parse_left(&mut self, sym: &Sym, n: usize) -> Option<Parsed> {
        self.advance(n);
        self.prev_kind = Kind::Left;
        self.nesting += 1;
        let (items, closer) = self.parse_expr(false);
        self.nesting -= 1;
        let close = if let Some((cl, cn)) = closer {
            self.advance(cn);
            self.prev_kind = Kind::Right;
            Some(cl)
        } else {
            None
        };
        let expr = if let Some(rows) = detect_matrix(&items) {
            Expr::Table {
                open: if sym.invisible {
                    None
                } else {
                    Some(sym.output.clone())
                },
                close: close.as_ref().and_then(|c| {
                    if sym.invisible {
                        None
                    } else {
                        Some(c.output.clone())
                    }
                }),
                rows,
                intent: None,
            }
        } else {
            let body = Expr::row(items);
            if sym.invisible {
                Expr::Group(Box::new(body))
            } else {
                Expr::Fenced {
                    open: sym.output.clone(),
                    close: close.map(|c| c.output),
                    body: Box::new(body),
                }
            }
        };
        Some(leaf(expr))
    }

    fn parse_abs(&mut self, sym: &Sym, n: usize) -> Option<Parsed> {
        self.advance(n);
        self.prev_kind = Kind::LeftRight;
        self.nesting += 1;
        let (items, closer) = self.parse_expr(true);
        self.nesting -= 1;
        let closed = if let Some((cl, cn)) = closer {
            if cl.kind == Kind::LeftRight {
                self.advance(cn);
                self.prev_kind = Kind::LeftRight;
                true
            } else {
                // The `)` / `]` belongs to an outer group.
                false
            }
        } else {
            false
        };
        let body = Expr::row(items);
        let expr = if closed {
            Expr::Fenced {
                open: sym.output.clone(),
                close: Some(sym.output.clone()),
                body: Box::new(body),
            }
        } else if body.is_empty_row() {
            // A bar with nothing after it is a mid (∣), not an empty abs.
            Expr::Operator("∣".into())
        } else {
            Expr::Fenced {
                open: sym.output.clone(),
                close: None,
                body: Box::new(body),
            }
        };
        Some(leaf(expr))
    }

    fn parse_text(&mut self, sym: &Sym, n: usize) -> Option<Parsed> {
        if sym.name == "\"" {
            let rest = self.rest();
            let body = &rest[1.min(rest.len())..];
            let (content, consumed) = if let Some(i) = body.find('"') {
                (body[..i].to_string(), 1 + i + 1)
            } else {
                // Unclosed quote: take the rest.
                (body.to_string(), rest.len())
            };
            self.advance(consumed);
            self.prev_kind = Kind::Text;
            return Some(leaf(text_with_edge_spaces(&content)));
        }
        self.advance(n);
        self.prev_kind = Kind::Text;
        self.skip_ws();
        let rest = self.rest();
        let Some(open) = rest.chars().next() else {
            return Some(leaf(Expr::Missing));
        };
        let close = match open {
            '{' => '}',
            '(' => ')',
            '[' => ']',
            _ => {
                return Some(leaf(Expr::Missing));
            }
        };
        let body = &rest[open.len_utf8()..];
        let (content, consumed) = if let Some(i) = body.find(close) {
            (body[..i].to_string(), open.len_utf8() + i + close.len_utf8())
        } else {
            (body.to_string(), rest.len())
        };
        self.advance(consumed);
        Some(leaf(text_with_edge_spaces(&content)))
    }

    fn parse_unary(&mut self, sym: &Sym, n: usize) -> Option<Parsed> {
        self.advance(n);
        self.prev_kind = Kind::Unary;
        let func_chars = sym.name.chars().count();

        if sym.func {
            self.skip_ws();
            let next = self.rest().chars().next();
            let bare = next.is_none()
                || self.at_closer().is_some()
                || matches!(next, Some('^' | '_' | '/' | '|' | ','))
                || (func_chars == 1 && next != Some('('));
            if bare {
                return Some(Parsed {
                    expr: emit_const(sym),
                    underover: false,
                    func: true,
                    func_chars,
                });
            }
            let expr = match self.parse_s() {
                Some(arg) => Expr::row(vec![emit_const(sym), arg.expr]),
                None => emit_const(sym),
            };
            return Some(Parsed {
                expr,
                underover: false,
                func: true,
                func_chars,
            });
        }

        if sym.name == "cancel" {
            let arg = self
                .parse_s()
                .map(|p| unwrap_fence(p.expr))
                .unwrap_or(Expr::Missing);
            return Some(leaf(Expr::Enclose {
                notation: "updiagonalstrike".into(),
                body: Box::new(arg),
            }));
        }
        if let Some((open, close)) = sym.fences {
            let arg = self
                .parse_s()
                .map(|p| unwrap_fence(p.expr))
                .unwrap_or(Expr::Missing);
            return Some(leaf(Expr::Fenced {
                open: open.into(),
                close: Some(close.into()),
                body: Box::new(arg),
            }));
        }
        if let Some(variant) = sym.variant {
            let arg = self.parse_s().map(|p| p.expr).unwrap_or(Expr::Missing);
            return Some(leaf(apply_variant(arg, variant)));
        }
        if sym.name == "sqrt" {
            let arg = self
                .parse_s()
                .map(|p| unwrap_fence(p.expr))
                .unwrap_or(Expr::Missing);
            return Some(leaf(Expr::Sqrt(Box::new(arg))));
        }
        if sym.accent {
            let arg = self
                .parse_s()
                .map(|p| unwrap_fence(p.expr))
                .unwrap_or(Expr::Missing);
            let accent = Expr::Operator(sym.output.clone());
            let under = matches!(sym.name.as_str(), "ul" | "ubrace");
            let expr = if under {
                Expr::Under(Box::new(arg), Box::new(accent))
            } else {
                Expr::Over(Box::new(arg), Box::new(accent))
            };
            return Some(Parsed {
                expr,
                underover: sym.kind == Kind::UnaryUnderOver,
                func: false,
                func_chars: 0,
            });
        }

        // Unknown unary: keep the name and, if present, the following atom.
        let expr = Expr::Identifier(sym.name.clone());
        self.skip_ws();
        if self.rest().is_empty() || self.at_closer().is_some() {
            return Some(leaf(expr));
        }
        match self.parse_s() {
            Some(arg) => Some(leaf(Expr::row(vec![expr, arg.expr]))),
            None => Some(leaf(expr)),
        }
    }

    fn parse_binary(&mut self, sym: &Sym, n: usize) -> Option<Parsed> {
        self.advance(n);
        self.prev_kind = Kind::Binary;
        if matches!(sym.name.as_str(), "color" | "class" | "id") {
            let _label = self.parse_s();
            let body = self.parse_s().map(|p| p.expr).unwrap_or(Expr::Missing);
            return Some(leaf(body));
        }
        let Some(first) = self.parse_s() else {
            return Some(leaf(Expr::Identifier(sym.name.clone())));
        };
        let Some(second) = self.parse_s() else {
            return Some(leaf(Expr::row(vec![
                Expr::Identifier(sym.name.clone()),
                first.expr,
            ])));
        };
        let a = unwrap_fence(first.expr);
        let b = unwrap_fence(second.expr);
        let expr = match sym.name.as_str() {
            "root" => Expr::Root(Box::new(a), Box::new(b)),
            "frac" => Expr::Frac(Box::new(a), Box::new(b)),
            "underset" => Expr::Under(Box::new(b), Box::new(a)),
            _ => Expr::Over(Box::new(b), Box::new(a)),
        };
        Some(leaf(expr))
    }
}

fn leaf(expr: Expr) -> Parsed {
    Parsed {
        expr,
        underover: false,
        func: false,
        func_chars: 0,
    }
}

fn emit_const(sym: &Sym) -> Expr {
    if sym.kind == Kind::Space {
        return Expr::Text(sym.output.clone());
    }
    if sym.ident {
        Expr::Identifier(sym.output.clone())
    } else {
        Expr::Operator(sym.output.clone())
    }
}

fn fallback_char(ch: char) -> Expr {
    if ch.is_ascii_alphabetic() {
        Expr::Identifier(ch.to_string())
    } else if ch.is_ascii_digit() {
        Expr::Number(ch.to_string())
    } else {
        Expr::Operator(ch.to_string())
    }
}

fn script_expr(base: Expr, sub: Option<Expr>, sup: Option<Expr>, underover: bool) -> Expr {
    let base = Box::new(base);
    if underover {
        return match (sub, sup) {
            (Some(s), Some(p)) => Expr::UnderOver(base, Box::new(s), Box::new(p)),
            (Some(s), None) => Expr::Under(base, Box::new(s)),
            (None, Some(p)) => Expr::Over(base, Box::new(p)),
            (None, None) => *base,
        };
    }
    match (sub, sup) {
        (Some(s), Some(p)) => Expr::SubSup(base, Box::new(s), Box::new(p)),
        (Some(s), None) => Expr::Sub(base, Box::new(s)),
        (None, Some(p)) => Expr::Sup(base, Box::new(p)),
        (None, None) => *base,
    }
}

fn unwrap_fence(expr: Expr) -> Expr {
    match expr {
        Expr::Fenced {
            open,
            close: Some(close),
            body,
        } if matches!(open.as_str(), "(" | "[" | "{")
            && matches!(close.as_str(), ")" | "]" | "}")
            && !matches!(body.as_ref(), Expr::Table { .. }) =>
        {
            *body
        }
        other => other,
    }
}

fn bar_is_separator(rest: &str, bar_len: usize) -> bool {
    let after = rest[bar_len..].trim_start();
    after.starts_with([',', ']', ')', '}'])
}

fn text_with_edge_spaces(content: &str) -> Expr {
    let mut parts = Vec::new();
    let core = content.trim();
    if content.starts_with(char::is_whitespace) {
        parts.push(Expr::Operator("\u{00A0}".into()));
    }
    parts.push(Expr::Text(core.to_string()));
    if content.ends_with(char::is_whitespace) {
        parts.push(Expr::Operator("\u{00A0}".into()));
    }
    Expr::row(parts)
}

fn is_comma(expr: &Expr) -> bool {
    matches!(expr, Expr::Operator(s) if s == ",")
}

fn split_top_commas(expr: &Expr) -> Vec<Expr> {
    let parts: Vec<Expr> = match expr {
        Expr::Row(v) => v.clone(),
        other => vec![other.clone()],
    };
    if parts.is_empty() {
        return vec![Expr::Missing];
    }
    let mut cells = Vec::new();
    let mut cur = Vec::new();
    for part in parts {
        if is_comma(&part) {
            cells.push(if cur.is_empty() {
                Expr::Missing
            } else {
                Expr::row(std::mem::take(&mut cur))
            });
        } else if matches!(&part, Expr::Operator(s) if s == "|" || s == "∣") && cur.is_empty() {
            // Augmented-matrix column marker. Drop it; the column break is the comma.
            continue;
        } else {
            cur.push(part);
        }
    }
    cells.push(if cur.is_empty() {
        Expr::Missing
    } else {
        Expr::row(cur)
    });
    cells
}

/// `[[a,b],[c,d]]` / `((a),(b))` → table rows. A single parenthesized group is not a matrix.
fn detect_matrix(items: &[Expr]) -> Option<Vec<Vec<Expr>>> {
    if items.is_empty() {
        return None;
    }
    let mut groups = Vec::new();
    let mut cur = Vec::new();
    for item in items {
        if is_comma(item) {
            if cur.len() != 1 {
                return None;
            }
            groups.push(cur.pop().unwrap());
        } else {
            cur.push(item.clone());
        }
    }
    if cur.len() != 1 {
        return None;
    }
    groups.push(cur.pop().unwrap());

    let mut rows = Vec::new();
    let mut open_ch: Option<&str> = None;
    for group in &groups {
        let Expr::Fenced {
            open,
            close: Some(close),
            body,
        } = group
        else {
            return None;
        };
        let ok = matches!((open.as_str(), close.as_str()), ("(", ")") | ("[", "]"));
        if !ok {
            return None;
        }
        if let Some(prev) = open_ch {
            if prev != open.as_str() {
                return None;
            }
        } else {
            open_ch = Some(open.as_str());
        }
        let cells = split_top_commas(body);
        if cells.len() == 1 && (matches!(cells[0], Expr::Table { .. }) || groups.len() == 1) {
            return None;
        }
        rows.push(cells);
    }
    if rows.len() >= 2 {
        let width = rows[0].len();
        if rows.iter().any(|r| r.len() != width) {
            return None;
        }
    }
    Some(rows)
}

fn decimal_sep() -> u8 {
    let pm = PreferenceManager::get();
    let pm = pm.borrow();
    let s = pm.pref_to_string("DecimalSeparators");
    if s.contains(',') && !s.contains('.') {
        b','
    } else {
        b'.'
    }
}

fn apply_variant(expr: Expr, variant: &str) -> Expr {
    let (letters, digits) = match variant {
        "bold" => (0x1D400, 0x1D7CE),
        "bold-italic" => (0x1D468, 0x1D7CE),
        "double-struck" => (0x1D538, 0x1D7D8),
        "script" => (0x1D49C, 0),
        "bold-script" => (0x1D4D0, 0),
        "fraktur" => (0x1D504, 0),
        "bold-fraktur" => (0x1D56C, 0),
        "sans-serif" => (0x1D5A0, 0x1D7E2),
        "bold-sans-serif" => (0x1D5D4, 0x1D7EC),
        "sans-serif-italic" => (0x1D608, 0),
        "sans-serif-bold-italic" => (0x1D63C, 0x1D7EC),
        "monospace" => (0x1D670, 0x1D7F6),
        _ => (0, 0),
    };
    map_expr(expr, letters, digits)
}

fn map_expr(expr: Expr, letters: u32, digits: u32) -> Expr {
    match expr {
        Expr::Identifier(s) => Expr::Identifier(map_text(&s, letters, digits)),
        Expr::Number(s) => Expr::Number(map_text(&s, letters, digits)),
        Expr::Operator(s) => Expr::Operator(map_text(&s, letters, digits)),
        Expr::Text(s) => Expr::Text(map_text(&s, letters, digits)),
        Expr::Row(v) => Expr::row(v.into_iter().map(|e| map_expr(e, letters, digits)).collect()),
        Expr::Group(e) => Expr::Group(Box::new(map_expr(*e, letters, digits))),
        Expr::Sup(a, b) => Expr::Sup(
            Box::new(map_expr(*a, letters, digits)),
            Box::new(map_expr(*b, letters, digits)),
        ),
        Expr::Sub(a, b) => Expr::Sub(
            Box::new(map_expr(*a, letters, digits)),
            Box::new(map_expr(*b, letters, digits)),
        ),
        Expr::SubSup(a, b, c) => Expr::SubSup(
            Box::new(map_expr(*a, letters, digits)),
            Box::new(map_expr(*b, letters, digits)),
            Box::new(map_expr(*c, letters, digits)),
        ),
        Expr::Frac(a, b) => Expr::Frac(
            Box::new(map_expr(*a, letters, digits)),
            Box::new(map_expr(*b, letters, digits)),
        ),
        Expr::BinomFrac(a, b) => Expr::BinomFrac(
            Box::new(map_expr(*a, letters, digits)),
            Box::new(map_expr(*b, letters, digits)),
        ),
        Expr::Sqrt(a) => Expr::Sqrt(Box::new(map_expr(*a, letters, digits))),
        Expr::Root(a, b) => Expr::Root(
            Box::new(map_expr(*a, letters, digits)),
            Box::new(map_expr(*b, letters, digits)),
        ),
        Expr::Fenced { open, close, body } => Expr::Fenced {
            open: map_text(&open, letters, digits),
            close: close.map(|c| map_text(&c, letters, digits)),
            body: Box::new(map_expr(*body, letters, digits)),
        },
        Expr::Over(a, b) => Expr::Over(
            Box::new(map_expr(*a, letters, digits)),
            Box::new(map_expr(*b, letters, digits)),
        ),
        Expr::Under(a, b) => Expr::Under(
            Box::new(map_expr(*a, letters, digits)),
            Box::new(map_expr(*b, letters, digits)),
        ),
        Expr::UnderOver(a, b, c) => Expr::UnderOver(
            Box::new(map_expr(*a, letters, digits)),
            Box::new(map_expr(*b, letters, digits)),
            Box::new(map_expr(*c, letters, digits)),
        ),
        Expr::Enclose { notation, body } => Expr::Enclose {
            notation,
            body: Box::new(map_expr(*body, letters, digits)),
        },
        Expr::Prescripts { base, sub, sup } => Expr::Prescripts {
            base: Box::new(map_expr(*base, letters, digits)),
            sub: sub.map(|e| Box::new(map_expr(*e, letters, digits))),
            sup: sup.map(|e| Box::new(map_expr(*e, letters, digits))),
        },
        Expr::MultiScripts { base, sub, sup } => Expr::MultiScripts {
            base: Box::new(map_expr(*base, letters, digits)),
            sub: sub.map(|e| Box::new(map_expr(*e, letters, digits))),
            sup: sup.map(|e| Box::new(map_expr(*e, letters, digits))),
        },
        Expr::Table { open, close, rows, intent } => Expr::Table {
            open,
            close,
            intent,
            rows: rows
                .into_iter()
                .map(|r| r.into_iter().map(|c| map_expr(c, letters, digits)).collect())
                .collect(),
        },
        Expr::StyledNumber { text, mathvariant } => Expr::StyledNumber {
            text: map_text(&text, letters, digits),
            mathvariant,
        },
        other => other,
    }
}

fn map_text(s: &str, letters: u32, digits: u32) -> String {
    if letters == 0 && digits == 0 {
        return s.to_string();
    }
    s.chars()
        .map(|c| {
            let cp = if c.is_ascii_uppercase() && letters != 0 {
                letters + (c as u32 - 'A' as u32)
            } else if c.is_ascii_lowercase() && letters != 0 {
                letters + 26 + (c as u32 - 'a' as u32)
            } else if c.is_ascii_digit() && digits != 0 {
                digits + (c as u32 - '0' as u32)
            } else {
                return c;
            };
            hole_char(cp)
        })
        .collect()
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
    char::from_u32(mapped).unwrap_or('?')
}

pub(super) fn braille_to_expr(input: &str) -> Result<Expr> {
    asciimath_symbols::ensure_asciimath_symbols_loaded()?;
    let mut parser = Parser::new(input);
    let (items, closer) = parser.parse_expr(false);
    let mut items = items;
    if let Some((sym, n)) = closer {
        parser.advance(n);
        items.push(Expr::Operator(sym.output));
    }
    // Anything still unparsed (should be rare) is kept, one token at a time.
    while !parser.rest().is_empty() {
        parser.skip_ws();
        if parser.rest().is_empty() {
            break;
        }
        let pos = parser.pos;
        if let Some(parsed) = parser.parse_i() {
            items.push(parsed.expr);
        }
        if parser.pos == pos &&
            let Some(ch) = parser.rest().chars().next()
        {
            parser.advance(ch.len_utf8());
            items.push(fallback_char(ch));
        }
    }
    Ok(Expr::row(items))
}

#[cfg(test)]
mod tests {
    use crate::parser::Braille_to_MathML;

    fn parse(src: &str) -> String {
        Braille_to_MathML(src, "ASCIIMath").unwrap_or_else(|e| panic!("parse failed for {src:?}: {e}"))
    }

    #[test]
    fn longest_match_succeq_not_succ() {
        let mml = parse("succeq");
        assert!(
            mml.contains("&#x2AB0;"),
            "succeq should be one relation, got {mml}"
        );
        assert!(
            !mml.contains("&#x227B;"),
            "succeq must not split into succ, got {mml}"
        );
    }

    #[test]
    fn latex_partial_and_longer_command() {
        let partial = parse(r"\partial");
        assert!(partial.contains("&#x2202;"), "{partial}");
        let longer = parse(r"\partialmeetcontraction");
        assert!(
            longer.contains("&#x2AA3;"),
            "longest LaTeX name, got {longer}"
        );
        assert!(!longer.contains("&#x2202;"), "{longer}");
        let then_letter = parse(r"\partial W");
        assert!(then_letter.contains("&#x2202;"), "{then_letter}");
        assert!(then_letter.contains("<mi>W</mi>"), "{then_letter}");
    }

    #[test]
    fn unclosed_and_missing_do_not_fail() {
        let mml = parse("(x+1");
        assert!(mml.contains("<mo>(</mo>"), "{mml}");
        assert!(mml.contains("<mi>x</mi>"), "{mml}");
        let sqrt = parse("sqrt");
        assert!(sqrt.contains("&#xFFFD;") || sqrt.contains("<msqrt>"), "{sqrt}");
        let junk = parse("((((@");
        assert!(junk.contains("@") || junk.contains("&#x2218;"), "{junk}");
    }
}
