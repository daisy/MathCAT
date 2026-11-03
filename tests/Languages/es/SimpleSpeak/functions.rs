/// Tests for:
/// *  functions including trig functions, logs, and functions to powers
/// *  implied times/functional call and explicit times/function call
/// *  parens
/// These are all intertwined, so they are in one file
use crate::common::*;
use anyhow::Result;

// REVISAR (phi recta, inglés: phi): «seno de x más coseno de y más tangente de z más secante de alfa, más cosecante de phi recta, más cotangente de phi»
#[test]
#[ignore = "lectura en español pendiente de revisión"]
fn trig_names() -> Result<()> {
    let expr = "<math><mrow>
    <mi>sin</mi><mi>x</mi><mo>+</mo>
    <mi>cos</mi><mi>y</mi><mo>+</mo>
    <mi>tan</mi><mi>z</mi><mo>+</mo>
    <mi>sec</mi><mi>&#x03B1;</mi><mo>+</mo>
    <mi>csc</mi><mi>&#x03D5;</mi><mo>+</mo>
    <mi>cot</mi><mi>&#x03C6;</mi>
    </mrow></math>";
    test("es", "SimpleSpeak", expr, "seno de x más coseno de y más tangente de z más secante de alfa, más cosecante de phi recta, más cotangente de phi")?;
    return Ok(());

}

// REVISAR (phi recta, inglés: phi): «seno hiperbólico de x, más coseno hiperbólico de y, más tangente hiperbólica de z, más secante hiperbólica de alfa, más, cosecante hiperbólica de phi recta; más, cotangente hiperbólica de phi»
#[test]
#[ignore = "lectura en español pendiente de revisión"]
fn hyperbolic_trig_names() -> Result<()> {
    let expr = "<math><mrow>
    <mi>sinh</mi><mi>x</mi><mo>+</mo>
    <mi>cosh</mi><mi>y</mi><mo>+</mo>
    <mi>tanh</mi><mi>z</mi><mo>+</mo>
    <mi>sech</mi><mi>&#x03B1;</mi><mo>+</mo>
    <mi>csch</mi><mi>&#x03D5;</mi><mo>+</mo>
    <mi>coth</mi><mi>&#x03C6;</mi>
    </mrow></math>";
    test("es", "SimpleSpeak", expr, "seno hiperbólico de x, más coseno hiperbólico de y, más tangente hiperbólica de z, más secante hiperbólica de alfa, más, cosecante hiperbólica de phi recta; más, cotangente hiperbólica de phi")?;
                                return Ok(());

}


#[test]
fn inverse_trig() -> Result<()> {
    let expr = "<math><msup><mi>sin</mi><mrow><mo>-</mo><mn>1</mn></mrow></msup><mi>x</mi></math>";
    test("es", "SimpleSpeak", expr, "inverso seno de x")?;
    return Ok(());

}

#[test]
fn trig_squared() -> Result<()> {
    let expr = "<math><msup><mi>sin</mi><mn>2</mn></msup><mi>x</mi></math>";
    test("es", "SimpleSpeak", expr, "seno al cuadrado de x")?;
    return Ok(());

}

#[test]
fn trig_cubed() -> Result<()> {
    let expr = "<math><msup><mi>tan</mi><mn>3</mn></msup><mi>x</mi></math>";
    test("es", "SimpleSpeak", expr, "tangente cúbica de x")?;
    return Ok(());

}

// REVISAR (cuarto potencia): «la cuarto potencia de, secante de x»
#[test]
#[ignore = "lectura en español pendiente de revisión"]
fn trig_fourth() -> Result<()> {
    let expr = "<math><msup><mi>sec</mi><mn>4</mn></msup><mi>x</mi></math>";
    test("es", "SimpleSpeak", expr, "la cuarto potencia de, secante de x")?;
    return Ok(());

}


#[test]
fn trig_power_other() -> Result<()> {
    let expr = "<math><msup><mi>sinh</mi><mrow>><mi>n</mi><mo>-</mo><mn>1</mn></mrow></msup><mi>x</mi></math>";
    test("es", "SimpleSpeak", expr, "la n menos 1 potencia de, seno hiperbólico de x")?;
    return Ok(());

}

#[test]
fn simple_log() -> Result<()> {
    let expr = "<math> <mrow>  <mi>log</mi><mi>x</mi></mrow> </math>";
    test("es", "SimpleSpeak", expr, "el log de x")?;
    return Ok(());

}

