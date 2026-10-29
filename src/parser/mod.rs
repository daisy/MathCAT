//! Braille technical material and ASCIIMath → MathML parser.
//!
//! UEB, Nemeth, ASCIIMath, and LaTeX share the [`Expr`] AST. Each notation is its own
//! lexer and grammar: the languages do not share productions. What they share
//! is the symbol-table loader in [`symbols`] and the registry below.
//!
//! Adding a notation:
//! 1. Add `parser/<code>.rs` with `braille_to_expr`, turning input into [`Expr`].
//!    Incomplete input should stay in the tree (a missing piece, an unclosed
//!    fence) instead of failing the parse.
//! 2. Add a symbol table that reads `Rules/Braille/<Code>/unicode.yaml` through
//!    [`symbols::collect_templates`]. Flag letters and collision policy belong
//!    to that code.
//! 3. Append one [`Notation`] here. [`clear_symbol_caches`] follows the list.

#![allow(clippy::needless_return)]
#![allow(non_snake_case)]

use crate::errors::{bail, Result};

mod asciimath;
mod expr;
mod latex;
mod nemeth;
pub(crate) mod symbols;
mod ueb;

pub use expr::Expr;

struct Notation {
    code: &'static str,
    parse: fn(&str) -> Result<Expr>,
    clear_symbols: fn(),
}

const NOTATIONS: &[Notation] = &[
    Notation {
        code: "UEB",
        parse: ueb::braille_to_expr,
        clear_symbols: crate::ueb_symbols::clear_ueb_symbol_cache,
    },
    Notation {
        code: "Nemeth",
        parse: nemeth::braille_to_expr,
        clear_symbols: crate::nemeth_symbols::clear_nemeth_symbol_cache,
    },
    Notation {
        code: "ASCIIMath",
        parse: asciimath::braille_to_expr,
        clear_symbols: crate::asciimath_symbols::clear_asciimath_symbol_cache,
    },
    Notation {
        code: "LaTeX",
        parse: latex::braille_to_expr,
        clear_symbols: crate::latex_symbols::clear_latex_symbol_cache,
    },
];

/// Parse Unicode braille or ASCIIMath into a MathML fragment wrapped in `<math>`.
///
/// `braille_code` selects the notation (`"UEB"`, `"Nemeth"`, `"ASCIIMath"`, or `"LaTeX"`).
pub fn Braille_to_MathML(braille: &str, braille_code: &str) -> Result<String> {
    let expr = Braille_to_Expr(braille, braille_code)?;
    Ok(format!("<math>{}</math>", expr.to_mathml()))
}

/// Parse Unicode braille or ASCIIMath into an expression AST (useful for tests / tooling).
///
/// `braille_code` selects the notation (`"UEB"`, `"Nemeth"`, `"ASCIIMath"`, or `"LaTeX"`).
pub fn Braille_to_Expr(braille: &str, braille_code: &str) -> Result<Expr> {
    match NOTATIONS.iter().find(|n| n.code == braille_code) {
        Some(n) => (n.parse)(braille),
        None => bail!("Braille_to_Expr: unsupported braille code '{braille_code}'"),
    }
}

/// Drop every notation's cached symbol table (rules directory or code changed).
pub(crate) fn clear_symbol_caches() {
    for n in NOTATIONS {
        (n.clear_symbols)();
    }
}
