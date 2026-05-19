use crate::common::*;
use anyhow::Result;

#[test]
fn transpose() -> Result<()> {
  let expr = "<math> <msup><mi>M</mi><mi>T</mi></msup> </math>";
  test("nb", "SimpleSpeak", expr, "stor m transponert")?;
  return Ok(());

}

#[test]
fn trace() -> Result<()> {
  let expr = "<math> <mi>Tr</mi><mi>M</mi> </math>";
  test("nb", "SimpleSpeak", expr, "sporet av stor m")?;
  return Ok(());

}

#[test]
fn dimension() -> Result<()> {
  let expr = "<math> <mi>Dim</mi><mi>M</mi> </math>";
  test("nb", "SimpleSpeak", expr, "dimensjonen til stor m")?;
  return Ok(());

}

#[test]
fn homomorphism() -> Result<()> {
  let expr = "<math> <mi>Hom</mi><mo>(</mo><mi>M</mi><mo>)</mo> </math>";
  test("nb", "SimpleSpeak", expr, "mengden av homomorfier på stor m")?;
  return Ok(());

}

#[test]
fn kernel() -> Result<()> {
  let expr = "<math> <mi>ker</mi><mrow><mo>(</mo><mi>L</mi><mo>)</mo></mrow> </math>";
  test("nb", "SimpleSpeak", expr, "nullrommet til stor l")?;
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
  test("nb", "SimpleSpeak", expr, "normen til f")?;
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
  test("nb", "SimpleSpeak", expr, "normen til x pluss y, slutt norm")?;
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
  test("nb", "SimpleSpeak", expr, "p normen til f")?;
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
  test("nb", "SimpleSpeak", expr, ", startparentes; b ganger nabla; sluttparentes; ganger a")?;
  return Ok(());

}