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
    test("el", "ClearSpeak", expr, "1 δεύτερο")?;
    return Ok(());

}

#[test]
fn common_fraction_thirds() -> Result<()> {
    let expr = "<math>
                    <mfrac> <mn>2</mn> <mn>3</mn> </mfrac>
                </math>";
    test("el", "ClearSpeak", expr, "2 τρίτα")?;
    return Ok(());

}

#[test]
fn common_fraction_tenths() -> Result<()> {
    let expr = "<math>
                    <mfrac> <mn>17</mn> <mn>10</mn> </mfrac>
                </math>";
    test_prefs("el", "ClearSpeak", vec![("Verbosity", "Medium"), ("ClearSpeak_Fractions", "Auto")], expr, "17 δέκατα")?;
    test_prefs("el", "ClearSpeak", vec![("Verbosity", "Medium"), ("ClearSpeak_Fractions", "Ordinal")], expr, "17 δέκατα")?;
    return Ok(());

}

#[test]
#[allow(non_snake_case)]
fn not_ClearSpeak_common_fraction_tenths() -> Result<()> {
    let expr = "<math>
                    <mfrac> <mn>89</mn> <mn>10</mn> </mfrac>
                </math>";
    test_prefs("el", "ClearSpeak", vec![("Verbosity", "Medium"), ("ClearSpeak_Fractions", "Auto")], expr, "89 προς 10")?;
    test_prefs("el", "ClearSpeak", vec![("Verbosity", "Medium"), ("ClearSpeak_Fractions", "Ordinal")], expr, "89 δέκατα")?;
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
     test_prefs("el", "ClearSpeak", vec![("Verbosity", "Medium"), ("ClearSpeak_Fractions", "Auto")], expr, "το κλάσμα με αριθμητή; x συν y; και παρονομαστή x μείον y")?;
     test_prefs("el", "ClearSpeak", vec![("Verbosity", "Medium"), ("ClearSpeak_Fractions", "Ordinal")], expr, "το κλάσμα με αριθμητή; x συν y; και παρονομαστή x μείον y")?;
     test_prefs("el", "ClearSpeak", vec![("Verbosity", "Medium"), ("ClearSpeak_Fractions", "Over")], expr, "x συν y προς x μείον y")?;
     test_prefs("el", "ClearSpeak", vec![("Verbosity", "Medium"), ("ClearSpeak_Fractions", "FracOver")], expr, "το κλάσμα x συν y προς x μείον y")?;
     test_prefs("el", "ClearSpeak", vec![("Verbosity", "Medium"), ("ClearSpeak_Fractions", "General")], expr, "το κλάσμα με αριθμητή; x συν y; και παρονομαστή x μείον y")?;
     test_prefs("el", "ClearSpeak", vec![("Verbosity", "Medium"), ("ClearSpeak_Fractions", "EndFrac")], expr, "το κλάσμα με αριθμητή; x συν y; και παρονομαστή x μείον y; τέλος κλάσματος")?;
     test_prefs("el", "ClearSpeak", vec![("Verbosity", "Medium"), ("ClearSpeak_Fractions", "GeneralEndFrac")], expr, "το κλάσμα με αριθμητή; x συν y; και παρονομαστή x μείον y; τέλος κλάσματος")?;
     test_prefs("el", "ClearSpeak", vec![("Verbosity", "Medium"), ("ClearSpeak_Fractions", "OverEndFrac")], expr, "x συν y προς x μείον y, τέλος κλάσματος")?;
     test_prefs("el", "ClearSpeak", vec![("Verbosity", "Medium"), ("ClearSpeak_Fractions", "Per")], expr, "x συν y ανά x μείον y")?;
     test_prefs("el", "ClearSpeak", vec![("Verbosity", "Verbose"),("ClearSpeak_Fractions", "Auto")], expr, "το κλάσμα με αριθμητή; x συν y; και παρονομαστή x μείον y; τέλος κλάσματος")?;
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
    test("el", "ClearSpeak", expr, "62 μίλια ανά ώρα")?;
    return Ok(());

}


#[test]
fn mixed_number() -> Result<()> {
    let expr = "<math>
                    <mn>3</mn>
                    <mfrac> <mn>1</mn> <mn>2</mn> </mfrac>
                </math>";
    test("el", "ClearSpeak", expr, "3 και 1 δεύτερο")?;
    return Ok(());

}

#[test]
fn explicit_mixed_number() -> Result<()> {
    let expr = "<math>
                    <mn>3</mn>
                    <mo>&#x2064;</mo>
                    <mfrac> <mn>1</mn> <mn>8</mn> </mfrac>
                </math>";
    test("el", "ClearSpeak", expr, "3 και 1 όγδοο")?;
    return Ok(());

}

#[test]
fn mixed_number_big() -> Result<()> {
    let expr = "<math>
                    <mn>3</mn>
                    <mfrac> <mn>7</mn> <mn>83</mn> </mfrac>
                </math>";
    test("el", "ClearSpeak", expr, "3 και 7 προς 83")?;
    return Ok(());

}

