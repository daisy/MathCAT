use crate::common::*;
use anyhow::Result;

#[test]
fn msub_simple() -> Result<()> {
    let expr = "<math> <msub> <mi>x</mi> <mn>1</mn> </msub> </math>";
    test_prefs("fr", "SimpleSpeak", vec![("Verbosity", "Terse")], expr, "x 1")?;
    test_prefs("fr", "SimpleSpeak", vec![("Verbosity", "Medium")], expr, "x indice 1")?;
    test_prefs("fr", "SimpleSpeak", vec![("Verbosity", "Verbose")], expr, "x indice 1")?;
    return Ok(());

  }

#[test]
fn msub_not_simple() -> Result<()> {
    let expr = "<math> <msub> <mi>x</mi> <mn>1,2</mn> </msub> </math>";
    test_prefs("fr", "SimpleSpeak", vec![("Verbosity", "Terse")], expr, "x indice 1,2")?;
    return Ok(());

  }

#[test]
fn msubsup_not_simple() -> Result<()> {
    let expr = "<math> <msubsup> <mi>x</mi> <mn>1,2</mn> <mn>3</mn></msubsup> </math>";
    test_prefs("fr", "SimpleSpeak", vec![("Verbosity", "Terse")], expr, "x indice 1,2, au cube")?;
    return Ok(());

  }

#[test]
fn msub_simple_mi() -> Result<()> {
    let expr = "<math> <msub> <mi>x</mi> <mi>i</mi> </msub> </math>";
    test_prefs("fr", "SimpleSpeak", vec![("Verbosity", "Terse")], expr, "x indice i")?;
    test_prefs("fr", "SimpleSpeak", vec![("Verbosity", "Verbose")], expr, "x indice i")?;
    return Ok(());

}

#[test]
fn msub_simple_number_follows() -> Result<()> {
    let expr = "<math> <msub> <mi>x</mi> <mn>1</mn> </msub> <msup><mn>10</mn><mn>2</mn></msup> </math>";
    test_prefs("fr", "SimpleSpeak", vec![("Verbosity", "Terse")], expr, "x 1, 10 au carré")?;
    test_prefs("fr", "SimpleSpeak", vec![("Verbosity", "Verbose")], expr, "x indice 1, 10 au carré")?;
    return Ok(());

}

#[test]
fn msub_simple_non_number_follows() -> Result<()> {
    let expr = "<math> <msubsup> <mi>x</mi> <mn>1</mn> <mn>2</mn> </msubsup> </math>";
    test_prefs("fr", "SimpleSpeak", vec![("Verbosity", "Terse")], expr, "x 1, au carré")?;
    test_prefs("fr", "SimpleSpeak", vec![("Verbosity", "Verbose")], expr, "x indice 1, au carré")?;
    return Ok(());

}

#[test]
fn msubsup_simple() -> Result<()> {
    let expr = "<math> <msub> <mi>x</mi> <mn>1</mn> </msub> <msup><mi>x</mi>,<mn>2</mn></msup> </math>";
    test_prefs("fr", "SimpleSpeak", vec![("Verbosity", "Terse")], expr, "x 1, x au carré")?;
    test_prefs("fr", "SimpleSpeak", vec![("Verbosity", "Verbose")], expr, "x indice 1, x au carré")?;
    return Ok(());

}