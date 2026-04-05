use crate::common::*;

#[test]
fn case_1() {
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
    test("ru", "SimpleSpeak", expr, "эф от икс равно; 3 случая; \
                случай 1; минус 1, если икс; меньше 0; \
                случай 2; 0, если икс, равно 0; \
                случай 3; 1, если икс, больше 0");
}

#[test]
fn equation_1() {
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
    test("ru", "SimpleSpeak", expr, "2 уравнения; \
                уравнение 1; икс плюс игрек равно 7; \
                уравнение 2; 2 икс плюс 3 игрек равно 17");
}
