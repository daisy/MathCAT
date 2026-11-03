use crate::common::*;
use anyhow::Result;

// REVISAR (inglés: actuarial): «símbolo actuarial, agregando 3 más 2 i agregación final»
#[test]
#[ignore = "lectura en español pendiente de revisión"]
fn menclose_actuarial() -> Result<()> {
    let expr = "<math>
                    <menclose notation='actuarial'>  <mn>3</mn><mo>+</mo><mn>2</mn><mi>i</mi> </menclose>
                </math>";
    test("es", "ClearSpeak", expr, "símbolo actuarial, agregando 3 más 2 i agregación final")?;
    return Ok(());

}

#[test]
fn menclose_box() -> Result<()> {
    let expr = "<math>
                    <menclose notation='box circle'>  <mn>3</mn><mo>+</mo><mn>2</mn><mi>i</mi> </menclose>
                </math>";
    test("es", "ClearSpeak", expr, "celda, círculo, agregando 3 más 2 i agregación final")?;
    return Ok(());

}

// REVISAR (mitad): «línea sobre izquierda, agregando 3 mitades agregación final»
#[test]
#[ignore = "lectura en español pendiente de revisión"]
fn menclose_left() -> Result<()> {
    let expr = "<math>
                    <menclose notation='left'>  <mfrac><mn>3</mn><mn>2</mn></mfrac> </menclose>
                </math>";
    test("es", "ClearSpeak", expr, "línea sobre izquierda, agregando 3 mitades agregación final")?;
    return Ok(());

}

// REVISAR (mitad): «línea sobre derecha, agregando 3 mitades agregación final»
#[test]
#[ignore = "lectura en español pendiente de revisión"]
fn menclose_right() -> Result<()> {
    let expr = "<math>
                    <menclose notation='right'>  <mfrac><mn>3</mn><mn>2</mn></mfrac> </menclose>
                </math>";
    test("es", "ClearSpeak", expr, "línea sobre derecha, agregando 3 mitades agregación final")?;
    return Ok(());

}

// REVISAR (mitad): «línea sobre superior, inferior, agregando 3 mitades agregación final»
#[test]
#[ignore = "lectura en español pendiente de revisión"]
fn menclose_top_bottom() -> Result<()> {
    let expr = "<math>
                    <menclose notation='top bottom'>  <mfrac><mn>3</mn><mn>2</mn></mfrac> </menclose>
                </math>";
    test("es", "ClearSpeak", expr, "línea sobre superior, inferior, agregando 3 mitades agregación final")?;
    return Ok(());

}

// REVISAR (mitad): «diagonal superior, tachar, agregando 3 mitades agregación final»
#[test]
#[ignore = "lectura en español pendiente de revisión"]
fn menclose_updiagonalstrike() -> Result<()> {
    let expr = "<math>
                    <menclose notation='updiagonalstrike'>  <mfrac><mn>3</mn><mn>2</mn></mfrac> </menclose>
                </math>";
    test("es", "ClearSpeak", expr, "diagonal superior, tachar, agregando 3 mitades agregación final")?;
    return Ok(());

}

// REVISAR (mitad): «diagonal inferior, tachar, agregando 3 mitades agregación final»
#[test]
#[ignore = "lectura en español pendiente de revisión"]
fn menclose_downdiagonalstrike() -> Result<()> {
    let expr = "<math>
                    <menclose notation='downdiagonalstrike'>  <mfrac><mn>3</mn><mn>2</mn></mfrac> </menclose>
                </math>";
    test("es", "ClearSpeak", expr, "diagonal inferior, tachar, agregando 3 mitades agregación final")?;
    return Ok(());

}

// REVISAR (mitad): «x, tachar, agregando 3 mitades agregación final»
#[test]
#[ignore = "lectura en español pendiente de revisión"]
fn menclose_cross_out() -> Result<()> {
    let expr = "<math>
                    <menclose notation='updiagonalstrike downdiagonalstrike'>  <mfrac><mn>3</mn><mn>2</mn></mfrac> </menclose>
                </math>";
    test("es", "ClearSpeak", expr, "x, tachar, agregando 3 mitades agregación final")?;
    return Ok(());

}

