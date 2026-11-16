//! Representative formula and domain-specific speech regressions.

use crate::common::*;
use anyhow::Result;

#[test]
fn nested_numbers_powers_and_functions_have_stable_readings() -> Result<()> {
    // Cover structures whose meaning depends on nesting or adjacent notation.
    let cases = [
        (
            "nested-fraction",
            "<math><mfrac><mn>1</mn><mfrac><mn>1</mn><mi>x</mi></mfrac></mfrac></math>",
            "分数, 分数, x 分之 1, 结束分数; 分之 1, 结束分数",
        ),
        (
            "mixed-number",
            "<math><mn>2</mn><mo>&#x2064;</mo><mfrac><mn>1</mn><mn>3</mn></mfrac></math>",
            "2 又 3 分之 1",
        ),
        (
            "decimal-fraction",
            "<math><mfrac><mn>0.25</mn><mn>0.5</mn></mfrac></math>",
            "0.5 分之 0.25",
        ),
        (
            "negative-exponent",
            "<math><msup><mi>x</mi><mrow><mo>&#x2212;</mo><mn>2</mn></mrow></msup></math>",
            "x 的 负 2 次方",
        ),
        (
            "natural-logarithm",
            "<math><mi>ln</mi><mo>&#x2061;</mo><mi>x</mi></math>",
            "自然对数 x",
        ),
        (
            "exponential-function",
            "<math><msup><mi>e</mi><mi>x</mi></msup></math>",
            "e 的 x 次方",
        ),
        (
            "factorial",
            "<math><mn>5</mn><mo>!</mo></math>",
            "5 阶乘",
        ),
        (
            "double-factorial",
            "<math><mn>5</mn><mo>&#x203c;</mo></math>",
            "5 双阶乘",
        ),
    ];

    for (name, mathml, expected) in cases {
        test("zh", "SimpleSpeak", mathml, expected)
            .map_err(|error| anyhow::anyhow!("{name}: {error}"))?;
    }
    Ok(())
}

