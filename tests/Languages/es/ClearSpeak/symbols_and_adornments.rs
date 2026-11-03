use crate::common::*;
use anyhow::Result;

#[test]
fn multiplication() -> Result<()> {
    let expr = "<math>
                    <mn>2</mn><mo>×</mo><mn>3</mn>
                </math>";
    test("es", "ClearSpeak", expr, "2 por 3")?;
    return Ok(());

}

#[test]
fn multiplication_by() -> Result<()> {
    let expr = "<math>
                    <mn>2</mn><mo>×</mo><mn>3</mn>
                </math>";
    test_ClearSpeak("es", "ClearSpeak_MultSymbolX", "By", expr, "2 por 3")?;
    return Ok(());

}

#[test]
fn multiplication_cross() -> Result<()> {
    let expr = "<math>
                    <mi>u</mi><mo>×</mo><mi>v</mi>
                </math>";
    test_ClearSpeak("es", "ClearSpeak_MultSymbolX", "Cross", expr, "u cruz v")?;
    return Ok(());

}

#[test]
fn ellipses_auto_start() -> Result<()> {
    let expr = "<math>
            <mi>…</mi><mo>,</mo>
            <mo>-</mo><mn>2</mn><mo>,</mo><mo>-</mo><mn>1</mn><mo>,</mo><mn>0</mn>
        </math>";
    test("es", "ClearSpeak", expr, "puntos suspensivos coma, menos 2 coma, menos 1 coma, 0")?;
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
    test_ClearSpeak("es", "ClearSpeak_Ellipses", "Auto", expr, "1 coma, 2 coma, 3 coma, puntos suspensivos")?;
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
    test_ClearSpeak("es", "ClearSpeak_Ellipses", "Auto", expr,
            "1 coma, 2 coma, 3 coma, puntos suspensivos coma, 20")?;
            return Ok(());

}

#[test]
fn ellipses_auto_both() -> Result<()> {
    let expr = "<math>
            <mi>…</mi><mo>,</mo>
            <mo>-</mo><mn>2</mn><mo>,</mo><mo>-</mo><mn>1</mn><mo>,</mo><mn>0</mn><mo>,</mo><mn>1</mn><mo>,</mo><mn>2</mn>
            <mo>,</mo><mi>…</mi>
       </math>";
    test_ClearSpeak("es", "ClearSpeak_Ellipses", "Auto", expr,
            "puntos suspensivos coma, menos 2 coma, menos 1 coma, 0 coma, 1 coma, 2 coma, puntos suspensivos")?;
            return Ok(());

}

#[test]
fn ellipses_and_so_on_start() -> Result<()> {
    let expr = "<math>
            <mi>…</mi><mo>,</mo>
            <mo>-</mo><mn>2</mn><mo>,</mo><mo>-</mo><mn>1</mn><mo>,</mo><mn>0</mn>
        </math>";
        test_ClearSpeak("es", "ClearSpeak_Ellipses", "AndSoOn", expr, "puntos suspensivos coma, menos 2 coma, menos 1 coma, 0")?;
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
    test_ClearSpeak("es", "ClearSpeak_Ellipses", "AndSoOn", expr, "1 coma, 2 coma, 3 etcétera")?;
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
    test_ClearSpeak("es", "ClearSpeak_Ellipses", "AndSoOn", expr,
            "1 coma, 2 coma, 3 puntos suspensivos 20")?;
            return Ok(());

}

#[test]
fn ellipses_and_so_on_both() -> Result<()> {
    let expr = "<math>
            <mi>…</mi><mo>,</mo>
            <mo>-</mo><mn>2</mn><mo>,</mo><mo>-</mo><mn>1</mn><mo>,</mo><mn>0</mn><mo>,</mo><mn>1</mn><mo>,</mo><mn>2</mn>
            <mo>,</mo><mi>…</mi>
       </math>";
    test_ClearSpeak("es", "ClearSpeak_Ellipses", "AndSoOn", expr,
            "puntos suspensivos coma, menos 2 coma, menos 1 coma, 0 coma, 1 coma, 2 coma, puntos suspensivos")?;
            return Ok(());

}

#[test]
fn vertical_line_auto() -> Result<()> {
    let expr = "<math>
        <mn>3</mn><mo>|</mo><mn>6</mn>
    </math>";
    test_ClearSpeak("es", "ClearSpeak_VerticalLine", "Auto", expr,
            "3 divide a 6")?;
            return Ok(());

}

#[test]
fn vertical_line_divides() -> Result<()> {
    let expr = "<math>
        <mn>3</mn><mo>|</mo><mn>6</mn>
    </math>";
    test_ClearSpeak("es", "ClearSpeak_VerticalLine", "Divides", expr,
            "3 divide a 6")?;
            return Ok(());

}

    #[test]
    fn vertical_line_given() -> Result<()> {
        let expr = "<math>
            <mn>3</mn><mo>|</mo><mn>6</mn>
        </math>";
        test_ClearSpeak("es", "ClearSpeak_VerticalLine", "Given", expr,
                "3 dado 6")?;
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
        test_ClearSpeak_prefs("es", vec![("ClearSpeak_VerticalLine", "Given"), ("ClearSpeak_ImpliedTimes", "None")]
                        , expr, "mayúscula p; se abren paréntesis, mayúscula a dado mayúscula b; se cierran paréntesis")?;
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
    test_ClearSpeak("es", "ClearSpeak_VerticalLine", "Auto", expr,
            "el conjunto de todos los x tal que x es mayor que 0")?;
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
    test_ClearSpeak("es", "ClearSpeak_VerticalLine", "SuchThat", expr,
            "el conjunto de todos los x tal que x es mayor que 0")?;
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
    test_ClearSpeak("es", "ClearSpeak_VerticalLine", "Given", expr,
            "el conjunto de todos los x tal que x es mayor que 0")?;
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
    test_ClearSpeak("es", "ClearSpeak_VerticalLine", "Auto", expr,
        "el conjunto de todos los x tal que el valor absoluto de x; es mayor que 2")?;
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
    test_ClearSpeak("es", "ClearSpeak_VerticalLine", "Auto", expr,
        "f de x evaluado en x es igual a 5")?;
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
    test_ClearSpeak("es", "ClearSpeak_VerticalLine", "Auto", expr,
        "x al cuadrado más x, evaluado en 1 menos la misma expresión evaluada en 0")?;
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
    test_ClearSpeak("es", "ClearSpeak_VerticalLine", "Divides", expr,
        "f de x evaluado en x es igual a 5")?;
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
    test_ClearSpeak("es", "ClearSpeak_VerticalLine", "Given", expr,
        "x al cuadrado más x, evaluado en 1 menos la misma expresión evaluada en 0")?;
        return Ok(());

}

#[test]
fn double_factorial_test() -> Result<()> {
    let expr1 = "<math><mn>5</mn><mo>!!</mo></math>";
    let expr2 = "<math><mn>5</mn><mo>‼</mo></math>";
    test("es", "ClearSpeak", expr2, "5 doble factorial")?;
    test("es", "ClearSpeak", expr1, "5 doble factorial")?;
    test("es", "SimpleSpeak", expr1, "5 doble factorial")?;
    Ok(())
}

