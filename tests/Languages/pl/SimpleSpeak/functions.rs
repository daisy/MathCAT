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
    test("pl", "SimpleSpeak", expr, "sinus z x plus cosinus z y plus tangens z z plus sekans z alfa, plus kosekans z fi plus cotangens z fi")?;
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
    test("pl", "SimpleSpeak", expr, "sinus hiperboliczny z x, plus cosinus hiperboliczny z y, plus tangens hiperboliczny z z, plus sekans hiperboliczny z alfa, plus kosekans hiperboliczny z fi, plus cotangens hiperboliczny z fi")?;
                                return Ok(());

}


#[test]
fn inverse_trig() -> Result<()> {
    let expr = "<math><msup><mi>sin</mi><mrow><mo>-</mo><mn>1</mn></mrow></msup><mi>x</mi></math>";
    test("pl", "SimpleSpeak", expr, "odwrotność sinus z x")?;
    return Ok(());

}

#[test]
fn trig_squared() -> Result<()> {
    let expr = "<math><msup><mi>sin</mi><mn>2</mn></msup><mi>x</mi></math>";
    test("pl", "SimpleSpeak", expr, "sinus do kwadratu z x")?;
    return Ok(());

}

#[test]
fn trig_cubed() -> Result<()> {
    let expr = "<math><msup><mi>tan</mi><mn>3</mn></msup><mi>x</mi></math>";
    test("pl", "SimpleSpeak", expr, "tangens do sześcianu z x")?;
    return Ok(());

}

#[test]
fn trig_fourth() -> Result<()> {
    let expr = "<math><msup><mi>sec</mi><mn>4</mn></msup><mi>x</mi></math>";
    test("pl", "SimpleSpeak", expr, "sekans do potęgi czwartej, z x")?;
    return Ok(());

}


#[test]
fn trig_power_other() -> Result<()> {
    let expr = "<math><msup><mi>sinh</mi><mrow>><mi>n</mi><mo>-</mo><mn>1</mn></mrow></msup><mi>x</mi></math>";
    test("pl", "SimpleSpeak", expr, "sinus hiperboliczny do potęgi n minus 1, z x")?;
    return Ok(());

}

#[test]
fn simple_log() -> Result<()> {
    let expr = "<math> <mrow>  <mi>log</mi><mi>x</mi></mrow> </math>";
    test("pl", "SimpleSpeak", expr, "logarytm z x")?;
    return Ok(());

}

#[test]
fn principal_value_log() -> Result<()> {
    let expr = "<math><mrow><mi>Log</mi><mi>z</mi></mrow></math>";
    test("pl", "SimpleSpeak", expr, "Log z z")?;
    return Ok(());

}

#[test]
fn normal_log() -> Result<()> {
    let expr = "<math><mrow><mi>log</mi><mrow><mo>(</mo><mrow><mi>x</mi><mo>+</mo><mi>y</mi></mrow><mo>)</mo></mrow></mrow></math>";
    test("pl", "SimpleSpeak", expr, "logarytm z, nawias otwierający, x plus y, nawias zamykający")?;
    return Ok(());

}

#[test]
fn simple_log_with_base() -> Result<()> {
    let expr = "<math> <mrow>  <msub><mi>log</mi><mi>b</mi></msub><mi>x</mi></mrow> </math>";
    test("pl", "SimpleSpeak", expr, "logarytm o podstawie b, z x")?;
    return Ok(());

}

#[test]
fn normal_log_with_base() -> Result<()> {
    let expr = "<math><mrow><msub><mi>log</mi><mi>b</mi></msub><mrow><mo>(</mo><mrow><mi>x</mi><mo>+</mo><mi>y</mi></mrow><mo>)</mo></mrow></mrow></math>";
    test("pl", "SimpleSpeak", expr, "logarytm o podstawie b, z, nawias otwierający, x plus y, nawias zamykający")?;
    return Ok(());

}

