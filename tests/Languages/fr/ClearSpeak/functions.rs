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
    test("fr", "ClearSpeak", expr, "sinus x plus cosinus y plus tangente z plus sécante alpha, plus cosécante phi, plus cotangente phi")?;
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
    test("fr", "ClearSpeak", expr, "sinus hyperbolique de x, plus \
                                cosinus hyperbolique de y, plus \
                                tangente hyperbolique de z, plus, \
                                sécante hyperbolique de alpha, plus, \
                                cosécante hyperbolique de phi, plus, \
                                cotangente hyperbolique de phi")?;
                                return Ok(());

}


#[test]
fn inverse_trig() -> Result<()> {
    let expr = "<math><msup><mi>sin</mi><mrow><mo>-</mo><mn>1</mn></mrow></msup><mi>x</mi></math>";
    test("fr", "ClearSpeak", expr, "sinus inverse de x")?;
    return Ok(());

}

#[test]
fn inverse_trig_trig_inverse() -> Result<()> {
    let expr = "<math><msup><mi>tan</mi><mrow><mo>-</mo><mn>1</mn></mrow></msup><mi>x</mi></math>";
    test_ClearSpeak("fr", "ClearSpeak_Trig", "TrigInverse",expr,
        "tangente inverse de x")?;
        return Ok(());

}

#[test]
fn inverse_trig_arc() -> Result<()> {
    let expr = "<math><msup><mi>cosh</mi><mrow><mo>-</mo><mn>1</mn></mrow></msup><mi>x</mi></math>";
    test_ClearSpeak("fr", "ClearSpeak_Trig", "ArcTrig",expr,
        "arc cosinus hyperbolique de x")?;
        return Ok(());

}

#[test]
fn trig_squared() -> Result<()> {
    let expr = "<math><msup><mi>sin</mi><mn>2</mn></msup><mi>x</mi></math>";
    test("fr", "ClearSpeak", expr, "sinus au carré de x")?;
    return Ok(());

}

#[test]
fn trig_cubed() -> Result<()> {
    let expr = "<math><msup><mi>tan</mi><mn>3</mn></msup><mi>x</mi></math>";
    test("fr", "ClearSpeak", expr, "tangente au cube de x")?;
    return Ok(());

}

#[test]
fn trig_fourth() -> Result<()> {
    let expr = "<math><msup><mi>sec</mi><mn>4</mn></msup><mi>x</mi></math>";
    test("fr", "ClearSpeak", expr, "la quatrième puissance de, sécante de x")?;
    return Ok(());

}


#[test]
fn trig_power_other() -> Result<()> {
    let expr = "<math><msup><mi>sinh</mi><mrow>><mi>n</mi><mo>-</mo><mn>1</mn></mrow></msup><mi>x</mi></math>";
    test("fr", "ClearSpeak", expr, "la puissance n moins 1 de, sinus hyperbolique de x")?;
    return Ok(());

}

#[test]
fn simple_log() -> Result<()> {
    let expr = "<math> <mrow>  <mi>log</mi><mi>x</mi></mrow> </math>";
    test("fr", "ClearSpeak", expr, "logarithme x")?;
    return Ok(());

}

#[test]
fn normal_log() -> Result<()> {
    let expr = "<math><mrow><mi>log</mi><mrow><mo>(</mo><mrow><mi>x</mi><mo>+</mo><mi>y</mi></mrow><mo>)</mo></mrow></mrow></math>";
    test("fr", "ClearSpeak", expr, "le logarithme de, parenthèse gauche, x plus y, parenthèse droite")?;
    return Ok(());

}

#[test]
fn simple_log_with_base() -> Result<()> {
    let expr = "<math> <mrow>  <msub><mi>log</mi><mi>b</mi></msub><mi>x</mi></mrow> </math>";
    test("fr", "ClearSpeak", expr, "le log en base b, de x")?;
    return Ok(());

}

