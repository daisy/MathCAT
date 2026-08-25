//! FromBraille: incomplete / mid-typing Nemeth input (soft recovery).
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
        set_preference("BrailleCode", "Nemeth").unwrap();
        set_preference("Language", "en").unwrap();
        set_preference("LaTeX_UseShortName", "false").unwrap();

        Braille_to_MathML(braille, "Nemeth").unwrap_or_else(|e| {
            panic!("Nemeth partial parse failed: {e}\nbraille={braille}")
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
    assert_contains("⠹", &["<mfrac>"])
}

#[test]
fn frac_num_no_line() -> Result<()> {
    assert_contains("⠹⠁", &["<mfrac>", "<mi>a</mi>"])
}

#[test]
fn frac_line_empty_den() -> Result<()> {
    assert_contains("⠹⠁⠌", &["<mfrac>", "<mi>a</mi>"])
}

#[test]
fn frac_den_no_close() -> Result<()> {
    assert_contains("⠹⠁⠌⠃", &["<mfrac>", "<mi>a</mi>", "<mi>b</mi>"])
}

#[test]
fn superscript_missing_item() -> Result<()> {
    assert_contains("⠭⠘", &["<msup>", "<mi>x</mi>"])
}

#[test]
fn paren_open_only() -> Result<()> {
    let mml = parse_partial("⠷")?;
    assert!(mml.contains("<mo>(</mo>"), "{mml}");
    Ok(())
}

#[test]
fn paren_content_no_close() -> Result<()> {
    let mml = parse_partial("⠷⠼⠂")?;
    assert!(mml.contains("<mo>(</mo>"), "{mml}");
    assert!(mml.contains("<mn>1</mn>"), "{mml}");
    assert!(!mml.contains("<mo>)</mo>"), "{mml}");
    Ok(())
}

#[test]
fn radical_open_only() -> Result<()> {
    assert_contains("⠜", &["<msqrt>"])
}

#[test]
fn radical_content_no_close() -> Result<()> {
    assert_contains("⠜⠭", &["<msqrt>", "<mi>x</mi>"])
}

#[test]
fn trailing_baseline() -> Result<()> {
    let mml = parse_partial("⠭⠐")?;
    assert!(mml.contains("<mi>x</mi>"), "{mml}");
    Ok(())
}
