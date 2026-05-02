/// Tests for:
/// *  functions including trig functions, logs, and functions to powers
/// *  implied times/functional call and explicit times/function call
/// *  parens
/// These are all intertwined, so they are in one file
use crate::common::*;
use anyhow::Result;

#[test]
fn trig_names() -> Result<()> {
    let expr = "<math><mrow>
    <mi>sin</mi><mi>x</mi><mo>+</mo>
    <mi>cos</mi><mi>y</mi><mo>+</mo>
    <mi>tan</mi><mi>z</mi><mo>+</mo>
    <mi>sec</mi><mi>&#x03B1;</mi><mo>+</mo>
    <mi>csc</mi><mi>&#x03D5;</mi><mo>+</mo>
    <mi>cot</mi><mi>&#x03C6;</mi>
    </mrow></math>";
    test("en", "ClearSpeak", expr, "sine x plus cosine y plus tangent z plus secant alpha, plus cosecant phi, plus cotangent phi")?;
    return Ok(());

}

#[test]
fn hyperbolic_trig_names() -> Result<()> {
    let expr = "<math><mrow>
    <mi>sinh</mi><mi>x</mi><mo>+</mo>
    <mi>cosh</mi><mi>y</mi><mo>+</mo>
    <mi>tanh</mi><mi>z</mi><mo>+</mo>
    <mi>sech</mi><mi>&#x03B1;</mi><mo>+</mo>
    <mi>csch</mi><mi>&#x03D5;</mi><mo>+</mo>
    <mi>coth</mi><mi>&#x03C6;</mi>
    </mrow></math>";
    test("en", "ClearSpeak", expr, "hyperbolic sine of x, plus \
                                hyperbolic cosine of y, plus \
                                hyperbolic tangent of z, plus \
                                hyperbolic secant of alpha, plus \
                                hyperbolic cosecant of phi, plus \
                                hyperbolic cotangent of phi")?;
                                return Ok(());

}


#[test]
fn inverse_trig() -> Result<()> {
    let expr = "<math><msup><mi>sin</mi><mrow><mo>-</mo><mn>1</mn></mrow></msup><mi>x</mi></math>";
    test("en", "ClearSpeak", expr, "inverse sine of x")?;
    return Ok(());

}

#[test]
fn inverse_trig_trig_inverse() -> Result<()> {
    let expr = "<math><msup><mi>tan</mi><mrow><mo>-</mo><mn>1</mn></mrow></msup><mi>x</mi></math>";
    test_ClearSpeak("en", "ClearSpeak_Trig", "TrigInverse",expr,
        "tangent inverse of x")?;
        return Ok(());

}

#[test]
fn inverse_trig_arc() -> Result<()> {
    let expr = "<math><msup><mi>cosh</mi><mrow><mo>-</mo><mn>1</mn></mrow></msup><mi>x</mi></math>";
    test_ClearSpeak("en", "ClearSpeak_Trig", "ArcTrig",expr,
        "arc hyperbolic cosine of x")?;
        return Ok(());

}

#[test]
fn trig_squared() -> Result<()> {
    let expr = "<math><msup><mi>sin</mi><mn>2</mn></msup><mi>x</mi></math>";
    test("en", "ClearSpeak", expr, "sine squared of x")?;
    return Ok(());

}

#[test]
fn trig_cubed() -> Result<()> {
    let expr = "<math><msup><mi>tan</mi><mn>3</mn></msup><mi>x</mi></math>";
    test("en", "ClearSpeak", expr, "tangent cubed of x")?;
    return Ok(());

}

#[test]
fn trig_fourth() -> Result<()> {
    let expr = "<math><msup><mi>sec</mi><mn>4</mn></msup><mi>x</mi></math>";
    test("en", "ClearSpeak", expr, "the fourth power of, secant of x")?;
    return Ok(());

}


#[test]
fn trig_power_other() -> Result<()> {
    let expr = "<math><msup><mi>sinh</mi><mrow>><mi>n</mi><mo>-</mo><mn>1</mn></mrow></msup><mi>x</mi></math>";
    test("en", "ClearSpeak", expr, "the n minus 1 power of, hyperbolic sine of x")?;
    return Ok(());

}

