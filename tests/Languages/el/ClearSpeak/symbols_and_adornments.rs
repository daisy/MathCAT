use crate::common::*;
use anyhow::Result;

#[test]
fn multiplication() -> Result<()> {
    let expr = "<math>
                    <mn>2</mn><mo>×</mo><mn>3</mn>
                </math>";
    test("el", "ClearSpeak", expr, "2 επί 3")?;
    return Ok(());

}

#[test]
fn multiplication_by() -> Result<()> {
    let expr = "<math>
                    <mn>2</mn><mo>×</mo><mn>3</mn>
                </math>";
    test_ClearSpeak("el", "ClearSpeak_MultSymbolX", "By", expr, "2 επί 3")?;
    return Ok(());

}

#[test]
fn multiplication_cross() -> Result<()> {
    let expr = "<math>
                    <mi>u</mi><mo>×</mo><mi>v</mi>
                </math>";
    test_ClearSpeak("el", "ClearSpeak_MultSymbolX", "Cross", expr, "u πολλαπλασιασμός v")?;
    return Ok(());

}

#[test]
fn ellipses_auto_start() -> Result<()> {
    let expr = "<math>
            <mi>…</mi><mo>,</mo>
            <mo>-</mo><mn>2</mn><mo>,</mo><mo>-</mo><mn>1</mn><mo>,</mo><mn>0</mn>
        </math>";
    test("el", "ClearSpeak", expr, "αποσιωπητικά, κόμμα, μείον 2 κόμμα, μείον 1 κόμμα, 0")?;
    return Ok(());

}

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
    test_ClearSpeak("el", "ClearSpeak_Ellipses", "Auto", expr, "1 κόμμα, 2 κόμμα, 3 κόμμα; αποσιωπητικά")?;
    return Ok(());

}

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
    test_ClearSpeak("el", "ClearSpeak_Ellipses", "Auto", expr,
            "1 κόμμα, 2 κόμμα, 3 κόμμα; αποσιωπητικά, κόμμα, 20")?;
            return Ok(());

}

#[test]
fn ellipses_auto_both() -> Result<()> {
    let expr = "<math>
            <mi>…</mi><mo>,</mo>
            <mo>-</mo><mn>2</mn><mo>,</mo><mo>-</mo><mn>1</mn><mo>,</mo><mn>0</mn><mo>,</mo><mn>1</mn><mo>,</mo><mn>2</mn>
            <mo>,</mo><mi>…</mi>
       </math>";
    test_ClearSpeak("el", "ClearSpeak_Ellipses", "Auto", expr,
            "αποσιωπητικά, κόμμα, μείον 2 κόμμα, μείον 1 κόμμα, 0 κόμμα, 1 κόμμα, 2 κόμμα; αποσιωπητικά")?;
            return Ok(());

}

#[test]
fn ellipses_and_so_on_start() -> Result<()> {
    let expr = "<math>
            <mi>…</mi><mo>,</mo>
            <mo>-</mo><mn>2</mn><mo>,</mo><mo>-</mo><mn>1</mn><mo>,</mo><mn>0</mn>
        </math>";
        test_ClearSpeak("el", "ClearSpeak_Ellipses", "AndSoOn", expr, "αποσιωπητικά, κόμμα, μείον 2 κόμμα, μείον 1 κόμμα, 0")?;
        return Ok(());

}

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
    test_ClearSpeak("el", "ClearSpeak_Ellipses", "AndSoOn", expr, "1 κόμμα, 2 κόμμα, 3 και ούτω καθεξής")?;
    return Ok(());

}

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
    test_ClearSpeak("el", "ClearSpeak_Ellipses", "AndSoOn", expr,
            "1 κόμμα, 2 κόμμα, 3 και ούτω καθεξής ως, 20")?;
            return Ok(());

}

