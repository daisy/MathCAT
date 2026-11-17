//! unicode-full relations, structured symbols, and phase audits.

use crate::common::*;
use anyhow::Result;

#[test]
fn unicode_full_relations_keep_distinct_mathematical_meanings() -> Result<()> {
    // These look similar to common relations, so retain the Unicode-defined distinction in speech.
    let cases = [
        (
            "natural-join",
            "<math><mi>R</mi><mo>&#x22c8;</mo><mi>S</mi></math>",
            "大写 r 自然连接 大写 s",
        ),
        (
            "slanted-less-or-equal",
            "<math><mi>a</mi><mo>&#x2a7d;</mo><mi>b</mi></math>",
            "a 小于或倾斜等于 b",
        ),
        (
            "slanted-greater-or-equal",
            "<math><mi>a</mi><mo>&#x2a7e;</mo><mi>b</mi></math>",
            "a 大于或倾斜等于 b",
        ),
        (
            "less-above-similar-or-equal",
            "<math><mi>a</mi><mo>&#x2a8d;</mo><mi>b</mi></math>",
            "a 小于、相似或等于 b",
        ),
        (
            "greater-above-similar-or-equal",
            "<math><mi>a</mi><mo>&#x2a8e;</mo><mi>b</mi></math>",
            "a 大于、相似或等于 b",
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
fn unicode_full_structured_symbols_have_verified_readings() -> Result<()> {
    // Use real MathML structures where a symbol's role affects its spoken form.
    let cases = [
        (
            "U+2A00 n-ary circled dot",
            "<math><munderover><mo>&#x2a00;</mo><mrow><mi>i</mi><mo>=</mo><mn>1</mn></mrow><mi>n</mi></munderover><msub><mi>a</mi><mi>i</mi></msub></math>",
            "n 元带圈点运算符 从 i 等于 1 到 n; a 下标 i",
        ),
        (
            "U+2A01 n-ary circled plus",
            "<math><munderover><mo>&#x2a01;</mo><mrow><mi>i</mi><mo>=</mo><mn>1</mn></mrow><mi>n</mi></munderover><msub><mi>a</mi><mi>i</mi></msub></math>",
            "n 元带圈加号运算符 从 i 等于 1 到 n; a 下标 i",
        ),
        (
            "U+2A02 n-ary circled times",
            "<math><munderover><mo>&#x2a02;</mo><mrow><mi>i</mi><mo>=</mo><mn>1</mn></mrow><mi>n</mi></munderover><msub><mi>a</mi><mi>i</mi></msub></math>",
            "n 元带圈乘号运算符 从 i 等于 1 到 n; a 下标 i",
        ),
        (
            "U+2A03 n-ary union with dot",
            "<math><munderover><mo>&#x2a03;</mo><mrow><mi>i</mi><mo>=</mo><mn>1</mn></mrow><mi>n</mi></munderover><msub><mi>A</mi><mi>i</mi></msub></math>",
            "带点的 n 元并集运算符 从 i 等于 1 到 n; 大写 a 下标 i",
        ),
        (
            "U+2A04 n-ary union with plus",
            "<math><munderover><mo>&#x2a04;</mo><mrow><mi>i</mi><mo>=</mo><mn>1</mn></mrow><mi>n</mi></munderover><msub><mi>A</mi><mi>i</mi></msub></math>",
            "带加号的 n 元并集运算符 从 i 等于 1 到 n; 大写 a 下标 i",
        ),
        (
            "U+2A05 n-ary square intersection",
            "<math><munderover><mo>&#x2a05;</mo><mrow><mi>i</mi><mo>=</mo><mn>1</mn></mrow><mi>n</mi></munderover><msub><mi>A</mi><mi>i</mi></msub></math>",
            "n 元方交集运算符 从 i 等于 1 到 n; 大写 a 下标 i",
        ),
        (
            "U+2A06 n-ary square union",
            "<math><munderover><mo>&#x2a06;</mo><mrow><mi>i</mi><mo>=</mo><mn>1</mn></mrow><mi>n</mi></munderover><msub><mi>A</mi><mi>i</mi></msub></math>",
            "n 元方并集运算符 从 i 等于 1 到 n; 大写 a 下标 i",
        ),
        (
            "U+228E multiset union",
            "<math><mi>A</mi><mo>&#x228e;</mo><mi>B</mi></math>",
            "大写 a 多重集并集 大写 b",
        ),
        (
            "U+2A0B summation with integral",
            "<math><mo>&#x2a0b;</mo><mi>f</mi><mo>&#x2061;</mo><mi>x</mi><mi>d</mi><mi>x</mi></math>",
            "带积分号的求和 f x d x",
        ),
        (
            "U+2A0C quadruple integral",
            "<math><mo>&#x2a0c;</mo><mi>f</mi><mo>&#x2061;</mo><mi>x</mi><mi>d</mi><mi>x</mi></math>",
            "四重积分运算符 f x d x",
        ),
        (
            "U+2A0D finite-part integral",
            "<math><mo>&#x2a0d;</mo><mi>f</mi><mo>&#x2061;</mo><mi>x</mi><mi>d</mi><mi>x</mi></math>",
            "有限部积分 f x d x",
        ),
        (
            "U+2A0E integral with double stroke",
            "<math><mo>&#x2a0e;</mo><mi>f</mi><mo>&#x2061;</mo><mi>x</mi><mi>d</mi><mi>x</mi></math>",
            "双线积分 f x d x",
        ),
        (
            "U+2A0F average integral",
            "<math><mo>&#x2a0f;</mo><mi>f</mi><mo>&#x2061;</mo><mi>x</mi><mi>d</mi><mi>x</mi></math>",
            "带斜线的平均积分号 f x d x",
        ),
        (
            "U+2A10 circulation function",
            "<math><mo>&#x2a10;</mo><mi>f</mi><mo>&#x2061;</mo><mi>x</mi><mi>d</mi><mi>x</mi></math>",
            "环流函数 f x d x",
        ),
        (
            "U+2A11 anticlockwise integration",
            "<math><mo>&#x2a11;</mo><mi>f</mi><mo>&#x2061;</mo><mi>x</mi><mi>d</mi><mi>x</mi></math>",
            "逆时针积分 f x d x",
        ),
        (
            "U+2A12 rectangular path around pole",
            "<math><mo>&#x2a12;</mo><mi>f</mi><mo>&#x2061;</mo><mi>x</mi><mi>d</mi><mi>x</mi></math>",
            "绕极点矩形路径线积分 f x d x",
        ),
        (
            "U+2A13 semicircular path around pole",
            "<math><mo>&#x2a13;</mo><mi>f</mi><mo>&#x2061;</mo><mi>x</mi><mi>d</mi><mi>x</mi></math>",
            "绕极点半圆路径线积分 f x d x",
        ),
        (
            "U+2A14 line integration not including pole",
            "<math><mo>&#x2a14;</mo><mi>f</mi><mo>&#x2061;</mo><mi>x</mi><mi>d</mi><mi>x</mi></math>",
            "不包含极点的线积分 f x d x",
        ),
        (
            "U+2A15 integral around a point",
            "<math><mo>&#x2a15;</mo><mi>f</mi><mo>&#x2061;</mo><mi>x</mi><mi>d</mi><mi>x</mi></math>",
            "绕点积分运算符 f x d x",
        ),
        (
            "U+2A16 quaternion integral",
            "<math><mo>&#x2a16;</mo><mi>f</mi><mo>&#x2061;</mo><mi>x</mi><mi>d</mi><mi>x</mi></math>",
            "四元数积分运算符 f x d x",
        ),
        (
            "U+2AC3 subset or equal with dot above",
            "<math><mi>A</mi><mo>&#x2ac3;</mo><mi>B</mi></math>",
            "大写 a, 上方带点的子集或等于号, 大写 b",
        ),
        (
            "U+2AC4 superset or equal with dot above",
            "<math><mi>A</mi><mo>&#x2ac4;</mo><mi>B</mi></math>",
            "大写 a, 上方带点的超集或等于号, 大写 b",
        ),
        (
            "U+2ADB transversal intersection",
            "<math><mi>A</mi><mo>&#x2adb;</mo><mi>B</mi></math>",
            "大写 a 横截相交 大写 b",
        ),
        (
            "U+2ADC forking",
            "<math><mi>A</mi><mo>&#x2adc;</mo><mi>B</mi></math>",
            "大写 a 分叉 大写 b",
        ),
        (
            "U+2ADD nonforking",
            "<math><mi>A</mi><mo>&#x2add;</mo><mi>B</mi></math>",
            "大写 a 非分叉 大写 b",
        ),
        (
            "U+2AF9 double-line slanted less or equal",
            "<math><mi>a</mi><mo>&#x2af9;</mo><mi>b</mi></math>",
            "a 双线倾斜小于或等于 b",
        ),
        (
            "U+2AFA double-line slanted greater or equal",
            "<math><mi>a</mi><mo>&#x2afa;</mo><mi>b</mi></math>",
            "a 双线倾斜大于或等于 b",
        ),
        (
            "U+2947 right arrow through x",
            "<math><mi>A</mi><mo>&#x2947;</mo><mi>B</mi></math>",
            "大写 a 穿过叉号的向右箭头, 大写 b",
        ),
        (
            "U+2948 left-right arrow through circle",
            "<math><mi>A</mi><mo>&#x2948;</mo><mi>B</mi></math>",
            "大写 a, 穿过小圆圈的左右箭头, 大写 b",
        ),
        (
            "U+2949 upward two-headed arrow from circle",
            "<math><mo>&#x2949;</mo><mi>x</mi></math>",
            "从小圆圈出发的向上双头箭头 x",
        ),
        (
            "U+297C left fish tail",
            "<math><mi>A</mi><mo>&#x297c;</mo><mi>B</mi></math>",
            "大写 a 左鱼尾 大写 b",
        ),
        (
            "U+297D right fish tail",
            "<math><mi>A</mi><mo>&#x297d;</mo><mi>B</mi></math>",
            "大写 a 右鱼尾 大写 b",
        ),
        (
            "U+297E upward fish tail",
            "<math><mi>A</mi><mo>&#x297e;</mo><mi>B</mi></math>",
            "大写 a 上鱼尾 大写 b",
        ),
        (
            "U+297F downward fish tail",
            "<math><mi>A</mi><mo>&#x297f;</mo><mi>B</mi></math>",
            "大写 a 下鱼尾 大写 b",
        ),
        (
            "U+29C4 squared rising diagonal slash",
            "<math><mo>&#x29c4;</mo><mi>x</mi></math>",
            "方框内上升斜线 x",
        ),
        (
            "U+29CA triangle with dot above",
            "<math><mo>&#x29ca;</mo><mi>x</mi></math>",
            "上方带点的三角形 x",
        ),
        (
            "U+20E1 combining left-right arrow above",
            "<math><mover><mi>v</mi><mo>&#x20e1;</mo></mover></math>",
            "v 上加左右箭头",
        ),
        (
            "U+20DE combining enclosing square",
            "<math><mover><mi>x</mi><mo>&#x20de;</mo></mover></math>",
            "x 外加方形",
        ),
        (
            "U+20E0 combining enclosing circle backslash",
            "<math><mover><mi>x</mi><mo>&#x20e0;</mo></mover></math>",
            "x 外加圆圈反斜杠",
        ),
        (
            "U+20DC combining four dots above",
            "<math><mover><mi>x</mi><mo>&#x20dc;</mo></mover></math>",
            "x 上加四点",
        ),
        (
            "U+20E9 combining wide bridge above",
            "<math><mover><mi>x</mi><mo>&#x20e9;</mo></mover></math>",
            "x 上加宽桥形符",
        ),
        (
            "U+20EB combining long double solidus overlay",
            "<math><mover><mi>x</mi><mo>&#x20eb;</mo></mover></math>",
            "x 叠加长双斜杠",
        ),
        (
            "U+20EF combining right arrow below",
            "<math><munder><mi>v</mi><mo>&#x20ef;</mo></munder></math>",
            "v 下加向右箭头",
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
fn unicode_script_characters_keep_standalone_and_structural_readings() -> Result<()> {
    let superscripts = [
        ("2070", "mn", "零次方", "x 的 0 次方"),
        ("2071", "mi", "i次方", "x 的 i 次方"),
        ("2074", "mn", "四次方", "x 的 4 次方"),
        ("2075", "mn", "五次方", "x 的 5 次方"),
        ("2076", "mn", "六次方", "x 的 6 次方"),
        ("2077", "mn", "七次方", "x 的 7 次方"),
        ("2078", "mn", "八次方", "x 的 8 次方"),
        ("2079", "mn", "九次方", "x 的 9 次方"),
        ("207a", "mo", "上标加号", "x 上标 加"),
        ("207b", "mo", "上标减号", "x 上标 减"),
        ("207c", "mo", "上标等号", "x 上标 等于"),
        ("207d", "mo", "上标左圆括号", "x 上标 左括号"),
        ("207e", "mo", "上标右圆括号", "x 上标 右括号"),
        ("207f", "mi", "n次方", "x 的 n 次方"),
    ];
    let subscripts = [
        ("2080", "mn", "下标零", "x 下标 0"),
        ("2081", "mn", "下标一", "x 下标 1"),
        ("2082", "mn", "下标二", "x 下标 2"),
        ("2083", "mn", "下标三", "x 下标 3"),
        ("2084", "mn", "下标四", "x 下标 4"),
        ("2085", "mn", "下标五", "x 下标 5"),
        ("2086", "mn", "下标六", "x 下标 6"),
        ("2087", "mn", "下标七", "x 下标 7"),
        ("2088", "mn", "下标八", "x 下标 8"),
        ("2089", "mn", "下标九", "x 下标 9"),
        ("208a", "mo", "下标加号", "x 下标 加 结束下标"),
        ("208b", "mo", "下标减号", "x 下标 减 结束下标"),
        ("208c", "mo", "下标等号", "x 下标 等于 结束下标"),
        ("208d", "mo", "下标左圆括号", "x 下标 左括号 结束下标"),
        ("208e", "mo", "下标右圆括号", "x 下标 右括号 结束下标"),
        ("2090", "mi", "下标a", "x 下标 a"),
        ("2091", "mi", "下标e", "x 下标 e"),
        ("2092", "mi", "下标o", "x 下标 o"),
        ("2093", "mi", "下标x", "x 下标 x"),
        ("2095", "mi", "下标h", "x 下标 h"),
        ("2096", "mi", "下标k", "x 下标 k"),
        ("2097", "mi", "下标l", "x 下标 l"),
        ("2098", "mi", "下标m", "x 下标 m"),
        ("2099", "mi", "下标n", "x 下标 n"),
        ("209a", "mi", "下标p", "x 下标 p"),
        ("209b", "mi", "下标s", "x 下标 s"),
        ("209c", "mi", "下标t", "x 下标 t"),
    ];

    for style in ["SimpleSpeak", "ClearSpeak"] {
        for &(codepoint, tag, standalone, structured) in &superscripts {
            let expr = format!("<math><mo>&#x{codepoint};</mo></math>");
            test("zh", style, &expr, standalone)
                .map_err(|error| anyhow::anyhow!("{style}/U+{codepoint}/alone: {error}"))?;
            let expr =
                format!("<math><msup><mi>x</mi><{tag}>&#x{codepoint};</{tag}></msup></math>");
            test("zh", style, &expr, structured)
                .map_err(|error| anyhow::anyhow!("{style}/U+{codepoint}/msup: {error}"))?;
        }
        for &(codepoint, tag, standalone, structured) in &subscripts {
            let expr = format!("<math><mo>&#x{codepoint};</mo></math>");
            test("zh", style, &expr, standalone)
                .map_err(|error| anyhow::anyhow!("{style}/U+{codepoint}/alone: {error}"))?;
            let expr =
                format!("<math><msub><mi>x</mi><{tag}>&#x{codepoint};</{tag}></msub></math>");
            test("zh", style, &expr, structured)
                .map_err(|error| anyhow::anyhow!("{style}/U+{codepoint}/msub: {error}"))?;
        }
    }
    Ok(())
}

#[test]
fn audited_unicode_gap_symbols_have_direct_and_contextual_coverage() -> Result<()> {
    // TINY and MINY are proper operator names, not translations of the size adjectives.
    let direct = [
        ("20e2", "外加屏幕"),
        ("220b", "包含"),
        ("220c", "不包含"),
        ("220d", "小型包含"),
        ("2256", "环等于"),
        ("228b", "真包含"),
        ("228d", "多重集乘法"),
        ("22e4", "方形像或不等于"),
        ("22e5", "方形原像或不等于"),
        ("22ee", "垂直省略号"),
        ("22f0", "右上对角线省略号"),
        ("22f1", "右下对角线省略号"),
        ("2a07", "双逻辑与运算符"),
        ("2a08", "双逻辑或运算符"),
        ("2a87", "小于且单线不等于"),
        ("2a88", "大于且单线不等于"),
        ("2a89", "小于且不约等于"),
        ("2a8a", "大于且不约等于"),
        ("2acc", "真包含"),
        ("2298", "带圆圈除号斜线"),
        ("22d4", "真相交"),
        ("29fe", "Tiny 算子"),
        ("29ff", "Miny 算子"),
        ("2ae1", "带 S 的垂直符号"),
    ];
    let contextual = [
        ("220b", "x", "大写 a 包含 x"),
        ("220c", "x", "大写 a 不包含 x"),
        ("220d", "x", "大写 a 小型包含 x"),
        ("2298", "B", "大写 a 带圆圈除号斜线, 大写 b"),
        ("22d4", "B", "大写 a 真相交 大写 b"),
        ("2a87", "b", "大写 a 小于且单线不等于 b"),
        ("2a88", "b", "大写 a 大于且单线不等于 b"),
        ("2ae1", "B", "大写 a 带 S 的垂直符号, 大写 b"),
    ];

    for style in ["SimpleSpeak", "ClearSpeak"] {
        for &(codepoint, expected) in &direct {
            let expr = format!("<math><mo>&#x{codepoint};</mo></math>");
            test("zh", style, &expr, expected)
                .map_err(|error| anyhow::anyhow!("{style}/U+{codepoint}/direct: {error}"))?;
        }
        for &(codepoint, rhs, expected) in &contextual {
            let expr = format!(
                "<math><mi>A</mi><mo>&#x{codepoint};</mo><mi>{rhs}</mi></math>"
            );
            test("zh", style, &expr, expected)
                .map_err(|error| anyhow::anyhow!("{style}/U+{codepoint}/context: {error}"))?;
        }
    }
    Ok(())
}

#[test]
fn remaining_unicode_phase_symbols_have_verified_readings() -> Result<()> {
    // Unicode NamesList supplies the identities; the structured cases verify placement too.
    let combining = [
        ("20d0", "mover", "v", "v 上加左鱼叉箭头"),
        ("20d1", "mover", "v", "v 上加右鱼叉箭头"),
        ("20d2", "mover", "x", "x 叠加长竖直线"),
        ("20d3", "mover", "x", "x 叠加短竖直线"),
        ("20d4", "mover", "x", "x 上加逆时针箭头"),
        ("20d5", "mover", "x", "x 上加顺时针箭头"),
        ("20d6", "mover", "v", "v 上加向左箭头"),
        ("20d7", "mover", "v", "v 上加向右箭头"),
        ("20d8", "mover", "x", "x 叠加圆环"),
        ("20d9", "mover", "x", "x 叠加顺时针圆环"),
        ("20da", "mover", "x", "x 叠加逆时针圆环"),
        ("20db", "mover", "x", "x 上加三点"),
        ("20dd", "mover", "x", "x 外加圆圈"),
        ("20df", "mover", "x", "x 外加菱形"),
        ("20e3", "mover", "x", "x 外加键帽"),
        ("20e4", "mover", "x", "x 外加向上三角形"),
        ("20e5", "mover", "x", "x 叠加反斜杠"),
        ("20e6", "mover", "x", "x 叠加双竖线"),
        ("20e8", "munder", "x", "x 下加三点"),
        ("20ea", "mover", "x", "x 叠加向左箭头"),
        (
            "20ec",
            "munder",
            "v",
            "v 下加带向下倒钩的向右鱼叉箭头",
        ),
        (
            "20ed",
            "munder",
            "v",
            "v 下加带向下倒钩的向左鱼叉箭头",
        ),
        ("20ee", "munder", "v", "v 下加向左箭头"),
        ("20f0", "mover", "x", "x 上加星号"),
    ];
    let relations = [
        ("22d0", "a 双子集 b"),
        ("22d1", "a 双超集 b"),
        ("22d2", "a 双交集 b"),
        ("22d3", "a 双并集 b"),
        ("22d5", "a 等于且平行于 b"),
        ("22d6", "a 带点小于 b"),
        ("22d7", "a 带点大于 b"),
        // Keep these distinct from U+226A/U+226B ("远小于/远大于").
        ("22d8", "a 极小于 b"),
        ("22d9", "a 极大于 b"),
        ("22da", "a 小于、等于或大于 b"),
        ("22db", "a 大于、等于或小于 b"),
        ("22dd", "a 等于或大于 b"),
        ("22de", "a 等于或先于 b"),
        ("22df", "a 等于或后于 b"),
    ];
    let phase_relations = [
        ("22e0", "a 既不先于也不等于 b"),
        ("22e1", "a 既不后于也不等于 b"),
        ("22e2", "a 既非方形像也不等于 b"),
        ("22e3", "a 既非方形原像也不等于 b"),
        ("22e4", "a 方形像或不等于 b"),
        ("22e5", "a 方形原像或不等于 b"),
        ("22e6", "a 小于但不等价于 b"),
        ("22e7", "a 大于但不等价于 b"),
        ("22e8", "a 先于但不等价于 b"),
        ("22e9", "a 后于但不等价于 b"),
        ("22ea", "a 不是正规子群 b"),
        ("22eb", "a 不包含正规子群 b"),
        ("22ec", "a 既不是正规子群也不等于 b"),
        ("22ed", "a 既不包含正规子群也不等于 b"),
    ];
    let ellipses = [
        ("22ee", "垂直省略号"),
        ("22ef", "中线水平省略号"),
        ("22f0", "右上对角线省略号"),
        ("22f1", "右下对角线省略号"),
    ];
    let set_relations = [
        ("22f2", "x 带长横线的属于号 a"),
        ("22f3", "x 横线末端带竖线的属于号 a"),
        ("22f4", "x 横线末端带竖线的较小属于号 a"),
        ("22f5", "x 上方带点的属于号 a"),
        ("22f6", "x 上方带横线的属于号 a"),
        ("22f7", "x 上方带横线的较小属于号 a"),
        ("22f8", "x 下方带横线的属于号 a"),
        ("22f9", "x 带两条横线的属于号 a"),
        ("22fa", "a 带长横线的包含号 x"),
        ("22fb", "a 横线末端带竖线的包含号 x"),
        ("22fc", "a 横线末端带竖线的较小包含号 x"),
        ("22fd", "a 上方带横线的包含号 x"),
        ("22fe", "a 上方带横线的较小包含号 x"),
        ("22ff", "x Z 记号多重集隶属符 a"),
    ];
    let integrals = [
        ("2a0a", "模二和 f"),
        ("2a17", "带钩向左箭头的积分号 f"),
        ("2a18", "带乘号的积分号 f"),
        ("2a19", "带交集号的积分号 f"),
        ("2a1a", "带并集号的积分号 f"),
        ("2a1b", "带上横线的积分号 f"),
        ("2a1c", "带下横线的积分号 f"),
    ];

    for style in ["SimpleSpeak", "ClearSpeak"] {
        for &(codepoint, tag, base, expected) in &combining {
            let expr = format!(
                "<math><{tag}><mi>{base}</mi><mo>&#x{codepoint};</mo></{tag}></math>"
            );
            test("zh", style, &expr, expected)
                .map_err(|error| anyhow::anyhow!("{style}/U+{codepoint}/structured: {error}"))?;
        }
        for &(codepoint, expected) in &relations {
            let expr = format!(
                "<math><mi>a</mi><mo>&#x{codepoint};</mo><mi>b</mi></math>"
            );
            test("zh", style, &expr, expected)
                .map_err(|error| anyhow::anyhow!("{style}/U+{codepoint}/relation: {error}"))?;
        }
        for &(codepoint, expected) in &phase_relations {
            let expr = format!(
                "<math><mi>a</mi><mo>&#x{codepoint};</mo><mi>b</mi></math>"
            );
            test("zh", style, &expr, expected)
                .map_err(|error| anyhow::anyhow!("{style}/U+{codepoint}/phase-relation: {error}"))?;
        }
        for &(codepoint, expected) in &ellipses {
            let expr = format!("<math><mo>&#x{codepoint};</mo></math>");
            test("zh", style, &expr, expected)
                .map_err(|error| anyhow::anyhow!("{style}/U+{codepoint}/ellipsis: {error}"))?;
        }
        for &(codepoint, expected) in &set_relations {
            let (left, right) = if matches!(codepoint, "22fa" | "22fb" | "22fc" | "22fd" | "22fe") {
                ("a", "x")
            } else {
                ("x", "a")
            };
            let expr = format!(
                "<math><mi>{left}</mi><mo>&#x{codepoint};</mo><mi>{right}</mi></math>"
            );
            test("zh", style, &expr, expected)
                .map_err(|error| anyhow::anyhow!("{style}/U+{codepoint}/set-relation: {error}"))?;
        }
        for &(codepoint, expected) in &integrals {
            let expr = format!("<math><mo>&#x{codepoint};</mo><mi>f</mi></math>");
            test("zh", style, &expr, expected)
                .map_err(|error| anyhow::anyhow!("{style}/U+{codepoint}/integral: {error}"))?;
        }
    }
    Ok(())
}

#[test]
fn miscellaneous_technical_symbols_have_verified_readings() -> Result<()> {
    // Cover every U+2300-U+23E7 entry not already exercised by another zh test.
    let cases = [
        ("2300", "直径"),
        ("2301", "电箭头"),
        ("2302", "房屋"),
        ("2304", "向下箭头尖"),
        ("2305", "射影"),
        ("2306", "透视"),
        ("2307", "波浪线"),
        ("230c", "右下裁切符"),
        ("230d", "左下裁切符"),
        ("230e", "右上裁切符"),
        ("230f", "左上裁切符"),
        ("2310", "反向非"),
        ("2311", "方菱形"),
        ("2312", "弧"),
        ("2313", "弓形"),
        ("2314", "扇形"),
        ("2315", "电话记录器符号"),
        ("2316", "位置指示十字线"),
        ("2317", "视图数据方框"),
        ("2318", "命令键"),
        ("2319", "倒置非"),
        ("231a", "手表"),
        ("231b", "沙漏"),
        ("231d", "右上角"),
        ("231e", "左下角"),
        ("231f", "右下角"),
        ("2320", "积分号上半部"),
        ("2321", "积分号下半部"),
        ("2322", "皱眉"),
        ("2323", "微笑"),
        ("2324", "回车键"),
        ("2325", "选项键"),
        ("2326", "向前删除键"),
        ("2328", "键盘"),
        ("2329", "左尖括号"),
        ("232a", "右尖括号"),
        ("232b", "退格键"),
        ("232c", "苯环"),
        ("232d", "圆柱度"),
        ("232e", "全周轮廓符号"),
        ("232f", "对称度"),
        ("2330", "全跳动"),
        ("2331", "尺寸原点"),
        ("2334", "沉孔"),
        ("2335", "沉头孔"),
        ("2336", "APL 功能符号工字梁"),
        ("233d", "APL 功能符号圆竖线"),
        ("233f", "APL 功能符号斜杠横线"),
        ("2370", "APL 功能符号方框问号"),
        ("237c", "带向下之字形箭头的直角"),
        ("2394", "六边形"),
        ("2395", "APL 功能符号方框"),
        ("23b4", "上置方括号"),
        ("23b5", "下置方括号"),
        ("23b6", "上方为下置方括号，下方为上置方括号"),
        ("23dd", "下置圆括号"),
        ("23de", "上置花括号"),
        ("23e2", "白色梯形"),
        ("23e4", "直线度"),
        ("23e5", "平面度"),
        ("23e7", "电路交叉点"),
    ];

    for style in ["SimpleSpeak", "ClearSpeak"] {
        for &(codepoint, expected) in &cases {
            let expr = format!("<math><mo>&#x{codepoint};</mo></math>");
            test("zh", style, &expr, expected)
                .map_err(|error| anyhow::anyhow!("{style}/U+{codepoint}: {error}"))?;
        }
    }
    Ok(())
}
