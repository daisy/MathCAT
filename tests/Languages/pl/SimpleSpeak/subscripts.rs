use crate::common::*;
use anyhow::Result;

#[test]
fn msub_simple() -> Result<()> {
    let expr = "<math> <msub> <mi>x</mi> <mn>1</mn> </msub> </math>";
    test_prefs("pl", "SimpleSpeak", vec![("Verbosity", "Terse")], expr, "x 1")?;
    test_prefs("pl", "SimpleSpeak", vec![("Verbosity", "Medium")], expr, "x indeks dolny 1")?;
    test_prefs("pl", "SimpleSpeak", vec![("Verbosity", "Verbose")], expr, "x indeks dolny 1")?;
    return Ok(());

  }

#[test]
fn msub_not_simple() -> Result<()> {
    let expr = "<math> <msub> <mi>x</mi> <mn>1.2</mn> </msub> </math>";
    test_prefs("pl", "SimpleSpeak", vec![("Verbosity", "Terse")], expr, "x indeks dolny 12")?;
    return Ok(());

  }

#[test]
fn msubsup_not_simple() -> Result<()> {
    let expr = "<math> <msubsup> <mi>x</mi> <mn>1.2</mn> <mn>3</mn></msubsup> </math>";
    test_prefs("pl", "SimpleSpeak", vec![("Verbosity", "Terse")], expr, "x indeks dolny 12, do sześcianu")?;
    return Ok(());

  }

#[test]
fn msub_simple_mi() -> Result<()> {
    let expr = "<math> <msub> <mi>x</mi> <mi>i</mi> </msub> </math>";
    test_prefs("pl", "SimpleSpeak", vec![("Verbosity", "Terse")], expr, "x indeks dolny i")?;
    test_prefs("pl", "SimpleSpeak", vec![("Verbosity", "Verbose")], expr, "x indeks dolny i")?;
    return Ok(());

}

#[test]
fn msub_simple_number_follows() -> Result<()> {
    let expr = "<math> <msub> <mi>x</mi> <mn>1</mn> </msub> <msup><mn>10</mn><mn>2</mn></msup> </math>";
    test_prefs("pl", "SimpleSpeak", vec![("Verbosity", "Terse")], expr, "x 1, 10 do kwadratu")?;
    test_prefs("pl", "SimpleSpeak", vec![("Verbosity", "Verbose")], expr, "x indeks dolny 1; 10 do kwadratu")?;
    return Ok(());

}

#[test]
fn msub_simple_non_number_follows() -> Result<()> {
    let expr = "<math> <msubsup> <mi>x</mi> <mn>1</mn> <mn>2</mn> </msubsup> </math>";
    test_prefs("pl", "SimpleSpeak", vec![("Verbosity", "Terse")], expr, "x 1, do kwadratu")?;
    test_prefs("pl", "SimpleSpeak", vec![("Verbosity", "Verbose")], expr, "x indeks dolny 1, do kwadratu")?;
    return Ok(());

}

#[test]
fn msubsup_simple() -> Result<()> {
    let expr = "<math> <msub> <mi>x</mi> <mn>1</mn> </msub> <msup><mi>x</mi>,<mn>2</mn></msup> </math>";
    test_prefs("pl", "SimpleSpeak", vec![("Verbosity", "Terse")], expr, "x 1, x do kwadratu")?;
    test_prefs("pl", "SimpleSpeak", vec![("Verbosity", "Verbose")], expr, "x indeks dolny 1; x do kwadratu")?;
    return Ok(());

}