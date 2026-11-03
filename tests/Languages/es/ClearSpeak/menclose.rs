use crate::common::*;
use anyhow::Result;

#[test]
fn menclose_actuarial() -> Result<()> {
    let expr = "<math>
                    <menclose notation='actuarial'>  <mn>3</mn><mo>+</mo><mn>2</mn><mi>i</mi> </menclose>
                </math>";
    test("es", "ClearSpeak", expr, "símbolo actuarial, que encierra 3 más 2 i fin de lo encerrado")?;
    return Ok(());

}

#[test]
fn menclose_box() -> Result<()> {
    let expr = "<math>
                    <menclose notation='box circle'>  <mn>3</mn><mo>+</mo><mn>2</mn><mi>i</mi> </menclose>
                </math>";
    test("es", "ClearSpeak", expr, "recuadro, círculo, que encierra 3 más 2 i fin de lo encerrado")?;
    return Ok(());

}

#[test]
fn menclose_left() -> Result<()> {
    let expr = "<math>
                    <menclose notation='left'>  <mfrac><mn>3</mn><mn>2</mn></mfrac> </menclose>
                </math>";
    test("es", "ClearSpeak", expr, "línea izquierda, que encierra 3 medios fin de lo encerrado")?;
    return Ok(());

}

#[test]
fn menclose_right() -> Result<()> {
    let expr = "<math>
                    <menclose notation='right'>  <mfrac><mn>3</mn><mn>2</mn></mfrac> </menclose>
                </math>";
    test("es", "ClearSpeak", expr, "línea derecha, que encierra 3 medios fin de lo encerrado")?;
    return Ok(());

}

#[test]
fn menclose_top_bottom() -> Result<()> {
    let expr = "<math>
                    <menclose notation='top bottom'>  <mfrac><mn>3</mn><mn>2</mn></mfrac> </menclose>
                </math>";
    test("es", "ClearSpeak", expr, "línea arriba, abajo, que encierra 3 medios fin de lo encerrado")?;
    return Ok(());

}

#[test]
fn menclose_updiagonalstrike() -> Result<()> {
    let expr = "<math>
                    <menclose notation='updiagonalstrike'>  <mfrac><mn>3</mn><mn>2</mn></mfrac> </menclose>
                </math>";
    test("es", "ClearSpeak", expr, "diagonal ascendente, tachado, que encierra 3 medios fin de lo encerrado")?;
    return Ok(());

}

#[test]
fn menclose_downdiagonalstrike() -> Result<()> {
    let expr = "<math>
                    <menclose notation='downdiagonalstrike'>  <mfrac><mn>3</mn><mn>2</mn></mfrac> </menclose>
                </math>";
    test("es", "ClearSpeak", expr, "diagonal descendente, tachado, que encierra 3 medios fin de lo encerrado")?;
    return Ok(());

}

#[test]
fn menclose_cross_out() -> Result<()> {
    let expr = "<math>
                    <menclose notation='updiagonalstrike downdiagonalstrike'>  <mfrac><mn>3</mn><mn>2</mn></mfrac> </menclose>
                </math>";
    test("es", "ClearSpeak", expr, "x, tachado, que encierra 3 medios fin de lo encerrado")?;
    return Ok(());

}

#[test]
fn menclose_vertical_horizontal_strike() -> Result<()> {
    let expr = "<math>
                    <menclose notation='verticalstrike horizontalstrike'>  <mfrac><mn>3</mn><mn>2</mn></mfrac> </menclose>
                </math>";
    test("es", "ClearSpeak", expr, "vertical, horizontal, tachado, que encierra 3 medios fin de lo encerrado")?;
    return Ok(());

}

#[test]
fn menclose_leftarrow() -> Result<()> {
    let expr = "<math>
                    <menclose notation='leftarrow'> <mfrac><mn>3</mn><mn>2</mn></mfrac> </menclose>
                </math>";
    test("es", "ClearSpeak", expr, "flecha izquierda, que encierra 3 medios fin de lo encerrado")?;
    return Ok(());

}

#[test]
fn menclose_right_up_down_arrow() -> Result<()> {
    let expr = "<math>
                    <menclose notation=' rightarrow downarrow  uparrow  '> <mfrac><mn>3</mn><mn>2</mn></mfrac> </menclose>
                </math>";
    test("es", "ClearSpeak", expr, "flecha arriba, flecha abajo, flecha derecha, que encierra 3 medios fin de lo encerrado")?;
    return Ok(());

}

#[test]
fn menclose_northeastarrow() -> Result<()> {
    let expr = "<math>
                    <menclose notation='northeastarrow'> <mfrac><mn>3</mn><mn>2</mn></mfrac> </menclose>
                </math>";
    test("es", "ClearSpeak", expr, "flecha noreste, que encierra 3 medios fin de lo encerrado")?;
    return Ok(());

}

