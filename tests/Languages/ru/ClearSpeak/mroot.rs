use crate::common::*;

#[test]
fn msqrt_simple() {
    let expr = "<math>
                    <msqrt> <mi>x</mi> </msqrt>
                </math>";
    test("ru", "ClearSpeak", expr, "квадратный корень из икс");
}

#[test]
fn msqrt_simple_end_root() {
    let expr = "<math>
                    <msqrt> <mi>x</mi> </msqrt>
                </math>";
    test_ClearSpeak("ru", "ClearSpeak_Roots", "RootEnd", expr, "квадратный корень из икс, конец корня");
}

#[test]
fn msqrt_simple_positive() {
    let expr = "<math>
                    <msqrt> <mi>x</mi> </msqrt>
                </math>";
    test_ClearSpeak("ru", "ClearSpeak_Roots", "PosNegSqRoot", expr, "квадратный корень из икс");
}

#[test]
fn msqrt_simple_pos_end_root() {
    let expr = "<math>
                    <msqrt> <mi>x</mi> </msqrt>
                </math>";
    test_ClearSpeak("ru", "ClearSpeak_Roots", "PosNegSqRootEnd", expr, "квадратный корень из икс, конец корня");
}

#[test]
fn msqrt_simple_pos_end_with_neg_root() {
    let expr = "<math>
                    <mo>-</mo> <msqrt> <mi>x</mi> </msqrt>
                    <mo>-</mo> <mroot> <mi>x</mi> <mn>3</mn></mroot>
                </math>";
    test_ClearSpeak("ru", "ClearSpeak_Roots", "PosNegSqRootEnd", expr, 
    "минус квадратный корень из икс, конец корня; минус, кубический корень из икс, конец корня");
}

#[test]
fn mroot_simple_pos_end_with_neg_root() {
    let expr = "<math>
                    <mo>-</mo> <mroot> <mi>x</mi> <mn>3</mn></mroot>
                    <mo>-</mo> <msqrt> <mi>x</mi> </msqrt>

                </math>";
    test_ClearSpeak("ru", "ClearSpeak_Roots", "PosNegSqRoot", expr, 
    "минус кубический корень из икс; минус квадратный корень из икс");
}

#[test]
fn neg_without_root() {
    let expr = "<math>
                    <mo>-</mo> <mi>x</mi> <mo>-</mo> <mi>y</mi>
                </math>";
    test("ru", "ClearSpeak", expr, "минус икс минус игрек");
}

#[test]
fn msqrt() {
    let expr = "<math>
                    <msqrt>
                        <mrow> <mi>x</mi> <mo>+</mo> <mi>y</mi> </mrow>
                    </msqrt>
                </math>";
    test("ru", "ClearSpeak", expr, "квадратный корень из икс плюс игрек");
}

#[test]
fn mroot_as_square_root() {
    let expr = "<math>
                    <mroot> <mi>x</mi> <mn>2</mn> </mroot>
                </math>";
    test("ru", "ClearSpeak", expr, "квадратный корень из икс");
}

#[test]
fn cube_root() {
    let expr = "<math>
                    <mroot> <mi>x</mi> <mn>3</mn> </mroot>
                </math>";
    test("ru", "ClearSpeak", expr, "кубический корень из икс");
}

#[test]
fn ordinal_root() {
    let expr = "<math>
                    <mroot> <mi>x</mi> <mn>9</mn> </mroot>
                </math>";
    test("ru", "ClearSpeak", expr, "корень девятой степени из икс");
}

#[test]
fn simple_mi_root() {
    let expr = "<math>
                    <mroot> <mi>x</mi> <mi>n</mi> </mroot>
                </math>";
    test("ru", "ClearSpeak", expr, "корень степени эн из икс");
}

#[test]
fn mroot_simple_pos_end_root() {
    let expr = "<math>
                <mroot> <mi>x</mi> <mi>t</mi> </mroot>
                </math>";
    test_ClearSpeak("ru", "ClearSpeak_Roots", "PosNegSqRootEnd", expr, "корень степени тэ из икс, конец корня");
}

#[test]
fn mroot_simple_end_root() {
    let expr = "<math>
                    <mroot> <mrow> <mi>x</mi> <mo>+</mo> <mi>y</mi> </mrow> 
                    <mn>21</mn></mroot>
                </math>";
    test_ClearSpeak("ru", "ClearSpeak_Roots", "RootEnd", expr, "корень двадцать первой степени из икс плюс игрек, конец корня");
}

#[test]
fn simple_fraction_power() {
    let expr = "<math>
                    <mroot>
                        <mi>x</mi> 
                        <mfrac><mn>1</mn><mn>3</mn></mfrac>
                    </mroot>
                </math>";
    test("ru", "ClearSpeak", expr, "корень степени одна третья из икс");
}
