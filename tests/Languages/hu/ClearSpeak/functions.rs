/// Tests for:
/// *  functions including trig functions, logs, and functions to powers
/// *  implied times/functional call and explicit times/function call
/// *  parens
/// These are all intertwined, so they are in one file
use crate::common::*;
use anyhow::Result;

// AI generated
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
    test("hu", "ClearSpeak", expr, "szinusz x plusz koszinusz y, plusz tangens z plusz szekáns alfa, plusz koszekáns phi, plusz kotangens fí")?;
    return Ok(());

}

// AI generated
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
    test("hu", "ClearSpeak", expr, "hiperbolikus szinusz, x, plusz hiperbolikus koszinusz, y, plusz hiperbolikus tangens, z, plusz, hiperbolikus szekáns, alfa, plusz, hiperbolikus koszekáns, phi, plusz, hiperbolikus kotangens, fí")?;
                                return Ok(());

}


// AI generated
#[test]
fn inverse_trig() -> Result<()> {
    let expr = "<math><msup><mi>sin</mi><mrow><mo>-</mo><mn>1</mn></mrow></msup><mi>x</mi></math>";
    test("hu", "ClearSpeak", expr, "inverz szinusz x")?;
    return Ok(());

}

// AI generated
#[test]
fn inverse_trig_trig_inverse() -> Result<()> {
    let expr = "<math><msup><mi>tan</mi><mrow><mo>-</mo><mn>1</mn></mrow></msup><mi>x</mi></math>";
    test_ClearSpeak("hu", "ClearSpeak_Trig", "TrigInverse",expr,
        "tangens inverz x")?;
        return Ok(());

}

// AI generated
#[test]
fn inverse_trig_arc() -> Result<()> {
    let expr = "<math><msup><mi>cosh</mi><mrow><mo>-</mo><mn>1</mn></mrow></msup><mi>x</mi></math>";
    test_ClearSpeak("hu", "ClearSpeak_Trig", "ArcTrig",expr,
        "arkusz hiperbolikus koszinusz, x")?;
        return Ok(());

}

// AI generated
#[test]
fn trig_squared() -> Result<()> {
    let expr = "<math><msup><mi>sin</mi><mn>2</mn></msup><mi>x</mi></math>";
    test("hu", "ClearSpeak", expr, "szinusz x a négyzeten")?;
    return Ok(());

}

#[test]
fn trig_squared2() -> Result<()> {
    let expr = "<math><msup><mi>sin</mi><mn>2</mn></msup><mo>(</mo><mi>x</mi><mo>+</mo><mi>y</mi><mo>)</mo></math>";
    test("hu", "ClearSpeak", expr, "szinusz, nyitott zárójel x plusz y zárt zárójel a négyzeten")?;
    return Ok(());

}

#[test]
fn fifth_sinus_expression() -> Result<()> {
    let expr = "<math><msup><mi>sin</mi><mn>5</mn></msup><mo>(</mo><mi>x</mi><mo>+</mo><mi>y</mi><mo>)</mo>
</math>";
    test("hu", "ClearSpeak", expr, "szinusz, nyitott zárójel x plusz y zárt zárójel a(z) ötödik hatványon")?;
    return Ok(());

}

#[test]
fn letter_exponent_combined_sinus_expression() -> Result<()> {
    let expr = "<math><msup><mi>sin</mi><mi>n</mi></msup><mi>x</mi></math>";
    test("hu", "ClearSpeak", expr, "szinusz, x, a(z) n hatványon")?;
    return Ok(());

}

#[test]
fn combined_sinus_expression() -> Result<()> {
    let expr = "<math><msup><mi>sin</mi><mn>3</mn></msup><mo>(</mo><mi>x</mi><mo>+</mo><mi>y</mi><mo>)</mo></math>";
    test("hu", "ClearSpeak", expr, "szinusz, nyitott zárójel x plusz y zárt zárójel a köbön")?;
    return Ok(());

}

