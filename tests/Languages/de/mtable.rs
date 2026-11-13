use crate::common::*;
use anyhow::Result;

#[test]
fn matrix_1x1() -> Result<()> {
    let expr = "
    <math xmlns='http://www.w3.org/1998/Math/MathML'>
      <mrow>
      <mrow><mo>(</mo>
        <mtable><mtr><mtd>
        <mn>3</mn>
      </mtd> </mtr></mtable>
        <mo>)</mo></mrow></mrow>
    </math>
                                ";
    test("de", "ClearSpeak",  expr, "die 1 mal 1 matrix mit eintrag 3")?;
    test("de", "SimpleSpeak", expr, "die 1 mal 1 matrix mit eintrag 3")?;
    return Ok(());

}

#[test]
fn determinant_1x1() -> Result<()> {
    let expr = "
    <math xmlns='http://www.w3.org/1998/Math/MathML'>
      <mrow>
      <mrow><mo>|</mo>
        <mtable><mtr><mtd>
        <mn>3</mn>
      </mtd> </mtr></mtable>
        <mo>|</mo></mrow></mrow>
    </math>
                                ";
    test("de", "ClearSpeak",  expr, "die 1 mal 1 determinante mit eintrag 3")?;
    test("de", "SimpleSpeak", expr, "die 1 mal 1 determinante mit eintrag 3")?;
    return Ok(());

}

#[test]
fn single_line_with_label() -> Result<()> {
    let expr = r#"<math>
  <mtable class="gather" displaystyle="true" intent=":system-of-equations">
    <mtr>
      <mtd intent=":equation-label"><mtext>(2)</mtext></mtd>
      <mtd><mi>𝑏</mi><mo>=</mo><mn>2</mn></mtd>
    </mtr>
  </mtable>
  </math>"#;
    test_prefs("de", "ClearSpeak", vec![("Verbosity", "Terse")],
        expr, "1 zeile, mit Beschriftung 2; b ist gleich 2")?;
    Ok(())
}

// Added after a rule review found the German 1xN row-matrix rule repeated `die 1 mal` instead of saying `matrix`/`vektor`.
#[test]
fn matrix_1x2_row_regression() -> Result<()> {
    let expr = "
    <math xmlns='http://www.w3.org/1998/Math/MathML'>
      <mrow>
      <mrow><mo>(</mo>
        <mtable>
          <mtr>
          <mtd>
            <mn>3</mn>
          </mtd>
          <mtd>
            <mn>5</mn>
          </mtd>
          </mtr>
        </mtable>
      <mo>)</mo></mrow></mrow>
    </math>
                                ";
    test("de", "ClearSpeak",  expr, "die 1 mal 2 Zeile matrix; 3, 5")?;
    test("de", "SimpleSpeak", expr, "die 1 mal 2 Zeile matrix; 3, 5")?;
    Ok(())
}

#[test]
fn identity_matrix() -> Result<()> {
    let expr = "<math>
      <mo>[</mo>
      <mtable>
        <mtr><mtd><mn>1</mn></mtd><mtd><mn>0</mn></mtd><mtd><mn>0</mn></mtd></mtr>
        <mtr><mtd><mn>0</mn></mtd><mtd><mn>1</mn></mtd><mtd><mn>0</mn></mtd></mtr>
        <mtr><mtd><mn>0</mn></mtd><mtd><mn>0</mn></mtd><mtd><mn>1</mn></mtd></mtr>
      </mtable>
      <mo>]</mo>
  </math>";
    test("de", "SimpleSpeak", expr, "die 3 mal 3 identitätsmatrix")?;
    Ok(())
}

#[test]
fn identity_matrix_false_positive_negative_one() -> Result<()> {
    let expr = "<math>
      <mo>[</mo>
      <mtable>
        <mtr><mtd><mn>1</mn></mtd><mtd><mn>0</mn></mtd></mtr>
        <mtr><mtd><mn>0</mn></mtd><mtd><mn>-1</mn></mtd></mtr>
      </mtable>
      <mo>]</mo>
  </math>";
    test_prefs("de", "SimpleSpeak", vec![("Verbosity", "Terse")],
        expr, "die 2 mal 2 diagonalmatrix; spalte 1; 1; spalte 2; minus 1")?;
    Ok(())
}

#[test]
fn identity_matrix_false_positive_zero_diagonal() -> Result<()> {
    let expr = "<math>
      <mo>[</mo>
      <mtable>
        <mtr><mtd><mn>1</mn></mtd><mtd><mn>0</mn></mtd></mtr>
        <mtr><mtd><mn>0</mn></mtd><mtd><mn>0</mn></mtd></mtr>
      </mtable>
      <mo>]</mo>
  </math>";
    test_prefs("de", "SimpleSpeak", vec![("Verbosity", "Terse")],
        expr, "die 2 mal 2 diagonalmatrix; spalte 1; 1")?;
    Ok(())
}