#[test]
fn principal_value_log() -> Result<()> {
    let expr = "<math><mrow><mi>Log</mi><mi>z</mi></mrow></math>";
    test("es", "SimpleSpeak", expr, "Log de z")?;
    return Ok(());

}

#[test]
fn normal_log() -> Result<()> {
    let expr = "<math><mrow><mi>log</mi><mrow><mo>(</mo><mrow><mi>x</mi><mo>+</mo><mi>y</mi></mrow><mo>)</mo></mrow></mrow></math>";
    test("es", "SimpleSpeak", expr, "el log de, se abren paréntesis, x más y, se cierran paréntesis")?;
    return Ok(());

}

#[test]
fn simple_log_with_base() -> Result<()> {
    let expr = "<math> <mrow>  <msub><mi>log</mi><mi>b</mi></msub><mi>x</mi></mrow> </math>";
    test("es", "SimpleSpeak", expr, "el logaritmo en base b, de x")?;
    return Ok(());

}

#[test]
fn normal_log_with_base() -> Result<()> {
    let expr = "<math><mrow><msub><mi>log</mi><mi>b</mi></msub><mrow><mo>(</mo><mrow><mi>x</mi><mo>+</mo><mi>y</mi></mrow><mo>)</mo></mrow></mrow></math>";
    test("es", "SimpleSpeak", expr, "el logaritmo en base b, de, se abren paréntesis, x más y, se cierran paréntesis")?;
    return Ok(());

}

// REVISAR (inglés: natural): «el natural log de, se abren paréntesis, x más y, se cierran paréntesis»
// REVISAR (inglés: natural): «el natural log de, se abren paréntesis, x más y, se cierran paréntesis»
#[test]
#[ignore = "lectura en español pendiente de revisión"]
fn normal_ln() -> Result<()> {
    let expr = "<math><mrow><mi>ln</mi><mrow><mo>(</mo><mrow><mi>x</mi><mo>+</mo><mi>y</mi></mrow><mo>)</mo></mrow></mrow></math>";
    test_prefs("es", "SimpleSpeak", vec![("Verbosity", "Terse")],
                expr, "l n, abrir paréntesis, x más y, cerrar paréntesis")?;
    test_prefs("es", "SimpleSpeak", vec![("Verbosity", "Medium")],
                expr, "el natural log de, se abren paréntesis, x más y, se cierran paréntesis")?;
    test_prefs("es", "SimpleSpeak", vec![("Verbosity", "Verbose")],
                expr, "el natural log de, se abren paréntesis, x más y, se cierran paréntesis")?;
                return Ok(());

}

// REVISAR (inglés: natural): «el natural log de x»
// REVISAR (inglés: natural): «el natural log de x»
#[test]
#[ignore = "lectura en español pendiente de revisión"]
fn simple_ln() -> Result<()> {
    let expr = "<math> <mrow>  <mi>ln</mi><mi>x</mi></mrow> </math>";
    test_prefs("es", "SimpleSpeak", vec![("Verbosity", "Terse")],
                expr, "l n x")?;
    test_prefs("es", "SimpleSpeak", vec![("Verbosity", "Medium")],
                expr, "el natural log de x")?;
    test_prefs("es", "SimpleSpeak", vec![("Verbosity", "Verbose")],
                expr, "el natural log de x")?;
                return Ok(());

}

// REVISAR (inglés: cov): «Cov x»
// REVISAR (inglés: cov): «Cov x»
// REVISAR (inglés: exp): «exp x»
// REVISAR (inglés: exp): «exp de x»
#[test]
#[ignore = "lectura en español pendiente de revisión"]
fn other_names() -> Result<()> {
    let expr = "<math> <mrow><mi>Cov</mi><mi>x</mi></mrow> </math>";
    test_prefs("es", "SimpleSpeak", vec![("Verbosity", "Terse")],
                expr, "Cov x")?;
    test_prefs("es", "SimpleSpeak", vec![("Verbosity", "Medium")],
                expr, "Cov x")?;
    let expr = "<math> <mrow><mi>exp</mi><mo>(</mo><mi>x</mi><mo>)</mo></mrow> </math>";
    test_prefs("es", "SimpleSpeak", vec![("Verbosity", "Terse")],
                expr, "exp x")?;
    test_prefs("es", "SimpleSpeak", vec![("Verbosity", "Medium")],
                expr, "exp de x")?;
                return Ok(());

}

