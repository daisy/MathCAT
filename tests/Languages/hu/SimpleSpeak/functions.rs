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
    test("hu", "SimpleSpeak", expr, "szinusz x plusz koszinusz y, plusz tangens z plusz szekáns alfa, plusz koszekáns phi, plusz kotangens fí")?;
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
    test("hu", "SimpleSpeak", expr, "hiperbolikus szinusz, x, plusz hiperbolikus koszinusz, y, plusz hiperbolikus tangens, z, plusz, hiperbolikus szekáns, alfa, plusz, hiperbolikus koszekáns, phi, plusz, hiperbolikus kotangens, fí")?;
                                return Ok(());

}


// AI generated
#[test]
fn inverse_trig() -> Result<()> {
    let expr = "<math><msup><mi>sin</mi><mrow><mo>-</mo><mn>1</mn></mrow></msup><mi>x</mi></math>";
    test("hu", "SimpleSpeak", expr, "inverz szinusz x")?;
    return Ok(());

}

// AI generated
#[test]
fn trig_squared() -> Result<()> {
    let expr = "<math><msup><mi>sin</mi><mn>2</mn></msup><mi>x</mi></math>";
    test("hu", "SimpleSpeak", expr, "szinusz x a négyzeten")?;
    return Ok(());

}

// AI generated
#[test]
fn trig_cubed() -> Result<()> {
    let expr = "<math><msup><mi>tan</mi><mn>3</mn></msup><mi>x</mi></math>";
    test("hu", "SimpleSpeak", expr, "tangens x a köbön")?;
    return Ok(());

}

// AI generated
#[test]
fn trig_fourth() -> Result<()> {
    let expr = "<math><msup><mi>sec</mi><mn>4</mn></msup><mi>x</mi></math>";
    test("hu", "SimpleSpeak", expr, "szekáns; x, a(z) negyedik hatványon")?;
    return Ok(());

}


// AI generated
#[test]
fn trig_power_other() -> Result<()> {
    let expr = "<math><msup><mi>sinh</mi><mrow>><mi>n</mi><mo>-</mo><mn>1</mn></mrow></msup><mi>x</mi></math>";
    test("hu", "SimpleSpeak", expr, "hiperbolikus szinusz; x, a(z) n mínusz 1 hatványon")?;
    return Ok(());

}

// AI generated
#[test]
fn simple_log() -> Result<()> {
    let expr = "<math> <mrow>  <mi>log</mi><mi>x</mi></mrow> </math>";
    test("hu", "SimpleSpeak", expr, "logaritmus x")?;
    return Ok(());

}

// AI generated
#[test]
fn normal_log() -> Result<()> {
    let expr = "<math><mrow><mi>log</mi><mrow><mo>(</mo><mrow><mi>x</mi><mo>+</mo><mi>y</mi></mrow><mo>)</mo></mrow></mrow></math>";
    test("hu", "SimpleSpeak", expr, "logaritmus, nyitott zárójel, x plusz y, zárt zárójel")?;
    return Ok(());

}

// AI generated
#[test]
fn simple_log_with_base() -> Result<()> {
    let expr = "<math> <mrow>  <msub><mi>log</mi><mi>b</mi></msub><mi>x</mi></mrow> </math>";
    test("hu", "SimpleSpeak", expr, "alap logaritmus b; x")?;
    return Ok(());

}

// AI generated
#[test]
fn normal_log_with_base() -> Result<()> {
    let expr = "<math><mrow><msub><mi>log</mi><mi>b</mi></msub><mrow><mo>(</mo><mrow><mi>x</mi><mo>+</mo><mi>y</mi></mrow><mo>)</mo></mrow></mrow></math>";
    test("hu", "SimpleSpeak", expr, "alap logaritmus b; nyitott zárójel, x plusz y, zárt zárójel")?;
    return Ok(());

}

// AI generated
#[test]
fn normal_ln() -> Result<()> {
    let expr = "<math><mrow><mi>ln</mi><mrow><mo>(</mo><mrow><mi>x</mi><mo>+</mo><mi>y</mi></mrow><mo>)</mo></mrow></mrow></math>";
    test_prefs("hu", "SimpleSpeak", vec![("Verbosity", "Terse")],
                expr, "l n, nyitott zárójel, x plusz y, zárt zárójel")?;
    test_prefs("hu", "SimpleSpeak", vec![("Verbosity", "Medium")],
                expr, "természetes alapú logaritmus; nyitott zárójel, x plusz y, zárt zárójel")?;
    test_prefs("hu", "SimpleSpeak", vec![("Verbosity", "Verbose")],
                expr, "természetes alapú logaritmus; nyitott zárójel, x plusz y, zárt zárójel")?;
                return Ok(());

}

