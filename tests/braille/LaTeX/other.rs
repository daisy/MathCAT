// Other tests for LaTeX generation
use crate::common::*;
use anyhow::Result;


#[test]
fn menclose_strikes () -> Result<()> {
    let expr = r#"<math><menclose notation='updiagonalstrike downdiagonalstrike verticalstrike horizontalstrike'>
        <mi>x</mi>
    </menclose></math>"#;
    test_braille("LaTeX", expr, r"\overwrite{\overwrite{\overwrite{\overwrite{x}{\senwline}}{\neswline}}{\vline}}{\hline}")?;
    return Ok(());

}

#[test]
fn menclose_box_and_circle () -> Result<()> {
    let expr = r#"<math><menclose notation='box circle'>
        <mi>x</mi>
    </menclose></math>"#;
    test_braille("LaTeX", expr, r"\boxed{\circle{x}}")?;
    return Ok(());

}

#[test]
fn menclose_sides () -> Result<()> {
    let expr = r#"<math>
        <menclose notation='left right '><mi>x</mi> </menclose>
        <menclose notation='top bottom'><mi>x</mi> </menclose>
    </math>"#;
    test_braille("LaTeX", expr, r"|x|\overline{\underline{x}}")?;
    return Ok(());

}

#[test]
fn menclose_all_sides() -> Result<()> {
    let expr = r#"<math><menclose notation='left right top bottom'>
        <mi>x</mi>
    </menclose></math>"#;
    test_braille("LaTeX", expr, r"\boxed{x}")?;
    return Ok(());

}

#[test]
fn menclose_diagonal_arrows() -> Result<()> {
    let expr = r#"<math><menclose notation='northeastarrow southeastarrow southwestarrow northwestarrow'>
        <mi>x</mi>
    </menclose></math>"#;
    test_braille("LaTeX", expr, r"\overwrite{\overwrite{\overwrite{\overwrite{x}{\nearrow}}{\searrow}}{\swarrow}}{\nwarrow}")?;
    return Ok(());

}

#[test]
fn menclose_double_arrows() -> Result<()> {
    // extra spaces are deliberate -- they shouldn't make a difference
    let expr = r#"<math><menclose notation='updownarrow leftrightarrow northeastsouthwestarrow  northwestsoutheastarrow '>
        <mi>x</mi>
    </menclose></math>"#;
    test_braille("LaTeX", expr, r"\overwrite{\overwrite{\overwrite{\overwrite{x}{\updownarrow}}{\longleftrightarrow}}{\neswarrow}}{\nwsearrow}")?;
    return Ok(());

}

#[test]
fn menclose_horiz_and_vert_arrows() -> Result<()> {
    let expr = r#"<math><menclose notation='uparrow downarrow leftarrow rightarrow'>
        <mi>x</mi>
    </menclose></math>"#;
    test_braille("LaTeX", expr, r"\overwrite{\overwrite{\overwrite{\overwrite{x}{\longleftarrow}}{\longrightarrow}}{\uparrow}}{\downarrow}")?;
    return Ok(());

}
