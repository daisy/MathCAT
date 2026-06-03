use crate::common::*;
use anyhow::Result;

#[test]
fn numbers_and_operators() -> Result<()> {
    let expr = r#"<math><mrow><mn>5</mn><mo>+</mo><mn>12</mn><mo>=</mo><mn>17</mn></mrow></math>"#;
    test_braille("Russian", expr, "⠼⠑⠀⠖⠼⠁⠃⠀⠶⠼⠁⠛")?;
    return Ok(());
}

#[test]
fn fraction() -> Result<()> {
    let expr = r#"<math><mfrac><mi>x</mi><mn>2</mn></mfrac></math>"#;
    test_braille("Russian", expr, "⠆⠠⠭⠀⠳⠼⠃⠰")?;
    return Ok(());
}

#[test]
fn scripts_and_root() -> Result<()> {
    let expr = r#"<math><mrow><msup><mi>x</mi><mn>2</mn></msup><mo>+</mo><msqrt><mi>y</mi></msqrt></mrow></math>"#;
    test_braille("Russian", expr, "⠠⠭⠌⠼⠃⠱⠀⠖⠩⠱⠠⠽⠹")?;
    return Ok(());
}

#[test]
fn cyrillic_text() -> Result<()> {
    let expr = r#"<math><mtext>угол</mtext></math>"#;
    test_braille("Russian", expr, "⠥⠛⠕⠇")?;
    return Ok(());
}

#[test]
fn wikipedia_linear_parens_flat() -> Result<()> {
    let expr = r#"<math><mrow><mn>3</mn><mo>&#x22C5;</mo><mrow><mo>(</mo><mn>9</mn><mo>-</mo><mn>7</mn><mo>)</mo></mrow><mo>=</mo><mn>6</mn></mrow></math>"#;
    test_braille("Russian", expr, "⠼⠉⠄⠣⠼⠊⠀⠤⠼⠛⠜⠀⠶⠼⠋")?;
    return Ok(());
}

#[test]
fn source_arithmetic_examples() -> Result<()> {
    let expr = r#"<math><mrow><mn>24</mn><mo>&#x22C5;</mo><mn>81</mn><mo>=</mo><mn>1944</mn></mrow></math>"#;
    test_braille("Russian", expr, "⠼⠃⠙⠄⠼⠓⠁⠀⠶⠼⠁⠊⠙⠙")?;

    let expr = r#"<math><mrow><mn>783</mn><mo>:</mo><mn>9</mn><mo>=</mo><mn>87</mn></mrow></math>"#;
    test_braille("Russian", expr, "⠼⠛⠓⠉⠀⠲⠼⠊⠀⠶⠼⠓⠛")?;

    let expr = r#"<math><mrow><mn>12</mn><mo>&#xD7;</mo><mn>35</mn><mo>=</mo><mn>420</mn></mrow></math>"#;
    test_braille("Russian", expr, "⠼⠁⠃⠀⠦⠼⠉⠑⠀⠶⠼⠙⠃⠚")?;
    return Ok(());
}

#[test]
fn nested_fraction_and_root() -> Result<()> {
    let expr = r#"<math><mfrac><mrow><mi>x</mi><mo>+</mo><msqrt><mfrac><mn>1</mn><mi>y</mi></mfrac></msqrt></mrow><mrow><mi>x</mi><mo>-</mo><mi>y</mi></mrow></mfrac></math>"#;
    test_braille("Russian", expr, "⠆⠠⠭⠀⠖⠩⠱⠆⠼⠁⠀⠳⠠⠽⠰⠹⠀⠳⠭⠀⠤⠽⠰")?;
    return Ok(());
}

#[test]
fn latin_alphabet_indicators() -> Result<()> {
    let expr = r#"<math><mrow><mi>x</mi><mo>+</mo><mi>A</mi><mo>+</mo><mi>y</mi><mo>+</mo><mi>B</mi><mo>=</mo><mi>x</mi><mo>+</mo><mi>y</mi><mo>+</mo><mi>A</mi><mo>+</mo><mi>B</mi></mrow></math>"#;
    test_braille("Russian", expr, "⠠⠭⠀⠖⠨⠁⠀⠖⠠⠽⠀⠖⠨⠃⠀⠶⠠⠭⠀⠖⠽⠀⠖⠨⠁⠀⠖⠃")?;
    return Ok(());
}

#[test]
fn alphabet_indicators_after_numbers_and_greek() -> Result<()> {
    let expr = r#"<math><mrow><mn>2</mn><mo>&#x22C5;</mo><mi>x</mi><mo>+</mo><mn>15</mn><mo>=</mo><mn>23</mn></mrow></math>"#;
    test_braille("Russian", expr, "⠼⠃⠄⠠⠭⠀⠖⠼⠁⠑⠀⠶⠼⠃⠉")?;

    let expr = r#"<math><mrow><mi>L</mi><mo>=</mo><mn>2</mn><mi>&#x3C0;</mi><mi>r</mi></mrow></math>"#;
    test_braille("Russian", expr, "⠨⠇⠀⠶⠼⠃⠰⠏⠠⠗")?;

    let expr = r#"<math><mrow><mi>&#x3B1;</mi><mo>+</mo><mi>&#x3B2;</mi><mo>=</mo><mi>&#x391;</mi><mo>+</mo><mi>&#x392;</mi></mrow></math>"#;
    test_braille("Russian", expr, "⠰⠁⠀⠖⠃⠀⠶⠸⠁⠀⠖⠃")?;
    return Ok(());
}

#[test]
fn wikipedia_times_divide() -> Result<()> {
    let expr = r#"<math><mn>6</mn><mo>&#xD7;</mo><mn>7</mn><mo>:</mo><mn>14</mn><mo>=</mo><mn>3</mn></math>"#;
    test_braille("Russian", expr, "⠼⠋⠀⠦⠼⠛⠀⠲⠼⠁⠙⠀⠶⠼⠉")?;
    return Ok(());
}

#[test]
fn wikipedia_linear_parens() -> Result<()> {
    let expr = r#"<math><mn>3</mn><mo>&#xB7;</mo><mo>(</mo><mn>9</mn><mo>&#x2212;</mo><mn>7</mn><mo>)</mo><mo>=</mo><mn>6</mn></math>"#;
    test_braille("Russian", expr, "⠼⠉⠄⠣⠼⠊⠀⠤⠼⠛⠜⠀⠶⠼⠋")?;
    return Ok(());
}

#[test]
fn wikipedia_sqrt() -> Result<()> {
    let expr = r#"<math><msqrt><mn>10000</mn></msqrt><mo>&lt;</mo><mn>101</mn></math>"#;
    test_braille("Russian", expr, "⠩⠱⠼⠁⠚⠚⠚⠚⠹⠀⠪⠀⠼⠁⠚⠁")?;
    return Ok(());
}
