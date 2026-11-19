use crate::common::*;
use anyhow::Result;

#[test]
fn menclose_actuarial() -> Result<()> {
    let expr = "<math>
                    <menclose notation='actuarial'>  <mn>3</mn><mo>+</mo><mn>2</mn><mi>i</mi> </menclose>
                </math>";
    test("pl", "ClearSpeak", expr, "symbol aktuarialny, obejmujący 3 plus 2 i koniec obramowania")?;
    return Ok(());

}

#[test]
fn menclose_box() -> Result<()> {
    let expr = "<math>
                    <menclose notation='box circle'>  <mn>3</mn><mo>+</mo><mn>2</mn><mi>i</mi> </menclose>
                </math>";
    test("pl", "ClearSpeak", expr, "ramka, okrąg, obejmujący 3 plus 2 i koniec obramowania")?;
    return Ok(());

}

#[test]
fn menclose_left() -> Result<()> {
    let expr = "<math>
                    <menclose notation='left'>  <mfrac><mn>3</mn><mn>2</mn></mfrac> </menclose>
                </math>";
    test("pl", "ClearSpeak", expr, "linia na lewo, obejmujący 3 drugie koniec obramowania")?;
    return Ok(());

}

#[test]
fn menclose_right() -> Result<()> {
    let expr = "<math>
                    <menclose notation='right'>  <mfrac><mn>3</mn><mn>2</mn></mfrac> </menclose>
                </math>";
    test("pl", "ClearSpeak", expr, "linia na prawo, obejmujący 3 drugie koniec obramowania")?;
    return Ok(());

}

#[test]
fn menclose_top_bottom() -> Result<()> {
    let expr = "<math>
                    <menclose notation='top bottom'>  <mfrac><mn>3</mn><mn>2</mn></mfrac> </menclose>
                </math>";
    test("pl", "ClearSpeak", expr, "linia na góra, dół, obejmujący 3 drugie koniec obramowania")?;
    return Ok(());

}

#[test]
fn menclose_updiagonalstrike() -> Result<()> {
    let expr = "<math>
                    <menclose notation='updiagonalstrike'>  <mfrac><mn>3</mn><mn>2</mn></mfrac> </menclose>
                </math>";
    test("pl", "ClearSpeak", expr, "ukośnie w górę, krzyż na zewnątrz, obejmujący 3 drugie koniec obramowania")?;
    return Ok(());

}

#[test]
fn menclose_downdiagonalstrike() -> Result<()> {
    let expr = "<math>
                    <menclose notation='downdiagonalstrike'>  <mfrac><mn>3</mn><mn>2</mn></mfrac> </menclose>
                </math>";
    test("pl", "ClearSpeak", expr, "ukośnie w dół, krzyż na zewnątrz, obejmujący 3 drugie koniec obramowania")?;
    return Ok(());

}

#[test]
fn menclose_cross_out() -> Result<()> {
    let expr = "<math>
                    <menclose notation='updiagonalstrike downdiagonalstrike'>  <mfrac><mn>3</mn><mn>2</mn></mfrac> </menclose>
                </math>";
    test("pl", "ClearSpeak", expr, "x, krzyż na zewnątrz, obejmujący 3 drugie koniec obramowania")?;
    return Ok(());

}

#[test]
fn menclose_vertical_horizontal_strike() -> Result<()> {
    let expr = "<math>
                    <menclose notation='verticalstrike horizontalstrike'>  <mfrac><mn>3</mn><mn>2</mn></mfrac> </menclose>
                </math>";
    test("pl", "ClearSpeak", expr, "pionowy, poziomy, krzyż na zewnątrz, obejmujący 3 drugie koniec obramowania")?;
    return Ok(());

}

#[test]
fn menclose_leftarrow() -> Result<()> {
    let expr = "<math>
                    <menclose notation='leftarrow'> <mfrac><mn>3</mn><mn>2</mn></mfrac> </menclose>
                </math>";
    test("pl", "ClearSpeak", expr, "strzałka w lewo, obejmujący 3 drugie koniec obramowania")?;
    return Ok(());

}

#[test]
fn menclose_right_up_down_arrow() -> Result<()> {
    let expr = "<math>
                    <menclose notation=' rightarrow downarrow  uparrow  '> <mfrac><mn>3</mn><mn>2</mn></mfrac> </menclose>
                </math>";
    test("pl", "ClearSpeak", expr, "strzałka w górę, strzałka w dół, strzałka w prawo, obejmujący 3 drugie koniec obramowania")?;
    return Ok(());

}

#[test]
fn menclose_northeastarrow() -> Result<()> {
    let expr = "<math>
                    <menclose notation='northeastarrow'> <mfrac><mn>3</mn><mn>2</mn></mfrac> </menclose>
                </math>";
    test("pl", "ClearSpeak", expr, "strzałka na północny wschód, obejmujący 3 drugie koniec obramowania")?;
    return Ok(());

}

