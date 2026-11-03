use crate::common::*;
use anyhow::Result;


// REVISAR (inglés: if): «f de x es igual a; 3 casos, caso 1; menos 1 if x es menor que 0; caso 2; 0 if x es igual a 0; caso 3; 1 if x es mayor que 0»
#[test]
#[ignore = "lectura en español pendiente de revisión"]
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
  test_ClearSpeak("es", "ClearSpeak_MultiLineLabel", "Auto", expr,
    "f de x es igual a; 3 casos, caso 1; menos 1 if x es menor que 0; caso 2; 0 if x es igual a 0; caso 3; 1 if x es mayor que 0"
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
   test_ClearSpeak("es", "ClearSpeak_MultiLineLabel", "Auto", expr,
                "2 líneas, línea 1; x más y, es igual a 7; línea 2; 2 x más 3 y; es igual a 17")?;
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
   test_ClearSpeak("es", "ClearSpeak_MultiLineLabel", "Auto", expr, "2 líneas, línea 1; x más y es igual a 7; línea 2; 2 x, más 3 y, es igual a 17")?;
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
   test_ClearSpeak("es", "ClearSpeak_MultiLineLabel", "Case", expr, 
   "2 casos, caso 1; x más y, es igual a 7; caso 2; 2 x más 3 y; es igual a 17")?;
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
   test_ClearSpeak("es", "ClearSpeak_MultiLineLabel", "Constraint", expr, "2 superficie de ligaduras, superficie de ligadura 1; x más y, es igual a 7; superficie de ligadura 2; 2 x más 3 y; es igual a 17")?;
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
   test_ClearSpeak("es", "ClearSpeak_MultiLineLabel", "Equation", expr, "2 ecuacións, ecuación 1; x más y, es igual a 7; ecuación 2; 2 x más 3 y; es igual a 17")?;
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
   test_ClearSpeak("es", "ClearSpeak_MultiLineLabel", "Line", expr, "2 líneas, línea 1; x más y, es igual a 7; línea 2; 2 x más 3 y; es igual a 17")?;
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
   test_ClearSpeak("es", "ClearSpeak_MultiLineLabel", "None", expr,
        "2, x más y, es igual a 7; 2 x más 3 y; es igual a 17")?;
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
   test_ClearSpeak("es", "ClearSpeak_MultiLineLabel", "Row", expr, "2 filas, fila 1; x más y, es igual a 7; fila 2; 2 x más 3 y; es igual a 17")?;
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
   test_ClearSpeak("es", "ClearSpeak_MultiLineLabel", "Step", expr, "2 nivels, nivel 1; x más y, es igual a 7; nivel 2; 2 x más 3 y; es igual a 17")?;
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
test("es", "SimpleSpeak", expr,
     "3 ecuacións; ecuación 1; x es igual a y; ecuación 2; vacío vacío más 1; ecuación 3; y es igual a 1")?;
    return Ok(());
}
