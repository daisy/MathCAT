//! Basic Unicode names and visible symbol features.

use crate::common::*;
use anyhow::Result;

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
        ("210e", "普朗克常量"),
        ("210f", "约化普朗克常量"),
        ("2114", "磅符号"),
        ("2116", "序号符号"),
        ("2127", "姆欧"),
        ("2129", "倒置 iota"),
        ("214e", "倒置 f"),
        ("02b0", "修饰用 h"),
        ("02b1", "带钩的修饰用 h"),
        ("02b2", "修饰用 j"),
        ("02b3", "修饰用 r"),
        ("02b4", "修饰用倒置 r"),
        ("02b5", "带钩的修饰用倒置 r"),
        ("02b7", "修饰用 w"),
        ("02b8", "修饰用 y"),
        ("02e1", "修饰用 l"),
        ("02e2", "修饰用 s"),
        ("02e3", "修饰用 x"),
        ("223c", "波浪运算符"),
        ("223d", "反转波浪号"),
        ("2240", "圈积"),
        ("2244", "不渐近等于"),
        ("2246", "近似但不等于"),
        ("2257", "圆圈等于"),
        ("225c", "delta 等于"),
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
        test("zh-cn", "SimpleSpeak", &expr, expected)
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
        ("21f5", "左侧下箭头右侧上箭头"),
        ("21ea", "从横线向上的白色箭头"),
        ("25cd", "竖直线填充的圆"),
        ("25d4", "右上象限为黑色的圆"),
        ("25d5", "除左上象限外为黑色的圆"),
        ("25d9", "反白圆"),
        ("25da", "上半反白圆"),
        ("25db", "下半反白圆"),
        ("25e6", "空心圆点"),
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
        test("zh-cn", "SimpleSpeak", &expr, expected)
            .map_err(|error| anyhow::anyhow!("U+{codepoint}: {error}"))?;
    }
    Ok(())
}

#[test]
fn cjk_brackets_and_dashes_use_standard_simplified_names() -> Result<()> {
    // Match the Unicode CLDR zh TTS names for CJK brackets and the wavy dash.
    let cases = [
        ("3008", "左尖括号"),
        ("3009", "右尖括号"),
        ("300a", "左双尖括号"),
        ("300b", "右双尖括号"),
        ("300c", "左角括号"),
        ("300d", "右角括号"),
        ("300e", "左中空角括号"),
        ("300f", "右中空角括号"),
        ("3010", "左黑色透镜状方括号"),
        ("3011", "右黑色透镜状方括号"),
        ("3016", "左中空透镜状方括号"),
        ("3017", "右中空透镜状方括号"),
        ("301c", "波浪号"),
        ("3030", "波浪型破折号"),
    ];

    for (codepoint, expected) in cases {
        let expr = format!("<math><mo>&#x{codepoint};</mo></math>");
        test("zh-cn", "SimpleSpeak", &expr, expected)
            .map_err(|error| anyhow::anyhow!("U+{codepoint}: {error}"))?;
    }
    Ok(())
}

#[test]
fn unicode_dash_and_quill_bracket_names_follow_mainland_usage() -> Result<()> {
    // CLDR zh uses 连接号、破折号、横条 for U+2013..U+2015. The quill
    // brackets use the same 羽毛笔 wording as U+2E20/U+2E21 above.
    let cases = [
        ("2013", "连接号"),
        ("2014", "破折号"),
        ("2015", "横条"),
        ("2045", "带羽毛笔的左方括号"),
        ("2046", "带羽毛笔的右方括号"),
    ];
    for (codepoint, expected) in cases {
        let expr = format!("<math><mo>&#x{codepoint};</mo></math>");
        test("zh-cn", "SimpleSpeak", &expr, expected)
            .map_err(|error| anyhow::anyhow!("U+{codepoint}: {error}"))?;
    }
    Ok(())
}

#[test]
fn unicode_punctuation_and_currency_names_use_mainland_terms() -> Result<()> {
    // “数字线” is the Chinese name used for FIGURE DASH; the other names
    // avoid Traditional wording and distinguish a minus sign from a negative sign.
    let cases = [
        ("2012", "数字线"),
        ("2040", "字符连接符"),
        ("2052", "商业减号"),
        ("20b6", "图尔里弗尔"),
        ("20b8", "坚戈符号"),
    ];
    for (codepoint, expected) in cases {
        let expr = format!("<math><mo>&#x{codepoint};</mo></math>");
        test("zh-cn", "SimpleSpeak", &expr, expected)
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
        test("zh-cn", "SimpleSpeak", &expr, expected)
            .map_err(|error| anyhow::anyhow!("U+{codepoint}: {error}"))?;
    }
    Ok(())
}

// Uppercase cues follow the reader's preference, including for Greek and legacy glyphs.
#[test]
fn capital_letter_preference_controls_symbol_speech() -> Result<()> {
    for (symbol, with_cap, without_cap) in [
        ("Ϙ", "大写 古体 koppa", "古体 koppa"),
        ("ϙ", "古体 koppa", "古体 koppa"),
        ("ϒ", "带钩的 大写 upsilon", "带钩的 upsilon"),
        ("ϓ", "带锐音与钩的 大写 upsilon", "带锐音与钩的 upsilon"),
        ("ϔ", "带分音与钩的 大写 upsilon", "带分音与钩的 upsilon"),
        ("㍵", "o 大写 v", "o v"),
        ("&#xed01;", "倒置无衬线体 大写 g", "倒置无衬线体 g"),
        ("ʶ", "修饰用倒置小型大写 R", "修饰用倒置小型大写 R"),
    ] {
        let expr = format!("<math><mi>{symbol}</mi></math>");
        for (setting, expected) in [("true", with_cap), ("false", without_cap)] {
            test_prefs("zh-cn", "SimpleSpeak", vec![("CapitalLetters_UseWord", setting)], &expr, expected)?;
        }
    }
    return Ok(());
}
