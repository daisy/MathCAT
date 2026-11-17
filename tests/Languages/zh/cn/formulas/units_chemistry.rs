//! Representative formula and domain-specific speech regressions.

use crate::common::*;
use anyhow::Result;

#[test]
fn common_compound_units_use_natural_chinese_order() -> Result<()> {
    // Keep number-unit adjacency and numerator-before-denominator unit order in real measurements.
    let cases = [
        (
            "speed",
            "<math><mfrac><mrow><mn>90</mn><mi intent=':unit'>km</mi></mrow><mi intent=':unit'>h</mi></mfrac></math>",
            "90 千米每 小时",
        ),
        (
            "temperature",
            "<math><mn>25</mn><mi intent=':unit'>&#xb0;C</mi></math>",
            "25 摄氏度",
        ),
    ];

    for (name, mathml, expected) in cases {
        test("zh", "SimpleSpeak", mathml, expected)
            .map_err(|error| anyhow::anyhow!("{name}: {error}"))?;
    }
    Ok(())
}

#[test]
fn units_chemistry_and_adornments_keep_standard_terms() -> Result<()> {
    // Units and chemical notation combine ordinary symbols with special contextual rules.
    let cases = [
        (
            "radian-unit",
            "<math><mfrac><mi>π</mi><mn>2</mn></mfrac><mi intent=':unit'>rad</mi></math>",
            "2 分之 pi, 弧度",
            "2 分之 pi 弧度",
        ),
        (
            "si-prefix",
            "<math><mn>5</mn><mi intent=':unit'>kN</mi></math>",
            "5 千牛顿",
            "5 千牛顿",
        ),
        (
            "density-unit",
            "<math><mfrac><mi intent=':unit'>kg</mi><msup><mi intent=':unit'>m</mi><mn>3</mn></msup></mfrac></math>",
            "千克每 立方米",
            "千克每 立方米",
        ),
        (
            "molarity",
            "<math><mfrac><mrow><mn>0.5</mn><mi intent=':unit'>mol</mi></mrow><mi intent=':unit'>L</mi></mfrac></math>",
            "0.5 摩尔每 升",
            "0.5 摩尔每 升",
        ),
        (
            "fahrenheit-unit",
            "<math><mn>68</mn><mi intent=':unit'>°F</mi></math>",
            "68 华氏度",
            "68 华氏度",
        ),
        (
            "ohm-unit",
            "<math><mn>10</mn><mi intent=':unit'>Ω</mi></math>",
            "10 欧姆",
            "10 欧姆",
        ),
        (
            "aluminum-sulfate",
            "<math><mrow><msub><mi mathvariant='normal'>Al</mi><mn>2</mn></msub><msub><mrow><mo>(</mo><mi mathvariant='normal'>S</mi><msub><mi mathvariant='normal'>O</mi><mn>4</mn></msub><mo>)</mo></mrow><mn>3</mn></msub></mrow></math>",
            "大写 a l, 下标 2; 左括号, 大写 s, 大写 o, 下标 4; 右括号 下标 3",
            "大写 a l, 下标 2; 左括号, 大写 s, 大写 o, 下标 4; 右括号 下标 3",
        ),
        (
            "precipitation-reaction",
            "<math><mrow><mi mathvariant='normal'>Ag</mi><mo>+</mo><mi mathvariant='normal'>Cl</mi><mo>→</mo><mi mathvariant='normal'>AgCl</mi><mo>↓</mo></mrow></math>",
            "大写 a g, 加 大写 c l; 反应形成, 大写 a g, 大写 c l; 向下箭头",
            "大写 a g, 加 大写 c l; 反应形成, 大写 a g, 大写 c l; 向下箭头",
        ),
        (
            "reaction-condition",
            "<math><mrow><mi mathvariant='normal'>A</mi><mover><mo>→</mo><mi>Δ</mi></mover><mi mathvariant='normal'>B</mi></mrow></math>",
            "大写 a, 向右箭头 上方有 大写 delta, 大写 b",
            "大写 a, 向右箭头 上方有 大写 delta, 大写 b",
        ),
        (
            "hydrate-dot",
            "<math><mrow><mi mathvariant='normal'>Cu</mi><msub><mi mathvariant='normal'>SO</mi><mn>4</mn></msub><mo>·</mo><mn>5</mn><msub><mi mathvariant='normal'>H</mi><mn>2</mn></msub><mi mathvariant='normal'>O</mi></mrow></math>",
            "大写 c u, 大写 s, 大写 o, 下标 4; 点, 5, 大写 h, 下标 2, 大写 o",
            "大写 c u, 大写 s, 大写 o, 下标 4; 乘, 5, 大写 h, 下标 2, 大写 o",
        ),
        (
            "chemical-triple-bond",
            "<math><mrow data-chem-formula='3'><mi mathvariant='normal' data-chem-element='1'>N</mi><mo data-chemical-bond='true' data-chem-formula-op='1'>≡</mo><mi mathvariant='normal' data-chem-element='1'>N</mi></mrow></math>",
            "大写 n, 三键 大写 n",
            "大写 n, 三键 大写 n",
        ),
        (
            "double-prime",
            "<math><msup><mi>f</mi><mo>″</mo></msup><mo>⁡</mo><mi>x</mi></math>",
            "f 双撇号, x",
            "f 双撇号, x",
        ),
        (
            "triple-prime",
            "<math><msup><mi>f</mi><mo>‴</mo></msup><mo>⁡</mo><mi>x</mi></math>",
            "f 三撇号, x",
            "f 三撇号, x",
        ),
        (
            "hat-variable",
            "<math><mover><mi>x</mi><mo>^</mo></mover></math>",
            "x 帽符",
            "x 帽符",
        ),
        (
            "dot-variable",
            "<math><mover><mi>x</mi><mo>˙</mo></mover></math>",
            "x 上点符",
            "x 上点符",
        ),
        (
            "overbrace",
            "<math><mover><mrow><mi>a</mi><mo>+</mo><mi>b</mi></mrow><mo>⏞</mo></mover></math>",
            "a 加 b 上方有 上置花括号",
            "a 加 b 上方有 上置花括号",
        ),
        (
            "underbrace",
            "<math><munder><mrow><mi>a</mi><mo>+</mo><mi>b</mi></mrow><mo>⏟</mo></munder></math>",
            "a 加 b 下方有 下置花括号",
            "a 加 b 下方有 下置花括号",
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
fn inline_unit_operators_grouping_and_currency_rates_use_natural_chinese() -> Result<()> {
    // Inline unit slashes read as "每", unit multiplication dots/crosses and redundant
    // denominator grouping parentheses stay silent, Greek mu (U+03BC) matches micro (U+00B5),
    // and currency amounts work with variables and per-unit rates.
    test(
        "zh",
        "SimpleSpeak",
        "<math><mn>5</mn><mi mathvariant='normal'>J</mi><mo>/</mo><mo>(</mo><mi mathvariant='normal'>kg</mi><mo>·</mo><mi mathvariant='normal'>K</mi><mo>)</mo></math>",
        "5, 焦耳 每, 千克 开尔文",
    )?;
    test(
        "zh",
        "SimpleSpeak",
        "<math><mi mathvariant='normal'>m</mi><mo>∕</mo><mi mathvariant='normal'>s</mi></math>",
        "米 每 秒",
    )?;
    test(
        "zh",
        "SimpleSpeak",
        "<math><mi mathvariant='normal'>N</mi><mo>⋅</mo><mi mathvariant='normal'>m</mi></math>",
        "牛顿 米",
    )?;
    test(
        "zh",
        "SimpleSpeak",
        "<math><mi mathvariant='normal'>kW</mi><mo>×</mo><mi mathvariant='normal'>h</mi></math>",
        "千瓦特 小时",
    )?;
    test(
        "zh",
        "SimpleSpeak",
        "<math><mi mathvariant='normal'>Pa</mi><mo>∙</mo><mi mathvariant='normal'>s</mi></math>",
        "帕 秒",
    )?;
    test(
        "zh",
        "SimpleSpeak",
        "<math><mn>10</mn><mi intent=':unit'>μm</mi></math>",
        "10 微米",
    )?;
    test("zh", "SimpleSpeak", "<math><mo>$</mo><mi>x</mi></math>", "x 美元")?;
    test(
        "zh",
        "SimpleSpeak",
        "<math><mfrac><mrow><mo>$</mo><mn>5</mn></mrow><mi mathvariant='normal'>h</mi></mfrac></math>",
        "5 美元每 小时",
    )?;
    test(
        "zh",
        "ClearSpeak",
        "<math><mfrac><mrow><mo>$</mo><mn>5</mn></mrow><mi mathvariant='normal'>h</mi></mfrac></math>",
        "5 美元每 小时",
    )?;
    return Ok(());
}