// AI generated
#[test]
fn trig_first() -> Result<()> {
    let expr = "<math><msup><mi>tan</mi><mn>3</mn></msup><mn>1</mn></math>";
    test("hu", "ClearSpeak", expr, "tangens 1 a köbön")?;
    return Ok(());

}

// AI generated
#[test]
fn trig_cubed() -> Result<()> {
    let expr = "<math><msup><mi>tan</mi><mn>3</mn></msup><mi>x</mi></math>";
    test("hu", "ClearSpeak", expr, "tangens x a köbön")?;
    return Ok(());

}

#[test]
fn trig_tangent_test2() -> Result<()> {
    let expr = "<math><msup><mi>tan</mi><mn>4</mn></msup><mi>x</mi></math>";
    test("hu", "ClearSpeak", expr, "tangens; x, a(z) negyedik hatványon")?;
    return Ok(());

}

// AI generated
#[test]
fn trig_fourth() -> Result<()> {
    let expr = "<math><msup><mi>sec</mi><mn>4</mn></msup><mi>x</mi></math>";
    test("hu", "ClearSpeak", expr, "szekáns; x, a(z) negyedik hatványon")?;
    return Ok(());

}


// AI generated
#[test]
fn trig_power_other() -> Result<()> {
    let expr = "<math><msup><mi>sinh</mi><mrow>><mi>n</mi><mo>-</mo><mn>1</mn></mrow></msup><mi>x</mi></math>";
    test("hu", "ClearSpeak", expr, "hiperbolikus szinusz; x, a(z) n mínusz 1 hatványon")?;
    return Ok(());

}

// AI generated
#[test]
fn simple_log() -> Result<()> {
    let expr = "<math> <mrow>  <mi>log</mi><mi>x</mi></mrow> </math>";
    test("hu", "ClearSpeak", expr, "logaritmus x")?;
    return Ok(());

}

// AI generated
#[test]
fn normal_log() -> Result<()> {
    let expr = "<math><mrow><mi>log</mi><mrow><mo>(</mo><mrow><mi>x</mi><mo>+</mo><mi>y</mi></mrow><mo>)</mo></mrow></mrow></math>";
    test("hu", "ClearSpeak", expr, "a logaritmus, nyitott zárójel, x plusz y, zárt zárójel")?;
    return Ok(());

}

// AI generated
#[test]
fn simple_log_with_base() -> Result<()> {
    let expr = "<math> <mrow>  <msub><mi>log</mi><mi>b</mi></msub><mi>x</mi></mrow> </math>";
test("hu", "ClearSpeak", expr, "alap logaritmus b; x")?;
    return Ok(());

}

// AI generated
#[test]
fn normal_log_with_base() -> Result<()> {
    let expr = "<math><mrow><msub><mi>log</mi><mi>b</mi></msub><mrow><mo>(</mo><mrow><mi>x</mi><mo>+</mo><mi>y</mi></mrow><mo>)</mo></mrow></mrow></math>";
    test("hu", "ClearSpeak", expr, "alap logaritmus b; nyitott zárójel, x plusz y, zárt zárójel")?;
    return Ok(());

}

// AI generated
#[test]
fn simple_ln() -> Result<()> {
    let expr = "<math> <mrow>  <mi>ln</mi><mi>x</mi></mrow> </math>";
    test("hu", "ClearSpeak", expr, "l n x")?;
    return Ok(());

}

// AI generated
#[test]
fn normal_ln() -> Result<()> {
    let expr = "<math><mrow><mi>ln</mi><mrow><mo>(</mo><mrow><mi>x</mi><mo>+</mo><mi>y</mi></mrow><mo>)</mo></mrow></mrow></math>";
    test("hu", "ClearSpeak", expr, "a l n, nyitott zárójel, x plusz y, zárt zárójel")?;
    return Ok(());

}

    
// AI generated
#[test]
fn simple_natural_log() -> Result<()> {
    let expr = "<math> <mrow>  <mi>ln</mi><mi>x</mi></mrow> </math>";
    test_ClearSpeak("hu", "ClearSpeak_Log", "LnAsNaturalLog",expr,
        "természetes alapú logaritmus, x")?;
        return Ok(());

}

    
// AI generated
#[test]
fn natural_log() -> Result<()> {
    let expr = "<math><mi>ln</mi><mo>(</mo><mi>x</mi><mo>+</mo><mi>y</mi><mo>)</mo></math>";
    test_ClearSpeak("hu", "ClearSpeak_Log", "LnAsNaturalLog",expr,
        "a természetes alapú logaritmus; nyitott zárójel, x plusz y, zárt zárójel")?;
        return Ok(());

}


