//! Core Simplified Chinese speech and navigation regression tests.

use crate::common::*;
use anyhow::Result;
use std::panic::{catch_unwind, AssertUnwindSafe};

#[test]
fn fractions_use_denominator_first_order() -> Result<()> {
    test(
        "zh",
        "SimpleSpeak",
        "<math><mfrac><mn>1</mn><mn>2</mn></mfrac></math>",
        "2 分之 1",
    )?;

    let complex =
        "<math><mfrac><mi>a</mi><mrow><mi>b</mi><mo>+</mo><mn>1</mn></mrow></mfrac></math>";
    test(
        "zh",
        "SimpleSpeak",
        complex,
        "分数, b 加 1, 分之 a, 结束分数",
    )?;
    test("zh", "ClearSpeak", complex, "分数，分子为 a; 分母为 b 加 1")
}

#[test]
fn zh_cn_locale_falls_back_to_simplified_chinese_rules() -> Result<()> {
    // Regional Chinese locales without overrides must use the base zh rules.
    test(
        "zh-cn",
        "SimpleSpeak",
        "<math><mfrac><mn>1</mn><mn>2</mn></mfrac></math>",
        "2 分之 1",
    )
}

#[test]
fn roots_and_powers_use_chinese_word_order() -> Result<()> {
    test(
        "zh",
        "SimpleSpeak",
        "<math><msqrt><mrow><mi>x</mi><mo>+</mo><mi>y</mi></mrow></msqrt></math>",
        "根号 x 加 y 结束根号",
    )?;
    test(
        "zh",
        "SimpleSpeak",
        "<math><mroot><mi>x</mi><mi>n</mi></mroot></math>",
        "x 的 n 次方根",
    )?;
    test(
        "zh",
        "SimpleSpeak",
        "<math><msup><mi>x</mi><mi>n</mi></msup></math>",
        "x 的 n 次方",
    )?;
    test(
        "zh",
        "LiteralSpeak",
        "<math><mroot><mrow><mi>x</mi><mo>+</mo><mi>y</mi></mrow><mi>n</mi></mroot></math>",
        "根指数 n 根号, x 加 y, 结束根号",
    )
}

#[test]
fn absolute_values_use_argument_first_order() -> Result<()> {
    let expr =
        "<math><mrow><mo>|</mo><mrow><mi>x</mi><mo>+</mo><mn>1</mn></mrow><mo>|</mo></mrow></math>";
    test("zh", "SimpleSpeak", expr, "x 加 1 的绝对值")?;
    test("zh", "ClearSpeak", expr, "x 加 1 的绝对值")?;
    test_prefs(
        "zh",
        "ClearSpeak",
        vec![("ClearSpeak_AbsoluteValue", "AbsEnd")],
        expr,
        "x 加 1 的绝对值, 结束绝对值",
    )?;
    test_prefs(
        "zh",
        "ClearSpeak",
        vec![("ClearSpeak_AbsoluteValue", "Cardinality")],
        "<math><mrow><mo>|</mo><mi>S</mi><mo>|</mo></mrow></math>",
        "大写 s 的基数",
    )
}

#[test]
fn literal_speech_matches_documented_examples() -> Result<()> {
    test(
        "zh",
        "LiteralSpeak",
        "<math><mfrac><mn>1</mn><mn>2</mn></mfrac></math>",
        "2 分之 1",
    )?;
    test(
        "zh",
        "LiteralSpeak",
        "<math><msqrt><mi>x</mi></msqrt></math>",
        "根号 x, 结束根号",
    )
}

#[test]
fn units_and_laplacian_use_standard_terms() -> Result<()> {
    test(
        "zh",
        "SimpleSpeak",
        "<math><mn>1</mn><mi intent=':unit'>kat</mi></math>",
        "1 开特",
    )?;
    test(
        "zh",
        "SimpleSpeak",
        "<math><mo>&#x2206;</mo><mi>f</mi></math>",
        "拉普拉斯算子 f",
    )
}

#[test]
fn permutation_cycles_and_repeating_decimals_use_concise_terms() -> Result<()> {
    // Use the standard nouns "轮换" and "循环节", without literal English-style expansion.
    test(
        "zh",
        "ClearSpeak",
        "<math><mrow intent='permutation-cycle($x)'><mi arg='x'>x</mi></mrow></math>",
        "轮换 x",
    )?;
    test(
        "zh",
        "SimpleSpeak",
        "<math><mrow intent='repeating-decimal($a,$b)'><mn arg='a'>0.1</mn><mn arg='b'>6</mn></mrow></math>",
        "0.1 循环节为 6",
    )
}

