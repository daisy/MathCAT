use crate::common::*;
use anyhow::Result;

// AI generated
#[test]
fn complex() -> Result<()> {
    let expr = "<math>
                    <mi>ℂ</mi>
                </math>";
    test("hu", "SimpleSpeak", expr, "a komplex számok halmaza")?;
    return Ok(());

}

// AI generated
#[test]
fn natural() -> Result<()> {
    let expr = "<math>
                    <mi>ℕ</mi>
                </math>";
    test("hu", "SimpleSpeak", expr, "a természetes számok halmaza")?;
    return Ok(());

}

// AI generated
#[test]
fn rationals() -> Result<()> {
    let expr = "<math>
                    <mi>ℚ</mi>
                </math>";
    test("hu", "SimpleSpeak", expr, "a racionális számok halmaza")?;
    return Ok(());

}

// AI generated
#[test]
fn reals() -> Result<()> {
    let expr = "<math>
                    <mi>ℝ</mi>
                </math>";
    test("hu", "SimpleSpeak", expr, "a valós számok halmaza")?;
    return Ok(());

}

// AI generated
#[test]
fn integers() -> Result<()> {
    let expr = "<math>
                    <mi>ℤ</mi>
                </math>";
    test("hu", "SimpleSpeak", expr, "az egész számok halmaza")?;
    return Ok(());

}



// AI generated
#[test]
fn msup_complex() -> Result<()> {
    let expr = "<math>
                <msup>
                    <mi>ℂ</mi>
                    <mn>2</mn>
                </msup>
                </math>";
    test("hu", "SimpleSpeak", expr, "komplex számhalmaz 2")?;
    return Ok(());

}

// AI generated
#[test]
fn msup_natural() -> Result<()> {
    let expr = "<math>
                <msup>
                    <mi>ℕ</mi>
                    <mn>2</mn>
                </msup>
            </math>";
    test("hu", "SimpleSpeak", expr, "természetes számhalmaz 2")?;
    return Ok(());

}

// AI generated
#[test]
fn msup_rationals() -> Result<()> {
    let expr = "<math>
                <msup>
                    <mi>ℚ</mi>
                    <mn>2</mn>
                </msup>
            </math>";
    test("hu", "SimpleSpeak", expr, "racionális számhalmaz 2")?;
    return Ok(());

}

// AI generated
#[test]
fn msup_reals() -> Result<()> {
    let expr = "<math>
                <msup>
                    <mi>ℝ</mi>
                    <mn>3</mn>
                </msup>
            </math>";
    test("hu", "SimpleSpeak", expr, "valós számhalmaz 3")?;
    return Ok(());

}

// AI generated
#[test]
fn msup_integers() -> Result<()> {
    let expr = "<math>
                <msup>
                    <mi>ℤ</mi>
                    <mn>4</mn>
                </msup>
            </math>";
    test("hu", "SimpleSpeak", expr, "egész számhalmaz 4")?;
    return Ok(());

}

// AI generated
#[test]
fn msup_positive_integers() -> Result<()> {
    let expr = "<math>
                <msup>
                    <mi>ℤ</mi>
                    <mo>+</mo>
                </msup>
            </math>";
    test("hu", "SimpleSpeak", expr, "pozitív egész számok")?;
    return Ok(());

}

// AI generated
#[test]
fn msup_negative_integers() -> Result<()> {
    let expr = "<math>
                <msup>
                    <mi>ℤ</mi>
                    <mo>-</mo>
                </msup>
            </math>";
    test("hu", "SimpleSpeak", expr, "negatív egész számok")?;
    return Ok(());

}

// AI generated
#[test]
fn msup_positive_rationals() -> Result<()> {
    let expr = "<math>
                <msup>
                    <mi>ℚ</mi>
                    <mo>+</mo>
                </msup>
            </math>";
    test("hu", "SimpleSpeak", expr, "pozitív racionális számok")?;
    return Ok(());

}

// AI generated
#[test]
fn msup_negative_rationals() -> Result<()> {
    let expr = "<math>
                <msup>
                    <mi>ℚ</mi>
                    <mo>-</mo>
                </msup>
            </math>";
    test("hu", "SimpleSpeak", expr, "negatív racionális számok")?;
    return Ok(());

}

