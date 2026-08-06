use crate::common::*;
use anyhow::Result;

// AI generated
#[test]
fn menclose_actuarial() -> Result<()> {
    let expr = "<math>
                    <menclose notation='actuarial'>  <mn>3</mn><mo>+</mo><mn>2</mn><mi>i</mi> </menclose>
                </math>";
    test("hu", "ClearSpeak", expr, "aktuárius szimbólum, kezdet: 3 plusz 2 i végpont vége")?;
    return Ok(());

}

// AI generated
#[test]
fn menclose_box() -> Result<()> {
    let expr = "<math>
                    <menclose notation='box circle'>  <mn>3</mn><mo>+</mo><mn>2</mn><mi>i</mi> </menclose>
                </math>";
    test("hu", "ClearSpeak", expr, "doboz, kör, kezdet: 3 plusz 2 i végpont vége")?;
    return Ok(());

}

// AI generated
#[test]
fn menclose_left() -> Result<()> {
    let expr = "<math>
                    <menclose notation='left'>  <mfrac><mn>3</mn><mn>2</mn></mfrac> </menclose>
                </math>";
    test("hu", "ClearSpeak", expr, "vonal balra, kezdet: 3 ketted végpont vége")?;
    return Ok(());

}

// AI generated
#[test]
fn menclose_right() -> Result<()> {
    let expr = "<math>
                    <menclose notation='right'>  <mfrac><mn>3</mn><mn>2</mn></mfrac> </menclose>
                </math>";
    test("hu", "ClearSpeak", expr, "vonal jobbra, kezdet: 3 ketted végpont vége")?;
    return Ok(());

}

// AI generated
#[test]
fn menclose_top_bottom() -> Result<()> {
    let expr = "<math>
                    <menclose notation='top bottom'>  <mfrac><mn>3</mn><mn>2</mn></mfrac> </menclose>
                </math>";
    test("hu", "ClearSpeak", expr, "vonal felső, alsó, kezdet: 3 ketted végpont vége")?;
    return Ok(());

}

// AI generated
#[test]
fn menclose_updiagonalstrike() -> Result<()> {
    let expr = "<math>
                    <menclose notation='updiagonalstrike'>  <mfrac><mn>3</mn><mn>2</mn></mfrac> </menclose>
                </math>";
    test("hu", "ClearSpeak", expr, "átlósan felfelé, kihúzva, kezdet: 3 ketted végpont vége")?;
    return Ok(());

}

// AI generated
#[test]
fn menclose_downdiagonalstrike() -> Result<()> {
    let expr = "<math>
                    <menclose notation='downdiagonalstrike'>  <mfrac><mn>3</mn><mn>2</mn></mfrac> </menclose>
                </math>";
    test("hu", "ClearSpeak", expr, "átlósan lefelé, kihúzva, kezdet: 3 ketted végpont vége")?;
    return Ok(());

}

// AI generated
#[test]
fn menclose_cross_out() -> Result<()> {
    let expr = "<math>
                    <menclose notation='updiagonalstrike downdiagonalstrike'>  <mfrac><mn>3</mn><mn>2</mn></mfrac> </menclose>
                </math>";
    test("hu", "ClearSpeak", expr, "x, kihúzva, kezdet: 3 ketted végpont vége")?;
    return Ok(());

}

// AI generated
#[test]
fn menclose_vertical_horizontal_strike() -> Result<()> {
    let expr = "<math>
                    <menclose notation='verticalstrike horizontalstrike'>  <mfrac><mn>3</mn><mn>2</mn></mfrac> </menclose>
                </math>";
    test("hu", "ClearSpeak", expr, "függőleges, vízszintes, kihúzva, kezdet: 3 ketted végpont vége")?;
    return Ok(());

}

// AI generated
#[test]
fn menclose_leftarrow() -> Result<()> {
    let expr = "<math>
                    <menclose notation='leftarrow'> <mfrac><mn>3</mn><mn>2</mn></mfrac> </menclose>
                </math>";
    test("hu", "ClearSpeak", expr, "balra nyíl, kezdet: 3 ketted végpont vége")?;
    return Ok(());

}

// AI generated
#[test]
fn menclose_right_up_down_arrow() -> Result<()> {
    let expr = "<math>
                    <menclose notation=' rightarrow downarrow  uparrow  '> <mfrac><mn>3</mn><mn>2</mn></mfrac> </menclose>
                </math>";
    test("hu", "ClearSpeak", expr, "felfelé nyíl, lefelé nyíl, jobbra nyíl, kezdet: 3 ketted végpont vége")?;
    return Ok(());

}

// AI generated
#[test]
fn menclose_northeastarrow() -> Result<()> {
    let expr = "<math>
                    <menclose notation='northeastarrow'> <mfrac><mn>3</mn><mn>2</mn></mfrac> </menclose>
                </math>";
    test("hu", "ClearSpeak", expr, "északkeleti nyíl, kezdet: 3 ketted végpont vége")?;
    return Ok(());

}

