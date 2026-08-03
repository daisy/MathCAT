use crate::common::*;
use anyhow::Result;

#[test]
fn menclose_actuarial() -> Result<()> {
    let expr = "<math>
                    <menclose notation='actuarial'>  <mn>3</mn><mo>+</mo><mn>2</mn><mi>i</mi> </menclose>
                </math>";
    test("fr", "ClearSpeak", expr, "symbole actuariel, contenant 3 plus 2 i fin d'encadrement")?;
    return Ok(());

}

#[test]
fn menclose_box() -> Result<()> {
    let expr = "<math>
                    <menclose notation='box circle'>  <mn>3</mn><mo>+</mo><mn>2</mn><mi>i</mi> </menclose>
                </math>";
    test("fr", "ClearSpeak", expr, "case, rond, contenant 3 plus 2 i fin d'encadrement")?;
    return Ok(());

}

#[test]
fn menclose_left() -> Result<()> {
    let expr = "<math>
                    <menclose notation='left'>  <mfrac><mn>3</mn><mn>2</mn></mfrac> </menclose>
                </math>";
    test("fr", "ClearSpeak", expr, "ligne à gauche, contenant 3 demis fin d'encadrement")?;
    return Ok(());

}

#[test]
fn menclose_right() -> Result<()> {
    let expr = "<math>
                    <menclose notation='right'>  <mfrac><mn>3</mn><mn>2</mn></mfrac> </menclose>
                </math>";
    test("fr", "ClearSpeak", expr, "ligne à droite, contenant 3 demis fin d'encadrement")?;
    return Ok(());

}

#[test]
fn menclose_top_bottom() -> Result<()> {
    let expr = "<math>
                    <menclose notation='top bottom'>  <mfrac><mn>3</mn><mn>2</mn></mfrac> </menclose>
                </math>";
    test("fr", "ClearSpeak", expr, "ligne au dessus, ligne en dessous, contenant 3 demis fin d'encadrement")?;
    return Ok(());

}

#[test]
fn menclose_updiagonalstrike() -> Result<()> {
    let expr = "<math>
                    <menclose notation='updiagonalstrike'>  <mfrac><mn>3</mn><mn>2</mn></mfrac> </menclose>
                </math>";
    test("fr", "ClearSpeak", expr, "diagonale vers le haut, barré, contenant 3 demis fin d'encadrement")?;
    return Ok(());

}

#[test]
fn menclose_downdiagonalstrike() -> Result<()> {
    let expr = "<math>
                    <menclose notation='downdiagonalstrike'>  <mfrac><mn>3</mn><mn>2</mn></mfrac> </menclose>
                </math>";
    test("fr", "ClearSpeak", expr, "diagonale vers le bas, barré, contenant 3 demis fin d'encadrement")?;
    return Ok(());

}

#[test]
fn menclose_cross_out() -> Result<()> {
    let expr = "<math>
                    <menclose notation='updiagonalstrike downdiagonalstrike'>  <mfrac><mn>3</mn><mn>2</mn></mfrac> </menclose>
                </math>";
    test("fr", "ClearSpeak", expr, "x, barré, contenant 3 demis fin d'encadrement")?;
    return Ok(());

}

#[test]
fn menclose_vertical_horizontal_strike() -> Result<()> {
    let expr = "<math>
                    <menclose notation='verticalstrike horizontalstrike'>  <mfrac><mn>3</mn><mn>2</mn></mfrac> </menclose>
                </math>";
    test("fr", "ClearSpeak", expr, "verticale, horizontale, barré, contenant 3 demis fin d'encadrement")?;
    return Ok(());

}

#[test]
fn menclose_leftarrow() -> Result<()> {
    let expr = "<math>
                    <menclose notation='leftarrow'> <mfrac><mn>3</mn><mn>2</mn></mfrac> </menclose>
                </math>";
    test("fr", "ClearSpeak", expr, "flèche vers la gauche, contenant 3 demis fin d'encadrement")?;
    return Ok(());

}

#[test]
fn menclose_right_up_down_arrow() -> Result<()> {
    let expr = "<math>
                    <menclose notation=' rightarrow downarrow  uparrow  '> <mfrac><mn>3</mn><mn>2</mn></mfrac> </menclose>
                </math>";
    test("fr", "ClearSpeak", expr, "flèche vers le haut, flèche vers le bas, flèche vers la droite, contenant 3 demis fin d'encadrement")?;
    return Ok(());

}

#[test]
fn menclose_northeastarrow() -> Result<()> {
    let expr = "<math>
                    <menclose notation='northeastarrow'> <mfrac><mn>3</mn><mn>2</mn></mfrac> </menclose>
                </math>";
    test("fr", "ClearSpeak", expr, "flèche nord-est, contenant 3 demis fin d'encadrement")?;
    return Ok(());

}

