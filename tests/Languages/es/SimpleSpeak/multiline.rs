use crate::common::*;
use anyhow::Result;

// REVISAR (inglés: if): «f de x es igual a; 3 casos; caso 1; negativo 1 if x; es menor que 0; caso 2; 0 if x, es igual a 0; caso 3; 1 if x, es mayor que 0»
#[test]
#[ignore = "lectura en español pendiente de revisión"]
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
    test("es", "SimpleSpeak", expr, "f de x es igual a; 3 casos; caso 1; negativo 1 if x; es menor que 0; caso 2; 0 if x, es igual a 0; caso 3; 1 if x, es mayor que 0")?;
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
    test("es", "SimpleSpeak", expr, "2 ecuacións; ecuación 1; x más y, es igual a 7; ecuación 2; 2 x más 3 y; es igual a 17")?;
    return Ok(());
}
