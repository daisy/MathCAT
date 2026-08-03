use crate::common::*;
use anyhow::Result;

// AI generated
#[test]
fn multiplication() -> Result<()> {
    let expr = "<math>
                    <mn>2</mn><mo>×</mo><mn>3</mn>
                </math>";
    test("hu", "ClearSpeak", expr, "2 szorozva 3")?;
    return Ok(());

}

// AI generated
#[test]
fn multiplication_by() -> Result<()> {
    let expr = "<math>
                    <mn>2</mn><mo>×</mo><mn>3</mn>
                </math>";
    test_ClearSpeak("hu", "ClearSpeak_MultSymbolX", "By", expr, "2 szorozva 3")?;
    return Ok(());

}

// AI generated
#[test]
fn multiplication_cross() -> Result<()> {
    let expr = "<math>
                    <mi>u</mi><mo>×</mo><mi>v</mi>
                </math>";
    test_ClearSpeak("hu", "ClearSpeak_MultSymbolX", "Cross", expr, "u kereszt v")?;
    return Ok(());

}

// AI generated
#[test]
fn ellipses_auto_start() -> Result<()> {
    let expr = "<math>
            <mi>…</mi><mo>,</mo>
            <mo>-</mo><mn>2</mn><mo>,</mo><mo>-</mo><mn>1</mn><mo>,</mo><mn>0</mn>
        </math>";
    test("hu", "ClearSpeak", expr, "pont pont pont vessző, negatív 2 vessző, negatív 1 vessző, 0")?;
    return Ok(());

}

// AI generated
#[test]
fn ellipses_auto_end() -> Result<()> {
    let expr = "<math>
            <mn>1</mn>
            <mo>,</mo>
            <mn>2</mn>
            <mo>,</mo>
            <mn>3</mn>
            <mo>,</mo>
            <mi>…</mi>
        </math>";
    test_ClearSpeak("hu", "ClearSpeak_Ellipses", "Auto", expr, "1 vessző, 2 vessző, 3 vessző, pont pont pont")?;
    return Ok(());

}

// AI generated
#[test]
fn ellipses_auto_middle() -> Result<()> {
    let expr = "<math>
            <mrow>
                <mn>1</mn>
                <mo>,</mo>
                <mn>2</mn>
                <mo>,</mo>
                <mn>3</mn>
                <mo>,</mo>
                <mi>…</mi>
                <mo>,</mo>
                <mn>20</mn>
            </mrow>
        </math>";
    test_ClearSpeak("hu", "ClearSpeak_Ellipses", "Auto", expr,
            "1 vessző, 2 vessző, 3 vessző, pont pont pont vessző, 20")?;
            return Ok(());

}

// AI generated
#[test]
fn ellipses_auto_both() -> Result<()> {
    let expr = "<math>
            <mi>…</mi><mo>,</mo>
            <mo>-</mo><mn>2</mn><mo>,</mo><mo>-</mo><mn>1</mn><mo>,</mo><mn>0</mn><mo>,</mo><mn>1</mn><mo>,</mo><mn>2</mn>
            <mo>,</mo><mi>…</mi>
       </math>";
    test_ClearSpeak("hu", "ClearSpeak_Ellipses", "Auto", expr,
            "pont pont pont vessző, negatív 2 vessző, negatív 1 vessző, 0 vessző, 1 vessző, 2 vessző, pont pont pont")?;
            return Ok(());

}

// AI generated
#[test]
fn ellipses_and_so_on_start() -> Result<()> {
    let expr = "<math>
            <mi>…</mi><mo>,</mo>
            <mo>-</mo><mn>2</mn><mo>,</mo><mo>-</mo><mn>1</mn><mo>,</mo><mn>0</mn>
        </math>";
        test_ClearSpeak("hu", "ClearSpeak_Ellipses", "AndSoOn", expr, "pont pont pont vessző, negatív 2 vessző, negatív 1 vessző, 0")?;
        return Ok(());

}

// AI generated
#[test]
fn ellipses_and_so_on_end() -> Result<()> {
    let expr = "<math>
            <mn>1</mn>
            <mo>,</mo>
            <mn>2</mn>
            <mo>,</mo>
            <mn>3</mn>
            <mo>,</mo>
            <mi>…</mi>
        </math>";
    test_ClearSpeak("hu", "ClearSpeak_Ellipses", "AndSoOn", expr, "1 vessző, 2 vessző, 3 és így tovább")?;
    return Ok(());

}