#[test]
fn menclose_other_single_arrows() -> Result<()> {
    let expr = "<math>
                    <menclose notation='northwestarrow southwestarrow southeastarrow'> <mfrac><mn>3</mn><mn>2</mn></mfrac> </menclose>
                </math>";
    test("fr", "ClearSpeak", expr, "flèche sud-est, flèche sud-ouest, flèche nord-ouest, contenant 3 demis fin d'encadrement")?;
    return Ok(());

}

#[test]
fn menclose_northwestsoutheastarrow() -> Result<()> {
    let expr = "<math>
                    <menclose notation='northwestsoutheastarrow'> <mfrac><mn>3</mn><mn>2</mn></mfrac> </menclose>
                </math>";
    test("fr", "ClearSpeak", expr, "flèche diagonale vers le bas à double extrémité, contenant 3 demis fin d'encadrement")?;
    return Ok(());

}

#[test]
fn menclose_other_double_arrows() -> Result<()> {
    let expr = "<math>
                    <menclose notation='updownarrow leftrightarrow northeastsouthwestarrow'>  <mfrac><mn>3</mn><mn>2</mn></mfrac> </menclose>
                </math>";
    test("fr", "ClearSpeak", expr, "flèche verticale à double extrémité, flèche horizontale à double extrémité, flèche diagonale vers le haut à double extrémité, contenant 3 demis fin d'encadrement")?;
    return Ok(());

}

#[test]
fn menclose_madrub() -> Result<()> {
    let expr = "<math>
                    <menclose notation='madrub'> <mfrac><mn>3</mn><mn>2</mn></mfrac> </menclose>
                </math>";
    test("fr", "ClearSpeak", expr, "symbole factoriel arabe, contenant 3 demis fin d'encadrement")?;
    return Ok(());

}

#[test]
fn menclose_phasorangle() -> Result<()> {
    let expr = "<math>
                    <menclose notation='phasorangle'> <mfrac><mn>3</mn><mn>2</mn></mfrac> </menclose>
                </math>";
    test("fr", "ClearSpeak", expr, "angle de phaseur, contenant 3 demis fin d'encadrement")?;
    return Ok(());

}

#[test]
fn menclose_circle_phasorangle() -> Result<()> {
    let expr = "<math>
                    <menclose notation='phasorangle circle'> <mfrac><mn>3</mn><mn>2</mn></mfrac> </menclose>
                </math>";
    test("fr", "ClearSpeak", expr, "rond, angle de phaseur, contenant 3 demis fin d'encadrement")?;
    return Ok(());

}

#[test]
fn menclose_longdiv() -> Result<()> {
    let expr = "<math>
                    <menclose notation='longdiv'> <mfrac><mn>3</mn><mn>2</mn></mfrac> </menclose>
                </math>";
    test("fr", "ClearSpeak", expr, "symbole de division longue, contenant 3 demis fin d'encadrement")?;
    return Ok(());

}

#[test]
fn menclose_longdiv_default() -> Result<()> {
    let expr = "<math>
                    <menclose> <mfrac><mn>3</mn><mn>2</mn></mfrac> </menclose>
                </math>";
    test("fr", "ClearSpeak", expr, "symbole de division longue, contenant 3 demis fin d'encadrement")?;
    return Ok(());

}

#[test]
fn menclose_longdiv_empty_string() -> Result<()> {
    let expr = "<math>
                    <menclose notation=''> <mfrac><mn>3</mn><mn>2</mn></mfrac> </menclose>
                </math>";
    test("fr", "ClearSpeak", expr, "symbole de division longue, contenant 3 demis fin d'encadrement")?;
    return Ok(());

}

#[test]
fn menclose_longdiv_whitespace_string() -> Result<()> {
    let expr = "<math>
                    <menclose notation='  '> <mfrac><mn>3</mn><mn>2</mn></mfrac> </menclose>
                </math>";
    test("fr", "ClearSpeak", expr, "symbole de division longue, contenant 3 demis fin d'encadrement")?;
    return Ok(());

}

#[test]
fn menclose_radical() -> Result<()> {
    let expr = "<math>
                    <menclose notation='radical'> <mfrac><mn>3</mn><mn>2</mn></mfrac> </menclose>
                </math>";
    test("fr", "ClearSpeak", expr, "racine carrée, contenant 3 demis fin d'encadrement")?;
    return Ok(());

}

#[test]
fn simple_speak_menclose_top_bottom() -> Result<()> {
    let expr = "<math>
                    <menclose notation='top bottom'>  <mfrac><mn>3</mn><mn>2</mn></mfrac> </menclose>
                </math>";
    test("fr", "SimpleSpeak", expr, "ligne au dessus, ligne en dessous, contenant 3 demis fin d'encadrement")?;
    return Ok(());

}
