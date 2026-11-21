//! Representative formula and domain-specific speech regressions.

use crate::common::*;
use anyhow::Result;

#[test]
fn clearspeak_multiline_labels_use_chinese_count_and_ordinal_order() -> Result<()> {
    // Overview counts take classifiers, while each branch is introduced as an ordinal.
    test(
        "zh-cn",
        "ClearSpeak",
        "<math><mrow><mo>{</mo><mtable><mtr><mtd><mi>x</mi><mo>&gt;</mo><mn>0</mn></mtd></mtr><mtr><mtd><mi>x</mi><mo>&lt;</mo><mn>0</mn></mtd></mtr></mtable></mrow></math>",
        "2 个分支; 第 1 个分支; x 大于 0; 第 2 个分支; x 小于 0",
    )
}

#[test]
fn uncommon_structures_keep_formula_order() -> Result<()> {
    // These formulas exercise structural branches that are easy to mis-order in Chinese speech.
    let cases = [
        (
            "polar-coordinate",
            "<math><mrow intent='polar-coordinate($x,$y)'><mi arg='x'>r</mi><mi arg='y'>θ</mi></mrow></math>",
            "极坐标 r 逗号, theta",
            "极坐标 r 逗号, theta",
        ),
        (
            "spherical-coordinate",
            "<math><mrow intent='spherical-coordinate($x,$y,$z)'><mi arg='x'>r</mi><mi arg='y'>θ</mi><mi arg='z'>φ</mi></mrow></math>",
            "球坐标 r 逗号, theta 逗号, phi",
            "球坐标 r 逗号, theta 逗号, phi",
        ),
        (
            "coordinate",
            "<math><mrow intent='coordinate($x,$y,$z)'><mi arg='x'>x</mi><mi arg='y'>y</mi><mi arg='z'>z</mi></mrow></math>",
            "坐标 x 逗号, y 逗号, z",
            "坐标 x 逗号, y 逗号, z",
        ),
        (
            "open-interval-to-infinity",
            "<math><mrow><mo>(</mo><mi>c</mi><mo>,</mo><mo>∞</mo><mo>)</mo></mrow></math>",
            "从 c 到无穷大的区间, 不包括端点 c",
            "从 c 到无穷大的区间, 不包括端点 c",
        ),
        (
            "closed-open-interval-to-infinity",
            "<math><mrow><mo>[</mo><mi>c</mi><mo>,</mo><mo>+</mo><mo>∞</mo><mo>)</mo></mrow></math>",
            "从 c 到正无穷大的区间, 包括端点 c",
            "从 c 到正无穷大的区间, 包括端点 c",
        ),
        (
            "negative-infinity-to-open",
            "<math><mrow><mo>(</mo><mo>−</mo><mo>∞</mo><mo>,</mo><mi>d</mi><mo>)</mo></mrow></math>",
            "从负无穷大到 d 的区间, 不包括端点 d",
            "从负无穷大到 d 的区间, 不包括端点 d",
        ),
        (
            "negative-infinity-to-closed",
            "<math><mrow><mo>(</mo><mo>−</mo><mo>∞</mo><mo>,</mo><mi>d</mi><mo>]</mo></mrow></math>",
            "从负无穷大到 d 的区间, 包括端点 d",
            "从负无穷大到 d 的区间, 包括端点 d",
        ),
        (
            "whole-real-line",
            "<math><mrow><mo>(</mo><mo>−</mo><mo>∞</mo><mo>,</mo><mo>∞</mo><mo>)</mo></mrow></math>",
            "实数集",
            "实数集",
        ),
        (
            "whole-real-line-explicit-positive-infinity",
            "<math><mrow><mo>(</mo><mo>−</mo><mo>∞</mo><mo>,</mo><mo>+</mo><mo>∞</mo><mo>)</mo></mrow></math>",
            "实数集",
            "实数集",
        ),
        (
            "arccosine",
            "<math><mrow intent='arccosine($x)'><mi arg='x'>x</mi></mrow></math>",
            "反余弦 x",
            "反余弦 x",
        ),
        (
            "inverse-hyperbolic-tangent",
            "<math><mrow intent='arc-hyperbolic-tangent($x)'><mi arg='x'>x</mi></mrow></math>",
            "反双曲正切 x",
            "反双曲正切 x",
        ),
        (
            "evaluated-at",
            "<math><mrow intent='evaluated-at($x,$y)'><mi arg='x'>f</mi><mi arg='y'>2</mi></mrow></math>",
            "f 取值于 2",
            "f 取值于 2",
        ),
        (
            "power-intent",
            "<math><mrow intent='power($x,$y)'><mi arg='x'>x</mi><mi arg='y'>n</mi></mrow></math>",
            "x 的 n 次方",
            "x 的 n 次方",
        ),
        (
            "mixed-partial-derivative",
            "<math><mfrac><mrow><msup><mo>∂</mo><mn>2</mn></msup><mi>f</mi></mrow><mrow><mo>∂</mo><mi>x</mi><mo>∂</mo><mi>y</mi></mrow></mfrac></math>",
            "分数, 偏导数 x 偏导数 y, 分之, 偏导数 平方 f, 结束分数",
            "分数，分子为; 偏导数 平方 f; 分母为 偏导数 x 偏导数 y",
        ),
        (
            "third-derivative",
            "<math><mfrac><mrow><msup><mo>d</mo><mn>3</mn></msup><mi>y</mi></mrow><mrow><mi>d</mi><msup><mi>x</mi><mn>3</mn></msup></mrow></mfrac></math>",
            "分数, d x 立方, 分之, d 立方 y, 结束分数",
            "分数，分子为; d 立方 y; 分母为 d x 立方",
        ),
        (
            "derivative-of-parenthesized-function",
            "<math><mfrac><mrow><mi>d</mi><mrow><mo>(</mo><mi>f</mi><mo>(</mo><mi>x</mi><mo>)</mo><mo>)</mo></mrow></mrow><mrow><mi>d</mi><mi>x</mi></mrow></mfrac></math>",
            "分数, d x, 分之, d, 左括号 f x 右括号, 结束分数",
            "分数，分子为; d, 左括号 f x 右括号; 分母为 d x",
        ),
        (
            "fractional-exponent",
            "<math><msup><mi>x</mi><mfrac><mn>1</mn><mn>2</mn></mfrac></msup></math>",
            "x 的 2 分之 1 次方",
            "x 的 2 分之 1 次方",
        ),
        (
            "fourth-root",
            "<math><mroot><mi>x</mi><mn>4</mn></mroot></math>",
            "x 的 4 次方根",
            "x 的 4 次方根",
        ),
        (
            "negative-cube-radicand",
            "<math><mroot><mrow><mo>−</mo><mn>8</mn></mrow><mn>3</mn></mroot></math>",
            "负 8 的 立方根, 结束根号",
            "负 8 的 立方根",
        ),
        (
            "subscript-and-superscript",
            "<math><msubsup><mi>x</mi><mi>i</mi><mi>j</mi></msubsup></math>",
            "x 下标 i, 的 j 次方",
            "x 下标 i, 的 j 次方",
        ),
        (
            "tensor-postscripts",
            "<math><mmultiscripts><mi>R</mi><mi>i</mi><none/><none/><mi>j</mi><mi>k</mi><none/></mmultiscripts></math>",
            "大写 r 有 3 组后置上下标, 下标 i 上标 j 下标 k",
            "大写 r 有 3 组后置上下标, 下标 i 上标 j 下标 k",
        ),
        (
            "augmented-matrix",
            "<math><mrow><mo>[</mo><mtable columnlines='none solid'><mtr><mtd><mn>1</mn></mtd><mtd><mn>2</mn></mtd><mtd><mn>3</mn></mtd></mtr><mtr><mtd><mn>4</mn></mtd><mtd><mn>5</mn></mtd><mtd><mn>6</mn></mtd></mtr></mtable><mo>]</mo></mrow></math>",
            "2 乘 3 增广矩阵; 第 1 行; 1, 2, 列分隔线, 3; 第 2 行; 4, 5, 列分隔线, 6",
            "2 乘 3 增广矩阵; 第 1 行; 1, 2, 列分隔线, 3; 第 2 行; 4, 5, 列分隔线, 6",
        ),
        (
            "matrix-product",
            "<math><mrow><mo>(</mo><mtable><mtr><mtd><mn>1</mn></mtd><mtd><mn>2</mn></mtd></mtr><mtr><mtd><mn>3</mn></mtd><mtd><mn>4</mn></mtd></mtr></mtable><mo>)</mo><mo>×</mo><mo>(</mo><mtable><mtr><mtd><mi>a</mi></mtd><mtd><mi>b</mi></mtd></mtr><mtr><mtd><mi>c</mi></mtd><mtd><mi>d</mi></mtd></mtr></mtable><mo>)</mo></mrow></math>",
            "2 乘 2 矩阵; 第 1 行; 1, 2; 第 2 行; 3, 4; 乘, 2 乘 2 矩阵; 第 1 行; a, b; 第 2 行; c, d",
            "2 乘 2 矩阵; 第 1 行; 1, 2; 第 2 行; 3, 4; 乘, 2 乘 2 矩阵; 第 1 行; a, b; 第 2 行; c, d",
        ),
        (
            "three-branch-piecewise",
            "<math><mrow><mo>{</mo><mtable><mtr><mtd><mrow><msup><mi>x</mi><mn>2</mn></msup><mtext> 当 </mtext><mi>x</mi><mo>&lt;</mo><mn>0</mn></mrow></mtd></mtr><mtr><mtd><mrow><mn>0</mn><mtext> 当 </mtext><mi>x</mi><mo>=</mo><mn>0</mn></mrow></mtd></mtr><mtr><mtd><mrow><mo>−</mo><mi>x</mi><mtext> 当 </mtext><mi>x</mi><mo>&gt;</mo><mn>0</mn></mrow></mtd></mtr></mtable></mrow></math>",
            "3 个分支; 第 1 个分支; x 平方 当 x, 小于 0; 第 2 个分支; 0 当 x, 等于 0; 第 3 个分支; 负 x 当 x, 大于 0",
            "3 个分支; 第 1 个分支; x 平方 当 x, 小于 0; 第 2 个分支; 0 当 x, 等于 0; 第 3 个分支; 负 x 当 x, 大于 0",
        ),
        (
            "piecewise-without-condition",
            "<math><mrow><mo>{</mo><mtable><mtr><mtd><mrow><mi>x</mi><mtext> 当 </mtext><mi>x</mi><mo>&lt;</mo><mn>0</mn></mrow></mtd></mtr><mtr><mtd><mo>−</mo><mi>x</mi></mtd></mtr></mtable></mrow></math>",
            "2 个分支; 第 1 个分支; x 当 x, 小于 0; 第 2 个分支; 负 x",
            "2 个分支; 第 1 个分支; x 当 x, 小于 0; 第 2 个分支; 负 x",
        ),
    ];

    for (name, mathml, simple, clear) in cases {
        test("zh-cn", "SimpleSpeak", mathml, simple)
            .map_err(|error| anyhow::anyhow!("SimpleSpeak/{name}: {error}"))?;
        test("zh-cn", "ClearSpeak", mathml, clear)
            .map_err(|error| anyhow::anyhow!("ClearSpeak/{name}: {error}"))?;
    }

    // ClearSpeak intentionally omits the closing marker for a nested radical; keep the
    // regression assertion on the complete SimpleSpeak reading instead.
    test(
        "zh-cn",
        "SimpleSpeak",
        "<math><msqrt><mrow><mn>1</mn><mo>+</mo><msqrt><mi>x</mi></msqrt></mrow></msqrt></math>",
        "根号 1 加 根号 x, 结束根号",
    )?;
    Ok(())
}

#[test]
fn unfenced_tables_and_arrays_announce_dimensions_without_row_commas() -> Result<()> {
    // Unfenced mtable receives inferred array intent and announces row/column counts
    // (including the single-column $NumColumns=1 case) without inserting commas between rows.
    let single_col = "<math><mtable><mtr><mtd><mn>3</mn></mtd></mtr><mtr><mtd><mn>2</mn></mtd></mtr></mtable></math>";
    test(
        "zh-cn",
        "SimpleSpeak",
        single_col,
        "表格有 2 行 与 1 列; 第 1 行; 第 1 列; 3; 第 2 行; 第 1 列; 2",
    )?;
    test(
        "zh-cn",
        "ClearSpeak",
        single_col,
        "表格有 2 行 与 1 列; 第 1 行; 第 1 列; 3; 第 2 行; 第 1 列; 2",
    )?;
    return Ok(());
}
