use crate::common::*;
use anyhow::Result;

// AI generated
#[test]
fn transpose() -> Result<()> {
  let expr = "<math> <msup><mi>M</mi><mi>T</mi></msup> </math>";
  test("hu", "SimpleSpeak", expr, "nagy m transzponált")?;
  return Ok(());

}

// AI generated
#[test]
fn trace() -> Result<()> {
  let expr = "<math> <mi>Tr</mi><mi>M</mi> </math>";
  test("hu", "SimpleSpeak", expr, "nyom nagy m")?;
  return Ok(());

}

// AI generated
#[test]
fn dimension() -> Result<()> {
  let expr = "<math> <mi>Dim</mi><mi>M</mi> </math>";
  test("hu", "SimpleSpeak", expr, "dimenzió nagy m")?;
  return Ok(());

}

// AI generated
#[test]
fn homomorphism() -> Result<()> {
  let expr = "<math> <mi>Hom</mi><mo>(</mo><mi>M</mi><mo>)</mo> </math>";
  test("hu", "SimpleSpeak", expr, "homomorfizmus nagy m")?;
  return Ok(());

}

// AI generated
#[test]
fn kernel() -> Result<()> {
  let expr = "<math> <mi>ker</mi><mrow><mo>(</mo><mi>L</mi><mo>)</mo></mrow> </math>";
  test("hu", "SimpleSpeak", expr, "kernel nagy l")?;
  return Ok(());

}

// AI generated
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
  test("hu", "SimpleSpeak", expr, "normalizálás f")?;
  return Ok(());

}

// AI generated
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
  test("hu", "SimpleSpeak", expr, "normalizálás x plusz y normalizálás vége")?;
  return Ok(());

}

// AI generated
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
  test("hu", "SimpleSpeak", expr, "p normalizálás a f")?;
  return Ok(());

}

// AI generated
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
  test("hu", "SimpleSpeak", expr, "nyitott zárójel, b szorozva nahblah, zárt zárójel; szorozva a")?;
  return Ok(());

}
