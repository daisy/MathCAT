//! Representative formula and domain-specific speech regressions.

use crate::common::*;
use anyhow::Result;

#[test]
fn calculus_formulas_cover_limits_derivatives_and_improper_integrals() -> Result<()> {
    // Exercise complete textbook formulas so contextual operators are not only tested in isolation.
    let cases = [
        (
            "limit-at-infinity",
            "<math><munder><mo>lim</mo><mrow><mi>x</mi><mo>&#x2192;</mo><mi>&#x221e;</mi></mrow></munder><mfrac><mn>1</mn><mi>x</mi></mfrac></math>",
            "极限，当 x 趋于 无穷大; x 分之 1",
        ),
        (
            "left-hand-limit",
            "<math><munder><mo>lim</mo><mrow><mi>x</mi><mo>&#x2197;</mo><mn>2</mn></mrow></munder><mi>f</mi><mo>(</mo><mi>x</mi><mo>)</mo></math>",
            "极限，当 x 从下方趋于 2; f x",
        ),
        (
            "right-hand-limit",
            "<math><munder><mo>lim</mo><mrow><mi>x</mi><mo>&#x2198;</mo><mn>2</mn></mrow></munder><mi>f</mi><mo>(</mo><mi>x</mi><mo>)</mo></math>",
            "极限，当 x 从上方趋于 2; f x",
        ),
        (
            "first-derivative",
            "<math><mfrac><mrow><mo>d</mo><mi>y</mi></mrow><mrow><mo>d</mo><mi>x</mi></mrow></mfrac><mo>=</mo><mn>2</mn><mi>x</mi></math>",
            "分数, d x, 分之, d y, 结束分数; 等于 2 x",
        ),
        (
            "second-derivative",
            "<math><mfrac><mrow><msup><mo>d</mo><mn>2</mn></msup><mi>y</mi></mrow><mrow><mi>d</mi><msup><mi>x</mi><mn>2</mn></msup></mrow></mfrac></math>",
            "分数, d x 平方, 分之, d 平方 y, 结束分数",
        ),
        (
            "improper-integral",
            "<math><msubsup><mo>&#x222b;</mo><mn>0</mn><mi>&#x221e;</mi></msubsup><msup><mi>e</mi><mrow><mo>&#x2212;</mo><mi>x</mi></mrow></msup><mo>&#x2062;</mo><mi>d</mi><mi>x</mi></math>",
            "积分 从 0 到 无穷大; e 的 负 x 次方, d x",
        ),
    ];

    for (name, mathml, expected) in cases {
        test("zh-cn", "SimpleSpeak", mathml, expected)
            .map_err(|error| anyhow::anyhow!("{name}: {error}"))?;
    }
    Ok(())
}