#[test]
fn menclose_other_single_arrows() -> Result<()> {
    let expr = "<math>
                    <menclose notation='northwestarrow southwestarrow southeastarrow'> <mfrac><mn>3</mn><mn>2</mn></mfrac> </menclose>
                </math>";
    test("es", "ClearSpeak", expr, "flecha sureste, flecha suroeste, flecha noroeste, que encierra 3 medios fin de lo encerrado")?;
    return Ok(());

}

#[test]
fn menclose_northwestsoutheastarrow() -> Result<()> {
    let expr = "<math>
                    <menclose notation='northwestsoutheastarrow'> <mfrac><mn>3</mn><mn>2</mn></mfrac> </menclose>
                </math>";
    test("es", "ClearSpeak", expr, "flecha diagonal descendente de doble punta, que encierra 3 medios fin de lo encerrado")?;
    return Ok(());

}

#[test]
fn menclose_other_double_arrows() -> Result<()> {
    let expr = "<math>
                    <menclose notation='updownarrow leftrightarrow northeastsouthwestarrow'>  <mfrac><mn>3</mn><mn>2</mn></mfrac> </menclose>
                </math>";
    test("es", "ClearSpeak", expr, "flecha vertical de doble punta, flecha horizontal de doble punta, flecha diagonal ascendente de doble punta, que encierra 3 medios fin de lo encerrado")?;
    return Ok(());

}

#[test]
fn menclose_madrub() -> Result<()> {
    let expr = "<math>
                    <menclose notation='madrub'> <mfrac><mn>3</mn><mn>2</mn></mfrac> </menclose>
                </math>";
    test("es", "ClearSpeak", expr, "símbolo factorial árabe, que encierra 3 medios fin de lo encerrado")?;
    return Ok(());

}

#[test]
fn menclose_phasorangle() -> Result<()> {
    let expr = "<math>
                    <menclose notation='phasorangle'> <mfrac><mn>3</mn><mn>2</mn></mfrac> </menclose>
                </math>";
    test("es", "ClearSpeak", expr, "ángulo fasorial, que encierra 3 medios fin de lo encerrado")?;
    return Ok(());

}

#[test]
fn menclose_circle_phasorangle() -> Result<()> {
    let expr = "<math>
                    <menclose notation='phasorangle circle'> <mfrac><mn>3</mn><mn>2</mn></mfrac> </menclose>
                </math>";
    test("es", "ClearSpeak", expr, "círculo, ángulo fasorial, que encierra 3 medios fin de lo encerrado")?;
    return Ok(());

}

#[test]
fn menclose_longdiv() -> Result<()> {
    let expr = "<math>
                    <menclose notation='longdiv'> <mfrac><mn>3</mn><mn>2</mn></mfrac> </menclose>
                </math>";
    test("es", "ClearSpeak", expr, "símbolo de división larga, que encierra 3 medios fin de lo encerrado")?;
    return Ok(());

}

#[test]
fn menclose_longdiv_default() -> Result<()> {
    let expr = "<math>
                    <menclose> <mfrac><mn>3</mn><mn>2</mn></mfrac> </menclose>
                </math>";
    test("es", "ClearSpeak", expr, "símbolo de división larga, que encierra 3 medios fin de lo encerrado")?;
    return Ok(());

}

#[test]
fn menclose_longdiv_empty_string() -> Result<()> {
    let expr = "<math>
                    <menclose notation=''> <mfrac><mn>3</mn><mn>2</mn></mfrac> </menclose>
                </math>";
    test("es", "ClearSpeak", expr, "símbolo de división larga, que encierra 3 medios fin de lo encerrado")?;
    return Ok(());

}

#[test]
fn menclose_longdiv_whitespace_string() -> Result<()> {
    let expr = "<math>
                    <menclose notation='  '> <mfrac><mn>3</mn><mn>2</mn></mfrac> </menclose>
                </math>";
    test("es", "ClearSpeak", expr, "símbolo de división larga, que encierra 3 medios fin de lo encerrado")?;
    return Ok(());

}

#[test]
fn menclose_radical() -> Result<()> {
    let expr = "<math>
                    <menclose notation='radical'> <mfrac><mn>3</mn><mn>2</mn></mfrac> </menclose>
                </math>";
    test("es", "ClearSpeak", expr, "raíz cuadrada, que encierra 3 medios fin de lo encerrado")?;
    return Ok(());

}

#[test]
fn simple_speak_menclose_top_bottom() -> Result<()> {
    let expr = "<math>
                    <menclose notation='top bottom'>  <mfrac><mn>3</mn><mn>2</mn></mfrac> </menclose>
                </math>";
    test("es", "SimpleSpeak", expr, "línea arriba, abajo, que encierra 3 medios fin de lo encerrado")?;
    return Ok(());

}
