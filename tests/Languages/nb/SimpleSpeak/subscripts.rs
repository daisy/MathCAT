use crate::common::*;
use anyhow::Result;

#[test]
fn msub_simple() -> Result<()> {
    let expr = "<math> <msub> <mi>x</mi> <mn>1</mn> </msub> </math>";
    test_prefs("nb", "SimpleSpeak", vec![("Verbosity", "Terse")], expr, "x 1")?;
    test_prefs("nb", "SimpleSpeak", vec![("Verbosity", "Medium")], expr, "x, senket 1")?;
    test_prefs("nb", "SimpleSpeak", vec![("Verbosity", "Verbose")], expr, "x, senket 1")?;
    return Ok(());

  }

#[test]
fn msub_not_simple() -> Result<()> {
    let expr = "<math> <msub> <mi>x</mi> <mn>1,2</mn> </msub> </math>";
    test_prefs("nb", "SimpleSpeak", vec![("Verbosity", "Terse"), ("DecimalSeparators", ","), ("BlockSeparators", ".")], expr, "x, senket 1,2")?;
    return Ok(());

  }

#[test]
fn msubsup_not_simple() -> Result<()> {
    let expr = "<math> <msubsup> <mi>x</mi> <mn>1,2</mn> <mn>3</mn></msubsup> </math>";
    test_prefs("nb", "SimpleSpeak", vec![("Verbosity", "Terse"), ("DecimalSeparators", ","), ("BlockSeparators", ".")], expr, "x, senket 1,2, i tredje")?;
    return Ok(());

  }

#[test]
fn msub_simple_mi() -> Result<()> {
    let expr = "<math> <msub> <mi>x</mi> <mi>i</mi> </msub> </math>";
    test_prefs("nb", "SimpleSpeak", vec![("Verbosity", "Terse")], expr, "x, senket i")?;
    test_prefs("nb", "SimpleSpeak", vec![("Verbosity", "Verbose")], expr, "x, senket i")?;
    return Ok(());

}

#[test]
fn msub_simple_number_follows() -> Result<()> {
    let expr = "<math> <msub> <mi>x</mi> <mn>1</mn> </msub> <msup><mn>10</mn><mn>2</mn></msup> </math>";
    test_prefs("nb", "SimpleSpeak", vec![("Verbosity", "Terse")], expr, "x 1, 10 i andre")?;
    test_prefs("nb", "SimpleSpeak", vec![("Verbosity", "Verbose")], expr, "x, senket 1; 10 i andre")?;
    return Ok(());

}

#[test]
fn msub_simple_non_number_follows() -> Result<()> {
    let expr = "<math> <msubsup> <mi>x</mi> <mn>1</mn> <mn>2</mn> </msubsup> </math>";
    test_prefs("nb", "SimpleSpeak", vec![("Verbosity", "Terse")], expr, "x 1, i andre")?;
    test_prefs("nb", "SimpleSpeak", vec![("Verbosity", "Verbose")], expr, "x, senket 1, i andre")?;
    return Ok(());

}

#[test]
fn msubsup_simple() -> Result<()> {
    let expr = "<math> <msub> <mi>x</mi> <mn>1</mn> </msub> <msup><mi>x</mi>,<mn>2</mn></msup> </math>";
    test_prefs("nb", "SimpleSpeak", vec![("Verbosity", "Terse")], expr, "x 1, x i andre")?;
    test_prefs("nb", "SimpleSpeak", vec![("Verbosity", "Verbose")], expr, "x, senket 1; x i andre")?;
    return Ok(());

}