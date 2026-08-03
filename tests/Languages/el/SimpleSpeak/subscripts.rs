use crate::common::*;
use anyhow::Result;

#[test]
fn msub_simple() -> Result<()> {
    let expr = "<math> <msub> <mi>x</mi> <mn>1</mn> </msub> </math>";
    test_prefs("el", "SimpleSpeak", vec![("Verbosity", "Terse")], expr, "x 1")?;
    test_prefs("el", "SimpleSpeak", vec![("Verbosity", "Medium")], expr, "x δείκτης 1")?;
    test_prefs("el", "SimpleSpeak", vec![("Verbosity", "Verbose")], expr, "x δείκτης 1")?;
    return Ok(());

  }

#[test]
fn msub_not_simple() -> Result<()> {
    let expr = "<math> <msub> <mi>x</mi> <mn>1.2</mn> </msub> </math>";
    test_prefs("el", "SimpleSpeak", vec![("Verbosity", "Terse")], expr, "x δείκτης 1.2")?;
    return Ok(());
//theodora. fails --> converts 1.2 to 12.
// The issue is at src/prefs (set_separator, decimal_separator)
  }

#[test]
fn msubsup_not_simple() -> Result<()> {
    let expr = "<math> <msubsup> <mi>x</mi> <mn>1.2</mn> <mn>3</mn></msubsup> </math>";
    test_prefs("el", "SimpleSpeak", vec![("Verbosity", "Terse")], expr, "x δείκτης 1.2, στον κύβο")?;
    return Ok(());
//theodora. fails --> converts 1.2 to 12, check what happens with decimals
// The issue is at src/prefs (set_separator, decimal_separator)
  }

#[test]
fn msub_simple_mi() -> Result<()> {
    let expr = "<math> <msub> <mi>x</mi> <mi>i</mi> </msub> </math>";
    test_prefs("el", "SimpleSpeak", vec![("Verbosity", "Terse")], expr, "x δείκτης i")?;
    test_prefs("el", "SimpleSpeak", vec![("Verbosity", "Verbose")], expr, "x δείκτης i")?;
    return Ok(());

}

#[test]
fn msub_simple_number_follows() -> Result<()> {
    let expr = "<math> <msub> <mi>x</mi> <mn>1</mn> </msub> <msup><mn>10</mn><mn>2</mn></msup> </math>";
    test_prefs("el", "SimpleSpeak", vec![("Verbosity", "Terse")], expr, "x 1, 10 στο τετράγωνο")?;
    test_prefs("el", "SimpleSpeak", vec![("Verbosity", "Verbose")], expr, "x δείκτης 1; 10 στο τετράγωνο")?;
    return Ok(());

}

#[test]
fn msub_simple_non_number_follows() -> Result<()> {
    let expr = "<math> <msubsup> <mi>x</mi> <mn>1</mn> <mn>2</mn> </msubsup> </math>";
    test_prefs("el", "SimpleSpeak", vec![("Verbosity", "Terse")], expr, "x 1, στο τετράγωνο")?;
    test_prefs("el", "SimpleSpeak", vec![("Verbosity", "Verbose")], expr, "x δείκτης 1, στο τετράγωνο")?;
    return Ok(());

}

#[test]
fn msubsup_simple() -> Result<()> {
    let expr = "<math> <msub> <mi>x</mi> <mn>1</mn> </msub> <msup><mi>x</mi>,<mn>2</mn></msup> </math>";
    test_prefs("el", "SimpleSpeak", vec![("Verbosity", "Terse")], expr, "x 1, x στο τετράγωνο")?;
    test_prefs("el", "SimpleSpeak", vec![("Verbosity", "Verbose")], expr, "x δείκτης 1; x στο τετράγωνο")?;
    return Ok(());

}
    