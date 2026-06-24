/// Tests for geometry listed in intent
///   ABC as mtext and as separated letters
use crate::common::*;
use anyhow::Result;

// AI generated
#[test]
fn arc() -> Result<()> {
  let expr = "<math>  <mover><mrow><mi>B</mi><mi>C</mi></mrow><mo>⌒</mo></mover> </math>";
  test("hu", "SimpleSpeak", expr, "ív nagy b nagy c")?;
  return Ok(());

}

// AI generated
#[test]
fn ray() -> Result<()> {
  let expr = "<math> <mover><mrow><mi>X</mi><mi>Y</mi></mrow><mo>&#xAF;</mo></mover> </math>";
  test("hu", "SimpleSpeak", expr, "vonalszakasz nagy x nagy y")?;
  return Ok(());

}

// AI generated
#[test]
fn arc_mtext() -> Result<()> {
  let expr = "<math> <mover><mtext>BC</mtext><mo>⌒</mo></mover> </math>";
  test("hu", "SimpleSpeak", expr, "ív nagy b nagy c")?;
  return Ok(());

}

// AI generated
#[test]
fn ray_mtext() -> Result<()> {
  let expr = "<math> <mover><mtext>XY</mtext><mo>→</mo></mover> </math>";
  test("hu", "SimpleSpeak", expr, "sugár nagy x nagy y")?;
  return Ok(());

}
