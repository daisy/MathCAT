use crate::common::*;
use anyhow::Result;

// AI generated
#[test]
fn msqrt_simple() -> Result<()> {
    let expr = "<math>
                    <msqrt> <mi>x</mi> </msqrt>
                </math>";
    test("hu", "ClearSpeak", expr, "the square root of x")?;
    return Ok(());

}

// AI generated
#[test]
fn msqrt_simple_end_root() -> Result<()> {
    let expr = "<math>
                    <msqrt> <mi>x</mi> </msqrt>
                </math>";
    test_ClearSpeak("hu", "ClearSpeak_Roots", "RootEnd", expr, "the square root of x, end root")?;
    return Ok(());

}

// AI generated
#[test]
fn msqrt_simple_positive() -> Result<()> {
    let expr = "<math>
                    <msqrt> <mi>x</mi> </msqrt>
                </math>";
    test_ClearSpeak("hu", "ClearSpeak_Roots", "PosNegSqRoot", expr, "the positive square root of x")?;
    return Ok(());

}

// AI generated
#[test]
fn msqrt_simple_pos_end_root() -> Result<()> {
    let expr = "<math>
                    <msqrt> <mi>x</mi> </msqrt>
                </math>";
    test_ClearSpeak("hu", "ClearSpeak_Roots", "PosNegSqRootEnd", expr, "the positive square root of x, end root")?;
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
    "the negative square root of x, end root; minus, the positive cube root of x, end root")?;
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
    "the negative cube root of x; minus the positive square root of x")?;
    return Ok(());

}

// AI generated
#[test]
fn neg_without_root() -> Result<()> {
    let expr = "<math>
                    <mo>-</mo> <mi>x</mi> <mo>-</mo> <mi>y</mi>
                </math>";
    test("hu", "ClearSpeak", expr, "negative x minus y")?;
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
    test("hu", "ClearSpeak", expr, "the square root of x plus y")?;
    return Ok(());

}

// AI generated
#[test]
fn mroot_as_square_root() -> Result<()> {
    let expr = "<math>
                    <mroot> <mi>x</mi> <mn>2</mn> </mroot>
                </math>";
    test("hu", "ClearSpeak", expr, "the square root of x")?;
    return Ok(());

}

// AI generated
#[test]
fn cube_root() -> Result<()> {
    let expr = "<math>
                    <mroot> <mi>x</mi> <mn>3</mn> </mroot>
                </math>";
    test("hu", "ClearSpeak", expr, "the cube root of x")?;
    return Ok(());

}

// AI generated
#[test]
fn ordinal_root() -> Result<()> {
    let expr = "<math>
                    <mroot> <mi>x</mi> <mn>9</mn> </mroot>
                </math>";
    test("hu", "ClearSpeak", expr, "the ninth root of x")?;
    return Ok(());

}

// AI generated
#[test]
fn simple_mi_root() -> Result<()> {
    let expr = "<math>
                    <mroot> <mi>x</mi> <mi>n</mi> </mroot>
                </math>";
    test("hu", "ClearSpeak", expr, "the n-th root of x")?;
    return Ok(());

}

// AI generated
#[test]
fn mroot_simple_pos_end_root() -> Result<()> {
    let expr = "<math>
                <mroot> <mi>x</mi> <mi>t</mi> </mroot>
                </math>";
    test_ClearSpeak("hu", "ClearSpeak_Roots", "PosNegSqRootEnd", expr, "the positive t-th root of x, end root")?;
    return Ok(());

}

// AI generated
#[test]
fn mroot_simple_end_root() -> Result<()> {
    let expr = "<math>
                    <mroot> <mrow> <mi>x</mi> <mo>+</mo> <mi>y</mi> </mrow> 
                    <mn>21</mn></mroot>
                </math>";
    test_ClearSpeak("hu", "ClearSpeak_Roots", "RootEnd", expr, "the twenty first root of x plus y, end root")?;
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
    test("hu", "ClearSpeak", expr, "the 1 third root of x")?;
    return Ok(());

}
