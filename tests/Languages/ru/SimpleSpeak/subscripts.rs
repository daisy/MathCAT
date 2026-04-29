use crate::common::*;

use anyhow::Result;

#[test]
fn msub_simple() -> Result<()> {
    let expr = "<math> <msub> <mi>x</mi> <mn>1</mn> </msub> </math>";
    test_prefs("ru", "SimpleSpeak", vec![("Verbosity", "Terse")], expr, "икс 1")?;
    test_prefs("ru", "SimpleSpeak", vec![("Verbosity", "Medium")], expr, "икс с индексом 1")?;
    test_prefs("ru", "SimpleSpeak", vec![("Verbosity", "Verbose")], expr, "икс с индексом 1")?;
    return Ok(());
}

#[test]
fn msub_not_simple() -> Result<()> {
    let expr = "<math> <msub> <mi>x</mi> <mn>1.2</mn> </msub> </math>";
    test_prefs("ru", "SimpleSpeak", vec![("Verbosity", "Terse")], expr, "икс с индексом 1.2")?;
    return Ok(());
}

#[test]
fn msubsup_not_simple() -> Result<()> {
    let expr = "<math> <msubsup> <mi>x</mi> <mn>1.2</mn> <mn>3</mn></msubsup> </math>";
    test_prefs("ru", "SimpleSpeak", vec![("Verbosity", "Terse")], expr, "икс с индексом 1.2, в кубе")?;
    return Ok(());
}

#[test]
fn msub_simple_mi() -> Result<()> {
    let expr = "<math> <msub> <mi>x</mi> <mi>i</mi> </msub> </math>";
    test_prefs("ru", "SimpleSpeak", vec![("Verbosity", "Terse")], expr, "икс с индексом i")?;
    test_prefs("ru", "SimpleSpeak", vec![("Verbosity", "Verbose")], expr, "икс с индексом i")?;
    return Ok(());
}

#[test]
fn msub_simple_number_follows() -> Result<()> {
    let expr = "<math> <msub> <mi>x</mi> <mn>1</mn> </msub> <msup><mn>10</mn><mn>2</mn></msup> </math>";
    test_prefs("ru", "SimpleSpeak", vec![("Verbosity", "Terse")], expr, "икс 1, 10 в квадрате")?;
    test_prefs("ru", "SimpleSpeak", vec![("Verbosity", "Verbose")], expr, "икс с индексом 1, 10 в квадрате")?;
    return Ok(());
}

#[test]
fn msub_simple_non_number_follows() -> Result<()> {
    let expr = "<math> <msubsup> <mi>x</mi> <mn>1</mn> <mn>2</mn> </msubsup> </math>";
    test_prefs("ru", "SimpleSpeak", vec![("Verbosity", "Terse")], expr, "икс 1, в квадрате")?;
    test_prefs("ru", "SimpleSpeak", vec![("Verbosity", "Verbose")], expr, "икс с индексом 1, в квадрате")?;
    return Ok(());
}

#[test]
fn msubsup_simple() -> Result<()> {
    let expr = "<math> <msub> <mi>x</mi> <mn>1</mn> </msub> <msup><mi>x</mi><mn>2</mn></msup> </math>";
    test_prefs("ru", "SimpleSpeak", vec![("Verbosity", "Terse")], expr, "икс 1, икс в квадрате")?;
    test_prefs("ru", "SimpleSpeak", vec![("Verbosity", "Verbose")], expr, "икс с индексом 1, икс в квадрате")?;
    return Ok(());
}