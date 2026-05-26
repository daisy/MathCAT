use anyhow::Result;
use crate::common::{test_braille, test_braille_prefs};

#[test]
fn mlongdiv_unsupported_nemeth_en() -> Result<()> {
    let expr = "<math><mlongdiv><mn>3</mn><mn>435</mn><mn>1306</mn></mlongdiv></math>";
    test_braille(
        "Nemeth",
        expr,
        "⠠⠇⠕⠝⠛⠀⠙⠊⠧⠊⠎⠊⠕⠝⠀⠑⠝⠉⠕⠥⠝⠞⠑⠗⠑⠙⠀⠃⠥⠞⠀⠝⠕⠞⠀⠎⠥⠏⠏⠕⠗⠞⠑⠙",
    )?;
    return Ok(());
}

#[test]
fn mstack_unsupported_ueb_en() -> Result<()> {
    let expr = "<math><mstack><mn>123</mn><msrow><mo>+</mo><mn>456</mn></msrow><msline/></mstack></math>";
    test_braille(
        "UEB",
        expr,
        "⠠⠑⠇⠑⠰⠞⠜⠽⠀⠍⠁⠹⠀⠢⠉⠳⠝⠞⠻⠫⠀⠃⠥⠞⠀⠝⠕⠞⠀⠎⠥⠏⠏⠕⠗⠞⠫",
    )?;
    return Ok(());
}

#[test]
fn mlongdiv_unsupported_latex_en() -> Result<()> {
    let expr = "<math><mlongdiv><mn>3</mn><mn>435</mn><mn>1306</mn></mlongdiv></math>";
    test_braille("LaTeX", expr, "Long\\space division\\space encountered\\space but\\space not\\space supported")?;
    return Ok(());
}

#[test]
fn mstack_unsupported_latex_en() -> Result<()> {
    let expr = "<math><mstack><mn>123</mn><msrow><mo>+</mo><mn>456</mn></msrow><msline/></mstack></math>";
    test_braille("LaTeX", expr, "Elementary\\space math\\space encountered\\space but\\space not\\space supported")?;
    return Ok(());
}

#[test]
fn mlongdiv_unsupported_ueb_de() -> Result<()> {
    let expr = "<math><mlongdiv><mn>3</mn><mn>435</mn><mn>1306</mn></mlongdiv></math>";
    test_braille_prefs(
        "UEB",
        vec![("Language", "de")],
        expr,
        "⠠⠇⠁⠝⠛⠑⠀⠠⠙⠊⠧⠊⠎⠊⠕⠝⠀⠛⠑⠋⠥⠝⠙⠢⠂⠀⠁⠃⠻⠀⠝⠊⠡⠞⠀⠥⠝⠞⠻⠌⠄⠸⠡⠭⠼⠚⠚⠚⠚⠰⠋⠉⠄⠞⠵⠞",
    )?;
    return Ok(());
}

#[test]
fn mlongdiv_unsupported_swedish() -> Result<()> {
    let expr = "<math><mlongdiv><mn>3</mn><mn>435</mn><mn>1306</mn></mlongdiv></math>";
    test_braille_prefs(
        "Swedish",
        vec![("Language", "sv")],
        expr,
        "⠠⠇'⠭⠼⠚⠚⠚⠚⠱⠑⠼⠑'⠝⠛⠙⠊⠧⠊⠎⠊⠕⠝⠀⠏'⠭⠼⠚⠚⠚⠚⠱⠑⠼⠑'⠞⠗'⠭⠼⠚⠚⠚⠚⠱⠑⠼⠙'⠋⠋⠁⠙⠀⠍⠑⠝⠀⠎⠞'⠭⠼⠚⠚⠚⠚⠱⠋⠼⠋'⠙⠎⠀⠊⠝⠞⠑",
    )?;
    return Ok(());
}
