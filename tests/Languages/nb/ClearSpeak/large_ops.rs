use crate::common::*;
use anyhow::Result;

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
    test("nb", "ClearSpeak", expr, "sum fra n er lik 1, til 10, av n")?;
    return Ok(());

}

#[test]
fn sum_under() -> Result<()> {
    let expr = "<math>
        <munder>
            <mo>∑</mo>
            <mi>S</mi>
        </munder>
        <mi>i</mi>
    </math>";
    test("nb", "ClearSpeak", expr, "sum over stor s, av i")?;
    return Ok(());

}
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
    test("nb", "ClearSpeak", expr, "sum fra n er lik 1, til 10, av n")?;
    return Ok(());

}

#[test]
fn sum_sub() -> Result<()> {
    let expr = "<math>
        <msub>
            <mo>∑</mo>
            <mi>S</mi>
        </msub>
        <mi>i</mi>
    </math>";
    test("nb", "ClearSpeak", expr, "sum over stor s, av i")?;
    return Ok(());

}

#[test]
fn sum() -> Result<()> {
    let expr = "<math>
            <mo>∑</mo>
            <msub><mi>a</mi><mi>i</mi></msub>
    </math>";
    test("nb", "ClearSpeak", expr, "sum av, a, senket i")?;
    return Ok(());

}

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
    test("nb", "ClearSpeak", expr, "produktet fra n er lik 1, til 10, av n")?;
    return Ok(());

}

#[test]
fn product_under() -> Result<()> {
    let expr = "<math>
        <munder>
            <mo>∏</mo>
            <mi>S</mi>
        </munder>
        <mi>i</mi>
    </math>";
    test("nb", "ClearSpeak", expr, "produktet over stor s, av i")?;
    return Ok(());

}

#[test]
fn product() -> Result<()> {
    let expr = "<math>
            <mo>∏</mo>
            <msub><mi>a</mi><mi>i</mi></msub>
    </math>";
    test("nb", "ClearSpeak", expr, "produktet av, a, senket i")?;
    return Ok(());

}

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
    test("nb", "ClearSpeak", expr, "snitt fra i er lik 1, til 10, av; stor s, senket i")?;
    return Ok(());

}

#[test]
fn intersection_under() -> Result<()> {
    let expr = "<math>
        <munder>
            <mo>⋂</mo>
            <mi>C</mi>
        </munder>
        <msub><mi>S</mi><mi>i</mi></msub>
    </math>";
    test("nb", "ClearSpeak", expr, "snitt over stor c, av, stor s, senket i")?;
    return Ok(());

}

#[test]
fn intersection() -> Result<()> {
    let expr = "<math>
            <mo>⋂</mo>
            <msub><mi>S</mi><mi>i</mi></msub>
            </math>";
    test("nb", "ClearSpeak", expr, "snitt av, stor s, senket i")?;
    return Ok(());

}

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
    test("nb", "ClearSpeak", expr, "union fra i er lik 1, til 10, av; stor s, senket i")?;
    return Ok(());

}

#[test]
fn union_under() -> Result<()> {
    let expr = "<math>
        <munder>
            <mo>⋃</mo>
            <mi>C</mi>
        </munder>
        <msub><mi>S</mi><mi>i</mi></msub>
    </math>";
    test("nb", "ClearSpeak", expr, "union over stor c, av, stor s, senket i")?;
    return Ok(());

}

#[test]
fn union() -> Result<()> {
    let expr = "<math>
            <mo>⋃</mo>
            <msub><mi>S</mi><mi>i</mi></msub>
            </math>";
    test("nb", "ClearSpeak", expr, "union av, stor s, senket i")?;
    return Ok(());

}

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
    test("nb", "ClearSpeak", expr, "integralet fra 0, til 1, av, f av x; d x")?;
    return Ok(());

}

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
    test("nb", "ClearSpeak", expr, "integralet over de reelle tallene, av; f av x d x")?;
    return Ok(());

}

#[test]
fn integral() -> Result<()> {
    let expr = "<math>
            <mo>∫</mo>
            <mrow><mi>f</mi><mrow><mo>(</mo><mi>x</mi> <mo>)</mo></mrow></mrow>
            <mi>d</mi><mi>x</mi>
            </math>";
    test("nb", "ClearSpeak", expr, "integralet av, f av x d x")?;
    return Ok(());

}