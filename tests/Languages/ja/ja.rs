use crate::common::*;
use anyhow::Result;

/// Verifies that basic arithmetic operators use the seeded Japanese names.
#[test]
fn arithmetic_operators() -> Result<()> {
    let expr = "<math><mn>1</mn><mo>+</mo><mn>2</mn><mo>&#x2212;</mo><mn>3</mn><mo>=</mo><mn>0</mn></math>";
    test("ja", "ClearSpeak", expr, "1 プラス 2 マイナス 3; イコール 0")?;
    return Ok(());
}

/// A fraction of two numbers is read denominator-first in Japanese:
/// 21/22 is "22 分の 21", literally "of 22, 21". Reading it the other way round
/// says 22/21.
#[test]
fn simple_fraction() -> Result<()> {
    let expr = "<math><mfrac><mn>21</mn><mn>22</mn></mfrac></math>";
    test("ja", "ClearSpeak", expr, "22 分の 21")?;
    test("ja", "SimpleSpeak", expr, "22 分の 21")?;
    return Ok(());
}

/// The denominator-first pattern is not limited to the small numbers that English
/// has ordinals for ("three fourths"); it is how any two numbers are read.
#[test]
fn numeric_fraction_large_denominator() -> Result<()> {
    let expr = "<math><mfrac><mn>3</mn><mn>128</mn></mfrac></math>";
    test("ja", "ClearSpeak", expr, "128 分の 3")?;
    return Ok(());
}

/// When the parts are not plain numbers, Japanese keeps the written order and
/// borrows the English preposition as "オーバー" instead (Yamaguchi et al. 1996).
#[test]
fn fraction_of_variables() -> Result<()> {
    let expr = "<math><mfrac><mi>x</mi><mi>y</mi></mfrac></math>";
    test("ja", "ClearSpeak", expr, "x オーバー y")?;
    test("ja", "SimpleSpeak", expr, "x オーバー y")?;
    return Ok(());
}

/// Verifies that a square root uses the seeded Japanese root wording.
#[test]
fn square_root() -> Result<()> {
    let expr = "<math><msqrt><mn>9</mn></msqrt></math>";
    test("ja", "ClearSpeak", expr, "平方根 の 9")?;
    return Ok(());
}

/// Verifies that a squared value uses the seeded Japanese exponent wording.
#[test]
fn squared() -> Result<()> {
    let expr = "<math><msup><mn>3</mn><mn>2</mn></msup></math>";
    test("ja", "ClearSpeak", expr, "3 スクエア")?;
    return Ok(());
}

/// Verifies both Japanese gradient readings selected by verbosity.
#[test]
fn gradient() -> Result<()> {
    let expr = "<math><mo>&#x2207;</mo><mi mathvariant='normal'>F</mi></math>";
    test_prefs("ja", "SimpleSpeak", vec![("Verbosity", "Terse")], expr, "デル 大文字 f")?;
    test_prefs("ja", "SimpleSpeak", vec![("Verbosity", "Verbose")], expr, "勾配 の 大文字 f")?;
    return Ok(());
}

/// Verifies that multiplication and division symbols use Japanese operator names.
#[test]
fn multiplication_and_division() -> Result<()> {
    let expr = "<math><mn>6</mn><mo>&#x00D7;</mo><mn>4</mn><mo>&#x00F7;</mo><mn>2</mn></math>";
    test("ja", "ClearSpeak", expr, "6 掛ける 4 割る 2")?;
    return Ok(());
}

/// Verifies that explicit parentheses retain Japanese opening and closing cues.
#[test]
fn parenthesized_expression() -> Result<()> {
    let expr = "<math><mrow><mo>(</mo><mn>1</mn><mo>+</mo><mn>2</mn><mo>)</mo></mrow></math>";
    test("ja", "ClearSpeak", expr, "開き丸括弧, 1 プラス 2, 閉じ丸括弧")?;
    return Ok(());
}

/// Verifies the Japanese ClearSpeak wording for an absolute value.
#[test]
fn absolute_value() -> Result<()> {
    let expr = "<math><mrow><mo>|</mo><mi>x</mi><mo>|</mo></mrow></math>";
    test("ja", "ClearSpeak", expr, "絶対値 の x")?;
    return Ok(());
}

/// Verifies that an indexed cube root receives the Japanese cube-root cue.
#[test]
fn cube_root() -> Result<()> {
    let expr = "<math><mroot><mn>8</mn><mn>3</mn></mroot></math>";
    test("ja", "ClearSpeak", expr, "立方根 の 8")?;
    return Ok(());
}

/// Verifies the basic Japanese SimpleSpeak subscript pattern.
#[test]
fn subscript() -> Result<()> {
    let expr = "<math><msub><mi>x</mi><mn>1</mn></msub></math>";
    test("ja", "SimpleSpeak", expr, "x サブ 1")?;
    return Ok(());
}

/// Verifies that a common trigonometric function is spoken in Japanese SimpleSpeak.
#[test]
fn sine_function() -> Result<()> {
    let expr = "<math><mi>sin</mi><mo>&#x2061;</mo><mi>x</mi></math>";
    test("ja", "SimpleSpeak", expr, "サイン の x")?;
    return Ok(());
}

/// Verifies Japanese speech for the less-than comparison operator.
#[test]
fn less_than() -> Result<()> {
    let expr = "<math><mi>x</mi><mo>&lt;</mo><mn>5</mn></math>";
    test("ja", "ClearSpeak", expr, "x は 小なり 5")?;
    return Ok(());
}

/// Verifies that common lowercase Greek letters use their Japanese names.
#[test]
fn greek_letters() -> Result<()> {
    let expr = "<math><mi>&#x03B1;</mi><mo>+</mo><mi>&#x03B2;</mi></math>";
    test("ja", "ClearSpeak", expr, "アルファ プラス ベータ")?;
    return Ok(());
}

/// Verifies Japanese SimpleSpeak wording for set membership.
#[test]
fn set_membership() -> Result<()> {
    let expr = "<math><mi>x</mi><mo>&#x2208;</mo><mi mathvariant='double-struck'>R</mi></math>";
    test("ja", "SimpleSpeak", expr, "x は 属する 実数")?;
    return Ok(());
}

/// Verifies the seeded Japanese cues for a summation with limits.
#[test]
fn summation() -> Result<()> {
    let expr = "<math><munderover><mo>&#x2211;</mo><mrow><mi>i</mi><mo>=</mo><mn>1</mn></mrow><mi>n</mi></munderover><mi>i</mi></math>";
    test("ja", "SimpleSpeak", expr, "総和 から i イコール 1, に n の i")?;
    return Ok(());
}

/// Verifies the seeded Japanese cues for a definite integral.
#[test]
fn definite_integral() -> Result<()> {
    let expr = "<math><msubsup><mo>&#x222B;</mo><mn>0</mn><mn>1</mn></msubsup><mi>x</mi><mo>&#x2146;</mo><mi>x</mi></math>";
    test("ja", "SimpleSpeak", expr, "積分 から 0, に 1 の; x 微分 d x")?;
    return Ok(());
}
