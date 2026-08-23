use crate::common::*;
use anyhow::Result;


#[test]
fn case_1() -> Result<()> {
  let expr = "<math>
    <mi>f</mi>
    <mrow>
      <mo>(</mo>
      <mi>x</mi>
      <mo>)</mo>
    </mrow>
    <mo>=</mo>
    <mrow>
      <mo stretchy='true'>{</mo>
      <mtable>
        <mtr><mtd><mo>-</mo><mn>1</mn></mtd><mtd><mtext>si</mtext></mtd><mtd><mi>x</mi><mo>&lt;</mo><mn>0</mn></mtd></mtr>
        <mtr><mtd><mn>0</mn></mtd><mtd><mtext>si</mtext></mtd><mtd><mi>x</mi><mo>=</mo><mn>0</mn></mtd></mtr>
        <mtr><mtd><mn>1</mn></mtd><mtd><mtext>si</mtext></mtd><mtd><mi>x</mi><mo>&gt;</mo><mn>0</mn></mtd></mtr>
      </mtable>
    </mrow>
  </math>
   ";
  test_ClearSpeak("fr", "ClearSpeak_MultiLineLabel", "Auto", expr,
    "f de x est égal à; 3 cas; \
                cas 1; moins 1 si x est inférieur à 0; \
                cas 2; 0 si x est égal à 0; \
                cas 3; 1 si x est supérieur à 0"
    )?;
    return Ok(());
}

#[test]
fn equation_auto() -> Result<()> {
    let expr = "<math>
     <mrow>
      <mtable>
       <mtr> <mtd> <mrow> <mi>x</mi><mo>+</mo><mi>y</mi></mrow> </mtd>  <mtd><mo>=</mo> </mtd>  <mtd><mn>7</mn></mtd>  </mtr>
       <mtr> <mtd> <mrow> <mn>2</mn><mi>x</mi><mo>+</mo><mn>3</mn><mi>y</mi></mrow></mtd>  <mtd><mo>=</mo></mtd>  <mtd><mrow><mn>17</mn></mrow></mtd> </mtr>
      </mtable></mrow>
    </math>
   ";
   test_ClearSpeak("fr", "ClearSpeak_MultiLineLabel", "Auto", expr,
                "2 lignes; \
                ligne 1; x plus y, est égal à 7; \
                ligne 2; 2 x plus 3 y; est égal à 17")?;
    return Ok(());
}


#[test]
fn equation_plus_at_start() -> Result<()> {
  let expr = "<math>
     <mrow>
      <mtable>
       <mtr> <mtd> <mi>x</mi></mtd><mtd><mo>+</mo><mi>y</mi> </mtd>  <mtd><mo>=</mo> </mtd>  <mtd><mn>7</mn></mtd>  </mtr>
       <mtr> <mtd> <mn>2</mn><mi>x</mi></mtd><mtd><mo>+</mo><mn>3</mn><mi>y</mi></mtd>  <mtd><mo>=</mo></mtd>  <mtd><mrow><mn>17</mn></mrow></mtd> </mtr>
      </mtable></mrow>
    </math>
   ";
   test_ClearSpeak("fr", "ClearSpeak_MultiLineLabel", "Auto", expr, "2 lignes; \
                ligne 1; x plus y est égal à 7; \
                ligne 2; 2 x, plus 3 y, est égal à 17")?;
    return Ok(());
}

#[test]
fn equation_case() -> Result<()> {
    let expr = "<math>
     <mrow>
      <mtable>
       <mtr> <mtd> <mrow> <mi>x</mi><mo>+</mo><mi>y</mi></mrow> </mtd>  <mtd><mo>=</mo> </mtd>  <mtd><mn>7</mn></mtd>  </mtr>
       <mtr> <mtd> <mrow> <mn>2</mn><mi>x</mi><mo>+</mo><mn>3</mn><mi>y</mi></mrow></mtd>  <mtd><mo>=</mo></mtd>  <mtd><mrow><mn>17</mn></mrow></mtd> </mtr>
      </mtable></mrow>
    </math>
   ";
   test_ClearSpeak("fr", "ClearSpeak_MultiLineLabel", "Case", expr, 
   "2 cas; cas 1; x plus y, est égal à 7; cas 2; 2 x plus 3 y; est égal à 17")?;
    return Ok(());
}

#[test]
fn equation_constraint() -> Result<()> {
  let expr = "<math>
     <mrow>
      <mtable>
       <mtr> <mtd> <mrow> <mi>x</mi><mo>+</mo><mi>y</mi></mrow> </mtd>  <mtd><mo>=</mo> </mtd>  <mtd><mn>7</mn></mtd>  </mtr>
       <mtr> <mtd> <mrow> <mn>2</mn><mi>x</mi><mo>+</mo><mn>3</mn><mi>y</mi></mrow></mtd>  <mtd><mo>=</mo></mtd>  <mtd><mrow><mn>17</mn></mrow></mtd> </mtr>
      </mtable></mrow>
    </math>
   ";
   test_ClearSpeak("fr", "ClearSpeak_MultiLineLabel", "Constraint", expr, "2 contraintes; \
                contrainte 1; x plus y, est égal à 7; \
                contrainte 2; 2 x plus 3 y; est égal à 17")?;
   return Ok(());
}

#[test]
fn equation_equation() -> Result<()> {
    let expr = "<math>
     <mrow>
      <mtable>
       <mtr> <mtd> <mrow> <mi>x</mi><mo>+</mo><mi>y</mi></mrow> </mtd>  <mtd><mo>=</mo> </mtd>  <mtd><mn>7</mn></mtd>  </mtr>
       <mtr> <mtd> <mrow> <mn>2</mn><mi>x</mi><mo>+</mo><mn>3</mn><mi>y</mi></mrow></mtd>  <mtd><mo>=</mo></mtd>  <mtd><mrow><mn>17</mn></mrow></mtd> </mtr>
      </mtable></mrow>
    </math>
   ";
   test_ClearSpeak("fr", "ClearSpeak_MultiLineLabel", "Equation", expr, "2 équations; \
                équation 1; x plus y, est égal à 7; \
                équation 2; 2 x plus 3 y; est égal à 17")?;
   return Ok(());
}

#[test]
fn equation_line() -> Result<()> {
    let expr = "<math>
     <mrow>
      <mtable>
       <mtr> <mtd> <mrow> <mi>x</mi><mo>+</mo><mi>y</mi></mrow> </mtd>  <mtd><mo>=</mo> </mtd>  <mtd><mn>7</mn></mtd>  </mtr>
       <mtr> <mtd> <mrow> <mn>2</mn><mi>x</mi><mo>+</mo><mn>3</mn><mi>y</mi></mrow></mtd>  <mtd><mo>=</mo></mtd>  <mtd><mrow><mn>17</mn></mrow></mtd> </mtr>
      </mtable></mrow>
    </math>
   ";
   test_ClearSpeak("fr", "ClearSpeak_MultiLineLabel", "Line", expr, "2 lignes; \
                ligne 1; x plus y, est égal à 7; \
                ligne 2; 2 x plus 3 y; est égal à 17")?;
    return Ok(());
}

#[test]
fn equation_none() -> Result<()> {
    let expr = "<math>
     <mrow>
      <mtable>
       <mtr> <mtd> <mrow> <mi>x</mi><mo>+</mo><mi>y</mi></mrow> </mtd>  <mtd><mo>=</mo> </mtd>  <mtd><mn>7</mn></mtd>  </mtr>
       <mtr> <mtd> <mrow> <mn>2</mn><mi>x</mi><mo>+</mo><mn>3</mn><mi>y</mi></mrow></mtd>  <mtd><mo>=</mo></mtd>  <mtd><mrow><mn>17</mn></mrow></mtd> </mtr>
      </mtable></mrow>
    </math>
   ";
   test_ClearSpeak("fr", "ClearSpeak_MultiLineLabel", "None", expr,
        "2 lignes; \
                x plus y, est égal à 7; \
                2 x plus 3 y; est égal à 17")?;
   return Ok(());
}

#[test]
fn equation_row() -> Result<()> {
    let expr = "<math>
     <mrow>
      <mtable>
       <mtr> <mtd> <mrow> <mi>x</mi><mo>+</mo><mi>y</mi></mrow> </mtd>  <mtd><mo>=</mo> </mtd>  <mtd><mn>7</mn></mtd>  </mtr>
       <mtr> <mtd> <mrow> <mn>2</mn><mi>x</mi><mo>+</mo><mn>3</mn><mi>y</mi></mrow></mtd>  <mtd><mo>=</mo></mtd>  <mtd><mrow><mn>17</mn></mrow></mtd> </mtr>
      </mtable></mrow>
    </math>
   ";
   test_ClearSpeak("fr", "ClearSpeak_MultiLineLabel", "Row", expr, "2 rangées; \
                rangée 1; x plus y, est égal à 7; \
                rangée 2; 2 x plus 3 y; est égal à 17")?;
   return Ok(());
}

#[test]
fn equation_step() -> Result<()> {
    let expr = "<math>
     <mrow>
      <mtable>
       <mtr> <mtd> <mrow> <mi>x</mi><mo>+</mo><mi>y</mi></mrow> </mtd>  <mtd><mo>=</mo> </mtd>  <mtd><mn>7</mn></mtd>  </mtr>
       <mtr> <mtd> <mrow> <mn>2</mn><mi>x</mi><mo>+</mo><mn>3</mn><mi>y</mi></mrow></mtd>  <mtd><mo>=</mo></mtd>  <mtd><mrow><mn>17</mn></mrow></mtd> </mtr>
      </mtable></mrow>
    </math>
   ";
   test_ClearSpeak("fr", "ClearSpeak_MultiLineLabel", "Step", expr, "2 étapes; \
                étape 1; x plus y, est égal à 7; \
                étape 2; 2 x plus 3 y; est égal à 17")?;
   return Ok(());
}

#[test]
fn continued_row() -> Result<()> {
  let expr = "<math>
  <mtable intent=':system-of-equations'>
   <mtr><mtd><mi>x</mi></mtd><mtd><mo>=</mo></mtd><mtd><mi>y</mi></mtd></mtr>
   <mtr intent=':continued-row'><mtd/><mtd/><mtd><mrow><mo>+</mo><mn>1</mn></mrow></mtd></mtr>
   <mtr><mtd><mi>y</mi></mtd><mtd><mo>=</mo></mtd><mtd><mn>1</mn></mtd></mtr>
  </mtable>
</math>";
test("fr", "SimpleSpeak", expr,
     "2 équations; équation 1; x est égal à y plus 1; équation 2; y est égal à 1")?;
    return Ok(());
}
