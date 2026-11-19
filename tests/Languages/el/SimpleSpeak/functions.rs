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
    test("el", "SimpleSpeak", expr, "ημίτονο του x, συν, συνημίτονο του y, συν, εφαπτομένη του z, συν, τέμνουσα του άλφα; συν, συντέμνουσα, του παραλλαγή του φ; συν, συνεφαπτομένη, του φ")?; 
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
    test("el", "SimpleSpeak", expr, "υπερβολικό ημίτονο, του x; συν, \
                                υπερβολικό συνημίτονο, του y; συν, \
                                υπερβολική εφαπτομένη, του z; συν, \
                                υπερβολική τέμνουσα, του άλφα; συν; \
                                υπερβολική συντέμνουσα, του παραλλαγή του φ; συν, \
                                υπερβολική συνεφαπτομένη; του φ")?;
                                return Ok(());
  

}


#[test]
fn inverse_trig() -> Result<()> {
    let expr = "<math><msup><mi>sin</mi><mrow><mo>-</mo><mn>1</mn></mrow></msup><mi>x</mi></math>";
    test("el", "SimpleSpeak", expr, "αντίστροφη ημίτονο, του x")?;
    return Ok(());

}

#[test]
fn trig_squared() -> Result<()> {
    let expr = "<math><msup><mi>sin</mi><mn>2</mn></msup><mi>x</mi></math>";
    test("el", "SimpleSpeak", expr, "ημίτονο στο τετράγωνο, του x")?; // ημίτονο στο τετράγωνο, του x
    return Ok(());

}

#[test]
fn trig_cubed() -> Result<()> {
    let expr = "<math><msup><mi>tan</mi><mn>3</mn></msup><mi>x</mi></math>";
    test("el", "SimpleSpeak", expr, "εφαπτομένη στον κύβο, του x")?;
    return Ok(());

}

#[test]
fn trig_fourth() -> Result<()> {
    let expr = "<math><msup><mi>sec</mi><mn>4</mn></msup><mi>x</mi></math>";
    test("el", "SimpleSpeak", expr, "η τέταρτη δύναμη του, τέμνουσα; του x")?;
    return Ok(());

}


#[test]
fn trig_power_other() -> Result<()> {
    let expr = "<math><msup><mi>sinh</mi><mrow>><mi>n</mi><mo>-</mo><mn>1</mn></mrow></msup><mi>x</mi></math>";
    test("el", "SimpleSpeak", expr, "η n μείον 1 δύναμη του, υπερβολικό ημίτονο; του x")?;
    return Ok(());

}

#[test]
fn simple_log() -> Result<()> {
    let expr = "<math> <mrow>  <mi>log</mi><mi>x</mi></mrow> </math>"; 
    test("el", "SimpleSpeak", expr, "ο λογάριθμος, του x")?;
    return Ok(());

}

#[test]
fn normal_log() -> Result<()> {
    let expr = "<math><mrow><mi>log</mi><mrow><mo>(</mo><mrow><mi>x</mi><mo>+</mo><mi>y</mi></mrow><mo>)</mo></mrow></mrow></math>";
    test("el", "SimpleSpeak", expr, "ο λογάριθμος, του; ανοίγει παρένθεση, x συν y, κλείνει παρένθεση")?;
    return Ok(());

}

#[test]
fn simple_log_with_base() -> Result<()> {
    let expr = "<math> <mrow>  <msub><mi>log</mi><mi>b</mi></msub><mi>x</mi></mrow> </math>";
    test("el", "SimpleSpeak", expr, "ο λογάριθμος με βάση b; του x")?;
    return Ok(());

}
    
#[test]
fn normal_log_with_base() -> Result<()> {
    let expr = "<math><mrow><msub><mi>log</mi><mi>b</mi></msub><mrow><mo>(</mo><mrow><mi>x</mi><mo>+</mo><mi>y</mi></mrow><mo>)</mo></mrow></mrow></math>";
    test("el", "SimpleSpeak", expr, "ο λογάριθμος με βάση b; του; ανοίγει παρένθεση, x συν y, κλείνει παρένθεση")?;
    return Ok(());

}

#[test]
fn normal_ln() -> Result<()> {
    let expr = "<math><mrow><mi>ln</mi><mrow><mo>(</mo><mrow><mi>x</mi><mo>+</mo><mi>y</mi></mrow><mo>)</mo></mrow></mrow></math>";
    test_prefs("el", "SimpleSpeak", vec![("Verbosity", "Terse")],
                expr, "l n, άνοιγμα, x συν y, κλείσιμο")?;
    test_prefs("el", "SimpleSpeak", vec![("Verbosity", "Medium")],
               expr, "ο φυσικός λογάριθμος, του; ανοίγει παρένθεση, x συν y, κλείνει παρένθεση")?;
    test_prefs("el", "SimpleSpeak", vec![("Verbosity", "Verbose")],
                expr, "ο φυσικός λογάριθμος, του; ανοίγει παρένθεση, x συν y, κλείνει παρένθεση")?;
                return Ok(());

}

