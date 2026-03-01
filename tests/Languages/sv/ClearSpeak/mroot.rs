use crate::common::*;
use anyhow::Result;

#[test]
fn msqrt_simple() -> Result<()> {
    let expr = "<math>
                    <msqrt> <mi>x</mi> </msqrt>
                </math>";
    test("sv", "ClearSpeak", expr, "kvadratroten ur x")?;
    return Ok(());

}

#[test]
fn msqrt_simple_end_root() -> Result<()> {
    let expr = "<math>
                    <msqrt> <mi>x</mi> </msqrt>
                </math>";
    test_ClearSpeak("sv", "ClearSpeak_Roots", "RootEnd", expr, "kvadratroten ur x, slut rot")?;
    return Ok(());

}

#[test]
fn msqrt_simple_positive() -> Result<()> {
    let expr = "<math>
                    <msqrt> <mi>x</mi> </msqrt>
                </math>";
    test_ClearSpeak("sv", "ClearSpeak_Roots", "PosNegSqRoot", expr, "positiva kvadratroten ur x")?;
    return Ok(());

}

#[test]
fn msqrt_simple_pos_end_root() -> Result<()> {
    let expr = "<math>
                    <msqrt> <mi>x</mi> </msqrt>
                </math>";
    test_ClearSpeak("sv", "ClearSpeak_Roots", "PosNegSqRootEnd", expr, "positiva kvadratroten ur x, slut rot")?;
    return Ok(());

}

#[test]
fn msqrt_simple_pos_end_with_neg_root() -> Result<()> {
    let expr = "<math>
                    <mo>-</mo> <msqrt> <mi>x</mi> </msqrt>
                    <mo>-</mo> <mroot> <mi>x</mi> <mn>3</mn></mroot>
                </math>";
    test_ClearSpeak("sv", "ClearSpeak_Roots", "PosNegSqRootEnd", expr, 
    "negativa kvadratroten ur x, slut rot; minus, positiva kubikroten ur x, slut rot")?;
    return Ok(());

}

#[test]
fn mroot_simple_pos_end_with_neg_root() -> Result<()> {
    let expr = "<math>
                    <mo>-</mo> <mroot> <mi>x</mi> <mn>3</mn></mroot>
                    <mo>-</mo> <msqrt> <mi>x</mi> </msqrt>

                </math>";
    test_ClearSpeak("sv", "ClearSpeak_Roots", "PosNegSqRoot", expr, 
    "negativa kubikroten ur x; minus positiva kvadratroten ur x")?;
    return Ok(());

}

#[test]
fn neg_without_root() -> Result<()> {
    let expr = "<math>
                    <mo>-</mo> <mi>x</mi> <mo>-</mo> <mi>y</mi>
                </math>";
    test("sv", "ClearSpeak", expr, "minus x minus y")?;
    return Ok(());

}

#[test]
fn msqrt() -> Result<()> {
    let expr = "<math>
                    <msqrt>
                        <mrow> <mi>x</mi> <mo>+</mo> <mi>y</mi> </mrow>
                    </msqrt>
                </math>";
    test("sv", "ClearSpeak", expr, "kvadratroten ur x plus y")?;
    return Ok(());

}

#[test]
fn mroot_as_square_root() -> Result<()> {
    let expr = "<math>
                    <mroot> <mi>x</mi> <mn>2</mn> </mroot>
                </math>";
    test("sv", "ClearSpeak", expr, "kvadratroten ur x")?;
    return Ok(());

}

#[test]
fn cube_root() -> Result<()> {
    let expr = "<math>
                    <mroot> <mi>x</mi> <mn>3</mn> </mroot>
                </math>";
    test("sv", "ClearSpeak", expr, "kubikroten ur x")?;
    return Ok(());

}

#[test]
fn ordinal_root() -> Result<()> {
    let expr = "<math>
                    <mroot> <mi>x</mi> <mn>9</mn> </mroot>
                </math>";
    test("sv", "ClearSpeak", expr, "nionde roten ur x")?;
    return Ok(());

}

#[test]
fn simple_mi_root() -> Result<()> {
    let expr = "<math>
                    <mroot> <mi>x</mi> <mi>n</mi> </mroot>
                </math>";
    test("sv", "ClearSpeak", expr, "roten av grad n ur x")?;
    return Ok(());

}

#[test]
fn mroot_simple_pos_end_root() -> Result<()> {
    let expr = "<math>
                <mroot> <mi>x</mi> <mi>t</mi> </mroot>
                </math>";
    test_ClearSpeak("sv", "ClearSpeak_Roots", "PosNegSqRootEnd", expr, "positiva roten av grad t ur x, slut rot")?;
    return Ok(());

}

#[test]
fn mroot_simple_end_root() -> Result<()> {
    let expr = "<math>
                    <mroot> <mrow> <mi>x</mi> <mo>+</mo> <mi>y</mi> </mrow> 
                    <mn>21</mn></mroot>
                </math>";
    test_ClearSpeak("sv", "ClearSpeak_Roots", "RootEnd", expr, "tjugo första roten ur x plus y, slut rot")?;
    return Ok(());

}

#[test]
fn simple_fraction_power() -> Result<()> {
    let expr = "<math>
                    <mroot>
                        <mi>x</mi> 
                        <mfrac><mn>1</mn><mn>3</mn></mfrac>
                    </mroot>
                </math>";
    test("sv", "ClearSpeak", expr, "roten av grad 1 tredjedel ur x")?;
    return Ok(());

}
