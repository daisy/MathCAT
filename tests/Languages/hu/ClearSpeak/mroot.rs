use crate::common::*;
use anyhow::Result;

// AI generated
#[test]
fn msqrt_simple() -> Result<()> {
    let expr = "<math>
                    <msqrt> <mi>x</mi> </msqrt>
                </math>";
    test("hu", "ClearSpeak", expr, "négyzetgyök x")?;
    return Ok(());

}

// AI generated
#[test]
fn msqrt_simple_end_root() -> Result<()> {
    let expr = "<math>
                    <msqrt> <mi>x</mi> </msqrt>
                </math>";
    test_ClearSpeak("hu", "ClearSpeak_Roots", "RootEnd", expr, "négyzetgyök x, gyök vége")?;
    return Ok(());

}

// AI generated
#[test]
fn msqrt_simple_positive() -> Result<()> {
    let expr = "<math>
                    <msqrt> <mi>x</mi> </msqrt>
                </math>";
    test_ClearSpeak("hu", "ClearSpeak_Roots", "PosNegSqRoot", expr, "pozitív négyzetgyök x")?;
    return Ok(());

}

// AI generated
#[test]
fn msqrt_simple_pos_end_root() -> Result<()> {
    let expr = "<math>
                    <msqrt> <mi>x</mi> </msqrt>
                </math>";
    test_ClearSpeak("hu", "ClearSpeak_Roots", "PosNegSqRootEnd", expr, "pozitív négyzetgyök x, gyök vége")?;
    return Ok(());

}

// AI generated
#[test]
fn msqrt_simple_pos_end_with_neg_root() -> Result<()> {
    let expr = "<math>
                    <mo>-</mo> <msqrt> <mi>x</mi> </msqrt>
                    <mo>-</mo> <mroot> <mi>x</mi> <mn>3</mn></mroot>
                </math>";
    test_ClearSpeak("hu", "ClearSpeak_Roots", "PosNegSqRootEnd", expr, 
    "negatív négyzetgyök x, gyök vége; mínusz, a pozitív köbgyök x, gyök vége")?;
    return Ok(());

}

// AI generated
#[test]
fn mroot_simple_pos_end_with_neg_root() -> Result<()> {
    let expr = "<math>
                    <mo>-</mo> <mroot> <mi>x</mi> <mn>3</mn></mroot>
                    <mo>-</mo> <msqrt> <mi>x</mi> </msqrt>

                </math>";
    test_ClearSpeak("hu", "ClearSpeak_Roots", "PosNegSqRoot", expr, 
    "a negatív köbgyök x; mínusz pozitív négyzetgyök x")?;
    return Ok(());

}

// AI generated
#[test]
fn neg_without_root() -> Result<()> {
    let expr = "<math>
                    <mo>-</mo> <mi>x</mi> <mo>-</mo> <mi>y</mi>
                </math>";
    test("hu", "ClearSpeak", expr, "negatív x mínusz y")?;
    return Ok(());

}

// AI generated
#[test]
fn msqrt() -> Result<()> {
    let expr = "<math>
                    <msqrt>
                        <mrow> <mi>x</mi> <mo>+</mo> <mi>y</mi> </mrow>
                    </msqrt>
                </math>";
    test("hu", "ClearSpeak", expr, "négyzetgyök x plusz y")?;
    return Ok(());

}

// AI generated
#[test]
fn mroot_as_square_root() -> Result<()> {
    let expr = "<math>
                    <mroot> <mi>x</mi> <mn>2</mn> </mroot>
                </math>";
    test("hu", "ClearSpeak", expr, "négyzetgyök x")?;
    return Ok(());

}

// AI generated
#[test]
fn cube_root() -> Result<()> {
    let expr = "<math>
                    <mroot> <mi>x</mi> <mn>3</mn> </mroot>
                </math>";
    test("hu", "ClearSpeak", expr, "a köbgyök x")?;
    return Ok(());

}

// AI generated
#[test]
fn ordinal_root() -> Result<()> {
    let expr = "<math>
                    <mroot> <mi>x</mi> <mn>9</mn> </mroot>
                </math>";
    test("hu", "ClearSpeak", expr, "a kilencedik gyök x")?;
    return Ok(());

}

// AI generated
#[test]
fn simple_mi_root() -> Result<()> {
    let expr = "<math>
                    <mroot> <mi>x</mi> <mi>n</mi> </mroot>
                </math>";
    test("hu", "ClearSpeak", expr, "a n-edik gyök x")?;
    return Ok(());

}

// AI generated
#[test]
fn mroot_simple_pos_end_root() -> Result<()> {
    let expr = "<math>
                <mroot> <mi>x</mi> <mi>t</mi> </mroot>
                </math>";
    test_ClearSpeak("hu", "ClearSpeak_Roots", "PosNegSqRootEnd", expr, "a pozitív t-edik gyök x, gyök vége")?;
    return Ok(());

}

// AI generated
#[test]
fn mroot_simple_end_root() -> Result<()> {
    let expr = "<math>
                    <mroot> <mrow> <mi>x</mi> <mo>+</mo> <mi>y</mi> </mrow> 
                    <mn>21</mn></mroot>
                </math>";
    test_ClearSpeak("hu", "ClearSpeak_Roots", "RootEnd", expr, "a huszonegyedik gyök x plusz y, gyök vége")?;
    return Ok(());

}

// AI generated
#[test]
fn simple_fraction_power() -> Result<()> {
    let expr = "<math>
                    <mroot>
                        <mi>x</mi> 
                        <mfrac><mn>1</mn><mn>3</mn></mfrac>
                    </mroot>
                </math>";
    test("hu", "ClearSpeak", expr, "a 1 harmad gyök x")?;
    return Ok(());

}