#[test]
fn calculus_and_operator_cases_keep_high_risk_order() -> Result<()> {
    // These formulas exercise operator order, limits, and nested operands that are easy to
    // misread when English word order is copied directly into Chinese.
    let cases = [
        (
            "principal-logarithm",
            "<math><mi>Log</mi><mo>⁡</mo><mi>z</mi></math>",
            "主值对数 z",
        ),
        (
            "logarithm-with-natural-base",
            "<math><msub><mi>log</mi><mi>e</mi></msub><mi>x</mi></math>",
            "以 e 为底, x 的对数",
        ),
        (
            "contour-integral",
            "<math><mo>∮</mo><mi>f</mi><mo>⁡</mo><mi>z</mi><mi>d</mi><mi>z</mi></math>",
            "围道积分 f z d z",
        ),
        (
            "surface-integral",
            "<math><mo>∯</mo><mi>f</mi><mo>⁡</mo><mi>x</mi><mi>d</mi><mi>S</mi></math>",
            "曲面积分 f x d 大写 s",
        ),
        (
            "volume-integral",
            "<math><mo>∰</mo><mi>f</mi><mo>⁡</mo><mi>x</mi><mi>d</mi><mi>V</mi></math>",
            "体积分 f x d 大写 v",
        ),
        (
            "triple-integral",
            "<math><mo>∭</mo><mi>f</mi><mo>⁡</mo><mi>x</mi><mi>y</mi><mi>z</mi><mi>d</mi><mi>x</mi><mi>d</mi><mi>y</mi><mi>d</mi><mi>z</mi></math>",
            "三重积分 f xyzdxdydz",
        ),
        (
            "intersection-with-limits",
            "<math><munderover><mo>⋂</mo><mrow><mi>i</mi><mo>=</mo><mn>1</mn></mrow><mi>n</mi></munderover><msub><mi>A</mi><mi>i</mi></msub></math>",
            "n 元交集 从 i 等于 1 到 n; 大写 a 下标 i",
        ),
        (
            "union-with-limits",
            "<math><munderover><mo>⋃</mo><mrow><mi>i</mi><mo>=</mo><mn>1</mn></mrow><mi>n</mi></munderover><msub><mi>A</mi><mi>i</mi></msub></math>",
            "n 元并集 从 i 等于 1 到 n; 大写 a 下标 i",
        ),
        (
            "product-with-limits-and-expression",
            "<math><munderover><mo>∏</mo><mrow><mi>j</mi><mo>=</mo><mn>1</mn></mrow><mi>m</mi></munderover><mrow><mo>(</mo><mn>1</mn><mo>+</mo><msub><mi>x</mi><mi>j</mi></msub><mo>)</mo></mrow></math>",
            "连乘 从 j 等于 1 到 m; 左括号, 1 加 x 下标 j; 右括号",
        ),
        (
            "function-composition",
            "<math><mi>f</mi><mo>∘</mo><mi>g</mi></math>",
            "f 复合 g",
        ),
        (
            "evaluated-at-bar",
            "<math><mrow><mi>f</mi><mo>⁡</mo><mi>x</mi></mrow><msub><mo>|</mo><mn>0</mn></msub></math>",
            "f x 在 0 处的值",
        ),
        (
            "integration-by-parts",
            "<math><mo>∫</mo><mi>u</mi><mi>d</mi><mi>v</mi><mo>=</mo><mi>u</mi><mi>v</mi><mo>−</mo><mo>∫</mo><mi>v</mi><mi>d</mi><mi>u</mi></math>",
            "积分 u d v, 等于, u v 减 积分 v d u",
        ),
        (
            "binomial-expansion",
            "<math><msup><mrow><mo>(</mo><mi>x</mi><mo>+</mo><mi>y</mi><mo>)</mo></mrow><mn>2</mn></msup><mo>=</mo><msup><mi>x</mi><mn>2</mn></msup><mo>+</mo><mn>2</mn><mi>x</mi><mi>y</mi><mo>+</mo><msup><mi>y</mi><mn>2</mn></msup></math>",
            "左括号 x 加 y 右括号 平方; 等于, x 平方 加 2 x y, 加 y 平方",
        ),
        (
            "divisibility-pair",
            "<math><mn>3</mn><mo>∣</mo><mn>12</mn><mo>,</mo><mn>5</mn><mo>∤</mo><mn>12</mn></math>",
            "3 整除 12, 逗号, 5 不整除 12",
        ),
        (
            "prime-sequence-term",
            "<math><msub><mi>p</mi><mi>n</mi></msub><mo>≥</mo><mn>2</mn></math>",
            "p 下标 n; 大于等于 2",
        ),
    ];

    for (name, mathml, expected) in cases {
        for style in ["SimpleSpeak", "ClearSpeak"] {
            test("zh-cn", style, mathml, expected)
                .map_err(|error| anyhow::anyhow!("{style}/{name}: {error}"))?;
        }
    }

    test(
        "zh-cn",
        "SimpleSpeak",
        "<math><mfrac><mrow><mi>d</mi><mi>y</mi></mrow><mrow><mi>d</mi><mi>x</mi></mrow></mfrac><mo>=</mo><mfrac><mrow><mi>d</mi><mi>y</mi></mrow><mrow><mi>d</mi><mi>u</mi></mrow></mfrac><mfrac><mrow><mi>d</mi><mi>u</mi></mrow><mrow><mi>d</mi><mi>x</mi></mrow></mfrac></math>",
        "分数, d x, 分之, d y, 结束分数; 等于; 分数, d u, 分之, d y, 结束分数; 分数, d x, 分之, d u, 结束分数",
    )?;
    test(
        "zh-cn",
        "ClearSpeak",
        "<math><mfrac><mrow><mi>d</mi><mi>y</mi></mrow><mrow><mi>d</mi><mi>x</mi></mrow></mfrac><mo>=</mo><mfrac><mrow><mi>d</mi><mi>y</mi></mrow><mrow><mi>d</mi><mi>u</mi></mrow></mfrac><mfrac><mrow><mi>d</mi><mi>u</mi></mrow><mrow><mi>d</mi><mi>x</mi></mrow></mfrac></math>",
        "d x 分之 d y, 等于; d u 分之 d y; d x 分之 d u",
    )
}
