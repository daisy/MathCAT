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

/// An exponent is read "<base> の <exponent> 乗"; 乗 closes it. Japanese has no
/// ordinal form here, so 2 is 2 and not "second".
#[test]
fn squared() -> Result<()> {
    let expr = "<math><msup><mn>3</mn><mn>2</mn></msup></math>";
    test("ja", "ClearSpeak", expr, "3 の 2 乗")?;
    test("ja", "SimpleSpeak", expr, "3 の 2 乗")?;
    return Ok(());
}

/// Same shape for a cube.
#[test]
fn cubed() -> Result<()> {
    let expr = "<math><msup><mn>5</mn><mn>3</mn></msup></math>";
    test("ja", "ClearSpeak", expr, "5 の 3 乗")?;
    test("ja", "SimpleSpeak", expr, "5 の 3 乗")?;
    return Ok(());
}

/// The pattern does not change for exponents above three.
#[test]
fn integer_exponent() -> Result<()> {
    let expr = "<math><msup><mi>x</mi><mn>5</mn></msup></math>";
    test("ja", "ClearSpeak", expr, "x の 5 乗")?;
    test("ja", "SimpleSpeak", expr, "x の 5 乗")?;
    return Ok(());
}

/// A variable exponent is read the same way (English adds "-th" here; Japanese does not).
#[test]
fn variable_exponent() -> Result<()> {
    let expr = "<math><msup><mi>x</mi><mi>n</mi></msup></math>";
    test("ja", "ClearSpeak", expr, "x の n 乗")?;
    test("ja", "SimpleSpeak", expr, "x の n 乗")?;
    return Ok(());
}

/// A complex exponent is read as a superscript instead, with an explicit close:
/// 「の上付き … 上付き終了」. Reading it as 乗 would end a nested exponent 「… 乗 乗」.
#[test]
fn complex_exponent() -> Result<()> {
    let expr = "<math><msup><mi>x</mi><mrow><mi>y</mi><mo>+</mo><mn>1</mn></mrow></msup></math>";
    test("ja", "SimpleSpeak", expr, "x の上付き y プラス 1 上付き終了")?;
    return Ok(());
}

/// A leading minus is read マイナス, the same word as the binary operator.
/// 負の / 正の name the *kind* of number (負の数 = "the negative numbers") and are
/// not how −5 is read aloud.
#[test]
fn negative_number() -> Result<()> {
    let expr = "<math><mo>&#x2212;</mo><mn>5</mn></math>";
    test("ja", "ClearSpeak", expr, "マイナス 5")?;
    test("ja", "SimpleSpeak", expr, "マイナス 5")?;
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
    test("ja", "ClearSpeak", expr, "丸括弧 1 プラス 2, 丸括弧閉じ")?;
    return Ok(());
}

/// Square and curly brackets follow the same pattern: the opening bracket names
/// the shape and the closing one adds the postposed cue (Yamaguchi et al. 1996).
#[test]
fn square_and_curly_brackets() -> Result<()> {
    let square = "<math><mrow><mo>[</mo><mn>1</mn><mo>+</mo><mn>2</mn><mo>]</mo></mrow></math>";
    test("ja", "ClearSpeak", square, "角括弧 1 プラス 2, 角括弧閉じ")?;
    let curly = "<math><mrow><mo>{</mo><mn>1</mn><mo>+</mo><mn>2</mn><mo>}</mo></mrow></math>";
    test("ja", "ClearSpeak", curly, "中括弧 1 プラス 2, 中括弧閉じ")?;
    return Ok(());
}

/// Verifies the Japanese ClearSpeak wording for an absolute value.
#[test]
fn absolute_value() -> Result<()> {
    let expr = "<math><mrow><mo>|</mo><mi>x</mi><mo>|</mo></mrow></math>";
    test("ja", "ClearSpeak", expr, "絶対値 x")?;
    return Ok(());
}

/// With AbsEnd the closing cue is spoken after the contents, not before it, so
/// the bar that ends the group is heard where it actually is.
#[test]
fn absolute_value_abs_end() -> Result<()> {
    let expr = "<math><mrow><mo>|</mo><mi>x</mi><mo>|</mo></mrow></math>";
    test_ClearSpeak("ja", "ClearSpeak_AbsoluteValue", "AbsEnd", expr, "絶対値 x, 絶対値 閉じ")?;
    return Ok(());
}