#[test]
fn simple_ln() -> Result<()> {
    let expr = "<math> <mrow>  <mi>ln</mi><mi>x</mi></mrow> </math>";
    test_prefs("el", "SimpleSpeak", vec![("Verbosity", "Terse")],
                expr, "l n x")?; // l n x
    test_prefs("el", "SimpleSpeak", vec![("Verbosity", "Medium")],
                expr, "ο φυσικός λογάριθμος, του x")?;
    test_prefs("el", "SimpleSpeak", vec![("Verbosity", "Verbose")],
                expr, "ο φυσικός λογάριθμος, του x")?;
                return Ok(());

}

#[test]
fn other_names() -> Result<()> {
    let expr = "<math> <mrow><mi>Cov</mi><mi>x</mi></mrow> </math>";
    test_prefs("el", "SimpleSpeak", vec![("Verbosity", "Terse")],
                expr, "συνδιακύμανση, x")?;
    test_prefs("el", "SimpleSpeak", vec![("Verbosity", "Medium")],
                expr, "συνδιακύμανση, x")?;
    let expr = "<math> <mrow><mi>exp</mi><mo>(</mo><mi>x</mi><mo>)</mo></mrow> </math>";
    test_prefs("el", "SimpleSpeak", vec![("Verbosity", "Terse")],
                expr, "εκθετική x")?;
    test_prefs("el", "SimpleSpeak", vec![("Verbosity", "Medium")],
                expr, "εκθετική του x")?;
                return Ok(());

}

#[test]
fn explicit_function_call_with_parens() -> Result<()> {
    let expr = "<math><mrow><mi>t</mi><mo>&#x2061;</mo><mrow><mo>(</mo><mi>x</mi><mo>)</mo></mrow></mrow></math>";
    test("el", "SimpleSpeak", expr, "t του x")?;
    return Ok(());

}


#[test]
fn explicit_times_with_parens() -> Result<()> {
    let expr = "<math><mrow><mi>t</mi><mo>&#x2062;</mo><mrow><mo>(</mo><mi>x</mi><mo>)</mo></mrow></mrow></math>";
    test("el", "SimpleSpeak", expr, "t επί x")?; 
    return Ok(());
 //theodora. fails because "times" was removed from the SimpleSpeak rule and left blank (keeping it resulted in false positives)


}

#[test]
fn explicit_function_call() -> Result<()> {
    let expr = "<math><mrow><mi>t</mi><mo>&#x2061;</mo><mrow><mi>x</mi></mrow></mrow></math>";
    test("el", "SimpleSpeak", expr, "t του x")?;
    return Ok(());

}

#[test]
fn explicit_times() -> Result<()> {
    let expr = "<math><mrow><mi>t</mi><mo>&#x2062;</mo><mrow><mi>x</mi></mrow></mrow></math>";
    test("el", "SimpleSpeak", expr, "t x")?;
    return Ok(());

}


// Tests for times
 
    
#[test]
fn no_times_binomial() -> Result<()> {
    let expr = "<math><mrow><mi>x</mi> <mo>&#x2062;</mo> <mi>y</mi></mrow></math>";
    test("el", "SimpleSpeak", expr, "x y")?;
    return Ok(());

}

#[test]
fn times_following_paren() -> Result<()> {
    let expr = "<math><mrow>
        <mn>2</mn>
        <mrow>  <mo>(</mo> <mn>3</mn>  <mo>)</mo> </mrow>
        </mrow></math>";
    test("el", "SimpleSpeak", expr, "2 επί 3")?;
    return Ok(());
//theodora. fails because "times" was removed from the SimpleSpeak rule and left blank (keeping it resulted in false positives)

}

#[test]
fn times_preceding_paren() -> Result<()> {
    let expr = "<math><mrow>
        <mrow>  <mo>(</mo> <mn>2</mn>  <mo>)</mo> </mrow>
        <mn>3</mn>
        </mrow></math>";
    test("el", "SimpleSpeak", expr, "2 επί 3")?;
    return Ok(());
 //theodora. fails because "times" was removed from the SimpleSpeak rule and left blank (keeping it resulted in false positives)

}

#[test]
fn no_times_sqrt() -> Result<()> {
    let expr = "<math><mrow>
        <msqrt> <mi>a</mi>  </msqrt>
        <msqrt> <mi>b</mi>  </msqrt>
        <mo>=</mo>
        <msqrt> <mrow>  <mi>a</mi><mi>b</mi></mrow> </msqrt>
        </mrow></math>";
    test("el", "SimpleSpeak", expr, 
            "η τετραγωνική ρίζα του a; επί, η τετραγωνική ρίζα του b; ισούται με; η τετραγωνική ρίζα του a b, τέλος ρίζας")?;
    test_prefs("el", "SimpleSpeak", vec![("Impairment", "LearningDisability")], expr,
            "η τετραγωνική ρίζα του a; επί, η τετραγωνική ρίζα του b; ισούται με; η τετραγωνική ρίζα του a b")?; 
            return Ok(());
 //theodora. fails because "times" was removed from the SimpleSpeak rule and left blank (keeping it resulted in false positives)
}