#[test]
fn explicit_function_call_with_parens() -> Result<()> {
    let expr = "<math><mrow><mi>t</mi><mo>&#x2061;</mo><mrow><mo>(</mo><mi>x</mi><mo>)</mo></mrow></mrow></math>";
    test("es", "SimpleSpeak", expr, "t de x")?;
    return Ok(());

}


#[test]
fn explicit_times_with_parens() -> Result<()> {
    let expr = "<math><mrow><mi>t</mi><mo>&#x2062;</mo><mrow><mo>(</mo><mi>x</mi><mo>)</mo></mrow></mrow></math>";
    test("es", "SimpleSpeak", expr, "t por x")?;
    return Ok(());

}

#[test]
fn explicit_function_call() -> Result<()> {
    let expr = "<math><mrow><mi>t</mi><mo>&#x2061;</mo><mrow><mi>x</mi></mrow></mrow></math>";
    test("es", "SimpleSpeak", expr, "t de x")?;
    return Ok(());

}

#[test]
fn explicit_times() -> Result<()> {
    let expr = "<math><mrow><mi>t</mi><mo>&#x2062;</mo><mrow><mi>x</mi></mrow></mrow></math>";
    test("es", "SimpleSpeak", expr, "t x")?;
    return Ok(());

}


/*
    * Tests for times
    */
#[test]
fn no_times_binomial() -> Result<()> {
    let expr = "<math><mrow><mi>x</mi> <mo>&#x2062;</mo> <mi>y</mi></mrow></math>";
    test("es", "SimpleSpeak", expr, "x y")?;
    return Ok(());

}

#[test]
fn times_following_paren() -> Result<()> {
    let expr = "<math><mrow>
        <mn>2</mn>
        <mrow>  <mo>(</mo> <mn>3</mn>  <mo>)</mo> </mrow>
        </mrow></math>";
    test("es", "SimpleSpeak", expr, "2 por 3")?;
    return Ok(());

}

#[test]
fn times_preceding_paren() -> Result<()> {
    let expr = "<math><mrow>
        <mrow>  <mo>(</mo> <mn>2</mn>  <mo>)</mo> </mrow>
        <mn>3</mn>
        </mrow></math>";
    test("es", "SimpleSpeak", expr, "2 por 3")?;
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
    test("es", "SimpleSpeak", expr, 
            "la raíz cuadrada de a; la raíz cuadrada de b; es igual a, la raíz cuadrada de a b fin de raíz")?;
    test_prefs("es", "SimpleSpeak", vec![("Impairment", "LearningDisability")], expr,
            "la raíz cuadrada de a; la raíz cuadrada de b; es igual a, la raíz cuadrada de a b fin de raíz")?;
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
        test("es", "SimpleSpeak", expr, "25 por x")?;
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
        test("es", "SimpleSpeak", expr, "b x y")?;
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
        test("es", "SimpleSpeak", expr, "2 más menos 2")?;
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
        test("es", "SimpleSpeak", expr, "menos 2 x, más 1")?;
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
        test("es", "SimpleSpeak", expr, "se abren paréntesis, 2 x, se cierran paréntesis al cuadrado")?;
        return Ok(());

    }

    // REVISAR (mitad): «2 más 1 mitad»
#[test]
#[ignore = "lectura en español pendiente de revisión"]
    fn no_parens_fraction() -> Result<()> {
        let expr = "<math><mrow>
        <mn>2</mn>
        <mo>+</mo>
        <mrow>
            <mrow><mo>(</mo>
            <mfrac> <mn>1</mn><mn>2</mn></mfrac>
            <mo>)</mo></mrow></mrow>
    </mrow></math>";
        test("es", "SimpleSpeak", expr, "2 más 1 mitad")?;
        return Ok(());

    }


    // Tests for the four types of intervals in SimpleSpeak
    // REVISAR (inglés: interval open): «el open interval de c a d»
#[test]
#[ignore = "lectura en español pendiente de revisión"]
    fn parens_interval_open_open() -> Result<()> {
        let expr = "<math> 
        <mrow intent='open-interval($start, $end)'><mo>(</mo>
        <mrow> <mo arg='open'>(</mo><mi arg='start'>c</mi><mo>,</mo><mi arg='end'>d</mi></mrow><mo arg='close'>)</mo>
        <mo>)</mo></mrow>
    </math>";
    test("es", "SimpleSpeak",expr, "el open interval de c a d")?;
    return Ok(());

}