// AI generated
#[test]
fn empty_set() -> Result<()> {
    let expr = "<math>
                <mo>{</mo> <mo>}</mo>
            </math>";
    test("hu", "SimpleSpeak", expr, "az üres halmaz")?;
    return Ok(());

}

// AI generated
#[test]
fn single_element_set() -> Result<()> {
    let expr = "<math>
                <mo>{</mo> <mn>12</mn><mo>}</mo>
            </math>";
    test("hu", "SimpleSpeak", expr, "a(z) 12 halmaz")?;
    return Ok(());

}

// AI generated
#[test]
fn multiple_element_set() -> Result<()> {
    let expr = "<math>
                <mo>{</mo> <mn>5</mn> <mo>,</mo> <mn>10</mn>  <mo>,</mo> <mn>15</mn> <mo>}</mo>
            </math>";
    test("hu", "SimpleSpeak", expr, "a(z) 5 vessző, 10 vessző, 15 halmaz")?;
    return Ok(());

}

// AI generated
#[test]
fn set_with_colon() -> Result<()> {
    let expr = "<math>
                    <mo>{</mo> <mrow><mi>x</mi><mo>:</mo><mi>x</mi><mo>&#x003E;</mo><mn>2</mn></mrow> <mo>}</mo>
            </math>";
    test("hu", "SimpleSpeak", expr, "x melyekre teljesül, hogy minden x nagyobb, mint 2")?;
    return Ok(());

}

// AI generated
#[test]
fn set_with_bar() -> Result<()> {
    let expr = "<math>
                    <mo>{</mo> <mrow><mi>x</mi><mo>|</mo><mi>x</mi><mo>&#x003E;</mo><mn>2</mn></mrow> <mo>}</mo>
            </math>";
    test("hu", "SimpleSpeak", expr, "x melyekre teljesül, hogy minden x nagyobb, mint 2")?;
    return Ok(());

}

// AI generated
#[test]
fn element_alone() -> Result<()> {
    let expr = "<math>
            <mn>3</mn><mo>+</mo><mn>2</mn><mi>i</mi><mo>∉</mo><mi>ℝ</mi>
        </math>";
    test("hu", "SimpleSpeak", expr, "3 plusz 2 i, nem eleme ennek:, a valós számok halmaza")?;
    return Ok(());

}

// AI generated
#[test]
fn element_under_sum() -> Result<()> {
    let expr = "<math>
            <munder>
                <mo>∑</mo>
                <mrow> <mi>i</mi> <mo>∈</mo> <mi>ℤ</mi> </mrow>
            </munder>
            <mfrac>
                <mn>1</mn>
                <mrow> <msup>  <mi>i</mi> <mn>2</mn> </msup> </mrow>
            </mfrac>
        </math>";
    test("hu", "SimpleSpeak", expr,
                    "összeg alatta i eleme ennek:, az egész számok halmaza; tört, 1 per, i a négyzeten, tört vége")?;
                    return Ok(());

}

// AI generated
#[test]
fn complicated_set_with_colon() -> Result<()> {
    let expr = "<math>
            <mo>{</mo>
            <mi>x</mi>
            <mo>∈</mo>
            <mi>ℤ</mi>
            <mo>:</mo>
            <mn>2</mn>
            <mo>&#x003C;</mo>
            <mi>x</mi>
            <mo>&#x003C;</mo>
            <mn>7</mn>
            <mo>}</mo>
        </math>";
    test("hu", "SimpleSpeak", expr, "x eleme ennek:, az egész számok halmaza melyekre teljesül, hogy minden 2 kisebb, mint x kisebb, mint 7")?;
    return Ok(());

}

// AI generated
#[test]
fn complicated_set_with_mtext() -> Result<()> {
    // as of 8/5/21, parsing of "|" is problematic an element of the example, so <mrows> are needed for this test
    let expr = "<math>
        <mo>{</mo>
        <mrow> <mi>x</mi><mo>∈</mo><mi>ℕ</mi></mrow>
        <mo>|</mo>
        <mrow><mi>x</mi> <mtext>&#x00A0;egy&#x00A0;páros&#x00A0;szám</mtext> </mrow>
        <mo>}</mo>
        </math>";
    test("hu", "SimpleSpeak", expr, 
            "x eleme ennek:, a természetes számok halmaza melyekre teljesül, hogy minden x egy páros szám")?;
            return Ok(());

}
