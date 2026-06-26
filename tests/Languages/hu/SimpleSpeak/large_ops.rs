use crate::common::*;
use anyhow::Result;

// AI generated
#[test]
fn sum_both() -> Result<()> {
    let expr = "<math>
        <munderover>
            <mo>∑</mo>
            <mrow><mi>n</mi><mo>=</mo><mn>1</mn></mrow>
            <mrow><mn>10</mn></mrow>
        </munderover>
        <mi>n</mi>
    </math>";
    test("hu", "SimpleSpeak", expr, "összeg ettől: n egyenlő 1, eddig: 10 értéke: n")?;
    return Ok(());

}

// AI generated
#[test]
fn sum_under() -> Result<()> {
    let expr = "<math>
        <munder>
            <mo>∑</mo>
            <mi>S</mi>
        </munder>
        <mi>i</mi>
    </math>";
    test("hu", "SimpleSpeak", expr, "összeg alatta nagy s i")?;
    return Ok(());

}
// AI generated
#[test]
fn sum_both_msubsup() -> Result<()> {
    let expr = "<math>
        <msubsup>
            <mo>∑</mo>
            <mrow><mi>n</mi><mo>=</mo><mn>1</mn></mrow>
            <mrow><mn>10</mn></mrow>
        </msubsup>
        <mi>n</mi>
    </math>";
    test("hu", "SimpleSpeak", expr, "összeg ettől: n egyenlő 1, eddig: 10 értéke: n")?;
    return Ok(());

}

// AI generated
#[test]
fn sum_sub() -> Result<()> {
    let expr = "<math>
        <msub>
            <mo>∑</mo>
            <mi>S</mi>
        </msub>
        <mi>i</mi>
    </math>";
    test("hu", "SimpleSpeak", expr, "összeg alatta nagy s i")?;
    return Ok(());

}

// AI generated
#[test]
fn sum() -> Result<()> {
    let expr = "<math>
            <mo>∑</mo>
            <msub><mi>a</mi><mi>i</mi></msub>
    </math>";
    test("hu", "SimpleSpeak", expr, "összeg a alsó index i")?;
    return Ok(());

}

// AI generated
#[test]
fn product_both() -> Result<()> {
    let expr = "<math>
        <munderover>
            <mo>∏</mo>
            <mrow><mi>n</mi><mo>=</mo><mn>1</mn></mrow>
            <mrow><mn>10</mn></mrow>
        </munderover>
        <mi>n</mi>
    </math>";
    test("hu", "SimpleSpeak", expr, "produktum ettől: n egyenlő 1, eddig: 10 értéke: n")?;
    return Ok(());

}

// AI generated
#[test]
fn product_under() -> Result<()> {
    let expr = "<math>
        <munder>
            <mo>∏</mo>
            <mi>S</mi>
        </munder>
        <mi>i</mi>
    </math>";
    test("hu", "SimpleSpeak", expr, "produktum alatta nagy s i")?;
    return Ok(());

}

// AI generated
#[test]
fn product() -> Result<()> {
    let expr = "<math>
            <mo>∏</mo>
            <msub><mi>a</mi><mi>i</mi></msub>
    </math>";
    test("hu", "SimpleSpeak", expr, "produktum a alsó index i")?;
    return Ok(());

}

// AI generated
#[test]
fn intersection_both() -> Result<()> {
    let expr = "<math>
        <munderover>
            <mo>⋂</mo>
            <mrow><mi>i</mi><mo>=</mo><mn>1</mn> </mrow>
            <mn>10</mn>
        </munderover>
        <msub><mi>S</mi><mi>i</mi></msub>
    </math>";
    test("hu", "SimpleSpeak", expr, "kereszteződés ettől: i egyenlő 1, eddig: 10 értéke:; nagy s alsó index i")?;
    return Ok(());

}

// AI generated
#[test]
fn intersection_under() -> Result<()> {
    let expr = "<math>
        <munder>
            <mo>⋂</mo>
            <mi>C</mi>
        </munder>
        <msub><mi>S</mi><mi>i</mi></msub>
    </math>";
    test("hu", "SimpleSpeak", expr, "kereszteződés alatta nagy c, nagy s alsó index i")?;
    return Ok(());

}

// AI generated
#[test]
fn intersection() -> Result<()> {
    let expr = "<math>
            <mo>⋂</mo>
            <msub><mi>S</mi><mi>i</mi></msub>
            </math>";
    test("hu", "SimpleSpeak", expr, "kereszteződés nagy s alsó index i")?;
    return Ok(());

}

// AI generated
#[test]
fn union_both() -> Result<()> {
    let expr = "<math>
        <munderover>
            <mo>⋃</mo>
            <mrow><mi>i</mi><mo>=</mo><mn>1</mn> </mrow>
            <mn>10</mn>
        </munderover>
        <msub><mi>S</mi><mi>i</mi></msub>
    </math>";
    test("hu", "SimpleSpeak", expr, "unió ettől: i egyenlő 1, eddig: 10 értéke:; nagy s alsó index i")?;
    return Ok(());

}

// AI generated
#[test]
fn union_under() -> Result<()> {
    let expr = "<math>
        <munder>
            <mo>⋃</mo>
            <mi>C</mi>
        </munder>
        <msub><mi>S</mi><mi>i</mi></msub>
    </math>";
    test("hu", "SimpleSpeak", expr, "unió alatta nagy c, nagy s alsó index i")?;
    return Ok(());

}

// AI generated
#[test]
fn union() -> Result<()> {
    let expr = "<math>
            <mo>⋃</mo>
            <msub><mi>S</mi><mi>i</mi></msub>
            </math>";
    test("hu", "SimpleSpeak", expr, "unió nagy s alsó index i")?;
    return Ok(());

}

// AI generated
#[test]
fn integral_both() -> Result<()> {
    let expr = "<math>
            <mrow>
                <msubsup>
                    <mo>∫</mo>
                    <mn>0</mn>
                    <mn>1</mn>
                </msubsup>
                <mrow><mi>f</mi><mrow><mo>(</mo><mi>x</mi> <mo>)</mo></mrow></mrow>
            </mrow>
            <mtext>&#x2009;</mtext><mi>d</mi><mi>x</mi>
        </math>";
    test("hu", "SimpleSpeak", expr, "integrál ettől: 0, eddig: 1 értéke:; f x; d x")?;
    return Ok(());

}

// AI generated
#[test]
fn integral_under() -> Result<()> {
    let expr = "<math>
        <munder>
            <mo>∫</mo>
            <mi>ℝ</mi>
        </munder>
        <mrow><mi>f</mi><mrow><mo>(</mo><mi>x</mi> <mo>)</mo></mrow></mrow>
        <mi>d</mi><mi>x</mi>
        </math>";
    test("hu", "SimpleSpeak", expr, "integrál alatta a valós számok halmaza; f x d x")?;
    return Ok(());

}

// AI generated
#[test]
fn integral() -> Result<()> {
    let expr = "<math>
            <mo>∫</mo>
            <mrow><mi>f</mi><mrow><mo>(</mo><mi>x</mi> <mo>)</mo></mrow></mrow>
            <mi>d</mi><mi>x</mi>
            </math>";
    test("hu", "SimpleSpeak", expr, "integrál f x d x")?;
    return Ok(());

}
