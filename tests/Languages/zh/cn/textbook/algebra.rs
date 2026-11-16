//! Mainland textbook terminology and structural speech regressions.

use crate::common::*;
use anyhow::Result;

#[test]
fn linear_algebra_intents_use_standard_noun_phrases() -> Result<()> {
    // These intents denote a property of an object or a map between spaces.
    test(
        "zh",
        "ClearSpeak",
        "<math><mrow intent='determinant($x)'><mi arg='x'>a</mi></mrow></math>",
        "a 的行列式",
    )?;
    test(
        "zh",
        "ClearSpeak",
        "<math><mrow intent='adjugate($x)'><mi arg='x'>a</mi></mrow></math>",
        "a 的伴随矩阵",
    )?;
    test(
        "zh",
        "ClearSpeak",
        "<math><mrow intent='magnitude($x)'><mi arg='x'>v</mi></mrow></math>",
        "v 的模",
    )?;
    test(
        "zh",
        "ClearSpeak",
        "<math><mrow intent='transpose($x)'><mi arg='x'>a</mi></mrow></math>",
        "a 的转置",
    )?;
    test(
        "zh",
        "ClearSpeak",
        "<math><mrow intent='trace($x)'><mi arg='x'>a</mi></mrow></math>",
        "a 的迹",
    )?;
    test(
        "zh",
        "ClearSpeak",
        "<math><mrow intent='dimension($x)'><mi arg='x'>v</mi></mrow></math>",
        "v 的维数",
    )?;
    test(
        "zh",
        "ClearSpeak",
        "<math><mrow intent='kernel($x)'><mi arg='x'>f</mi></mrow></math>",
        "f 的核",
    )?;
    test(
        "zh",
        "ClearSpeak",
        "<math><mrow intent='span($u,$v)'><mi arg='u'>u</mi><mi arg='v'>v</mi></mrow></math>",
        "由 u 与 v 张成的空间",
    )?;
    test(
        "zh",
        "ClearSpeak",
        "<math><mrow intent='span($u)'><mi arg='u'>u</mi></mrow></math>",
        "由 u 张成的空间",
    )?;
    test(
        "zh",
        "ClearSpeak",
        "<math><mrow intent='homomorphism($m)'><mi arg='m'>m</mi></mrow></math>",
        "m 的同态",
    )?;
    test(
        "zh",
        "ClearSpeak",
        "<math><mrow intent='homomorphism($m,$n)'><mi arg='m'>m</mi><mi arg='n'>n</mi></mrow></math>",
        "m 到 n 的同态",
    )
}

#[test]
fn long_multiscripts_state_the_remaining_order_naturally() -> Result<()> {
    // Once explicit pairs are exhausted, state that the remaining lower and upper scripts alternate.
    test(
        "zh",
        "SimpleSpeak",
        "<math><mmultiscripts><mi>T</mi><mprescripts/><mi>k</mi><mi>l</mi><mi>m</mi><mi>n</mi><mi>o</mi><mi>p</mi></mmultiscripts></math>",
        "大写 t 有 3 组前置上下标, 前下标 k 且 前上标 l, 前下标 m 且 前上标 n 其余前置下标、上标依次交替 o p 结束前置上下标",
    )?;
    test(
        "zh",
        "SimpleSpeak",
        "<math><mmultiscripts><mi>T</mi><mi>a</mi><mi>b</mi><mi>c</mi><mi>d</mi><mi>e</mi><mi>f</mi><mi>g</mi><mi>h</mi><mi>i</mi><mi>j</mi></mmultiscripts></math>",
        "大写 t 有 5 组后置上下标, 下标 a 且 上标 b 下标 c 且 上标 d 下标 e 且 上标 f 下标 g 且 上标 h 其余后置下标、上标依次交替 i j 结束后置上下标",
    )
}

#[test]
fn quotient_and_remainder_follow_operand_order() -> Result<()> {
    test(
        "zh",
        "ClearSpeak",
        "<math><mrow intent='quotient($a,$b)'><mi arg='a'>x</mi><mi arg='b'>y</mi></mrow></math>",
        "x 除以 y 的商",
    )?;
    test(
        "zh",
        "ClearSpeak",
        "<math><mrow intent='remainder($a,$b)'><mi arg='a'>x</mi><mi arg='b'>y</mi></mrow></math>",
        "x 除以 y 的余数",
    )
}

#[test]
fn matrix_announces_dimensions_and_rows() -> Result<()> {
    // Matrix row numbers follow the Chinese ordinal pattern "第 n 行".
    test(
        "zh",
        "SimpleSpeak",
        "<math><mrow><mo>(</mo><mtable><mtr><mtd><mn>1</mn></mtd><mtd><mn>2</mn></mtd></mtr><mtr><mtd><mn>3</mn></mtd><mtd><mn>4</mn></mtd></mtr></mtable><mo>)</mo></mrow></math>",
        "2 乘 2 矩阵; 第 1 行; 1, 2; 第 2 行; 3, 4",
    )
}
