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
    test("hu", "SimpleSpeak", expr, "1 ketted")?;
    return Ok(());

}

// AI generated
#[test]
fn common_fraction_thirds() -> Result<()> {
    let expr = "<math>
                    <mfrac> <mn>2</mn> <mn>3</mn> </mfrac>
                </math>";
    test("hu", "SimpleSpeak", expr, "2 harmad")?;
    return Ok(());

}

// AI generated
#[test]
fn common_fraction_tenths() -> Result<()> {
    let expr = "<math>
                    <mfrac> <mn>17</mn> <mn>10</mn> </mfrac>
                </math>";
    test("hu", "SimpleSpeak", expr, "17 tized")?;
    return Ok(());

}

// AI generated
#[test]
#[allow(non_snake_case)]
fn not_SimpleSpeak_common_fraction_tenths() -> Result<()> {
    let expr = "<math>
                    <mfrac> <mn>89</mn> <mn>10</mn> </mfrac>
                </math>";
    test("hu", "SimpleSpeak", expr, "89 per 10")?;
    return Ok(());

}

// AI generated
#[test]
fn non_simple_fraction() -> Result<()> {
    let expr = "
    <math>
        <mrow>
        <mfrac>
        <mrow> <mi>x</mi><mo>+</mo><mi>y</mi> </mrow>
        <mrow>
        <mi>x</mi><mo>-</mo><mi>y</mi></mrow>
        </mfrac>
        </mrow>
    </math>
                            ";
    test("hu", "SimpleSpeak", expr, "tört, x plusz y, per, x mínusz y, tört vége")?;
    return Ok(());

}

// AI generated
#[test]
fn nested_fraction() -> Result<()> {
    let expr = "
    <math>
        <mrow>
        <mfrac>
        <mrow> <mi>x</mi><mo>+</mo>  <mfrac><mn>1</mn><mi>y</mi></mfrac>  </mrow>
        <mrow>
        <mi>x</mi><mo>-</mo><mi>y</mi></mrow>
        </mfrac>
        </mrow>
    </math>
                            ";
    test("hu", "SimpleSpeak", expr, "tört, x plusz, tört, 1 per y, tört vége; per, x mínusz y, tört vége")?;
    return Ok(());

}


// AI generated
#[test]
fn deeply_nested_fraction_msqrt() -> Result<()> {
    let expr = "
    <math>
        <mrow>
        <mfrac>
        <mrow> <mi>x</mi><mo>+</mo>  <msqrt><mrow><mfrac><mn>1</mn><mi>y</mi></mfrac></mrow> </msqrt> </mrow>
        <mrow>
        <mi>x</mi><mo>-</mo><mi>y</mi></mrow>
        </mfrac>
        </mrow>
    </math>
                            ";
    test("hu", "SimpleSpeak", expr, "tört, x plusz, a négyzetgyöke ennek: 1 per y; gyök vége; per, x mínusz y, tört vége")?;
    return Ok(());

}

// AI generated
#[test]
fn deeply_nested_fraction_mrow_msqrt() -> Result<()> {
    let expr = "
    <math>
        <mrow>
        <mfrac>
        <mrow> <mi>x</mi><mo>+</mo>  <msqrt><mrow><mn>2</mn><mo>+</mo><mfrac><mn>1</mn><mi>y</mi></mfrac></mrow> </msqrt> </mrow>
        <mrow>
        <mi>x</mi><mo>-</mo><mi>y</mi></mrow>
        </mfrac>
        </mrow>
    </math>
                            ";
    test("hu", "SimpleSpeak", expr, "tört, x plusz, a négyzetgyöke ennek: 2 plusz 1 per y; gyök vége; per, x mínusz y, tört vége")?;
    return Ok(());

}

// AI generated
#[test]
fn numerator_simple_fraction() -> Result<()> {
    let expr = "
    <math>
        <mrow>
        <mfrac>
        <mrow> <mi>x</mi></mrow>
        <mrow>
            <mi>x</mi><mo>-</mo><mi>y</mi></mrow>
        </mfrac>
        </mrow>
    </math>
                            ";
    test("hu", "SimpleSpeak", expr, "tört, x per, x mínusz y, tört vége")?;
    return Ok(());

}

// AI generated
#[test]
fn denominator_simple_fraction() -> Result<()> {
    let expr = "
    <math>
        <mfrac>
            <mrow> <mi>x</mi><mo>-</mo><mi>y</mi></mrow>
            <mrow> <mi>x</mi></mrow>
        </mfrac>
    </math>
                            ";
    test("hu", "SimpleSpeak", expr, "tört, x mínusz y, per x, tört vége")?;
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
    test("hu", "SimpleSpeak", expr, "62 mérföld per óra")?;
    return Ok(());

}

