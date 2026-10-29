//! U+FFFD uses a code's omission sign, or an empty cell when that code has none.
use crate::common::*;
use anyhow::Result;

#[test]
fn missing_char_uses_omission_or_empty_cell() -> Result<()> {
    // The replacement character sits between two letters so a blank cell is not
    // trimmed off the ends of the braille.
    let expr = "<math><mi>a</mi><mtext>&#xFFFD;</mtext><mi>b</mi></math>";
    // UEB, French, and Swedish use ⠬. Nemeth and Vietnam use ⠿. CMU uses ⠰⠤⠆.
    test_braille("UEB", expr, "⠁⠬⠃")?;
    test_braille("French", expr, "⠁⠬⠃")?;
    test_braille("Swedish", expr, "⠁⠬⠃")?;
    test_braille("Nemeth", expr, "⠁⠿⠃")?;
    test_braille("Vietnam", expr, "⠁⠿⠃")?;
    test_braille("CMU", expr, "⠁⠰⠤⠆⠃")?;
    // Russian, LaTeX, and ASCIIMath have no omission sign: an empty cell
    // (trimmed away entirely in the text codes).
    test_braille("Russian", expr, "⠠⠁⠀⠃")?;
    test_braille("LaTeX", expr, "ab")?;
    test_braille("ASCIIMath", expr, "ab")?;
    Ok(())
}
