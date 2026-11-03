use crate::common::*;
use anyhow::Result;

#[test]
fn transpose() -> Result<()> {
  let expr = "<math> <msup><mi>M</mi><mi>T</mi></msup> </math>";
  test("es", "SimpleSpeak", expr, "mayúscula m transpuesta")?;
  return Ok(());

}

#[test]
fn trace() -> Result<()> {
  let expr = "<math> <mi>Tr</mi><mi>M</mi> </math>";
  test("es", "SimpleSpeak", expr, "la traza de mayúscula m")?;
  return Ok(());

}

#[test]
fn dimension() -> Result<()> {
  let expr = "<math> <mi>Dim</mi><mi>M</mi> </math>";
  test("es", "SimpleSpeak", expr, "la dimensión de mayúscula m")?;
  return Ok(());

}

#[test]
fn homomorphism() -> Result<()> {
  let expr = "<math> <mi>Hom</mi><mo>(</mo><mi>M</mi><mo>)</mo> </math>";
  test("es", "SimpleSpeak", expr, "el homomorfismo de mayúscula m")?;
  return Ok(());

}

#[test]
fn kernel() -> Result<()> {
  let expr = "<math> <mi>ker</mi><mrow><mo>(</mo><mi>L</mi><mo>)</mo></mrow> </math>";
  test("es", "SimpleSpeak", expr, "el núcleo de mayúscula l")?;
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
  test("es", "SimpleSpeak", expr, "norma de f")?;
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
  test("es", "SimpleSpeak", expr, "norma de x más y fin de la norma")?;
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
  test("es", "SimpleSpeak", expr, "norma p de f")?;
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
  test("es", "SimpleSpeak", expr, "abrir paréntesis, b por nabla, cerrar paréntesis; por a")?;
  return Ok(());

}

#[test]
fn subscripted_double_bars() -> Result<()> {
  // ‖f‖_p en LaTeX corriente, sin intent
  let expr = "<math><msub><mrow><mo>‖</mo><mi>f</mi><mo>‖</mo></mrow><mi>p</mi></msub></math>";
  test("es", "SimpleSpeak", expr, "la norma p de f")?;
  let expr = "<math><mrow><mo>‖</mo><mi>v</mi><mo>‖</mo></mrow></math>";
  test("es", "SimpleSpeak", expr, "la norma de v")?;
  return Ok(());
}