// REVISAR (mitad): «vertical, horizontal, tachar, agregando 3 mitades agregación final»
#[test]
#[ignore = "lectura en español pendiente de revisión"]
fn menclose_vertical_horizontal_strike() -> Result<()> {
    let expr = "<math>
                    <menclose notation='verticalstrike horizontalstrike'>  <mfrac><mn>3</mn><mn>2</mn></mfrac> </menclose>
                </math>";
    test("es", "ClearSpeak", expr, "vertical, horizontal, tachar, agregando 3 mitades agregación final")?;
    return Ok(());

}

// REVISAR (mitad): «flecha izquierda, agregando 3 mitades agregación final»
#[test]
#[ignore = "lectura en español pendiente de revisión"]
fn menclose_leftarrow() -> Result<()> {
    let expr = "<math>
                    <menclose notation='leftarrow'> <mfrac><mn>3</mn><mn>2</mn></mfrac> </menclose>
                </math>";
    test("es", "ClearSpeak", expr, "flecha izquierda, agregando 3 mitades agregación final")?;
    return Ok(());

}

// REVISAR (mitad): «flecha arriba, flecha abajo, flecha derecha, agregando 3 mitades agregación final»
#[test]
#[ignore = "lectura en español pendiente de revisión"]
fn menclose_right_up_down_arrow() -> Result<()> {
    let expr = "<math>
                    <menclose notation=' rightarrow downarrow  uparrow  '> <mfrac><mn>3</mn><mn>2</mn></mfrac> </menclose>
                </math>";
    test("es", "ClearSpeak", expr, "flecha arriba, flecha abajo, flecha derecha, agregando 3 mitades agregación final")?;
    return Ok(());

}

// REVISAR (mitad): «flecha hacia el nordeste, agregando 3 mitades agregación final»
#[test]
#[ignore = "lectura en español pendiente de revisión"]
fn menclose_northeastarrow() -> Result<()> {
    let expr = "<math>
                    <menclose notation='northeastarrow'> <mfrac><mn>3</mn><mn>2</mn></mfrac> </menclose>
                </math>";
    test("es", "ClearSpeak", expr, "flecha hacia el nordeste, agregando 3 mitades agregación final")?;
    return Ok(());

}

// REVISAR (mitad): «flecha hacia el sudeste, flecha hacia el sudoeste, flecha hacia el noroeste, agregando 3 mitades agregación final»
#[test]
#[ignore = "lectura en español pendiente de revisión"]
fn menclose_other_single_arrows() -> Result<()> {
    let expr = "<math>
                    <menclose notation='northwestarrow southwestarrow southeastarrow'> <mfrac><mn>3</mn><mn>2</mn></mfrac> </menclose>
                </math>";
    test("es", "ClearSpeak", expr, "flecha hacia el sudeste, flecha hacia el sudoeste, flecha hacia el noroeste, agregando 3 mitades agregación final")?;
    return Ok(());

}

// REVISAR (mitad): «flecha diagonal inferior de doble extremo, agregando 3 mitades agregación final»
#[test]
#[ignore = "lectura en español pendiente de revisión"]
fn menclose_northwestsoutheastarrow() -> Result<()> {
    let expr = "<math>
                    <menclose notation='northwestsoutheastarrow'> <mfrac><mn>3</mn><mn>2</mn></mfrac> </menclose>
                </math>";
    test("es", "ClearSpeak", expr, "flecha diagonal inferior de doble extremo, agregando 3 mitades agregación final")?;
    return Ok(());

}

// REVISAR (mitad): «flecha vertical con doble extremo, flecha horizontal de doble extremo, flecha diagonal superior de doble extremo, agregando 3 mitades agregación final»
#[test]
#[ignore = "lectura en español pendiente de revisión"]
fn menclose_other_double_arrows() -> Result<()> {
    let expr = "<math>
                    <menclose notation='updownarrow leftrightarrow northeastsouthwestarrow'>  <mfrac><mn>3</mn><mn>2</mn></mfrac> </menclose>
                </math>";
    test("es", "ClearSpeak", expr, "flecha vertical con doble extremo, flecha horizontal de doble extremo, flecha diagonal superior de doble extremo, agregando 3 mitades agregación final")?;
    return Ok(());

}

// REVISAR (mitad): «símbolo factorial arábigo, agregando 3 mitades agregación final»
#[test]
#[ignore = "lectura en español pendiente de revisión"]
fn menclose_madrub() -> Result<()> {
    let expr = "<math>
                    <menclose notation='madrub'> <mfrac><mn>3</mn><mn>2</mn></mfrac> </menclose>
                </math>";
    test("es", "ClearSpeak", expr, "símbolo factorial arábigo, agregando 3 mitades agregación final")?;
    return Ok(());

}

