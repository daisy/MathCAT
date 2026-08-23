use crate::common::*;
use anyhow::Result;

#[test]
fn menclose_actuarial() -> Result<()> {
    let expr = "<math>
                    <menclose notation='actuarial'>  <mn>3</mn><mo>+</mo><mn>2</mn><mi>i</mi> </menclose>
                </math>";
    test("en", "ClearSpeak", expr, "actuarial symbol, enclosing 3 plus 2 i end enclosure")?;
    return Ok(());

}

#[test]
fn menclose_box() -> Result<()> {
    let expr = "<math>
                    <menclose notation='box circle'>  <mn>3</mn><mo>+</mo><mn>2</mn><mi>i</mi> </menclose>
                </math>";
    test("en", "ClearSpeak", expr, "box, circle, enclosing 3 plus 2 i end enclosure")?;
    return Ok(());

}

#[test]
fn menclose_left() -> Result<()> {
    let expr = "<math>
                    <menclose notation='left'>  <mfrac><mn>3</mn><mn>2</mn></mfrac> </menclose>
                </math>";
    test("en", "ClearSpeak", expr, "line on left, enclosing 3 halves end enclosure")?;
    return Ok(());

}

#[test]
fn menclose_right() -> Result<()> {
    let expr = "<math>
                    <menclose notation='right'>  <mfrac><mn>3</mn><mn>2</mn></mfrac> </menclose>
                </math>";
    test("en", "ClearSpeak", expr, "line on right, enclosing 3 halves end enclosure")?;
    return Ok(());

}

#[test]
fn menclose_top_bottom() -> Result<()> {
    let expr = "<math>
                    <menclose notation='top bottom'>  <mfrac><mn>3</mn><mn>2</mn></mfrac> </menclose>
                </math>";
    test("en", "ClearSpeak", expr, "line on top, bottom, enclosing 3 halves end enclosure")?;
    return Ok(());

}

#[test]
fn menclose_updiagonalstrike() -> Result<()> {
    let expr = "<math>
                    <menclose notation='updiagonalstrike'>  <mfrac><mn>3</mn><mn>2</mn></mfrac> </menclose>
                </math>";
    test("en", "ClearSpeak", expr, "up diagonal, cross out, enclosing 3 halves end enclosure")?;
    return Ok(());

}

#[test]
fn menclose_downdiagonalstrike() -> Result<()> {
    let expr = "<math>
                    <menclose notation='downdiagonalstrike'>  <mfrac><mn>3</mn><mn>2</mn></mfrac> </menclose>
                </math>";
    test("en", "ClearSpeak", expr, "down diagonal, cross out, enclosing 3 halves end enclosure")?;
    return Ok(());

}

#[test]
fn menclose_cross_out() -> Result<()> {
    let expr = "<math>
                    <menclose notation='updiagonalstrike downdiagonalstrike'>  <mfrac><mn>3</mn><mn>2</mn></mfrac> </menclose>
                </math>";
    test("en", "ClearSpeak", expr, "x, cross out, enclosing 3 halves end enclosure")?;
    return Ok(());

}

#[test]
fn menclose_vertical_horizontal_strike() -> Result<()> {
    let expr = "<math>
                    <menclose notation='verticalstrike horizontalstrike'>  <mfrac><mn>3</mn><mn>2</mn></mfrac> </menclose>
                </math>";
    test("en", "ClearSpeak", expr, "vertical, horizontal, cross out, enclosing 3 halves end enclosure")?;
    return Ok(());

}

#[test]
fn menclose_leftarrow() -> Result<()> {
    let expr = "<math>
                    <menclose notation='leftarrow'> <mfrac><mn>3</mn><mn>2</mn></mfrac> </menclose>
                </math>";
    test("en", "ClearSpeak", expr, "left arrow, enclosing 3 halves end enclosure")?;
    return Ok(());

}

#[test]
fn menclose_right_up_down_arrow() -> Result<()> {
    let expr = "<math>
                    <menclose notation=' rightarrow downarrow  uparrow  '> <mfrac><mn>3</mn><mn>2</mn></mfrac> </menclose>
                </math>";
    test("en", "ClearSpeak", expr, "up arrow, down arrow, right arrow, enclosing 3 halves end enclosure")?;
    return Ok(());

}

#[test]
fn menclose_northeastarrow() -> Result<()> {
    let expr = "<math>
                    <menclose notation='northeastarrow'> <mfrac><mn>3</mn><mn>2</mn></mfrac> </menclose>
                </math>";
    test("en", "ClearSpeak", expr, "northeast arrow, enclosing 3 halves end enclosure")?;
    return Ok(());

}