#[test]
fn normal_log_with_base() -> Result<()> {
    let expr = "<math><mrow><msub><mi>log</mi><mi>b</mi></msub><mrow><mo>(</mo><mrow><mi>x</mi><mo>+</mo><mi>y</mi></mrow><mo>)</mo></mrow></mrow></math>";
    test("fr", "ClearSpeak", expr, "le log en base b, de, parenthèse gauche, x plus y, parenthèse droite")?;
    return Ok(());

}

#[test]
fn simple_ln() -> Result<()> {
    let expr = "<math> <mrow>  <mi>ln</mi><mi>x</mi></mrow> </math>";
    test("fr", "ClearSpeak", expr, "l n x")?;
    return Ok(());

}

#[test]
fn normal_ln() -> Result<()> {
    let expr = "<math><mrow><mi>ln</mi><mrow><mo>(</mo><mrow><mi>x</mi><mo>+</mo><mi>y</mi></mrow><mo>)</mo></mrow></mrow></math>";
    test("fr", "ClearSpeak", expr, "le l n de, parenthèse gauche, x plus y, parenthèse droite")?;
    return Ok(());

}

    
#[test]
fn simple_natural_log() -> Result<()> {
    let expr = "<math> <mrow>  <mi>ln</mi><mi>x</mi></mrow> </math>";
    test_ClearSpeak("fr", "ClearSpeak_Log", "LnAsNaturalLog",expr,
        "logarithme naturel x")?;
        return Ok(());

}

    
#[test]
fn natural_log() -> Result<()> {
    let expr = "<math><mi>ln</mi><mo>(</mo><mi>x</mi><mo>+</mo><mi>y</mi><mo>)</mo></math>";
    test_ClearSpeak("fr", "ClearSpeak_Log", "LnAsNaturalLog",expr,
        "le logarithme naturel de, parenthèse gauche, x plus y, parenthèse droite")?;
        return Ok(());

}


#[test]
fn explicit_function_call_with_parens() -> Result<()> {
    let expr = "<math><mrow><mi>t</mi><mo>&#x2061;</mo><mrow><mo>(</mo><mi>x</mi><mo>)</mo></mrow></mrow></math>";
    test("fr", "ClearSpeak", expr, "t de x")?;
    return Ok(());

}


#[test]
fn explicit_times_with_parens() -> Result<()> {
    let expr = "<math><mrow><mi>t</mi><mo>&#x2062;</mo><mrow><mo>(</mo><mi>x</mi><mo>)</mo></mrow></mrow></math>";
    test("fr", "ClearSpeak", expr, "t fois x")?;
    return Ok(());

}

#[test]
fn explicit_function_call() -> Result<()> {
    let expr = "<math><mrow><mi>t</mi><mo>&#x2061;</mo><mrow><mi>x</mi></mrow></mrow></math>";
    test("fr", "ClearSpeak", expr, "t de x")?;
    return Ok(());

}

#[test]
fn explicit_times() -> Result<()> {
    let expr = "<math><mrow><mi>t</mi><mo>&#x2062;</mo><mrow><mi>x</mi></mrow></mrow></math>";
    test("fr", "ClearSpeak", expr, "t x")?;
    return Ok(());

}


#[test]
fn test_functions_none_pref() -> Result<()> {
    let expr = "<math>
    <mi>log</mi><mo>&#x2061;</mo><mrow><mo>(</mo><mrow><mi>x</mi><mo>+</mo><mi>y</mi></mrow><mo>)</mo></mrow>
    <mo>+</mo>
    <mi>f</mi><mo>&#x2061;</mo><mrow><mo>(</mo><mrow><mi>x</mi><mo>+</mo><mi>y</mi></mrow><mo>)</mo></mrow>
    </math>";
    test_ClearSpeak("fr", "ClearSpeak_Functions", "None",expr,
        "le logarithme de, parenthèse gauche, x plus y, parenthèse droite; plus; f fois, parenthèse gauche, x plus y, parenthèse droite")?;
        return Ok(());

}

#[test]
fn test_functions_none_pref_multiple_args() -> Result<()> {
    let expr = "<math>
        <mi>B</mi> <mrow><mo>(</mo> <mrow> <mn>2</mn><mo>,</mo><mn>6</mn></mrow> <mo>)</mo></mrow>
    </math>";
    test_ClearSpeak("fr", "ClearSpeak_Functions", "None",expr,
        "b majuscule fois, parenthèse gauche, 2 virgule, 6, parenthèse droite")?;
        return Ok(());

}


