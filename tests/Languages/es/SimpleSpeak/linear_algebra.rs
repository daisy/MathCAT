use crate::common::*;
use anyhow::Result;

// REVISAR (intercambiar): «mayúscula m intercambiar»
#[test]
#[ignore = "lectura en español pendiente de revisión"]
fn transpose() -> Result<()> {
  let expr = "<math> <msup><mi>M</mi><mi>T</mi></msup> </math>";
  test("es", "SimpleSpeak", expr, "mayúscula m intercambiar")?;
  return Ok(());

}

// REVISAR (de de): «traza de de mayúscula m»
#[test]
#[ignore = "lectura en español pendiente de revisión"]
fn trace() -> Result<()> {
  let expr = "<math> <mi>Tr</mi><mi>M</mi> </math>";
  test("es", "SimpleSpeak", expr, "traza de de mayúscula m")?;
  return Ok(());

}

// REVISAR (de de): «dimensión de de mayúscula m»
#[test]
#[ignore = "lectura en español pendiente de revisión"]
fn dimension() -> Result<()> {
  let expr = "<math> <mi>Dim</mi><mi>M</mi> </math>";
  test("es", "SimpleSpeak", expr, "dimensión de de mayúscula m")?;
  return Ok(());

}

// REVISAR (de de): «homomorfismo de de mayúscula m»
#[test]
#[ignore = "lectura en español pendiente de revisión"]
fn homomorphism() -> Result<()> {
  let expr = "<math> <mi>Hom</mi><mo>(</mo><mi>M</mi><mo>)</mo> </math>";
  test("es", "SimpleSpeak", expr, "homomorfismo de de mayúscula m")?;
  return Ok(());

}

// REVISAR (de de): «núcleo de de mayúscula l»
#[test]
#[ignore = "lectura en español pendiente de revisión"]
fn kernel() -> Result<()> {
  let expr = "<math> <mi>ker</mi><mrow><mo>(</mo><mi>L</mi><mo>)</mo></mrow> </math>";
  test("es", "SimpleSpeak", expr, "núcleo de de mayúscula l")?;
  return Ok(());

}

// REVISAR (regla): «regla de f»
#[test]
#[ignore = "lectura en español pendiente de revisión"]
fn norm() -> Result<()> {
  let expr = "  <math>
    <mrow>
      <mo>∥</mo>
      <mi>f</mi>
      <mo>∥</mo>
    </mrow>
</math>
";
  test("es", "SimpleSpeak", expr, "regla de f")?;
  return Ok(());

}

// REVISAR (regla): «regla de x más y fin de la regla»
#[test]
#[ignore = "lectura en español pendiente de revisión"]
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
  test("es", "SimpleSpeak", expr, "regla de x más y fin de la regla")?;
  return Ok(());

}

// REVISAR (regla): «p regla de f»
#[test]
#[ignore = "lectura en español pendiente de revisión"]
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
  test("es", "SimpleSpeak", expr, "p regla de f")?;
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
  test("es", "SimpleSpeak", expr, "se abren paréntesis, b punto el gradiente, se cierran paréntesis; por a")?;
  return Ok(());

}