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
fn non_divisibility_uses_the_textbook_relation() -> Result<()> {
    test(
        "zh",
        "ClearSpeak",
        "<math><mrow intent='does-not-divide($a,$b)'><mi arg='a'>a</mi><mi arg='b'>b</mi></mrow></math>",
        "a 不整除 b",
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
fn textbook_logic_and_geometry_symbols_use_standard_readings() -> Result<()> {
    // Prefer the relation and shape names used in mainland textbooks over visual Unicode descriptions.
    test(
        "zh",
        "SimpleSpeak",
        "<math><mo>&#x2200;</mo><mi>x</mi><mo>&#x2208;</mo><mi>R</mi><mo>,</mo><mi>p</mi><mo>&#x2227;</mo><mi>q</mi><mo>&#x21d2;</mo><mi>r</mi><mo>&#x2228;</mo><mi>s</mi></math>",
        "任意 x 属于 大写 r; 逗号; p 且 q 推出 r 或 s",
    )?;
    test("zh", "SimpleSpeak", "<math><mo>&#x2205;</mo></math>", "空集")?;
    test(
        "zh",
        "SimpleSpeak",
        "<math><mi>x</mi><mo>&#x21a6;</mo><msup><mi>x</mi><mn>2</mn></msup><mo>,</mo><mi>A</mi><mo>&#x2216;</mo><mi>B</mi></math>",
        "x 映射到 x 平方, 逗号; 大写 a 减去 大写 b",
    )?;
    test(
        "zh",
        "SimpleSpeak",
        "<math><mi>x</mi><mo>&#x27fc;</mo><msup><mi>x</mi><mn>2</mn></msup></math>",
        "x 映射到 x 平方",
    )?;
    test(
        "zh",
        "SimpleSpeak",
        "<math><mi>p</mi><mo>&#x27f9;</mo><mi>q</mi></math>",
        "p 推出 q",
    )?;
    test(
        "zh",
        "SimpleSpeak",
        "<math><mo>&#x25b3;</mo><mi>A</mi><mi>B</mi><mi>C</mi><mo>&#x223d;</mo><mo>&#x25b3;</mo><mi>D</mi><mi>E</mi><mi>F</mi></math>",
        "三角形, 大写 a 大写 b 大写 c; 相似于; 三角形, 大写 d 大写 e 大写 f",
    )?;
    test(
        "zh",
        "SimpleSpeak",
        "<math><mo>&#x25b3;</mo><mi>A</mi><mi>B</mi><mi>C</mi><mo>&#x224c;</mo><mo>&#x25b3;</mo><mi>D</mi><mi>E</mi><mi>F</mi></math>",
        "三角形, 大写 a 大写 b 大写 c; 全等于; 三角形, 大写 d 大写 e 大写 f",
    )?;
    test(
        "zh",
        "SimpleSpeak",
        "<math><mo>&#x2299;</mo><mi>O</mi><mo>,</mo><mo>&#x25b1;</mo><mi>A</mi><mi>B</mi><mi>C</mi><mi>D</mi></math>",
        "圆 大写 o, 逗号; 白色平行四边形; 大写 a 大写 b 大写 c 大写 d",
    )?;
    test(
        "zh",
        "SimpleSpeak",
        "<math><mi>a</mi><mo>&#x2299;</mo><mi>b</mi></math>",
        "a 带圈点运算符 b",
    )?;
    test(
        "zh",
        "SimpleSpeak",
        "<math><mo>&#x22bf;</mo><mi>A</mi><mi>B</mi><mi>C</mi></math>",
        "直角三角形, 大写 a 大写 b 大写 c",
    )
}

#[test]
fn plus_minus_symbols_follow_standard_contextual_readings() -> Result<()> {
    // GB 3102.11 distinguishes signs used alone from binary plus/minus operations.
    test(
        "zh",
        "SimpleSpeak",
        "<math><mo>&#xb1;</mo></math>",
        "正或负",
    )?;
    test(
        "zh",
        "SimpleSpeak",
        "<math><mo>&#xb1;</mo><mi>x</mi></math>",
        "正或负 x",
    )?;
    test(
        "zh",
        "SimpleSpeak",
        "<math><mn>20</mn><mo>&#xb1;</mo><mn>0.5</mn></math>",
        "20 加或减 0.5",
    )?;
    test(
        "zh",
        "SimpleSpeak",
        "<math><mo>&#x2213;</mo></math>",
        "负或正",
    )?;
    test(
        "zh",
        "SimpleSpeak",
        "<math><mi>a</mi><mo>&#x2213;</mo><mi>b</mi></math>",
        "a 减或加 b",
    )?;
    test(
        "zh",
        "ClearSpeak",
        "<math><mrow intent='plus-or-minus($a,$b)'><mi arg='a'>a</mi><mi arg='b'>b</mi></mrow></math>",
        "a 加或减 b",
    )?;
    test(
        "zh",
        "ClearSpeak",
        "<math><mrow intent='minus-or-plus($a,$b)'><mi arg='a'>c</mi><mi arg='b'>d</mi></mrow></math>",
        "c 减或加 d",
    )?;
    test(
        "zh",
        "SimpleSpeak",
        "<math><mi>a</mi><mo>&#x2266;</mo><mi>b</mi></math>",
        "a 小于等于 b",
    )?;
    test(
        "zh",
        "SimpleSpeak",
        "<math><mi>c</mi><mo>&#x2267;</mo><mi>d</mi></math>",
        "c 大于等于 d",
    )?;
    test(
        "zh",
        "SimpleSpeak",
        "<math><mi>a</mi><mo>&#x2a7d;</mo><mi>b</mi></math>",
        "a 小于等于 b",
    )?;
    test(
        "zh",
        "SimpleSpeak",
        "<math><mi>c</mi><mo>&#x2a7e;</mo><mi>d</mi></math>",
        "c 大于等于 d",
    )?;
    test(
        "zh",
        "SimpleSpeak",
        "<math><mi>f</mi><mo>&#x2243;</mo><mi>g</mi></math>",
        "f 渐近等于 g",
    )
}

#[test]
fn double_factorial_distinguishes_math_and_literal_contexts() -> Result<()> {
    // U+203C is a double factorial in formulas but remains punctuation in literal content.
    test(
        "zh",
        "SimpleSpeak",
        "<math><mo>&#x203c;</mo></math>",
        "双阶乘",
    )?;
    test(
        "zh",
        "SimpleSpeak",
        "<math><mrow intent=':literal'><mo>&#x203c;</mo></mrow></math>",
        "双感叹号",
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
    test("zh", "ClearSpeak", expr, "满足 x 大于 2 的所有 x 组成的集合")?;
    test_prefs(
        "zh",
        "ClearSpeak",
        vec![("ClearSpeak_Sets", "woAll")],
        expr,
        "满足 x 大于 2 的 x 组成的集合",
    )?;
    test("zh", "SimpleSpeak", expr, "满足 x 大于 2 的所有 x 组成的集合")
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
    test_prefs(
        "zh",
        "SimpleSpeak",
        vec![("Verbosity", "Verbose")],
        "<math><mrow intent='line-segment($a,$b)'><mi arg='a'>a</mi><mi arg='b'>b</mi></mrow></math>",
        "线段 a b",
    )?;
    test_prefs(
        "zh",
        "SimpleSpeak",
        vec![("Verbosity", "Verbose")],
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
    )?;
    test(
        "zh",
        "ClearSpeak",
        "<math><mrow intent='length($x)'><mi arg='x'>a</mi></mrow></math>",
        "a 的长度",
    )?;
    test(
        "zh",
        "ClearSpeak",
        "<math><mrow intent='area($x)'><mi arg='x'>s</mi></mrow></math>",
        "s 的面积",
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
    // Matrix row numbers follow the Chinese ordinal pattern "第 n 行".
    test(
        "zh",
        "SimpleSpeak",
        "<math><mrow><mo>(</mo><mtable><mtr><mtd><mn>1</mn></mtd><mtd><mn>2</mn></mtd></mtr><mtr><mtd><mn>3</mn></mtd><mtd><mn>4</mn></mtd></mtr></mtable><mo>)</mo></mrow></math>",
        "2 乘 2 矩阵; 第 1 行; 1, 2; 第 2 行; 3, 4",
    )
}

#[test]
fn menclose_names_the_mark_instead_of_the_result() -> Result<()> {
    // The notation describes the drawn enclosure: a radical sign or two crossing strike lines.
    test(
        "zh",
        "SimpleSpeak",
        "<math><menclose notation='radical'><mi>x</mi></menclose></math>",
        "根号, 包围 x",
    )?;
    test(
        "zh",
        "SimpleSpeak",
        "<math><menclose notation='updiagonalstrike downdiagonalstrike'><mi>x</mi></menclose></math>",
        "交叉, 划掉, 包围 x",
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

#[test]
fn unicode_letter_currency_and_operator_names_match_character_identity() -> Result<()> {
    // These names distinguish characters that were blank or assigned to a different symbol.
    let cases = [
        ("0306", "上加短音符"),
        ("030c", "上加倒抑扬符"),
        ("0430", "西里尔字母阿"),
        ("043b", "西里尔字母埃勒"),
        ("0440", "西里尔字母埃尔"),
        ("0607", "阿拉伯-印度四次方根"),
        ("20b3", "奥斯特拉尔货币符号"),
        ("20e7", "年金符号"),
        ("2106", "每个"),
        ("2114", "磅符号"),
        ("2116", "序号符号"),
        ("2127", "姆欧"),
        ("2129", "倒置希腊小写字母约塔"),
        ("223c", "波浪运算符"),
        ("223d", "相似于"),
        ("2240", "圈积"),
        ("2244", "不渐近等于"),
        ("2246", "近似但不等于"),
        ("2257", "约等于"),
        ("225c", "三角等号"),
        ("226d", "不等价于"),
        ("22a3", "左断言符"),
        ("22a6", "断言符"),
        ("22a8", "为真"),
        ("22b8", "多重映射"),
        ("22c6", "星号运算符"),
        ("2327", "矩形框内的 X"),
        ("2332", "锥度"),
        ("23e3", "带圆圈的苯环"),
        ("260c", "合"),
    ];

    for (codepoint, expected) in cases {
        let expr = format!("<math><mo>&#x{codepoint};</mo></math>");
        test("zh", "SimpleSpeak", &expr, expected)
            .map_err(|error| anyhow::anyhow!("U+{codepoint}: {error}"))?;
    }
    Ok(())
}

#[test]
fn unicode_arrows_shapes_and_ornaments_name_visible_features() -> Result<()> {
    // Direction, fill, quadrant, and bracket shape must remain distinguishable in speech.
    let cases = [
        ("219e", "向左双头箭头"),
        ("21ba", "逆时针开口圆箭头"),
        ("21bb", "顺时针开口圆箭头"),
        ("21dc", "向左曲线箭头"),
        ("21dd", "向右曲线箭头"),
        ("21c4", "上方右箭头下方左箭头"),
        ("21c5", "左侧上箭头右侧下箭头"),
        ("21c6", "上方左箭头下方右箭头"),
        ("21ea", "从横线向上的白色箭头"),
        ("25cd", "竖直线填充的圆"),
        ("25d4", "右上象限为黑色的圆"),
        ("25d5", "除左上象限外为黑色的圆"),
        ("25d9", "反白圆"),
        ("25da", "上半反白圆"),
        ("25db", "下半反白圆"),
        ("25e6", "复合"),
        ("29eb", "黑色长菱形"),
        ("2661", "空心红桃"),
        ("2665", "实心红桃"),
        ("2680", "骰子一点"),
        ("2688", "右侧带白点的实心圆"),
        ("2768", "中等左圆括号装饰符"),
        ("2774", "中等左花括号装饰符"),
    ];

    for (codepoint, expected) in cases {
        let expr = format!("<math><mo>&#x{codepoint};</mo></math>");
        test("zh", "SimpleSpeak", &expr, expected)
            .map_err(|error| anyhow::anyhow!("U+{codepoint}: {error}"))?;
    }
    Ok(())
}

#[test]
fn unicode_composite_symbols_preserve_feature_and_spatial_order() -> Result<()> {
    // Expected names follow the Unicode character identity, including which component is above.
    let cases = [
        ("2105", "转交"),
        ("231c", "左上角"),
        ("23dc", "上置圆括号"),
        ("23df", "下置花括号"),
        ("23e0", "上置六角括号"),
        ("23e1", "下置六角括号"),
        ("2681", "骰子二点"),
        ("26aa", "中等白色圆"),
        ("27c1", "内含小型白色三角形的白色三角形"),
        ("27c3", "开子集"),
        ("27c5", "左 S 形多重集定界符"),
        ("27c6", "右 S 形多重集定界符"),
        ("27ca", "带横线的竖线"),
        ("27d0", "中心带点的白色菱形"),
        ("27e0", "被横线分割的长菱形"),
        ("2772", "细左六角括号装饰符"),
        ("2773", "细右六角括号装饰符"),
        ("27ec", "左白六角括号"),
        ("27ed", "右白六角括号"),
        ("27f2", "逆时针缺口圆箭头"),
        ("27f3", "顺时针缺口圆箭头"),
        ("27f4", "带圈加号的向右箭头"),
        ("27ff", "向右长曲线箭头"),
        ("2938", "顺时针右侧弧形箭头"),
        ("2942", "短向左箭头上方的向右箭头"),
        (
            "294a",
            "倒钩向上的向左鱼叉箭头与倒钩向下的向右鱼叉箭头",
        ),
        ("2970", "圆头向右双线箭头"),
        ("2971", "向右箭头上方的等号"),
        ("2976", "向左箭头上方的小于号"),
        ("298d", "上角带短线的左方括号"),
        ("2997", "左黑六角括号"),
        ("2998", "右黑六角括号"),
        ("29a8", "开口边末端带向上偏右箭头的测量角"),
        ("29ac", "开口边末端带向右偏上箭头的测量角"),
        ("29b1", "上方带横线的空集"),
        ("29b5", "带横线的圆"),
        ("2a22", "上方带小圆圈的加号"),
        ("2a48", "并集号、横线、交集号从上到下排列"),
        ("2a81", "上方带点的小于或倾斜等号"),
        ("2a82", "上方带点的大于或倾斜等号"),
        ("2a83", "右上方带点的小于或倾斜等号"),
        ("2a84", "左上方带点的大于或倾斜等号"),
        ("2a8b", "小于号、双线等号、大于号从上到下排列"),
        ("2aa8", "曲线闭合的小于号在斜等号上方"),
        ("2aa9", "曲线闭合的大于号在斜等号上方"),
        ("2acd", "左侧开口的方框运算符"),
        ("2acf", "闭子集"),
        ("2ada", "顶部带丁字的叉形符号"),
        ("2b00", "向右上方的白色箭头"),
        ("2b1a", "点状方框"),
        ("2b27", "黑色中等长菱形"),
        ("2b39", "带箭尾和竖线的向左箭头"),
        ("3248", "黑色方块上的带圈数字十"),
        ("1f8d1", "上方为长向右鱼叉箭头，下方为长向左鱼叉箭头"),
        ("1f8d2", "上方为长向右鱼叉箭头，下方为短向左鱼叉箭头"),
        ("1f8d3", "上方为短向右鱼叉箭头，下方为长向左鱼叉箭头"),
    ];

    for (codepoint, expected) in cases {
        let expr = format!("<math><mo>&#x{codepoint};</mo></math>");
        test("zh", "SimpleSpeak", &expr, expected)
            .map_err(|error| anyhow::anyhow!("U+{codepoint}: {error}"))?;
    }
    Ok(())
}

#[test]
fn audited_unicode_symbols_keep_precise_names() -> Result<()> {
    // Each case locks a corrected identity, direction, shape, or spatial relationship.
    let cases = [
        ("00ab", "左双角引号"),
        ("21a8", "带底线的上下箭头"),
        ("21ab", "向左带环箭头"),
        ("21ac", "向右带环箭头"),
        ("21fd", "向左开口箭头"),
        ("21fe", "向右开口箭头"),
        ("21ff", "左右开口箭头"),
        ("224c", "全等于"),
        ("2298", "带圈斜杠"),
        ("229f", "方框减号"),
        ("22a0", "方框乘号"),
        ("22a1", "方框点运算符"),
        ("22a9", "力迫"),
        ("22ae", "不力迫"),
        ("22dc", "等于或小于"),
        ("22e2", "既非方形像也不等于"),
        ("22e3", "既非方形原像也不等于"),
        ("2303", "向上箭头尖"),
        ("2333", "斜度"),
        ("23e6", "交流电"),
        ("2472", "带圈数字十九"),
        ("2736", "黑色六角星"),
        ("2794", "粗宽头向右箭头"),
        ("290a", "向上三重箭头"),
        ("290b", "向下三重箭头"),
        ("2983", "左白色花括号"),
        ("2984", "右白色花括号"),
        ("2993", "左弧小于括号"),
        ("2994", "右弧大于括号"),
        ("2995", "双左弧大于括号"),
        ("2996", "双右弧小于括号"),
        ("299b", "开口向左的测量角"),
        ("299d", "带点的测量直角"),
        ("2abd", "带点的子集号"),
        ("2abe", "带点的超集号"),
        ("2ad3", "子集号在超集号上方"),
        ("2ad4", "超集号在子集号上方"),
        ("2ad5", "子集号在子集号上方"),
        ("2ad6", "超集号在超集号上方"),
        ("e920", "双重方形并集"),
        ("e921", "双重方形交集"),
        ("e92c", "带点的恒等号"),
        ("e994", "带双斜杠的恒等号"),
        ("e997", "竖直正比号"),
        ("ea70", "带竖线的既不是正规子群也不等于"),
        ("ea71", "带竖线的既不包含正规子群也不等于"),
        ("eb60", "否定向右波浪箭头"),
        ("eb61", "否定向右弯曲箭头"),
        ("ec44", "水平全长三键"),
        ("ec47", "竖直全长三键"),
        ("ec4c", "水平半长三键"),
        ("fe35", "上置圆括号"),
        ("fe36", "下置圆括号"),
        ("fe37", "上置花括号"),
        ("fe38", "下置花括号"),
        ("fe3f", "上置角括号"),
        ("fe40", "下置角括号"),
    ];

    for (codepoint, expected) in cases {
        let expr = format!("<math><mo>&#x{codepoint};</mo></math>");
        test("zh", "SimpleSpeak", &expr, expected)
            .map_err(|error| anyhow::anyhow!("U+{codepoint}: {error}"))?;
    }
    Ok(())
}

#[test]
fn audited_unicode_math_names_match_symbol_identity() -> Result<()> {
    // Each entry covers a corrected Unicode name or a standard mainland mathematical reading.
    let cases = [
        ("02d9", "上点符"),
        ("02ef", "修饰字母低位向下箭头尖"),
        ("02f0", "修饰字母低位向上箭头尖"),
        ("02f1", "修饰字母低位向左箭头尖"),
        ("02f2", "修饰字母低位向右箭头尖"),
        ("0332", "下加下划线"),
        ("0333", "下加双下划线"),
        ("2135", "阿列夫"),
        ("2136", "贝特"),
        ("2137", "吉梅尔"),
        ("2138", "达列特"),
        ("2140", "双线体求和号"),
        ("222f", "曲面积分"),
        ("2231", "顺时针积分"),
        ("225f", "是否等于"),
        ("22d8", "远远小于"),
        ("22d9", "远远大于"),
        ("299a", "竖直之字形线"),
        ("29e2", "混洗积"),
        ("2a00", "n 元带圈点运算符"),
        ("2a01", "n 元带圈加号运算符"),
        ("2a02", "n 元带圈乘号运算符"),
        ("2a03", "带点的 n 元并集运算符"),
        ("2a04", "带加号的 n 元并集运算符"),
        ("2a05", "n 元方交集运算符"),
        ("2a06", "n 元方并集运算符"),
        ("2a09", "n 元乘号运算符"),
        ("2a0f", "带斜线的平均积分号"),
        ("2a33", "压缩积"),
        ("2a50", "带衬线和压缩积的闭合并集"),
        ("2a85", "小于或约等于"),
        ("2a86", "大于或约等于"),
        ("2af9", "双线倾斜小于或等于"),
        ("2afa", "双线倾斜大于或等于"),
        ("3372", "道尔顿"),
        ("3375", "小写 o 大写 V"),
        ("fb05", "长 s t 连字"),
    ];

    for (codepoint, expected) in cases {
        let expr = format!("<math><mo>&#x{codepoint};</mo></math>");
        test("zh", "SimpleSpeak", &expr, expected)
            .map_err(|error| anyhow::anyhow!("U+{codepoint}: {error}"))?;
    }
    Ok(())
}

#[test]
fn audited_mathtype_private_use_symbols_match_their_source_names() -> Result<()> {
    // MathType PUA symbols have no Unicode fallback, so each corrected identity needs a direct check.
    let cases = [
        ("e916", "带点的超集号"),
        ("e917", "带点的子集号"),
        ("e918", "下方带点的等号"),
        ("e92e", "竖线运算符"),
        ("e92f", "双竖线运算符"),
        ("e930", "三重竖线运算符"),
        ("e950", "带竖线的正规包含于"),
        ("e951", "带竖线的包含正规子群"),
        ("e98f", "自由基圆点运算符"),
        ("e991", "恒等于且平行于"),
        ("e992", "压缩积"),
        ("e993", "带横线的三重竖线运算符"),
        ("e995", "带三条竖线的三重横线"),
        ("e9a0", "负正弦波"),
        ("ea06", "既不小于也不等于"),
        ("ea07", "既不大于也不等于"),
        ("ea15", "既不后于也不相似于"),
        ("ea1d", "既不小于也不等于"),
        ("ea1e", "既不大于也不等于"),
        ("ea2e", "否定竖线运算符"),
        ("ea2f", "否定双竖线运算符"),
        ("ea30", "否定三重竖线运算符"),
        ("ea50", "带竖线的不是正规子群"),
        ("ea51", "带竖线的不包含正规子群"),
        ("ea55", "既不等于也不相似于"),
        ("ea63", "不严格等价于"),
        ("eb0f", "带斜线的大左右箭头"),
        ("eb11", "带斜线的大左右双箭头"),
        ("eb18", "带尾部和斜线的向右箭头"),
        (
            "eb36",
            "左侧倒钩向下、右侧倒钩向上的双鱼叉箭头",
        ),
        (
            "eb37",
            "左侧倒钩向上、右侧倒钩向下的双鱼叉箭头",
        ),
        ("eb3f", "右上与右下双箭头"),
        ("eb4c", "粗短黑色向左箭头"),
    ];

    for (codepoint, expected) in cases {
        let expr = format!("<math><mo>&#x{codepoint};</mo></math>");
        test("zh", "SimpleSpeak", &expr, expected)
            .map_err(|error| anyhow::anyhow!("U+{codepoint}: {error}"))?;
    }
    Ok(())
}

#[test]
fn mathtype_double_struck_greek_matches_the_original_character_table() -> Result<()> {
    // Check every MathType PUA slot because the source order is non-alphabetic in both ranges.
    let capitals = [
        ("f201", "德尔塔"),
        ("f202", "克西"),
        ("f203", "拉姆达"),
        ("f204", "派"),
        ("f205", "西格马"),
        ("f206", "西塔"),
        ("f207", "伽马"),
        ("f208", "欧米伽"),
        ("f209", "宇普西隆"),
    ];
    for (codepoint, name) in capitals {
        let expr = format!("<math><mi>&#x{codepoint};</mi></math>");
        let expected = format!("双线体 大写 {name}");
        test("zh", "SimpleSpeak", &expr, &expected)
            .map_err(|error| anyhow::anyhow!("U+{codepoint}: {error}"))?;
    }

    let lowercase = [
        ("f220", "阿尔法"),
        ("f221", "贝塔"),
        ("f222", "斐"),
        ("f223", "泽塔"),
        ("f224", "普西"),
        ("f225", "德尔塔"),
        ("f226", "艾普西隆"),
        ("f227", "伽马"),
        ("f228", "伊塔"),
        ("f229", "约塔"),
        ("f22a", "克西"),
        ("f22b", "卡帕"),
        ("f22c", "拉姆达"),
        ("f22d", "缪"),
        ("f22e", "纽"),
        ("f22f", "艾普西隆"),
        ("f230", "派"),
        ("f231", "西塔"),
        ("f232", "柔"),
        ("f233", "西格马"),
        ("f234", "陶"),
        ("f235", "西塔"),
        ("f236", "欧米伽"),
    ];
    for (codepoint, name) in lowercase {
        let expr = format!("<math><mi>&#x{codepoint};</mi></math>");
        let expected = format!("双线体 {name}");
        test("zh", "SimpleSpeak", &expr, &expected)
            .map_err(|error| anyhow::anyhow!("U+{codepoint}: {error}"))?;
    }
    Ok(())
}

#[test]
fn mathematical_capital_theta_symbols_remain_speakable_in_every_variant() -> Result<()> {
    // Each 25-character range has an extra theta-symbol slot where U+03A2 must not be used.
    let cases = [
        ("1d6b9", "粗体 大写 西塔"),
        ("f419", "粗体 大写 西塔"),
        ("1d6f3", "大写 西塔"),
        ("f453", "大写 西塔"),
        ("1d72d", "粗体 大写 西塔"),
        ("f48d", "粗体 大写 西塔"),
        ("1d767", "粗体 大写 西塔"),
        ("f4c7", "粗体 大写 西塔"),
        ("1d7a1", "粗体 大写 西塔"),
        ("f501", "粗体 大写 西塔"),
    ];

    for (codepoint, expected) in cases {
        let expr = format!("<math><mi>&#x{codepoint};</mi></math>");
        test("zh", "SimpleSpeak", &expr, expected)
            .map_err(|error| anyhow::anyhow!("U+{codepoint}: {error}"))?;
    }
    Ok(())
}

#[test]
fn audited_unicode_ranges_keep_their_boundaries() -> Result<()> {
    // The first and last characters catch off-by-one and shifted translate mappings.
    let cases = [
        ("03aa", "大写 约塔 带分音符"),
        ("03ab", "大写 宇普西隆 带分音符"),
        ("03cf", "大写 凯"),
        ("24b6", "带圈 大写 a"),
        ("24cf", "带圈 大写 z"),
        ("24d0", "带圈 a"),
        ("24e9", "带圈 z"),
        ("1d538", "双线体 大写 a"),
        ("1d550", "双线体 大写 y"),
        ("f080", "双线体 大写 a"),
        ("f098", "双线体 大写 y"),
    ];

    for (codepoint, expected) in cases {
        let expr = format!("<math><mi>&#x{codepoint};</mi></math>");
        test("zh", "SimpleSpeak", &expr, expected)
            .map_err(|error| anyhow::anyhow!("U+{codepoint}: {error}"))?;
    }
    Ok(())
}

#[test]
fn unicode_dingbats_editorial_marks_and_compatibility_units_are_precise() -> Result<()> {
    // Less common symbols still need exact names because visual context is unavailable to speech users.
    let cases = [
        ("2798", "粗向右下箭头"),
        ("27a2", "顶部高亮的立体向右箭头尖"),
        ("27b4", "黑色羽状向右下箭头"),
        ("27bc", "楔尾向右箭头"),
        ("2999", "点状围栏"),
        ("29cc", "三角形内字母 S"),
        ("2b1d", "黑色极小方块"),
        ("2b2a", "黑色小长菱形"),
        ("2b2c", "黑色横向椭圆"),
        ("2b30", "带小圆圈的向左箭头"),
        ("2b33", "向左长曲线箭头"),
        ("2b51", "黑色小星"),
        ("2b59", "粗圆圈内的叉号"),
        ("2e00", "直角替换标记"),
        ("2e08", "点状换位标记"),
        ("2e13", "带点奥贝洛斯符号"),
        ("2e16", "带点的右指角"),
        ("2e18", "倒置疑问感叹号"),
        ("2e19", "棕榈枝"),
        ("2e1b", "上方带圆环的波浪线"),
        ("2e30", "圆环点"),
        ("3014", "左六角括号"),
        ("3015", "右六角括号"),
        ("3018", "左白六角括号"),
        ("3019", "右白六角括号"),
        ("33c2", "上午"),
        ("33d8", "下午"),
        ("33da", "拍伦琴"),
        ("33d4", "毫靶恩"),
        ("33c7", "公司"),
        ("33ff", "伽"),
        ("fe64", "小型小于号"),
        ("fe65", "小型大于号"),
    ];

    for (codepoint, expected) in cases {
        let expr = format!("<math><mo>&#x{codepoint};</mo></math>");
        test("zh", "SimpleSpeak", &expr, expected)
            .map_err(|error| anyhow::anyhow!("U+{codepoint}: {error}"))?;
    }
    Ok(())
}

#[test]
fn gallon_unit_remains_distinct_from_the_gal_acceleration_symbol() -> Result<()> {
    // The unit token "gal" means gallon; U+33FF is the Gal acceleration unit and is tested above.
    test(
        "zh",
        "SimpleSpeak",
        "<math><mn>1</mn><mi intent=':unit'>gal</mi></math>",
        "1 加仑",
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
fn navigation_uses_chinese_row_and_column_number_order() -> Result<()> {
    // Cell location announcements use "第 n 行，第 n 列" rather than noun-first order.
    init_panic_handler();
    let result = catch_unwind(AssertUnwindSafe(|| {
        init_navigation(
            "<math><mrow><mo>(</mo><mtable><mtr><mtd><mn id='r1c1'>1</mn></mtd><mtd><mn id='r1c2'>2</mn></mtd></mtr><mtr><mtd><mn id='r2c1'>3</mn></mtd><mtd><mn id='r2c2'>4</mn></mtd></mtr></mtable><mo>)</mo></mrow></math>",
        )?;
        set_navigation_node("r2c2", 0)?;
        let speech = do_navigate_command("ReadCellCurrent")?;
        let speech = speech.trim_end_matches([' ', ',', ';']);
        assert_eq!("朗读当前单元格; 第 2 行, 第 2 列, 4", speech);
        Ok(())
    }));
    report_any_panic(result)
}

#[test]
fn navigation_overview_keeps_the_root_possessive_marker() -> Result<()> {
    // Overview speech must keep 的 after a complex radicand, just like full speech.
    init_panic_handler();
    let result = catch_unwind(AssertUnwindSafe(|| {
        init_navigation(
            "<math><mroot><mrow><mi>x</mi><mo>+</mo><mi>y</mi></mrow><mi>n</mi></mroot></math>",
        )?;
        let speech = do_navigate_command("DescribeCurrent")?;
        let speech = speech.trim_end_matches([' ', ',', ';']);
        assert_eq!("概述 当前项; x 加 y 的 n 次方根", speech);
        Ok(())
    }));
    report_any_panic(result)
}

#[test]
fn clearspeak_multiline_labels_use_chinese_count_and_ordinal_order() -> Result<()> {
    // Overview counts take classifiers, while each branch is introduced as an ordinal.
    test(
        "zh",
        "ClearSpeak",
        "<math><mrow><mo>{</mo><mtable><mtr><mtd><mi>x</mi><mo>&gt;</mo><mn>0</mn></mtd></mtr><mtr><mtd><mi>x</mi><mo>&lt;</mo><mn>0</mn></mtd></mtr></mtable></mrow></math>",
        "2 个分支; 第 1 个分支; x 大于 0; 第 2 个分支; x 小于 0",
    )
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
            "3 米每 二 次方秒",
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
