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
    test("es", "SimpleSpeak", expr, "seno de x más coseno de y más tangente de z más secante de alfa, más cosecante de fi, más cotangente de fi")?;
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
    test("es", "SimpleSpeak", expr, "seno hiperbólico de x, más coseno hiperbólico de y, más tangente hiperbólica de z, más secante hiperbólica de alfa, más cosecante hiperbólica de fi, más cotangente hiperbólica de fi")?;
                                return Ok(());

}


#[test]
fn inverse_trig() -> Result<()> {
    let expr = "<math><msup><mi>sin</mi><mrow><mo>-</mo><mn>1</mn></mrow></msup><mi>x</mi></math>";
    test("es", "SimpleSpeak", expr, "seno inverso de x")?;
    return Ok(());

}

// prueba propia del español: "inverso"/"inversa" concuerda con la función
#[test]
fn inverse_trig_gender() -> Result<()> {
    let expr = "<math><msup><mi>tan</mi><mrow><mo>-</mo><mn>1</mn></mrow></msup><mi>x</mi></math>";
    test("es", "SimpleSpeak", expr, "tangente inversa de x")?;
    let expr = "<math><msup><mi>cos</mi><mrow><mo>-</mo><mn>1</mn></mrow></msup><mi>x</mi></math>";
    test("es", "SimpleSpeak", expr, "coseno inverso de x")?;
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
    test("es", "SimpleSpeak", expr, "tangente al cubo de x")?;
    return Ok(());

}

#[test]
fn trig_fourth() -> Result<()> {
    let expr = "<math><msup><mi>sec</mi><mn>4</mn></msup><mi>x</mi></math>";
    test("es", "SimpleSpeak", expr, "la cuarta potencia de, secante de x")?;
    return Ok(());

}


#[test]
fn trig_power_other() -> Result<()> {
    let expr = "<math><msup><mi>sinh</mi><mrow>><mi>n</mi><mo>-</mo><mn>1</mn></mrow></msup><mi>x</mi></math>";
    test("es", "SimpleSpeak", expr, "la potencia n menos 1 de, seno hiperbólico de x")?;
    return Ok(());

}

#[test]
fn simple_log() -> Result<()> {
    let expr = "<math> <mrow>  <mi>log</mi><mi>x</mi></mrow> </math>";
    test("es", "SimpleSpeak", expr, "el logaritmo de x")?;
    return Ok(());

}

#[test]
fn principal_value_log() -> Result<()> {
    let expr = "<math><mrow><mi>Log</mi><mi>z</mi></mrow></math>";
    test("es", "SimpleSpeak", expr, "el logaritmo principal de z")?;
    return Ok(());

}

#[test]
fn normal_log() -> Result<()> {
    let expr = "<math><mrow><mi>log</mi><mrow><mo>(</mo><mrow><mi>x</mi><mo>+</mo><mi>y</mi></mrow><mo>)</mo></mrow></mrow></math>";
    test("es", "SimpleSpeak", expr, "el logaritmo de, abrir paréntesis, x más y, cerrar paréntesis")?;
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
    test("es", "SimpleSpeak", expr, "el logaritmo en base b, de, abrir paréntesis, x más y, cerrar paréntesis")?;
    return Ok(());

}

#[test]
fn normal_ln() -> Result<()> {
    let expr = "<math><mrow><mi>ln</mi><mrow><mo>(</mo><mrow><mi>x</mi><mo>+</mo><mi>y</mi></mrow><mo>)</mo></mrow></mrow></math>";
    test_prefs("es", "SimpleSpeak", vec![("Verbosity", "Terse")],
                expr, "l n, abrir paréntesis, x más y, cerrar paréntesis")?;
    test_prefs("es", "SimpleSpeak", vec![("Verbosity", "Medium")],
                expr, "el logaritmo natural de, abrir paréntesis, x más y, cerrar paréntesis")?;
    test_prefs("es", "SimpleSpeak", vec![("Verbosity", "Verbose")],
                expr, "el logaritmo natural de, abrir paréntesis, x más y, cerrar paréntesis")?;
                return Ok(());

}

#[test]
fn simple_ln() -> Result<()> {
    let expr = "<math> <mrow>  <mi>ln</mi><mi>x</mi></mrow> </math>";
    test_prefs("es", "SimpleSpeak", vec![("Verbosity", "Terse")],
                expr, "l n x")?;
    test_prefs("es", "SimpleSpeak", vec![("Verbosity", "Medium")],
                expr, "el logaritmo natural de x")?;
    test_prefs("es", "SimpleSpeak", vec![("Verbosity", "Verbose")],
                expr, "el logaritmo natural de x")?;
                return Ok(());

}