#[test]
fn common_notation_formulas_keep_contextual_readings() -> Result<()> {
    // Exercise complete formulas in both Chinese speech styles, not isolated symbols only.
    let cases = [
        (
            "linear-equation",
            "<math><mn>2</mn><mi>x</mi><mo>+</mo><mn>3</mn><mo>=</mo><mn>7</mn></math>",
            "2 x 加 3, 等于 7",
            "2 x 加 3, 等于 7",
        ),
        (
            "quadratic-equation",
            "<math><mi>a</mi><msup><mi>x</mi><mn>2</mn></msup><mo>+</mo><mi>b</mi><mi>x</mi><mo>+</mo><mi>c</mi><mo>=</mo><mn>0</mn></math>",
            "a x 平方, 加 b x 加 c; 等于 0",
            "a x 平方, 加 b x 加 c; 等于 0",
        ),
        (
            "quadratic-formula",
            "<math><mi>x</mi><mo>=</mo><mfrac><mrow><mo>−</mo><mi>b</mi><mo>±</mo><msqrt><msup><mi>b</mi><mn>2</mn></msup><mo>−</mo><mn>4</mn><mi>a</mi><mi>c</mi></msqrt></mrow><mrow><mn>2</mn><mi>a</mi></mrow></mfrac></math>",
            "x 等于; 分数, 2 a, 分之, 负 b 加或减; 根号 b 平方 减 4 a c 结束根号; 结束分数",
            "x 等于; 分数，分子为; 根号 b 平方 减 4 a c; 分母为 2 a",
        ),
        (
            "inequality-chain",
            "<math><mrow><mo>−</mo><mn>1</mn><mo>&lt;</mo><mi>x</mi><mo>≤</mo><mn>3</mn></mrow></math>",
            "负 1 小于 x 小于等于 3",
            "负 1 小于 x 小于等于 3",
        ),
        (
            "absolute-inequality",
            "<math><mrow><mo>|</mo><mi>x</mi><mo>−</mo><mn>2</mn><mo>|</mo><mo>&lt;</mo><mn>5</mn></mrow></math>",
            "x 减 2 的绝对值, 小于 5",
            "x 减 2 的绝对值; 小于 5",
        ),
        (
            "rational-expression",
            "<math><mfrac><mrow><mi>x</mi><mo>+</mo><mn>1</mn></mrow><mrow><mi>x</mi><mo>−</mo><mn>1</mn></mrow></mfrac></math>",
            "分数, x 减 1, 分之, x 加 1, 结束分数",
            "分数，分子为; x 加 1; 分母为 x 减 1",
        ),
        (
            "exponent-law",
            "<math><msup><mi>a</mi><mi>m</mi></msup><mo>×</mo><msup><mi>a</mi><mi>n</mi></msup><mo>=</mo><msup><mi>a</mi><mrow><mi>m</mi><mo>+</mo><mi>n</mi></mrow></msup></math>",
            "a 的 m 次方 乘 a 的 n 次方; 等于 a 的 m 加 n 次方",
            "a 的 m 次方 乘 a 的 n 次方; 等于 a 的 m 加 n 次方",
        ),
        (
            "cube-root",
            "<math><mroot><mrow><mi>x</mi><mo>+</mo><mn>1</mn></mrow><mn>3</mn></mroot></math>",
            "x 加 1 的 立方根, 结束根号",
            "x 加 1 的 立方根",
        ),
        (
            "double-angle",
            "<math><mi>sin</mi><mo>⁡</mo><mrow><mo>(</mo><mn>2</mn><mi>x</mi><mo>)</mo></mrow><mo>=</mo><mn>2</mn><mi>sin</mi><mo>⁡</mo><mi>x</mi><mi>cos</mi><mo>⁡</mo><mi>x</mi></math>",
            "正弦 2 x, 等于, 2 正弦 x 余弦 x",
            "正弦 2 x, 等于, 2 正弦 x 余弦 x",
        ),
        (
            "binomial-coefficient",
            "<math><mrow><mo>(</mo><mfrac linethickness='0em'><mi>n</mi><mi>k</mi></mfrac><mo>)</mo></mrow></math>",
            "n 取 k",
            "n 取 k",
        ),
        (
            "sequence-term",
            "<math><msub><mi>a</mi><mi>n</mi></msub><mo>=</mo><msub><mi>a</mi><mn>1</mn></msub><mo>+</mo><mrow><mo>(</mo><mi>n</mi><mo>−</mo><mn>1</mn><mo>)</mo></mrow><mi>d</mi></math>",
            "a 下标 n, 等于; a 下标 1, 加, 左括号 n 减 1 右括号; 乘 d",
            "a 下标 n, 等于; a 下标 1, 加, 左括号 n 减 1 右括号; 乘 d",
        ),
        (
            "arithmetic-series",
            "<math><msub><mi>S</mi><mi>n</mi></msub><mo>=</mo><mfrac><mrow><mi>n</mi><mo>(</mo><msub><mi>a</mi><mn>1</mn></msub><mo>+</mo><msub><mi>a</mi><mi>n</mi></msub><mo>)</mo></mrow><mn>2</mn></mfrac></math>",
            "大写 s 下标 n; 等于; 分数, 2 分之, n 乘; 左括号, a 下标 1, 加 a 下标 n; 右括号, 结束分数",
            "大写 s 下标 n; 等于; 分数，分子为; n 乘; 左括号, a 下标 1, 加 a 下标 n; 右括号; 分母为 2",
        ),
        (
            "geometric-sum",
            "<math><munderover><mo>∑</mo><mrow><mi>k</mi><mo>=</mo><mn>0</mn></mrow><mi>n</mi></munderover><msup><mi>r</mi><mi>k</mi></msup></math>",
            "求和 从 k 等于 0 到 n, r 的 k 次方",
            "求和 从 k 等于 0 到 n, r 的 k 次方",
        ),
        (
            "definite-integral",
            "<math><msubsup><mo>∫</mo><mn>0</mn><mn>1</mn></msubsup><msup><mi>x</mi><mn>2</mn></msup><mi>d</mi><mi>x</mi></math>",
            "积分 从 0 到 1, x 平方 d x",
            "积分 从 0 到 1, x 平方 d x",
        ),
        (
            "derivative-equation",
            "<math><mfrac><mrow><mo>d</mo><mi>y</mi></mrow><mrow><mo>d</mo><mi>x</mi></mrow></mfrac><mo>=</mo><mn>3</mn><msup><mi>x</mi><mn>2</mn></msup></math>",
            "分数, d x, 分之, d y, 结束分数; 等于 3 x 平方",
            "分数，分子为; d y; 分母为 d x; 等于 3 x 平方",
        ),
        (
            "limit-sine",
            "<math><munder><mo>lim</mo><mrow><mi>x</mi><mo>→</mo><mn>0</mn></mrow></munder><mfrac><mrow><mi>sin</mi><mo>⁡</mo><mi>x</mi></mrow><mi>x</mi></mfrac></math>",
            "极限，当 x 趋于 0; 分数, x 分之, 正弦 x, 结束分数",
            "极限，当 x 趋于 0; x 分之 正弦 x",
        ),
        (
            "differential-equation",
            "<math><mfrac><mrow><mo>d</mo><mi>y</mi></mrow><mrow><mo>d</mo><mi>x</mi></mrow></mfrac><mo>=</mo><mi>k</mi><mi>y</mi></math>",
            "分数, d x, 分之, d y, 结束分数; 等于 k y",
            "分数，分子为; d y; 分母为 d x; 等于 k y",
        ),
        (
            "matrix-two-by-two",
            "<math><mrow><mo>[</mo><mtable><mtr><mtd><mi>a</mi></mtd><mtd><mi>b</mi></mtd></mtr><mtr><mtd><mi>c</mi></mtd><mtd><mi>d</mi></mtd></mtr></mtable><mo>]</mo></mrow></math>",
            "2 乘 2 矩阵; 第 1 行; a, b; 第 2 行; c, d",
            "2 乘 2 矩阵; 第 1 行; a, b; 第 2 行; c, d",
        ),
        (
            "determinant-formula",
            "<math><mrow><mo>|</mo><mtable><mtr><mtd><mi>a</mi></mtd><mtd><mi>b</mi></mtd></mtr><mtr><mtd><mi>c</mi></mtd><mtd><mi>d</mi></mtd></mtr></mtable><mo>|</mo></mrow><mo>=</mo><mi>a</mi><mi>d</mi><mo>−</mo><mi>b</mi><mi>c</mi></math>",
            "2 乘 2 行列式; 第 1 行; a, b; 第 2 行; c, d; 等于, a d 减 b c",
            "2 乘 2 行列式; 第 1 行; a, b; 第 2 行; c, d; 等于, a d 减 b c",
        ),
        (
            "transpose-notation",
            "<math><msup><mi>A</mi><mi>T</mi></msup></math>",
            "大写 a 的转置",
            "大写 a 的转置",
        ),
        (
            "eigen-equation",
            "<math><mi>A</mi><mi>v</mi><mo>=</mo><mi>λ</mi><mi>v</mi></math>",
            "大写 a v, 等于 lambda v",
            "大写 a v, 等于 lambda v",
        ),
        (
            "distance-formula",
            "<math><mi>d</mi><mo>=</mo><msqrt><msup><mrow><mo>(</mo><msub><mi>x</mi><mn>2</mn></msub><mo>−</mo><msub><mi>x</mi><mn>1</mn></msub><mo>)</mo></mrow><mn>2</mn></msup><mo>+</mo><msup><mrow><mo>(</mo><msub><mi>y</mi><mn>2</mn></msub><mo>−</mo><msub><mi>y</mi><mn>1</mn></msub><mo>)</mo></mrow><mn>2</mn></msup></msqrt></math>",
            "d 等于; 根号 左括号, x 下标 2, 减 x 下标 1; 右括号 平方; 加; 左括号, y 下标 2, 减 y 下标 1; 右括号 平方 结束根号",
            "d 等于; 根号 左括号, x 下标 2, 减 x 下标 1; 右括号 平方; 加; 左括号, y 下标 2, 减 y 下标 1; 右括号 平方",
        ),
        (
            "circle-equation",
            "<math><msup><mrow><mo>(</mo><mi>x</mi><mo>−</mo><mi>h</mi><mo>)</mo></mrow><mn>2</mn></msup><mo>+</mo><msup><mrow><mo>(</mo><mi>y</mi><mo>−</mo><mi>k</mi><mo>)</mo></mrow><mn>2</mn></msup><mo>=</mo><msup><mi>r</mi><mn>2</mn></msup></math>",
            "左括号 x 减 h 右括号 平方; 加, 左括号 y 减 k 右括号 平方; 等于 r 平方",
            "左括号 x 减 h 右括号 平方; 加, 左括号 y 减 k 右括号 平方; 等于 r 平方",
        ),
        (
            "pythagorean-theorem",
            "<math><msup><mi>a</mi><mn>2</mn></msup><mo>+</mo><msup><mi>b</mi><mn>2</mn></msup><mo>=</mo><msup><mi>c</mi><mn>2</mn></msup></math>",
            "a 平方 加 b 平方, 等于 c 平方",
            "a 平方 加 b 平方, 等于 c 平方",
        ),
        (
            "right-angle",
            "<math><mo>∠</mo><mi>A</mi><mi>B</mi><mi>C</mi><mo>=</mo><mn>90</mn><mo>°</mo></math>",
            "角, 大写 a 大写 b 大写 c; 等于 90 度",
            "角, 大写 a 大写 b 大写 c; 等于 90 度",
        ),
        (
            "set-union-intersection",
            "<math><mi>A</mi><mo>∪</mo><mi>B</mi><mo>=</mo><mi>A</mi><mo>∩</mo><mi>B</mi></math>",
            "大写 a 并 大写 b, 等于, 大写 a 交 大写 b",
            "大写 a 并 大写 b, 等于, 大写 a 交 大写 b",
        ),
        (
            "subset-chain",
            "<math><mi>A</mi><mo>⊂</mo><mi>B</mi><mo>⊆</mo><mi>C</mi></math>",
            "大写 a 子集, 大写 b 子集或等于 大写 c",
            "大写 a 子集, 大写 b 子集或等于 大写 c",
        ),
        (
            "interval",
            "<math><mrow><mo>[</mo><mn>0</mn><mo>,</mo><mn>1</mn><mo>)</mo></mrow></math>",
            "左闭右开区间 0 到 1",
            "从 0 到 1 的左闭右开区间",
        ),
        (
            "universal-inequality",
            "<math><mo>∀</mo><mi>x</mi><mo>∈</mo><mi>ℝ</mi><mo>,</mo><msup><mi>x</mi><mn>2</mn></msup><mo>≥</mo><mn>0</mn></math>",
            "任意 x 属于 实数集; 逗号; x 平方 大于等于 0",
            "任意 x 属于 实数集; 逗号; x 平方 大于等于 0",
        ),
        (
            "complex-number",
            "<math><mi>z</mi><mo>=</mo><mn>3</mn><mo>+</mo><mn>4</mn><mi>i</mi></math>",
            "z 等于 3 加 4 i",
            "z 等于 3 加 4 i",
        ),
        (
            "euler-identity",
            "<math><msup><mi>e</mi><mrow><mi>i</mi><mi>π</mi></mrow></msup><mo>+</mo><mn>1</mn><mo>=</mo><mn>0</mn></math>",
            "e 的 i pi 次方, 加 1; 等于 0",
            "e 的 i pi 次方, 加 1; 等于 0",
        ),
        (
            "speed-unit",
            "<math><mfrac><mrow><mn>72</mn><mi intent=':unit'>km</mi></mrow><mi intent=':unit'>h</mi></mfrac></math>",
            "72 千米每 小时",
            "72 千米每 小时",
        ),
        (
            "acceleration-unit",
            "<math><mfrac><mrow><mn>9.8</mn><mi intent=':unit'>m</mi></mrow><msup><mi intent=':unit'>s</mi><mn>2</mn></msup></mfrac></math>",
            "9.8 米每 平方秒",
            "9.8 米每 平方秒",
        ),
        (
            "temperature-unit",
            "<math><mn>20</mn><mi intent=':unit'>°C</mi></math>",
            "20 摄氏度",
            "20 摄氏度",
        ),
        (
            "chemical-water",
            "<math><mrow data-chem-formula='3'><msub><mi mathvariant='normal' data-chem-element='1'>H</mi><mn>2</mn></msub><mi mathvariant='normal' data-chem-element='1'>O</mi></mrow></math>",
            "大写 h, 下标 2, 大写 o",
            "大写 h, 下标 2, 大写 o",
        ),
        (
            "scientific-notation",
            "<math><mn>6.02</mn><mo>×</mo><msup><mn>10</mn><mn>23</mn></msup></math>",
            "6.02 乘 10 的 23 次方",
            "6.02 乘 10 的 23 次方",
        ),
    ];

    for (name, mathml, simple, clear) in cases {
        test("zh", "SimpleSpeak", mathml, simple)
            .map_err(|error| anyhow::anyhow!("SimpleSpeak/{name}: {error}"))?;
        test("zh", "ClearSpeak", mathml, clear)
            .map_err(|error| anyhow::anyhow!("ClearSpeak/{name}: {error}"))?;
    }
    Ok(())
}

