use crate::common::*;
use anyhow::Result;

#[test]
fn sum_both() -> Result<()> {
    let expr = "<math>
        <munderover>
            <mo>âˆ‘</mo>
            <mrow><mi>n</mi><mo>=</mo><mn>1</mn></mrow>
            <mrow><mn>10</mn></mrow>
        </munderover>
        <mi>n</mi>
    </math>";
    test("pt", "SimpleSpeak", expr, "the sum from n is equal to 1, to 10 of n")?;
    return Ok(());

}

#[test]
fn sum_under() -> Result<()> {
    let expr = "<math>
        <munder>
            <mo>âˆ‘</mo>
            <mi>S</mi>
        </munder>
        <mi>i</mi>
    </math>";
    test("pt", "SimpleSpeak", expr, "the sum over cap s of i")?;
    return Ok(());

}
#[test]
fn sum_both_msubsup() -> Result<()> {
    let expr = "<math>
        <msubsup>
            <mo>âˆ‘</mo>
            <mrow><mi>n</mi><mo>=</mo><mn>1</mn></mrow>
            <mrow><mn>10</mn></mrow>
        </msubsup>
        <mi>n</mi>
    </math>";
    test("pt", "SimpleSpeak", expr, "the sum from n is equal to 1, to 10 of n")?;
    return Ok(());

}

#[test]
fn sum_sub() -> Result<()> {
    let expr = "<math>
        <msub>
            <mo>âˆ‘</mo>
            <mi>S</mi>
        </msub>
        <mi>i</mi>
    </math>";
    test("pt", "SimpleSpeak", expr, "the sum over cap s of i")?;
    return Ok(());

}

#[test]
fn sum() -> Result<()> {
    let expr = "<math>
            <mo>âˆ‘</mo>
            <msub><mi>a</mi><mi>i</mi></msub>
    </math>";
    test("pt", "SimpleSpeak", expr, "the sum of eigh sub i")?;
    return Ok(());

}

#[test]
fn product_both() -> Result<()> {
    let expr = "<math>
        <munderover>
            <mo>âˆ</mo>
            <mrow><mi>n</mi><mo>=</mo><mn>1</mn></mrow>
            <mrow><mn>10</mn></mrow>
        </munderover>
        <mi>n</mi>
    </math>";
    test("pt", "SimpleSpeak", expr, "the product from n is equal to 1, to 10 of n")?;
    return Ok(());

}

#[test]
fn product_under() -> Result<()> {
    let expr = "<math>
        <munder>
            <mo>âˆ</mo>
            <mi>S</mi>
        </munder>
        <mi>i</mi>
    </math>";
    test("pt", "SimpleSpeak", expr, "the product over cap s of i")?;
    return Ok(());

}

#[test]
fn product() -> Result<()> {
    let expr = "<math>
            <mo>âˆ</mo>
            <msub><mi>a</mi><mi>i</mi></msub>
    </math>";
    test("pt", "SimpleSpeak", expr, "the product of eigh sub i")?;
    return Ok(());

}

#[test]
fn intersection_both() -> Result<()> {
    let expr = "<math>
        <munderover>
            <mo>â‹‚</mo>
            <mrow><mi>i</mi><mo>=</mo><mn>1</mn> </mrow>
            <mn>10</mn>
        </munderover>
        <msub><mi>S</mi><mi>i</mi></msub>
    </math>";
    test("pt", "SimpleSpeak", expr, "the intersection from i is equal to 1, to 10 of; cap s sub i")?;
    return Ok(());

}

#[test]
fn intersection_under() -> Result<()> {
    let expr = "<math>
        <munder>
            <mo>â‹‚</mo>
            <mi>C</mi>
        </munder>
        <msub><mi>S</mi><mi>i</mi></msub>
    </math>";
    test("pt", "SimpleSpeak", expr, "the intersection over cap c of, cap s sub i")?;
    return Ok(());

}

#[test]
fn intersection() -> Result<()> {
    let expr = "<math>
            <mo>â‹‚</mo>
            <msub><mi>S</mi><mi>i</mi></msub>
            </math>";
    test("pt", "SimpleSpeak", expr, "the intersection of cap s sub i")?;
    return Ok(());

}

#[test]
fn union_both() -> Result<()> {
    let expr = "<math>
        <munderover>
            <mo>â‹ƒ</mo>
            <mrow><mi>i</mi><mo>=</mo><mn>1</mn> </mrow>
            <mn>10</mn>
        </munderover>
        <msub><mi>S</mi><mi>i</mi></msub>
    </math>";
    test("pt", "SimpleSpeak", expr, "the union from i is equal to 1, to 10 of; cap s sub i")?;
    return Ok(());

}

#[test]
fn union_under() -> Result<()> {
    let expr = "<math>
        <munder>
            <mo>â‹ƒ</mo>
            <mi>C</mi>
        </munder>
        <msub><mi>S</mi><mi>i</mi></msub>
    </math>";
    test("pt", "SimpleSpeak", expr, "the union over cap c of, cap s sub i")?;
    return Ok(());

}

#[test]
fn union() -> Result<()> {
    let expr = "<math>
            <mo>â‹ƒ</mo>
            <msub><mi>S</mi><mi>i</mi></msub>
            </math>";
    test("pt", "SimpleSpeak", expr, "the union of cap s sub i")?;
    return Ok(());

}

#[test]
fn integral_both() -> Result<()> {
    let expr = "<math>
            <mrow>
                <msubsup>
                    <mo>âˆ«</mo>
                    <mn>0</mn>
                    <mn>1</mn>
                </msubsup>
                <mrow><mi>f</mi><mrow><mo>(</mo><mi>x</mi> <mo>)</mo></mrow></mrow>
            </mrow>
            <mtext>&#x2009;</mtext><mi>d</mi><mi>x</mi>
        </math>";
    test("pt", "SimpleSpeak", expr, "the integral from 0, to 1 of, f of x; d x")?;
    return Ok(());

}

#[test]
fn integral_under() -> Result<()> {
    let expr = "<math>
        <munder>
            <mo>âˆ«</mo>
            <mi>â„</mi>
        </munder>
        <mrow><mi>f</mi><mrow><mo>(</mo><mi>x</mi> <mo>)</mo></mrow></mrow>
        <mi>d</mi><mi>x</mi>
        </math>";
    test("pt", "SimpleSpeak", expr, "the integral over the real numbers of; f of x d x")?;
    return Ok(());

}

#[test]
fn integral() -> Result<()> {
    let expr = "<math>
            <mo>âˆ«</mo>
            <mrow><mi>f</mi><mrow><mo>(</mo><mi>x</mi> <mo>)</mo></mrow></mrow>
            <mi>d</mi><mi>x</mi>
            </math>";
    test("pt", "SimpleSpeak", expr, "the integral of f of x d x")?;
    return Ok(());

}