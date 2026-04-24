use crate::common::*;
use anyhow::Result;

#[test]
fn msqrt_simple() -> Result<()> {
    let expr = "<math>
                    <msqrt> <mi>x</mi> </msqrt>
                </math>";
    test("zh-tw", "SimpleSpeak", expr, "根號 x")?;
    return Ok(());

}

#[test]
fn neg_without_root() -> Result<()> {
    let expr = "<math>
                    <mo>-</mo> <mi>x</mi> <mo>-</mo> <mi>y</mi>
                </math>";
    test("zh-tw", "SimpleSpeak", expr, "負 x 減 y")?;
    return Ok(());

}

#[test]
fn msqrt() -> Result<()> {
    let expr = "<math>
                    <msqrt>
                        <mrow> <mi>x</mi> <mo>+</mo> <mi>y</mi> </mrow>
                    </msqrt>
                </math>";
    test("zh-tw", "SimpleSpeak", expr, "根號 x 加 y 結束根號")?;
    return Ok(());

}

#[test]
fn mroot_as_square_root() -> Result<()> {
    let expr = "<math>
                    <mroot> <mi>x</mi> <mn>2</mn> </mroot>
                </math>";
    test("zh-tw", "SimpleSpeak", expr, "根號 x")?;
    return Ok(());

}

#[test]
fn cube_root() -> Result<()> {
    let expr = "<math>
                    <mroot> <mi>x</mi> <mn>3</mn> </mroot>
                </math>";
    test("zh-tw", "SimpleSpeak", expr, "根號 x 的 立方根")?;
    return Ok(());

}

#[test]
fn ordinal_root() -> Result<()> {
    let expr = "<math>
                    <mroot> <mi>x</mi> <mn>9</mn> </mroot>
                </math>";
    test("zh-tw", "SimpleSpeak", expr, "根號 x 的 9 次方根")?;
    return Ok(());

}
#[test]
fn ordinal_root_2() -> Result<()> {
    let expr = "<math>
                    <mroot> <mi>x</mi> <mn>9.1</mn> </mroot>
                </math>";
    test("zh-tw", "SimpleSpeak", expr, "根號 x 的 9.1 次方根")?;
    return Ok(());

}

#[test]
fn simple_mi_root() -> Result<()> {
    let expr = "<math>
                    <mroot> <mi>x</mi> <mi>n</mi> </mroot>
                </math>";
    test("zh-tw", "SimpleSpeak", expr, "根號 x 的 n 次方根")?;
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
    test("zh-tw", "SimpleSpeak", expr, "根號 x 的 3 分之 1 次方根")?;
    return Ok(());

}
