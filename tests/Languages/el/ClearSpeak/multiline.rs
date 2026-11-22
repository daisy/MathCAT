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
        <mtr><mtd><mo>-</mo><mn>1</mn></mtd><mtd><mtext>if</mtext></mtd><mtd><mi>x</mi><mo>&lt;</mo><mn>0</mn></mtd></mtr>
        <mtr><mtd><mn>0</mn></mtd><mtd><mtext>if</mtext></mtd><mtd><mi>x</mi><mo>=</mo><mn>0</mn></mtd></mtr>
        <mtr><mtd><mn>1</mn></mtd><mtd><mtext>if</mtext></mtd><mtd><mi>x</mi><mo>&gt;</mo><mn>0</mn></mtd></mtr>
      </mtable>
    </mrow>
  </math>
   ";
  test_ClearSpeak("el", "ClearSpeak_MultiLineLabel", "Auto", expr,
    "f του x, ισούται με; 3 περιπτώσεις; \
                περίπτωση 1; μείον 1 if, x είναι μικρότερο από 0; \
                περίπτωση 2; 0 if x ισούται με 0; \
                περίπτωση 3; 1 if, x είναι μεγαλύτερο από 0"
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
   test_ClearSpeak("el", "ClearSpeak_MultiLineLabel", "Auto", expr,
                "2 σειρές; \
                σειρά 1; x συν y, ισούται με 7; \
                σειρά 2; 2 x συν 3 y; ισούται με 17")?;
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
   test_ClearSpeak("el", "ClearSpeak_MultiLineLabel", "Auto", expr, "2 σειρές; \
                σειρά 1; x συν y, ισούται με 7; \
                σειρά 2; 2 x, συν 3 y, ισούται με 17")?;
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
   test_ClearSpeak("el", "ClearSpeak_MultiLineLabel", "Case", expr, 
   "2 περιπτώσεις; περίπτωση 1; x συν y, ισούται με 7; περίπτωση 2; 2 x συν 3 y; ισούται με 17")?;
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
   test_ClearSpeak("el", "ClearSpeak_MultiLineLabel", "Constraint", expr, "2 περιορισμοί; \
                περιορισμός 1; x συν y, ισούται με 7; \
                περιορισμός 2; 2 x συν 3 y; ισούται με 17")?;
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
   test_ClearSpeak("el", "ClearSpeak_MultiLineLabel", "Equation", expr, "2 εξισώσεις; \
                εξίσωση 1; x συν y, ισούται με 7; \
                εξίσωση 2; 2 x συν 3 y; ισούται με 17")?;
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
   test_ClearSpeak("el", "ClearSpeak_MultiLineLabel", "Line", expr, "2 σειρές; \
                σειρά 1; x συν y, ισούται με 7; \
                σειρά 2; 2 x συν 3 y; ισούται με 17")?;
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
   test_ClearSpeak("el", "ClearSpeak_MultiLineLabel", "None", expr,
        "2 σειρές; \
                x συν y, ισούται με 7; \
                2 x συν 3 y; ισούται με 17")?;
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
   test_ClearSpeak("el", "ClearSpeak_MultiLineLabel", "Row", expr, "2 γραμμές; \
                γραμμή 1; x συν y, ισούται με 7; \
                γραμμή 2; 2 x συν 3 y; ισούται με 17")?;
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
   test_ClearSpeak("el", "ClearSpeak_MultiLineLabel", "Step", expr, "2 βήματα; \
                βήμα 1; x συν y, ισούται με 7; \
                βήμα 2; 2 x συν 3 y; ισούται με 17")?;
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
test("el", "SimpleSpeak", expr,
     "2 εξισώσεις; εξίσωση 1; x ισούται με y άδειο άδειο συν 1; εξίσωση 2; y ισούται με 1")?;
    return Ok(());
}
