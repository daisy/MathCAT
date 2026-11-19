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
    test("el", "SimpleSpeak", expr, "άθροισμα από n ισούται με 1 ως 10 του n")?;
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
    test("el", "SimpleSpeak", expr, "άθροισμα για κεφαλαίο s του i")?;
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
    test("el", "SimpleSpeak", expr, "άθροισμα από n ισούται με 1 ως 10 του n")?;
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
    test("el", "SimpleSpeak", expr, "άθροισμα για κεφαλαίο s του i")?;
    return Ok(());

}

#[test]
fn sum() -> Result<()> {
    let expr = "<math>
            <mo>∑</mo>
            <msub><mi>a</mi><mi>i</mi></msub>
    </math>";
    test("el", "SimpleSpeak", expr, "άθροισμα του a δείκτης i")?;
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
    test("el", "SimpleSpeak", expr, "γινόμενο από n ισούται με 1 ως 10 του n")?;
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
    test("el", "SimpleSpeak", expr, "γινόμενο για κεφαλαίο s του i")?;
    return Ok(());

}

#[test]
fn product() -> Result<()> {
    let expr = "<math>
            <mo>∏</mo>
            <msub><mi>a</mi><mi>i</mi></msub>
    </math>";
    test("el", "SimpleSpeak", expr, "γινόμενο του a δείκτης i")?;
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
    test("el", "SimpleSpeak", expr, "τομή από i ισούται με 1 ως 10 του; κεφαλαίο s δείκτης i")?;
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
    test("el", "SimpleSpeak", expr, "τομή για κεφαλαίο c του; κεφαλαίο s δείκτης i")?;
    return Ok(());

}

#[test]
fn intersection() -> Result<()> {
    let expr = "<math>
            <mo>⋂</mo>
            <msub><mi>S</mi><mi>i</mi></msub>
            </math>";
    test("el", "SimpleSpeak", expr, "τομή του κεφαλαίο s δείκτης i")?;
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
    test("el", "SimpleSpeak", expr, "ένωση από i ισούται με 1 ως 10 του; κεφαλαίο s δείκτης i")?; // ένωση από i ισούται με 1 ως 10 του; κεφαλαίο s δείκτης i
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
    test("el", "SimpleSpeak", expr, "ένωση για κεφαλαίο c του; κεφαλαίο s δείκτης i")?; // ένωση όπου κεφαλαίο c του; κεφαλαίο s δείκτης i
    return Ok(());

}

#[test]
fn union() -> Result<()> {
    let expr = "<math>
            <mo>⋃</mo>
            <msub><mi>S</mi><mi>i</mi></msub>
            </math>";
    test("el", "SimpleSpeak", expr, "ένωση του κεφαλαίο s δείκτης i")?; // ένωση του κεφαλαίο s δείκτης i
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
    test("el", "SimpleSpeak", expr, "ολοκλήρωμα από 0 ως 1 του; f του x; d x")?; // ολοκλήρωμα από 0 ως 1 του; f του x; d x
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
    test("el", "SimpleSpeak", expr, "ολοκλήρωμα επί του συνόλου των πραγματικών αριθμών του; f του x, d x")?; // ολοκλήρωμα όπου οι πραγματικοί αριθμοί του; f του x, d x  ???? όπου
    return Ok(());

}

#[test]
fn integral() -> Result<()> {
    let expr = "<math>
            <mo>∫</mo>
            <mrow><mi>f</mi><mrow><mo>(</mo><mi>x</mi> <mo>)</mo></mrow></mrow>
            <mi>d</mi><mi>x</mi>
            </math>";
    test("el", "SimpleSpeak", expr, "ολοκλήρωμα του f του x, d x")?; // ολοκλήρωμα του f του x, d x
    return Ok(());

} 