#[test]
fn normal_ln() -> Result<()> {
    let expr = "<math><mrow><mi>ln</mi><mrow><mo>(</mo><mrow><mi>x</mi><mo>+</mo><mi>y</mi></mrow><mo>)</mo></mrow></mrow></math>";
    test_prefs("pl", "SimpleSpeak", vec![("Verbosity", "Terse")],
                expr, "l n, otwórz x plus y zamknij")?;
    test_prefs("pl", "SimpleSpeak", vec![("Verbosity", "Medium")],
                expr, "logarytm naturalny z, nawias otwierający, x plus y, nawias zamykający")?;
    test_prefs("pl", "SimpleSpeak", vec![("Verbosity", "Verbose")],
                expr, "logarytm naturalny z, nawias otwierający, x plus y, nawias zamykający")?;
                return Ok(());

}

#[test]
fn simple_ln() -> Result<()> {
    let expr = "<math> <mrow>  <mi>ln</mi><mi>x</mi></mrow> </math>";
    test_prefs("pl", "SimpleSpeak", vec![("Verbosity", "Terse")],
                expr, "l n x")?;
    test_prefs("pl", "SimpleSpeak", vec![("Verbosity", "Medium")],
                expr, "logarytm naturalny z x")?;
    test_prefs("pl", "SimpleSpeak", vec![("Verbosity", "Verbose")],
                expr, "logarytm naturalny z x")?;
                return Ok(());

}

#[test]
fn other_names() -> Result<()> {
    let expr = "<math> <mrow><mi>Cov</mi><mi>x</mi></mrow> </math>";
    test_prefs("pl", "SimpleSpeak", vec![("Verbosity", "Terse")],
                expr, "kowariancja x")?;
    test_prefs("pl", "SimpleSpeak", vec![("Verbosity", "Medium")],
                expr, "kowariancja x")?;
    let expr = "<math> <mrow><mi>exp</mi><mo>(</mo><mi>x</mi><mo>)</mo></mrow> </math>";
    test_prefs("pl", "SimpleSpeak", vec![("Verbosity", "Terse")],
                expr, "eksponenta x")?;
    test_prefs("pl", "SimpleSpeak", vec![("Verbosity", "Medium")],
                expr, "funkcja wykładnicza z x")?;
                return Ok(());

}

#[test]
fn explicit_function_call_with_parens() -> Result<()> {
    let expr = "<math><mrow><mi>t</mi><mo>&#x2061;</mo><mrow><mo>(</mo><mi>x</mi><mo>)</mo></mrow></mrow></math>";
    test("pl", "SimpleSpeak", expr, "t z x")?;
    return Ok(());

}


#[test]
fn explicit_times_with_parens() -> Result<()> {
    let expr = "<math><mrow><mi>t</mi><mo>&#x2062;</mo><mrow><mo>(</mo><mi>x</mi><mo>)</mo></mrow></mrow></math>";
    test("pl", "SimpleSpeak", expr, "t razy x")?;
    return Ok(());

}

#[test]
fn explicit_function_call() -> Result<()> {
    let expr = "<math><mrow><mi>t</mi><mo>&#x2061;</mo><mrow><mi>x</mi></mrow></mrow></math>";
    test("pl", "SimpleSpeak", expr, "t z x")?;
    return Ok(());

}

#[test]
fn explicit_times() -> Result<()> {
    let expr = "<math><mrow><mi>t</mi><mo>&#x2062;</mo><mrow><mi>x</mi></mrow></mrow></math>";
    test("pl", "SimpleSpeak", expr, "t x")?;
    return Ok(());

}


/*
    * Tests for times
    */
#[test]
fn no_times_binomial() -> Result<()> {
    let expr = "<math><mrow><mi>x</mi> <mo>&#x2062;</mo> <mi>y</mi></mrow></math>";
    test("pl", "SimpleSpeak", expr, "x y")?;
    return Ok(());

}

#[test]
fn times_following_paren() -> Result<()> {
    let expr = "<math><mrow>
        <mn>2</mn>
        <mrow>  <mo>(</mo> <mn>3</mn>  <mo>)</mo> </mrow>
        </mrow></math>";
    test("pl", "SimpleSpeak", expr, "2 razy 3")?;
    return Ok(());

}

#[test]
fn times_preceding_paren() -> Result<()> {
    let expr = "<math><mrow>
        <mrow>  <mo>(</mo> <mn>2</mn>  <mo>)</mo> </mrow>
        <mn>3</mn>
        </mrow></math>";
    test("pl", "SimpleSpeak", expr, "2 razy 3")?;
    return Ok(());

}

