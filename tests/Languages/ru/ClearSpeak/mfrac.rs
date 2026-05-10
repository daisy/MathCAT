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
    test("ru", "ClearSpeak", expr, "одна вторая")?;
    return Ok(());
}

#[test]
fn common_fraction_thirds() -> Result<()> {
    let expr = "<math>
                    <mfrac> <mn>2</mn> <mn>3</mn> </mfrac>
                </math>";
    test("ru", "ClearSpeak", expr, "две третьих")?;
    return Ok(());
}

#[test]
fn common_fraction_tenths() -> Result<()> {
    let expr = "<math>
                    <mfrac> <mn>17</mn> <mn>10</mn> </mfrac>
                </math>";
    test_prefs("ru", "ClearSpeak", vec![("Verbosity", "Medium"), ("ClearSpeak_Fractions", "Auto")], expr, "семнадцать десятых")?;
    test_prefs("ru", "ClearSpeak", vec![("Verbosity", "Medium"), ("ClearSpeak_Fractions", "Ordinal")], expr, "семнадцать десятых")?;
    return Ok(());
}

#[test]
#[allow(non_snake_case)]
fn not_ClearSpeak_common_fraction_tenths() -> Result<()> {
    let expr = "<math>
                    <mfrac> <mn>89</mn> <mn>10</mn> </mfrac>
                </math>";
    test_prefs("ru", "ClearSpeak", vec![("Verbosity", "Medium"), ("ClearSpeak_Fractions", "Auto")], expr, "89 на 10")?;
    test_prefs("ru", "ClearSpeak", vec![("Verbosity", "Medium"), ("ClearSpeak_Fractions", "Ordinal")], expr, "восемьдесят девять десятых")?;
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
    test_prefs("ru", "ClearSpeak", vec![("Verbosity", "Medium"), ("ClearSpeak_Fractions", "Auto")], expr, "дробь, числитель: икс плюс игрек; знаменатель: икс минус игрек")?;
    test_prefs("ru", "ClearSpeak", vec![("Verbosity", "Medium"), ("ClearSpeak_Fractions", "Ordinal")], expr, "дробь, числитель: икс плюс игрек; знаменатель: икс минус игрек")?;
    test_prefs("ru", "ClearSpeak", vec![("Verbosity", "Medium"), ("ClearSpeak_Fractions", "Over")], expr, "икс плюс игрек на икс минус игрек")?;
    test_prefs("ru", "ClearSpeak", vec![("Verbosity", "Medium"), ("ClearSpeak_Fractions", "FracOver")], expr, "дробь икс плюс игрек на икс минус игрек")?;
    test_prefs("ru", "ClearSpeak", vec![("Verbosity", "Medium"), ("ClearSpeak_Fractions", "General")], expr, "дробь, числитель: икс плюс игрек; знаменатель: икс минус игрек")?;
    test_prefs("ru", "ClearSpeak", vec![("Verbosity", "Medium"), ("ClearSpeak_Fractions", "EndFrac")], expr, "дробь, числитель: икс плюс игрек; знаменатель: икс минус игрек; конец дроби")?;
    test_prefs("ru", "ClearSpeak", vec![("Verbosity", "Medium"), ("ClearSpeak_Fractions", "GeneralEndFrac")], expr, "дробь, числитель: икс плюс игрек; знаменатель: икс минус игрек; конец дроби")?;
    test_prefs("ru", "ClearSpeak", vec![("Verbosity", "Medium"), ("ClearSpeak_Fractions", "OverEndFrac")], expr, "икс плюс игрек на икс минус игрек, конец дроби")?;
    test_prefs("ru", "ClearSpeak", vec![("Verbosity", "Medium"), ("ClearSpeak_Fractions", "Per")], expr, "икс плюс игрек на икс минус игрек")?;
    test_prefs("ru", "ClearSpeak", vec![("Verbosity", "Verbose"),("ClearSpeak_Fractions", "Auto")], expr, "дробь, числитель: икс плюс игрек; знаменатель: икс минус игрек; конец дроби")?;
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
    test("ru", "ClearSpeak", expr, "62 мили в час")?;
    return Ok(());
}


#[test]
fn mixed_number() -> Result<()> {
    let expr = "<math>
                    <mn>3</mn>
                    <mfrac> <mn>1</mn> <mn>2</mn> </mfrac>
                </math>";
    test("ru", "ClearSpeak", expr, "3 и одна вторая")?;
    return Ok(());
}

#[test]
fn explicit_mixed_number() -> Result<()> {
    let expr = "<math>
                    <mn>3</mn>
                    <mo>&#x2064;</mo>
                    <mfrac> <mn>1</mn> <mn>8</mn> </mfrac>
                </math>";
    test("ru", "ClearSpeak", expr, "3 и одна восьмая")?;
    return Ok(());
}

#[test]
fn mixed_number_big() -> Result<()> {
    let expr = "<math>
                    <mn>3</mn>
                    <mfrac> <mn>7</mn> <mn>83</mn> </mfrac>
                </math>";
    test("ru", "ClearSpeak", expr, "3 и 7 на 83")?;
    return Ok(());
}

