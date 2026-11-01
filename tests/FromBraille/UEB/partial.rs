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
    assert_contains("⠐⠣", &["<math><mo>(</mo></math>"])
}

#[test]
fn paren_content_no_close() -> Result<()> {
    let mml = parse_partial("⠐⠣⠼⠁")?;
    assert!(mml.contains("<mo>(</mo>"), "{mml}");
    assert!(mml.contains("<mn>1</mn>"), "{mml}");
    assert!(!mml.contains("&#xFFFD;"), "{mml}");
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
    assert!(mml.contains("<mo>)</mo>"), "{mml}");
    assert!(!mml.contains("&#xFFFD;"), "{mml}");
    Ok(())
}

#[test]
fn trailing_stray_close_after_sum() -> Result<()> {
    // 2a+b)  — unmatched close paren after an expression
    let mml = parse_partial("⠼⠃⠰⠁⠐⠖⠃⠐⠜")?;
    assert!(mml.contains("<mn>2</mn>"), "{mml}");
    assert!(mml.contains("<mi>a</mi>"), "{mml}");
    assert!(mml.contains("<mo>+</mo>") || mml.contains("+"), "{mml}");
    assert!(mml.contains("<mi>b</mi>"), "{mml}");
    assert!(mml.contains("<mo>)</mo>"), "{mml}");
    assert!(!mml.contains("<mtext>"), "{mml}");
    Ok(())
}

#[test]
fn assigned_shapes_gtm_14() -> Result<()> {
    // ICEB 3.19 / GTM 14. Side count is the regular polygon. The terminator is optional.
    assert_contains("⠰⠫⠼⠉", &["&#x25B3;"])?; // triangle
    assert_contains("⠫⠼⠉", &["&#x25B3;"])?; // triangle without grade 1
    assert_contains("⠨⠫⠼⠉", &["&#x25B3;"])?; // shaded triangle
    assert_contains("⠰⠫⠼⠙", &["&#x25A1;"])?; // square
    assert_contains("⠫⠿", &["&#x25CB;"])?; // white circle
    assert_contains("⠰⠫⠿", &["&#x25CB;"])?; // white circle after grade 1
    assert_contains("⠸⠫⠿", &["&#x25CF;"])?; // black circle
    assert_contains("⠰⠫⠼⠑", &["&#x2B20;"])?; // pentagon
    assert_contains("⠰⠫⠼⠋⠱", &["&#x2B21;"])?; // hexagon, shape terminator
    assert_contains("⠸⠫⠼⠑", &["&#x2B1F;"])?; // black pentagon
    assert_contains("⠸⠫⠼⠋", &["&#x2B22;"])?; // black hexagon
    assert_contains("⠸⠫⠈⠼⠋", &["&#x2B23;"])?; // horizontal black hexagon
    assert_contains("⠰⠫⠼⠓", &["&#x2BC4;"])?; // white octagon
    assert_contains("⠸⠫⠼⠓", &["&#x2BC3;"])?; // black octagon
    assert_contains("⠸⠫⠼⠉", &["&#x25B2;"])?; // filled triangle
    assert_contains("⠨⠫⠿", &["&#x25CD;"])?; // shaded circle
    assert_contains("⠰⠫⠈⠼⠙", &["&#x25B1;"])?; // parallelogram
    // Heptagon is an assigned shape with no Unicode character: keep the whole shape.
    let heptagon = parse_partial("⠰⠫⠼⠛")?;
    assert!(
        heptagon.contains("<mtext>&#x2830;&#x282B;&#x283C;&#x281B;</mtext>"),
        "{heptagon}"
    );
    assert!(!heptagon.contains("<mn>7</mn>"), "{heptagon}");
    Ok(())
}

#[test]
fn unmatched_shape_prefixes_stay_braille() -> Result<()> {
    // ⠰⠫, ⠸⠫, and ⠨⠫ are shape indicators. With no defined shape after them
    // they stay as those cells. U+25C7 is not a UEB shape.
    for braille in ["⠰⠫", "⠸⠫", "⠨⠫"] {
        let mml = parse_partial(braille)?;
        let cells: String = braille
            .chars()
            .map(|c| format!("&#x{:X};", c as u32))
            .collect();
        assert!(
            mml.contains(&format!("<mtext>{cells}</mtext>")),
            "expected braille cells {cells} in MathML\nbraille: {braille}\ngot: {mml}"
        );
        assert!(!mml.contains("25C7"), "{mml}");
    }
    // A defined shape is still the print character (△), not the indicator cells.
    let triangle = parse_partial("⠰⠫⠼⠉")?;
    assert!(
        triangle.contains("&#x25B3;") || triangle.contains("△"),
        "{triangle}"
    );
    assert!(!triangle.contains("25C7"), "{triangle}");
    Ok(())
}

#[test]
fn trailing_incomplete_prefix() -> Result<()> {
    // a+b⠐
    let mml = parse_partial("⠁⠐⠖⠃⠐")?;
    assert!(mml.contains("<mi>a</mi>"), "{mml}");
    assert!(mml.contains("<mi>b</mi>"), "{mml}");
    assert!(
        mml.contains("<mtext>&#x2810;</mtext>") || mml.contains("<mtext>⠐</mtext>"),
        "{mml}"
    );
    Ok(())
}