#[test]
fn simple_text() -> Result<()> {
    let expr = "<math>
    <mfrac> <mi>rise</mi> <mi>run</mi> </mfrac>
                </math>";
    test("el", "ClearSpeak", expr, "rise προς run")?;
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
    test("el", "ClearSpeak", expr, "2 miles προς 3 gallons")?;
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
    test_prefs("el", "ClearSpeak", vec![("ClearSpeak_Fractions", "Auto")], expr, "1 δεύτερο προς 2 τρίτα")?;
    test_prefs("el", "ClearSpeak", vec![("ClearSpeak_Fractions", "Ordinal")], expr, "1 δεύτερο προς 2 τρίτα")?;
    test_prefs("el", "ClearSpeak", vec![("ClearSpeak_Fractions", "Over")], expr, "1 προς 2 προς 2 προς 3")?;
    test_prefs("el", "ClearSpeak", vec![("ClearSpeak_Fractions", "FracOver")], expr,
            "το κλάσμα το κλάσμα 1 προς 2 προς το κλάσμα 2 προς 3")?;
    test_prefs("el", "ClearSpeak", vec![("ClearSpeak_Fractions", "General")], expr,
            "το κλάσμα με αριθμητή το κλάσμα με αριθμητή 1; και παρονομαστή 2; και παρονομαστή το κλάσμα με αριθμητή 2; και παρονομαστή 3")?;
    test_prefs("el", "ClearSpeak", vec![("ClearSpeak_Fractions", "EndFrac")], expr, "1 δεύτερο προς 2 τρίτα")?;
    test_prefs("el", "ClearSpeak", vec![("ClearSpeak_Fractions", "GeneralEndFrac")], expr,
            "το κλάσμα με αριθμητή το κλάσμα με αριθμητή 1; και παρονομαστή 2; τέλος κλάσματος; και παρονομαστή το κλάσμα με αριθμητή 2; και παρονομαστή 3; τέλος κλάσματος; τέλος κλάσματος")?;
    test_prefs("el", "ClearSpeak", vec![("ClearSpeak_Fractions", "OverEndFrac")], expr,
            "1 προς 2, τέλος κλάσματος, προς 2 προς 3, τέλος κλάσματος; τέλος κλάσματος")?;
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
    test("el", "ClearSpeak", expr, "2 τρίτα x προς 6")?;
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
    test("el", "ClearSpeak", expr, "το κλάσμα με αριθμητή; 10 προς n; και παρονομαστή 2 προς n")?;
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
    test("el", "ClearSpeak", expr, "το κλάσμα με αριθμητή; το κλάσμα με αριθμητή; n συν 10; και παρονομαστή n; και παρονομαστή 2 προς n")?;
    return Ok(());

}

#[test]
fn simple_function() -> Result<()> {
    let expr = "<math><mfrac><mrow><mi>f</mi><mo>(</mo><mi>x</mi><mo>)</mo></mrow><mn>2</mn></mfrac></math>";
    test_prefs("el", "ClearSpeak", vec![("Verbosity", "Medium"), ("ClearSpeak_Fractions", "Auto")], expr, "f του x προς 2")?;
    test_prefs("el", "ClearSpeak", vec![("Verbosity", "Verbose"), ("ClearSpeak_Fractions", "Auto")], expr, "f του x προς 2, τέλος κλάσματος")?;
    return Ok(());

}

#[test]
fn function_over_function() -> Result<()> {
    let expr = "<math><mfrac>
            <mrow><mi>f</mi><mo>(</mo><mi>x</mi><mo>)</mo></mrow>
            <mrow><mi>g</mi><mo>(</mo><mi>x</mi><mo>)</mo></mrow>
        </mfrac></math>";
    test_prefs("el", "ClearSpeak", vec![("Verbosity", "Medium"), ("ClearSpeak_Fractions", "Auto")], expr, "f του x προς g του x")?;
    test_prefs("el", "ClearSpeak", vec![("Verbosity", "Verbose"), ("ClearSpeak_Fractions", "Auto")], expr, "f του x προς g του x, τέλος κλάσματος")?;
    return Ok(());

}

#[test]
fn non_simple_function_over_function() -> Result<()> {
    let expr = "<math><mfrac>
            <mrow><mi>f</mi><mo>(</mo><mi>x</mi><mo>+</mo><mn>1</mn><mo>)</mo></mrow>
            <mrow><mi>g</mi><mo>(</mo><mi>x</mi><mo>)</mo></mrow>
        </mfrac></math>";
    test_prefs("el", "ClearSpeak", vec![("Verbosity", "Medium"), ("ClearSpeak_Fractions", "Auto")], expr,
             "το κλάσμα με αριθμητή; f του; ανοίγει παρένθεση, x συν 1, κλείνει παρένθεση; και παρονομαστή g του x")?;
    test_prefs("el", "ClearSpeak", vec![("Verbosity", "Verbose"), ("ClearSpeak_Fractions", "Auto")], expr,
             "το κλάσμα με αριθμητή; f του; ανοίγει παρένθεση, x συν 1, κλείνει παρένθεση; και παρονομαστή g του x; τέλος κλάσματος")?;
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
    test("el", "ClearSpeak", expr, "2 επί 7 ανά 3")?;
    return Ok(());
    //theodora. fails because of the implicit "times" rule in Clearspeak which is left blank due to many false positives

}
