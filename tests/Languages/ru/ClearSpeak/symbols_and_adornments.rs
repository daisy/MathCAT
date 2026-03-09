use crate::common::*;

#[test]
fn multiplication() {
    let expr = "<math>
                    <mn>2</mn><mo>×</mo><mn>3</mn>
                </math>";
    test("ru", "ClearSpeak", expr, "2 умножить на 3");
}

#[test]
fn multiplication_by() {
    let expr = "<math>
                    <mn>2</mn><mo>×</mo><mn>3</mn>
                </math>";
    test_ClearSpeak("ru", "ClearSpeak_MultSymbolX", "By", expr, "2 на 3");
}

#[test]
fn multiplication_cross() {
    let expr = "<math>
                    <mi>u</mi><mo>×</mo><mi>v</mi>
                </math>";
    test_ClearSpeak("ru", "ClearSpeak_MultSymbolX", "Cross", expr, "u векторное произведение v");
}

#[test]
fn ellipses_auto_start() {
    let expr = "<math>
            <mi>…</mi><mo>,</mo>
            <mo>-</mo><mn>2</mn><mo>,</mo><mo>-</mo><mn>1</mn><mo>,</mo><mn>0</mn>
        </math>";
    test("ru", "ClearSpeak", expr, "многоточие запятая, минус 2 запятая, минус 1 запятая, 0");
}

#[test]
fn ellipses_auto_end() {
    let expr = "<math>
            <mn>1</mn>
            <mo>,</mo>
            <mn>2</mn>
            <mo>,</mo>
            <mn>3</mn>
            <mo>,</mo>
            <mi>…</mi>
        </math>";
    test_ClearSpeak("ru", "ClearSpeak_Ellipses", "Auto", expr, "1 запятая, 2 запятая, 3 запятая, многоточие");
}

#[test]
fn ellipses_auto_middle() {
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
    test_ClearSpeak("ru", "ClearSpeak_Ellipses", "Auto", expr,
            "1 запятая, 2 запятая, 3 запятая, многоточие запятая, 20");
}

#[test]
fn ellipses_auto_both() {
    let expr = "<math>
            <mi>…</mi><mo>,</mo>
            <mo>-</mo><mn>2</mn><mo>,</mo><mo>-</mo><mn>1</mn><mo>,</mo><mn>0</mn><mo>,</mo><mn>1</mn><mo>,</mo><mn>2</mn>
            <mo>,</mo><mi>…</mi>
       </math>";
    test_ClearSpeak("ru", "ClearSpeak_Ellipses", "Auto", expr,
            "многоточие запятая, минус 2 запятая, минус 1 запятая, 0 запятая, 1 запятая, 2 запятая, многоточие");
}

#[test]
fn ellipses_and_so_on_start() {
    let expr = "<math>
            <mi>…</mi><mo>,</mo>
            <mo>-</mo><mn>2</mn><mo>,</mo><mo>-</mo><mn>1</mn><mo>,</mo><mn>0</mn>
        </math>";
        test_ClearSpeak("ru", "ClearSpeak_Ellipses", "AndSoOn", expr, "многоточие запятая, минус 2 запятая, минус 1 запятая, 0");
}

#[test]
fn ellipses_and_so_on_end() {
    let expr = "<math>
            <mn>1</mn>
            <mo>,</mo>
            <mn>2</mn>
            <mo>,</mo>
            <mn>3</mn>
            <mo>,</mo>
            <mi>…</mi>
        </math>";
    test_ClearSpeak("ru", "ClearSpeak_Ellipses", "AndSoOn", expr, "1 запятая, 2 запятая, 3 и так далее");
}

#[test]
fn ellipses_and_so_on_middle() {
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
    test_ClearSpeak("ru", "ClearSpeak_Ellipses", "AndSoOn", expr,
            "1 запятая, 2 запятая, 3 и так далее до 20");
}

#[test]
fn ellipses_and_so_on_both() {
    let expr = "<math>
            <mi>…</mi><mo>,</mo>
            <mo>-</mo><mn>2</mn><mo>,</mo><mo>-</mo><mn>1</mn><mo>,</mo><mn>0</mn><mo>,</mo><mn>1</mn><mo>,</mo><mn>2</mn>
            <mo>,</mo><mi>…</mi>
       </math>";
    test_ClearSpeak("ru", "ClearSpeak_Ellipses", "AndSoOn", expr,
            "многоточие запятая, минус 2 запятая, минус 1 запятая, 0 запятая, 1 запятая, 2 запятая, многоточие");
}

#[test]
fn vertical_line_auto() {
    let expr = "<math>
        <mn>3</mn><mo>|</mo><mn>6</mn>
    </math>";
    test_ClearSpeak("ru", "ClearSpeak_VerticalLine", "Auto", expr,
            "3 делит 6");
}

#[test]
fn vertical_line_divides() {
    let expr = "<math>
        <mn>3</mn><mo>|</mo><mn>6</mn>
    </math>";
    test_ClearSpeak("ru", "ClearSpeak_VerticalLine", "Divides", expr,
            "3 делит 6");
}

#[test]
fn vertical_line_given() {
    let expr = "<math>
            <mn>3</mn><mo>|</mo><mn>6</mn>
        </math>";
    test_ClearSpeak("ru", "ClearSpeak_VerticalLine", "Given", expr,
            "3 при условии 6");
}

#[test]
fn vertical_line_probability_given() {
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
    test_ClearSpeak_prefs("ru", vec![("ClearSpeak_VerticalLine", "Given"), ("ClearSpeak_ImpliedTimes", "None")],
                    expr, "пэ большое, открывается скобка, эй большое при условии бэ большое, закрывается скобка");
}

#[test]
fn vertical_line_set() {
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
    test_ClearSpeak("ru", "ClearSpeak_VerticalLine", "Auto", expr,
            "множество всех x таких, что x больше 0");
}

#[test]
fn vertical_line_set_such_that() {
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
    test_ClearSpeak("ru", "ClearSpeak_VerticalLine", "SuchThat", expr,
            "множество всех x таких, что x больше 0");
}

#[test]
fn vertical_line_set_given() {
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
    test_ClearSpeak("ru", "ClearSpeak_VerticalLine", "Given", expr,
            "множество всех x таких, что x больше 0");
}

#[test]
fn vertical_line_set_and_abs() {
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
    test_ClearSpeak("ru", "ClearSpeak_VerticalLine", "Auto", expr,
        "множество всех x таких, что абсолютное значение x; больше 2");
}

#[test]
fn vertical_line_evaluated_at() {
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
    test_ClearSpeak("ru", "ClearSpeak_VerticalLine", "Auto", expr,
        "эф от икс, вычисленное при, x равно 5");
}

#[test]
fn vertical_line_evaluated_at_both() {
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
    test_ClearSpeak("ru", "ClearSpeak_VerticalLine", "Auto", expr,
        "икс в квадрате плюс икс, вычисленное при 1 минус то же выражение, вычисленное при 0");
}

#[test]
fn vertical_line_evaluated_at_divides() {
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
    test_ClearSpeak("ru", "ClearSpeak_VerticalLine", "Divides", expr,
        "эф от икс, вычисленное при, x равно 5");
}

#[test]
fn vertical_line_evaluated_at_both_given() {
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
    test_ClearSpeak("ru", "ClearSpeak_VerticalLine", "Given", expr,
        "икс в квадрате плюс икс, вычисленное при 1 минус то же выражение, вычисленное при 0");
}