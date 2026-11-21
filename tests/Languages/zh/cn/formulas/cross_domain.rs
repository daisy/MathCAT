//! Representative formula and domain-specific speech regressions.

use crate::common::*;
use anyhow::Result;

#[test]
fn real_world_formulas_use_natural_chinese_order() -> Result<()> {
    let cases = [
        (
            "trig-simple",
            "SimpleSpeak",
            "Medium",
            "<math><msup><mi>sin</mi><mn>2</mn></msup><mi>x</mi><mo>+</mo><msup><mi>cos</mi><mn>2</mn></msup><mi>x</mi><mo>=</mo><mn>1</mn></math>",
            "正弦 平方 x, 加 余弦 平方 x; 等于 1",
        ),
        (
            "trig-clear",
            "ClearSpeak",
            "Medium",
            "<math><msup><mi>sin</mi><mn>2</mn></msup><mi>x</mi><mo>+</mo><msup><mi>cos</mi><mn>2</mn></msup><mi>x</mi><mo>=</mo><mn>1</mn></math>",
            "正弦 平方 x, 加 余弦 平方 x; 等于 1",
        ),
        (
            "log-base-simple",
            "SimpleSpeak",
            "Medium",
            "<math><msub><mi>log</mi><mn>2</mn></msub><mn>8</mn><mo>=</mo><mn>3</mn></math>",
            "以 2 为底, 8 的对数, 等于 3",
        ),
        (
            "log-base-clear",
            "ClearSpeak",
            "Medium",
            "<math><msub><mi>log</mi><mn>2</mn></msub><mn>8</mn><mo>=</mo><mn>3</mn></math>",
            "以 2 为底, 8 的对数, 等于 3",
        ),
        (
            "limit-simple",
            "SimpleSpeak",
            "Medium",
            "<math><munder><mo>lim</mo><mrow><mi>x</mi><mo>&#x2192;</mo><mn>0</mn></mrow></munder><mfrac><mrow><mi>sin</mi><mo>&#x2061;</mo><mi>x</mi></mrow><mi>x</mi></mfrac><mo>=</mo><mn>1</mn></math>",
            "极限，当 x 趋于 0; 分数, x 分之, 正弦 x, 结束分数; 等于 1",
        ),
        (
            "partial-derivative",
            "ClearSpeak",
            "Medium",
            "<math><mrow intent='partial-derivative($x)'><mi arg='x'>f</mi></mrow></math>",
            "f 的偏导数",
        ),
        (
            "sum",
            "SimpleSpeak",
            "Medium",
            "<math><munderover><mo>&#x2211;</mo><mrow><mi>k</mi><mo>=</mo><mn>1</mn></mrow><mi>n</mi></munderover><msup><mi>k</mi><mn>2</mn></msup></math>",
            "求和 从 k 等于 1 到 n, k 平方",
        ),
        (
            "product",
            "SimpleSpeak",
            "Medium",
            "<math><munderover><mo>&#x220f;</mo><mrow><mi>k</mi><mo>=</mo><mn>1</mn></mrow><mi>n</mi></munderover><mi>k</mi></math>",
            "连乘 从 k 等于 1 到 n k",
        ),
        (
            "prime-derivative",
            "SimpleSpeak",
            "Medium",
            "<math><msup><mi>f</mi><mo>&#x2032;</mo></msup><mrow><mo>(</mo><mi>x</mi><mo>)</mo></mrow><mo>=</mo><mn>2</mn><mi>x</mi></math>",
            "f 撇号, x, 等于 2 x",
        ),
        (
            "dot-product",
            "SimpleSpeak",
            "Medium",
            "<math><mover><mi>v</mi><mo>&#x2192;</mo></mover><mo>&#x22c5;</mo><mover><mi>w</mi><mo>&#x2192;</mo></mover></math>",
            "向量 v, 点乘 向量 w",
        ),
        (
            "explicit-dot-product-intent",
            "ClearSpeak",
            "Medium",
            "<math><mrow intent='dot-product($a,$b)'><mi arg='a'>v</mi><mi arg='b'>w</mi></mrow></math>",
            "v 与 w 的数量积",
        ),
        (
            "scalar-centered-dot",
            "SimpleSpeak",
            "Medium",
            "<math><mi>v</mi><mo>&#x22c5;</mo><mi>w</mi></math>",
            "v 乘 w",
        ),
        (
            "cross-product",
            "SimpleSpeak",
            "Medium",
            "<math><mover><mi>v</mi><mo>&#x2192;</mo></mover><mo>&#xd7;</mo><mover><mi>w</mi><mo>&#x2192;</mo></mover></math>",
            "向量 v, 叉乘 向量 w",
        ),
        (
            "explicit-cross-product-intent",
            "ClearSpeak",
            "Medium",
            "<math><mrow intent='cross-product($a,$b)'><mi arg='a'>v</mi><mi arg='b'>w</mi></mrow></math>",
            "v 与 w 的叉积",
        ),
        (
            "plain-norm",
            "SimpleSpeak",
            "Medium",
            "<math><mrow><mo>&#x2225;</mo><mrow><mi>x</mi><mo>+</mo><mi>y</mi></mrow><mo>&#x2225;</mo></mrow></math>",
            "x 加 y 的范数",
        ),
        (
            "norm",
            "SimpleSpeak",
            "Medium",
            "<math><msub><mrow><mo>&#x2225;</mo><mi>x</mi><mo>&#x2225;</mo></mrow><mn>2</mn></msub></math>",
            "x 的 2 范数",
        ),
        (
            "determinant",
            "SimpleSpeak",
            "Medium",
            "<math><mrow><mo>|</mo><mtable><mtr><mtd><mi>a</mi></mtd><mtd><mi>b</mi></mtd></mtr><mtr><mtd><mi>c</mi></mtd><mtd><mi>d</mi></mtd></mtr></mtable><mo>|</mo></mrow></math>",
            "2 乘 2 行列式; 第 1 行; a, b; 第 2 行; c, d",
        ),
        (
            "mean",
            "ClearSpeak",
            "Medium",
            "<math><mrow intent='mean($x)'><mi arg='x'>x</mi></mrow></math>",
            "x 的平均值",
        ),
        (
            "standard-deviation",
            "ClearSpeak",
            "Medium",
            "<math><mrow intent='standard-deviation($x)'><mi arg='x'>x</mi></mrow></math>",
            "x 的标准差",
        ),
        (
            "variance",
            "ClearSpeak",
            "Medium",
            "<math><mrow intent='variance($x)'><mi arg='x'>x</mi></mrow></math>",
            "x 的方差",
        ),
        (
            "median",
            "ClearSpeak",
            "Medium",
            "<math><mrow intent='median($x)'><mi arg='x'>x</mi></mrow></math>",
            "x 的中位数",
        ),
        (
            "mode",
            "ClearSpeak",
            "Medium",
            "<math><mrow intent='mode($x)'><mi arg='x'>x</mi></mrow></math>",
            "x 的众数",
        ),
        (
            "water-terse",
            "SimpleSpeak",
            "Terse",
            "<math><msub><mi>H</mi><mn>2</mn></msub><mi>O</mi></math>",
            "大写 h, 2 大写 o",
        ),
        (
            "water-medium",
            "SimpleSpeak",
            "Medium",
            "<math><msub><mi>H</mi><mn>2</mn></msub><mi>O</mi></math>",
            "大写 h, 下标 2, 大写 o",
        ),
        (
            "water-verbose",
            "SimpleSpeak",
            "Verbose",
            "<math><msub><mi>H</mi><mn>2</mn></msub><mi>O</mi></math>",
            "大写 h, 下标 2, 大写 o",
        ),
        (
            "sulfate-medium",
            "SimpleSpeak",
            "Medium",
            "<math><msup><mrow><mo>[</mo><mi>S</mi><msub><mi>O</mi><mn>4</mn></msub><mo>]</mo></mrow><mrow><mn>2</mn><mo>&#x2212;</mo></mrow></msup></math>",
            "左方括号, 大写 s, 大写 o, 下标 4; 右方括号 上标 2 负",
        ),
        (
            "aqueous-terse",
            "SimpleSpeak",
            "Terse",
            "<math><mi>Fe</mi><msub><mi>Cl</mi><mn>3</mn></msub><mrow><mo>(</mo><mi>aq</mi><mo>)</mo></mrow></math>",
            "大写 f e, 大写 c l, 3 水溶液",
        ),
        (
            "acceleration-unit",
            "SimpleSpeak",
            "Medium",
            "<math><mfrac><mrow><mn>3</mn><mi intent=':unit'>m</mi></mrow><msup><mi intent=':unit'>s</mi><mn>2</mn></msup></mfrac></math>",
            "3 米每 平方秒",
        ),
        (
            "piecewise",
            "SimpleSpeak",
            "Medium",
            "<math><mi>f</mi><mrow><mo>(</mo><mi>x</mi><mo>)</mo></mrow><mo>=</mo><mrow><mo>{</mo><mtable><mtr><mtd><msup><mi>x</mi><mn>2</mn></msup><mtext> 当 </mtext><mi>x</mi><mo>&#x2265;</mo><mn>0</mn></mtd></mtr><mtr><mtd><mo>&#x2212;</mo><mi>x</mi><mtext> 当 </mtext><mi>x</mi><mo>&lt;</mo><mn>0</mn></mtd></mtr></mtable></mrow></math>",
            "f x 等于; 2 个分支; 第 1 个分支; x 平方 当 x, 大于等于 0; 第 2 个分支; 负 x 当 x, 小于 0",
        ),
        (
            "system-of-equations",
            "SimpleSpeak",
            "Medium",
            "<math><mtable><mtr><mtd><mi>x</mi><mo>+</mo><mi>y</mi></mtd><mtd><mo>=</mo></mtd><mtd><mn>7</mn></mtd></mtr><mtr><mtd><mn>2</mn><mi>x</mi><mo>+</mo><mn>3</mn><mi>y</mi></mtd><mtd><mo>=</mo></mtd><mtd><mn>17</mn></mtd></mtr></mtable></math>",
            "2 个方程; 第 1 个方程; x 加 y 等于 7; 第 2 个方程; 2 x 加 3 y; 等于 17",
        ),
        (
            "quadratic-formula-simple",
            "SimpleSpeak",
            "Medium",
            "<math><mi>x</mi><mo>=</mo><mfrac><mrow><mo>&#x2212;</mo><mi>b</mi><mo>&#xb1;</mo><msqrt><mrow><msup><mi>b</mi><mn>2</mn></msup><mo>&#x2212;</mo><mn>4</mn><mi>a</mi><mi>c</mi></mrow></msqrt></mrow><mrow><mn>2</mn><mi>a</mi></mrow></mfrac></math>",
            "x 等于; 分数, 2 a, 分之, 负 b 加或减; 根号 b 平方 减 4 a c 结束根号; 结束分数",
        ),
        (
            "quadratic-formula-literal",
            "LiteralSpeak",
            "Medium",
            "<math><mi>x</mi><mo>=</mo><mfrac><mrow><mo>&#x2212;</mo><mi>b</mi><mo>&#xb1;</mo><msqrt><mrow><msup><mi>b</mi><mn>2</mn></msup><mo>&#x2212;</mo><mn>4</mn><mi>a</mi><mi>c</mi></mrow></msqrt></mrow><mrow><mn>2</mn><mi>a</mi></mrow></mfrac></math>",
            "x 等于; 分数, 2 a, 分之, 减 b 加或减; 根号 b 上标 2 结束上标, 减 4 a c, 结束根号; 结束分数",
        ),
        (
            "binomial",
            "SimpleSpeak",
            "Medium",
            "<math><mmultiscripts><mi>C</mi><mi>k</mi><none/><mprescripts/><mi>n</mi><none/></mmultiscripts></math>",
            "n 取 k",
        ),
    ];

    for (name, style, verbosity, mathml, expected) in cases {
        test_prefs(
            "zh-cn",
            style,
            vec![("Verbosity", verbosity)],
            mathml,
            expected,
        )
        .map_err(|error| anyhow::anyhow!("{name}: {error}"))?;
    }
    Ok(())
}

