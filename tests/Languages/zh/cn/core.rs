//! Core Simplified Chinese speech and navigation regression tests.

use crate::common::*;
use anyhow::Result;

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
    test_prefs(
        "zh",
        "SimpleSpeak",
        vec![("Verbosity", "Terse")],
        "<math><mroot><mi>x</mi><mn>3</mn></mroot></math>",
        "x 的 立方根",
    )?;
    test(
        "zh",
        "SimpleSpeak",
        "<math><mroot><mrow><mi>x</mi><mo>+</mo><mi>y</mi></mrow><mi>n</mi></mroot></math>",
        "x 加 y 的 n 次方根, 结束根号",
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
fn clearspeak_ordinal_exponents_keep_the_chinese_power_noun() -> Result<()> {
    // Chinese exponent readings use "次方" rather than an inflected ordinal form.
    test_prefs(
        "zh",
        "ClearSpeak",
        vec![("ClearSpeak_Exponents", "Ordinal")],
        "<math><msup><mi>x</mi><mn>4</mn></msup></math>",
        "x 的 4 次方",
    )
}

#[test]
fn confirmed_repairs_have_regression_coverage() -> Result<()> {
    // Keep the repaired readings consistent across both Chinese speech styles.
    let cases = [
        (
            "floor-and-ceiling",
            "<math><mrow><mo>&#x230a;</mo><mi>x</mi><mo>&#x230b;</mo></mrow><mo>+</mo><mrow><mo>&#x2308;</mo><mi>y</mi><mo>&#x2309;</mo></mrow></math>",
            "x 向下取整 加 y 向上取整",
        ),
        (
            "second-partial-derivative",
            "<math><mfrac><mrow><msup><mo>&#x2202;</mo><mn>2</mn></msup><mi>f</mi></mrow><mrow><mo>&#x2202;</mo><msup><mi>x</mi><mn>2</mn></msup></mrow></mfrac></math>",
            "f 对 x 的二阶偏导数",
        ),
        (
            "square-meter",
            "<math><mn>3</mn><msup><mi intent=':unit'>m</mi><mn>2</mn></msup></math>",
            "3 平方米",
        ),
        (
            "cubic-meter",
            "<math><mn>2</mn><msup><mi intent=':unit'>m</mi><mn>3</mn></msup></math>",
            "2 立方米",
        ),
        (
            "general-unit-power",
            "<math><msup><mi intent=':unit'>m</mi><mn>4</mn></msup></math>",
            "米的 4 次方",
        ),
        (
            "tilde-modified-variable",
            "<math><mover><mi>z</mi><mo>&#x303;</mo></mover></math>",
            "z 上加波浪线",
        ),
    ];

    for style in ["SimpleSpeak", "ClearSpeak"] {
        for (name, mathml, expected) in cases {
            test("zh", style, mathml, expected)
                .map_err(|error| anyhow::anyhow!("{style}/{name}: {error}"))?;
        }
    }
    Ok(())
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
fn units_increment_and_calculus_operators_use_standard_terms() -> Result<()> {
    // Differential operators put their operand before the operator name in Chinese.
    test(
        "zh",
        "SimpleSpeak",
        "<math><mn>1</mn><mi intent=':unit'>kat</mi></math>",
        "1 开特",
    )?;
    test(
        "zh",
        "SimpleSpeak",
        "<math><mo>&#x2206;</mo><mi>x</mi></math>",
        "增量 x",
    )?;
    test(
        "zh",
        "ClearSpeak",
        "<math><mrow intent='laplacian($x)'><mi arg='x'>f</mi></mrow></math>",
        "f 的拉普拉斯算子",
    )?;
    test(
        "zh",
        "ClearSpeak",
        "<math><mrow intent='divergence($x)'><mi arg='x'>f</mi></mrow></math>",
        "f 的散度",
    )?;
    test(
        "zh",
        "ClearSpeak",
        "<math><mrow intent='curl($x)'><mi arg='x'>f</mi></mrow></math>",
        "f 的旋度",
    )?;
    test(
        "zh",
        "ClearSpeak",
        "<math><mrow intent='gradient($x)'><mi arg='x'>f</mi></mrow></math>",
        "f 的梯度",
    )
}

#[test]
fn limit_arrows_and_variable_bars_use_mathematical_context() -> Result<()> {
    // Diagonal arrows denote one-sided limits only in a limit, and a bar over a variable is not a phonetic macron.
    test(
        "zh",
        "SimpleSpeak",
        "<math><munder><mo>lim</mo><mrow><mi>x</mi><mo>&#x2197;</mo><mn>0</mn></mrow></munder></math>",
        "极限，当 x 从下方趋于 0",
    )?;
    test(
        "zh",
        "SimpleSpeak",
        "<math><munder><mo>lim</mo><mrow><mi>x</mi><mo>&#x2198;</mo><mn>0</mn></mrow></munder></math>",
        "极限，当 x 从上方趋于 0",
    )?;
    test(
        "zh",
        "SimpleSpeak",
        "<math><mi>x</mi><mo>&#x2197;</mo><mi>y</mi><mo>,</mo><mi>x</mi><mo>&#x2198;</mo><mi>y</mi></math>",
        "x 右上箭头 y, 逗号; x 右下箭头 y",
    )?;
    test(
        "zh",
        "SimpleSpeak",
        "<math><mover><mi>x</mi><mo>&#xaf;</mo></mover></math>",
        "x 上横线",
    )?;
    test(
        "zh",
        "SimpleSpeak",
        "<math><mo>&#xaf;</mo></math>",
        "长音符",
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
fn permutation_counts_use_mainland_textbook_word_order() -> Result<()> {
    // All supported P-notation layouts mean the number of permutations obtained by taking k from n.
    let cases = [
        "<math><mmultiscripts><mi>P</mi><mi>k</mi><none/><mprescripts/><mi>n</mi><none/></mmultiscripts></math>",
        "<math><mmultiscripts><mi>P</mi><mi>k</mi><none/><mprescripts/><none/><mi>n</mi></mmultiscripts></math>",
        "<math><msubsup><mi>P</mi><mi>k</mi><mi>n</mi></msubsup></math>",
    ];

    for mathml in cases {
        test("zh", "SimpleSpeak", mathml, "n 取 k 的排列数")?;
    }
    Ok(())
}

#[test]
fn ellipses_use_the_standard_symbol_name() -> Result<()> {
    // U+2026 is the general ellipsis; U+22EF keeps its distinct midline name.
    test(
        "zh",
        "SimpleSpeak",
        "<math><mn>1</mn><mo>&#x2026;</mo><mn>3</mn><mo>,</mo><mo>&#x22ef;</mo></math>",
        "1 省略号 3, 逗号, 中线水平省略号",
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
        &equation("&#x1f8d1;"),
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
fn chemical_quadruple_bond_uses_the_standard_bond_order_term() -> Result<()> {
    // A bond formed by four shared electron pairs is a 四重键, parallel to 单键、双键、三键.
    test(
        "zh",
        "SimpleSpeak",
        "<math><mrow data-chem-formula='3'><mi mathvariant='normal' data-chem-element='1'>C</mi><mo data-chemical-bond='true' data-chem-formula-op='1'>&#x2263;</mo><mi mathvariant='normal' data-chem-element='1'>C</mi></mrow></math>",
        "大写 c, 四重键 大写 c",
    )
}