#[test]
fn simple_text() -> Result<()> {
    let expr = "<math>
    <mfrac> <mi>rise</mi> <mi>run</mi> </mfrac>
                </math>";
    test("ru", "ClearSpeak", expr, "подъём на длину")?;
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
    test("ru", "ClearSpeak", expr, "2 мили на 3 галлона")?;
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
    test_prefs("ru", "ClearSpeak", vec![("ClearSpeak_Fractions", "Auto")], expr, "одна вторая на две третьих")?;
    test_prefs("ru", "ClearSpeak", vec![("ClearSpeak_Fractions", "Ordinal")], expr, "одна вторая на две третьих")?;
    test_prefs("ru", "ClearSpeak", vec![("ClearSpeak_Fractions", "Over")], expr, "1 на 2 на 2 на 3")?;
    test_prefs("ru", "ClearSpeak", vec![("ClearSpeak_Fractions", "FracOver")], expr,
            "дробь дробь 1 на 2 на дробь 2 на 3")?;
    test_prefs("ru", "ClearSpeak", vec![("ClearSpeak_Fractions", "General")], expr,
            "дробь, числитель: дробь, числитель: 1; знаменатель: 2; и знаменатель: дробь, числитель: 2; знаменатель: 3")?;
    test_prefs("ru", "ClearSpeak", vec![("ClearSpeak_Fractions", "EndFrac")], expr, "одна вторая на две третьих")?;
    test_prefs("ru", "ClearSpeak", vec![("ClearSpeak_Fractions", "GeneralEndFrac")], expr,
            "дробь, числитель: дробь, числитель: 1; знаменатель: 2; конец дроби; и знаменатель: дробь, числитель: 2; знаменатель: 3; конец дроби; конец дроби")?;
    test_prefs("ru", "ClearSpeak", vec![("ClearSpeak_Fractions", "OverEndFrac")], expr,
            "1 на 2, конец дроби, на 2 на 3, конец дроби; конец дроби")?;
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
    test("ru", "ClearSpeak", expr, "две третьих икс на 6")?;
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
    test("ru", "ClearSpeak", expr, "дробь, числитель: 10 на эн; и знаменатель: 2 на эн")?;
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
    test("ru", "ClearSpeak", expr, "дробь, числитель: дробь, числитель: эн плюс 10; знаменатель: эн; и знаменатель: 2 на эн")?;
    return Ok(());
}

#[test]
fn simple_function() -> Result<()> {
    let expr = "<math><mfrac><mrow><mi>f</mi><mo>(</mo><mi>x</mi><mo>)</mo></mrow><mn>2</mn></mfrac></math>";
    test_prefs("ru", "ClearSpeak", vec![("Verbosity", "Medium"), ("ClearSpeak_Fractions", "Auto")], expr, "эф от икс на 2")?;
    test_prefs("ru", "ClearSpeak", vec![("Verbosity", "Verbose"), ("ClearSpeak_Fractions", "Auto")], expr, "эф от икс на 2, конец дроби")?;
    return Ok(());
}

#[test]
fn function_over_function() -> Result<()> {
    let expr = "<math><mfrac>
            <mrow><mi>f</mi><mo>(</mo><mi>x</mi><mo>)</mo></mrow>
            <mrow><mi>g</mi><mo>(</mo><mi>x</mi><mo>)</mo></mrow>
        </mfrac></math>";
    test_prefs("ru", "ClearSpeak", vec![("Verbosity", "Medium"), ("ClearSpeak_Fractions", "Auto")], expr, "эф от икс на же от икс")?;
    test_prefs("ru", "ClearSpeak", vec![("Verbosity", "Verbose"), ("ClearSpeak_Fractions", "Auto")], expr, "эф от икс на же от икс, конец дроби")?;
    return Ok(());
}

#[test]
fn non_simple_function_over_function() -> Result<()> {
    let expr = "<math><mfrac>
            <mrow><mi>f</mi><mo>(</mo><mi>x</mi><mo>+</mo><mn>1</mn><mo>)</mo></mrow>
            <mrow><mi>g</mi><mo>(</mo><mi>x</mi><mo>)</mo></mrow>
        </mfrac></math>";
    test_prefs("ru", "ClearSpeak", vec![("Verbosity", "Medium"), ("ClearSpeak_Fractions", "Auto")], expr,
             "дробь, числитель: эф от, открывается скобка икс плюс 1, закрывается скобка; знаменатель: же от икс")?;
    test_prefs("ru", "ClearSpeak", vec![("Verbosity", "Verbose"), ("ClearSpeak_Fractions", "Auto")], expr,
             "дробь, числитель: эф от, открывается скобка икс плюс 1, закрывается скобка; знаменатель: же от икс; конец дроби")?;
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
    test("ru", "ClearSpeak", expr, "2 умножить на число сочетаний из 7 по 3")?;
    return Ok(());
}
