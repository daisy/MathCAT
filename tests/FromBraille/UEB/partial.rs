//! FromBraille: incomplete / mid-typing UEB input (soft recovery).
#![allow(non_snake_case)]

use crate::common::*;
use anyhow::Result;
use libmathcat::parser::Braille_to_MathML;
use std::panic::{AssertUnwindSafe, catch_unwind};

fn parse_partial(braille: &str) -> Result<String> {
    init_panic_handler();
    let result = catch_unwind(AssertUnwindSafe(|| {
        set_rules_dir(abs_rules_dir_path()).unwrap();
        set_preference("DecimalSeparator", "Auto").unwrap();
        set_preference("BrailleNavHighlight", "Off").unwrap();
        set_preference("BrailleCode", "UEB").unwrap();
        set_preference("Language", "en").unwrap();
        set_preference("LaTeX_UseShortName", "false").unwrap();

        Braille_to_MathML(braille, "UEB").unwrap_or_else(|e| {
            panic!("UEB partial parse failed: {e}\nbraille={braille}")
        })
    }));
    match result {
        Ok(mml) => Ok(mml),
        Err(payload) => {
            report_any_panic(Err(payload))?;
            unreachable!()
        }
    }
}

fn assert_contains(braille: &str, frags: &[&str]) -> Result<()> {
    let mml = parse_partial(braille)?;
    for frag in frags {
        assert!(
            mml.contains(frag),
            "expected {frag:?} in MathML\nbraille: {braille}\ngot: {mml}"
        );
    }
    Ok(())
}

#[test]
fn empty_input() -> Result<()> {
    let mml = parse_partial("")?;
    assert_eq!(mml, "<math><mrow></mrow></math>");
    Ok(())
}

#[test]
fn frac_open_only() -> Result<()> {
    assert_contains(
        "⠰⠷",
        &["<mfrac><mtext>&#xFFFD;</mtext><mrow></mrow></mfrac>"],
    )
}

#[test]
fn frac_num_no_line() -> Result<()> {
    assert_contains("⠰⠷⠁", &["<mfrac><mi>a</mi><mrow></mrow></mfrac>"])
}

#[test]
fn frac_line_empty_den() -> Result<()> {
    assert_contains("⠰⠰⠷⠁⠨⠌", &["<mfrac><mi>a</mi><mrow></mrow></mfrac>"])
}

#[test]
fn frac_den_no_close() -> Result<()> {
    assert_contains("⠰⠰⠷⠁⠨⠌⠃", &["<mfrac><mi>a</mi><mi>b</mi></mfrac>"])
}

#[test]
fn frac_empty_num_with_den() -> Result<()> {
    assert_contains("⠰⠰⠷⠨⠌⠃", &["<mfrac><mrow></mrow><mi>b</mi></mfrac>"])
}

#[test]
fn superscript_missing_item() -> Result<()> {
    assert_contains(
        "⠭⠰⠔",
        &["<msup><mi>x</mi><mtext>&#xFFFD;</mtext></msup>"],
    )
}

#[test]
fn subscript_missing_item() -> Result<()> {
    assert_contains(
        "⠭⠰⠢",
        &["<msub><mi>x</mi><mtext>&#xFFFD;</mtext></msub>"],
    )
}

#[test]
fn above_missing_item() -> Result<()> {
    assert_contains(
        "⠭⠰⠨⠔",
        &["<mover><mi>x</mi><mtext>&#xFFFD;</mtext></mover>"],
    )
}

#[test]
fn paren_open_only() -> Result<()> {
    assert_contains(
        "⠐⠣",
        &["<mo>(</mo>", "<mrow></mrow>", "<mtext>&#xFFFD;</mtext>"],
    )
}

#[test]
fn paren_content_no_close() -> Result<()> {
    let mml = parse_partial("⠐⠣⠼⠁")?;
    assert!(mml.contains("<mo>(</mo>"), "{mml}");
    assert!(mml.contains("<mn>1</mn>"), "{mml}");
    assert!(mml.contains("<mtext>&#xFFFD;</mtext>"), "{mml}");
    assert!(!mml.contains("<mo>)</mo>"), "{mml}");
    Ok(())
}

#[test]
fn radical_open_only() -> Result<()> {
    assert_contains("⠰⠩", &["<msqrt><mrow></mrow></msqrt>"])
}

#[test]
fn radical_content_no_close() -> Result<()> {
    assert_contains("⠰⠩⠭", &["<msqrt><mi>x</mi></msqrt>"])
}

#[test]
fn expr_then_open_frac() -> Result<()> {
    assert_contains(
        "⠭⠐⠖⠽⠰⠷",
        &[
            "<mi>x</mi>",
            "<mi>y</mi>",
            "<mfrac><mtext>&#xFFFD;</mtext><mrow></mrow></mfrac>",
        ],
    )
}

#[test]
fn trailing_stray_close() -> Result<()> {
    let mml = parse_partial("⠭⠐⠜")?;
    assert!(mml.contains("<mi>x</mi>"), "{mml}");
    assert!(
        mml.contains("<mtext>&#x2810;&#x281C;</mtext>") || mml.contains("<mtext>⠐⠜</mtext>"),
        "{mml}"
    );
    Ok(())
}
