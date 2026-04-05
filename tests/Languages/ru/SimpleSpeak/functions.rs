/// Tests for:
/// *  functions including trig functions, logs, and functions to powers
/// *  implied times/functional call and explicit times/function call
/// *  parens
/// These are all intertwined, so they are in one file
use crate::common::*;

#[test]
fn trig_names() {
    let expr = "<math><mrow>
    <mi>sin</mi><mi>x</mi><mo>+</mo>
    <mi>cos</mi><mi>y</mi><mo>+</mo>
    <mi>tan</mi><mi>z</mi><mo>+</mo>
    <mi>sec</mi><mi>&#x03B1;</mi><mo>+</mo>
    <mi>csc</mi><mi>&#x03D5;</mi><mo>+</mo>
    <mi>cot</mi><mi>&#x03C6;</mi>
    </mrow></math>";
    test("ru", "SimpleSpeak", expr, "синус икс плюс косинус игрек плюс тангенс зет плюс секанс альфа, плюс косеканс фи, плюс котангенс фи");
}

#[test]
fn hyperbolic_trig_names() {
    let expr = "<math><mrow>
    <mi>sinh</mi><mi>x</mi><mo>+</mo>
    <mi>cosh</mi><mi>y</mi><mo>+</mo>
    <mi>tanh</mi><mi>z</mi><mo>+</mo>
    <mi>sech</mi><mi>&#x03B1;</mi><mo>+</mo>
    <mi>csch</mi><mi>&#x03D5;</mi><mo>+</mo>
    <mi>coth</mi><mi>&#x03C6;</mi>
    </mrow></math>";
    test("ru", "SimpleSpeak", expr, "гиперболический синус икс, плюс \
                                гиперболический косинус игрек, плюс \
                                гиперболический тангенс зет, плюс \
                                гиперболический секанс альфа, плюс \
                                гиперболический косеканс фи, плюс \
                                гиперболический котангенс фи");
}

#[test]
fn inverse_trig() {
    let expr = "<math><msup><mi>sin</mi><mrow><mo>-</mo><mn>1</mn></mrow></msup><mi>x</mi></math>";
    test("ru", "SimpleSpeak", expr, "арксинус икс");
}

#[test]
fn trig_squared() {
    let expr = "<math><msup><mi>sin</mi><mn>2</mn></msup><mi>x</mi></math>";
    test("ru", "SimpleSpeak", expr, "синус в квадрате икс");
}

#[test]
fn trig_cubed() {
    let expr = "<math><msup><mi>tan</mi><mn>3</mn></msup><mi>x</mi></math>";
    test("ru", "SimpleSpeak", expr, "тангенс в кубе икс");
}

#[test]
fn trig_fourth() {
    let expr = "<math><msup><mi>sec</mi><mn>4</mn></msup><mi>x</mi></math>";
    test("ru", "SimpleSpeak", expr, "секанс в четвертой степени икс");
}

#[test]
fn trig_power_other() {
    let expr = "<math><msup><mi>sinh</mi><mrow>><mi>n</mi><mo>-</mo><mn>1</mn></mrow></msup><mi>x</mi></math>";
    test("ru", "SimpleSpeak", expr, "гиперболический синус икс в степени n минус 1");
}

#[test]
fn simple_log() {
    let expr = "<math> <mrow>  <mi>log</mi><mi>x</mi></mrow> </math>";
    test("ru", "SimpleSpeak", expr, "логарифм икс");
}

#[test]
fn normal_log() {
    let expr = "<math><mrow><mi>log</mi><mrow><mo>(</mo><mrow><mi>x</mi><mo>+</mo><mi>y</mi></mrow><mo>)</mo></mrow></mrow></math>";
    test("ru", "SimpleSpeak", expr, "логарифм от, скобка открывается икс плюс игрек, скобка закрывается");
}

#[test]
fn simple_log_with_base() {
    let expr = "<math> <mrow>  <msub><mi>log</mi><mi>b</mi></msub><mi>x</mi></mrow> </math>";
    test("ru", "SimpleSpeak", expr, "логарифм по основанию бэ от икс");
}

#[test]
fn normal_log_with_base() {
    let expr = "<math><mrow><msub><mi>log</mi><mi>b</mi></msub><mrow><mo>(</mo><mrow><mi>x</mi><mo>+</mo><mi>y</mi></mrow><mo>)</mo></mrow></mrow></math>";
    test("ru", "SimpleSpeak", expr, "логарифм по основанию бэ от, скобка открывается икс плюс игрек, скобка закрывается");
}