/*
    * Tests for times
    */
#[test]
fn no_times_binomial() -> Result<()> {
    let expr = "<math><mrow><mi>x</mi> <mo>&#x2062;</mo> <mi>y</mi></mrow></math>";
    test("fr", "ClearSpeak", expr, "x y")?;
    return Ok(());

}

#[test]
fn times_following_paren() -> Result<()> {
    let expr = "<math><mrow>
        <mn>2</mn>
        <mrow>  <mo>(</mo> <mn>3</mn>  <mo>)</mo> </mrow>
        </mrow></math>";
    test("fr", "ClearSpeak", expr, "2 fois 3")?;
    return Ok(());

}

#[test]
fn times_preceding_paren() -> Result<()> {
    let expr = "<math><mrow>
        <mrow>  <mo>(</mo> <mn>2</mn>  <mo>)</mo> </mrow>
        <mn>3</mn>
        </mrow></math>";
    test("fr", "ClearSpeak", expr, "2 fois 3")?;
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
    test("fr", "ClearSpeak", expr, "la racine carrée de a; fois la racine carrée de b; est égal à, la racine carrée de a b")?;
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
    test_ClearSpeak("fr", "ClearSpeak_ImpliedTimes", "MoreImpliedTimes",expr,
        "parenthèse gauche, 2 fois x, parenthèse droite au carré")?;
        return Ok(());

}

#[test]
fn explicit_times_more_implied_times() -> Result<()> {
    let expr = "<math><mrow><mi>t</mi><mo>&#x2062;</mo><mrow><mi>x</mi></mrow></mrow></math>";
    test_ClearSpeak("fr", "ClearSpeak_ImpliedTimes", "MoreImpliedTimes",expr, "t fois x")?;
    return Ok(());

}

#[test]
fn explicit_times_none_simple_right() -> Result<()> {
    let expr = "<math><mn>2</mn><mo>[</mo><mn>3</mn> <mo>]</mo></math>";
    test_ClearSpeak("fr", "ClearSpeak_ImpliedTimes", "None",
        expr, "2, crochet ouvrant 3 crochet fermant")?;
        return Ok(());

}

#[test]
fn explicit_times_none_simple_left() -> Result<()> {
    let expr = "<math><mo>(</mo><mn>2</mn><mo>&#x2212;</mo><mn>1</mn><mo>)</mo><mi>x</mi></math>";
    test_ClearSpeak("fr", "ClearSpeak_ImpliedTimes", "None",
        expr, "parenthèse gauche, 2 moins 1, parenthèse droite; x")?;
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
    test_ClearSpeak_prefs("fr", 
        vec![("ClearSpeak_ImpliedTimes", "None"), ("ClearSpeak_Functions", "None")],
        expr, "f, parenthèse gauche x parenthèse droite; est égal à; x au carré, parenthèse gauche, x plus 1, parenthèse droite")?;
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
        test("fr", "ClearSpeak", expr, "25 fois x")?;
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
        test("fr", "ClearSpeak", expr, "b, parenthèse gauche, x y parenthèse droite")?;
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
        test("fr", "ClearSpeak", expr, "2 plus moins 2")?;
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
        test("fr", "ClearSpeak", expr, "moins 2 x plus 1")?;
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
        test("fr", "ClearSpeak", expr, "parenthèse gauche, 2 x parenthèse droite au carré")?;
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
        test("fr", "ClearSpeak", expr, "2 plus 1 demi")?;
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
    test_ClearSpeak("fr", "ClearSpeak_Paren", "Interval",expr,
    "l'intervalle de c à d, n'incluant pas c ou d")?;
    return Ok(());

}

#[test]
    fn parens_interval_closed_open() -> Result<()> {
        let expr = "<math> 
        <mrow><mo>[</mo>
        <mrow> <mi>c</mi><mo>,</mo><mi>d</mi></mrow>
        <mo>)</mo></mrow>
    </math>";
    test_ClearSpeak("fr", "ClearSpeak_Paren", "Interval ",expr,
    "l'intervalle de c à d, incluant c mais n'incluant pas d")?;
    return Ok(());

}


