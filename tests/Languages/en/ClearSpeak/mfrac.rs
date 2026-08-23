/// Tests for fractions
///   includes simple fractions and more complex fractions
///   also tests mixed fractions (implicit and explicit)
use crate::common::*;
use anyhow::Result;

#[test]
fn common_fraction_half() -> Result<()> {
    let expr = "<math>
                    <mfrac> <mn>1</mn> <mn>2</mn> </mfrac>
                </math>";
    test("en", "ClearSpeak", expr, "1 half")?;
    return Ok(());

}

#[test]
fn common_fraction_thirds() -> Result<()> {
    let expr = "<math>
                    <mfrac> <mn>2</mn> <mn>3</mn> </mfrac>
                </math>";
    test("en", "ClearSpeak", expr, "2 thirds")?;
    return Ok(());

}

#[test]
fn common_fraction_tenths() -> Result<()> {
    let expr = "<math>
                    <mfrac> <mn>17</mn> <mn>10</mn> </mfrac>
                </math>";
    test_prefs("en", "ClearSpeak", vec![("Verbosity", "Medium"), ("ClearSpeak_Fractions", "Auto")], expr, "17 tenths")?;
    test_prefs("en", "ClearSpeak", vec![("Verbosity", "Medium"), ("ClearSpeak_Fractions", "Ordinal")], expr, "17 tenths")?;
    return Ok(());

}

#[test]
#[allow(non_snake_case)]
fn not_ClearSpeak_common_fraction_tenths() -> Result<()> {
    let expr = "<math>
                    <mfrac> <mn>89</mn> <mn>10</mn> </mfrac>
                </math>";
    test_prefs("en", "ClearSpeak", vec![("Verbosity", "Medium"), ("ClearSpeak_Fractions", "Auto")], expr, "89 over 10")?;
    test_prefs("en", "ClearSpeak", vec![("Verbosity", "Medium"), ("ClearSpeak_Fractions", "Ordinal")], expr, "89 tenths")?;
    return Ok(());

}

#[test]
fn non_simple_fraction() -> Result<()> {
    let expr = "
    <math>
        <mrow>
        <mfrac>
        <mrow>
        <mi>x</mi><mo>+</mo><mi>y</mi></mrow>
        <mrow>
        <mi>x</mi><mo>-</mo><mi>y</mi></mrow>
        </mfrac>
        </mrow>
    </math>";
    test_prefs("en", "ClearSpeak", vec![("Verbosity", "Medium"), ("ClearSpeak_Fractions", "Auto")], expr, "the fraction with numerator; x plus y; and denominator x minus y")?;
    test_prefs("en", "ClearSpeak", vec![("Verbosity", "Medium"), ("ClearSpeak_Fractions", "Ordinal")], expr, "the fraction with numerator; x plus y; and denominator x minus y")?;
    test_prefs("en", "ClearSpeak", vec![("Verbosity", "Medium"), ("ClearSpeak_Fractions", "Over")], expr, "x plus y over x minus y")?;
    test_prefs("en", "ClearSpeak", vec![("Verbosity", "Medium"), ("ClearSpeak_Fractions", "FracOver")], expr, "the fraction x plus y over x minus y")?;
    test_prefs("en", "ClearSpeak", vec![("Verbosity", "Medium"), ("ClearSpeak_Fractions", "General")], expr, "the fraction with numerator; x plus y; and denominator x minus y")?;
    test_prefs("en", "ClearSpeak", vec![("Verbosity", "Medium"), ("ClearSpeak_Fractions", "EndFrac")], expr, "the fraction with numerator; x plus y; and denominator x minus y; end fraction")?;
    test_prefs("en", "ClearSpeak", vec![("Verbosity", "Medium"), ("ClearSpeak_Fractions", "GeneralEndFrac")], expr, "the fraction with numerator; x plus y; and denominator x minus y; end fraction")?;
    test_prefs("en", "ClearSpeak", vec![("Verbosity", "Medium"), ("ClearSpeak_Fractions", "OverEndFrac")], expr, "x plus y over x minus y, end fraction")?;
    test_prefs("en", "ClearSpeak", vec![("Verbosity", "Medium"), ("ClearSpeak_Fractions", "Per")], expr, "x plus y per x minus y")?;
    test_prefs("en", "ClearSpeak", vec![("Verbosity", "Verbose"),("ClearSpeak_Fractions", "Auto")], expr, "the fraction with numerator; x plus y; and denominator x minus y; end fraction")?;
    return Ok(());

}

