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
    test("fr", "SimpleSpeak", expr, "sinus de x plus cosinus de y plus tangente de z plus sécante de alpha, plus cosécante de phi droit, plus cotangente de phi")?;
    Ok(())
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
    test("fr", "SimpleSpeak", expr, "sinus hyperbolique de x, plus cosinus hyperbolique de y, plus tangente hyperbolique de z, plus, sécante hyperbolique de alpha, plus, cosécante hyperbolique de phi droit; plus, cotangente hyperbolique de phi")?;
                                Ok(())
}


// AI generated
#[test]
fn inverse_trig() -> Result<()> {
    let expr = "<math><msup><mi>sin</mi><mrow><mo>-</mo><mn>1</mn></mrow></msup><mi>x</mi></math>";
    test("fr", "SimpleSpeak", expr, "inverse sinus de x")?;
    Ok(())
}

// AI generated
#[test]
fn trig_squared() -> Result<()> {
    let expr = "<math><msup><mi>sin</mi><mn>2</mn></msup><mi>x</mi></math>";
    test("fr", "SimpleSpeak", expr, "sinus au carré de x")?;
    Ok(())
}

// AI generated
#[test]
fn trig_cubed() -> Result<()> {
    let expr = "<math><msup><mi>tan</mi><mn>3</mn></msup><mi>x</mi></math>";
    test("fr", "SimpleSpeak", expr, "tangente au cube de x")?;
    Ok(())
}

// AI generated
#[test]
fn trig_fourth() -> Result<()> {
    let expr = "<math><msup><mi>sec</mi><mn>4</mn></msup><mi>x</mi></math>";
    test("fr", "SimpleSpeak", expr, "4 puissance de, sécante de x")?;
    Ok(())
}


// AI generated
#[test]
fn trig_power_other() -> Result<()> {
    let expr = "<math><msup><mi>sinh</mi><mrow>><mi>n</mi><mo>-</mo><mn>1</mn></mrow></msup><mi>x</mi></math>";
    test("fr", "SimpleSpeak", expr, "n moins 1 puissance de, sinus hyperbolique de x")?;
    Ok(())
}

// AI generated
#[test]
fn simple_log() -> Result<()> {
    let expr = "<math> <mrow>  <mi>log</mi><mi>x</mi></mrow> </math>";
    test("fr", "SimpleSpeak", expr, "le log de x")?;
    Ok(())
}

// AI generated
#[test]
fn normal_log() -> Result<()> {
    let expr = "<math><mrow><mi>log</mi><mrow><mo>(</mo><mrow><mi>x</mi><mo>+</mo><mi>y</mi></mrow><mo>)</mo></mrow></mrow></math>";
    test("fr", "SimpleSpeak", expr, "le log de, parenthèse gauche, x plus y, parenthèse droite")?;
    Ok(())
}

// AI generated
#[test]
fn simple_log_with_base() -> Result<()> {
    let expr = "<math> <mrow>  <msub><mi>log</mi><mi>b</mi></msub><mi>x</mi></mrow> </math>";
    test("fr", "SimpleSpeak", expr, "le log en base b, de x")?;
    Ok(())
}

// AI generated
#[test]
fn normal_log_with_base() -> Result<()> {
    let expr = "<math><mrow><msub><mi>log</mi><mi>b</mi></msub><mrow><mo>(</mo><mrow><mi>x</mi><mo>+</mo><mi>y</mi></mrow><mo>)</mo></mrow></mrow></math>";
    test("fr", "SimpleSpeak", expr, "le log en base b, de, parenthèse gauche, x plus y, parenthèse droite")?;
    Ok(())
}

// AI generated
#[test]
fn normal_ln() -> Result<()> {
    let expr = "<math><mrow><mi>ln</mi><mrow><mo>(</mo><mrow><mi>x</mi><mo>+</mo><mi>y</mi></mrow><mo>)</mo></mrow></mrow></math>";
    test_prefs("fr", "SimpleSpeak", vec![("Verbosity", "Terse")], expr, "l n, ouvert x plus y fermer")?;
    test_prefs("fr", "SimpleSpeak", vec![("Verbosity", "Medium")], expr, "le logarithme naturel de, parenthèse gauche, x plus y, parenthèse droite")?;
    test_prefs("fr", "SimpleSpeak", vec![("Verbosity", "Verbose")], expr, "le logarithme naturel de, parenthèse gauche, x plus y, parenthèse droite")?;
                Ok(())
}