#[test]
fn no_times_sqrt() -> Result<()> {
    let expr = "<math><mrow>
        <msqrt> <mi>a</mi>  </msqrt>
        <msqrt> <mi>b</mi>  </msqrt>
        <mo>=</mo>
        <msqrt> <mrow>  <mi>a</mi><mi>b</mi></mrow> </msqrt>
        </mrow></math>";
    test("pl", "SimpleSpeak", expr, 
            "pierwiastek kwadratowy z a; razy pierwiastek kwadratowy z b; równa się, pierwiastek kwadratowy z a b, koniec pierwiastka")?;
    test_prefs("pl", "SimpleSpeak", vec![("Impairment", "LearningDisability")], expr,
            "pierwiastek kwadratowy z a; razy pierwiastek kwadratowy z b; równa się, pierwiastek kwadratowy z a b")?;
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
        test("pl", "SimpleSpeak", expr, "25 razy x")?;
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
        test("pl", "SimpleSpeak", expr, "b, nawias otwierający, x y nawias zamykający")?;
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
        test("pl", "SimpleSpeak", expr, "2 plus minus 2")?;
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
        test("pl", "SimpleSpeak", expr, "minus 2 x, plus 1")?;
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
        test("pl", "SimpleSpeak", expr, "nawias otwierający, 2 x nawias zamykający do kwadratu")?;
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
        test("pl", "SimpleSpeak", expr, "2 plus jedna druga")?;
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
    test("pl", "SimpleSpeak",expr, "przedział otwarty od c do d")?;
    return Ok(());

}

#[test]
    fn parens_interval_closed_open() -> Result<()> {
        let expr = "<math> 
        <mrow intent='closed-open-interval($start, $end)'><mo>[</mo>
            <mrow> <mo arg='open'>[(]</mo><mi arg='start'>c</mi><mo>,</mo><mi arg='end'>d</mi></mrow><mo arg='close'>)</mo>
            <mo>)</mo></mrow>
        </math>";
    test("pl", "SimpleSpeak",expr, "przedział lewostronnie domknięty prawostronnie otwarty od c do d")?;
    return Ok(());

}


#[test]
fn parens_interval_open_closed() -> Result<()> {
    let expr = "<math> 
    <mrow intent='open-closed-interval($start, $end)'><mo>(</mo>
        <mrow> <mo arg='open'>(</mo><mi arg='start'>c</mi><mo>,</mo><mi arg='end'>d</mi></mrow><mo arg='close'>]</mo>
        <mo>]</mo></mrow>
    </math>";
    test("pl", "SimpleSpeak",expr,"przedział lewostronnie otwarty prawostronnie domknięty od c do d")?;
    return Ok(());

}


#[test]
fn parens_interval_closed_closed() -> Result<()> {
    let expr = "<math> 
        <mrow intent='closed-interval($start, $end)'><mo>[</mo>
            <mrow> <mo arg='open'>[(]</mo><mi arg='start'>c</mi><mo>,</mo><mi arg='end'>d</mi></mrow><mo arg='close'>]</mo>
            <mo>]</mo></mrow>
    </math>";
    test("pl", "SimpleSpeak",expr, "przedział domknięty od c do d")?;
    return Ok(());

}

    #[test]
    fn parens_interval_neg_infinity_open_open() -> Result<()> {
        let expr = "<math> 
        <mrow intent='open-interval($start, $end)'><mo arg='open'>(</mo>
        <mrow><mrow arg='start'><mo>-</mo> <mi>∞</mi></mrow><mo>,</mo><mi arg='end'>d</mi></mrow><mo arg='close'>)</mo>
        <mo>)</mo></mrow>
    </math>";
    test("pl", "SimpleSpeak",expr,
    "przedział otwarty od minus nieskończoność do d")?;
    return Ok(());

}

    #[test]
    fn parens_interval_neg_infinity_open_closed() -> Result<()> {
        let expr = "<math> 
        <mrow intent='open-closed-interval($start, $end)'><mo arg='open'>(</mo>
        <mrow><mrow arg='start'><mo>-</mo> <mi>∞</mi></mrow><mo>,</mo><mi arg='end'>d</mi></mrow><mo arg='close'>]</mo>
        <mo>]</mo></mrow>
    </math>";
    test("pl", "SimpleSpeak",expr,
    "przedział lewostronnie otwarty prawostronnie domknięty od minus nieskończoność do d")?;
    return Ok(());

}

