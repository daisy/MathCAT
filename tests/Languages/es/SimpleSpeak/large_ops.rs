use crate::common::*;
use anyhow::Result;

// REVISAR (el suma): «el suma de n es igual a 1 a 10 de n»
#[test]
#[ignore = "lectura en español pendiente de revisión"]
fn sum_both() -> Result<()> {
    let expr = "<math>
        <munderover>
            <mo>∑</mo>
            <mrow><mi>n</mi><mo>=</mo><mn>1</mn></mrow>
            <mrow><mn>10</mn></mrow>
        </munderover>
        <mi>n</mi>
    </math>";
    test("es", "SimpleSpeak", expr, "el suma de n es igual a 1 a 10 de n")?;
    return Ok(());

}

// REVISAR (el suma): «el suma partido por mayúscula s de i»
#[test]
#[ignore = "lectura en español pendiente de revisión"]
fn sum_under() -> Result<()> {
    let expr = "<math>
        <munder>
            <mo>∑</mo>
            <mi>S</mi>
        </munder>
        <mi>i</mi>
    </math>";
    test("es", "SimpleSpeak", expr, "el suma partido por mayúscula s de i")?;
    return Ok(());

}
// REVISAR (el suma): «el suma de n es igual a 1 a 10 de n»
#[test]
#[ignore = "lectura en español pendiente de revisión"]
fn sum_both_msubsup() -> Result<()> {
    let expr = "<math>
        <msubsup>
            <mo>∑</mo>
            <mrow><mi>n</mi><mo>=</mo><mn>1</mn></mrow>
            <mrow><mn>10</mn></mrow>
        </msubsup>
        <mi>n</mi>
    </math>";
    test("es", "SimpleSpeak", expr, "el suma de n es igual a 1 a 10 de n")?;
    return Ok(());

}

// REVISAR (el suma): «el suma partido por mayúscula s de i»
#[test]
#[ignore = "lectura en español pendiente de revisión"]
fn sum_sub() -> Result<()> {
    let expr = "<math>
        <msub>
            <mo>∑</mo>
            <mi>S</mi>
        </msub>
        <mi>i</mi>
    </math>";
    test("es", "SimpleSpeak", expr, "el suma partido por mayúscula s de i")?;
    return Ok(());

}

// REVISAR (el suma): «el suma de a subíndice i»
#[test]
#[ignore = "lectura en español pendiente de revisión"]
fn sum() -> Result<()> {
    let expr = "<math>
            <mo>∑</mo>
            <msub><mi>a</mi><mi>i</mi></msub>
    </math>";
    test("es", "SimpleSpeak", expr, "el suma de a subíndice i")?;
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
    test("es", "SimpleSpeak", expr, "el producto de n es igual a 1 a 10 de n")?;
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
    test("es", "SimpleSpeak", expr, "el producto partido por mayúscula s de i")?;
    return Ok(());

}

#[test]
fn product() -> Result<()> {
    let expr = "<math>
            <mo>∏</mo>
            <msub><mi>a</mi><mi>i</mi></msub>
    </math>";
    test("es", "SimpleSpeak", expr, "el producto de a subíndice i")?;
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
    test("es", "SimpleSpeak", expr, "el intersección de i es igual a 1 a 10 de; mayúscula s subíndice i")?;
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
    test("es", "SimpleSpeak", expr, "el intersección partido por mayúscula c de; mayúscula s subíndice i")?;
    return Ok(());

}

#[test]
fn intersection() -> Result<()> {
    let expr = "<math>
            <mo>⋂</mo>
            <msub><mi>S</mi><mi>i</mi></msub>
            </math>";
    test("es", "SimpleSpeak", expr, "el intersección de mayúscula s subíndice i")?;
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
    test("es", "SimpleSpeak", expr, "el unión de i es igual a 1 a 10 de; mayúscula s subíndice i")?;
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
    test("es", "SimpleSpeak", expr, "el unión partido por mayúscula c de; mayúscula s subíndice i")?;
    return Ok(());

}

#[test]
fn union() -> Result<()> {
    let expr = "<math>
            <mo>⋃</mo>
            <msub><mi>S</mi><mi>i</mi></msub>
            </math>";
    test("es", "SimpleSpeak", expr, "el unión de mayúscula s subíndice i")?;
    return Ok(());

}

// REVISAR (el integral): «el integral de 0 a 1 de, f de x; d x»
#[test]
#[ignore = "lectura en español pendiente de revisión"]
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
    test("es", "SimpleSpeak", expr, "el integral de 0 a 1 de, f de x; d x")?;
    return Ok(());

}

// REVISAR (el integral): «el integral partido por números reales de; f de x d x»
#[test]
#[ignore = "lectura en español pendiente de revisión"]
fn integral_under() -> Result<()> {
    let expr = "<math>
        <munder>
            <mo>∫</mo>
            <mi>ℝ</mi>
        </munder>
        <mrow><mi>f</mi><mrow><mo>(</mo><mi>x</mi> <mo>)</mo></mrow></mrow>
        <mi>d</mi><mi>x</mi>
        </math>";
    test("es", "SimpleSpeak", expr, "el integral partido por números reales de; f de x d x")?;
    return Ok(());

}

// REVISAR (el integral): «el integral de f de x d x»
#[test]
#[ignore = "lectura en español pendiente de revisión"]
fn integral() -> Result<()> {
    let expr = "<math>
            <mo>∫</mo>
            <mrow><mi>f</mi><mrow><mo>(</mo><mi>x</mi> <mo>)</mo></mrow></mrow>
            <mi>d</mi><mi>x</mi>
            </math>";
    test("es", "SimpleSpeak", expr, "el integral de f de x d x")?;
    return Ok(());

}