#[test]
fn named_intents_use_verified_mathematical_terms() -> Result<()> {
    // Explicit intents provide deterministic checks for terms that presentation markup can hide.
    let cases = [
        (
            "conditional-probability",
            "<math><mrow intent='conditional-probability($A,$B)'><mi arg='A'>A</mi><mi arg='B'>B</mi></mrow></math>",
            "在 大写 b 条件下 大写 a 的概率",
        ),
        (
            "gcd",
            "<math><mrow intent='greatest-common-divisor($a,$b)'><mi arg='a'>12</mi><mi arg='b'>18</mi></mrow></math>",
            "12 与 18 的最大公约数",
        ),
        (
            "lcm",
            "<math><mrow intent='least-common-multiple($a,$b)'><mi arg='a'>4</mi><mi arg='b'>6</mi></mrow></math>",
            "4 与 6 的最小公倍数",
        ),
        (
            "quotient",
            "<math><mrow intent='quotient($a,$b)'><mi arg='a'>7</mi><mi arg='b'>3</mi></mrow></math>",
            "7 除以 3 的商",
        ),
        (
            "remainder",
            "<math><mrow intent='remainder($a,$b)'><mi arg='a'>7</mi><mi arg='b'>3</mi></mrow></math>",
            "7 除以 3 的余数",
        ),
        (
            "mean",
            "<math><mrow intent='mean($x)'><mi arg='x'>x</mi></mrow></math>",
            "x 的平均值",
        ),
        (
            "variance",
            "<math><mrow intent='variance($x)'><mi arg='x'>x</mi></mrow></math>",
            "x 的方差",
        ),
        (
            "complex-conjugate",
            "<math><mrow intent='complex-conjugate($z)'><mi arg='z'>z</mi></mrow></math>",
            "z 的共轭复数",
        ),
        (
            "real-part",
            "<math><mrow intent='real-part($z)'><mi arg='z'>z</mi></mrow></math>",
            "z 的实部",
        ),
        (
            "dot-product",
            "<math><mrow intent='dot-product($a,$b)'><mi arg='a'>a</mi><mi arg='b'>b</mi></mrow></math>",
            "a 与 b 的数量积",
        ),
        (
            "cross-product",
            "<math><mrow intent='cross-product($a,$b)'><mi arg='a'>a</mi><mi arg='b'>b</mi></mrow></math>",
            "a 与 b 的叉积",
        ),
        (
            "line-segment",
            "<math><mrow intent='line-segment($A,$B)'><mi arg='A'>A</mi><mi arg='B'>B</mi></mrow></math>",
            "线段 大写 a 大写 b",
        ),
        (
            "cartesian-coordinate",
            "<math><mrow intent='cartesian-coordinate($x,$y)'><mi arg='x'>x</mi><mi arg='y'>y</mi></mrow></math>",
            "直角坐标 x 逗号, y",
        ),
        (
            "trace",
            "<math><mrow intent='trace($A)'><mi arg='A'>A</mi></mrow></math>",
            "大写 a 的迹",
        ),
        (
            "kernel",
            "<math><mrow intent='kernel($T)'><mi arg='T'>T</mi></mrow></math>",
            "大写 t 的核",
        ),
        (
            "arcsine",
            "<math><mrow intent='arcsine($x)'><mi arg='x'>x</mi></mrow></math>",
            "反正弦 x",
        ),
        (
            "hyperbolic-sine",
            "<math><mi>sinh</mi><mo>⁡</mo><mi>x</mi></math>",
            "双曲正弦 x",
        ),
    ];

    for (name, mathml, expected) in cases {
        for style in ["SimpleSpeak", "ClearSpeak"] {
            test("zh", style, mathml, expected)
                .map_err(|error| anyhow::anyhow!("{style}/{name}: {error}"))?;
        }
    }
    Ok(())
}

