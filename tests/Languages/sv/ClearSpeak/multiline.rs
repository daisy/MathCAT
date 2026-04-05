use crate::common::*;
use anyhow::Result;

#[test]
fn case_1() -> Result<()> {
    // init_logger();
    let expr = "<math>
            <mrow>
            <mi>f</mi><mrow><mo>(</mo>
            <mi>x</mi>
            <mo>)</mo></mrow><mo>=</mo><mrow><mo>{</mo> <mrow>
            <mtable>
            <mtr>
                <mtd>
                <mrow>
                <mo>&#x2212;</mo><mn>1</mn><mtext>&#x00A0;if&#x00A0;</mtext><mi>x</mi><mo>&#x003C;</mo><mn>0</mn></mrow>
                </mtd>
            </mtr>
            <mtr>
                <mtd>
                <mrow>
                <mn>0</mn><mtext>&#x00A0;if&#x00A0;</mtext><mi>x</mi><mo>=</mo><mn>0</mn></mrow>
                </mtd>
            </mtr>
            <mtr>
                <mtd>
                <mrow>
                <mn>1</mn><mtext>&#x00A0;if&#x00A0;</mtext><mi>x</mi><mo>&#x003E;</mo><mn>0</mn></mrow>
                </mtd>
            </mtr>
            </mtable></mrow> </mrow></mrow>
        </math>
   ";
   test_ClearSpeak("sv", "ClearSpeak_MultiLineLabel", "Auto", expr,
                   "f av x lika med; 3 fall; fall 1; minus 1 if x, är mindre än 0; fall 2; 0 if x, lika med 0; fall 3; 1 if x, är större än 0")?;
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
   test_ClearSpeak("sv", "ClearSpeak_MultiLineLabel", "Auto", expr,
                   "2 rader; rad 1; x plus y lika med 7; rad 2; 2 x plus 3 y; lika med 17")?;
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
   test_ClearSpeak("sv", "ClearSpeak_MultiLineLabel", "Case", expr,
                   "2 fall; fall 1; x plus y lika med 7; fall 2; 2 x plus 3 y; lika med 17")?;
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
   test_ClearSpeak("sv", "ClearSpeak_MultiLineLabel", "Constraint", expr,
                   "2 avgränsningar; avgränsning 1; x plus y lika med 7; avgränsning 2; 2 x plus 3 y; lika med 17")?;
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
   test_ClearSpeak("sv", "ClearSpeak_MultiLineLabel", "Equation", expr,
                   "2 ekvationer; ekvation 1; x plus y lika med 7; ekvation 2; 2 x plus 3 y; lika med 17")?;
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
   test_ClearSpeak("sv", "ClearSpeak_MultiLineLabel", "Line", expr,
                   "2 rader; rad 1; x plus y lika med 7; rad 2; 2 x plus 3 y; lika med 17")?;
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
   test_ClearSpeak("sv", "ClearSpeak_MultiLineLabel", "None", expr,
                   "2; x plus y lika med 7; 2 x plus 3 y; lika med 17")?;
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
   test_ClearSpeak("sv", "ClearSpeak_MultiLineLabel", "Row", expr,
                   "2 rader; rad 1; x plus y lika med 7; rad 2; 2 x plus 3 y; lika med 17")?;
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
   test_ClearSpeak("sv", "ClearSpeak_MultiLineLabel", "Step", expr,
                   "2 steg; steg 1; x plus y lika med 7; steg 2; 2 x plus 3 y; lika med 17")?;
   return Ok(());
}
