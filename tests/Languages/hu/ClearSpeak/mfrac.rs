/// Tests for fractions
///   includes simple fractions and more complex fractions
///   also tests mixed fractions (implicit and explicit)
use crate::common::*;
use anyhow::Result;

// AI generated
#[test]
fn common_fraction_half() -> Result<()> {
    let expr = "<math>
                    <mfrac> <mn>1</mn> <mn>2</mn> </mfrac>
                </math>";
    test("hu", "ClearSpeak", expr, "1 ketted")?;
    return Ok(());

}

// AI generated
#[test]
fn common_fraction_thirds() -> Result<()> {
    let expr = "<math>
                    <mfrac> <mn>2</mn> <mn>3</mn> </mfrac>
                </math>";
    test("hu", "ClearSpeak", expr, "2 harmad")?;
    return Ok(());

}

// AI generated
#[test]
fn common_fraction_tenths() -> Result<()> {
    let expr = "<math>
                    <mfrac> <mn>17</mn> <mn>10</mn> </mfrac>
                </math>";
    test_prefs("hu", "ClearSpeak", vec![("Verbosity", "Medium"), ("ClearSpeak_Fractions", "Auto")], expr, "17 tized")?;
    test_prefs("hu", "ClearSpeak", vec![("Verbosity", "Medium"), ("ClearSpeak_Fractions", "Ordinal")], expr, "17 tized")?;
    return Ok(());

}

// AI generated
#[test]
#[allow(non_snake_case)]
fn not_ClearSpeak_common_fraction_tenths() -> Result<()> {
    let expr = "<math>
                    <mfrac> <mn>89</mn> <mn>10</mn> </mfrac>
                </math>";
    test_prefs("hu", "ClearSpeak", vec![("Verbosity", "Medium"), ("ClearSpeak_Fractions", "Auto")], expr, "89 per 10")?;
    test_prefs("hu", "ClearSpeak", vec![("Verbosity", "Medium"), ("ClearSpeak_Fractions", "Ordinal")], expr, "89 tized")?;
    return Ok(());

}

// AI generated
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
    test_prefs("hu", "ClearSpeak", vec![("Verbosity", "Medium"), ("ClearSpeak_Fractions", "Auto")], expr, "a tört, melynek számlálója; x plusz y; nevezője pedig x mínusz y")?;
    test_prefs("hu", "ClearSpeak", vec![("Verbosity", "Medium"), ("ClearSpeak_Fractions", "Ordinal")], expr, "a tört, melynek számlálója; x plusz y; nevezője pedig x mínusz y")?;
    test_prefs("hu", "ClearSpeak", vec![("Verbosity", "Medium"), ("ClearSpeak_Fractions", "Over")], expr, "x plusz y per x mínusz y")?;
    test_prefs("hu", "ClearSpeak", vec![("Verbosity", "Medium"), ("ClearSpeak_Fractions", "FracOver")], expr, "a tört x plusz y per x mínusz y")?;
    test_prefs("hu", "ClearSpeak", vec![("Verbosity", "Medium"), ("ClearSpeak_Fractions", "General")], expr, "a tört, melynek számlálója; x plusz y; nevezője pedig x mínusz y")?;
    test_prefs("hu", "ClearSpeak", vec![("Verbosity", "Medium"), ("ClearSpeak_Fractions", "EndFrac")], expr, "a tört, melynek számlálója; x plusz y; nevezője pedig x mínusz y; tört vége")?;
    test_prefs("hu", "ClearSpeak", vec![("Verbosity", "Medium"), ("ClearSpeak_Fractions", "GeneralEndFrac")], expr, "a tört, melynek számlálója; x plusz y; nevezője pedig x mínusz y; tört vége")?;
    test_prefs("hu", "ClearSpeak", vec![("Verbosity", "Medium"), ("ClearSpeak_Fractions", "OverEndFrac")], expr, "x plusz y per x mínusz y, tört vége")?;
    test_prefs("hu", "ClearSpeak", vec![("Verbosity", "Medium"), ("ClearSpeak_Fractions", "Per")], expr, "x plusz y per x mínusz y")?;
    test_prefs("hu", "ClearSpeak", vec![("Verbosity", "Verbose"),("ClearSpeak_Fractions", "Auto")], expr, "a tört, melynek számlálója; x plusz y; nevezője pedig x mínusz y; tört vége")?;
    return Ok(());

}

// AI generated
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
    test("hu", "ClearSpeak", expr, "62 mérföld per óra")?;
    return Ok(());

}


// AI generated
#[test]
fn mixed_number() -> Result<()> {
    let expr = "<math>
                    <mn>3</mn>
                    <mfrac> <mn>1</mn> <mn>2</mn> </mfrac>
                </math>";
    test("hu", "ClearSpeak", expr, "3 és 1 ketted")?;
    return Ok(());

}

// AI generated
#[test]
fn explicit_mixed_number() -> Result<()> {
    let expr = "<math>
                    <mn>3</mn>
                    <mo>&#x2064;</mo>
                    <mfrac> <mn>1</mn> <mn>8</mn> </mfrac>
                </math>";
    test("hu", "ClearSpeak", expr, "3 és 1 nyolcad")?;
    return Ok(());

}

// AI generated
#[test]
fn mixed_number_big() -> Result<()> {
    let expr = "<math>
                    <mn>3</mn>
                    <mfrac> <mn>7</mn> <mn>83</mn> </mfrac>
                </math>";
    test("hu", "ClearSpeak", expr, "3 és 7 per 83")?;
    return Ok(());

}