#[test]
fn simple_log() -> Result<()> {
    let expr = "<math> <mrow>  <mi>log</mi><mi>x</mi></mrow> </math>";
    test("en", "ClearSpeak", expr, "log x")?;
    return Ok(());

}

#[test]
fn normal_log() -> Result<()> {
    let expr = "<math><mrow><mi>log</mi><mrow><mo>(</mo><mrow><mi>x</mi><mo>+</mo><mi>y</mi></mrow><mo>)</mo></mrow></mrow></math>";
    test("en", "ClearSpeak", expr, "the log of, open paren x plus y, close paren")?;
    return Ok(());

}

#[test]
fn simple_log_with_base() -> Result<()> {
    let expr = "<math> <mrow>  <msub><mi>log</mi><mi>b</mi></msub><mi>x</mi></mrow> </math>";
    test("en", "ClearSpeak", expr, "the log base b, of x")?;
    return Ok(());

}

#[test]
fn normal_log_with_base() -> Result<()> {
    let expr = "<math><mrow><msub><mi>log</mi><mi>b</mi></msub><mrow><mo>(</mo><mrow><mi>x</mi><mo>+</mo><mi>y</mi></mrow><mo>)</mo></mrow></mrow></math>";
    test("en", "ClearSpeak", expr, "the log base b, of, open paren x plus y, close paren")?;
    return Ok(());

}

#[test]
fn simple_ln() -> Result<()> {
    let expr = "<math> <mrow>  <mi>ln</mi><mi>x</mi></mrow> </math>";
    test("en", "ClearSpeak", expr, "l n x")?;
    return Ok(());

}

#[test]
fn normal_ln() -> Result<()> {
    let expr = "<math><mrow><mi>ln</mi><mrow><mo>(</mo><mrow><mi>x</mi><mo>+</mo><mi>y</mi></mrow><mo>)</mo></mrow></mrow></math>";
    test("en", "ClearSpeak", expr, "the l n of, open paren x plus y, close paren")?;
    return Ok(());

}

    
#[test]
fn simple_natural_log() -> Result<()> {
    let expr = "<math> <mrow>  <mi>ln</mi><mi>x</mi></mrow> </math>";
    test_ClearSpeak("en", "ClearSpeak_Log", "LnAsNaturalLog",expr,
        "natural log x")?;
        return Ok(());

}

    
#[test]
fn natural_log() -> Result<()> {
    let expr = "<math><mi>ln</mi><mo>(</mo><mi>x</mi><mo>+</mo><mi>y</mi><mo>)</mo></math>";
    test_ClearSpeak("en", "ClearSpeak_Log", "LnAsNaturalLog",expr,
        "the natural log of, open paren x plus y, close paren")?;
        return Ok(());

}


#[test]
fn explicit_function_call_with_parens() -> Result<()> {
    let expr = "<math><mrow><mi>t</mi><mo>&#x2061;</mo><mrow><mo>(</mo><mi>x</mi><mo>)</mo></mrow></mrow></math>";
    test("en", "ClearSpeak", expr, "t of x")?;
    return Ok(());

}


#[test]
fn explicit_times_with_parens() -> Result<()> {
    let expr = "<math><mrow><mi>t</mi><mo>&#x2062;</mo><mrow><mo>(</mo><mi>x</mi><mo>)</mo></mrow></mrow></math>";
    test("en", "ClearSpeak", expr, "t times x")?;
    return Ok(());

}

#[test]
fn explicit_function_call() -> Result<()> {
    let expr = "<math><mrow><mi>t</mi><mo>&#x2061;</mo><mrow><mi>x</mi></mrow></mrow></math>";
    test("en", "ClearSpeak", expr, "t of x")?;
    return Ok(());

}

#[test]
fn explicit_times() -> Result<()> {
    let expr = "<math><mrow><mi>t</mi><mo>&#x2062;</mo><mrow><mi>x</mi></mrow></mrow></math>";
    test("en", "ClearSpeak", expr, "t x")?;
    return Ok(());

}