// AI generated
#[test]
fn simple_ln() -> Result<()> {
    let expr = "<math> <mrow>  <mi>ln</mi><mi>x</mi></mrow> </math>";
    test_prefs("fr", "SimpleSpeak", vec![("Verbosity", "Terse")], expr, "l n x")?;
    test_prefs("fr", "SimpleSpeak", vec![("Verbosity", "Medium")], expr, "le logarithme naturel de x")?;
    test_prefs("fr", "SimpleSpeak", vec![("Verbosity", "Verbose")], expr, "le logarithme naturel de x")?;
                Ok(())
}

// AI generated
#[test]
fn other_names() -> Result<()> {
    let expr = "<math> <mrow><mi>Cov</mi><mi>x</mi></mrow> </math>";
    test_prefs("fr", "SimpleSpeak", vec![("Verbosity", "Terse")], expr, "Cov x")?;
    test_prefs("fr", "SimpleSpeak", vec![("Verbosity", "Medium")], expr, "covariance x")?;
    let expr = "<math> <mrow><mi>exp</mi><mo>(</mo><mi>x</mi><mo>)</mo></mrow> </math>";
    test_prefs("fr", "SimpleSpeak", vec![("Verbosity", "Terse")], expr, "exp x")?;
    test_prefs("fr", "SimpleSpeak", vec![("Verbosity", "Medium")], expr, "exponentielle de x")?;
                Ok(())
}

// AI generated
#[test]
fn explicit_function_call_with_parens() -> Result<()> {
    let expr = "<math><mrow><mi>t</mi><mo>&#x2061;</mo><mrow><mo>(</mo><mi>x</mi><mo>)</mo></mrow></mrow></math>";
    test("fr", "SimpleSpeak", expr, "t de x")?;
    Ok(())
}


// AI generated
#[test]
fn explicit_times_with_parens() -> Result<()> {
    let expr = "<math><mrow><mi>t</mi><mo>&#x2062;</mo><mrow><mo>(</mo><mi>x</mi><mo>)</mo></mrow></mrow></math>";
    test("fr", "SimpleSpeak", expr, "t fois x")?;
    Ok(())
}

// AI generated
#[test]
fn explicit_function_call() -> Result<()> {
    let expr = "<math><mrow><mi>t</mi><mo>&#x2061;</mo><mrow><mi>x</mi></mrow></mrow></math>";
    test("fr", "SimpleSpeak", expr, "t de x")?;
    Ok(())
}

// AI generated
#[test]
fn explicit_times() -> Result<()> {
    let expr = "<math><mrow><mi>t</mi><mo>&#x2062;</mo><mrow><mi>x</mi></mrow></mrow></math>";
    test("fr", "SimpleSpeak", expr, "t x")?;
    Ok(())
}


/*
    * Tests for times
    */
// AI generated
#[test]
fn no_times_binomial() -> Result<()> {
    let expr = "<math><mrow><mi>x</mi> <mo>&#x2062;</mo> <mi>y</mi></mrow></math>";
    test("fr", "SimpleSpeak", expr, "x y")?;
    Ok(())
}

// AI generated
#[test]
fn times_following_paren() -> Result<()> {
    let expr = "<math><mrow>
        <mn>2</mn>
        <mrow>  <mo>(</mo> <mn>3</mn>  <mo>)</mo> </mrow>
        </mrow></math>";
    test("fr", "SimpleSpeak", expr, "2 fois 3")?;
    Ok(())
}

// AI generated
#[test]
fn times_preceding_paren() -> Result<()> {
    let expr = "<math><mrow>
        <mrow>  <mo>(</mo> <mn>2</mn>  <mo>)</mo> </mrow>
        <mn>3</mn>
        </mrow></math>";
    test("fr", "SimpleSpeak", expr, "2 fois 3")?;
    Ok(())
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
    test("fr", "SimpleSpeak", expr, "la racine carrée de a; fois la racine carrée de b; est égal à, la racine carrée de a b, fin de racine")?;
    test_prefs("fr", "SimpleSpeak", vec![("Impairment", "LearningDisability")], expr, "la racine carrée de a; fois la racine carrée de b; est égal à, la racine carrée de a b")?;
            Ok(())
}

