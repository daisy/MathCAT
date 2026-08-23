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
    test("zh-tw", "SimpleSpeak", expr, "和 從 n 等於 1 到 10 項目 n")?;
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
    test("zh-tw", "SimpleSpeak", expr, "和 下層 大寫 s 項目 i")?;
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
    test("zh-tw", "SimpleSpeak", expr, "和 從 n 等於 1 到 10 項目 n")?;
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
    test("zh-tw", "SimpleSpeak", expr, "和 下層 大寫 s 項目 i")?;
    return Ok(());

}

#[test]
fn sum() -> Result<()> {
    let expr = "<math>
            <mo>∑</mo>
            <msub><mi>a</mi><mi>i</mi></msub>
    </math>";
    test("zh-tw", "SimpleSpeak", expr, "和 項目 a 下標 i")?;
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
    test("zh-tw", "SimpleSpeak", expr, "積 從 n 等於 1 到 10 項目 n")?;
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
    test("zh-tw", "SimpleSpeak", expr, "積 下層 大寫 s 項目 i")?;
    return Ok(());

}

#[test]
fn product() -> Result<()> {
    let expr = "<math>
            <mo>∏</mo>
            <msub><mi>a</mi><mi>i</mi></msub>
    </math>";
    test("zh-tw", "SimpleSpeak", expr, "積 項目 a 下標 i")?;
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
    test("zh-tw", "SimpleSpeak", expr, "交集 從 i 等於 1 到 10 項目; 大寫 s 下標 i")?;
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
    test("zh-tw", "SimpleSpeak", expr, "交集 下層 大寫 c 項目, 大寫 s 下標 i")?;
    return Ok(());

}

#[test]
fn intersection() -> Result<()> {
    let expr = "<math>
            <mo>⋂</mo>
            <msub><mi>S</mi><mi>i</mi></msub>
            </math>";
    test("zh-tw", "SimpleSpeak", expr, "交集 項目 大寫 s 下標 i")?;
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
    test("zh-tw", "SimpleSpeak", expr, "聯集 從 i 等於 1 到 10 項目; 大寫 s 下標 i")?;
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
    test("zh-tw", "SimpleSpeak", expr, "聯集 下層 大寫 c 項目, 大寫 s 下標 i")?;
    return Ok(());

}

#[test]
fn union() -> Result<()> {
    let expr = "<math>
            <mo>⋃</mo>
            <msub><mi>S</mi><mi>i</mi></msub>
            </math>";
    test("zh-tw", "SimpleSpeak", expr, "聯集 項目 大寫 s 下標 i")?;
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
    test("zh-tw", "SimpleSpeak", expr, "積分 從 0 到 1 項目, f x; d x")?;
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
    test("zh-tw", "SimpleSpeak", expr, "積分 下層 實數集 項目; f x d x")?;
    return Ok(());

}

#[test]
fn integral() -> Result<()> {
    let expr = "<math>
            <mo>∫</mo>
            <mrow><mi>f</mi><mrow><mo>(</mo><mi>x</mi> <mo>)</mo></mrow></mrow>
            <mi>d</mi><mi>x</mi>
            </math>";
    test("zh-tw", "SimpleSpeak", expr, "積分 項目 f x d x")?;
    return Ok(());

}