// REVISAR (inglés: closed interval open): «el closed open interval de c a d»
#[test]
#[ignore = "lectura en español pendiente de revisión"]
    fn parens_interval_closed_open() -> Result<()> {
        let expr = "<math> 
        <mrow intent='closed-open-interval($start, $end)'><mo>[</mo>
            <mrow> <mo arg='open'>[(]</mo><mi arg='start'>c</mi><mo>,</mo><mi arg='end'>d</mi></mrow><mo arg='close'>)</mo>
            <mo>)</mo></mrow>
        </math>";
    test("es", "SimpleSpeak",expr, "el closed open interval de c a d")?;
    return Ok(());

}


// REVISAR (inglés: closed interval open): «el open closed interval de c a d»
#[test]
#[ignore = "lectura en español pendiente de revisión"]
fn parens_interval_open_closed() -> Result<()> {
    let expr = "<math> 
    <mrow intent='open-closed-interval($start, $end)'><mo>(</mo>
        <mrow> <mo arg='open'>(</mo><mi arg='start'>c</mi><mo>,</mo><mi arg='end'>d</mi></mrow><mo arg='close'>]</mo>
        <mo>]</mo></mrow>
    </math>";
    test("es", "SimpleSpeak",expr,"el open closed interval de c a d")?;
    return Ok(());

}


// REVISAR (inglés: closed interval): «el closed interval de c a d»
#[test]
#[ignore = "lectura en español pendiente de revisión"]
fn parens_interval_closed_closed() -> Result<()> {
    let expr = "<math> 
        <mrow intent='closed-interval($start, $end)'><mo>[</mo>
            <mrow> <mo arg='open'>[(]</mo><mi arg='start'>c</mi><mo>,</mo><mi arg='end'>d</mi></mrow><mo arg='close'>]</mo>
            <mo>]</mo></mrow>
    </math>";
    test("es", "SimpleSpeak",expr, "el closed interval de c a d")?;
    return Ok(());

}

    // REVISAR (inglés: interval open): «el open interval de menos infinito a d»
#[test]
#[ignore = "lectura en español pendiente de revisión"]
    fn parens_interval_neg_infinity_open_open() -> Result<()> {
        let expr = "<math> 
        <mrow intent='open-interval($start, $end)'><mo arg='open'>(</mo>
        <mrow><mrow arg='start'><mo>-</mo> <mi>∞</mi></mrow><mo>,</mo><mi arg='end'>d</mi></mrow><mo arg='close'>)</mo>
        <mo>)</mo></mrow>
    </math>";
    test("es", "SimpleSpeak",expr,
    "el open interval de menos infinito a d")?;
    return Ok(());

}

    // REVISAR (inglés: closed interval open): «el open closed interval de menos infinito a d»
#[test]
#[ignore = "lectura en español pendiente de revisión"]
    fn parens_interval_neg_infinity_open_closed() -> Result<()> {
        let expr = "<math> 
        <mrow intent='open-closed-interval($start, $end)'><mo arg='open'>(</mo>
        <mrow><mrow arg='start'><mo>-</mo> <mi>∞</mi></mrow><mo>,</mo><mi arg='end'>d</mi></mrow><mo arg='close'>]</mo>
        <mo>]</mo></mrow>
    </math>";
    test("es", "SimpleSpeak",expr,
    "el open closed interval de menos infinito a d")?;
    return Ok(());

}

// Tests propios del español (no existen en inglés)
#[test]
fn normal_ln_terse() -> Result<()> {
    let expr = "<math><mrow><mi>ln</mi><mrow><mo>(</mo><mrow><mi>x</mi><mo>+</mo><mi>y</mi></mrow><mo>)</mo></mrow></mrow></math>";
    test_prefs("es", "SimpleSpeak", vec![("Verbosity", "Terse")],
                expr, "l n, abrir paréntesis, x más y, cerrar paréntesis")?;
                return Ok(());

}

#[test]
fn simple_ln_terse() -> Result<()> {
    let expr = "<math> <mrow>  <mi>ln</mi><mi>x</mi></mrow> </math>";
    test_prefs("es", "SimpleSpeak", vec![("Verbosity", "Terse")],
                expr, "l n x")?;
                return Ok(());

}
