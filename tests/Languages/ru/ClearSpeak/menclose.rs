use crate::common::*;

#[test]
fn menclose_actuarial() {
    let expr = "<math>
                    <menclose notation='actuarial'>  <mn>3</mn><mo>+</mo><mn>2</mn><mi>i</mi> </menclose>
                </math>";
    test("ru", "ClearSpeak", expr, "актуарный символ, содержащее 3 плюс 2 и конец");
}

#[test]
fn menclose_box() {
    let expr = "<math>
                    <menclose notation='box circle'>  <mn>3</mn><mo>+</mo><mn>2</mn><mi>i</mi> </menclose>
                </math>";
    test("ru", "ClearSpeak", expr, "рамка, окружность, содержащее 3 плюс 2 и конец");
}

#[test]
fn menclose_left() {
    let expr = "<math>
                    <menclose notation='left'>  <mfrac><mn>3</mn><mn>2</mn></mfrac> </menclose>
                </math>";
    test("ru", "ClearSpeak", expr, "линия слева, содержащее три вторых конец");
}

#[test]
fn menclose_right() {
    let expr = "<math>
                    <menclose notation='right'>  <mfrac><mn>3</mn><mn>2</mn></mfrac> </menclose>
                </math>";
    test("ru", "ClearSpeak", expr, "линия справа, содержащее три вторых конец");
}

#[test]
fn menclose_top_bottom() {
    let expr = "<math>
                    <menclose notation='top bottom'>  <mfrac><mn>3</mn><mn>2</mn></mfrac> </menclose>
                </math>";
    test("ru", "ClearSpeak", expr, "линия сверху, снизу, содержащее три вторых конец");
}

#[test]
fn menclose_updiagonalstrike() {
    let expr = "<math>
                    <menclose notation='updiagonalstrike'>  <mfrac><mn>3</mn><mn>2</mn></mfrac> </menclose>
                </math>";
    test("ru", "ClearSpeak", expr, "диагональ вверх, перечеркивание, содержащее три вторых конец");
}

#[test]
fn menclose_downdiagonalstrike() {
    let expr = "<math>
                    <menclose notation='downdiagonalstrike'>  <mfrac><mn>3</mn><mn>2</mn></mfrac> </menclose>
                </math>";
    test("ru", "ClearSpeak", expr, "диагональ вниз, перечеркивание, содержащее три вторых конец");
}

#[test]
fn menclose_cross_out() {
    let expr = "<math>
                    <menclose notation='updiagonalstrike downdiagonalstrike'>  <mfrac><mn>3</mn><mn>2</mn></mfrac> </menclose>
                </math>";
    test("ru", "ClearSpeak", expr, "икс, перечеркивание, содержащее три вторых конец");
}

#[test]
fn menclose_vertical_horizontal_strike() {
    let expr = "<math>
                    <menclose notation='verticalstrike horizontalstrike'>  <mfrac><mn>3</mn><mn>2</mn></mfrac> </menclose>
                </math>";
    test("ru", "ClearSpeak", expr, "вертикальная, горизонтальная, перечеркивание, содержащее три вторых конец");
}

#[test]
fn menclose_leftarrow() {
    let expr = "<math>
                    <menclose notation='leftarrow'> <mfrac><mn>3</mn><mn>2</mn></mfrac> </menclose>
                </math>";
    test("ru", "ClearSpeak", expr, "стрелка влево, содержащее три вторых конец");
}

#[test]
fn menclose_right_up_down_arrow() {
    let expr = "<math>
                    <menclose notation=' rightarrow downarrow  uparrow  '> <mfrac><mn>3</mn><mn>2</mn></mfrac> </menclose>
                </math>";
    test("ru", "ClearSpeak", expr, "стрелка вверх, стрелка вниз, стрелка вправо, содержащее три вторых конец");
}

#[test]
fn menclose_northeastarrow() {
    let expr = "<math>
                    <menclose notation='northeastarrow'> <mfrac><mn>3</mn><mn>2</mn></mfrac> </menclose>
                </math>";
    test("ru", "ClearSpeak", expr, "стрелка на северо-восток, содержащее три вторых конец");
}