/// Verifies that an indexed cube root receives the Japanese cube-root cue.
#[test]
fn cube_root() -> Result<()> {
    let expr = "<math><mroot><mn>8</mn><mn>3</mn></mroot></math>";
    test("ja", "ClearSpeak", expr, "立方根 の 8")?;
    return Ok(());
}

/// An n-th root is 「n 乗根」 -- the index, then 乗根. English builds an ordinal
/// ("the fifth root"); Japanese has no such form and reads the number as it is.
#[test]
fn nth_root() -> Result<()> {
    let expr = "<math><mroot><mi>x</mi><mn>5</mn></mroot></math>";
    test("ja", "ClearSpeak", expr, "5 乗根 の x")?;
    test("ja", "SimpleSpeak", expr, "5 乗根 の x")?;
    return Ok(());
}

/// The same holds when the index is a variable (English appends "-th" here).
#[test]
fn variable_index_root() -> Result<()> {
    let expr = "<math><mroot><mi>x</mi><mi>n</mi></mroot></math>";
    test("ja", "ClearSpeak", expr, "n 乗根 の x")?;
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

/// Geometry has settled Japanese terms: 線分, 半直線, 弧, 点. The katakana
/// transliterations of the English words are not used for these.
#[test]
fn geometry_terms() -> Result<()> {
    for (intent, expected) in [
        ("line-segment", "線分 x y"),
        ("directed-line-segment", "有向線分 x y"),
        ("line", "直線 x y"),
        ("ray", "半直線 x y"),
        ("arc", "弧 x y"),
    ] {
        let expr = format!(
            "<math><mrow intent='{intent}($x,$y)'><mi arg='x'>x</mi><mi arg='y'>y</mi></mrow></math>"
        );
        test("ja", "ClearSpeak", &expr, expected)?;
    }
    return Ok(());
}

/// A point is 点, not ポイント.
#[test]
fn geometry_point() -> Result<()> {
    let expr = "<math><mrow intent='point($x,$y,$z)'><mi arg='x'>x</mi><mi arg='y'>y</mi><mi arg='z'>z</mi></mrow></math>";
    test("ja", "ClearSpeak", expr, "点 x y z")?;
    return Ok(());
}

/// Verbose keeps the head term first, as the reference asks, but a ray and a
/// segment differ in what the second point is: a segment stops at it, a ray only
/// passes through it. 「まで」 would claim the ray ends at B.
#[test]
fn geometry_verbose_from_to() -> Result<()> {
    let seg = "<math><mrow intent='line-segment($x,$y)'><mi arg='x'>x</mi><mi arg='y'>y</mi></mrow></math>";
    test_prefs("ja", "ClearSpeak", vec![("Verbosity", "Verbose")], seg, "線分 x から y まで")?;
    let ray = "<math><mrow intent='ray($x,$y)'><mi arg='x'>x</mi><mi arg='y'>y</mi></mrow></math>";
    test_prefs("ja", "ClearSpeak", vec![("Verbosity", "Verbose")], ray, "半直線 x を始点として y を通る")?;
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
    test("ja", "SimpleSpeak", expr, "総和 i イコール 1 から, n まで オブ i")?;
    return Ok(());
}

/// A large operator with only a lower limit takes the postposed "over" cue.
#[test]
fn summation_lower_limit_only() -> Result<()> {
    let expr = "<math><munder><mo>&#x2211;</mo><mrow><mi>i</mi><mo>=</mo><mn>1</mn></mrow></munder><mi>i</mi></math>";
    test("ja", "SimpleSpeak", expr, "総和 i イコール 1 にわたる オブ i")?;
    return Ok(());
}

/// The same shape is used for the other large operators.
#[test]
fn product_with_limits() -> Result<()> {
    let expr = "<math><munderover><mo>&#x220F;</mo><mrow><mi>i</mi><mo>=</mo><mn>1</mn></mrow><mi>n</mi></munderover><mi>i</mi></math>";
    test("ja", "SimpleSpeak", expr, "プロダクト i イコール 1 から, n まで オブ i")?;
    return Ok(());
}

/// Verifies the seeded Japanese cues for a definite integral.
#[test]
fn definite_integral() -> Result<()> {
    let expr = "<math><msubsup><mo>&#x222B;</mo><mn>0</mn><mn>1</mn></msubsup><mi>x</mi><mo>&#x2146;</mo><mi>x</mi></math>";
    test("ja", "SimpleSpeak", expr, "積分 0 から, 1 まで オブ; x 微分 d x")?;
    return Ok(());
}