#[test]
fn other_names() -> Result<()> {
    let expr = "<math> <mrow><mi>Cov</mi><mi>x</mi></mrow> </math>";
    test_prefs("es", "SimpleSpeak", vec![("Verbosity", "Terse")],
                expr, "cov x")?;
    test_prefs("es", "SimpleSpeak", vec![("Verbosity", "Medium")],
                expr, "covarianza x")?;
    let expr = "<math> <mrow><mi>exp</mi><mo>(</mo><mi>x</mi><mo>)</mo></mrow> </math>";
    test_prefs("es", "SimpleSpeak", vec![("Verbosity", "Terse")],
                expr, "exp x")?;
    test_prefs("es", "SimpleSpeak", vec![("Verbosity", "Medium")],
                expr, "exponencial de x")?;
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
            "la raíz cuadrada de a; por la raíz cuadrada de b; es igual a, la raíz cuadrada de a b, fin de raíz")?;
    test_prefs("es", "SimpleSpeak", vec![("Impairment", "LearningDisability")], expr,
            "la raíz cuadrada de a; por la raíz cuadrada de b; es igual a, la raíz cuadrada de a b")?;
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
        test("es", "SimpleSpeak", expr, "b, abrir paréntesis, x y cerrar paréntesis")?;
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
        test("es", "SimpleSpeak", expr, "2 más 2 negativo")?;
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
        test("es", "SimpleSpeak", expr, "abrir paréntesis, 2 x cerrar paréntesis al cuadrado")?;
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
        test("es", "SimpleSpeak", expr, "2 más un medio")?;
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
    test("es", "SimpleSpeak",expr, "el intervalo abierto de extremos c y d")?;
    return Ok(());

}

#[test]
    fn parens_interval_closed_open() -> Result<()> {
        let expr = "<math> 
        <mrow intent='closed-open-interval($start, $end)'><mo>[</mo>
            <mrow> <mo arg='open'>[(]</mo><mi arg='start'>c</mi><mo>,</mo><mi arg='end'>d</mi></mrow><mo arg='close'>)</mo>
            <mo>)</mo></mrow>
        </math>";
    test("es", "SimpleSpeak",expr, "el intervalo abierto por la derecha de extremos c y d")?;
    return Ok(());

}


#[test]
fn parens_interval_open_closed() -> Result<()> {
    let expr = "<math> 
    <mrow intent='open-closed-interval($start, $end)'><mo>(</mo>
        <mrow> <mo arg='open'>(</mo><mi arg='start'>c</mi><mo>,</mo><mi arg='end'>d</mi></mrow><mo arg='close'>]</mo>
        <mo>]</mo></mrow>
    </math>";
    test("es", "SimpleSpeak",expr,"el intervalo abierto por la izquierda de extremos c y d")?;
    return Ok(());

}


#[test]
fn parens_interval_closed_closed() -> Result<()> {
    let expr = "<math> 
        <mrow intent='closed-interval($start, $end)'><mo>[</mo>
            <mrow> <mo arg='open'>[(]</mo><mi arg='start'>c</mi><mo>,</mo><mi arg='end'>d</mi></mrow><mo arg='close'>]</mo>
            <mo>]</mo></mrow>
    </math>";
    test("es", "SimpleSpeak",expr, "el intervalo cerrado de extremos c y d")?;
    return Ok(());

}

    #[test]
    fn parens_interval_neg_infinity_open_open() -> Result<()> {
        let expr = "<math> 
        <mrow intent='open-interval($start, $end)'><mo arg='open'>(</mo>
        <mrow><mrow arg='start'><mo>-</mo> <mi>∞</mi></mrow><mo>,</mo><mi arg='end'>d</mi></mrow><mo arg='close'>)</mo>
        <mo>)</mo></mrow>
    </math>";
    test("es", "SimpleSpeak",expr,
    "el intervalo abierto de extremos menos infinito y d")?;
    return Ok(());

}

    #[test]
    fn parens_interval_neg_infinity_open_closed() -> Result<()> {
        let expr = "<math> 
        <mrow intent='open-closed-interval($start, $end)'><mo arg='open'>(</mo>
        <mrow><mrow arg='start'><mo>-</mo> <mi>∞</mi></mrow><mo>,</mo><mi arg='end'>d</mi></mrow><mo arg='close'>]</mo>
        <mo>]</mo></mrow>
    </math>";
    test("es", "SimpleSpeak",expr,
    "el intervalo abierto por la izquierda de extremos menos infinito y d")?;
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