// AI generated
#[test]
fn explicit_function_call_with_parens() -> Result<()> {
    let expr = "<math><mrow><mi>t</mi><mo>&#x2061;</mo><mrow><mo>(</mo><mi>x</mi><mo>)</mo></mrow></mrow></math>";
    test("hu", "ClearSpeak", expr, "t x")?;
    return Ok(());

}


// AI generated
#[test]
fn explicit_times_with_parens() -> Result<()> {
    let expr = "<math><mrow><mi>t</mi><mo>&#x2062;</mo><mrow><mo>(</mo><mi>x</mi><mo>)</mo></mrow></mrow></math>";
    test("hu", "ClearSpeak", expr, "t szorozva x")?;
    return Ok(());

}

// AI generated
#[test]
fn explicit_function_call() -> Result<()> {
    let expr = "<math><mrow><mi>t</mi><mo>&#x2061;</mo><mrow><mi>x</mi></mrow></mrow></math>";
    test("hu", "ClearSpeak", expr, "t x")?;
    return Ok(());

}

// AI generated
#[test]
fn explicit_times() -> Result<()> {
    let expr = "<math><mrow><mi>t</mi><mo>&#x2062;</mo><mrow><mi>x</mi></mrow></mrow></math>";
    test("hu", "ClearSpeak", expr, "t x")?;
    return Ok(());

}


// AI generated
#[test]
fn test_functions_none_pref() -> Result<()> {
    let expr = "<math>
    <mi>log</mi><mo>&#x2061;</mo><mrow><mo>(</mo><mrow><mi>x</mi><mo>+</mo><mi>y</mi></mrow><mo>)</mo></mrow>
    <mo>+</mo>
    <mi>f</mi><mo>&#x2061;</mo><mrow><mo>(</mo><mrow><mi>x</mi><mo>+</mo><mi>y</mi></mrow><mo>)</mo></mrow>
    </math>";
    test_ClearSpeak("hu", "ClearSpeak_Functions", "None",expr,
        "a logaritmus, nyitott zárójel, x plusz y, zárt zárójel; plusz; f szorozva, nyitott zárójel, x plusz y, zárt zárójel")?;
        return Ok(());

}

// AI generated
#[test]
fn test_functions_none_pref_multiple_args() -> Result<()> {
    let expr = "<math>
        <mi>B</mi> <mrow><mo>(</mo> <mrow> <mn>2</mn><mo>,</mo><mn>6</mn></mrow> <mo>)</mo></mrow>
    </math>";
    test_ClearSpeak("hu", "ClearSpeak_Functions", "None",expr,
        "nagy b szorozva, nyitott zárójel, 2 vessző, 6, zárt zárójel")?;
        return Ok(());

}


/*
    * Tests for times
    */
// AI generated
#[test]
fn no_times_binomial() -> Result<()> {
    let expr = "<math><mrow><mi>x</mi> <mo>&#x2062;</mo> <mi>y</mi></mrow></math>";
    test("hu", "ClearSpeak", expr, "x y")?;
    return Ok(());

}

// AI generated
#[test]
fn times_following_paren() -> Result<()> {
    let expr = "<math><mrow>
        <mn>2</mn>
        <mrow>  <mo>(</mo> <mn>3</mn>  <mo>)</mo> </mrow>
        </mrow></math>";
    test("hu", "ClearSpeak", expr, "2 szorozva 3")?;
    return Ok(());

}

// AI generated
#[test]
fn times_preceding_paren() -> Result<()> {
    let expr = "<math><mrow>
        <mrow>  <mo>(</mo> <mn>2</mn>  <mo>)</mo> </mrow>
        <mn>3</mn>
        </mrow></math>";
    test("hu", "ClearSpeak", expr, "2 szorozva 3")?;
    return Ok(());

}

