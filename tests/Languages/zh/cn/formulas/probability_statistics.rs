//! Representative formula and domain-specific speech regressions.

use crate::common::*;
use anyhow::Result;

#[test]
fn symbolic_sets_logic_and_probability_read_as_complete_expressions() -> Result<()> {
    // Test the presentation notation users encounter, in addition to semantic intent rules.
    let cases = [
        (
            "set-membership",
            "<math><mi>x</mi><mo>&#x2208;</mo><mi>&#x211d;</mi><mo>,</mo><mi>n</mi><mo>&#x2209;</mo><mi>&#x2124;</mi></math>",
            "x 属于 实数集, 逗号; n 不属于 整数集",
        ),
        (
            "nested-quantifiers",
            "<math><mo>&#x2200;</mo><mi>x</mi><mo>&#x2203;</mo><mi>y</mi><mo>:</mo><mi>P</mi><mo>(</mo><mi>x</mi><mo>,</mo><mi>y</mi><mo>)</mo></math>",
            "任意 x 存在 y, 冒号; 大写 p, 左括号 x 逗号, y, 右括号",
        ),
        (
            "probability-function",
            "<math><mi>P</mi><mo>(</mo><mi>A</mi><mo>)</mo></math>",
            "大写 a 的概率",
        ),
        (
            "logical-equivalence",
            "<math><mi>p</mi><mo>&#x21d2;</mo><mi>q</mi><mo>&#x21d4;</mo><mi>r</mi><mo>&#x2227;</mo><mi>s</mi></math>",
            "p 推出, q 当且仅当 r 且 s",
        ),
        (
            "conditional-probability-identity",
            "<math><mi>P</mi><mo>(</mo><mi>A</mi><mo>|</mo><mi>B</mi><mo>)</mo><mo>=</mo><mfrac><mrow><mi>P</mi><mo>(</mo><mi>A</mi><mo>&#x2229;</mo><mi>B</mi><mo>)</mo></mrow><mrow><mi>P</mi><mo>(</mo><mi>B</mi><mo>)</mo></mrow></mfrac></math>",
            "在 大写 b 条件下 大写 a 的概率, 等于; 分数, 大写 b 的概率, 分之, 大写 a 交 大写 b 的概率, 结束分数",
        ),
    ];

    for (name, mathml, expected) in cases {
        test("zh", "SimpleSpeak", mathml, expected)
            .map_err(|error| anyhow::anyhow!("{name}: {error}"))?;
    }
    Ok(())
}