#[test]
fn test_functions_none_pref() -> Result<()> {
    let expr = "<math>
    <mi>log</mi><mo>&#x2061;</mo><mrow><mo>(</mo><mrow><mi>x</mi><mo>+</mo><mi>y</mi></mrow><mo>)</mo></mrow>
    <mo>+</mo>
    <mi>f</mi><mo>&#x2061;</mo><mrow><mo>(</mo><mrow><mi>x</mi><mo>+</mo><mi>y</mi></mrow><mo>)</mo></mrow>
    </math>";
    test_ClearSpeak("en", "ClearSpeak_Functions", "None",expr,
        "the log of, open paren x plus y, close paren; plus, f times, open paren x plus y, close paren")?;
        return Ok(());

}

#[test]
fn test_functions_none_pref_multiple_args() -> Result<()> {
    let expr = "<math>
        <mi>B</mi> <mrow><mo>(</mo> <mrow> <mn>2</mn><mo>,</mo><mn>6</mn></mrow> <mo>)</mo></mrow>
    </math>";
    test_ClearSpeak("en", "ClearSpeak_Functions", "None",expr,
        "cap b times, open paren 2 comma, 6, close paren")?;
        return Ok(());

}


/*
    * Tests for times
    */
#[test]
fn no_times_binomial() -> Result<()> {
    let expr = "<math><mrow><mi>x</mi> <mo>&#x2062;</mo> <mi>y</mi></mrow></math>";
    test("en", "ClearSpeak", expr, "x y")?;
    return Ok(());

}

#[test]
fn times_following_paren() -> Result<()> {
    let expr = "<math><mrow>
        <mn>2</mn>
        <mrow>  <mo>(</mo> <mn>3</mn>  <mo>)</mo> </mrow>
        </mrow></math>";
    test("en", "ClearSpeak", expr, "2 times 3")?;
    return Ok(());

}

#[test]
fn times_preceding_paren() -> Result<()> {
    let expr = "<math><mrow>
        <mrow>  <mo>(</mo> <mn>2</mn>  <mo>)</mo> </mrow>
        <mn>3</mn>
        </mrow></math>";
    test("en", "ClearSpeak", expr, "2 times 3")?;
    return Ok(());

}

#[test]
fn times_sqrt() -> Result<()> {
    let expr = "<math><mrow>
        <msqrt> <mi>a</mi>  </msqrt>
        <msqrt> <mi>b</mi>  </msqrt>
        <mo>=</mo>
        <msqrt> <mrow>  <mi>a</mi><mi>b</mi></mrow> </msqrt>
        </mrow></math>";
    test("en", "ClearSpeak", expr, "the square root of eigh; times the square root of b; is equal to, the square root of eigh b")?;
    return Ok(());

}

#[test]
fn more_implied_times() -> Result<()> {
    let expr = "<math><mrow>
    <mrow>
    <msup>
        <mrow>
        <mrow><mo>(</mo>
        <mrow> <mn>2</mn><mi>x</mi></mrow>
        <mo>)</mo></mrow></mrow>
        <mn>2</mn>
    </msup>
    </mrow>
    </mrow></math>";
    test_ClearSpeak("en", "ClearSpeak_ImpliedTimes", "MoreImpliedTimes",expr,
        "open paren 2 times x, close paren squared")?;
        return Ok(());

}

#[test]
fn explicit_times_more_implied_times() -> Result<()> {
    let expr = "<math><mrow><mi>t</mi><mo>&#x2062;</mo><mrow><mi>x</mi></mrow></mrow></math>";
    test_ClearSpeak("en", "ClearSpeak_ImpliedTimes", "MoreImpliedTimes",expr, "t times x")?;
    return Ok(());

}

#[test]
fn explicit_times_none_simple_right() -> Result<()> {
    let expr = "<math><mn>2</mn><mo>[</mo><mn>3</mn> <mo>]</mo></math>";
    test_ClearSpeak("en", "ClearSpeak_ImpliedTimes", "None",
        expr, "2, open bracket 3 close bracket")?;
        return Ok(());

}

#[test]
fn explicit_times_none_simple_left() -> Result<()> {
    let expr = "<math><mo>(</mo><mn>2</mn><mo>&#x2212;</mo><mn>1</mn><mo>)</mo><mi>x</mi></math>";
    test_ClearSpeak("en", "ClearSpeak_ImpliedTimes", "None",
        expr, "open paren 2 minus 1, close paren; x")?;
        return Ok(());

}