#[test]
fn parens_interval_open_closed() -> Result<()> {
    let expr = "<math> 
    <mrow><mo>(</mo>
        <mrow> <mi>c</mi><mo>,</mo><mi>d</mi></mrow>
    <mo>]</mo></mrow>
    </math>";
    test_ClearSpeak("fr", "ClearSpeak_Paren", "Interval ",expr,
    "l'intervalle de c à d, n'incluant pas c mais incluant d")?;
    return Ok(());

}


#[test]
fn parens_interval_closed_closed() -> Result<()> {
    let expr = "<math> 
    <mrow><mo>[</mo>
    <mrow> <mi>c</mi><mo>,</mo><mi>d</mi></mrow>
    <mo>]</mo></mrow>
</math>";
test_ClearSpeak("fr", "ClearSpeak_Paren", "Interval ",expr,
"l'intervalle de c à d, incluant c et d")?;
return Ok(());

}

    #[test]
    fn parens_interval_neg_infinity_open_open() -> Result<()> {
        let expr = "<math> 
        <mrow><mo>(</mo>
        <mrow><mo>-</mo> <mi>∞</mi><mo>,</mo><mi>d</mi></mrow>
        <mo>)</mo></mrow>
    </math>";
    test_ClearSpeak("fr", "ClearSpeak_Paren", "Interval ",expr,
    "l'intervalle de moins infini à d, n'incluant pas d")?;
    return Ok(());

}

    #[test]
    fn parens_interval_neg_infinity_closed_open() -> Result<()> {
        let expr = "<math> 
        <mrow><mo>(</mo>
        <mrow> <mo>-</mo> <mi>∞</mi><mo>,</mo><mi>d</mi></mrow>
        <mo>]</mo></mrow>
    </math>";
    test_ClearSpeak("fr", "ClearSpeak_Paren", "Interval ",expr,
    "l'intervalle de moins infini à d, incluant d")?;
    return Ok(());

}


#[test]
fn parens_interval_open_open_infinity() -> Result<()> {
    let expr = "<math> 
    <mrow><mo>(</mo>
        <mrow> <mi>c</mi><mo>,</mo><mi>∞</mi></mrow>
    <mo>)</mo></mrow>
    </math>";
    test_ClearSpeak("fr", "ClearSpeak_Paren", "Interval ",expr,
    "l'intervalle de c à infini, n'incluant pas c")?;
    return Ok(());

}


#[test]
fn parens_interval_closed_open_infinity() -> Result<()> {
    let expr = "<math> 
        <mrow><mo>[</mo>
        <mrow> <mi>c</mi><mo>,</mo><mi>∞</mi></mrow>
        <mo>)</mo></mrow>
    </math>";
    test_ClearSpeak("fr", "ClearSpeak_Paren", "Interval ",expr,
"l'intervalle de c à infini, incluant c")?;
return Ok(());

}

#[test]
fn parens_interval_neg_infinity_to_infinity() -> Result<()> {
    let expr = "<math> 
    <mrow><mo>(</mo>
        <mrow><mo>-</mo> <mi>∞</mi><mo>,</mo><mi>∞</mi></mrow>
        <mo>)</mo></mrow>
    </math>";
    test_ClearSpeak("fr", "ClearSpeak_Paren", "Interval ",expr,
    "l'intervalle de moins infini à infini")?;
    return Ok(());

}

#[test]
fn parens_interval_neg_infinity_to_pos_infinity() -> Result<()> {
    let expr = "<math> 
    <mrow><mo>(</mo>
        <mrow><mo>-</mo> <mi>∞</mi><mo>,</mo><mo>+</mo><mi>∞</mi></mrow>
    <mo>)</mo></mrow>
    </math>";
    test_ClearSpeak("fr", "ClearSpeak_Paren", "Interval ",expr,
    "l'intervalle de moins infini à plus infini")?;
    return Ok(());

}