#[test]
fn normal_ln() {
    let expr = "<math><mrow><mi>ln</mi><mrow><mo>(</mo><mrow><mi>x</mi><mo>+</mo><mi>y</mi></mrow><mo>)</mo></mrow></mrow></math>";
    test_prefs("ru", "SimpleSpeak", vec![("Verbosity", "Terse")],
                expr, "эл эн, открывается икс плюс игрек закрывается");
    test_prefs("ru", "SimpleSpeak", vec![("Verbosity", "Medium")],
                expr, "натуральный логарифм от, скобка открывается икс плюс игрек, скобка закрывается");
    test_prefs("ru", "SimpleSpeak", vec![("Verbosity", "Verbose")],
                expr, "натуральный логарифм от, скобка открывается икс плюс игрек, скобка закрывается");
}

#[test]
fn simple_ln() {
    let expr = "<math> <mrow>  <mi>ln</mi><mi>x</mi></mrow> </math>";
    test_prefs("ru", "SimpleSpeak", vec![("Verbosity", "Terse")],
                expr, "эл эн икс");
    test_prefs("ru", "SimpleSpeak", vec![("Verbosity", "Medium")],
                expr, "натуральный логарифм икс");
    test_prefs("ru", "SimpleSpeak", vec![("Verbosity", "Verbose")],
                expr, "натуральный логарифм икс");
}

#[test]
fn other_names() {
    let expr = "<math> <mrow><mi>Cov</mi><mi>x</mi></mrow> </math>";
    test_prefs("ru", "SimpleSpeak", vec![("Verbosity", "Terse")],
                expr, "Cov икс");
    test_prefs("ru", "SimpleSpeak", vec![("Verbosity", "Medium")],
                expr, "ковариация икс");
    let expr = "<math> <mrow><mi>exp</mi><mo>(</mo><mi>x</mi><mo>)</mo></mrow> </math>";
    test_prefs("ru", "SimpleSpeak", vec![("Verbosity", "Terse")],
                expr, "эксп от икс");
    test_prefs("ru", "SimpleSpeak", vec![("Verbosity", "Medium")],
                expr, "экспонента от икс");
}

#[test]
fn explicit_function_call_with_parens() {
    let expr = "<math><mrow><mi>t</mi><mo>&#x2061;</mo><mrow><mo>(</mo><mi>x</mi><mo>)</mo></mrow></mrow></math>";
    test("ru", "SimpleSpeak", expr, "тэ от икс");
}

#[test]
fn explicit_times_with_parens() {
    let expr = "<math><mrow><mi>t</mi><mo>&#x2062;</mo><mrow><mo>(</mo><mi>x</mi><mo>)</mo></mrow></mrow></math>";
    test("ru", "SimpleSpeak", expr, "тэ умножить на икс");
}

#[test]
fn explicit_function_call() {
    let expr = "<math><mrow><mi>t</mi><mo>&#x2061;</mo><mrow><mi>x</mi></mrow></mrow></math>";
    test("ru", "SimpleSpeak", expr, "тэ от икс");
}

#[test]
fn explicit_times() {
    let expr = "<math><mrow><mi>t</mi><mo>&#x2062;</mo><mrow><mi>x</mi></mrow></mrow></math>";
    test("ru", "SimpleSpeak", expr, "тэ икс");
}

/*
    * Tests for times
    */
#[test]
fn no_times_binomial() {
    let expr = "<math><mrow><mi>x</mi> <mo>&#x2062;</mo> <mi>y</mi></mrow></math>";
    test("ru", "SimpleSpeak", expr, "икс игрек");
}

#[test]
fn times_following_paren() {
    let expr = "<math><mrow>
        <mn>2</mn>
        <mrow>  <mo>(</mo> <mn>3</mn>  <mo>)</mo> </mrow>
        </mrow></math>";
    test("ru", "SimpleSpeak", expr, "2 умножить на 3");
}

#[test]
fn times_preceding_paren() {
    let expr = "<math><mrow>
        <mrow>  <mo>(</mo> <mn>2</mn>  <mo>)</mo> </mrow>
        <mn>3</mn>
        </mrow></math>";
    test("ru", "SimpleSpeak", expr, "2 умножить на 3");
}

#[test]
fn no_times_sqrt() {
    let expr = "<math><mrow>
        <msqrt> <mi>a</mi>  </msqrt>
        <msqrt> <mi>b</mi>  </msqrt>
        <mo>=</mo>
        <msqrt> <mrow>  <mi>a</mi><mi>b</mi></mrow> </msqrt>
        </mrow></math>";
    test("ru", "SimpleSpeak", expr, 
            "квадратный корень из а; умножить на квадратный корень из бэ; равно, квадратный корень из а бэ конец корня");
    test_prefs("ru", "SimpleSpeak", vec![("Impairment", "LearningDisability")], expr,
            "квадратный корень из а; умножить на квадратный корень из бэ; равно, квадратный корень из а бэ");
}

/*
    * Tests for parens
    */
#[test]
fn no_parens_number() {
    let expr = "<math><mrow>
        <mrow><mo>(</mo>
        <mn>25</mn>
        <mo>)</mo></mrow>
        <mi>x</mi>
        </mrow></math>";
    test("ru", "SimpleSpeak", expr, "25 умножить на икс");
}

