use crate::common::*;
use anyhow::Result;

#[test]
fn case_1() -> Result<()> {
    let expr = "<math>
            <mrow>
            <mi>f</mi><mrow><mo>(</mo>
            <mi>x</mi>
            <mo>)</mo></mrow><mo>=</mo><mrow><mo>{</mo> <mrow>
            <mtable>
            <mtr>
                <mtd>
                <mrow>
                <mo>&#x2212;</mo><mn>1</mn><mtext>&#x00A0;si&#x00A0;</mtext><mi>x</mi><mo>&#x003C;</mo><mn>0</mn></mrow>
                </mtd>
            </mtr>
            <mtr>
                <mtd>
                <mrow>
                <mn>0</mn><mtext>&#x00A0;si&#x00A0;</mtext><mi>x</mi><mo>=</mo><mn>0</mn></mrow>
                </mtd>
            </mtr>
            <mtr>
                <mtd>
                <mrow>
                <mn>1</mn><mtext>&#x00A0;si&#x00A0;</mtext><mi>x</mi><mo>&#x003E;</mo><mn>0</mn></mrow>
                </mtd>
            </mtr>
            </mtable></mrow> </mrow></mrow>
        </math>
   ";
    test("fr", "SimpleSpeak", expr, "f de x est égal à; 3 cas; \
    cas 1; moins 1 si x; est inférieur à 0; \
    cas 2; 0 si x, est égal à 0; \
    cas 3; 1 si x, est supérieur à 0")?;
    return Ok(());
}

#[test]
fn equation_1() -> Result<()> {
    let expr = "<math>
     <mrow>
      <mtable>
       <mtr>
        <mtd>
         <mrow>
          <mi>x</mi><mo>+</mo><mi>y</mi></mrow>
        </mtd>
        <mtd>
         <mo>=</mo>
        </mtd>
        <mtd>
         <mn>7</mn>
        </mtd>
       </mtr>
       <mtr>
        <mtd>
         <mrow>
          <mn>2</mn><mi>x</mi><mo>+</mo><mn>3</mn><mi>y</mi></mrow>
        </mtd>
        <mtd>
         <mo>=</mo>
        </mtd>
        <mtd>
         <mrow>
          <mn>17</mn></mrow>
        </mtd>
       </mtr>
       
      </mtable></mrow>
    </math>
   ";
    test("fr", "SimpleSpeak", expr, "2 équations; \
                équation 1; x plus y, est égal à 7; \
                équation 2; 2 x plus 3 y; est égal à 17")?;
    return Ok(());
}