// AI generated
#[test]
fn simple_text() -> Result<()> {
    let expr = "<math>
    <mfrac> <mi>rise</mi> <mi>run</mi> </mfrac>
                </math>";
    test("hu", "ClearSpeak", expr, "rise per run")?;
    return Ok(());

}

// AI generated
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
    test("hu", "ClearSpeak", expr, "2 miles per 3 gallons")?;
    return Ok(());

}


// AI generated
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
    test_prefs("hu", "ClearSpeak", vec![("ClearSpeak_Fractions", "Auto")], expr, "1 ketted per 2 harmad")?;
    test_prefs("hu", "ClearSpeak", vec![("ClearSpeak_Fractions", "Ordinal")], expr, "1 ketted per 2 harmad")?;
    test_prefs("hu", "ClearSpeak", vec![("ClearSpeak_Fractions", "Over")], expr, "1 per 2 per 2 per 3")?;
    test_prefs("hu", "ClearSpeak", vec![("ClearSpeak_Fractions", "FracOver")], expr,
            "a tört a tört 1 per 2 per a tört 2 per 3")?;
    test_prefs("hu", "ClearSpeak", vec![("ClearSpeak_Fractions", "General")], expr,
            "a tört, melynek számlálója tört, melynek számlálója 1; nevezője pedig 2; nevezője pedig a tört, melynek számlálója 2; nevezője pedig 3")?;
    test_prefs("hu", "ClearSpeak", vec![("ClearSpeak_Fractions", "EndFrac")], expr, "1 ketted per 2 harmad")?;
    test_prefs("hu", "ClearSpeak", vec![("ClearSpeak_Fractions", "GeneralEndFrac")], expr,
            "a tört, melynek számlálója tört, melynek számlálója 1; nevezője pedig 2; tört vége; nevezője pedig a tört, melynek számlálója 2; nevezője pedig 3; tört vége; tört vége")?;
    test_prefs("hu", "ClearSpeak", vec![("ClearSpeak_Fractions", "OverEndFrac")], expr,
            "1 per 2, tört vége, per 2 per 3, tört vége; tört vége")?;
            return Ok(());

}


// AI generated
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
    test("hu", "ClearSpeak", expr, "2 harmad x per 6")?;
    return Ok(());

}

// AI generated
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
    test("hu", "ClearSpeak", expr, "a tört, melynek számlálója; 10 per n; nevezője pedig 2 per n")?;
    return Ok(());

}

// AI generated
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
    test("hu", "ClearSpeak", expr, "a tört, melynek számlálója; a tört, melynek számlálója; n plusz 10; nevezője pedig n; nevezője pedig 2 per n")?;
    return Ok(());

}

// AI generated
#[test]
fn simple_function() -> Result<()> {
    let expr = "<math><mfrac><mrow><mi>f</mi><mo>(</mo><mi>x</mi><mo>)</mo></mrow><mn>2</mn></mfrac></math>";
    test_prefs("hu", "ClearSpeak", vec![("Verbosity", "Medium"), ("ClearSpeak_Fractions", "Auto")], expr, "f x per 2")?;
    test_prefs("hu", "ClearSpeak", vec![("Verbosity", "Verbose"), ("ClearSpeak_Fractions", "Auto")], expr, "f x per 2, tört vége")?;
    return Ok(());

}

// AI generated
#[test]
fn function_over_function() -> Result<()> {
    let expr = "<math><mfrac>
            <mrow><mi>f</mi><mo>(</mo><mi>x</mi><mo>)</mo></mrow>
            <mrow><mi>g</mi><mo>(</mo><mi>x</mi><mo>)</mo></mrow>
        </mfrac></math>";
    test_prefs("hu", "ClearSpeak", vec![("Verbosity", "Medium"), ("ClearSpeak_Fractions", "Auto")], expr, "f x per g x")?;
    test_prefs("hu", "ClearSpeak", vec![("Verbosity", "Verbose"), ("ClearSpeak_Fractions", "Auto")], expr, "f x per g x, tört vége")?;
    return Ok(());

}

// AI generated
#[test]
fn non_simple_function_over_function() -> Result<()> {
    let expr = "<math><mfrac>
            <mrow><mi>f</mi><mo>(</mo><mi>x</mi><mo>+</mo><mn>1</mn><mo>)</mo></mrow>
            <mrow><mi>g</mi><mo>(</mo><mi>x</mi><mo>)</mo></mrow>
        </mfrac></math>";
    test_prefs("hu", "ClearSpeak", vec![("Verbosity", "Medium"), ("ClearSpeak_Fractions", "Auto")], expr,
             "a tört, melynek számlálója; f, nyitott zárójel, x plusz 1, zárt zárójel; nevezője pedig g x")?;
    test_prefs("hu", "ClearSpeak", vec![("Verbosity", "Verbose"), ("ClearSpeak_Fractions", "Auto")], expr,
             "a tört, melynek számlálója; f, nyitott zárójel, x plusz 1, zárt zárójel; nevezője pedig g x; tört vége")?;
             return Ok(());

}

// AI generated
#[test]
fn binomial() -> Result<()> {
    let expr = "<math>
                    <mn>2</mn>
                    <mo>(</mo>
                    <mfrac linethickness='0'> <mn>7</mn> <mn>3</mn> </mfrac>
                    <mo>)</mo>
                </math>";
    test("hu", "ClearSpeak", expr, "2 szorozva 7 a 3")?;
    return Ok(());

}