// AI generated
#[test]
fn menclose_other_single_arrows() -> Result<()> {
    let expr = "<math>
                    <menclose notation='northwestarrow southwestarrow southeastarrow'> <mfrac><mn>3</mn><mn>2</mn></mfrac> </menclose>
                </math>";
    test("hu", "ClearSpeak", expr, "délkeleti nyíl, délnyugati nyíl, északnyugati nyíl, kezdet: 3 ketted végpont vége")?;
    return Ok(());

}

// AI generated
#[test]
fn menclose_northwestsoutheastarrow() -> Result<()> {
    let expr = "<math>
                    <menclose notation='northwestsoutheastarrow'> <mfrac><mn>3</mn><mn>2</mn></mfrac> </menclose>
                </math>";
    test("hu", "ClearSpeak", expr, "kétvégű lefelé mutató átlós nyíl, kezdet: 3 ketted végpont vége")?;
    return Ok(());

}

// AI generated
#[test]
fn menclose_other_double_arrows() -> Result<()> {
    let expr = "<math>
                    <menclose notation='updownarrow leftrightarrow northeastsouthwestarrow'>  <mfrac><mn>3</mn><mn>2</mn></mfrac> </menclose>
                </math>";
    test("hu", "ClearSpeak", expr, "kétvégű függőleges nyíl, kétvégű vízszintes nyíl, dupla végű átlós nyíl, kezdet: 3 ketted végpont vége")?;
    return Ok(());

}

// AI generated
#[test]
fn menclose_madrub() -> Result<()> {
    let expr = "<math>
                    <menclose notation='madrub'> <mfrac><mn>3</mn><mn>2</mn></mfrac> </menclose>
                </math>";
    test("hu", "ClearSpeak", expr, "arab faktoriális szimbólum, kezdet: 3 ketted végpont vége")?;
    return Ok(());

}

// AI generated
#[test]
fn menclose_phasorangle() -> Result<()> {
    let expr = "<math>
                    <menclose notation='phasorangle'> <mfrac><mn>3</mn><mn>2</mn></mfrac> </menclose>
                </math>";
    test("hu", "ClearSpeak", expr, "fázisszög, kezdet: 3 ketted végpont vége")?;
    return Ok(());

}

// AI generated
#[test]
fn menclose_circle_phasorangle() -> Result<()> {
    let expr = "<math>
                    <menclose notation='phasorangle circle'> <mfrac><mn>3</mn><mn>2</mn></mfrac> </menclose>
                </math>";
    test("hu", "ClearSpeak", expr, "kör, fázisszög, kezdet: 3 ketted végpont vége")?;
    return Ok(());

}

// AI generated
#[test]
fn menclose_longdiv() -> Result<()> {
    let expr = "<math>
                    <menclose notation='longdiv'> <mfrac><mn>3</mn><mn>2</mn></mfrac> </menclose>
                </math>";
    test("hu", "ClearSpeak", expr, "hosszú osztásjel, kezdet: 3 ketted végpont vége")?;
    return Ok(());

}

// AI generated
#[test]
fn menclose_longdiv_default() -> Result<()> {
    let expr = "<math>
                    <menclose> <mfrac><mn>3</mn><mn>2</mn></mfrac> </menclose>
                </math>";
    test("hu", "ClearSpeak", expr, "hosszú osztásjel, kezdet: 3 ketted végpont vége")?;
    return Ok(());

}

// AI generated
#[test]
fn menclose_longdiv_empty_string() -> Result<()> {
    let expr = "<math>
                    <menclose notation=''> <mfrac><mn>3</mn><mn>2</mn></mfrac> </menclose>
                </math>";
    test("hu", "ClearSpeak", expr, "hosszú osztásjel, kezdet: 3 ketted végpont vége")?;
    return Ok(());

}

// AI generated
#[test]
fn menclose_longdiv_whitespace_string() -> Result<()> {
    let expr = "<math>
                    <menclose notation='  '> <mfrac><mn>3</mn><mn>2</mn></mfrac> </menclose>
                </math>";
    test("hu", "ClearSpeak", expr, "hosszú osztásjel, kezdet: 3 ketted végpont vége")?;
    return Ok(());

}

// AI generated
#[test]
fn menclose_radical() -> Result<()> {
    let expr = "<math>
                    <menclose notation='radical'> <mfrac><mn>3</mn><mn>2</mn></mfrac> </menclose>
                </math>";
    test("hu", "ClearSpeak", expr, "négyzetgyök, kezdet: 3 ketted végpont vége")?;
    return Ok(());

}

// AI generated
#[test]
fn simple_speak_menclose_top_bottom() -> Result<()> {
    let expr = "<math>
                    <menclose notation='top bottom'>  <mfrac><mn>3</mn><mn>2</mn></mfrac> </menclose>
                </math>";
    test("hu", "SimpleSpeak", expr, "vonal felső, alsó, kezdet: 3 ketted végpont vége")?;
    return Ok(());

}
