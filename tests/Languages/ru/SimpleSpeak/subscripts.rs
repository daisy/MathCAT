use crate::common::*;

#[test]
fn msub_simple() {
    let expr = "<math> <msub> <mi>x</mi> <mn>1</mn> </msub> </math>";
    test_prefs("ru", "SimpleSpeak", vec![("Verbosity", "Terse")], expr, "икс 1");
    test_prefs("ru", "SimpleSpeak", vec![("Verbosity", "Medium")], expr, "икс с индексом 1");
    test_prefs("ru", "SimpleSpeak", vec![("Verbosity", "Verbose")], expr, "икс с индексом 1");
}

#[test]
fn msub_not_simple() {
    let expr = "<math> <msub> <mi>x</mi> <mn>1.2</mn> </msub> </math>";
    test_prefs("ru", "SimpleSpeak", vec![("Verbosity", "Terse")], expr, "икс с индексом 1.2");
}

#[test]
fn msubsup_not_simple() {
    let expr = "<math> <msubsup> <mi>x</mi> <mn>1.2</mn> <mn>3</mn></msubsup> </math>";
    test_prefs("ru", "SimpleSpeak", vec![("Verbosity", "Terse")], expr, "икс с индексом 1.2, в кубе");
}

#[test]
fn msub_simple_mi() {
    let expr = "<math> <msub> <mi>x</mi> <mi>i</mi> </msub> </math>";
    test_prefs("ru", "SimpleSpeak", vec![("Verbosity", "Terse")], expr, "икс с индексом i");
    test_prefs("ru", "SimpleSpeak", vec![("Verbosity", "Verbose")], expr, "икс с индексом i");
}

#[test]
fn msub_simple_number_follows() {
    let expr = "<math> <msub> <mi>x</mi> <mn>1</mn> </msub> <msup><mn>10</mn><mn>2</mn></msup> </math>";
    test_prefs("ru", "SimpleSpeak", vec![("Verbosity", "Terse")], expr, "икс 1, 10 в квадрате");
    test_prefs("ru", "SimpleSpeak", vec![("Verbosity", "Verbose")], expr, "икс с индексом 1, 10 в квадрате");
}

#[test]
fn msub_simple_non_number_follows() {
    let expr = "<math> <msubsup> <mi>x</mi> <mn>1</mn> <mn>2</mn> </msubsup> </math>";
    test_prefs("ru", "SimpleSpeak", vec![("Verbosity", "Terse")], expr, "икс 1, в квадрате");
    test_prefs("ru", "SimpleSpeak", vec![("Verbosity", "Verbose")], expr, "икс с индексом 1, в квадрате");
}

#[test]
fn msubsup_simple() {
    let expr = "<math> <msub> <mi>x</mi> <mn>1</mn> </msub> <msup><mi>x</mi><mn>2</mn></msup> </math>";
    test_prefs("ru", "SimpleSpeak", vec![("Verbosity", "Terse")], expr, "икс 1, икс в квадрате");
    test_prefs("ru", "SimpleSpeak", vec![("Verbosity", "Verbose")], expr, "икс с индексом 1, икс в квадрате");
}