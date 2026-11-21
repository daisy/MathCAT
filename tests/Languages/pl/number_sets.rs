/// Number sets: ℂ ℕ ℚ ℝ ℤ, on their own and with a dimension (ℝ³).
///
/// These rules had no Polish test at all, which is how the earlier defects in
/// this file were able to sit unnoticed: a rule with no test in a given
/// language is not "known good", it is "never looked at".
use crate::common::*;
use anyhow::Result;

/// A bare number set is spoken as a full Polish phrase.
#[test]
fn simple_number_sets() -> Result<()> {
    let expr = r#"<math>
        <mi>&#x2102;</mi><mo>,</mo>
        <mi>&#x2115;</mi><mo>,</mo>
        <mi>&#x211A;</mi><mo>,</mo>
        <mi>&#x211D;</mi><mo>,</mo>
        <mi>&#x2124;</mi>
    </math>"#;
    test("pl", "SimpleSpeak", expr, "liczby zespolone przecinek, liczby naturalne przecinek, liczby wymierne przecinek, liczby rzeczywiste, przecinek, liczby całkowite")?;
    Ok(())
}

/// With a dimension the letter itself is spoken, as in every language except
/// Hungarian: "R 3", not "liczby rzeczywiste 3".
#[test]
fn dimension_number_sets() -> Result<()> {
    let expr = r#"<math>
        <msup><mi>&#x211D;</mi><mn>3</mn></msup><mo>,</mo>
        <msup><mi>&#x2102;</mi><mn>2</mn></msup><mo>,</mo>
        <msup><mi>&#x2115;</mi><mn>4</mn></msup>
    </math>"#;
    test("pl", "SimpleSpeak", expr, "R 3 przecinek, C 2 przecinek, N 4")?;
    Ok(())
}

/// A non-numeric exponent is not a dimension, so the full phrase comes back.
/// This is the negative control for the rule above: it fires on `count(*)=2`
/// with a digit, and must not swallow every superscript.
#[test]
fn number_set_with_variable_exponent() -> Result<()> {
    let expr = r#"<math>
        <msup><mi>&#x2124;</mi><mi>n</mi></msup>
    </math>"#;
    test("pl", "SimpleSpeak", expr, "liczby całkowite do potęgi n")?;
    Ok(())
}

/// Positive and negative variants (ℝ⁺, ℤ⁻) are a separate rule again.
#[test]
fn pos_neg_number_sets() -> Result<()> {
    let expr = r#"<math>
        <msup><mi>&#x211D;</mi><mo>+</mo></msup><mo>,</mo>
        <msup><mi>&#x2124;</mi><mo>-</mo></msup>
    </math>"#;
    test("pl", "SimpleSpeak", expr, "plus liczby rzeczywiste, przecinek; minus liczby całkowite")?;
    Ok(())
}

/// ClearSpeak must agree with SimpleSpeak here - the rules live in
/// SharedRules, so a divergence would mean one of them stopped matching.
#[test]
fn number_sets_clearspeak_matches() -> Result<()> {
    let expr = r#"<math>
        <mi>&#x211D;</mi><mo>,</mo>
        <msup><mi>&#x211D;</mi><mn>3</mn></msup>
    </math>"#;
    test("pl", "ClearSpeak", expr, "liczby rzeczywiste, przecinek, R 3")?;
    Ok(())
}
