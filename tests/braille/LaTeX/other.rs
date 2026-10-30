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

#[test]
fn dots() -> Result<()> {
  let expr = "<math>
         <mover><mi>x</mi><mo>.</mo></mover><mo>+</mo>
         <mover><mi>x</mi><mo>..</mo></mover><mo>+</mo>
         <mover><mi>x</mi><mo>...</mo></mover>
    </math>";
    test_braille("LaTeX",expr, r"\dot{x} + \ddot{x} + \overset{...}{x}")?;
    return Ok(());
}

#[test]
fn gather_lines_keep_begin_end() -> Result<()> {
    // A bare table of equations is a gather. The intent is what names the environment.
    let expr = r#"<math><mtable intent=":lines">
        <mtr><mtd><mi>a</mi><mo>+</mo><mi>b</mi><mo>=</mo><mi>c</mi></mtd></mtr>
        <mtr><mtd><msup><mi>x</mi><mn>2</mn></msup><mo>+</mo><msup><mi>y</mi><mn>2</mn></msup><mo>+</mo><msup><mi>z</mi><mn>2</mn></msup><mo>=</mo><msup><mi>r</mi><mn>2</mn></msup></mtd></mtr>
        <mtr><mtd><mi>E</mi><mo>=</mo><mi>m</mi><msup><mi>c</mi><mn>2</mn></msup></mtd></mtr>
    </mtable></math>"#;
    test_braille(
        "LaTeX",
        expr,
        r"\begin{gather} a +b =c \\ x^2 +y^2 +z^2 =r^2 \\ E =mc^2 \end{gather}",
    )?;
    let matrix = r#"<math><mtable intent=":matrix"><mtr><mtd><mi>a</mi></mtd></mtr></mtable></math>"#;
    test_braille("LaTeX", matrix, r"\begin{matrix} a \end{matrix}")?;
    let pmatrix = r#"<math><mrow><mo>(</mo><mtable intent=":matrix"><mtr><mtd><mi>a</mi></mtd></mtr></mtable><mo>)</mo></mrow></math>"#;
    test_braille("LaTeX", pmatrix, r"\begin{pmatrix} a \end{pmatrix}")?;
    let cases = r#"<math><mrow><mo>{</mo><mtable intent=":piecewise"><mtr><mtd><mi>a</mi></mtd></mtr><mtr><mtd><mi>b</mi></mtd></mtr></mtable></mrow></math>"#;
    test_braille("LaTeX", cases, r"\begin{cases} a \\ b \end{cases}")?;
    return Ok(());
}
