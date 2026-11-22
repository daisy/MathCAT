use crate::common::*;
use anyhow::Result;

#[test]
fn msqrt_simple() -> Result<()> {
    let expr = "<math>
                    <msqrt> <mi>x</mi> </msqrt>
                </math>";
    test("pl", "ClearSpeak", expr, "pierwiastek kwadratowy z x")?;
    return Ok(());

}

#[test]
fn msqrt_simple_end_root() -> Result<()> {
    let expr = "<math>
                    <msqrt> <mi>x</mi> </msqrt>
                </math>";
    test_ClearSpeak("pl", "ClearSpeak_Roots", "RootEnd", expr, "pierwiastek kwadratowy z x, koniec pierwiastka")?;
    return Ok(());

}

#[test]
fn msqrt_simple_positive() -> Result<()> {
    let expr = "<math>
                    <msqrt> <mi>x</mi> </msqrt>
                </math>";
    test_ClearSpeak("pl", "ClearSpeak_Roots", "PosNegSqRoot", expr, "plus pierwiastek kwadratowy z x")?;
    return Ok(());

}

#[test]
fn msqrt_simple_pos_end_root() -> Result<()> {
    let expr = "<math>
                    <msqrt> <mi>x</mi> </msqrt>
                </math>";
    test_ClearSpeak("pl", "ClearSpeak_Roots", "PosNegSqRootEnd", expr, "plus pierwiastek kwadratowy z x, koniec pierwiastka")?;
    return Ok(());

}

#[test]
fn msqrt_simple_pos_end_with_neg_root() -> Result<()> {
    let expr = "<math>
                    <mo>-</mo> <msqrt> <mi>x</mi> </msqrt>
                    <mo>-</mo> <mroot> <mi>x</mi> <mn>3</mn></mroot>
                </math>";
    test_ClearSpeak("pl", "ClearSpeak_Roots", "PosNegSqRootEnd", expr, 
    "minus pierwiastek kwadratowy z x, koniec pierwiastka; minus, plus pierwiastek sześcienny z x, koniec pierwiastka")?;
    return Ok(());

}

#[test]
fn mroot_simple_pos_end_with_neg_root() -> Result<()> {
    let expr = "<math>
                    <mo>-</mo> <mroot> <mi>x</mi> <mn>3</mn></mroot>
                    <mo>-</mo> <msqrt> <mi>x</mi> </msqrt>

                </math>";
    test_ClearSpeak("pl", "ClearSpeak_Roots", "PosNegSqRoot", expr, 
    "minus pierwiastek sześcienny z x; minus plus pierwiastek kwadratowy z x")?;
    return Ok(());

}

#[test]
fn neg_without_root() -> Result<()> {
    let expr = "<math>
                    <mo>-</mo> <mi>x</mi> <mo>-</mo> <mi>y</mi>
                </math>";
    test("pl", "ClearSpeak", expr, "minus x minus y")?;
    return Ok(());

}

#[test]
fn msqrt() -> Result<()> {
    let expr = "<math>
                    <msqrt>
                        <mrow> <mi>x</mi> <mo>+</mo> <mi>y</mi> </mrow>
                    </msqrt>
                </math>";
    test("pl", "ClearSpeak", expr, "pierwiastek kwadratowy z x plus y")?;
    return Ok(());

}

#[test]
fn mroot_as_square_root() -> Result<()> {
    let expr = "<math>
                    <mroot> <mi>x</mi> <mn>2</mn> </mroot>
                </math>";
    test("pl", "ClearSpeak", expr, "pierwiastek kwadratowy z x")?;
    return Ok(());

}

#[test]
fn cube_root() -> Result<()> {
    let expr = "<math>
                    <mroot> <mi>x</mi> <mn>3</mn> </mroot>
                </math>";
    test("pl", "ClearSpeak", expr, "pierwiastek sześcienny z x")?;
    return Ok(());

}

#[test]
fn ordinal_root() -> Result<()> {
    let expr = "<math>
                    <mroot> <mi>x</mi> <mn>9</mn> </mroot>
                </math>";
    test("pl", "ClearSpeak", expr, "dziewiątej pierwiastek z x")?;
    return Ok(());

}

#[test]
fn simple_mi_root() -> Result<()> {
    let expr = "<math>
                    <mroot> <mi>x</mi> <mi>n</mi> </mroot>
                </math>";
    test("pl", "ClearSpeak", expr, "n pierwiastek z x")?;
    return Ok(());

}

#[test]
fn mroot_simple_pos_end_root() -> Result<()> {
    let expr = "<math>
                <mroot> <mi>x</mi> <mi>t</mi> </mroot>
                </math>";
    test_ClearSpeak("pl", "ClearSpeak_Roots", "PosNegSqRootEnd", expr, "plus t pierwiastek z x, koniec pierwiastka")?;
    return Ok(());

}

#[test]
fn mroot_simple_end_root() -> Result<()> {
    let expr = "<math>
                    <mroot> <mrow> <mi>x</mi> <mo>+</mo> <mi>y</mi> </mrow> 
                    <mn>21</mn></mroot>
                </math>";
    test_ClearSpeak("pl", "ClearSpeak_Roots", "RootEnd", expr, "dwadzieścia pierwszej pierwiastek z x plus y, koniec pierwiastka")?;
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
    test("pl", "ClearSpeak", expr, "jedna trzecia pierwiastek z x")?;
    return Ok(());

}

#[test]
fn no_double_the_532() -> Result<()> {
    let expr = "<math><mroot><msqrt><mn>42</mn></msqrt><mn>3</mn></mroot></math>";
    test("pl", "ClearSpeak", expr, "pierwiastek sześcienny z pierwiastek kwadratowy z 42")?;
    return Ok(());

}
