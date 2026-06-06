use crate::common::*;
use anyhow::Result;

#[test]
fn transpose() -> Result<()> {
  let expr = "<math> <msup><mi>M</mi><mi>T</mi></msup> </math>";
  test("zh-tw", "SimpleSpeak", expr, "大寫 m 的 轉置")?;
  return Ok(());

}

#[test]
fn trace() -> Result<()> {
  let expr = "<math> <mi>Tr</mi><mi>M</mi> </math>";
  test("zh-tw", "SimpleSpeak", expr, "大寫 m 的 跡")?;
  return Ok(());

}

#[test]
fn dimension() -> Result<()> {
  let expr = "<math> <mi>Dim</mi><mi>M</mi> </math>";
  test("zh-tw", "SimpleSpeak", expr, "大寫 m 的 維數")?;
  return Ok(());

}

#[test]
fn homomorphism() -> Result<()> {
  let expr = "<math> <mi>Hom</mi><mo>(</mo><mi>M</mi><mo>)</mo> </math>";
  test("zh-tw", "SimpleSpeak", expr, "大寫 m 的 同態")?;
  return Ok(());

}

#[test]
fn kernel() -> Result<()> {
  let expr = "<math> <mi>ker</mi><mrow><mo>(</mo><mi>L</mi><mo>)</mo></mrow> </math>";
  test("zh-tw", "SimpleSpeak", expr, "大寫 l 的 核")?;
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
  test("zh-tw", "SimpleSpeak", expr, "f 的 範數")?;
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
  test("zh-tw", "SimpleSpeak", expr, "f 的 p 範數")?;
  return Ok(());

}