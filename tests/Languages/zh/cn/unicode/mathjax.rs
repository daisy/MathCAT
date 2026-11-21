//! MathJax private-use and compatibility symbol audits.

use crate::common::*;
use anyhow::Result;

#[test]
fn mathjax_v4_private_use_chemistry_symbols_match_their_source_semantics() -> Result<()> {
    // MathJax mhchem emits these PUA code points for partial bonds and arrows.
    let cases = [
        ("e410", "部分单键"),
        ("e411", "部分双键"),
        ("e412", "部分三键"),
        ("e413", "部分三键"),
        ("e428", "长向左箭头"),
        ("e429", "长向右箭头"),
        ("e42a", "长左右箭头"),
        ("e408", "可逆反应"),
        ("e409", "平衡偏右"),
        ("e40a", "平衡偏左"),
        ("e42b", "上方右箭头下方左箭头"),
        ("e42c", "向左箭头"),
        ("e42d", "向右箭头"),
        ("e42e", "左右箭头"),
    ];
    for (codepoint, expected) in cases {
        let expr = format!(
            "<math><mo mathvariant='-mhchem' data-mjx-texclass='REL'>&#x{codepoint};</mo></math>"
        );
        test("zh-cn", "SimpleSpeak", &expr, expected)
            .map_err(|error| anyhow::anyhow!("MathJax PUA U+{codepoint}: {error}"))?;
    }

    for (codepoint, latex, expected) in [
        ("e408", r"\mhchemlongrightleftharpoons", "大写 h 可逆反应 大写 i"),
        ("e409", r"\mhchemlongRightleftharpoons", "大写 h 平衡偏右 大写 i"),
        ("e40a", r"\mhchemlongLeftrightharpoons", "大写 h 平衡偏左 大写 i"),
    ] {
        let expr = format!(
            "<math><mrow data-mjx-texclass='ORD'><mi mathvariant='normal'>H</mi><mo mathvariant='-mhchem' data-mjx-texclass='REL' stretchy='true' data-latex='{latex}'>&#x{codepoint};</mo><mi mathvariant='normal'>I</mi></mrow></math>"
        );
        test("zh-cn", "SimpleSpeak", &expr, expected)
            .map_err(|error| anyhow::anyhow!("MathJax chemistry U+{codepoint}: {error}"))?;
    }
    Ok(())
}

#[test]
fn enclosed_alphanumeric_names_use_consistent_mainland_terms() -> Result<()> {
    // Negative circled capitals and double-circled digits must match NVDA's established terms.
    let cases = [
        ("1f150", "带圈反白 大写 a"),
        ("1f169", "带圈反白 大写 z"),
        ("24f5", "双圈 1"),
        ("24fd", "双圈 9"),
        ("24fe", "双圈数字十"),
    ];

    for style in ["SimpleSpeak", "ClearSpeak"] {
        for (codepoint, expected) in cases {
            let expr = format!("<math><mi>&#x{codepoint};</mi></math>");
            test("zh-cn", style, &expr, expected)
                .map_err(|error| anyhow::anyhow!("{style}/U+{codepoint}: {error}"))?;
        }
    }
    Ok(())
}

#[test]
fn unicode_dingbats_editorial_marks_and_compatibility_units_are_precise() -> Result<()> {
    // Less common symbols still need exact names because visual context is unavailable to speech users.
    // U+33C2/U+33D8 decompose to a.m./p.m.; they are not atto-/picometre units.
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
        ("33da", "公关"),
        ("33d4", "毫巴"),
        ("33c7", "公司"),
        ("33cd", "株式会社"),
        ("33ff", "伽"),
        ("fe64", "小型小于号"),
        ("fe65", "小型大于号"),
    ];

    for (codepoint, expected) in cases {
        let expr = format!("<math><mo>&#x{codepoint};</mo></math>");
        test("zh-cn", "SimpleSpeak", &expr, expected)
            .map_err(|error| anyhow::anyhow!("U+{codepoint}: {error}"))?;
    }
    Ok(())
}

#[test]
fn gallon_unit_remains_distinct_from_the_gal_acceleration_symbol() -> Result<()> {
    // The unit token "gal" means gallon; U+33FF is the Gal acceleration unit and is tested above.
    test(
        "zh-cn",
        "SimpleSpeak",
        "<math><mn>1</mn><mi intent=':unit'>gal</mi></math>",
        "1 加仑",
    )
}