#[test]
fn explicit_times_none_superscript() -> Result<()> {
    let expr = "<math> 
    <mi>f</mi><mo>(</mo><mi>x</mi><mo>)</mo><mo>=</mo><msup>
<mi>x</mi>
<mn>2</mn>
</msup>
<mrow><mo>(</mo>
<mrow>
<mi>x</mi><mo>+</mo><mn>1</mn></mrow>
<mo>)</mo></mrow>
    </math>";
    test_ClearSpeak_prefs("en", 
        vec![("ClearSpeak_ImpliedTimes", "None"), ("ClearSpeak_Functions", "None")],
        expr, "f open paren x close paren; is equal to; x squared, open paren x plus 1, close paren")?;
        return Ok(());

}

/*
    * Tests for parens
    */
    #[test]
    fn no_parens_number() -> Result<()> {
        let expr = "<math><mrow>
        <mrow><mo>(</mo>
        <mn>25</mn>
        <mo>)</mo></mrow>
        <mi>x</mi>
        </mrow></math>";
        test("en", "ClearSpeak", expr, "25 times x")?;
        return Ok(());

    }

    #[test]
    fn no_parens_monomial() -> Result<()> {
        let expr = "<math><mrow>
        <mi>b</mi>
        <mrow><mo>(</mo>
        <mrow><mi>x</mi><mi>y</mi></mrow>
        <mo>)</mo></mrow>
        </mrow></math>";
        test("en", "ClearSpeak", expr, "b, open paren x y close paren")?;
        return Ok(());

    }

    #[test]
    fn no_parens_negative_number() -> Result<()> {
        let expr = "<math><mrow>
        <mn>2</mn><mo>+</mo>
        <mrow><mo>(</mo>
        <mrow><mo>&#x2212;</mo><mn>2</mn></mrow>
        <mo>)</mo></mrow>
        </mrow></math>";
        test("en", "ClearSpeak", expr, "2 plus negative 2")?;
        return Ok(());

    }


    #[test]
    fn no_parens_negative_number_with_var() -> Result<()> {
        let expr = "<math><mrow>
        <mrow><mo>(</mo>
        <mrow><mo>&#x2212;</mo><mn>2</mn></mrow><mi>x</mi>
        <mo>)</mo>
        </mrow>
        <mo>+</mo><mn>1</mn>
        </mrow></math>";
        test("en", "ClearSpeak", expr, "negative 2 x, plus 1")?;
        return Ok(());

    }

    #[test]
    fn parens_superscript() -> Result<()> {
        let expr = "<math><mrow>
        <mrow>
        <msup>
        <mrow>
            <mrow><mo>(</mo>
            <mrow> <mn>2</mn><mi>x</mi></mrow>
            <mo>)</mo></mrow></mrow>
        <mn>2</mn>
        </msup>
        </mrow>
    </mrow></math>";
        test("en", "ClearSpeak", expr, "open paren 2 x close paren squared")?;
        return Ok(());

    }

    #[test]
    fn no_parens_fraction() -> Result<()> {
        let expr = "<math><mrow>
        <mn>2</mn>
        <mo>+</mo>
        <mrow>
            <mrow><mo>(</mo>
            <mfrac> <mn>1</mn><mn>2</mn></mfrac>
            <mo>)</mo></mrow></mrow>
    </mrow></math>";
        test("en", "ClearSpeak", expr, "2 plus 1 half")?;
        return Ok(());

    }


    // Tests for the ten types of intervals in ClearSpeak
    #[test]
    fn parens_interval_open_open() -> Result<()> {
        let expr = "<math> 
        <mrow><mo>(</mo>
        <mrow> <mi>c</mi><mo>,</mo><mi>d</mi></mrow>
        <mo>)</mo></mrow>
    </math>";
    test_ClearSpeak("en", "ClearSpeak_Paren", "Interval",expr,
    "the interval from c to d, not including c or d")?;
    return Ok(());

}

