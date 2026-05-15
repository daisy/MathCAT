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
    test("fr", "SimpleSpeak", expr, "1 demi")?;
    return Ok(());

}

#[test]
fn common_fraction_thirds() -> Result<()> {
    let expr = "<math>
                    <mfrac> <mn>2</mn> <mn>3</mn> </mfrac>
                </math>";
    test("fr", "SimpleSpeak", expr, "2 tiers")?;
    return Ok(());

}

#[test]
fn common_fraction_tenths() -> Result<()> {
    let expr = "<math>
                    <mfrac> <mn>17</mn> <mn>10</mn> </mfrac>
                </math>";
    test("fr", "SimpleSpeak", expr, "17 dixièmes")?;
    return Ok(());

}

#[test]
#[allow(non_snake_case)]
fn not_SimpleSpeak_common_fraction_tenths() -> Result<()> {
    let expr = "<math>
                    <mfrac> <mn>89</mn> <mn>10</mn> </mfrac>
                </math>";
    test("fr", "SimpleSpeak", expr, "89 sur 10")?;
    return Ok(());

}

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
    test("fr", "SimpleSpeak", expr, "fraction, x plus y, sur, x moins y, fin de fraction")?;
    return Ok(());

}

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
    test("fr", "SimpleSpeak", expr, "fraction, x plus, fraction, 1 sur y, fin de fraction; sur, x moins y, fin de fraction")?;
    return Ok(());

}


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
    test("fr", "SimpleSpeak", expr, "fraction, x plus, la racine carrée de 1 sur y; fin de racine; sur, x moins y, fin de fraction")?;
    return Ok(());

}

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
    test("fr", "SimpleSpeak", expr, "fraction, x plus, la racine carrée de 2 plus 1 sur y; fin de racine; sur, x moins y, fin de fraction")?;
    return Ok(());

}

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
    test("fr", "SimpleSpeak", expr, "fraction, x sur, x moins y, fin de fraction")?;
    return Ok(());

}

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
    test("fr", "SimpleSpeak", expr, "fraction, x moins y, sur x, fin de fraction")?;
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
    test("fr", "SimpleSpeak", expr, "62 milles par heure")?;
    return Ok(());

}

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
    test("fr", "SimpleSpeak", expr, "1 gallon par mille")?;
    return Ok(());

}

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
    test("fr", "SimpleSpeak", expr, "3 gallons par mille")?;
    return Ok(());

}

#[test]
fn units_with_powers() -> Result<()> {
    let expr = "
    <math>
        <mfrac>
            <mrow> <mn>3</mn> <mi intent=':unit'>m</mi> </mrow>
            <msup> <mi intent=':unit'>s</mi><mn>2</mn> </msup>
        </mfrac>
    </math>";
    test("fr", "SimpleSpeak", expr, "3 mètres par seconde au carré")?;
    return Ok(());

}


#[test]
fn mixed_number() -> Result<()> {
    let expr = "<math>
                    <mn>3</mn>
                    <mfrac> <mn>1</mn> <mn>2</mn> </mfrac>
                </math>";
    test("fr", "SimpleSpeak", expr, "3 et 1 demi")?;
    return Ok(());

}

#[test]
fn explicit_mixed_number() -> Result<()> {
    let expr = "<math>
                    <mn>3</mn>
                    <mo>&#x2064;</mo>
                    <mfrac> <mn>1</mn> <mn>8</mn> </mfrac>
                </math>";
    test("fr", "SimpleSpeak", expr, "3 et 1 huitième")?;
    return Ok(());

}

#[test]
fn mixed_number_big() -> Result<()> {
    let expr = "<math>
                    <mn>3</mn>
                    <mfrac> <mn>7</mn> <mn>83</mn> </mfrac>
                </math>";
    test("fr", "SimpleSpeak", expr, "3 et 7 quatre-vingt troisièmes")?;
    return Ok(());

}

#[test]
fn simple_text() -> Result<()> {
    let expr = "<math>
    <mfrac> <mi>rise</mi> <mi>run</mi> </mfrac>
                </math>";
    test("fr", "SimpleSpeak", expr, "rise sur run")?;
    return Ok(());

}

#[test]
fn number_and_text() -> Result<()> {
    let expr = "<math>
            <mfrac>
            <mrow>
                <mn>2</mn><mtext>milles</mtext></mrow>
            <mrow>
                <mn>3</mn><mtext>gallons</mtext></mrow>
            </mfrac>
        </math>";
    test("fr", "SimpleSpeak", expr, "fraction, 2 milles, sur, 3 gallons, fin de fraction")?;
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
    test("fr", "SimpleSpeak", expr, "fraction, 1 demi, sur, 2 tiers, fin de fraction")?;
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
    test("fr", "SimpleSpeak", expr, "2 fois 7 parmi 3")?;
    return Ok(());

}

#[test]
fn binomial_non_simple_top() -> Result<()> {
    let expr = "<math>
                    <mn>2</mn>
                    <mo>(</mo>
                    <mfrac linethickness='0'> <mrow><mi>n</mi><mo>+</mo><mn>7</mn></mrow> <mn>3</mn> </mfrac>
                    <mo>)</mo>
                </math>";
    test("fr", "SimpleSpeak", expr, "2 fois, binôme n plus 7 parmi 3")?;
    return Ok(());

}

#[test]
fn binomial_non_simple_bottom() -> Result<()> {
    let expr = "<math>
                    <mn>2</mn>
                    <mo>(</mo>
                    <mfrac linethickness='0'> <mn>7</mn> <mrow><mi>k</mi><mo>+</mo><mn>3</mn></mrow> </mfrac>
                    <mo>)</mo>
                </math>";
    test("fr", "SimpleSpeak", expr, "2 fois, 7 parmi k plus 3 fin du binôme")?;
    return Ok(());

}

#[test]
fn binomial_non_simple_top_and_bottom() -> Result<()> {
    let expr = "<math>
                    <mn>2</mn>
                    <mo>(</mo>
                    <mfrac linethickness='0'> <mrow><mi>n</mi><mo>+</mo><mn>7</mn></mrow> <mrow><mi>k</mi><mo>+</mo><mn>3</mn></mrow> </mfrac>
                    <mo>)</mo>
                </math>";
    test("fr", "SimpleSpeak", expr, "2 fois, binôme n plus 7 parmi k plus 3 fin du binôme")?;
    return Ok(());

}