#[test]
fn no_parens_monomial() {
    let expr = "<math><mrow>
        <mi>b</mi>
        <mrow><mo>(</mo>
        <mrow><mi>x</mi><mi>y</mi></mrow>
        <mo>)</mo></mrow>
        </mrow></math>";
    test("ru", "SimpleSpeak", expr, "бэ, скобка открывается икс игрек скобка закрывается");
}

#[test]
fn no_parens_negative_number() {
    let expr = "<math><mrow>
        <mn>2</mn><mo>+</mo>
        <mrow><mo>(</mo>
        <mrow><mo>&#x2212;</mo><mn>2</mn></mrow>
        <mo>)</mo></mrow>
        </mrow></math>";
    test("ru", "SimpleSpeak", expr, "2 плюс минус 2");
}

#[test]
fn no_parens_negative_number_with_var() {
    let expr = "<math><mrow>
        <mrow><mo>(</mo>
        <mrow><mo>&#x2212;</mo><mn>2</mn></mrow><mi>x</mi>
        <mo>)</mo></mrow>
        <mo>+</mo><mn>1</mn>
        </mrow></math>";
    test("ru", "SimpleSpeak", expr, "минус 2 икс, плюс 1");
}

#[test]
fn parens_superscript() {
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
    test("ru", "SimpleSpeak", expr, "скобка открывается 2 икс скобка закрывается в квадрате");
}

#[test]
fn no_parens_fraction() {
    let expr = "<math><mrow>
        <mn>2</mn>
        <mo>+</mo>
        <mrow>
            <mrow><mo>(</mo>
            <mfrac> <mn>1</mn><mn>2</mn></mfrac>
            <mo>)</mo></mrow></mrow>
    </mrow></math>";
    test("ru", "SimpleSpeak", expr, "2 плюс одна вторая");
}

// Tests for the four types of intervals in SimpleSpeak
#[test]
fn parens_interval_open_open() {
    let expr = "<math> 
        <mrow intent='open-interval($start, $end)'><mo>(</mo>
        <mrow> <mo arg='open'>(</mo><mi arg='start'>c</mi><mo>,</mo><mi arg='end'>d</mi></mrow><mo arg='close'>)</mo>
        <mo>)</mo></mrow>
    </math>";
    test("ru", "SimpleSpeak", expr, "открытый интервал от цэ до дэ");
}

#[test]
fn parens_interval_closed_open() {
    let expr = "<math> 
        <mrow intent='closed-open-interval($start, $end)'><mo>[</mo>
            <mrow> <mo arg='open'>[(]</mo><mi arg='start'>c</mi><mo>,</mo><mi arg='end'>d</mi></mrow><mo arg='close'>)</mo>
            <mo>)</mo></mrow>
        </math>";
    test("ru", "SimpleSpeak", expr, "закрыто-открытый интервал от цэ до дэ");
}

#[test]
fn parens_interval_open_closed() {
    let expr = "<math> 
    <mrow intent='open-closed-interval($start, $end)'><mo>(</mo>
        <mrow> <mo arg='open'>(</mo><mi arg='start'>c</mi><mo>,</mo><mi arg='end'>d</mi></mrow><mo arg='close'>]</mo>
        <mo>]</mo></mrow>
    </math>";
    test("ru", "SimpleSpeak", expr, "открыто-закрытый интервал от цэ до дэ");
}

#[test]
fn parens_interval_closed_closed() {
    let expr = "<math> 
        <mrow intent='closed-interval($start, $end)'><mo>[</mo>
            <mrow> <mo arg='open'>[(]</mo><mi arg='start'>c</mi><mo>,</mo><mi arg='end'>d</mi></mrow><mo arg='close'>]</mo>
            <mo>]</mo></mrow>
    </math>";
    test("ru", "SimpleSpeak", expr, "закрытый интервал от цэ до дэ");
}

#[test]
fn parens_interval_neg_infinity_open_open() {
    let expr = "<math> 
        <mrow intent='open-interval($start, $end)'><mo arg='open'>(</mo>
        <mrow><mrow arg='start'><mo>-</mo> <mi>∞</mi></mrow><mo>,</mo><mi arg='end'>d</mi></mrow><mo arg='close'>)</mo>
        <mo>)</mo></mrow>
    </math>";
    test("ru", "SimpleSpeak", expr,
        "открытый интервал от минус бесконечности до дэ");
}

#[test]
fn parens_interval_neg_infinity_open_closed() {
    let expr = "<math> 
        <mrow intent='open-closed-interval($start, $end)'><mo arg='open'>(</mo>
        <mrow><mrow arg='start'><mo>-</mo> <mi>∞</mi></mrow><mo>,</mo><mi arg='end'>d</mi></mrow><mo arg='close'>]</mo>
        <mo>]</mo></mrow>
    </math>";
    test("ru", "SimpleSpeak", expr,
        "открыто-закрытый интервал от минус бесконечности до дэ");
}