#[test]
    fn parens_interval_closed_open() -> Result<()> {
        let expr = "<math> 
        <mrow><mo>[</mo>
        <mrow> <mi>c</mi><mo>,</mo><mi>d</mi></mrow>
        <mo>)</mo></mrow>
    </math>";
    test_ClearSpeak("en", "ClearSpeak_Paren", "Interval ",expr,
    "the interval from c to d, including c but not including d")?;
    return Ok(());

}


#[test]
fn parens_interval_open_closed() -> Result<()> {
    let expr = "<math> 
    <mrow><mo>(</mo>
        <mrow> <mi>c</mi><mo>,</mo><mi>d</mi></mrow>
    <mo>]</mo></mrow>
    </math>";
    test_ClearSpeak("en", "ClearSpeak_Paren", "Interval ",expr,
    "the interval from c to d, not including c but including d")?;
    return Ok(());

}


#[test]
fn parens_interval_closed_closed() -> Result<()> {
    let expr = "<math> 
    <mrow><mo>[</mo>
    <mrow> <mi>c</mi><mo>,</mo><mi>d</mi></mrow>
    <mo>]</mo></mrow>
</math>";
test_ClearSpeak("en", "ClearSpeak_Paren", "Interval ",expr,
"the interval from c to d, including c and d")?;
return Ok(());

}

    #[test]
    fn parens_interval_neg_infinity_open_open() -> Result<()> {
        let expr = "<math> 
        <mrow><mo>(</mo>
        <mrow><mo>-</mo> <mi>∞</mi><mo>,</mo><mi>d</mi></mrow>
        <mo>)</mo></mrow>
    </math>";
    test_ClearSpeak("en", "ClearSpeak_Paren", "Interval ",expr,
    "the interval from negative infinity to d, not including d")?;
    return Ok(());

}

    #[test]
    fn parens_interval_neg_infinity_closed_open() -> Result<()> {
        let expr = "<math> 
        <mrow><mo>(</mo>
        <mrow> <mo>-</mo> <mi>∞</mi><mo>,</mo><mi>d</mi></mrow>
        <mo>]</mo></mrow>
    </math>";
    test_ClearSpeak("en", "ClearSpeak_Paren", "Interval ",expr,
    "the interval from negative infinity to d, including d")?;
    return Ok(());

}


#[test]
fn parens_interval_open_open_infinity() -> Result<()> {
    let expr = "<math> 
    <mrow><mo>(</mo>
        <mrow> <mi>c</mi><mo>,</mo><mi>∞</mi></mrow>
    <mo>)</mo></mrow>
    </math>";
    test_ClearSpeak("en", "ClearSpeak_Paren", "Interval ",expr,
    "the interval from c to infinity, not including c")?;
    return Ok(());

}


#[test]
fn parens_interval_closed_open_infinity() -> Result<()> {
    let expr = "<math> 
        <mrow><mo>[</mo>
        <mrow> <mi>c</mi><mo>,</mo><mi>∞</mi></mrow>
        <mo>)</mo></mrow>
    </math>";
    test_ClearSpeak("en", "ClearSpeak_Paren", "Interval ",expr,
"the interval from c to infinity, including c")?;
return Ok(());

}

#[test]
fn parens_interval_neg_infinity_to_infinity() -> Result<()> {
    let expr = "<math> 
    <mrow><mo>(</mo>
        <mrow><mo>-</mo> <mi>∞</mi><mo>,</mo><mi>∞</mi></mrow>
        <mo>)</mo></mrow>
    </math>";
    test_ClearSpeak("en", "ClearSpeak_Paren", "Interval ",expr,
    "the interval from negative infinity to infinity")?;
    return Ok(());

}

#[test]
fn parens_interval_neg_infinity_to_pos_infinity() -> Result<()> {
    let expr = "<math> 
    <mrow><mo>(</mo>
        <mrow><mo>-</mo> <mi>∞</mi><mo>,</mo><mo>+</mo><mi>∞</mi></mrow>
    <mo>)</mo></mrow>
    </math>";
    test_ClearSpeak("en", "ClearSpeak_Paren", "Interval ",expr,
    "the interval from negative infinity to positive infinity")?;
    return Ok(());

}
