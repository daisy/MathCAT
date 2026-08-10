//! FromBraille: UEB linearized matrices (GTM 15 / UEB_Rules mtable).
#![allow(non_snake_case)]
use crate::common::*;
use anyhow::Result;

#[test]
fn matrix_2x2_parens() -> Result<()> {
    let expr = r#"<math><mrow><mo>(</mo><mtable><mtr><mtd><mn>1</mn></mtd><mtd><mn>0</mn></mtd></mtr><mtr><mtd><mn>0</mn></mtd><mtd><mn>1</mn></mtd></mtr></mtable><mo>)</mo></mrow></math>"#;
    test_from_braille("UEB", expr, "⠠⠐⠣⠼⠁⠀⠼⠚⠠⠐⠜⠠⠐⠣⠼⠚⠀⠼⠁⠠⠐⠜")?;
    Ok(())
}

#[test]
fn matrix_2x2_brackets() -> Result<()> {
    let expr = r#"<math><mrow><mo>[</mo><mtable><mtr><mtd><mi>a</mi></mtd><mtd><mi>b</mi></mtd></mtr><mtr><mtd><mi>c</mi></mtd><mtd><mi>d</mi></mtd></mtr></mtable><mo>]</mo></mrow></math>"#;
    test_from_braille("UEB", expr, "⠠⠨⠣⠁⠀⠃⠠⠨⠜⠠⠨⠣⠉⠀⠙⠠⠨⠜")?;
    Ok(())
}

#[test]
fn determinant_2x2() -> Result<()> {
    let expr = r#"<math><mrow><mo>|</mo><mtable><mtr><mtd><mn>1</mn></mtd><mtd><mn>2</mn></mtd></mtr><mtr><mtd><mn>3</mn></mtd><mtd><mn>4</mn></mtd></mtr></mtable><mo>|</mo></mrow></math>"#;
    test_from_braille("UEB", expr, "⠠⠸⠳⠼⠁⠀⠼⠃⠠⠸⠳⠠⠸⠳⠼⠉⠀⠼⠙⠠⠸⠳")?;
    Ok(())
}

#[test]
fn matrix_1x3_parens() -> Result<()> {
    let expr = r#"<math><mrow><mo>(</mo><mtable><mtr><mtd><mn>1</mn></mtd><mtd><mn>2</mn></mtd><mtd><mn>3</mn></mtd></mtr></mtable><mo>)</mo></mrow></math>"#;
    test_from_braille("UEB", expr, "⠐⠣⠼⠁⠀⠼⠃⠀⠼⠉⠐⠜")?;
    Ok(())
}
