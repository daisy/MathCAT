use crate::common::*;
use anyhow::Result;

#[test]
fn msqrt_simple() -> Result<()> {
    let expr = "<math>
                    <msqrt> <mi>x</mi> </msqrt>
                </math>";
    test("el", "ClearSpeak", expr, "η τετραγωνική ρίζα του x")?;
    return Ok(());

}

#[test]
fn msqrt_simple_end_root() -> Result<()> {
    let expr = "<math>
                    <msqrt> <mi>x</mi> </msqrt>
                </math>";
    test_ClearSpeak("el", "ClearSpeak_Roots", "RootEnd", expr, "η τετραγωνική ρίζα του x, τέλος ρίζας")?;
    return Ok(());

}

#[test]
fn msqrt_simple_positive() -> Result<()> {
    let expr = "<math>
                    <msqrt> <mi>x</mi> </msqrt>
                </math>";
    test_ClearSpeak("el", "ClearSpeak_Roots", "PosNegSqRoot", expr, "η συν τετραγωνική ρίζα του x")?;
    return Ok(());

}

#[test]
fn msqrt_simple_pos_end_root() -> Result<()> {
    let expr = "<math>
                    <msqrt> <mi>x</mi> </msqrt>
                </math>";
    test_ClearSpeak("el", "ClearSpeak_Roots", "PosNegSqRootEnd", expr, "η συν τετραγωνική ρίζα του x, τέλος ρίζας")?;
    return Ok(());

}

#[test]
fn msqrt_simple_pos_end_with_neg_root() -> Result<()> {
    let expr = "<math>
                    <mo>-</mo> <msqrt> <mi>x</mi> </msqrt>
                    <mo>-</mo> <mroot> <mi>x</mi> <mn>3</mn></mroot>
                </math>";
    test_ClearSpeak("el", "ClearSpeak_Roots", "PosNegSqRootEnd", expr, 
    "η μείον τετραγωνική ρίζα του x, τέλος ρίζας; μείον, η συν κυβική ρίζα του x, τέλος ρίζας")?;
    return Ok(());

}

#[test]
fn msqrt_simple_pos_end_with_neg_root_2() -> Result<()> {
    let expr = "<math xmlns='http://www.w3.org/1998/Math/MathML'><mi>x</mi><mo>=</mo><msqrt><mi>x</mi></msqrt><mo>-</mo><mroot><mi>x</mi><mn>3</mn></mroot></math>";
    test_ClearSpeak("el", "ClearSpeak_Roots", "PosNegSqRootEnd", expr, 
    "η μείον τετραγωνική ρίζα του x, τέλος ρίζας; μείον, η συν κυβική ρίζα του x, τέλος ρίζας")?;
    return Ok(());

}

#[test]
fn mroot_simple_pos_end_with_neg_root() -> Result<()> {
    let expr = "<math>
                    <mo>-</mo> <mroot> <mi>x</mi> <mn>3</mn></mroot>
                    <mo>-</mo> <msqrt> <mi>x</mi> </msqrt>

                </math>";
    test_ClearSpeak("el", "ClearSpeak_Roots", "PosNegSqRoot", expr, 
    "η μείον κυβική ρίζα του x; μείον, η συν τετραγωνική ρίζα του x")?;
    return Ok(());

}

#[test]
fn neg_without_root() -> Result<()> {
    let expr = "<math>
                    <mo>-</mo> <mi>x</mi> <mo>-</mo> <mi>y</mi>
                </math>";
    test("el", "ClearSpeak", expr, "μείον x μείον y")?;
    return Ok(());

}

#[test]
fn msqrt() -> Result<()> {
    let expr = "<math>
                    <msqrt>
                        <mrow> <mi>x</mi> <mo>+</mo> <mi>y</mi> </mrow>
                    </msqrt>
                </math>";
    test("el", "ClearSpeak", expr, "η τετραγωνική ρίζα του x συν y")?;
    return Ok(());

}

#[test]
fn mroot_as_square_root() -> Result<()> {
    let expr = "<math>
                    <mroot> <mi>x</mi> <mn>2</mn> </mroot>
                </math>";
    test("el", "ClearSpeak", expr, "η τετραγωνική ρίζα του x")?;
    return Ok(());

}

#[test]
fn cube_root() -> Result<()> {
    let expr = "<math>
                    <mroot> <mi>x</mi> <mn>3</mn> </mroot>
                </math>";
    test("el", "ClearSpeak", expr, "η κυβική ρίζα του x")?;
    return Ok(());

}

#[test]
fn ordinal_root() -> Result<()> {
    let expr = "<math>
                    <mroot> <mi>x</mi> <mn>9</mn> </mroot>
                </math>";
    test("el", "ClearSpeak", expr, "η ένατη ρίζα του x")?;
    return Ok(());

}

#[test]
fn simple_mi_root() -> Result<()> {
    let expr = "<math>
                    <mroot> <mi>x</mi> <mi>n</mi> </mroot>
                </math>";
    test("el", "ClearSpeak", expr, "η n οστή ρίζα του x")?;
    return Ok(());

}

#[test]
fn mroot_simple_pos_end_root() -> Result<()> {
    let expr = "<math>
                <mroot> <mi>x</mi> <mi>t</mi> </mroot>
                </math>";
    test_ClearSpeak("el", "ClearSpeak_Roots", "PosNegSqRootEnd", expr, "η συν t οστή ρίζα του x, τέλος ρίζας")?;
    return Ok(());

}

#[test]
fn mroot_simple_end_root() -> Result<()> {
    let expr = "<math>
                    <mroot> <mrow> <mi>x</mi> <mo>+</mo> <mi>y</mi> </mrow> 
                    <mn>21</mn></mroot>
                </math>";
    test_ClearSpeak("el", "ClearSpeak_Roots", "RootEnd", expr, "η εικοστή πρώτη ρίζα του x συν y, τέλος ρίζας")?;
    return Ok(());
// theodora. Now ok but ordinal numbers over 100 will have issues. The issue is likely in the src\xpath_functions.rs file, hundreds_to_words
}

#[test]
fn simple_fraction_power() -> Result<()> {
    let expr = "<math>
                    <mroot>
                        <mi>x</mi> 
                        <mfrac><mn>1</mn><mn>3</mn></mfrac>
                    </mroot>
                </math>";
    test("el", "ClearSpeak", expr, "η 1 τρίτο ρίζα του x")?;
    return Ok(());

}

#[test]
fn no_double_the_532() -> Result<()> {
    let expr = "<math><mroot><msqrt><mn>42</mn></msqrt><mn>3</mn></mroot></math>";
    test("el", "ClearSpeak", expr, "η κυβική ρίζα της τετραγωνικής ρίζας του 42")?;
    return Ok(());
 //theodora. fails with current rules. Should be fine when adding genitive
}