// AI generated
#[test]
fn simple_ln() -> Result<()> {
    let expr = "<math> <mrow>  <mi>ln</mi><mi>x</mi></mrow> </math>";
    test_prefs("hu", "SimpleSpeak", vec![("Verbosity", "Terse")],
                expr, "l n x")?;
    test_prefs("hu", "SimpleSpeak", vec![("Verbosity", "Medium")],
                expr, "természetes alapú logaritmus, x")?;
    test_prefs("hu", "SimpleSpeak", vec![("Verbosity", "Verbose")],
                expr, "természetes alapú logaritmus, x")?;
                return Ok(());

}

// AI generated
#[test]
fn other_names() -> Result<()> {
    let expr = "<math> <mrow><mi>Cov</mi><mi>x</mi></mrow> </math>";
    test_prefs("hu", "SimpleSpeak", vec![("Verbosity", "Terse")],
                expr, "kovariancia x")?;
    test_prefs("hu", "SimpleSpeak", vec![("Verbosity", "Medium")],
                expr, "kovariancia x")?;
    let expr = "<math> <mrow><mi>exp</mi><mo>(</mo><mi>x</mi><mo>)</mo></mrow> </math>";
    test_prefs("hu", "SimpleSpeak", vec![("Verbosity", "Terse")],
                expr, "exponenciális x")?;
    test_prefs("hu", "SimpleSpeak", vec![("Verbosity", "Medium")],
                expr, "exponenciális x")?;
                return Ok(());

}

// AI generated
#[test]
fn explicit_function_call_with_parens() -> Result<()> {
    let expr = "<math><mrow><mi>t</mi><mo>&#x2061;</mo><mrow><mo>(</mo><mi>x</mi><mo>)</mo></mrow></mrow></math>";
    test("hu", "SimpleSpeak", expr, "t x")?;
    return Ok(());

}


// AI generated
#[test]
fn explicit_times_with_parens() -> Result<()> {
    let expr = "<math><mrow><mi>t</mi><mo>&#x2062;</mo><mrow><mo>(</mo><mi>x</mi><mo>)</mo></mrow></mrow></math>";
    test("hu", "SimpleSpeak", expr, "t szorozva x")?;
    return Ok(());

}

// AI generated
#[test]
fn explicit_function_call() -> Result<()> {
    let expr = "<math><mrow><mi>t</mi><mo>&#x2061;</mo><mrow><mi>x</mi></mrow></mrow></math>";
    test("hu", "SimpleSpeak", expr, "t x")?;
    return Ok(());

}

// AI generated
#[test]
fn explicit_times() -> Result<()> {
    let expr = "<math><mrow><mi>t</mi><mo>&#x2062;</mo><mrow><mi>x</mi></mrow></mrow></math>";
    test("hu", "SimpleSpeak", expr, "t x")?;
    return Ok(());

}


/*
    * Tests for times
    */
// AI generated
#[test]
fn no_times_binomial() -> Result<()> {
    let expr = "<math><mrow><mi>x</mi> <mo>&#x2062;</mo> <mi>y</mi></mrow></math>";
    test("hu", "SimpleSpeak", expr, "x y")?;
    return Ok(());

}

// AI generated
#[test]
fn times_following_paren() -> Result<()> {
    let expr = "<math><mrow>
        <mn>2</mn>
        <mrow>  <mo>(</mo> <mn>3</mn>  <mo>)</mo> </mrow>
        </mrow></math>";
    test("hu", "SimpleSpeak", expr, "2 szorozva 3")?;
    return Ok(());

}

// AI generated
#[test]
fn times_preceding_paren() -> Result<()> {
    let expr = "<math><mrow>
        <mrow>  <mo>(</mo> <mn>2</mn>  <mo>)</mo> </mrow>
        <mn>3</mn>
        </mrow></math>";
    test("hu", "SimpleSpeak", expr, "2 szorozva 3")?;
    return Ok(());

}

