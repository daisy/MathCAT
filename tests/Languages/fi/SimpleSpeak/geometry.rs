/// Tests for geometry listed in intent
///   ABC as mtext and as separated letters
use crate::common::*;
use anyhow::Result;

#[test]
fn arc() -> Result<()> {
  let expr = "<math>  <mover><mrow><mi>B</mi><mi>C</mi></mrow><mo>⌒</mo></mover> </math>";
  test("fi", "SimpleSpeak", expr, "kaari iso b iso c")?;
  return Ok(());

}

#[test]
fn ray() -> Result<()> {
  let expr = "<math> <mover><mrow><mi>X</mi><mi>Y</mi></mrow><mo>&#xAF;</mo></mover> </math>";
  test("fi", "SimpleSpeak", expr, "jana iso x iso y")?;
  return Ok(());

}

#[test]
fn arc_mtext() -> Result<()> {
  let expr = "<math> <mover><mtext>BC</mtext><mo>⌒</mo></mover> </math>";
  test("fi", "SimpleSpeak", expr, "kaari iso b iso c")?;
  return Ok(());

}

#[test]
fn ray_mtext() -> Result<()> {
  let expr = "<math> <mover><mtext>XY</mtext><mo>→</mo></mover> </math>";
  test("fi", "SimpleSpeak", expr, "puolisuora iso x iso y")?;
  return Ok(());

}
