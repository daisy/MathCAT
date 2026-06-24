use crate::common::*;
use anyhow::Result;

// AI generated
#[test]
fn msub_simple() -> Result<()> {
    let expr = "<math> <msub> <mi>x</mi> <mn>1</mn> </msub> </math>";
    test_prefs("hu", "SimpleSpeak", vec![("Verbosity", "Terse")], expr, "x 1")?;
    test_prefs("hu", "SimpleSpeak", vec![("Verbosity", "Medium")], expr, "x alsó index 1")?;
    test_prefs("hu", "SimpleSpeak", vec![("Verbosity", "Verbose")], expr, "x alsó index 1")?;
    return Ok(());

  }

// AI generated
#[test]
fn msub_not_simple() -> Result<()> {
    let expr = "<math> <msub> <mi>x</mi> <mn>1.2</mn> </msub> </math>";
    test_prefs("hu", "SimpleSpeak", vec![("Verbosity", "Terse")], expr, "x alsó index 12")?;
    return Ok(());

  }

// AI generated
#[test]
fn msubsup_not_simple() -> Result<()> {
    let expr = "<math> <msubsup> <mi>x</mi> <mn>1.2</mn> <mn>3</mn></msubsup> </math>";
    test_prefs("hu", "SimpleSpeak", vec![("Verbosity", "Terse")], expr, "x alsó index 12, a köbön")?;
    return Ok(());

  }

// AI generated
#[test]
fn msub_simple_mi() -> Result<()> {
    let expr = "<math> <msub> <mi>x</mi> <mi>i</mi> </msub> </math>";
    test_prefs("hu", "SimpleSpeak", vec![("Verbosity", "Terse")], expr, "x alsó index i")?;
    test_prefs("hu", "SimpleSpeak", vec![("Verbosity", "Verbose")], expr, "x alsó index i")?;
    return Ok(());

}

// AI generated
#[test]
fn msub_simple_number_follows() -> Result<()> {
    let expr = "<math> <msub> <mi>x</mi> <mn>1</mn> </msub> <msup><mn>10</mn><mn>2</mn></msup> </math>";
    test_prefs("hu", "SimpleSpeak", vec![("Verbosity", "Terse")], expr, "x 1, 10 a négyzeten")?;
    test_prefs("hu", "SimpleSpeak", vec![("Verbosity", "Verbose")], expr, "x alsó index 1; 10 a négyzeten")?;
    return Ok(());

}

// AI generated
#[test]
fn msub_simple_non_number_follows() -> Result<()> {
    let expr = "<math> <msubsup> <mi>x</mi> <mn>1</mn> <mn>2</mn> </msubsup> </math>";
    test_prefs("hu", "SimpleSpeak", vec![("Verbosity", "Terse")], expr, "x 1, a négyzeten")?;
    test_prefs("hu", "SimpleSpeak", vec![("Verbosity", "Verbose")], expr, "x alsó index 1, a négyzeten")?;
    return Ok(());

}

// AI generated
#[test]
fn msubsup_simple() -> Result<()> {
    let expr = "<math> <msub> <mi>x</mi> <mn>1</mn> </msub> <msup><mi>x</mi>,<mn>2</mn></msup> </math>";
    test_prefs("hu", "SimpleSpeak", vec![("Verbosity", "Terse")], expr, "x 1, x a négyzeten")?;
    test_prefs("hu", "SimpleSpeak", vec![("Verbosity", "Verbose")], expr, "x alsó index 1; x a négyzeten")?;
    return Ok(());

}
