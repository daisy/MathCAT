use crate::common::*;

use anyhow::Result;

#[test]
fn transpose() -> Result<()> {
  let expr = "<math> <msup><mi>M</mi><mi>T</mi></msup> </math>";
  test("ru", "SimpleSpeak", expr, "эм большое транспонированная")?;
  return Ok(());
}

#[test]
fn trace() -> Result<()> {
  let expr = "<math> <mi>Tr</mi><mi>M</mi> </math>";
  test("ru", "SimpleSpeak", expr, "след от эм большое")?;
  return Ok(());
}

#[test]
fn dimension() -> Result<()> {
  let expr = "<math> <mi>Dim</mi><mi>M</mi> </math>";
  test("ru", "SimpleSpeak", expr, "размерность эм большое")?;
  return Ok(());
}

#[test]
fn homomorphism() -> Result<()> {
  let expr = "<math> <mi>Hom</mi><mo>(</mo><mi>M</mi><mo>)</mo> </math>";
  test("ru", "SimpleSpeak", expr, "гомоморфизм от эм большое")?;
  return Ok(());
}

#[test]
fn kernel() -> Result<()> {
  let expr = "<math> <mi>ker</mi><mrow><mo>(</mo><mi>L</mi><mo>)</mo></mrow> </math>";
  test("ru", "SimpleSpeak", expr, "ядро от эль большое")?;
  return Ok(());
}

#[test]
fn norm() -> Result<()> {
  let expr = "  <math>
    <mrow>
      <mo>∥</mo>
      <mi>f</mi>
      <mo>∥</mo>
    </mrow>
</math>
";
  test("ru", "SimpleSpeak", expr, "норма от эф")?;
  return Ok(());
}

#[test]
fn norm_non_simple() -> Result<()> {
  let expr = "  <math>
    <mrow>
      <mo>∥</mo>
      <mi>x</mi>
      <mo>+</mo>
      <mi>y</mi>
      <mo>∥</mo>
    </mrow>
</math>
";
  test("ru", "SimpleSpeak", expr, "норма от икс плюс игрек конец нормы")?;
  return Ok(());
}

#[test]
fn norm_subscripted() -> Result<()> {
  let expr = "  <math>
    <msub>
      <mrow>
        <mo>∥</mo>
        <mi>f</mi>
        <mo>∥</mo>
      </mrow>
      <mi>p</mi>
    </msub>
</math>
";
  test("ru", "SimpleSpeak", expr, "p-норма от эф")?;
  return Ok(());
}

#[test]
fn not_gradient() -> Result<()> {
  // the nabla is at the end, so it can't be gradient because it doesn't operate on anything
  let expr = r#"<math>
  <mo>(</mo>
  <mi>b</mi>
  <mo>&#x22C5;</mo>
  <mrow>
    <mo>&#x2207;</mo>
  </mrow>
  <mo>)</mo>
  <mi>a</mi>
</math>
"#;
  test("ru", "SimpleSpeak", expr, "скобка открывается, бэ умножить на набла, скобка закрывается; умножить на а")?;
  return Ok(());
}