#[test]
fn trig_names_terse() -> Result<()> {
    let expr = "<math><mi>sen</mi><mo>&#x2061;</mo><mi>x</mi><mo>+</mo><mi>sinh</mi><mo>&#x2061;</mo><mi>y</mi><mo>+</mo><mi>tgh</mi><mo>&#x2061;</mo><mi>z</mi></math>";
    test_prefs("es", "SimpleSpeak", vec![("Verbosity", "Terse")],
                expr, "sen x más sen hache y, más tan hache z")?;
    test_prefs("es", "SimpleSpeak", vec![("Verbosity", "Medium")],
                expr, "seno de x más seno hiperbólico de y, más tangente hiperbólica de z")?;
    return Ok(());
}

#[test]
fn arc_trig_names() -> Result<()> {
    let expr = "<math><mi>arcsen</mi><mo>&#x2061;</mo><mi>x</mi><mo>+</mo><mi>arctan</mi><mo>&#x2061;</mo><mi>y</mi><mo>+</mo><mi>argsenh</mi><mo>&#x2061;</mo><mi>z</mi></math>";
    test_prefs("es", "SimpleSpeak", vec![("Verbosity", "Medium")],
                expr, "arco seno de x, más arco tangente de y, más, argumento seno hiperbólico de z")?;
    test_prefs("es", "SimpleSpeak", vec![("Verbosity", "Terse")],
                expr, "arco seno x, más arco tangente y, más, argumento seno hiperbólico, z")?;
    return Ok(());
}

#[test]
fn arc_trig_babel_split() -> Result<()> {
    // babel en español escribe \arcsen como "arc\,sen" (parche temporal en general.yaml)
    let expr = "<math><mrow><mi mathvariant='normal'>arc</mi><mspace width='0.167em'></mspace><mi mathvariant='normal'>sen</mi></mrow><mspace width='0.167em'></mspace><mi>x</mi></math>";
    test("es", "SimpleSpeak", expr, "arco seno de x")?;
    let expr = "<math><mi>arc</mi><mspace width='0.167em'></mspace><mi>tg</mi><mspace width='0.167em'></mspace><mo>(</mo><mi>x</mi><mo>+</mo><mn>1</mn><mo>)</mo></math>";
    test_prefs("es", "SimpleSpeak", vec![("Verbosity", "Terse")],
                expr, "arco tangente; abrir paréntesis, x más 1, cerrar paréntesis")?;
    return Ok(());
}

#[test]
fn max_min_spanish() -> Result<()> {
    let expr = "<math><mi>máx</mi><mo>(</mo><mi>a</mi><mo>,</mo><mi>b</mi><mo>)</mo></math>";
    test("es", "SimpleSpeak", expr, "el máximo de, abrir paréntesis, a coma, b, cerrar paréntesis")?;
    let expr = "<math><msub><mi>m</mi><mi>mín</mi></msub></math>";
    test("es", "SimpleSpeak", expr, "m sub mín fin de sub")?;
    return Ok(());
}

#[test]
fn signs_without_positivo_negativo() -> Result<()> {
    // sin paréntesis los signos se leen "más" y "menos"; un número con signo entre paréntesis,
    // como en el colegio: (−2) "2 negativo", (+1) "1 positivo"
    let expr = "<math><mo>+</mo><mn>3</mn></math>";
    test("es", "SimpleSpeak", expr, "más 3")?;
    let expr = "<math><mo>-</mo><mn>1</mn><mo>+</mo><mn>2</mn></math>";
    test("es", "SimpleSpeak", expr, "menos 1 más 2")?;
    let expr = "<math><mn>3</mn><mo>-</mo><mo>(</mo><mo>-</mo><mn>2</mn><mo>)</mo></math>";
    test("es", "SimpleSpeak", expr, "3 menos 2 negativo")?;
    let expr = "<math><mo>(</mo><mo>-</mo><mn>2</mn><mo>)</mo><mo>+</mo><mo>(</mo><mo>+</mo><mn>1</mn><mo>)</mo></math>";
    test("es", "SimpleSpeak", expr, "2 negativo más 1 positivo")?;
    let expr = "<math><msup><mrow><mo>(</mo><mo>-</mo><mn>2</mn><mo>)</mo></mrow><mn>2</mn></msup></math>";
    test("es", "SimpleSpeak", expr, "2 negativo al cuadrado")?;
    return Ok(());
}