#[test]
fn ellipses_and_so_on_both() -> Result<()> {
    let expr = "<math>
            <mi>…</mi><mo>,</mo>
            <mo>-</mo><mn>2</mn><mo>,</mo><mo>-</mo><mn>1</mn><mo>,</mo><mn>0</mn><mo>,</mo><mn>1</mn><mo>,</mo><mn>2</mn>
            <mo>,</mo><mi>…</mi>
       </math>";
    test_ClearSpeak("el", "ClearSpeak_Ellipses", "AndSoOn", expr,
            "αποσιωπητικά, κόμμα, μείον 2 κόμμα, μείον 1 κόμμα, 0 κόμμα, 1 κόμμα, 2 κόμμα; αποσιωπητικά")?;
            return Ok(());

}

#[test]
fn vertical_line_auto() -> Result<()> {
    let expr = "<math>
        <mn>3</mn><mo>|</mo><mn>6</mn>
    </math>";
    test_ClearSpeak("el", "ClearSpeak_VerticalLine", "Auto", expr,
            "3 διαιρεί 6")?;
            return Ok(());

}

#[test]
fn vertical_line_divides() -> Result<()> {
    let expr = "<math>
        <mn>3</mn><mo>|</mo><mn>6</mn>
    </math>";
    test_ClearSpeak("el", "ClearSpeak_VerticalLine", "Divides", expr,
            "3 διαιρεί 6")?;
            return Ok(());

}

    #[test]
    fn vertical_line_given() -> Result<()> {
        let expr = "<math>
            <mn>3</mn><mo>|</mo><mn>6</mn>
        </math>";
        test_ClearSpeak("el", "ClearSpeak_VerticalLine", "Given", expr,
                "3 δεδομένου 6")?;
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
        test_ClearSpeak_prefs("el", vec![("ClearSpeak_VerticalLine", "Given"), ("ClearSpeak_ImpliedTimes", "None")]
                        , expr, "κεφαλαίο p; ανοίγει παρένθεση; κεφαλαίο A, δεδομένου, κεφαλαίο b; κλείνει παρένθεση")?;
                        return Ok(());
   
    }

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
    test_ClearSpeak("el", "ClearSpeak_VerticalLine", "Auto", expr,
            "το σύνολο όλων των x τέτοια ώστε x είναι μεγαλύτερο από 0")?;
            return Ok(());

}


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
    test_ClearSpeak("el", "ClearSpeak_VerticalLine", "SuchThat", expr,
            "το σύνολο όλων των x τέτοια ώστε x είναι μεγαλύτερο από 0")?;
            return Ok(());

}

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
    test_ClearSpeak("el", "ClearSpeak_VerticalLine", "Given", expr,
            "το σύνολο όλων των x τέτοια ώστε x είναι μεγαλύτερο από 0")?;
            return Ok(());

}

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
    test_ClearSpeak("el", "ClearSpeak_VerticalLine", "Auto", expr,
        "το σύνολο όλων των x τέτοια ώστε απόλυτη τιμή του x; είναι μεγαλύτερο από 2")?;
        return Ok(());

}

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
    test_ClearSpeak("el", "ClearSpeak_VerticalLine", "Auto", expr,
        "f του x, υπολογισμένο για; x ισούται με 5")?;
        return Ok(());

}

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
    test_ClearSpeak("el", "ClearSpeak_VerticalLine", "Auto", expr,
        "x στο τετράγωνο, συν x; υπολογισμένο για 1 μείον την ίδια έκφραση υπολογισμένη στο 0")?;
        return Ok(());

}
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
    test_ClearSpeak("el", "ClearSpeak_VerticalLine", "Divides", expr,
        "f του x, υπολογισμένο για; x ισούται με 5")?;
        return Ok(());

}

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
    test_ClearSpeak("el", "ClearSpeak_VerticalLine", "Given", expr,
        "x στο τετράγωνο, συν x; υπολογισμένο για 1 μείον την ίδια έκφραση υπολογισμένη στο 0")?;
        return Ok(());

}