#[test]
fn menclose_other_single_arrows() -> Result<()> {
    let expr = "<math>
                    <menclose notation='northwestarrow southwestarrow southeastarrow'> <mfrac><mn>3</mn><mn>2</mn></mfrac> </menclose>
                </math>";
    test("en", "ClearSpeak", expr, "southeast arrow, southwest arrow, northwest arrow, enclosing 3 halves end enclosure")?;
    return Ok(());

}

#[test]
fn menclose_northwestsoutheastarrow() -> Result<()> {
    let expr = "<math>
                    <menclose notation='northwestsoutheastarrow'> <mfrac><mn>3</mn><mn>2</mn></mfrac> </menclose>
                </math>";
    test("en", "ClearSpeak", expr, "double ended down diagonal arrow, enclosing 3 halves end enclosure")?;
    return Ok(());

}

#[test]
fn menclose_other_double_arrows() -> Result<()> {
    let expr = "<math>
                    <menclose notation='updownarrow leftrightarrow northeastsouthwestarrow'>  <mfrac><mn>3</mn><mn>2</mn></mfrac> </menclose>
                </math>";
    test("en", "ClearSpeak", expr, "double ended vertical arrow, double ended horizontal arrow, double ended up diagonal arrow, enclosing 3 halves end enclosure")?;
    return Ok(());

}

#[test]
fn menclose_madrub() -> Result<()> {
    let expr = "<math>
                    <menclose notation='madrub'> <mfrac><mn>3</mn><mn>2</mn></mfrac> </menclose>
                </math>";
    test("en", "ClearSpeak", expr, "arabic factorial symbol, enclosing 3 halves end enclosure")?;
    return Ok(());

}

#[test]
fn menclose_phasorangle() -> Result<()> {
    let expr = "<math>
                    <menclose notation='phasorangle'> <mfrac><mn>3</mn><mn>2</mn></mfrac> </menclose>
                </math>";
    test("en", "ClearSpeak", expr, "phasor angle, enclosing 3 halves end enclosure")?;
    return Ok(());

}

#[test]
fn menclose_circle_phasorangle() -> Result<()> {
    let expr = "<math>
                    <menclose notation='phasorangle circle'> <mfrac><mn>3</mn><mn>2</mn></mfrac> </menclose>
                </math>";
    test("en", "ClearSpeak", expr, "circle, phasor angle, enclosing 3 halves end enclosure")?;
    return Ok(());

}

#[test]
fn menclose_longdiv() -> Result<()> {
    let expr = "<math>
                    <menclose notation='longdiv'> <mfrac><mn>3</mn><mn>2</mn></mfrac> </menclose>
                </math>";
    test("en", "ClearSpeak", expr, "long division symbol, enclosing 3 halves end enclosure")?;
    return Ok(());

}

#[test]
fn menclose_longdiv_default() -> Result<()> {
    let expr = "<math>
                    <menclose> <mfrac><mn>3</mn><mn>2</mn></mfrac> </menclose>
                </math>";
    test("en", "ClearSpeak", expr, "long division symbol, enclosing 3 halves end enclosure")?;
    return Ok(());

}

#[test]
fn menclose_longdiv_empty_string() -> Result<()> {
    let expr = "<math>
                    <menclose notation=''> <mfrac><mn>3</mn><mn>2</mn></mfrac> </menclose>
                </math>";
    test("en", "ClearSpeak", expr, "long division symbol, enclosing 3 halves end enclosure")?;
    return Ok(());

}

#[test]
fn menclose_longdiv_whitespace_string() -> Result<()> {
    let expr = "<math>
                    <menclose notation='  '> <mfrac><mn>3</mn><mn>2</mn></mfrac> </menclose>
                </math>";
    test("en", "ClearSpeak", expr, "long division symbol, enclosing 3 halves end enclosure")?;
    return Ok(());

}

#[test]
fn menclose_radical() -> Result<()> {
    let expr = "<math>
                    <menclose notation='radical'> <mfrac><mn>3</mn><mn>2</mn></mfrac> </menclose>
                </math>";
    test("en", "ClearSpeak", expr, "square root, enclosing 3 halves end enclosure")?;
    return Ok(());

}

#[test]
fn simple_speak_menclose_top_bottom() -> Result<()> {
    let expr = "<math>
                    <menclose notation='top bottom'>  <mfrac><mn>3</mn><mn>2</mn></mfrac> </menclose>
                </math>";
    test("en", "SimpleSpeak", expr, "line on top, bottom, enclosing 3 halves end enclosure")?;
    return Ok(());

}