// AI generated
#[test]
fn no_times_sqrt() -> Result<()> {
    let expr = "<math><mrow>
        <msqrt> <mi>a</mi>  </msqrt>
        <msqrt> <mi>b</mi>  </msqrt>
        <mo>=</mo>
        <msqrt> <mrow>  <mi>a</mi><mi>b</mi></mrow> </msqrt>
        </mrow></math>";
    test("hu", "SimpleSpeak", expr, 
            "a négyzetgyöke ennek: a; szorozva a négyzetgyöke ennek: b; egyenlő, a négyzetgyöke ennek: a b, gyök vége")?;
    test_prefs("hu", "SimpleSpeak", vec![("Impairment", "LearningDisability")], expr,
            "a négyzetgyöke ennek: a; szorozva a négyzetgyöke ennek: b; egyenlő, a négyzetgyöke ennek: a b")?;
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
        test("hu", "SimpleSpeak", expr, "25 szorozva x")?;
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
        test("hu", "SimpleSpeak", expr, "b, nyitott zárójel, x y zárt zárójel")?;
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
        test("hu", "SimpleSpeak", expr, "2 plusz negatív 2")?;
        return Ok(());

    }


    #[test]
    fn no_parens_negative_number_with_var() -> Result<()> {
        let expr = "<math><mrow>
        <mrow><mo>(</mo>
        <mrow><mo>&#x2212;</mo><mn>2</mn></mrow><mi>x</mi>
        <mo>)</mo></mrow>
        <mo>+</mo><mn>1</mn>
        </mrow></math>";
        test("hu", "SimpleSpeak", expr, "negatív 2 x, plusz 1")?;
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
        test("hu", "SimpleSpeak", expr, "nyitott zárójel, 2 x zárt zárójel a négyzeten")?;
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
        test("hu", "SimpleSpeak", expr, "2 plusz 1 ketted")?;
        return Ok(());

    }


    // Tests for the four types of intervals in SimpleSpeak
    #[test]
    fn parens_interval_open_open() -> Result<()> {
        let expr = "<math> 
        <mrow intent='open-interval($start, $end)'><mo>(</mo>
        <mrow> <mo arg='open'>(</mo><mi arg='start'>c</mi><mo>,</mo><mi arg='end'>d</mi></mrow><mo arg='close'>)</mo>
        <mo>)</mo></mrow>
    </math>";
    test("hu", "SimpleSpeak",expr, "c től d ig terjedő intervallum, nem tartalmazza a c elemet vagy d elemet")?;
    return Ok(());

}

// AI generated
#[test]
    fn parens_interval_closed_open() -> Result<()> {
        let expr = "<math> 
        <mrow intent='closed-open-interval($start, $end)'><mo>[</mo>
            <mrow> <mo arg='open'>[(]</mo><mi arg='start'>c</mi><mo>,</mo><mi arg='end'>d</mi></mrow><mo arg='close'>)</mo>
            <mo>)</mo></mrow>
        </math>";
    test("hu", "SimpleSpeak",expr, "c től d ig terjedő intervallum, tartalmazza a c elemet de nem tartalmazza a d elemet")?;
    return Ok(());

}


// AI generated
#[test]
fn parens_interval_open_closed() -> Result<()> {
    let expr = "<math> 
    <mrow intent='open-closed-interval($start, $end)'><mo>(</mo>
        <mrow> <mo arg='open'>(</mo><mi arg='start'>c</mi><mo>,</mo><mi arg='end'>d</mi></mrow><mo arg='close'>]</mo>
        <mo>]</mo></mrow>
    </math>";
    test("hu", "SimpleSpeak",expr,"c től d ig terjedő intervallum, nem tartalmazza a c elemet de tartalmazza a d elemet")?;
    return Ok(());

}


// AI generated
#[test]
fn parens_interval_closed_closed() -> Result<()> {
    let expr = "<math> 
        <mrow intent='closed-interval($start, $end)'><mo>[</mo>
            <mrow> <mo arg='open'>[(]</mo><mi arg='start'>c</mi><mo>,</mo><mi arg='end'>d</mi></mrow><mo arg='close'>]</mo>
            <mo>]</mo></mrow>
    </math>";
    test("hu", "SimpleSpeak",expr, "c től d ig terjedő intervallum, tartalmazza a c elemet és d elemet")?;
    return Ok(());

}

    #[test]
    fn parens_interval_neg_infinity_open_open() -> Result<()> {
        let expr = "<math> 
        <mrow intent='open-interval($start, $end)'><mo arg='open'>(</mo>
        <mrow><mrow arg='start'><mo>-</mo> <mi>∞</mi></mrow><mo>,</mo><mi arg='end'>d</mi></mrow><mo arg='close'>)</mo>
        <mo>)</mo></mrow>
    </math>";
    test("hu", "SimpleSpeak",expr,
    "negatív végtelen től d ig terjedő intervallum, nem tartalmazza a d elemet")?;
    return Ok(());

}

    #[test]
    fn parens_interval_neg_infinity_open_closed() -> Result<()> {
        let expr = "<math> 
        <mrow intent='open-closed-interval($start, $end)'><mo arg='open'>(</mo>
        <mrow><mrow arg='start'><mo>-</mo> <mi>∞</mi></mrow><mo>,</mo><mi arg='end'>d</mi></mrow><mo arg='close'>]</mo>
        <mo>]</mo></mrow>
    </math>";
    test("hu", "SimpleSpeak",expr,
    "negatív végtelen től d ig terjedő intervallum, tartalmazza a d elemet")?;
    return Ok(());

}