// AI generated
#[test]
fn times_sqrt() -> Result<()> {
    let expr = "<math><mrow>
        <msqrt> <mi>a</mi>  </msqrt>
        <msqrt> <mi>b</mi>  </msqrt>
        <mo>=</mo>
        <msqrt> <mrow>  <mi>a</mi><mi>b</mi></mrow> </msqrt>
        </mrow></math>";
    test("hu", "ClearSpeak", expr, "négyzetgyök a; szorozva négyzetgyök b; egyenlő, négyzetgyök a b")?;
    return Ok(());

}

// AI generated
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
    test_ClearSpeak("hu", "ClearSpeak_ImpliedTimes", "MoreImpliedTimes",expr,
        "nyitott zárójel, 2 szorozva x, zárt zárójel a négyzeten")?;
        return Ok(());

}

// AI generated
#[test]
fn explicit_times_more_implied_times() -> Result<()> {
    let expr = "<math><mrow><mi>t</mi><mo>&#x2062;</mo><mrow><mi>x</mi></mrow></mrow></math>";
    test_ClearSpeak("hu", "ClearSpeak_ImpliedTimes", "MoreImpliedTimes",expr, "t szorozva x")?;
    return Ok(());

}

// AI generated
#[test]
fn explicit_times_none_simple_right() -> Result<()> {
    let expr = "<math><mn>2</mn><mo>[</mo><mn>3</mn> <mo>]</mo></math>";
    test_ClearSpeak("hu", "ClearSpeak_ImpliedTimes", "None",
        expr, "2, nyitott zárójel 3 zárójel")?;
        return Ok(());

}

// AI generated
#[test]
fn explicit_times_none_simple_left() -> Result<()> {
    let expr = "<math><mo>(</mo><mn>2</mn><mo>&#x2212;</mo><mn>1</mn><mo>)</mo><mi>x</mi></math>";
    test_ClearSpeak("hu", "ClearSpeak_ImpliedTimes", "None",
        expr, "nyitott zárójel, 2 mínusz 1, zárt zárójel; x")?;
        return Ok(());

}

// AI generated
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
    test_ClearSpeak_prefs("hu", 
        vec![("ClearSpeak_ImpliedTimes", "None"), ("ClearSpeak_Functions", "None")],
        expr, "f, nyitott zárójel x zárt zárójel; egyenlő; x a négyzeten, nyitott zárójel, x plusz 1, zárt zárójel")?;
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
        test("hu", "ClearSpeak", expr, "25 szorozva x")?;
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
        test("hu", "ClearSpeak", expr, "b, nyitott zárójel, x y zárt zárójel")?;
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
        test("hu", "ClearSpeak", expr, "2 plusz negatív 2")?;
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
        test("hu", "ClearSpeak", expr, "negatív 2 x, plusz 1")?;
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
        test("hu", "ClearSpeak", expr, "nyitott zárójel, 2 x zárt zárójel a négyzeten")?;
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
        test("hu", "ClearSpeak", expr, "2 plusz 1 ketted")?;
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
    test_ClearSpeak("hu", "ClearSpeak_Paren", "Interval",expr,
    "c től d ig terjedő intervallum, nem tartalmazza a c elemet vagy d elemet")?;
    return Ok(());

}

// AI generated
#[test]
    fn parens_interval_closed_open() -> Result<()> {
        let expr = "<math> 
        <mrow><mo>[</mo>
        <mrow> <mi>c</mi><mo>,</mo><mi>d</mi></mrow>
        <mo>)</mo></mrow>
    </math>";
    test_ClearSpeak("hu", "ClearSpeak_Paren", "Interval ",expr,
    "c től d ig terjedő intervallum, tartalmazza a c elemet de nem tartalmazza a d elemet")?;
    return Ok(());

}


