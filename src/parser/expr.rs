//! Shared presentation AST for braille → MathML (UEB and Nemeth).

#![allow(clippy::needless_return)]
#![allow(non_snake_case)]

// --- AST --------------------------------------------------------------------

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Expr {
    Number(String),
    /// `<mn mathvariant="…">` when no Unicode math-alphanumeric form exists (e.g. script digits).
    StyledNumber {
        text: String,
        mathvariant: String,
    },
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
    pub(crate) fn row(mut parts: Vec<Expr>) -> Expr {
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

    pub(crate) fn is_empty_row(&self) -> bool {
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
            Expr::StyledNumber { text, mathvariant } => format!(
                "<mn mathvariant=\"{}\">{}</mn>",
                xml_escape(mathvariant),
                xml_escape(text)
            ),
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

