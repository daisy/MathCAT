use crate::common::*;
use anyhow::Result;

#[test]
fn msqrt_simple() -> Result<()> {
    let expr = "<math>
                    <msqrt> <mi>x</mi> </msqrt>
                </math>";
    test("es", "ClearSpeak", expr, "la raíz cuadrada de x")?;
    return Ok(());

}

#[test]
fn msqrt_simple_end_root() -> Result<()> {
    let expr = "<math>
                    <msqrt> <mi>x</mi> </msqrt>
                </math>";
    test_ClearSpeak("es", "ClearSpeak_Roots", "RootEnd", expr, "la raíz cuadrada de x, fin de raíz")?;
    return Ok(());

}

#[test]
fn msqrt_simple_positive() -> Result<()> {
    let expr = "<math>
                    <msqrt> <mi>x</mi> </msqrt>
                </math>";
    test_ClearSpeak("es", "ClearSpeak_Roots", "PosNegSqRoot", expr, "la raíz cuadrada positiva de x")?;
    return Ok(());

}

#[test]
fn msqrt_simple_pos_end_root() -> Result<()> {
    let expr = "<math>
                    <msqrt> <mi>x</mi> </msqrt>
                </math>";
    test_ClearSpeak("es", "ClearSpeak_Roots", "PosNegSqRootEnd", expr, "la raíz cuadrada positiva de x, fin de raíz")?;
    return Ok(());

}

#[test]
fn msqrt_simple_pos_end_with_neg_root() -> Result<()> {
    let expr = "<math>
                    <mo>-</mo> <msqrt> <mi>x</mi> </msqrt>
                    <mo>-</mo> <mroot> <mi>x</mi> <mn>3</mn></mroot>
                </math>";
    test_ClearSpeak("es", "ClearSpeak_Roots", "PosNegSqRootEnd", expr, 
    "la raíz cuadrada negativa de x, fin de raíz; menos, la raíz cúbica positiva de x, fin de raíz")?;
    return Ok(());

}

#[test]
fn mroot_simple_pos_end_with_neg_root() -> Result<()> {
    let expr = "<math>
                    <mo>-</mo> <mroot> <mi>x</mi> <mn>3</mn></mroot>
                    <mo>-</mo> <msqrt> <mi>x</mi> </msqrt>

                </math>";
    test_ClearSpeak("es", "ClearSpeak_Roots", "PosNegSqRoot", expr, 
    "la raíz cúbica negativa de x; menos, la raíz cuadrada positiva de x")?;
    return Ok(());

}

#[test]
fn neg_without_root() -> Result<()> {
    let expr = "<math>
                    <mo>-</mo> <mi>x</mi> <mo>-</mo> <mi>y</mi>
                </math>";
    test("es", "ClearSpeak", expr, "menos x menos y")?;
    return Ok(());

}

#[test]
fn msqrt() -> Result<()> {
    let expr = "<math>
                    <msqrt>
                        <mrow> <mi>x</mi> <mo>+</mo> <mi>y</mi> </mrow>
                    </msqrt>
                </math>";
    test("es", "ClearSpeak", expr, "la raíz cuadrada de x más y")?;
    return Ok(());

}

#[test]
fn mroot_as_square_root() -> Result<()> {
    let expr = "<math>
                    <mroot> <mi>x</mi> <mn>2</mn> </mroot>
                </math>";
    test("es", "ClearSpeak", expr, "la raíz cuadrada de x")?;
    return Ok(());

}

#[test]
fn cube_root() -> Result<()> {
    let expr = "<math>
                    <mroot> <mi>x</mi> <mn>3</mn> </mroot>
                </math>";
    test("es", "ClearSpeak", expr, "la raíz cúbica de x")?;
    return Ok(());

}

#[test]
fn ordinal_root() -> Result<()> {
    let expr = "<math>
                    <mroot> <mi>x</mi> <mn>9</mn> </mroot>
                </math>";
    test("es", "ClearSpeak", expr, "la raíz novena de x")?;
    return Ok(());

}

#[test]
fn simple_mi_root() -> Result<()> {
    let expr = "<math>
                    <mroot> <mi>x</mi> <mi>n</mi> </mroot>
                </math>";
    test("es", "ClearSpeak", expr, "la raíz enésima de x")?;
    return Ok(());

}

#[test]
fn mroot_simple_pos_end_root() -> Result<()> {
    let expr = "<math>
                <mroot> <mi>x</mi> <mi>t</mi> </mroot>
                </math>";
    test_ClearSpeak("es", "ClearSpeak_Roots", "PosNegSqRootEnd", expr, "la raíz t-ésima positiva de x, fin de raíz")?;
    return Ok(());

}

#[test]
fn mroot_simple_end_root() -> Result<()> {
    let expr = "<math>
                    <mroot> <mrow> <mi>x</mi> <mo>+</mo> <mi>y</mi> </mrow> 
                    <mn>21</mn></mroot>
                </math>";
    test_ClearSpeak("es", "ClearSpeak_Roots", "RootEnd", expr, "la raíz de índice 21 de x más y, fin de raíz")?;
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
    test("es", "ClearSpeak", expr, "la raíz de índice un tercio de x")?;
    return Ok(());

}

#[test]
fn no_double_the_532() -> Result<()> {
    let expr = "<math><mroot><msqrt><mn>42</mn></msqrt><mn>3</mn></mroot></math>";
    test("es", "ClearSpeak", expr, "la raíz cúbica de la raíz cuadrada de 42")?;
    return Ok(());

}