#[test]
fn frac_with_units() -> Result<()> {
    let expr = "
    <math>
        <mrow>
        <mn>62</mn>
        <mfrac>
        <mi intent=':unit'>mi</mi>
        <mi intent=':unit'>hr</mi>
        </mfrac>
        </mrow>
    </math>";
    test("en", "ClearSpeak", expr, "62 miles per hour")?;
    return Ok(());

}


#[test]
fn mixed_number() -> Result<()> {
    let expr = "<math>
                    <mn>3</mn>
                    <mfrac> <mn>1</mn> <mn>2</mn> </mfrac>
                </math>";
    test("en", "ClearSpeak", expr, "3 and 1 half")?;
    return Ok(());

}

#[test]
fn explicit_mixed_number() -> Result<()> {
    let expr = "<math>
                    <mn>3</mn>
                    <mo>&#x2064;</mo>
                    <mfrac> <mn>1</mn> <mn>8</mn> </mfrac>
                </math>";
    test("en", "ClearSpeak", expr, "3 and 1 eighth")?;
    return Ok(());

}

#[test]
fn mixed_number_big() -> Result<()> {
    let expr = "<math>
                    <mn>3</mn>
                    <mfrac> <mn>7</mn> <mn>83</mn> </mfrac>
                </math>";
    test("en", "ClearSpeak", expr, "3 and 7 over 83")?;
    return Ok(());

}

#[test]
fn simple_text() -> Result<()> {
    let expr = "<math>
    <mfrac> <mi>rise</mi> <mi>run</mi> </mfrac>
                </math>";
    test("en", "ClearSpeak", expr, "rise over run")?;
    return Ok(());

}

#[test]
fn number_and_text() -> Result<()> {
    let expr = "<math>
            <mfrac>
            <mrow>
                <mn>2</mn><mtext>miles</mtext></mrow>
            <mrow>
                <mn>3</mn><mtext>gallons</mtext></mrow>
            </mfrac>
        </math>";
    test("en", "ClearSpeak", expr, "2 miles over 3 gallons")?;
    return Ok(());

}


#[test]
fn nested_simple_fractions() -> Result<()> {
    let expr = "<math>
                <mrow>
                <mfrac>
                    <mrow>
                    <mfrac>
                        <mn>1</mn>
                        <mn>2</mn>
                    </mfrac>
                    </mrow>
                    <mrow>
                    <mfrac>
                        <mn>2</mn>
                        <mn>3</mn>
                    </mfrac>
                    </mrow>
                </mfrac>
                </mrow>
            </math>";
    test_prefs("en", "ClearSpeak", vec![("ClearSpeak_Fractions", "Auto")], expr, "1 half over 2 thirds")?;
    test_prefs("en", "ClearSpeak", vec![("ClearSpeak_Fractions", "Ordinal")], expr, "1 half over 2 thirds")?;
    test_prefs("en", "ClearSpeak", vec![("ClearSpeak_Fractions", "Over")], expr, "1 over 2 over 2 over 3")?;
    test_prefs("en", "ClearSpeak", vec![("ClearSpeak_Fractions", "FracOver")], expr,
            "the fraction the fraction 1 over 2 over the fraction 2 over 3")?;
    test_prefs("en", "ClearSpeak", vec![("ClearSpeak_Fractions", "General")], expr,
            "the fraction with numerator the fraction with numerator 1; and denominator 2; and denominator the fraction with numerator 2; and denominator 3")?;
    test_prefs("en", "ClearSpeak", vec![("ClearSpeak_Fractions", "EndFrac")], expr, "1 half over 2 thirds")?;
    test_prefs("en", "ClearSpeak", vec![("ClearSpeak_Fractions", "GeneralEndFrac")], expr,
            "the fraction with numerator the fraction with numerator 1; and denominator 2; end fraction; and denominator the fraction with numerator 2; and denominator 3; end fraction; end fraction")?;
    test_prefs("en", "ClearSpeak", vec![("ClearSpeak_Fractions", "OverEndFrac")], expr,
            "1 over 2, end fraction, over 2 over 3, end fraction; end fraction")?;
            return Ok(());

}


