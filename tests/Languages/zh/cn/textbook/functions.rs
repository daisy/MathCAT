//! Mainland textbook terminology and structural speech regressions.

use crate::common::*;
use anyhow::Result;

#[test]
fn function_intents_use_natural_chinese_argument_order() -> Result<()> {
    // Chinese property names follow their argument; binary relationships keep their semantic order.
    test(
        "zh",
        "ClearSpeak",
        "<math><mrow intent='domain($x)'><mi arg='x'>f</mi></mrow></math>",
        "f 的定义域",
    )?;
    test(
        "zh",
        "ClearSpeak",
        "<math><mrow intent='complex-conjugate($x)'><mi arg='x'>z</mi></mrow></math>",
        "z 的共轭复数",
    )?;
    test(
        "zh",
        "SimpleSpeak",
        "<math><mrow intent='fractional-part($x)'><mi arg='x'>x</mi></mrow></math>",
        "x 的小数部分",
    )?;
    test(
        "zh",
        "SimpleSpeak",
        "<math><mrow intent='floor($x)'><mi arg='x'>x</mi></mrow></math>",
        "x 向下取整",
    )?;
    test(
        "zh",
        "SimpleSpeak",
        "<math><mrow intent='round($x)'><mi arg='x'>x</mi></mrow></math>",
        "x 四舍五入后的值",
    )?;
    test(
        "zh",
        "ClearSpeak",
        "<math><mrow intent='greatest-common-divisor($a,$b)'><mi arg='a'>x</mi><mi arg='b'>y</mi></mrow></math>",
        "x 与 y 的最大公约数",
    )?;
    test(
        "zh",
        "ClearSpeak",
        "<math><mrow intent='least-common-multiple($a,$b)'><mi arg='a'>x</mi><mi arg='b'>y</mi></mrow></math>",
        "x 与 y 的最小公倍数",
    )?;
    test(
        "zh",
        "ClearSpeak",
        "<math><mrow intent='conditional-probability($a,$b)'><mi arg='a'>a</mi><mi arg='b'>b</mi></mrow></math>",
        "在 b 条件下 a 的概率",
    )?;
    test(
        "zh",
        "ClearSpeak",
        "<math><mrow intent='tends-to-from-above($a,$b)'><mi arg='a'>x</mi><mi arg='b'>y</mi></mrow></math>",
        "x 从上方趋于 y",
    )?;
    test(
        "zh",
        "ClearSpeak",
        "<math><mrow intent='tends-to-from-below($a,$b)'><mi arg='a'>x</mi><mi arg='b'>y</mi></mrow></math>",
        "x 从下方趋于 y",
    )?;
    test(
        "zh",
        "ClearSpeak",
        "<math><mrow intent='set-difference($a,$b)'><mi arg='a'>a</mi><mi arg='b'>b</mi></mrow></math>",
        "a 与 b 的差集",
    )?;
    test(
        "zh",
        "ClearSpeak",
        "<math><mrow intent='least-common-denominator($x,$y,$z)'><mi arg='x'>x</mi><mi arg='y'>y</mi><mi arg='z'>z</mi></mrow></math>",
        "x, y 与 z 的最小公分母",
    )
}

#[test]
fn argument_owned_function_variants_keep_their_standard_terms() -> Result<()> {
    // Exercise every remaining branch of the shared argument-first function rule.
    let cases = [
        ("inverse", "f", "f 的逆"),
        ("codomain", "f", "f 的陪域"),
        ("image", "f", "f 的像"),
        ("max", "s", "s 的最大值"),
        ("min", "s", "s 的最小值"),
        ("complex-arg", "z", "z 的辐角"),
        ("real-part", "z", "z 的实部"),
        ("imaginary-part", "z", "z 的虚部"),
        ("complement", "a", "a 的补集"),
        ("cardinality", "s", "s 的基数"),
        ("probability", "a", "a 的概率"),
        ("volume", "v", "v 的体积"),
        ("chemistry-concentration", "c", "c 的浓度"),
        ("ceiling", "x", "x 向上取整"),
    ];

    for (intent, argument, expected) in cases {
        let mathml = format!(
            "<math><mrow intent='{intent}($x)'><mi arg='x'>{argument}</mi></mrow></math>"
        );
        test("zh", "ClearSpeak", &mathml, expected)
            .map_err(|error| anyhow::anyhow!("{intent}: {error}"))?;
    }
    Ok(())
}