// AI generated
#[test]
fn singular_frac_with_units() -> Result<()> {
    let expr = "
    <math>
        <mrow>
        <mn>1</mn>
        <mfrac>
        <mi intent=':unit'>gal</mi>
        <mi intent=':unit'>mi</mi>
        </mfrac>
        </mrow>
    </math>";
    test("hu", "SimpleSpeak", expr, "1 gallon per mérföld")?;
    return Ok(());

}

// AI generated
#[test]
fn number_in_numerator_with_units() -> Result<()> {
    let expr = "
    <math>
        <mfrac>
            <mrow>
                <mn>3</mn>
                <mi intent=':unit'>gal</mi>
            </mrow>
            <mi intent=':unit'>mi</mi>
        </mfrac>
    </math>";
    test("hu", "SimpleSpeak", expr, "3 gallon per mérföld")?;
    return Ok(());

}

// AI generated
#[test]
fn units_with_powers() -> Result<()> {
    let expr = "
    <math>
        <mfrac>
            <mrow> <mn>3</mn> <mi intent=':unit'>m</mi> </mrow>
            <msup> <mi intent=':unit'>s</mi><mn>2</mn> </msup>
        </mfrac>
    </math>";
    test("hu", "SimpleSpeak", expr, "3 méter per másodperc a négyzeten")?;
    return Ok(());

}


// AI generated
#[test]
fn mixed_number() -> Result<()> {
    let expr = "<math>
                    <mn>3</mn>
                    <mfrac> <mn>1</mn> <mn>2</mn> </mfrac>
                </math>";
    test("hu", "SimpleSpeak", expr, "3 és 1 ketted")?;
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
    test("hu", "SimpleSpeak", expr, "3 és 1 nyolcad")?;
    return Ok(());

}

// AI generated
#[test]
fn mixed_number_big() -> Result<()> {
    let expr = "<math>
                    <mn>3</mn>
                    <mfrac> <mn>7</mn> <mn>83</mn> </mfrac>
                </math>";
    test("hu", "SimpleSpeak", expr, "3 és 7 kilenc harmad")?;
    return Ok(());

}

// AI generated
#[test]
fn simple_text() -> Result<()> {
    let expr = "<math>
    <mfrac> <mi>rise</mi> <mi>run</mi> </mfrac>
                </math>";
    test("hu", "SimpleSpeak", expr, "rise per run")?;
    return Ok(());

}

// AI generated
#[test]
fn number_and_text() -> Result<()> {
    let expr = "<math>
            <mfrac>
            <mrow>
                <mn>2</mn><mtext>mérföld</mtext></mrow>
            <mrow>
                <mn>3</mn><mtext>gallon</mtext></mrow>
            </mfrac>
        </math>";
    test("hu", "SimpleSpeak", expr, "tört, 2 mérföld, per, 3 gallon, tört vége")?;
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
    test("hu", "SimpleSpeak", expr, "tört, 1 ketted, per, 2 harmad, tört vége")?;
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
    test("hu", "SimpleSpeak", expr, "2 szorozva 7 a 3")?;
    return Ok(());

}

// AI generated
#[test]
fn binomial_non_simple_top() -> Result<()> {
    let expr = "<math>
                    <mn>2</mn>
                    <mo>(</mo>
                    <mfrac linethickness='0'> <mrow><mi>n</mi><mo>+</mo><mn>7</mn></mrow> <mn>3</mn> </mfrac>
                    <mo>)</mo>
                </math>";
    test("hu", "SimpleSpeak", expr, "2 szorozva, binomiális kezdete n plusz 7 a 3")?;
    return Ok(());

}

// AI generated
#[test]
fn binomial_non_simple_bottom() -> Result<()> {
    let expr = "<math>
                    <mn>2</mn>
                    <mo>(</mo>
                    <mfrac linethickness='0'> <mn>7</mn> <mrow><mi>k</mi><mo>+</mo><mn>3</mn></mrow> </mfrac>
                    <mo>)</mo>
                </math>";
    test("hu", "SimpleSpeak", expr, "2 szorozva, 7 a k plusz 3 binomiális vége")?;
    return Ok(());

}

// AI generated
#[test]
fn binomial_non_simple_top_and_bottom() -> Result<()> {
    let expr = "<math>
                    <mn>2</mn>
                    <mo>(</mo>
                    <mfrac linethickness='0'> <mrow><mi>n</mi><mo>+</mo><mn>7</mn></mrow> <mrow><mi>k</mi><mo>+</mo><mn>3</mn></mrow> </mfrac>
                    <mo>)</mo>
                </math>";
    test("hu", "SimpleSpeak", expr, "2 szorozva; binomiális kezdete n plusz 7 a k plusz 3 binomiális vége")?;
    return Ok(());

}