#[test]
fn semi_nested_fraction() -> Result<()> {
    let expr = "<math>
                <mrow>
                        <mfrac>
                        <mrow>
                        <mfrac>
                        <mn>2</mn>
                        <mn>3</mn>
                        </mfrac>
                        <mi>x</mi>
                    </mrow>
                    <mn>6</mn>
                    </mfrac>
                </mrow>
                </math>";
    test("en", "ClearSpeak", expr, "2 thirds x over 6")?;
    return Ok(());

}

#[test]
fn general_nested_fraction() -> Result<()> {
    let expr = "
    <math>
    <mrow>
        <mfrac>
        <mrow>
        <mfrac>
            <mn>10</mn>
            <mi>n</mi>
        </mfrac>
        </mrow>
        <mrow>
        <mfrac>
        <mn>2</mn>
        <mi>n</mi>
        </mfrac>
        </mrow>
        </mfrac>
        </mrow>
    </math>
                    ";
    test("en", "ClearSpeak", expr, "the fraction with numerator; 10 over n; and denominator 2 over n")?;
    return Ok(());

}

#[test]
fn complex_nested_fraction() -> Result<()> {
    let expr = "
    <math>
    <mrow>
        <mfrac>
        <mrow>
        <mfrac>
            <mrow> <mi>n</mi> <mo>+</mo> <mn>10</mn> </mrow>
            <mi>n</mi>
        </mfrac>
        </mrow>
        <mrow>
        <mfrac>
        <mn>2</mn>
        <mi>n</mi>
        </mfrac>
        </mrow>
        </mfrac>
        </mrow>
    </math>
                    ";
    test("en", "ClearSpeak", expr, "the fraction with numerator; the fraction with numerator; n plus 10; and denominator n; and denominator 2 over n")?;
    return Ok(());

}

#[test]
fn simple_function() -> Result<()> {
    let expr = "<math><mfrac><mrow><mi>f</mi><mo>(</mo><mi>x</mi><mo>)</mo></mrow><mn>2</mn></mfrac></math>";
    test_prefs("en", "ClearSpeak", vec![("Verbosity", "Medium"), ("ClearSpeak_Fractions", "Auto")], expr, "f of x over 2")?;
    test_prefs("en", "ClearSpeak", vec![("Verbosity", "Verbose"), ("ClearSpeak_Fractions", "Auto")], expr, "f of x over 2, end fraction")?;
    return Ok(());

}

#[test]
fn function_over_function() -> Result<()> {
    let expr = "<math><mfrac>
            <mrow><mi>f</mi><mo>(</mo><mi>x</mi><mo>)</mo></mrow>
            <mrow><mi>g</mi><mo>(</mo><mi>x</mi><mo>)</mo></mrow>
        </mfrac></math>";
    test_prefs("en", "ClearSpeak", vec![("Verbosity", "Medium"), ("ClearSpeak_Fractions", "Auto")], expr, "f of x over g of x")?;
    test_prefs("en", "ClearSpeak", vec![("Verbosity", "Verbose"), ("ClearSpeak_Fractions", "Auto")], expr, "f of x over g of x, end fraction")?;
    return Ok(());

}

#[test]
fn non_simple_function_over_function() -> Result<()> {
    let expr = "<math><mfrac>
            <mrow><mi>f</mi><mo>(</mo><mi>x</mi><mo>+</mo><mn>1</mn><mo>)</mo></mrow>
            <mrow><mi>g</mi><mo>(</mo><mi>x</mi><mo>)</mo></mrow>
        </mfrac></math>";
    test_prefs("en", "ClearSpeak", vec![("Verbosity", "Medium"), ("ClearSpeak_Fractions", "Auto")], expr,
             "the fraction with numerator; f of, open paren x plus 1, close paren; and denominator g of x")?;
    test_prefs("en", "ClearSpeak", vec![("Verbosity", "Verbose"), ("ClearSpeak_Fractions", "Auto")], expr,
             "the fraction with numerator; f of, open paren x plus 1, close paren; and denominator g of x; end fraction")?;
             return Ok(());

}

#[test]
fn binomial() -> Result<()> {
    let expr = "<math>
                    <mn>2</mn>
                    <mo>(</mo>
                    <mfrac linethickness='0'> <mn>7</mn> <mn>3</mn> </mfrac>
                    <mo>)</mo>
                </math>";
    test("en", "ClearSpeak", expr, "2 times 7 choose 3")?;
    return Ok(());

}
