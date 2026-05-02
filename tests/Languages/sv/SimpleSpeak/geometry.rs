/// Tests for geometry listed in intent
///   ABC as mtext and as separated letters
use crate::common::*;
use anyhow::Result;

#[test]
fn arc() -> Result<()> {
  let expr = "<math>  <mover><mrow><mi>B</mi><mi>C</mi></mrow><mo>⌒</mo></mover> </math>";
  test("sv", "SimpleSpeak", expr, "bågen versal b versal c")?;
  return Ok(());

}

#[test]
fn ray() -> Result<()> {
  let expr = "<math> <mover><mrow><mi>X</mi><mi>Y</mi></mrow><mo>&#xAF;</mo></mover> </math>";
  test("sv", "SimpleSpeak", expr, "linje-segmentet versal x versal y")?;
  return Ok(());

}

#[test]
fn arc_mtext() -> Result<()> {
  let expr = "<math> <mover><mtext>BC</mtext><mo>⌒</mo></mover> </math>";
  test("sv", "SimpleSpeak", expr, "bågen versal b versal c")?;
  return Ok(());

}

#[test]
fn ray_mtext() -> Result<()> {
  let expr = "<math> <mover><mtext>XY</mtext><mo>→</mo></mover> </math>";
  test("sv", "SimpleSpeak", expr, "den riktade sträckan versal x versal y")?;
  return Ok(());

}