#[test]
fn menclose_other_single_arrows() -> Result<()> {
    let expr = "<math>
                    <menclose notation='northwestarrow southwestarrow southeastarrow'> <mfrac><mn>3</mn><mn>2</mn></mfrac> </menclose>
                </math>";
    test("pl", "ClearSpeak", expr, "strzałka na południowy wschód, strzałka na południowy zachód, strzałka na północny zachód, obejmujący 3 drugie koniec obramowania")?;
    return Ok(());

}

#[test]
fn menclose_northwestsoutheastarrow() -> Result<()> {
    let expr = "<math>
                    <menclose notation='northwestsoutheastarrow'> <mfrac><mn>3</mn><mn>2</mn></mfrac> </menclose>
                </math>";
    test("pl", "ClearSpeak", expr, "strzałka dwukierunkowa ukośna w dół, obejmujący 3 drugie koniec obramowania")?;
    return Ok(());

}

#[test]
fn menclose_other_double_arrows() -> Result<()> {
    let expr = "<math>
                    <menclose notation='updownarrow leftrightarrow northeastsouthwestarrow'>  <mfrac><mn>3</mn><mn>2</mn></mfrac> </menclose>
                </math>";
    test("pl", "ClearSpeak", expr, "strzałka dwukierunkowa pionowa, strzałka dwukierunkowa pozioma, strzałka dwukierunkowa ukośna w górę, obejmujący 3 drugie koniec obramowania")?;
    return Ok(());

}

#[test]
fn menclose_madrub() -> Result<()> {
    let expr = "<math>
                    <menclose notation='madrub'> <mfrac><mn>3</mn><mn>2</mn></mfrac> </menclose>
                </math>";
    test("pl", "ClearSpeak", expr, "arabski symbol silni, obejmujący 3 drugie koniec obramowania")?;
    return Ok(());

}

#[test]
fn menclose_phasorangle() -> Result<()> {
    let expr = "<math>
                    <menclose notation='phasorangle'> <mfrac><mn>3</mn><mn>2</mn></mfrac> </menclose>
                </math>";
    test("pl", "ClearSpeak", expr, "kąt fazowy, obejmujący 3 drugie koniec obramowania")?;
    return Ok(());

}

#[test]
fn menclose_circle_phasorangle() -> Result<()> {
    let expr = "<math>
                    <menclose notation='phasorangle circle'> <mfrac><mn>3</mn><mn>2</mn></mfrac> </menclose>
                </math>";
    test("pl", "ClearSpeak", expr, "okrąg, kąt fazowy, obejmujący 3 drugie koniec obramowania")?;
    return Ok(());

}

#[test]
fn menclose_longdiv() -> Result<()> {
    let expr = "<math>
                    <menclose notation='longdiv'> <mfrac><mn>3</mn><mn>2</mn></mfrac> </menclose>
                </math>";
    test("pl", "ClearSpeak", expr, "znak dzielenia pisemnego, obejmujący 3 drugie koniec obramowania")?;
    return Ok(());

}

#[test]
fn menclose_longdiv_default() -> Result<()> {
    let expr = "<math>
                    <menclose> <mfrac><mn>3</mn><mn>2</mn></mfrac> </menclose>
                </math>";
    test("pl", "ClearSpeak", expr, "znak dzielenia pisemnego, obejmujący 3 drugie koniec obramowania")?;
    return Ok(());

}

#[test]
fn menclose_longdiv_empty_string() -> Result<()> {
    let expr = "<math>
                    <menclose notation=''> <mfrac><mn>3</mn><mn>2</mn></mfrac> </menclose>
                </math>";
    test("pl", "ClearSpeak", expr, "znak dzielenia pisemnego, obejmujący 3 drugie koniec obramowania")?;
    return Ok(());

}

#[test]
fn menclose_longdiv_whitespace_string() -> Result<()> {
    let expr = "<math>
                    <menclose notation='  '> <mfrac><mn>3</mn><mn>2</mn></mfrac> </menclose>
                </math>";
    test("pl", "ClearSpeak", expr, "znak dzielenia pisemnego, obejmujący 3 drugie koniec obramowania")?;
    return Ok(());

}

#[test]
fn menclose_radical() -> Result<()> {
    let expr = "<math>
                    <menclose notation='radical'> <mfrac><mn>3</mn><mn>2</mn></mfrac> </menclose>
                </math>";
    test("pl", "ClearSpeak", expr, "pierwiastek kwadratowy, obejmujący 3 drugie koniec obramowania")?;
    return Ok(());

}

#[test]
fn simple_speak_menclose_top_bottom() -> Result<()> {
    let expr = "<math>
                    <menclose notation='top bottom'>  <mfrac><mn>3</mn><mn>2</mn></mfrac> </menclose>
                </math>";
    test("pl", "SimpleSpeak", expr, "linia na góra, dół, obejmujący 3 drugie koniec obramowania")?;
    return Ok(());

}