#[test]
fn menclose_other_single_arrows() {
    let expr = "<math>
                    <menclose notation='northwestarrow southwestarrow southeastarrow'> <mfrac><mn>3</mn><mn>2</mn></mfrac> </menclose>
                </math>";
    test("ru", "ClearSpeak", expr, "стрелка на юго-восток, стрелка на юго-запад, стрелка на северо-запад, содержащее три вторых конец");
}

#[test]
fn menclose_northwestsoutheastarrow() {
    let expr = "<math>
                    <menclose notation='northwestsoutheastarrow'> <mfrac><mn>3</mn><mn>2</mn></mfrac> </menclose>
                </math>";
    test("ru", "ClearSpeak", expr, "двунаправленная диагональная стрелка вниз, содержащее три вторых конец");
}

#[test]
fn menclose_other_double_arrows() {
    let expr = "<math>
                    <menclose notation='updownarrow leftrightarrow northeastsouthwestarrow'>  <mfrac><mn>3</mn><mn>2</mn></mfrac> </menclose>
                </math>";
    test("ru", "ClearSpeak", expr, "двунаправленная вертикальная стрелка, двунаправленная горизонтальная стрелка, двунаправленная диагональная стрелка вверх, содержащее три вторых конец");
}

#[test]
fn menclose_madrub() {
    let expr = "<math>
                    <menclose notation='madrub'> <mfrac><mn>3</mn><mn>2</mn></mfrac> </menclose>
                </math>";
    test("ru", "ClearSpeak", expr, "арабский символ факториала, содержащее три вторых конец");
}

#[test]
fn menclose_phasorangle() {
    let expr = "<math>
                    <menclose notation='phasorangle'> <mfrac><mn>3</mn><mn>2</mn></mfrac> </menclose>
                </math>";
    test("ru", "ClearSpeak", expr, "угол фазы, содержащее три вторых конец");
}

#[test]
fn menclose_circle_phasorangle() {
    let expr = "<math>
                    <menclose notation='phasorangle circle'> <mfrac><mn>3</mn><mn>2</mn></mfrac> </menclose>
                </math>";
    test("ru", "ClearSpeak", expr, "окружность, угол фазы, содержащее три вторых конец");
}

#[test]
fn menclose_longdiv() {
    let expr = "<math>
                    <menclose notation='longdiv'> <mfrac><mn>3</mn><mn>2</mn></mfrac> </menclose>
                </math>";
    test("ru", "ClearSpeak", expr, "знак деления уголком, содержащее три вторых конец");
}

#[test]
fn menclose_longdiv_default() {
    let expr = "<math>
                    <menclose> <mfrac><mn>3</mn><mn>2</mn></mfrac> </menclose>
                </math>";
    test("ru", "ClearSpeak", expr, "знак деления уголком, содержащее три вторых конец");
}

#[test]
fn menclose_longdiv_empty_string() {
    let expr = "<math>
                    <menclose notation=''> <mfrac><mn>3</mn><mn>2</mn></mfrac> </menclose>
                </math>";
    test("ru", "ClearSpeak", expr, "знак деления уголком, содержащее три вторых конец");
}

#[test]
fn menclose_longdiv_whitespace_string() {
    let expr = "<math>
                    <menclose notation='  '> <mfrac><mn>3</mn><mn>2</mn></mfrac> </menclose>
                </math>";
    test("ru", "ClearSpeak", expr, "знак деления уголком, содержащее три вторых конец");
}

#[test]
fn menclose_radical() {
    let expr = "<math>
                    <menclose notation='radical'> <mfrac><mn>3</mn><mn>2</mn></mfrac> </menclose>
                </math>";
    test("ru", "ClearSpeak", expr, "квадратный корень, содержащее три вторых конец");
}

#[test]
fn simple_speak_menclose_top_bottom() {
    let expr = "<math>
                    <menclose notation='top bottom'>  <mfrac><mn>3</mn><mn>2</mn></mfrac> </menclose>
                </math>";
    test("ru", "SimpleSpeak", expr, "линия сверху, снизу, содержащее три вторых конец");
}
