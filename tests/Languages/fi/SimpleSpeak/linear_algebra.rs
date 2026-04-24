use crate::common::*;
use anyhow::Result;

#[test]
fn transpose() -> Result<()> {
  let expr = "<math> <msup><mi>M</mi><mi>T</mi></msup> </math>";
  test("fi", "SimpleSpeak", expr, "iso m transpoosi")?;
  return Ok(());

}

#[test]
fn trace() -> Result<()> {
  let expr = "<math> <mi>Tr</mi><mi>M</mi> </math>";
  test("fi", "SimpleSpeak", expr, "jälki iso m")?;
  return Ok(());

}

#[test]
fn dimension() -> Result<()> {
  let expr = "<math> <mi>Dim</mi><mi>M</mi> </math>";
  test("fi", "SimpleSpeak", expr, "dimensio iso m")?;
  return Ok(());

}

#[test]
fn homomorphism() -> Result<()> {
  let expr = "<math> <mi>Hom</mi><mo>(</mo><mi>M</mi><mo>)</mo> </math>";
  test("fi", "SimpleSpeak", expr, "homomorfismi iso m")?;
  return Ok(());

}

#[test]
fn kernel() -> Result<()> {
  let expr = "<math> <mi>ker</mi><mrow><mo>(</mo><mi>L</mi><mo>)</mo></mrow> </math>";
  test("fi", "SimpleSpeak", expr, "ydin iso l")?;
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
  test("fi", "SimpleSpeak", expr, "normi f")?;
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
  test("fi", "SimpleSpeak", expr, "p normi f")?;
  return Ok(());

}