#[test]
fn semantic_function_cases_use_verified_terms() -> Result<()> {
    // Explicit intents make the intended Chinese mathematical term unambiguous.
    let cases = [
        (
            "arctangent",
            "<math><mrow intent='arctangent($x)'><mi arg='x'>x</mi></mrow></math>",
            "反正切 x",
        ),
        (
            "arcsecant",
            "<math><mrow intent='arcsecant($x)'><mi arg='x'>x</mi></mrow></math>",
            "反正割 x",
        ),
        (
            "hyperbolic-cosine",
            "<math><mrow intent='hyperbolic-cosine($x)'><mi arg='x'>x</mi></mrow></math>",
            "双曲余弦 x",
        ),
        (
            "arc-hyperbolic-sine",
            "<math><mrow intent='arc-hyperbolic-sine($x)'><mi arg='x'>x</mi></mrow></math>",
            "反双曲正弦 x",
        ),
        (
            "tuple",
            "<math><mrow intent='tuple($x,$y)'><mi arg='x'>x</mi><mi arg='y'>y</mi></mrow></math>",
            "元组 x 逗号, y",
        ),
        (
            "defined-as",
            "<math><mrow intent='defined-as($a,$b)'><mi arg='a'>f</mi><mi arg='b'>g</mi></mrow></math>",
            "f 定义为 g",
        ),
        (
            "equivalent-to",
            "<math><mrow intent='equivalent-to($a,$b)'><mi arg='a'>a</mi><mi arg='b'>b</mi></mrow></math>",
            "a 等价于 b",
        ),
        (
            "similar",
            "<math><mrow intent='similar($a,$b)'><mi arg='a'>A</mi><mi arg='b'>B</mi></mrow></math>",
            "大写 a 相似于 大写 b",
        ),
        (
            "ratio",
            "<math><mrow intent='ratio($a,$b)'><mi arg='a'>a</mi><mi arg='b'>b</mi></mrow></math>",
            "a 比 b",
        ),
        (
            "superset",
            "<math><mrow intent='superset($a,$b)'><mi arg='a'>A</mi><mi arg='b'>B</mi></mrow></math>",
            "大写 a 超集 大写 b",
        ),
        (
            "not-superset",
            "<math><mrow intent='not-superset($a,$b)'><mi arg='a'>A</mi><mi arg='b'>B</mi></mrow></math>",
            "大写 a 非超集 大写 b",
        ),
    ];

    for (name, mathml, expected) in cases {
        for style in ["SimpleSpeak", "ClearSpeak"] {
            test("zh", style, mathml, expected)
                .map_err(|error| anyhow::anyhow!("{style}/{name}: {error}"))?;
        }
    }
    Ok(())
}