#[test]
fn ellipses_use_the_standard_symbol_name() -> Result<()> {
    // Both horizontal ellipsis characters should be announced as "省略号".
    test(
        "zh",
        "SimpleSpeak",
        "<math><mn>1</mn><mo>&#x2026;</mo><mn>3</mn><mo>,</mo><mo>&#x22ef;</mo></math>",
        "1 省略号 3, 逗号, 省略号",
    )
}

#[test]
fn chemical_equilibrium_arrows_use_standard_reaction_terms() -> Result<()> {
    // Distinguish a reversible reaction from equilibria biased to either side.
    let equation = |arrow: &str| {
        format!(
            "<math><mrow data-chem-equation='3'><mi mathvariant='normal' data-chem-element='1'>H</mi><mo data-chem-equation-op='1'>{arrow}</mo><mi mathvariant='normal' data-chem-element='1'>I</mi></mrow></math>"
        )
    };
    test(
        "zh",
        "SimpleSpeak",
        &equation("&#x21cc;"),
        "大写 h, 可逆反应 大写 i",
    )?;
    test(
        "zh",
        "SimpleSpeak",
        &equation("&#x1f8d3;"),
        "大写 h, 平衡偏左 大写 i",
    )?;
    test(
        "zh",
        "SimpleSpeak",
        &equation("&#x1f8d2;"),
        "大写 h, 平衡偏右 大写 i",
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
fn set_relations_use_textbook_containment_terms() -> Result<()> {
    test(
        "zh",
        "SimpleSpeak",
        "<math><mi>A</mi><mo>&#x2282;</mo><mi>B</mi><mo>,</mo><mi>C</mi><mo>&#x2286;</mo><mi>D</mi><mo>,</mo><mi>E</mi><mo>&#x228a;</mo><mi>F</mi><mo>,</mo><mi>G</mi><mo>&#x2acb;</mo><mi>H</mi></math>",
        "大写 a 真包含于 大写 b; 逗号; 大写 c 包含于 大写 d; 逗号; 大写 e 真包含于 大写 f; 逗号; 大写 g 真包含于 大写 h",
    )?;
    test(
        "zh",
        "SimpleSpeak",
        "<math><mi>A</mi><mo>&#x2284;</mo><mi>B</mi><mo>,</mo><mi>C</mi><mo>&#x2288;</mo><mi>D</mi></math>",
        "大写 a 不真包含于 大写 b; 逗号; 大写 c 不包含于 大写 d",
    )?;
    test(
        "zh",
        "ClearSpeak",
        "<math><mrow intent='subset($a,$b)'><mi arg='a'>x</mi><mi arg='b'>y</mi></mrow></math>",
        "x 真包含于 y",
    )?;
    test(
        "zh",
        "ClearSpeak",
        "<math><mrow intent='subset-or-equal($a,$b)'><mi arg='a'>x</mi><mi arg='b'>y</mi></mrow></math>",
        "x 包含于 y",
    )
}

#[test]
fn set_operations_use_standard_verb_forms() -> Result<()> {
    test(
        "zh",
        "SimpleSpeak",
        "<math><mi>A</mi><mo>&#x2229;</mo><mi>B</mi><mo>&#x222a;</mo><mi>C</mi></math>",
        "大写 a 交 大写 b, 并 大写 c",
    )?;
    test(
        "zh",
        "ClearSpeak",
        "<math><mrow intent='intersection($a,$b)'><mi arg='a'>x</mi><mi arg='b'>y</mi></mrow></math>",
        "x 交 y",
    )?;
    test(
        "zh",
        "ClearSpeak",
        "<math><mrow intent='union($a,$b)'><mi arg='a'>x</mi><mi arg='b'>y</mi></mrow></math>",
        "x 并 y",
    )
}

#[test]
fn standard_number_sets_use_textbook_names() -> Result<()> {
    let cases = [
        ("set-of-integers", "ℤ", "全体整数的集合"),
        ("set-of-reals", "ℝ", "全体实数的集合"),
        ("set-of-rationals", "ℚ", "全体有理数的集合"),
        ("set-of-natural-numbers", "ℕ", "全体自然数的集合"),
        ("set-of-complex-numbers", "ℂ", "全体复数的集合"),
        ("set-of-primes", "ℙ", "全体素数的集合"),
    ];

    for (intent, symbol, expected) in cases {
        let expr = format!("<math><mi intent='{intent}'>{symbol}</mi></math>");
        test("zh", "ClearSpeak", &expr, expected)?;
    }
    Ok(())
}

#[test]
fn set_builder_description_is_natural() -> Result<()> {
    let expr = "<math><mrow><mo>{</mo><mrow><mi>x</mi><mo>|</mo><mi>x</mi><mo>&gt;</mo><mn>2</mn></mrow><mo>}</mo></mrow></math>";
    test("zh", "ClearSpeak", expr, "所有满足 x 大于 2 的 x 的集合")?;
    test_prefs(
        "zh",
        "ClearSpeak",
        vec![("ClearSpeak_Sets", "woAll")],
        expr,
        "满足 x 大于 2 的 x 的集合",
    )?;
    test("zh", "SimpleSpeak", expr, "所有满足 x 大于 2 的 x 的集合")
}

#[test]
fn multi_argument_set_intent_is_not_mistaken_for_empty_set() -> Result<()> {
    test(
        "zh",
        "ClearSpeak",
        "<math><mrow intent='set($a,$b)'><mi arg='a'>x</mi><mi arg='b'>y</mi></mrow></math>",
        "集合 x 逗号, y",
    )
}

#[test]
fn half_open_intervals_name_both_ends() -> Result<()> {
    test(
        "zh",
        "SimpleSpeak",
        "<math><mrow><mo>(</mo><mrow><mi>a</mi><mo>,</mo><mi>b</mi></mrow><mo>]</mo></mrow></math>",
        "左开右闭区间 a 逗号 b",
    )?;
    test(
        "zh",
        "SimpleSpeak",
        "<math><mrow><mo>[</mo><mrow><mi>a</mi><mo>,</mo><mi>b</mi></mrow><mo>)</mo></mrow></math>",
        "左闭右开区间 a 逗号 b",
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
fn coordinates_and_angles_use_standard_terms() -> Result<()> {
    test(
        "zh",
        "ClearSpeak",
        "<math><mrow intent='cartesian-coordinate($a,$b,$c)'><mi arg='a'>x</mi><mi arg='b'>y</mi><mi arg='c'>z</mi></mrow></math>",
        "直角坐标 x 逗号, y 逗号, z",
    )?;
    test(
        "zh",
        "ClearSpeak",
        "<math><mrow intent='measured-angle:prefix($x)'><mi arg='x'>x</mi></mrow></math>",
        "角 x 的度数",
    )?;
    test(
        "zh",
        "SimpleSpeak",
        "<math><mrow intent='angle-measure:prefix($x)'><mi arg='x'>y</mi></mrow></math>",
        "角 y 的度数",
    )
}

#[test]
fn geometry_objects_use_textbook_word_order() -> Result<()> {
    test(
        "zh",
        "SimpleSpeak",
        "<math><mrow intent='line-segment($a,$b)'><mi arg='a'>a</mi><mi arg='b'>b</mi></mrow></math>",
        "线段 a b",
    )?;
    test(
        "zh",
        "SimpleSpeak",
        "<math><mrow intent='ray($a,$b)'><mi arg='a'>a</mi><mi arg='b'>b</mi></mrow></math>",
        "射线 a b",
    )?;
    test(
        "zh",
        "SimpleSpeak",
        "<math><mrow intent='arc($a,$b)'><mi arg='a'>a</mi><mi arg='b'>b</mi></mrow></math>",
        "弧 a b",
    )?;
    test(
        "zh",
        "SimpleSpeak",
        "<math><mrow intent='measure-of-angle($a,$b,$c)'><mi arg='a'>a</mi><mi arg='b'>b</mi><mi arg='c'>c</mi></mrow></math>",
        "角 a b c 的度数",
    )
}

#[test]
fn definite_integral_announces_limits() -> Result<()> {
    test(
        "zh",
        "SimpleSpeak",
        "<math><msubsup><mo>&#x222b;</mo><mn>0</mn><mn>1</mn></msubsup><mi>f</mi><mrow><mo>(</mo><mi>x</mi><mo>)</mo></mrow><mi>d</mi><mi>x</mi></math>",
        "积分 从 0 到 1, f x d x",
    )
}

#[test]
fn matrix_announces_dimensions_and_rows() -> Result<()> {
    test(
        "zh",
        "SimpleSpeak",
        "<math><mrow><mo>(</mo><mtable><mtr><mtd><mn>1</mn></mtd><mtd><mn>2</mn></mtd></mtr><mtr><mtd><mn>3</mn></mtd><mtd><mn>4</mn></mtd></mtr></mtable><mo>)</mo></mrow></math>",
        "2 乘 2 矩阵; 行 1; 1, 2; 行 2; 3, 4",
    )
}

#[test]
fn tau_and_dotted_minus_symbols_are_distinguishable() -> Result<()> {
    test("zh", "SimpleSpeak", "<math><mi>&#x03c4;</mi></math>", "陶")?;
    test(
        "zh",
        "SimpleSpeak",
        "<math><mo>&#x2a2b;</mo><mo>,</mo><mo>&#x2a2c;</mo></math>",
        "带下降点列的减号, 逗号, 带上升点列的减号",
    )
}

fn init_navigation(mathml: &str) -> Result<()> {
    set_rules_dir(abs_rules_dir_path())?;
    set_preference("Language", "zh")?;
    set_preference("SpeechStyle", "SimpleSpeak")?;
    set_preference("Verbosity", "Medium")?;
    set_preference("NavMode", "Enhanced")?;
    set_preference("NavVerbosity", "Verbose")?;
    set_preference("AutoZoomOut", "False")?;
    set_preference("Overview", "False")?;
    set_mathml(mathml)?;
    Ok(())
}

#[test]
fn navigation_enters_fraction_numerator() -> Result<()> {
    init_panic_handler();
    let result = catch_unwind(AssertUnwindSafe(|| {
        init_navigation(
            "<math><mfrac id='frac'><mn id='num'>1</mn><mn id='den'>2</mn></mfrac></math>",
        )?;
        let speech = do_navigate_command("SetPlacemarker1")?;
        let speech = speech.trim_end_matches([' ', ',', ';']);
        assert_eq!("设置位置标记 1; 2 分之 1", speech);

        let speech = do_navigate_command("DescribeCurrent")?;
        let speech = speech.trim_end_matches([' ', ',', ';']);
        assert_eq!("概述 当前项; 2 分之 1", speech);

        let speech = do_navigate_command("ZoomIn")?;
        let speech = speech.trim_end_matches([' ', ',', ';']);
        assert_eq!("进入下一层; 进入 分子; 1", speech);
        Ok(())
    }));
    report_any_panic(result)
}

#[test]
fn additional_real_world_readings() -> Result<()> {
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
            "偏导数 f",
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
            "v 点乘 w",
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
            "向量 v, 向量积 向量 w",
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
            "2 乘 2 行列式; 行 1; a, b; 行 2; c, d",
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
            "左方括号, 大写 s, 大写 o, 下标 4; 右方括号 上标 2 减",
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
            "3 米 每 秒 平方",
        ),
        (
            "piecewise",
            "SimpleSpeak",
            "Medium",
            "<math><mi>f</mi><mrow><mo>(</mo><mi>x</mi><mo>)</mo></mrow><mo>=</mo><mrow><mo>{</mo><mtable><mtr><mtd><msup><mi>x</mi><mn>2</mn></msup><mtext> 当 </mtext><mi>x</mi><mo>&#x2265;</mo><mn>0</mn></mtd></mtr><mtr><mtd><mo>&#x2212;</mo><mi>x</mi><mtext> 当 </mtext><mi>x</mi><mo>&lt;</mo><mn>0</mn></mtd></mtr></mtable></mrow></math>",
            "f x 等于; 2 分支; 分支 1; x 平方 当 x, 大于等于 0; 分支 2; 负 x 当 x, 小于 0",
        ),
        (
            "system-of-equations",
            "SimpleSpeak",
            "Medium",
            "<math><mtable><mtr><mtd><mi>x</mi><mo>+</mo><mi>y</mi></mtd><mtd><mo>=</mo></mtd><mtd><mn>7</mn></mtd></mtr><mtr><mtd><mn>2</mn><mi>x</mi><mo>+</mo><mn>3</mn><mi>y</mi></mtd><mtd><mo>=</mo></mtd><mtd><mn>17</mn></mtd></mtr></mtable></math>",
            "2 方程; 方程 1; x 加 y 等于 7; 方程 2; 2 x 加 3 y; 等于 17",
        ),
        (
            "quadratic-formula-simple",
            "SimpleSpeak",
            "Medium",
            "<math><mi>x</mi><mo>=</mo><mfrac><mrow><mo>&#x2212;</mo><mi>b</mi><mo>&#xb1;</mo><msqrt><mrow><msup><mi>b</mi><mn>2</mn></msup><mo>&#x2212;</mo><mn>4</mn><mi>a</mi><mi>c</mi></mrow></msqrt></mrow><mrow><mn>2</mn><mi>a</mi></mrow></mfrac></math>",
            "x 等于; 分数, 2 a, 分之, 负 b 正负, 根号 b 平方 减 4 a c 结束根号; 结束分数",
        ),
        (
            "quadratic-formula-literal",
            "LiteralSpeak",
            "Medium",
            "<math><mi>x</mi><mo>=</mo><mfrac><mrow><mo>&#x2212;</mo><mi>b</mi><mo>&#xb1;</mo><msqrt><mrow><msup><mi>b</mi><mn>2</mn></msup><mo>&#x2212;</mo><mn>4</mn><mi>a</mi><mi>c</mi></mrow></msqrt></mrow><mrow><mn>2</mn><mi>a</mi></mrow></mfrac></math>",
            "x 等于; 分数, 2 a, 分之, 减 b 正负; 根号 b 上标 2 结束上标, 减 4 a c, 结束根号; 结束分数",
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
            "zh",
            style,
            vec![("Verbosity", verbosity)],
            mathml,
            expected,
        )
        .map_err(|error| anyhow::anyhow!("{name}: {error}"))?;
    }
    Ok(())
}