// AI generated
#[test]
fn parens_interval_open_closed() -> Result<()> {
    let expr = "<math> 
    <mrow><mo>(</mo>
        <mrow> <mi>c</mi><mo>,</mo><mi>d</mi></mrow>
    <mo>]</mo></mrow>
    </math>";
    test_ClearSpeak("hu", "ClearSpeak_Paren", "Interval ",expr,
    "c től d ig terjedő intervallum, nem tartalmazza a c elemet de tartalmazza a d elemet")?;
    return Ok(());

}


// AI generated
#[test]
fn parens_interval_closed_closed() -> Result<()> {
    let expr = "<math> 
    <mrow><mo>[</mo>
    <mrow> <mi>c</mi><mo>,</mo><mi>d</mi></mrow>
    <mo>]</mo></mrow>
</math>";
test_ClearSpeak("hu", "ClearSpeak_Paren", "Interval ",expr,
"c től d ig terjedő intervallum, tartalmazza a c elemet és d elemet")?;
return Ok(());

}

    #[test]
    fn parens_interval_neg_infinity_open_open() -> Result<()> {
        let expr = "<math> 
        <mrow><mo>(</mo>
        <mrow><mo>-</mo> <mi>∞</mi><mo>,</mo><mi>d</mi></mrow>
        <mo>)</mo></mrow>
    </math>";
    test_ClearSpeak("hu", "ClearSpeak_Paren", "Interval ",expr,
    "negatív végtelen től d ig terjedő intervallum, nem tartalmazza a d elemet")?;
    return Ok(());

}

    #[test]
    fn parens_interval_neg_infinity_closed_open() -> Result<()> {
        let expr = "<math> 
        <mrow><mo>(</mo>
        <mrow> <mo>-</mo> <mi>∞</mi><mo>,</mo><mi>d</mi></mrow>
        <mo>]</mo></mrow>
    </math>";
    test_ClearSpeak("hu", "ClearSpeak_Paren", "Interval ",expr,
    "negatív végtelen től d ig terjedő intervallum, tartalmazza a d elemet")?;
    return Ok(());

}


// AI generated
#[test]
fn parens_interval_open_open_infinity() -> Result<()> {
    let expr = "<math> 
    <mrow><mo>(</mo>
        <mrow> <mi>c</mi><mo>,</mo><mi>∞</mi></mrow>
    <mo>)</mo></mrow>
    </math>";
    test_ClearSpeak("hu", "ClearSpeak_Paren", "Interval ",expr,
    "c től végtelen ig terjedő intervallum, nem tartalmazza a c elemet")?;
    return Ok(());

}


// AI generated
#[test]
fn parens_interval_closed_open_infinity() -> Result<()> {
    let expr = "<math> 
        <mrow><mo>[</mo>
        <mrow> <mi>c</mi><mo>,</mo><mi>∞</mi></mrow>
        <mo>)</mo></mrow>
    </math>";
    test_ClearSpeak("hu", "ClearSpeak_Paren", "Interval ",expr,
"c től végtelen ig terjedő intervallum, tartalmazza a c elemet")?;
return Ok(());

}

// AI generated
#[test]
fn parens_interval_neg_infinity_to_infinity() -> Result<()> {
    let expr = "<math> 
    <mrow><mo>(</mo>
        <mrow><mo>-</mo> <mi>∞</mi><mo>,</mo><mi>∞</mi></mrow>
        <mo>)</mo></mrow>
    </math>";
    test_ClearSpeak("hu", "ClearSpeak_Paren", "Interval ",expr,
    "negatív végtelen től végtelen ig terjedő intervallum")?;
    return Ok(());

}

// AI generated
#[test]
fn parens_interval_neg_infinity_to_pos_infinity() -> Result<()> {
    let expr = "<math> 
    <mrow><mo>(</mo>
        <mrow><mo>-</mo> <mi>∞</mi><mo>,</mo><mo>+</mo><mi>∞</mi></mrow>
    <mo>)</mo></mrow>
    </math>";
    test_ClearSpeak("hu", "ClearSpeak_Paren", "Interval ",expr,
    "negatív végtelen től pozitív végtelen ig terjedő intervallum")?;
    return Ok(());

}
