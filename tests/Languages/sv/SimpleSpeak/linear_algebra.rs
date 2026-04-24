use crate::common::*;
use anyhow::Result;

#[test]
fn transpose() -> Result<()> {
  let expr = "<math> <msup><mi>M</mi><mi>T</mi></msup> </math>";
  test("sv", "SimpleSpeak", expr, "versal m transponat")?;
  return Ok(());

}

#[test]
fn trace() -> Result<()> {
  let expr = "<math> <mi>Tr</mi><mi>M</mi> </math>";
  test("sv", "SimpleSpeak", expr, "spåret av versal m")?;
  return Ok(());

}

#[test]
fn dimension() -> Result<()> {
  let expr = "<math> <mi>Dim</mi><mi>M</mi> </math>";
  test("sv", "SimpleSpeak", expr, "dimensionen av versal m")?;
  return Ok(());

}

#[test]
fn homomorphism() -> Result<()> {
  let expr = "<math> <mi>Hom</mi><mo>(</mo><mi>M</mi><mo>)</mo> </math>";
  test_prefs("sv", "SimpleSpeak", vec![("Verbosity", "Terse")],
  expr, "endomorfismerna på versal m")?;
  test_prefs("sv", "SimpleSpeak", vec![("Verbosity", "Medium")],
  expr, "mängden av endomorfismer på versal m")?;
  test_prefs("sv", "SimpleSpeak", vec![("Verbosity", "Verbose")],
  expr, "mängden av endomorfismer på versal m")?;
  return Ok(());

}

#[test]
fn kernel() -> Result<()> {
  let expr = "<math> <mi>ker</mi><mrow><mo>(</mo><mi>L</mi><mo>)</mo></mrow> </math>";
  test_prefs("sv", "SimpleSpeak", vec![("Verbosity", "Terse")],
  expr, "noll-rummet versal l")?;
  test_prefs("sv", "SimpleSpeak", vec![("Verbosity", "Medium")],
  expr, "noll-rummet till versal l")?;
  test_prefs("sv", "SimpleSpeak", vec![("Verbosity", "Verbose")],
  expr, "noll-rummet till versal l")?;
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
  test("sv", "SimpleSpeak", expr, "normen av f")?;
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
  test("sv", "SimpleSpeak", expr, "p normen av f")?;
  return Ok(());

}