// AI generated
#[test]
fn ellipses_and_so_on_middle() -> Result<()> {
    let expr = "<math>
            <mrow>
                <mn>1</mn>
                <mo>,</mo>
                <mn>2</mn>
                <mo>,</mo>
                <mn>3</mn>
                <mo>,</mo>
                <mi>…</mi>
                <mo>,</mo>
                <mn>20</mn>
            </mrow>
        </math>";
    test_ClearSpeak("hu", "ClearSpeak_Ellipses", "AndSoOn", expr,
            "1 vessző, 2 vessző, 3, és így tovább egészen az alábbi értékig:; 20")?;
            return Ok(());

}

// AI generated
#[test]
fn ellipses_and_so_on_both() -> Result<()> {
    let expr = "<math>
            <mi>…</mi><mo>,</mo>
            <mo>-</mo><mn>2</mn><mo>,</mo><mo>-</mo><mn>1</mn><mo>,</mo><mn>0</mn><mo>,</mo><mn>1</mn><mo>,</mo><mn>2</mn>
            <mo>,</mo><mi>…</mi>
       </math>";
    test_ClearSpeak("hu", "ClearSpeak_Ellipses", "AndSoOn", expr,
            "pont pont pont vessző, negatív 2 vessző, negatív 1 vessző, 0 vessző, 1 vessző, 2 vessző, pont pont pont")?;
            return Ok(());

}

// AI generated
#[test]
fn vertical_line_auto() -> Result<()> {
    let expr = "<math>
        <mn>3</mn><mo>|</mo><mn>6</mn>
    </math>";
    test_ClearSpeak("hu", "ClearSpeak_VerticalLine", "Auto", expr,
            "3 osztja 6")?;
            return Ok(());

}

// AI generated
#[test]
fn vertical_line_divides() -> Result<()> {
    let expr = "<math>
        <mn>3</mn><mo>|</mo><mn>6</mn>
    </math>";
    test_ClearSpeak("hu", "ClearSpeak_VerticalLine", "Divides", expr,
            "3 osztja 6")?;
            return Ok(());

}

    #[test]
    fn vertical_line_given() -> Result<()> {
        let expr = "<math>
            <mn>3</mn><mo>|</mo><mn>6</mn>
        </math>";
        test_ClearSpeak("hu", "ClearSpeak_VerticalLine", "Given", expr,
                "3 adott 6")?;
                return Ok(());

    }

    #[test]
    fn vertical_line_probability_given() -> Result<()> {
        let expr = "<math>
                <mi>P</mi>
                <mrow>
                    <mo>(</mo>
                    <mrow>
                        <mi>A</mi>
                        <mo>|</mo>
                        <mi>B</mi>
                    </mrow>
                    <mo>)</mo>
                </mrow>
            </math>";
        test_ClearSpeak_prefs("hu", vec![("ClearSpeak_VerticalLine", "Given"), ("ClearSpeak_ImpliedTimes", "None")]
                        , expr, "nagy p; nyitott zárójel, nagy a adott nagy b, zárt zárójel")?;
                        return Ok(());
    }

// AI generated
#[test]
fn vertical_line_set() -> Result<()> {
    let expr = "<math>
        <mo>{</mo>
        <mrow>
            <mi>x</mi>
            <mo>|</mo>
            <mi>x</mi>
            <mo>&gt;</mo>
            <mn>0</mn>
        </mrow>
        <mo>}</mo>    
    </math>";
    test_ClearSpeak("hu", "ClearSpeak_VerticalLine", "Auto", expr,
            "x melyekre teljesül, hogy minden x nagyobb, mint 0")?;
            return Ok(());

}


// AI generated
#[test]
fn vertical_line_set_such_that() -> Result<()> {
    let expr = "<math>
        <mo>{</mo>
        <mrow>
            <mi>x</mi>
            <mo>|</mo>
            <mi>x</mi>
            <mo>&gt;</mo>
            <mn>0</mn>
        </mrow>
        <mo>}</mo>    
    </math>";
    test_ClearSpeak("hu", "ClearSpeak_VerticalLine", "SuchThat", expr,
            "x melyekre teljesül, hogy minden x nagyobb, mint 0")?;
            return Ok(());

}

