//! Representative formula and domain-specific speech regressions.

use crate::common::*;
use anyhow::Result;

#[test]
fn vectors_geometry_and_complex_conjugates_keep_mathematical_context() -> Result<()> {
    // Combine notation so arrows, bars, angle signs, and relation symbols retain their mathematical meanings.
    let cases = [
        (
            "vector-components",
            "<math><mover><mi>v</mi><mo>&#x2192;</mo></mover><mo>=</mo><mi>a</mi><mi>i</mi><mo>+</mo><mi>b</mi><mi>j</mi></math>",
            "向量 v, 等于, a i 加 b j",
        ),
        (
            "right-angle-and-parallel-lines",
            "<math><mo>&#x2220;</mo><mi>A</mi><mo>=</mo><mn>90</mn><mo>&#xb0;</mo><mo>,</mo><mi>l</mi><mo>&#x2225;</mo><mi>m</mi></math>",
            "角 大写 a 等于 90 度; 逗号, l 平行于 m",
        ),
        (
            "complex-conjugate-bar",
            "<math><mi>z</mi><mo>=</mo><mn>3</mn><mo>+</mo><mn>4</mn><mi>i</mi><mo>,</mo><mover><mi>z</mi><mo>&#xaf;</mo></mover></math>",
            "z 等于 3 加 4 i; 逗号, z 上横线",
        ),
    ];

    for (name, mathml, expected) in cases {
        test("zh-cn", "SimpleSpeak", mathml, expected)
            .map_err(|error| anyhow::anyhow!("{name}: {error}"))?;
    }
    Ok(())
}

