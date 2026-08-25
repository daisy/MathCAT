//! Braille technical material → MathML parser.
//!
//! UEB (ICEB *Guidelines for Technical Material*) and Nemeth (BANA 2022) share
//! the [`Expr`] AST. Each code has its own lexer and recursive-descent parser.
//! Symbols for reverse lookup are loaded lazily from that code’s
//! `unicode.yaml` / `unicode-full.yaml`.

#![allow(clippy::needless_return)]
#![allow(non_snake_case)]

use crate::errors::{bail, Result};

mod expr;
mod nemeth;
mod ueb;

pub use expr::Expr;

/// Parse a Unicode braille string into a MathML fragment wrapped in `<math>`.
///
/// `braille_code` selects the braille system (`"UEB"` or `"Nemeth"`).
pub fn Braille_to_MathML(braille: &str, braille_code: &str) -> Result<String> {
    let expr = Braille_to_Expr(braille, braille_code)?;
    Ok(format!("<math>{}</math>", expr.to_mathml()))
}

/// Parse Unicode braille into an expression AST (useful for tests / tooling).
///
/// `braille_code` selects the braille system (`"UEB"` or `"Nemeth"`).
pub fn Braille_to_Expr(braille: &str, braille_code: &str) -> Result<Expr> {
    match braille_code {
        "UEB" => ueb::braille_to_expr(braille),
        "Nemeth" => nemeth::braille_to_expr(braille),
        other => bail!("Braille_to_Expr: unsupported braille code '{other}'"),
    }
}
