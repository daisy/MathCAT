//! Mainland textbook terminology and structural speech regressions.

use crate::common::*;
use anyhow::Result;

#[test]
fn textbook_logic_and_geometry_symbols_use_standard_readings() -> Result<()> {
    // Prefer the relation and shape names used in mainland textbooks over visual Unicode descriptions.
    test(
        "zh-cn",
        "SimpleSpeak",
        "<math><mo>&#x2200;</mo><mi>x</mi><mo>&#x2208;</mo><mi>R</mi><mo>,</mo><mi>p</mi><mo>&#x2227;</mo><mi>q</mi><mo>&#x21d2;</mo><mi>r</mi><mo>&#x2228;</mo><mi>s</mi></math>",
        "任意 x 属于 大写 r; 逗号; p 且 q 推出 r 或 s",
    )?;
    test("zh-cn", "SimpleSpeak", "<math><mo>&#x2205;</mo></math>", "空集")?;
    test(
        "zh-cn",
        "SimpleSpeak",
        "<math><mi>x</mi><mo>&#x21a6;</mo><msup><mi>x</mi><mn>2</mn></msup><mo>,</mo><mi>A</mi><mo>&#x2216;</mo><mi>B</mi></math>",
        "x 映射到 x 平方, 逗号; 大写 a 差集 大写 b",
    )?;
    test(
        "zh-cn",
        "SimpleSpeak",
        "<math><mi>x</mi><mo>&#x27fc;</mo><msup><mi>x</mi><mn>2</mn></msup></math>",
        "x 映射到 x 平方",
    )?;
    test(
        "zh-cn",
        "SimpleSpeak",
        "<math><mi>p</mi><mo>&#x27f9;</mo><mi>q</mi></math>",
        "p 推出 q",
    )?;
    test(
        "zh-cn",
        "SimpleSpeak",
        "<math><mrow intent='similar($a,$b)'><mrow arg='a'><mo>&#x25b3;</mo><mi>A</mi><mi>B</mi><mi>C</mi></mrow><mrow arg='b'><mo>&#x25b3;</mo><mi>D</mi><mi>E</mi><mi>F</mi></mrow></mrow></math>",
        "三角形, 大写 a 大写 b 大写 c 相似于; 三角形, 大写 d 大写 e 大写 f",
    )?;
    test(
        "zh-cn",
        "SimpleSpeak",
        "<math><mi>a</mi><mo>&#x223d;</mo><mi>b</mi></math>",
        "a 反转波浪号 b",
    )?;
    test(
        "zh-cn",
        "SimpleSpeak",
        "<math><mo>&#x25b3;</mo><mi>A</mi><mi>B</mi><mi>C</mi><mo>&#x2245;</mo><mo>&#x25b3;</mo><mi>D</mi><mi>E</mi><mi>F</mi></math>",
        "三角形, 大写 a 大写 b 大写 c; 近似等于; 三角形, 大写 d 大写 e 大写 f",
    )?;
    test(
        "zh-cn",
        "SimpleSpeak",
        "<math><mrow intent='congruent($a,$b)'><mrow arg='a'><mo>&#x25b3;</mo><mi>A</mi><mi>B</mi><mi>C</mi></mrow><mrow arg='b'><mo>&#x25b3;</mo><mi>D</mi><mi>E</mi><mi>F</mi></mrow></mrow></math>",
        "三角形, 大写 a 大写 b 大写 c 全等于; 三角形, 大写 d 大写 e 大写 f",
    )?;
    test(
        "zh-cn",
        "SimpleSpeak",
        "<math><mo>&#x2299;</mo><mi>O</mi><mo>,</mo><mo>&#x25b1;</mo><mi>A</mi><mi>B</mi><mi>C</mi><mi>D</mi></math>",
        "圆 大写 o, 逗号; 白色平行四边形; 大写 a 大写 b 大写 c 大写 d",
    )?;
    test(
        "zh-cn",
        "SimpleSpeak",
        "<math><mi>a</mi><mo>&#x2299;</mo><mi>b</mi></math>",
        "a 带圈点运算符 b",
    )?;
    test(
        "zh-cn",
        "SimpleSpeak",
        "<math><mo>&#x22bf;</mo><mi>A</mi><mi>B</mi><mi>C</mi></math>",
        "直角三角形, 大写 a 大写 b 大写 c",
    )
}

#[test]
fn coordinates_and_angles_use_standard_terms() -> Result<()> {
    test(
        "zh-cn",
        "ClearSpeak",
        "<math><mrow intent='cartesian-coordinate($a,$b,$c)'><mi arg='a'>x</mi><mi arg='b'>y</mi><mi arg='c'>z</mi></mrow></math>",
        "直角坐标 x 逗号, y 逗号, z",
    )?;
    test(
        "zh-cn",
        "ClearSpeak",
        "<math><mrow intent='measured-angle:prefix($x)'><mi arg='x'>x</mi></mrow></math>",
        "角 x 的度数",
    )?;
    test(
        "zh-cn",
        "SimpleSpeak",
        "<math><mrow intent='angle-measure:prefix($x)'><mi arg='x'>y</mi></mrow></math>",
        "角 y 的度数",
    )
}

#[test]
fn geometry_objects_use_textbook_word_order() -> Result<()> {
    test(
        "zh-cn",
        "SimpleSpeak",
        "<math><mrow intent='line-segment($a,$b)'><mi arg='a'>a</mi><mi arg='b'>b</mi></mrow></math>",
        "线段 a b",
    )?;
    test(
        "zh-cn",
        "SimpleSpeak",
        "<math><mrow intent='ray($a,$b)'><mi arg='a'>a</mi><mi arg='b'>b</mi></mrow></math>",
        "射线 a b",
    )?;
    test_prefs(
        "zh-cn",
        "SimpleSpeak",
        vec![("Verbosity", "Verbose")],
        "<math><mrow intent='line-segment($a,$b)'><mi arg='a'>a</mi><mi arg='b'>b</mi></mrow></math>",
        "线段 a b",
    )?;
    test_prefs(
        "zh-cn",
        "SimpleSpeak",
        vec![("Verbosity", "Verbose")],
        "<math><mrow intent='ray($a,$b)'><mi arg='a'>a</mi><mi arg='b'>b</mi></mrow></math>",
        "射线 a b",
    )?;
    test(
        "zh-cn",
        "SimpleSpeak",
        "<math><mrow intent='arc($a,$b)'><mi arg='a'>a</mi><mi arg='b'>b</mi></mrow></math>",
        "弧 a b",
    )?;
    test(
        "zh-cn",
        "SimpleSpeak",
        "<math><mrow intent='measure-of-angle($a,$b,$c)'><mi arg='a'>a</mi><mi arg='b'>b</mi><mi arg='c'>c</mi></mrow></math>",
        "角 a b c 的度数",
    )?;
    test(
        "zh-cn",
        "ClearSpeak",
        "<math><mrow intent='length($x)'><mi arg='x'>a</mi></mrow></math>",
        "a 的长度",
    )?;
    test(
        "zh-cn",
        "ClearSpeak",
        "<math><mrow intent='area($x)'><mi arg='x'>s</mi></mrow></math>",
        "s 的面积",
    )
}
