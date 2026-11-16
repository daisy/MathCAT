//! Mainland textbook terminology and structural speech regressions.

use crate::common::*;
use anyhow::Result;

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
        "a 小于或倾斜等于 b",
    )?;
    test(
        "zh",
        "SimpleSpeak",
        "<math><mi>c</mi><mo>&#x2a7e;</mo><mi>d</mi></math>",
        "c 大于或倾斜等于 d",
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
fn tau_and_dotted_minus_symbols_are_distinguishable() -> Result<()> {
    test("zh", "SimpleSpeak", "<math><mi>&#x03c4;</mi></math>", "tau")?;
    test(
        "zh",
        "SimpleSpeak",
        "<math><mo>&#x2a2b;</mo><mo>,</mo><mo>&#x2a2c;</mo></math>",
        "带下降点列的减号, 逗号, 带上升点列的减号",
    )
}
