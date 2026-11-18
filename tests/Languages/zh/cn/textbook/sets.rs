//! Mainland textbook terminology and structural speech regressions.

use crate::common::*;
use anyhow::Result;

#[test]
fn set_relations_use_textbook_containment_terms() -> Result<()> {
    test(
        "zh-cn",
        "SimpleSpeak",
        "<math><mi>A</mi><mo>&#x2282;</mo><mi>B</mi><mo>,</mo><mi>C</mi><mo>&#x2286;</mo><mi>D</mi><mo>,</mo><mi>E</mi><mo>&#x228a;</mo><mi>F</mi><mo>,</mo><mi>G</mi><mo>&#x2acb;</mo><mi>H</mi></math>",
        "大写 a 真包含于 大写 b; 逗号; 大写 c 包含于 大写 d; 逗号; 大写 e 真包含于 大写 f; 逗号; 大写 g 真包含于 大写 h",
    )?;
    test(
        "zh-cn",
        "SimpleSpeak",
        "<math><mi>A</mi><mo>&#x2284;</mo><mi>B</mi><mo>,</mo><mi>C</mi><mo>&#x2288;</mo><mi>D</mi></math>",
        "大写 a 不是 大写 b 的真子集, 逗号; 大写 c 不包含于 大写 d",
    )?;
    test(
        "zh-cn",
        "ClearSpeak",
        "<math><mrow intent='subset($a,$b)'><mi arg='a'>x</mi><mi arg='b'>y</mi></mrow></math>",
        "x 真包含于 y",
    )?;
    test(
        "zh-cn",
        "ClearSpeak",
        "<math><mrow intent='subset-or-equal($a,$b)'><mi arg='a'>x</mi><mi arg='b'>y</mi></mrow></math>",
        "x 包含于 y",
    )
}

#[test]
fn set_operations_use_standard_verb_forms() -> Result<()> {
    test(
        "zh-cn",
        "SimpleSpeak",
        "<math><mi>A</mi><mo>&#x2229;</mo><mi>B</mi><mo>&#x222a;</mo><mi>C</mi></math>",
        "大写 a 交 大写 b, 并 大写 c",
    )?;
    test(
        "zh-cn",
        "ClearSpeak",
        "<math><mrow intent='intersection($a,$b)'><mi arg='a'>x</mi><mi arg='b'>y</mi></mrow></math>",
        "x 交 y",
    )?;
    test(
        "zh-cn",
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
        test("zh-cn", "ClearSpeak", &expr, expected)?;
    }
    Ok(())
}

#[test]
fn set_builder_description_is_natural() -> Result<()> {
    let expr = "<math><mrow><mo>{</mo><mrow><mi>x</mi><mo>|</mo><mi>x</mi><mo>&gt;</mo><mn>2</mn></mrow><mo>}</mo></mrow></math>";
    test("zh-cn", "ClearSpeak", expr, "满足 x 大于 2 的所有 x 组成的集合")?;
    test_prefs(
        "zh-cn",
        "ClearSpeak",
        vec![("ClearSpeak_Sets", "woAll")],
        expr,
        "满足 x 大于 2 的 x 组成的集合",
    )?;
    test("zh-cn", "SimpleSpeak", expr, "满足 x 大于 2 的所有 x 组成的集合")
}

#[test]
fn multi_argument_set_intent_is_not_mistaken_for_empty_set() -> Result<()> {
    test(
        "zh-cn",
        "ClearSpeak",
        "<math><mrow intent='set($a,$b)'><mi arg='a'>x</mi><mi arg='b'>y</mi></mrow></math>",
        "集合 x 逗号, y",
    )
}

#[test]
fn finite_intervals_use_mainland_textbook_readings() -> Result<()> {
    let cases = [
        (
            "open",
            "<math><mrow intent='open-interval($start,$end)'><mi arg='start'>a</mi><mi arg='end'>b</mi></mrow></math>",
            "开区间 a 到 b",
            "从 a 到 b 的开区间",
        ),
        (
            "closed",
            "<math><mrow><mo>[</mo><mrow><mi>a</mi><mo>,</mo><mi>b</mi></mrow><mo>]</mo></mrow></math>",
            "闭区间 a 到 b",
            "从 a 到 b 的闭区间",
        ),
        (
            "open-closed",
            "<math><mrow><mo>(</mo><mrow><mi>a</mi><mo>,</mo><mi>b</mi></mrow><mo>]</mo></mrow></math>",
            "左开右闭区间 a 到 b",
            "从 a 到 b 的左开右闭区间",
        ),
        (
            "closed-open",
            "<math><mrow><mo>[</mo><mrow><mi>a</mi><mo>,</mo><mi>b</mi></mrow><mo>)</mo></mrow></math>",
            "左闭右开区间 a 到 b",
            "从 a 到 b 的左闭右开区间",
        ),
        (
            "decimal-membership",
            "<math xmlns='http://www.w3.org/1998/Math/MathML'><mi>η</mi><mo>∈</mo><mrow><mo stretchy='false'>[</mo><mrow><mn>0.15</mn><mo separator='true'>,</mo><mn>0.25</mn></mrow><mo stretchy='false'>]</mo></mrow></math>",
            "eta 属于 闭区间 0.15 到 0.25",
            "eta 属于 从 0.15 到 0.25 的闭区间",
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

// Equal operands make negated strict and inclusive containment differ in truth value.
#[test]
fn negated_containment_distinguishes_equal_sets() -> Result<()> {
    for (operator, expected) in [
        ("⊄", "a 不是 a 的真子集"),
        ("⊈", "a 不包含于 a"),
        ("⊅", "a 不是 a 的真超集"),
        ("⊉", "a 不包含 a"),
    ] {
        let expr = format!("<math><mi>a</mi><mo>{operator}</mo><mi>a</mi></math>");
        test("zh-cn", "ClearSpeak", &expr, expected)?;
    }
    test("zh-cn", "ClearSpeak", "<math><mrow intent='not-subset($a,$b)'><mi arg='a'>x</mi><mi arg='b'>y</mi></mrow></math>", "x 不是 y 的真子集")?;
    return Ok(());
}