// AI generated
#[test]
fn vertical_line_set_given() -> Result<()> {
    let expr = "<math>
        <mo>{</mo>
        <mrow>
            <mi>x</mi>
            <mo>|</mo>
            <mi>x</mi>
            <mo>&gt;</mo>
            <mn>0</mn>
        </mrow>
        <mo>}</mo>    
    </math>";
    // the rules for set will override all the options -- ClearSpeak spec should be clarified
    test_ClearSpeak("hu", "ClearSpeak_VerticalLine", "Given", expr,
            "x melyekre teljesül, hogy minden x nagyobb, mint 0")?;
            return Ok(());

}

// AI generated
#[test]
fn vertical_line_set_and_abs() -> Result<()> {
    let expr = "<math>
            <mo>{</mo>
            <mrow>
                <mi>x</mi>
                <mo>&#x007C;</mo>
                <mrow>
                    <mo>|</mo>
                    <mi>x</mi>
                    <mo>|</mo>
                </mrow>
                <mo>&gt;</mo>
                <mn>2</mn>
            </mrow>
            <mo>}</mo>
        </math>";
    test_ClearSpeak("hu", "ClearSpeak_VerticalLine", "Auto", expr,
        "x melyekre teljesül, hogy minden x abszolút értéke; nagyobb, mint 2")?;
        return Ok(());

}

// AI generated
#[test]
fn vertical_line_evaluated_at() -> Result<()> {
    let expr = "<math>
            <mi>f</mi>
            <mrow>
                <mo>(</mo>
                <mi>x</mi>
                <mo>)</mo>
            </mrow>
            <msub>
                <mo>&#x007C;</mo>
                <mrow>
                    <mi>x</mi>
                    <mo>=</mo>
                    <mn>5</mn>
                </mrow>
            </msub>
        </math>";
    test_ClearSpeak("hu", "ClearSpeak_VerticalLine", "Auto", expr,
        "f x helyettesítve, x egyenlő 5")?;
        return Ok(());

}

// AI generated
#[test]
fn vertical_line_evaluated_at_both() -> Result<()> {
    let expr = "<math>
            <msup>
            <mi>x</mi>
            <mn>2</mn>
            </msup>
            <mo>+</mo>
            <mi>x</mi>
            <msubsup>
                <mstyle mathsize='140%' displaystyle='true'> <mo>&#x007C;</mo> </mstyle>
                <mn>0</mn>
                <mn>1</mn>
            </msubsup>
        </math>";
    test_ClearSpeak("hu", "ClearSpeak_VerticalLine", "Auto", expr,
        "x a négyzeten plusz x, behelyettesítve a(z) 1 értékét, levonva belőle a korábbi kifejezéssel azonos eredményt, behelyettesítve a(z) 0 értéket")?;
        return Ok(());

}
// AI generated
#[test]
fn vertical_line_evaluated_at_divides() -> Result<()> {
    let expr = "<math>
            <mi>f</mi>
            <mrow>
                <mo>(</mo>
                <mi>x</mi>
                <mo>)</mo>
            </mrow>
            <msub>
                <mo>&#x007C;</mo>
                <mrow>
                    <mi>x</mi>
                    <mo>=</mo>
                    <mn>5</mn>
                </mrow>
            </msub>
        </math>";
    test_ClearSpeak("hu", "ClearSpeak_VerticalLine", "Divides", expr,
        "f x helyettesítve, x egyenlő 5")?;
        return Ok(());

}

// AI generated
#[test]
fn vertical_line_evaluated_at_both_given() -> Result<()> {
    let expr = "<math>
            <msup>
            <mi>x</mi>
            <mn>2</mn>
            </msup>
            <mo>+</mo>
            <mi>x</mi>
            <msubsup>
                <mstyle mathsize='140%' displaystyle='true'> <mo>&#x007C;</mo> </mstyle>
                <mn>0</mn>
                <mn>1</mn>
            </msubsup>
        </math>";
    test_ClearSpeak("hu", "ClearSpeak_VerticalLine", "Given", expr,
        "x a négyzeten plusz x, behelyettesítve a(z) 1 értékét, levonva belőle a korábbi kifejezéssel azonos eredményt, behelyettesítve a(z) 0 értéket")?;
        return Ok(());

}