// REVISAR (mitad): «ángulo fasorial, agregando 3 mitades agregación final»
#[test]
#[ignore = "lectura en español pendiente de revisión"]
fn menclose_phasorangle() -> Result<()> {
    let expr = "<math>
                    <menclose notation='phasorangle'> <mfrac><mn>3</mn><mn>2</mn></mfrac> </menclose>
                </math>";
    test("es", "ClearSpeak", expr, "ángulo fasorial, agregando 3 mitades agregación final")?;
    return Ok(());

}

// REVISAR (mitad): «círculo, ángulo fasorial, agregando 3 mitades agregación final»
#[test]
#[ignore = "lectura en español pendiente de revisión"]
fn menclose_circle_phasorangle() -> Result<()> {
    let expr = "<math>
                    <menclose notation='phasorangle circle'> <mfrac><mn>3</mn><mn>2</mn></mfrac> </menclose>
                </math>";
    test("es", "ClearSpeak", expr, "círculo, ángulo fasorial, agregando 3 mitades agregación final")?;
    return Ok(());

}

// REVISAR (mitad): «símbolo de larga partición, agregando 3 mitades agregación final»
#[test]
#[ignore = "lectura en español pendiente de revisión"]
fn menclose_longdiv() -> Result<()> {
    let expr = "<math>
                    <menclose notation='longdiv'> <mfrac><mn>3</mn><mn>2</mn></mfrac> </menclose>
                </math>";
    test("es", "ClearSpeak", expr, "símbolo de larga partición, agregando 3 mitades agregación final")?;
    return Ok(());

}

// REVISAR (mitad): «símbolo de larga partición, agregando 3 mitades agregación final»
#[test]
#[ignore = "lectura en español pendiente de revisión"]
fn menclose_longdiv_default() -> Result<()> {
    let expr = "<math>
                    <menclose> <mfrac><mn>3</mn><mn>2</mn></mfrac> </menclose>
                </math>";
    test("es", "ClearSpeak", expr, "símbolo de larga partición, agregando 3 mitades agregación final")?;
    return Ok(());

}

// REVISAR (mitad): «símbolo de larga partición, agregando 3 mitades agregación final»
#[test]
#[ignore = "lectura en español pendiente de revisión"]
fn menclose_longdiv_empty_string() -> Result<()> {
    let expr = "<math>
                    <menclose notation=''> <mfrac><mn>3</mn><mn>2</mn></mfrac> </menclose>
                </math>";
    test("es", "ClearSpeak", expr, "símbolo de larga partición, agregando 3 mitades agregación final")?;
    return Ok(());

}

// REVISAR (mitad): «símbolo de larga partición, agregando 3 mitades agregación final»
#[test]
#[ignore = "lectura en español pendiente de revisión"]
fn menclose_longdiv_whitespace_string() -> Result<()> {
    let expr = "<math>
                    <menclose notation='  '> <mfrac><mn>3</mn><mn>2</mn></mfrac> </menclose>
                </math>";
    test("es", "ClearSpeak", expr, "símbolo de larga partición, agregando 3 mitades agregación final")?;
    return Ok(());

}

// REVISAR (mitad): «raíz cuadrada, agregando 3 mitades agregación final»
#[test]
#[ignore = "lectura en español pendiente de revisión"]
fn menclose_radical() -> Result<()> {
    let expr = "<math>
                    <menclose notation='radical'> <mfrac><mn>3</mn><mn>2</mn></mfrac> </menclose>
                </math>";
    test("es", "ClearSpeak", expr, "raíz cuadrada, agregando 3 mitades agregación final")?;
    return Ok(());

}

// REVISAR (mitad): «línea sobre superior, inferior, agregando 3 mitades agregación final»
#[test]
#[ignore = "lectura en español pendiente de revisión"]
fn simple_speak_menclose_top_bottom() -> Result<()> {
    let expr = "<math>
                    <menclose notation='top bottom'>  <mfrac><mn>3</mn><mn>2</mn></mfrac> </menclose>
                </math>";
    test("es", "SimpleSpeak", expr, "línea sobre superior, inferior, agregando 3 mitades agregación final")?;
    return Ok(());

}