// Tests for parens
    
    
    #[test]
    fn no_parens_number() -> Result<()> {
        let expr = "<math><mrow>
        <mrow><mo>(</mo>
        <mn>25</mn>
        <mo>)</mo></mrow>
        <mi>x</mi>
        </mrow></math>";
        test("el", "SimpleSpeak", expr, "25 επί x")?; 
        return Ok(());
          //theodora. fails because "times" was removed from the SimpleSpeak rule and left blank (keeping it resulted in false positives)


    }

    #[test]
    fn no_parens_monomial() -> Result<()> {
        let expr = "<math><mrow>
        <mi>b</mi>
        <mrow><mo>(</mo>
        <mrow><mi>x</mi><mi>y</mi></mrow>
        <mo>)</mo></mrow>
        </mrow></math>";
        test("el", "SimpleSpeak", expr, "b; ανοίγει παρένθεση, x y, κλείνει παρένθεση")?;
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
        test("el", "SimpleSpeak", expr, "2 συν μείον 2")?;
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
        test("el", "SimpleSpeak", expr, "μείον 2 x, συν 1")?;
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
        test("el", "SimpleSpeak", expr, "ανοίγει παρένθεση, 2 x, κλείνει παρένθεση στο τετράγωνο")?;
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
        test("el", "SimpleSpeak", expr, "2 συν 1 δεύτερο")?;
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
    test("el", "SimpleSpeak",expr, "το ανοιχτό διάστημα από c ως d")?;
    return Ok(());

}

#[test]
    fn parens_interval_closed_open() -> Result<()> {
        let expr = "<math> 
        <mrow intent='closed-open-interval($start, $end)'><mo>[</mo>
            <mrow> <mo arg='open'>[(]</mo><mi arg='start'>c</mi><mo>,</mo><mi arg='end'>d</mi></mrow><mo arg='close'>)</mo>
            <mo>)</mo></mrow>
        </math>";
    test("el", "SimpleSpeak",expr, "το κλειστό ανοιχτό διάστημα από c ως d")?;
    return Ok(());

}

#[test]
fn parens_interval_open_closed() -> Result<()> {
    let expr = "<math> 
    <mrow intent='open-closed-interval($start, $end)'><mo>(</mo>
        <mrow> <mo arg='open'>(</mo><mi arg='start'>c</mi><mo>,</mo><mi arg='end'>d</mi></mrow><mo arg='close'>]</mo>
        <mo>]</mo></mrow>
    </math>";
    test("el", "SimpleSpeak",expr,"το ανοιχτό κλειστό διάστημα από c ως d")?;
    return Ok(());

}

#[test]
fn parens_interval_closed_closed() -> Result<()> {
    let expr = "<math> 
        <mrow intent='closed-interval($start, $end)'><mo>[</mo>
            <mrow> <mo arg='open'>[(]</mo><mi arg='start'>c</mi><mo>,</mo><mi arg='end'>d</mi></mrow><mo arg='close'>]</mo>
            <mo>]</mo></mrow>
    </math>";
    test("el", "SimpleSpeak",expr, "το κλειστό διάστημα από c ως d")?;
    return Ok(());

} 

 #[test]
    fn parens_interval_neg_infinity_open_open() -> Result<()> {
        let expr = "<math> 
        <mrow intent='open-interval($start, $end)'><mo arg='open'>(</mo>
        <mrow><mrow arg='start'><mo>-</mo> <mi>∞</mi></mrow><mo>,</mo><mi arg='end'>d</mi></mrow><mo arg='close'>)</mo>
        <mo>)</mo></mrow>
    </math>";
    test("el", "SimpleSpeak",expr,
    "το ανοιχτό διάστημα από μείον άπειρο ως d")?;
    return Ok(());

}

#[test]
    fn parens_interval_neg_infinity_open_closed() -> Result<()> {
        let expr = "<math> 
        <mrow intent='open-closed-interval($start, $end)'><mo arg='open'>(</mo>
        <mrow><mrow arg='start'><mo>-</mo> <mi>∞</mi></mrow><mo>,</mo><mi arg='end'>d</mi></mrow><mo arg='close'>]</mo>
        <mo>]</mo></mrow>
    </math>";
    test("el", "SimpleSpeak",expr,
    "το ανοιχτό κλειστό διάστημα από μείον άπειρο ως d")?;
    return Ok(());

}