#[test]
fn probability_linear_algebra_and_geometry_keep_context() -> Result<()> {
    // These are complete textbook formulas rather than isolated symbols, so operand order and
    // matrix boundaries are checked together.
    let cases = [
        (
            "covariance",
            "<math><mi>Cov</mi><mo>⁡</mo><mrow><mo>(</mo><mi>X</mi><mo>,</mo><mi>Y</mi><mo>)</mo></mrow></math>",
            "协方差, 左括号, 大写 x 逗号, 大写 y; 右括号",
            "协方差, 左括号, 大写 x 逗号, 大写 y; 右括号",
        ),
        (
            "correlation-coefficient",
            "<math><mi>ρ</mi><mo>=</mo><mfrac><mrow><mi>Cov</mi><mo>⁡</mo><mrow><mo>(</mo><mi>X</mi><mo>,</mo><mi>Y</mi><mo>)</mo></mrow></mrow><mrow><msub><mi>σ</mi><mi>X</mi></msub><msub><mi>σ</mi><mi>Y</mi></msub></mrow></mfrac></math>",
            "rho 等于; 分数, sigma 下标 大写 x; sigma 下标 大写 y; 分之, 协方差, 左括号, 大写 x 逗号, 大写 y; 右括号, 结束分数",
            "rho 等于; 分数，分子为; 协方差, 左括号, 大写 x 逗号, 大写 y; 右括号; 分母为 sigma 下标 大写 x; sigma 下标 大写 y",
        ),
        (
            "normal-density",
            "<math><mi>f</mi><mo>(</mo><mi>x</mi><mo>)</mo><mo>=</mo><mfrac><mn>1</mn><mrow><mi>σ</mi><msqrt><mn>2</mn><mi>π</mi></msqrt></mrow></mfrac><msup><mi>e</mi><mrow><mo>−</mo><mfrac><mrow><mo>(</mo><mi>x</mi><mo>−</mo><mi>μ</mi><mo>)</mo></mrow><mn>2</mn></mfrac></mrow></msup></math>",
            "f x 等于; 分数, sigma 乘 根号 2 pi 结束根号; 分之 1, 结束分数; 乘; e 的 负 分数, 2 分之, 左括号 x 减 mu 右括号, 结束分数; 次方",
            "f x 等于; 分数，分子为 1; 分母为 sigma 乘 根号 2 pi; 乘; e 的 负 分数，分子为; 左括号 x 减 mu 右括号; 分母为 2; 次方",
        ),
        (
            "binomial-probability",
            "<math><mi>P</mi><mo>(</mo><mi>X</mi><mo>=</mo><mi>k</mi><mo>)</mo><mo>=</mo><mrow><mo>(</mo><mfrac linethickness='0em'><mi>n</mi><mi>k</mi></mfrac><mo>)</mo></mrow><msup><mi>p</mi><mi>k</mi></msup><msup><mrow><mo>(</mo><mn>1</mn><mo>−</mo><mi>p</mi><mo>)</mo></mrow><mrow><mi>n</mi><mo>−</mo><mi>k</mi></mrow></msup></math>",
            "大写 x 等于 k 的概率, 等于; n 取 k p 的 k 次方 乘, 左括号 1 减 p 右括号 的 n 减 k 次方",
            "大写 x 等于 k 的概率, 等于; n 取 k p 的 k 次方 乘, 左括号 1 减 p 右括号 的 n 减 k 次方",
        ),
        (
            "complex-modulus",
            "<math><mo>|</mo><mi>z</mi><mo>|</mo><mo>=</mo><msqrt><msup><mi>x</mi><mn>2</mn></msup><mo>+</mo><msup><mi>y</mi><mn>2</mn></msup></msqrt></math>",
            "z 的绝对值 等于, 根号 x 平方 加 y 平方 结束根号",
            "z 的绝对值, 等于, 根号 x 平方 加 y 平方",
        ),
        (
            "polar-complex-form",
            "<math><mi>z</mi><mo>=</mo><mi>r</mi><mrow><mo>(</mo><mi>cos</mi><mo>⁡</mo><mi>θ</mi><mo>+</mo><mi>i</mi><mi>sin</mi><mo>⁡</mo><mi>θ</mi><mo>)</mo></mrow></math>",
            "z 等于; r; 左括号; 余弦 theta, 加, i 正弦 theta; 右括号",
            "z 等于; r; 左括号; 余弦 theta, 加, i 正弦 theta; 右括号",
        ),
        (
            "slope-formula",
            "<math><mi>m</mi><mo>=</mo><mfrac><mrow><msub><mi>y</mi><mn>2</mn></msub><mo>−</mo><msub><mi>y</mi><mn>1</mn></msub></mrow><mrow><msub><mi>x</mi><mn>2</mn></msub><mo>−</mo><msub><mi>x</mi><mn>1</mn></msub></mrow></mfrac></math>",
            "m 等于; 分数, x 下标 2, 减 x 下标 1; 分之, y 下标 2, 减 y 下标 1; 结束分数",
            "m 等于; 分数，分子为; y 下标 2, 减 y 下标 1; 分母为 x 下标 2, 减 x 下标 1",
        ),
        (
            "law-of-cosines",
            "<math><msup><mi>c</mi><mn>2</mn></msup><mo>=</mo><msup><mi>a</mi><mn>2</mn></msup><mo>+</mo><msup><mi>b</mi><mn>2</mn></msup><mo>−</mo><mn>2</mn><mi>a</mi><mi>b</mi><mi>cos</mi><mo>⁡</mo><mi>C</mi></math>",
            "c 平方 等于; a 平方 加 b 平方 减, 2 a b 余弦 大写 c",
            "c 平方 等于; a 平方 加 b 平方 减, 2 a b 余弦 大写 c",
        ),
        (
            "triangle-area",
            "<math><mi>S</mi><mo>=</mo><mfrac><mn>1</mn><mn>2</mn></mfrac><mi>a</mi><mi>h</mi></math>",
            "大写 s 等于, 2 分之 1 a h",
            "大写 s 等于, 2 分之 1 a h",
        ),
        (
            "inverse-matrix-identity",
            "<math><msup><mi>A</mi><mrow><mo>−</mo><mn>1</mn></mrow></msup><mi>A</mi><mo>=</mo><mi>I</mi></math>",
            "大写 a 的 负 1 次方, 大写 a; 等于 大写 i",
            "大写 a 的 负 1 次方, 大写 a; 等于 大写 i",
        ),
        (
            "trace-function-notation",
            "<math><mi>tr</mi><mo>⁡</mo><mrow><mo>(</mo><mi>A</mi><mo>)</mo></mrow></math>",
            "大写 a 的迹",
            "大写 a 的迹",
        ),
        (
            "explicit-vector",
            "<math><mrow intent='vector($v)'><mover><mi arg='v'>v</mi><mo>→</mo></mover></mrow></math>",
            "向量 v",
            "向量 v",
        ),
        (
            "one-by-one-matrix",
            "<math><mrow><mo>[</mo><mtable><mtr><mtd><mi>a</mi></mtd></mtr></mtable><mo>]</mo></mrow></math>",
            "1 乘 1 矩阵 元素为 a",
            "1 乘 1 矩阵 元素为 a",
        ),
        (
            "three-by-three-matrix",
            "<math><mrow><mo>[</mo><mtable><mtr><mtd><mi>a</mi></mtd><mtd><mi>b</mi></mtd><mtd><mi>c</mi></mtd></mtr><mtr><mtd><mi>d</mi></mtd><mtd><mi>e</mi></mtd><mtd><mi>f</mi></mtd></mtr><mtr><mtd><mi>g</mi></mtd><mtd><mi>h</mi></mtd><mtd><mi>i</mi></mtd></mtr></mtable><mo>]</mo></mrow></math>",
            "3 乘 3 矩阵; 第 1 行; a, b, c; 第 2 行; d, e, f; 第 3 行; g, h, i",
            "3 乘 3 矩阵; 第 1 行; a, b, c; 第 2 行; d, e, f; 第 3 行; g, h, i",
        ),
        (
            "matrix-row-and-column-separators",
            "<math><mrow><mo>[</mo><mtable rowlines='solid' columnlines='solid'><mtr><mtd><mn>1</mn></mtd><mtd><mn>2</mn></mtd><mtd><mn>3</mn></mtd></mtr><mtr><mtd><mn>4</mn></mtd><mtd><mn>5</mn></mtd><mtd><mn>6</mn></mtd></mtr></mtable><mo>]</mo></mrow></math>",
            "2 乘 3 增广矩阵; 第 1 行; 1, 列分隔线, 2, 列分隔线, 3, 行分隔线; 第 2 行; 4, 列分隔线, 5, 列分隔线, 6",
            "2 乘 3 增广矩阵; 第 1 行; 1, 列分隔线, 2, 列分隔线, 3, 行分隔线; 第 2 行; 4, 列分隔线, 5, 列分隔线, 6",
        ),
        (
            "labeled-matrix",
            "<math><mrow><mo>[</mo><mtable><mlabeledtr><mtd><mi>A</mi></mtd><mtd><mi>a</mi></mtd><mtd><mi>b</mi></mtd></mlabeledtr><mtr><mtd><mi>B</mi></mtd><mtd><mi>c</mi></mtd><mtd><mi>d</mi></mtd></mtr></mtable><mo>]</mo></mrow></math>",
            "2 乘 3 矩阵; 第 1 行 带有标签 大写 a; a, b; 第 2 行; 大写 b, c, d",
            "2 乘 3 矩阵; 第 1 行 标签为 大写 a; a, b; 第 2 行; 大写 b, c, d",
        ),
        (
            "non-simple-binomial-coefficient",
            "<math><mrow><mo>(</mo><mfrac linethickness='0em'><mrow><mi>n</mi><mo>+</mo><mn>1</mn></mrow><mrow><mi>k</mi><mo>−</mo><mn>1</mn></mrow></mfrac><mo>)</mo></mrow></math>",
            "二项式系数 n 加 1 取 k 减 1 结束二项式系数",
            "二项式系数 n 加 1 取 k 减 1 结束二项式系数",
        ),
        (
            "nested-power",
            "<math><msup><mrow><mo>(</mo><msup><mi>x</mi><mn>2</mn></msup><mo>)</mo></mrow><mn>3</mn></msup></math>",
            "左括号 x 平方 右括号 立方",
            "左括号 x 平方 右括号 立方",
        ),
        (
            "decimal-power",
            "<math><msup><mi>x</mi><mn>2.5</mn></msup></math>",
            "x 的 2.5 次方",
            "x 的 2.5 次方",
        ),
        (
            "zero-power",
            "<math><msup><mi>x</mi><mn>0</mn></msup></math>",
            "x 的 0 次方",
            "x 的 0 次方",
        ),
        (
            "fraction-under-radical",
            "<math><msqrt><mfrac><mn>1</mn><mn>2</mn></mfrac></msqrt></math>",
            "根号 2 分之 1 结束根号",
            "根号 2 分之 1",
        ),
    ];

    for (name, mathml, simple, clear) in cases {
        test("zh-cn", "SimpleSpeak", mathml, simple)
            .map_err(|error| anyhow::anyhow!("SimpleSpeak/{name}: {error}"))?;
        test("zh-cn", "ClearSpeak", mathml, clear)
            .map_err(|error| anyhow::anyhow!("ClearSpeak/{name}: {error}"))?;
    }
    Ok(())
}