/*
    * Tests for parens
    */
    // AI generated
    #[test]
    fn no_parens_number() -> Result<()> {
        let expr = "<math><mrow>
        <mrow><mo>(</mo>
        <mn>25</mn>
        <mo>)</mo></mrow>
        <mi>x</mi>
        </mrow></math>";
        test("fr", "SimpleSpeak", expr, "25 fois x")?;
        Ok(())
    }

    // AI generated
    #[test]
    fn no_parens_monomial() -> Result<()> {
        let expr = "<math><mrow>
        <mi>b</mi>
        <mrow><mo>(</mo>
        <mrow><mi>x</mi><mi>y</mi></mrow>
        <mo>)</mo></mrow>
        </mrow></math>";
        test("fr", "SimpleSpeak", expr, "b, parenthèse gauche, x y parenthèse droite")?;
        Ok(())
    }

    // AI generated
    #[test]
    fn no_parens_negative_number() -> Result<()> {
        let expr = "<math><mrow>
        <mn>2</mn><mo>+</mo>
        <mrow><mo>(</mo>
        <mrow><mo>&#x2212;</mo><mn>2</mn></mrow>
        <mo>)</mo></mrow>
        </mrow></math>";
        test("fr", "SimpleSpeak", expr, "2 plus négatif 2")?;
        Ok(())
    }


    // AI generated
    #[test]
    fn no_parens_negative_number_with_var() -> Result<()> {
        let expr = "<math><mrow>
        <mrow><mo>(</mo>
        <mrow><mo>&#x2212;</mo><mn>2</mn></mrow><mi>x</mi>
        <mo>)</mo></mrow>
        <mo>+</mo><mn>1</mn>
        </mrow></math>";
        test("fr", "SimpleSpeak", expr, "négatif 2 x, plus 1")?;
        Ok(())
    }

    // AI generated
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
        test("fr", "SimpleSpeak", expr, "parenthèse gauche, 2 x parenthèse droite au carré")?;
        Ok(())
    }

    // AI generated
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
        test("fr", "SimpleSpeak", expr, "2 plus 1 demi")?;
        Ok(())
    }


    // Tests for the four types of intervals in SimpleSpeak
    // AI generated
    #[test]
    fn parens_interval_open_open() -> Result<()> {
        let expr = "<math> 
        <mrow intent='open-interval($start, $end)'><mo>(</mo>
        <mrow> <mo arg='open'>(</mo><mi arg='start'>c</mi><mo>,</mo><mi arg='end'>d</mi></mrow><mo arg='close'>)</mo>
        <mo>)</mo></mrow>
    </math>";
    test("fr", "SimpleSpeak", expr, "open interval de c comma, d")?;
    Ok(())
}

// AI generated
#[test]
    fn parens_interval_closed_open() -> Result<()> {
        let expr = "<math> 
        <mrow intent='closed-open-interval($start, $end)'><mo>[</mo>
            <mrow> <mo arg='open'>[(]</mo><mi arg='start'>c</mi><mo>,</mo><mi arg='end'>d</mi></mrow><mo arg='close'>)</mo>
            <mo>)</mo></mrow>
        </math>";
    test("fr", "SimpleSpeak", expr, "closed open interval de c comma, d")?;
    Ok(())
}


// AI generated
#[test]
fn parens_interval_open_closed() -> Result<()> {
    let expr = "<math> 
    <mrow intent='open-closed-interval($start, $end)'><mo>(</mo>
        <mrow> <mo arg='open'>(</mo><mi arg='start'>c</mi><mo>,</mo><mi arg='end'>d</mi></mrow><mo arg='close'>]</mo>
        <mo>]</mo></mrow>
    </math>";
    test("fr", "SimpleSpeak", expr, "open closed interval de c comma, d")?;
    Ok(())
}


// AI generated
#[test]
fn parens_interval_closed_closed() -> Result<()> {
    let expr = "<math> 
        <mrow intent='closed-interval($start, $end)'><mo>[</mo>
            <mrow> <mo arg='open'>[(]</mo><mi arg='start'>c</mi><mo>,</mo><mi arg='end'>d</mi></mrow><mo arg='close'>]</mo>
            <mo>]</mo></mrow>
    </math>";
    test("fr", "SimpleSpeak", expr, "closed interval de c comma, d")?;
    Ok(())
}

    // AI generated
    #[test]
    fn parens_interval_neg_infinity_open_open() -> Result<()> {
        let expr = "<math> 
        <mrow intent='open-interval($start, $end)'><mo arg='open'>(</mo>
        <mrow><mrow arg='start'><mo>-</mo> <mi>∞</mi></mrow><mo>,</mo><mi arg='end'>d</mi></mrow><mo arg='close'>)</mo>
        <mo>)</mo></mrow>
    </math>";
    test("fr", "SimpleSpeak", expr, "open interval de négatif infini comma, d")?;
    Ok(())
}

    // AI generated
    #[test]
    fn parens_interval_neg_infinity_open_closed() -> Result<()> {
        let expr = "<math> 
        <mrow intent='open-closed-interval($start, $end)'><mo arg='open'>(</mo>
        <mrow><mrow arg='start'><mo>-</mo> <mi>∞</mi></mrow><mo>,</mo><mi arg='end'>d</mi></mrow><mo arg='close'>]</mo>
        <mo>]</mo></mrow>
    </math>";
    test("fr", "SimpleSpeak", expr, "open closed interval de négatif infini comma, d")?;
    Ok(())
}

