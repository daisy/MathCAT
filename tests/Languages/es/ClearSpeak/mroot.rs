use crate::common::*;
use anyhow::Result;

// REVISAR (el raíz): «el raíz cuadrada de x»
#[test]
#[ignore = "lectura en español pendiente de revisión"]
fn msqrt_simple() -> Result<()> {
    let expr = "<math>
                    <msqrt> <mi>x</mi> </msqrt>
                </math>";
    test("es", "ClearSpeak", expr, "el raíz cuadrada de x")?;
    return Ok(());

}

// REVISAR (el raíz): «el raíz cuadrada de x, fin de raíz»
#[test]
#[ignore = "lectura en español pendiente de revisión"]
fn msqrt_simple_end_root() -> Result<()> {
    let expr = "<math>
                    <msqrt> <mi>x</mi> </msqrt>
                </math>";
    test_ClearSpeak("es", "ClearSpeak_Roots", "RootEnd", expr, "el raíz cuadrada de x, fin de raíz")?;
    return Ok(());

}

#[test]
fn msqrt_simple_positive() -> Result<()> {
    let expr = "<math>
                    <msqrt> <mi>x</mi> </msqrt>
                </math>";
    test_ClearSpeak("es", "ClearSpeak_Roots", "PosNegSqRoot", expr, "el positivo raíz cuadrada de x")?;
    return Ok(());

}

#[test]
fn msqrt_simple_pos_end_root() -> Result<()> {
    let expr = "<math>
                    <msqrt> <mi>x</mi> </msqrt>
                </math>";
    test_ClearSpeak("es", "ClearSpeak_Roots", "PosNegSqRootEnd", expr, "el positivo raíz cuadrada de x, fin de raíz")?;
    return Ok(());

}

#[test]
fn msqrt_simple_pos_end_with_neg_root() -> Result<()> {
    let expr = "<math>
                    <mo>-</mo> <msqrt> <mi>x</mi> </msqrt>
                    <mo>-</mo> <mroot> <mi>x</mi> <mn>3</mn></mroot>
                </math>";
    test_ClearSpeak("es", "ClearSpeak_Roots", "PosNegSqRootEnd", expr, 
    "el menos raíz cuadrada de x, fin de raíz; menos, el positivo ra\"íz cúbica\" de x, fin de raíz")?;
    return Ok(());

}

#[test]
fn mroot_simple_pos_end_with_neg_root() -> Result<()> {
    let expr = "<math>
                    <mo>-</mo> <mroot> <mi>x</mi> <mn>3</mn></mroot>
                    <mo>-</mo> <msqrt> <mi>x</mi> </msqrt>

                </math>";
    test_ClearSpeak("es", "ClearSpeak_Roots", "PosNegSqRoot", expr, 
    "el menos ra\"íz cúbica\" de x; menos el positivo raíz cuadrada de x")?;
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

// REVISAR (el raíz): «el raíz cuadrada de x más y»
#[test]
#[ignore = "lectura en español pendiente de revisión"]
fn msqrt() -> Result<()> {
    let expr = "<math>
                    <msqrt>
                        <mrow> <mi>x</mi> <mo>+</mo> <mi>y</mi> </mrow>
                    </msqrt>
                </math>";
    test("es", "ClearSpeak", expr, "el raíz cuadrada de x más y")?;
    return Ok(());

}

// REVISAR (el raíz): «el raíz cuadrada de x»
#[test]
#[ignore = "lectura en español pendiente de revisión"]
fn mroot_as_square_root() -> Result<()> {
    let expr = "<math>
                    <mroot> <mi>x</mi> <mn>2</mn> </mroot>
                </math>";
    test("es", "ClearSpeak", expr, "el raíz cuadrada de x")?;
    return Ok(());

}

#[test]
fn cube_root() -> Result<()> {
    let expr = "<math>
                    <mroot> <mi>x</mi> <mn>3</mn> </mroot>
                </math>";
    test("es", "ClearSpeak", expr, "el ra\"íz cúbica\" de x")?;
    return Ok(());

}

#[test]
fn ordinal_root() -> Result<()> {
    let expr = "<math>
                    <mroot> <mi>x</mi> <mn>9</mn> </mroot>
                </math>";
    test("es", "ClearSpeak", expr, "el noveno raíz de x")?;
    return Ok(());

}

// REVISAR (-th, inglés: th): «el n-th raíz de x»
#[test]
#[ignore = "lectura en español pendiente de revisión"]
fn simple_mi_root() -> Result<()> {
    let expr = "<math>
                    <mroot> <mi>x</mi> <mi>n</mi> </mroot>
                </math>";
    test("es", "ClearSpeak", expr, "el n-th raíz de x")?;
    return Ok(());

}

// REVISAR (-th, inglés: th): «el positivo t-th raíz de x, fin de raíz»
#[test]
#[ignore = "lectura en español pendiente de revisión"]
fn mroot_simple_pos_end_root() -> Result<()> {
    let expr = "<math>
                <mroot> <mi>x</mi> <mi>t</mi> </mroot>
                </math>";
    test_ClearSpeak("es", "ClearSpeak_Roots", "PosNegSqRootEnd", expr, "el positivo t-th raíz de x, fin de raíz")?;
    return Ok(());

}

#[test]
fn mroot_simple_end_root() -> Result<()> {
    let expr = "<math>
                    <mroot> <mrow> <mi>x</mi> <mo>+</mo> <mi>y</mi> </mrow> 
                    <mn>21</mn></mroot>
                </math>";
    test_ClearSpeak("es", "ClearSpeak_Roots", "RootEnd", expr, "el veinte primero raíz de x más y, fin de raíz")?;
    return Ok(());

}

// REVISAR (tercero): «el 1 tercero raíz de x»
#[test]
#[ignore = "lectura en español pendiente de revisión"]
fn simple_fraction_power() -> Result<()> {
    let expr = "<math>
                    <mroot>
                        <mi>x</mi> 
                        <mfrac><mn>1</mn><mn>3</mn></mfrac>
                    </mroot>
                </math>";
    test("es", "ClearSpeak", expr, "el 1 tercero raíz de x")?;
    return Ok(());

}

// REVISAR (el raíz): «el ra"íz cúbica" de el raíz cuadrada de 42»
#[test]
#[ignore = "lectura en español pendiente de revisión"]
fn no_double_the_532() -> Result<()> {
    let expr = "<math><mroot><msqrt><mn>42</mn></msqrt><mn>3</mn></mroot></math>";
    test("es", "ClearSpeak", expr, "el ra\"íz cúbica\" de el raíz cuadrada de 42")?;
    return Ok(());

}