#[test]
fn probability_chemistry_and_algebra_keep_semantic_readings() -> Result<()> {
    // Cover named semantic branches and notation where operand order changes the meaning.
    let cases = [
        (
            "complement-event",
            "<math><mrow><mi>P</mi><mo>(</mo><mover><mi>A</mi><mo>¯</mo></mover><mo>)</mo><mo>=</mo><mn>1</mn><mo>−</mo><mi>P</mi><mo>(</mo><mi>A</mi><mo>)</mo></mrow></math>",
            "大写 a 上横线, 的概率, 等于, 1 减 大写 a 的概率",
            "大写 a 上横线, 的概率, 等于, 1 减 大写 a 的概率",
        ),
        (
            "ordered-pair",
            "<math><mrow intent='ordered-pair($x,$y)'><mi arg='x'>x</mi><mi arg='y'>y</mi></mrow></math>",
            "有序对 x 和 y",
            "有序对 x 和 y",
        ),
        (
            "cartesian-product",
            "<math><mrow intent='cartesian-product($x,$y)'><mi arg='x'>A</mi><mi arg='y'>B</mi></mrow></math>",
            "大写 a 笛卡尔积 大写 b",
            "大写 a 笛卡尔积 大写 b",
        ),
        (
            "direct-product",
            "<math><mrow intent='direct-product($x,$y)'><mi arg='x'>G</mi><mi arg='y'>H</mi></mrow></math>",
            "大写 g 直积 大写 h",
            "大写 g 直积 大写 h",
        ),
        (
            "inner-product",
            "<math><mrow intent='inner-product($x,$y)'><mi arg='x'>u</mi><mi arg='y'>v</mi></mrow></math>",
            "u 内积 v",
            "u 内积 v",
        ),
        (
            "outer-product",
            "<math><mrow intent='outer-product($x,$y)'><mi arg='x'>u</mi><mi arg='y'>v</mi></mrow></math>",
            "u 外积 v",
            "u 外积 v",
        ),
        (
            "rate",
            "<math><mrow intent='rate($x,$y)'><mi arg='x'>d</mi><mi arg='y'>t</mi></mrow></math>",
            "d 每 t",
            "d 每 t",
        ),
        (
            "constraint",
            "<math><mrow intent='constraint($x,$y)'><mi arg='x'>x</mi><mrow arg='y'><mi>x</mi><mo>&gt;</mo><mn>0</mn></mrow></mrow></math>",
            "x 条件为 x 大于 0",
            "x 条件为 x 大于 0",
        ),
        (
            "translation",
            "<math><mrow intent='translation($x,$y)'><mi arg='x'>f</mi><mi arg='y'>g</mi></mrow></math>",
            "平移 f 逗号, g",
            "平移 f 逗号, g",
        ),
        (
            "pochhammer-symbol",
            "<math><mrow intent='pochhammer($x,$y)'><mi arg='x'>x</mi><mi arg='y'>n</mi></mrow></math>",
            "升阶乘 x 逗号, n",
            "升阶乘 x 逗号, n",
        ),
        (
            "directed-line-segment",
            "<math><mrow intent='directed-line-segment($a,$b)'><mi arg='a'>A</mi><mi arg='b'>B</mi></mrow></math>",
            "有向线段 大写 a 大写 b",
            "有向线段 大写 a 大写 b",
        ),
        (
            "line",
            "<math><mrow intent='line($a,$b)'><mi arg='a'>A</mi><mi arg='b'>B</mi></mrow></math>",
            "直线 大写 a 大写 b",
            "直线 大写 a 大写 b",
        ),
        (
            "point",
            "<math><mrow intent='point($x,$y,$z)'><mi arg='x'>A</mi><mi arg='y'>B</mi><mi arg='z'>C</mi></mrow></math>",
            "点 大写 a 大写 b 大写 c",
            "点 大写 a 大写 b 大写 c",
        ),
        (
            "perpendicular",
            "<math><mrow intent='perpendicular($a,$b)'><mi arg='a'>l</mi><mi arg='b'>m</mi></mrow></math>",
            "l 垂直于 m",
            "l 垂直于 m",
        ),
        (
            "proportional",
            "<math><mrow intent='proportional($a,$b)'><mi arg='a'>y</mi><mi arg='b'>x</mi></mrow></math>",
            "y 正比于 x",
            "y 正比于 x",
        ),
        (
            "xor",
            "<math><mrow intent='xor($a,$b)'><mi arg='a'>p</mi><mi arg='b'>q</mi></mrow></math>",
            "p 异或 q",
            "p 异或 q",
        ),
        (
            "logical-not",
            "<math><mrow intent='not($x)'><mi arg='x'>p</mi></mrow></math>",
            "非 p",
            "非 p",
        ),
        (
            "there-does-not-exist",
            "<math><mrow intent='there-does-not-exist($x)'><mi arg='x'>x</mi></mrow></math>",
            "不存在 x",
            "不存在 x",
        ),
        (
            "evaluates-to",
            "<math><mrow intent='evaluates-to($x,$y)'><mi arg='x'>f</mi><mi arg='y'>3</mi></mrow></math>",
            "f 结果为 3",
            "f 结果为 3",
        ),
        (
            "approximately",
            "<math><mrow intent='approximately($a,$b)'><mi arg='a'>π</mi><mi arg='b'>3.14</mi></mrow></math>",
            "pi 约等于 3.14",
            "pi 约等于 3.14",
        ),
        (
            "identically-equals",
            "<math><mrow intent='identically-equals($a,$b)'><mi arg='a'>f</mi><mi arg='b'>g</mi></mrow></math>",
            "f 恒等于 g",
            "f 恒等于 g",
        ),
        (
            "golden-ratio",
            "<math><mi intent='golden-ratio'>φ</mi></math>",
            "黄金分割比",
            "黄金分割比",
        ),
        (
            "upper-limit",
            "<math><mrow intent='lim-sup($x)'><mi arg='x'>a</mi></mrow></math>",
            "上极限，当 a",
            "上极限，当 a",
        ),
        (
            "lower-limit",
            "<math><mrow intent='lim-inf($x)'><mi arg='x'>a</mi></mrow></math>",
            "下极限，当 a",
            "下极限，当 a",
        ),
        (
            "unit-vector",
            "<math><mrow intent='unit-vector($x)'><mi arg='x'>v</mi></mrow></math>",
            "单位向量 v",
            "单位向量 v",
        ),
        (
            "identity-matrix",
            "<math><mi intent='identity-matrix'>I</mi></math>",
            "单位矩阵",
            "单位矩阵",
        ),
        (
            "diagonal-matrix",
            "<math><mrow><mo>[</mo><mtable><mtr><mtd><mn>2</mn></mtd><mtd><mn>0</mn></mtd></mtr><mtr><mtd><mn>0</mn></mtd><mtd><mn>3</mn></mtd></mtr></mtable><mo>]</mo></mrow></math>",
            "2 乘 2 对角矩阵; 第 1 列; 2; 第 2 列; 3",
            "2 乘 2 对角矩阵; 第 1 列; 2; 第 2 列; 3",
        ),
        (
            "matrix-row-separator",
            "<math><mrow><mo>[</mo><mtable rowlines='solid'><mtr><mtd><mn>1</mn></mtd><mtd><mn>2</mn></mtd></mtr><mtr><mtd><mn>3</mn></mtd><mtd><mn>4</mn></mtd></mtr></mtable><mo>]</mo></mrow></math>",
            "2 乘 2 矩阵; 第 1 行; 1, 2, 行分隔线; 第 2 行; 3, 4",
            "2 乘 2 矩阵; 第 1 行; 1, 2, 行分隔线; 第 2 行; 3, 4",
        ),
        (
            "charged-ion",
            "<math><msup><mrow><mi>S</mi><msub><mi>O</mi><mn>4</mn></msub></mrow><mrow><mn>2</mn><mo>−</mo></mrow></msup></math>",
            "大写 s, 大写 o, 下标 4 上标 2 负",
            "大写 s, 大写 o, 下标 4 上标 2 负",
        ),
        (
            "chemical-states",
            "<math><mrow><mi>Na</mi><mrow><mo>(</mo><mi>aq</mi><mo>)</mo></mrow><mo>+</mo><mi>Cl</mi><mrow><mo>(</mo><mi>s</mi><mo>)</mo></mrow></mrow></math>",
            "大写 n a, 水溶液; 加, 大写 c l, 固体",
            "大写 n a, 水溶液; 加, 大写 c l, 固体",
        ),
        (
            "chemical-single-bond",
            "<math><mrow><mi>C</mi><msub><mi>H</mi><mn>3</mn></msub><mo>−</mo><mi>O</mi><mi>H</mi></mrow></math>",
            "大写 c, 大写 h, 下标 3, 单键 大写 o, 大写 h",
            "大写 c, 大写 h, 下标 3, 单键 大写 o, 大写 h",
        ),
        (
            "isotope-prescripts",
            "<math><mmultiscripts><mtext>C</mtext><mprescripts/><mn>6</mn><mn>14</mn></mmultiscripts></math>",
            "上标 14, 下标 6, 大写 c",
            "上标 14, 下标 6, 大写 c",
        ),
        (
            "inverse-square-unit",
            "<math><mrow><mn>5</mn><msup><mi intent=':unit'>m</mi><mrow><mo>−</mo><mn>2</mn></mrow></msup></mrow></math>",
            "5 米的 负 2 次方",
            "5 米的 负 2 次方",
        ),
        (
            "micro-meter",
            "<math><mn>3</mn><mi intent=':unit'>µm</mi></math>",
            "3 微米",
            "3 微米",
        ),
    ];

    for (name, mathml, simple, clear) in cases {
        test("zh-cn", "SimpleSpeak", mathml, simple)
            .map_err(|error| anyhow::anyhow!("SimpleSpeak/{name}: {error}"))?;
        test("zh-cn", "ClearSpeak", mathml, clear)
            .map_err(|error| anyhow::anyhow!("ClearSpeak/{name}: {error}"))?;
    }

    let bayes = "<math><mrow><mi>P</mi><mo>(</mo><mi>A</mi><mo>|</mo><mi>B</mi><mo>)</mo><mo>=</mo><mfrac><mrow><mi>P</mi><mo>(</mo><mi>B</mi><mo>|</mo><mi>A</mi><mo>)</mo><mi>P</mi><mo>(</mo><mi>A</mi><mo>)</mo></mrow><mrow><mi>P</mi><mo>(</mo><mi>B</mi><mo>)</mo></mrow></mfrac></mrow></math>";
    test(
        "zh-cn",
        "SimpleSpeak",
        bayes,
        "在 大写 b 条件下 大写 a 的概率, 等于; 分数, 大写 b 的概率, 分之, 在 大写 a 条件下 大写 b 的概率 乘 大写 a 的概率, 结束分数",
    )?;
    test(
        "zh-cn",
        "ClearSpeak",
        bayes,
        "在 大写 b 条件下 大写 a 的概率, 等于; 分数，分子为; 在 大写 a 条件下 大写 b 的概率 乘 大写 a 的概率; 分母为 大写 b 的概率",
    )?;

    let mean = "<math><mover><mi>x</mi><mo>¯</mo></mover><mo>=</mo><mfrac><mrow><mi>x</mi><mo>+</mo><mi>y</mi></mrow><mn>2</mn></mfrac></math>";
    test(
        "zh-cn",
        "SimpleSpeak",
        mean,
        "x 上横线, 等于, 分数, 2 分之, x 加 y, 结束分数",
    )?;
    test(
        "zh-cn",
        "ClearSpeak",
        mean,
        "x 上横线, 等于, 分数，分子为; x 加 y; 分母为 2",
    )?;

    let variance = "<math><mi>σ</mi><mo>²</mo><mo>=</mo><mfrac><mrow><munderover><mo>∑</mo><mrow><mi>i</mi><mo>=</mo><mn>1</mn></mrow><mi>n</mi></munderover><msup><mrow><mo>(</mo><mi>x</mi><mo>−</mo><mover><mi>x</mi><mo>¯</mo></mover><mo>)</mo></mrow><mn>2</mn></msup></mrow><mi>n</mi></mfrac></math>";
    test(
        "zh-cn",
        "SimpleSpeak",
        variance,
        "sigma 上标 二; 等于; 分数, n 分之, 求和 从 i 等于 1 到 n; 左括号, x 减 x 上横线; 右括号 平方, 结束分数",
    )?;
    test(
        "zh-cn",
        "ClearSpeak",
        variance,
        "sigma 上标 二; 等于; 分数，分子为; 求和 从 i 等于 1 到 n; 左括号, x 减 x 上横线; 右括号 平方; 分母为 n",
    )?;

    let reaction = "<math><mmultiscripts><mtext>C</mtext><mprescripts/><mn>6</mn><mn>14</mn></mmultiscripts><mo>→</mo><mmultiscripts><mtext>N</mtext><mprescripts/><mn>7</mn><mn>14</mn></mmultiscripts><mo>+</mo><mmultiscripts><mtext>e</mtext><mprescripts/><mrow><mo>−</mo><mn>1</mn></mrow><mn>0</mn></mmultiscripts></math>";
    for style in ["SimpleSpeak", "ClearSpeak"] {
        test_prefs(
            "zh-cn",
            style,
            vec![("Verbosity", "Terse")],
            reaction,
            "14, 6, 大写 c; 形成, 14, 7, 大写 n; 加 0, 负 1, e",
        )
        .map_err(|error| anyhow::anyhow!("{style}/chemical-reaction: {error}"))?;
    }
    Ok(())
}
