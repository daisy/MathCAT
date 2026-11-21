//! MathType and private-use symbol identity audits.

use crate::common::*;
use anyhow::Result;

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
        ("2247", "既不近似等于也不等于"),
        ("2298", "带圆圈除号斜线"),
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
        test("zh-cn", "SimpleSpeak", &expr, expected)
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
        ("225f", "问号等于"),
        ("22d8", "极小于"),
        ("22d9", "极大于"),
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
        ("2a3c", "内积"),
        ("2a3d", "右内积"),
        ("2a50", "带衬线和压缩积的闭合并集"),
        ("2a85", "小于或约等于"),
        ("2a86", "大于或约等于"),
        ("2af9", "双线倾斜小于或等于"),
        ("2afa", "双线倾斜大于或等于"),
        ("3372", "道尔顿"),
        ("3375", "o 大写 v"),
        ("fb05", "长 s t 连字"),
    ];

    for (codepoint, expected) in cases {
        let expr = format!("<math><mo>&#x{codepoint};</mo></math>");
        test("zh-cn", "SimpleSpeak", &expr, expected)
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
        ("e949", "反转波浪号"),
        ("e950", "带竖线的正规包含于"),
        ("e951", "带竖线的包含正规子群"),
        ("e982", "带方框的直角变体"),
        ("e98f", "自由基点"),
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
        ("eb01", "小型向左箭头上方的向右箭头"),
        ("eb02", "向左箭头上方的小型向右箭头"),
        ("eb03", "小型向左鱼叉箭头上方的向右鱼叉箭头"),
        ("eb04", "向左鱼叉箭头上方的小型向右鱼叉箭头"),
        ("eb0f", "带斜线的大型左右箭头"),
        ("eb11", "带斜线的大型左右双箭头"),
        ("eb12", "左侧下箭头右侧上箭头"),
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
        ("eb42", "左端带空心圆点的左右箭头"),
        ("eb4c", "粗短黑色向左箭头"),
        ("eb52", "左端带空心圆点的左右鱼叉箭头"),
        ("ed00", "约化普朗克常量"),
        ("ed01", "倒置无衬线体 大写 g"),
        ("ee1a", "左端带空心圆点的横线重音符"),
        ("ee1b", "左端带实心圆点的横线重音符"),
        ("ee1c", "右端带空心圆点的横线重音符"),
        ("ee23", "右端带实心圆点的横线重音符"),
        ("ef80", "顺时针围道积分"),
        ("ef81", "四元数积分运算符"),
        ("ef82", "带斜线的平均积分号"),
    ];

    for (codepoint, expected) in cases {
        let expr = format!("<math><mo>&#x{codepoint};</mo></math>");
        test("zh-cn", "SimpleSpeak", &expr, expected)
            .map_err(|error| anyhow::anyhow!("U+{codepoint}: {error}"))?;
    }
    Ok(())
}

#[test]
fn remaining_ea00_mathtype_private_use_symbols_match_verified_sources() -> Result<()> {
    // Audit every previously uncovered MathType PUA slot against its original character table.
    let cases = [
        (0xea00, "带竖线的非远小于"),
        (0xea01, "带竖线的非远大于"),
        (0xea02, "非远小于变体"),
        (0xea03, "非远大于变体"),
        (0xea04, "带竖线的小于且不等于"),
        (0xea05, "带竖线的大于且不等于"),
        (0xea09, "既不等于也不小于"),
        (0xea0a, "不包含也不等于"),
        (0xea0b, "不包含也不等于"),
        (0xea0c, "不包含于也不等于"),
        (0xea0d, "带反斜杠的子集符号"),
        (0xea0e, "既不等于也不大于"),
        (0xea0f, "否定减波浪运算符"),
        (0xea10, "既不等于也不小于"),
        (0xea11, "否定波浪号运算符"),
        (0xea12, "既不属于也不等于"),
        (0xea13, "既不等于也不大于"),
        (0xea14, "不约等于"),
        (0xea16, "带斜线的小于或倾斜等于"),
        (0xea17, "带斜线的大于或倾斜等于"),
        (0xea1a, "带斜杠的超集符号"),
        (0xea1b, "不包含"),
        (0xea1f, "不约等于减号"),
        (0xea22, "上加点的否定集合隶属符号"),
        (0xea2c, "带竖线的否定角符号"),
        (0xea2d, "倾斜不平行符号"),
        (0xea32, "小于但不约等于"),
        (0xea33, "大于但不约等于"),
        (0xea34, "小于或不等于"),
        (0xea35, "大于或不等于"),
        (0xea36, "非嵌套小于号"),
        (0xea37, "非嵌套大于号"),
        (0xea38, "非远小于"),
        (0xea39, "非远大于"),
        (0xea3a, "先于但不等价于"),
        (0xea3b, "后于但不等价于"),
        (0xea3c, "先于但不等于"),
        (0xea3d, "后于但不等于"),
        (0xea3e, "既不等于也不先于"),
        (0xea3f, "既不等于也不后于"),
        (0xea40, "先于但不等于"),
        (0xea41, "后于但不等于"),
        (0xea42, "不包含于也不等于"),
        (0xea43, "不包含也不等于"),
        (0xea44, "包含于或不等于"),
        (0xea45, "包含或不等于"),
        (0xea46, "不包含于也不等于"),
        (0xea47, "不包含也不等于"),
        (0xea48, "非三重小于号"),
        (0xea49, "非三重大于号"),
        (0xea4c, "不先于且不等于"),
        (0xea4d, "不后于且不等于"),
        (0xea52, "非差符号"),
        (0xea53, "几何不等价于"),
        (0xea54, "带竖线的非相似符号"),
        (0xea56, "带竖线的非约等符号"),
        (0xea57, "不近似恒等于"),
        (0xea58, "非波形等号"),
        (0xea59, "非波形单等号"),
        (0xea5a, "带点的不等号"),
        (0xea5b, "反向不等价号"),
        (0xea60, "非方形子集符号"),
        (0xea61, "非方形超集符号"),
        (0xea62, "等号上方的不约等于"),
        (0xea64, "带点的不全等符号"),
        (0xea65, "反向不等号"),
        (0xea80, "否定偏微分符号"),
        (0xeb00, "箭头修饰符延伸段"),
        (0xeb05, "右上至左下双箭头"),
        (0xeb06, "左上至右下双箭头"),
        (0xeb07, "水平鱼叉箭头延伸段"),
        (0xeb08, "逆时针弧形向左箭头"),
        (0xeb09, "逆时针弧形向右箭头"),
        (0xeb0b, "大型向右箭头重音符"),
        (0xeb0c, "大型向左箭头重音符"),
        (0xeb0d, "左箭头尖"),
        (0xeb0e, "右箭头尖"),
        (0xeb10, "水平双箭头延伸段"),
        (0xeb13, "带向下折角的向左箭头"),
        (0xeb14, "带向上折角的向右箭头"),
        (0xeb15, "带向上折角的向左箭头"),
        (0xeb16, "带加号的逆时针上半圆箭头"),
        (0xeb17, "带减号的顺时针上半圆箭头"),
        (0xeb19, "倒钩向下的向右鱼叉箭头"),
        (0xeb1a, "倒钩向下的向左鱼叉箭头"),
        (0xeb1b, "倒钩向下的左右鱼叉箭头"),
        (0xeb1c, "倒钩向上的左右鱼叉箭头"),
        (0xeb1d, "倒钩向左的上下鱼叉箭头"),
        (0xeb1e, "倒钩向右的上下鱼叉箭头"),
        (0xeb1f, "向下箭头右侧的向上箭头"),
        (0xeb20, "带倒钩向上的指向竖线的左鱼叉箭头"),
        (0xeb21, "带倒钩向上的指向竖线的右鱼叉箭头"),
        (0xeb22, "带倒钩向下的指向竖线的左鱼叉箭头"),
        (0xeb23, "带倒钩向下的指向竖线的右鱼叉箭头"),
        (0xeb24, "带倒钩向上的从竖线出发的左鱼叉箭头"),
        (0xeb25, "带倒钩向上的从竖线出发的右鱼叉箭头"),
        (0xeb26, "带倒钩向下的从竖线出发的左鱼叉箭头"),
        (0xeb27, "带倒钩向下的从竖线出发的右鱼叉箭头"),
        (0xeb28, "带倒钩向左的指向横线的上鱼叉箭头"),
        (0xeb29, "带倒钩向左的指向横线的下鱼叉箭头"),
        (0xeb2a, "带倒钩向右的指向横线的上鱼叉箭头"),
        (0xeb2b, "带倒钩向右的指向横线的下鱼叉箭头"),
        (0xeb2c, "带倒钩向左的从横线出发的上鱼叉箭头"),
        (0xeb2d, "带倒钩向左的从横线出发的下鱼叉箭头"),
        (0xeb2e, "带倒钩向右的从横线出发的上鱼叉箭头"),
        (0xeb2f, "带倒钩向右的从横线出发的下鱼叉箭头"),
        (0xeb30, "指向横线的向上箭头"),
        (0xeb31, "指向横线的向下箭头"),
        (0xeb32, "下鱼叉箭头左侧的上鱼叉箭头"),
        (0xeb33, "下鱼叉箭头右侧的上鱼叉箭头"),
        (0xeb34, "上箭头尖"),
        (0xeb35, "下箭头尖"),
        (0xeb36, "左侧倒钩向下、右侧倒钩向上的双鱼叉箭头"),
        (0xeb37, "左侧倒钩向上、右侧倒钩向下的双鱼叉箭头"),
        (0xeb38, "线上方的向左箭头"),
        (0xeb39, "线上方的向右箭头"),
        (0xeb3a, "线下方的向左箭头"),
        (0xeb3b, "线下方的向右箭头"),
        (0xeb3c, "左右三重箭头"),
        (0xeb40, "逆时针左半圆箭头"),
        (0xeb41, "顺时针左半圆箭头"),
        (0xeb44, "波浪号上方的向右箭头"),
        (0xeb45, "波浪号上方的向左箭头"),
        (0xeb48, "线上方的左鱼叉箭头"),
        (0xeb49, "线上方的右鱼叉箭头"),
        (0xeb4a, "线下方的左鱼叉箭头"),
        (0xeb4b, "线下方的右鱼叉箭头"),
        (0xeb50, "顺时针右半圆箭头"),
        (0xeb51, "逆时针右半圆箭头"),
        (0xeb58, "竖线左侧的向上箭头"),
        (0xeb59, "竖线左侧的向下箭头"),
        (0xeb5a, "竖线右侧的向上箭头"),
        (0xeb5b, "竖线右侧的向下箭头"),
        (0xeb5c, "带延长下钩的向右箭头"),
        (0xeb5d, "带延长钩的向左箭头"),
        (0xeb5e, "带延长下钩的向左箭头"),
        (0xeb5f, "带延长钩的向右箭头"),
        (0xeb68, "竖线左侧的向上鱼叉箭头"),
        (0xeb69, "竖线左侧的向下鱼叉箭头"),
        (0xeb6a, "竖线右侧的向上鱼叉箭头"),
        (0xeb6b, "竖线右侧的向下鱼叉箭头"),
        (0xeb6c, "竖直双箭头延伸段"),
        (0xeb6d, "倒钩向左的竖直鱼叉箭头延伸段"),
        (0xeb6e, "倒钩向右的竖直鱼叉箭头延伸段"),
        (0xeb6f, "左鱼叉箭头右部上方的右鱼叉箭头"),
        (0xeb70, "左鱼叉箭头左部上方的右鱼叉箭头"),
        (0xeb71, "右鱼叉箭头右部上方的左鱼叉箭头"),
        (0xeb72, "右鱼叉箭头左部上方的左鱼叉箭头"),
        (0xeb73, "从竖线出发的向左箭头尖"),
        (0xeb74, "从竖线出发的左右箭头延伸段"),
        (0xeb75, "从竖线出发的向左箭头尾部"),
        (0xeb76, "从竖线出发的向右箭头尾部"),
        (0xeb77, "从竖线出发的向右箭头尖"),
        (0xeb78, "带倒钩左箭头尖的从横线出发的上鱼叉箭头"),
        (0xeb79, "向左箭头右部上方的向右箭头"),
        (0xeb7a, "向左箭头左部上方的向右箭头"),
        (0xeb7b, "向右箭头右部上方的向左箭头"),
        (0xeb7c, "向右箭头左部上方的向左箭头"),
        (0xeb7d, "从横线出发的向上箭头尖"),
        (0xeb7e, "从横线出发的向上箭头尾部"),
        (0xeb7f, "从横线出发的向下箭头尾部"),
        (0xeb80, "从横线出发的向下箭头尖"),
        (0xeb81, "带倒钩右箭头尖的从横线出发的下鱼叉箭头"),
        (0xeb82, "下鱼叉箭头底部左侧的上鱼叉箭头"),
        (0xeb83, "下鱼叉箭头延伸段左侧的上鱼叉箭头"),
        (0xeb84, "上鱼叉箭头顶部左侧的下鱼叉箭头"),
        (0xeb85, "下鱼叉箭头顶部左侧的上鱼叉箭头"),
        (0xeb86, "上鱼叉箭头延伸段左侧的下鱼叉箭头"),
        (0xeb87, "上鱼叉箭头底部左侧的下鱼叉箭头"),
        (0xeb88, "向下箭头底部左侧的向上箭头"),
        (0xeb89, "向上箭头顶部左侧的向下箭头"),
        (0xeb8a, "向下箭头顶部左侧的向上箭头"),
        (0xeb8b, "向上箭头底部左侧的向下箭头"),
        (0xeb8c, "左右箭头延伸段"),
        (0xeb8d, "右上箭头延伸段"),
        (0xeb8e, "左上箭头延伸段"),
        (0xec00, "向下的花括号左部"),
        (0xec01, "向下的花括号中部"),
        (0xec02, "向下的花括号右部"),
        (0xec03, "水平花括号延伸段"),
        (0xec04, "向上的花括号左部"),
        (0xec05, "向上的花括号中部"),
        (0xec06, "向上的花括号右部"),
        (0xec07, "左竖线"),
        (0xec08, "右竖线"),
        (0xec09, "左双竖线"),
        (0xec0a, "右双竖线"),
        (0xec0b, "水平方括号延伸段"),
        (0xec0c, "下置方括号"),
        (0xec0d, "上置方括号"),
        (0xec0e, "下方方括号左部"),
        (0xec0f, "下方方括号右部"),
        (0xec10, "上方方括号左部"),
        (0xec11, "上方方括号右部"),
        (0xec12, "左圆括号部件一"),
        (0xec13, "左圆括号部件二"),
        (0xec14, "左圆括号部件三"),
        (0xec15, "左圆括号部件四"),
        (0xec16, "右圆括号部件一"),
        (0xec17, "右圆括号部件二"),
        (0xec18, "右圆括号部件三"),
        (0xec19, "右圆括号部件四"),
        (0xec1a, "根号部件一"),
        (0xec1b, "根号部件二"),
        (0xec1c, "根号部件三"),
        (0xec1d, "根号部件四"),
        (0xec1e, "根号部件五"),
        (0xec1f, "根号底部"),
        (0xec20, "根号竖直延伸段"),
        (0xec21, "根号顶部"),
        (0xec22, "左白色方括号顶部"),
        (0xec23, "左白色方括号延伸段"),
        (0xec24, "左白色方括号底部"),
        (0xec25, "右白色方括号顶部"),
        (0xec26, "右白色方括号延伸段"),
        (0xec27, "右白色方括号底部"),
        (0xec30, "左白色花括号"),
        (0xec31, "右白色花括号"),
        (0xec32, "长除号"),
        (0xec33, "长除号延伸段"),
        (0xec34, "短除号"),
        (0xec40, "左下至右上全长双键"),
        (0xec41, "左上至右下全长双键"),
        (0xec42, "水平全长单键"),
        (0xec43, "水平全长双键"),
        (0xec45, "竖直全长单键"),
        (0xec46, "竖直全长双键"),
        (0xec48, "小于号形全长键"),
        (0xec49, "大于号形全长键"),
        (0xec4a, "水平半长单键"),
        (0xec4b, "水平半长双键"),
        (0xec80, "矩形左上部"),
        (0xec81, "矩形左下部"),
        (0xec90, "矩形右上部"),
        (0xec91, "矩形右下部"),
        (0xec92, "综合除法角部"),
        (0xec93, "综合除法水平延伸段"),
        (0xec94, "综合除法竖直延伸段"),
        (0xec95, "左侧上下取整符号延伸段"),
        (0xec96, "右侧上下取整符号延伸段"),
        (0xec97, "上方括号延伸段"),
        (0xec98, "竖线延伸段"),
        (0xec99, "左双竖线延伸段"),
        (0xec9a, "横线延伸段"),
        (0xec9c, "下方括号延伸段"),
        (0xec9d, "向下的圆括号右部"),
        (0xec9e, "向下的圆括号延伸段"),
        (0xec9f, "向下的圆括号左部"),
        (0xeca0, "向上的花括号延伸段"),
        (0xeca1, "向上的圆括号左部"),
        (0xeca2, "向上的圆括号延伸段"),
        (0xeca3, "向上的圆括号右部"),
        (0xeca4, "向下的花括号延伸段"),
        (0xed02, "无点 j"),
        (0xed03, "digamma"),
        (0xed10, "d"),
        (0xed11, "e"),
        (0xed12, "i"),
        (0xed13, "j"),
        (0xed16, "大写 d"),
        (0xee00, "逆时针围道积分环"),
        (0xee01, "顺时针围道积分环"),
        (0xee0d, "连接状态修饰符"),
        (0xee0e, "连接状态左侧修饰符"),
        (0xee0f, "连接状态右侧修饰符"),
        (0xee10, "连接状态延伸段"),
        (0xee11, "积分环"),
        (0xee12, "双积分环"),
        (0xee13, "三重积分环"),
        (0xee15, "双扩展积分环"),
        (0xee16, "三重扩展积分环"),
        (0xee17, "渐近等于重音符"),
        (0xee18, "等号重音符"),
        (0xee19, "四撇号"),
        (0xee1d, "带上点的横线重音符"),
        (0xee1e, "带下点的横线重音符"),
        (0xee1f, "带上方双点的横线重音符"),
        (0xee20, "带下方双点的横线重音符"),
        (0xee21, "带脱字符的横线重音符"),
        (0xee22, "粗下横线重音符"),
        (0xee24, "上加大点"),
        (0xef00, "对齐标记"),
        (0xef41, "缺项"),
        (0xef83, "反向积分号"),
        (0xef90, "双零上方的双零"),
        (0xef91, "带斜线的零"),
    ];

    for (codepoint, expected) in cases {
        let expr = format!("<math><mo>&#x{codepoint:x};</mo></math>");
        test("zh-cn", "SimpleSpeak", &expr, expected)
            .map_err(|error| anyhow::anyhow!("MathType PUA U+{codepoint:04X}: {error}"))?;
    }

    // Wide accents and spacing controls are layout-only and must remain silent.
    let silent = (0xee04..=0xee0c)
        .chain(0xef01..=0xef0a)
        .chain(0xef22..=0xef24)
        .chain(std::iter::once(0xef29));
    for codepoint in silent {
        let expr = format!("<math><mo>&#x{codepoint:x};</mo></math>");
        test("zh-cn", "SimpleSpeak", &expr, "")
            .map_err(|error| anyhow::anyhow!("silent MathType PUA U+{codepoint:04X}: {error}"))?;
    }
    Ok(())
}

#[test]
fn mathtype_double_struck_greek_matches_the_original_character_table() -> Result<()> {
    // Check every MathType PUA slot because the source order is non-alphabetic in both ranges.
    let capitals = [
        ("f201", "delta"),
        ("f202", "zai"),
        ("f203", "lambda"),
        ("f204", "pi"),
        ("f205", "sigma"),
        ("f206", "theta"),
        ("f207", "gamma"),
        ("f208", "omega"),
        ("f209", "upsilon"),
    ];
    for (codepoint, name) in capitals {
        let expr = format!("<math><mi>&#x{codepoint};</mi></math>");
        let expected = format!("双线体 大写 {name}");
        test("zh-cn", "SimpleSpeak", &expr, &expected)
            .map_err(|error| anyhow::anyhow!("U+{codepoint}: {error}"))?;
    }

    let lowercase = [
        ("f220", "alpha"),
        ("f221", "beta"),
        ("f222", "phi"),
        ("f223", "zeta"),
        ("f224", "psi"),
        ("f225", "delta"),
        ("f226", "epsilon"),
        ("f227", "gamma"),
        ("f228", "eta"),
        ("f229", "iota"),
        ("f22a", "zai"),
        ("f22b", "kappa"),
        ("f22c", "lambda"),
        ("f22d", "mu"),
        ("f22e", "nu"),
        ("f22f", "epsilon"),
        ("f230", "pi"),
        ("f231", "theta"),
        ("f232", "rho"),
        ("f233", "sigma"),
        ("f234", "tau"),
        ("f235", "theta"),
        ("f236", "omega"),
    ];
    for (codepoint, name) in lowercase {
        let expr = format!("<math><mi>&#x{codepoint};</mi></math>");
        let expected = format!("双线体 {name}");
        test("zh-cn", "SimpleSpeak", &expr, &expected)
            .map_err(|error| anyhow::anyhow!("U+{codepoint}: {error}"))?;
    }

    let special = [
        ("f237", "双线体 词尾 sigma"),
        ("f250", "双线体 rho"),
        ("f251", "双线体 phi"),
    ];
    for (codepoint, expected) in special {
        let expr = format!("<math><mi>&#x{codepoint};</mi></math>");
        test("zh-cn", "SimpleSpeak", &expr, expected)
            .map_err(|error| anyhow::anyhow!("U+{codepoint}: {error}"))?;
    }
    Ok(())
}

#[test]
fn mathematical_capital_theta_symbols_remain_speakable_in_every_variant() -> Result<()> {
    // Each 25-character range has an extra theta-symbol slot where U+03A2 must not be used.
    let cases = [
        ("1d6b9", "粗体 大写 theta"),
        ("f419", "粗体 大写 theta"),
        ("1d6f3", "大写 theta"),
        ("f453", "大写 theta"),
        ("1d72d", "粗体 大写 theta"),
        ("f48d", "粗体 大写 theta"),
        ("1d767", "粗体 大写 theta"),
        ("f4c7", "粗体 大写 theta"),
        ("1d7a1", "粗体 大写 theta"),
        ("f501", "粗体 大写 theta"),
    ];

    for (codepoint, expected) in cases {
        let expr = format!("<math><mi>&#x{codepoint};</mi></math>");
        test("zh-cn", "SimpleSpeak", &expr, expected)
            .map_err(|error| anyhow::anyhow!("U+{codepoint}: {error}"))?;
    }
    Ok(())
}

#[test]
fn audited_unicode_ranges_keep_their_boundaries() -> Result<()> {
    // The first and last characters catch off-by-one and shifted translate mappings.
    let cases = [
        ("03aa", "大写 iota 带分音符"),
        ("03ab", "大写 upsilon 带分音符"),
        ("03cf", "大写 kai"),
        ("24b6", "带圈 大写 a"),
        ("24cf", "带圈 大写 z"),
        ("24d0", "带圈 a"),
        ("24e9", "带圈 z"),
        ("1d538", "双线体 大写 a"),
        ("1d550", "双线体 大写 y"),
        ("f000", "哥特体 大写 a"),
        ("f018", "哥特体 大写 y"),
        ("f01a", "哥特体 a"),
        ("f033", "哥特体 z"),
        ("f040", "哥特体 粗体 大写 a"),
        ("f059", "哥特体 粗体 大写 z"),
        ("f05a", "哥特体 粗体 a"),
        ("f073", "哥特体 粗体 z"),
        ("f080", "双线体 大写 a"),
        ("f098", "双线体 大写 y"),
        ("f09a", "双线体 a"),
        ("f0b3", "双线体 z"),
        ("f0c0", "双线体 0"),
        ("f0c9", "双线体 9"),
        ("f0ca", "双线体纳布拉"),
        ("f0cb", "双线体欧拉常数"),
        ("f100", "花体 大写 a"),
        ("f119", "花体 大写 z"),
        ("f11a", "花体 a"),
        ("f133", "花体 z"),
        ("f140", "花体 粗体 大写 a"),
        ("f159", "花体 粗体 大写 z"),
        ("f15a", "花体 粗体 a"),
        ("f173", "花体 粗体 z"),
        ("f180", "大写 a"),
        ("f199", "大写 z"),
        ("f19a", "大写 ae 连字"),
        ("f19b", "大写 德语双 s"),
        ("f19c", "大写 带斜线的 o"),
        ("f260", "粗体 大写 a"),
        ("f279", "粗体 大写 z"),
        ("f27a", "粗体 a"),
        ("f293", "粗体 z"),
        ("f294", "大写 a"),
        ("f2ad", "大写 z"),
        ("f2ae", "a"),
        ("f2c7", "z"),
        ("f2c8", "粗体 大写 a"),
        ("f2e1", "粗体 大写 z"),
        ("f2e2", "粗体 a"),
        ("f2fb", "粗体 z"),
        ("f300", "大写 a"),
        ("f319", "大写 z"),
        ("f31a", "a"),
        ("f333", "z"),
        ("f334", "粗体 大写 a"),
        ("f34d", "粗体 大写 z"),
        ("f34e", "粗体 a"),
        ("f367", "粗体 z"),
        ("f368", "大写 a"),
        ("f381", "大写 z"),
        ("f382", "a"),
        ("f39b", "z"),
        ("f39c", "粗体 大写 a"),
        ("f3b5", "粗体 大写 z"),
        ("f3b6", "粗体 a"),
        ("f3cf", "粗体 z"),
        ("f3d0", "大写 a"),
        ("f3e9", "大写 z"),
        ("f3ea", "a"),
        ("f403", "z"),
    ];

    for (codepoint, expected) in cases {
        let expr = format!("<math><mi>&#x{codepoint};</mi></math>");
        test("zh-cn", "SimpleSpeak", &expr, expected)
            .map_err(|error| anyhow::anyhow!("U+{codepoint}: {error}"))?;
    }
    Ok(())
}

#[test]
fn confirmed_greek_unicode_variants_keep_chinese_modifiers_and_english_cores() -> Result<()> {
    let cases = [
        ("02e0", "修饰用 gamma"),
        ("03ca", "带分音符的 iota"),
        ("03cb", "带分音符的 upsilon"),
        ("03cc", "带重音符的 omicron"),
        ("03cd", "带重音符的 upsilon"),
        ("03ce", "带重音符的 omega"),
        ("03d0", "beta"),
        ("03d1", "theta"),
        ("03d2", "带钩的 大写 upsilon"),
        ("03d3", "带锐音与钩的 大写 upsilon"),
        ("03d4", "带分音与钩的 大写 upsilon"),
        ("03d7", "kai"),
        ("03d8", "大写 古体 koppa"),
        ("03d9", "古体 koppa"),
        ("03dd", "digamma"),
        ("213c", "双线体 pi"),
        ("213d", "双线体 gamma"),
        ("f250", "双线体 rho"),
        ("f251", "双线体 phi"),
    ];

    for (codepoint, expected) in cases {
        let expr = format!("<math><mi>&#x{codepoint};</mi></math>");
        test("zh-cn", "SimpleSpeak", &expr, expected)
            .map_err(|error| anyhow::anyhow!("U+{codepoint}: {error}"))?;
    }
    Ok(())
}

#[test]
fn mathtype_greek_and_digit_ranges_keep_verified_character_order() -> Result<()> {
    // Check every mapped MathType slot in U+F404-U+F555, including easily shifted variants.
    let capitals = [
        "alpha",
        "beta",
        "gamma",
        "delta",
        "epsilon",
        "zeta",
        "eta",
        "theta",
        "iota",
        "kappa",
        "lambda",
        "mu",
        "nu",
        "zai",
        "omicron",
        "pi",
        "rho",
        "theta",
        "sigma",
        "tau",
        "upsilon",
        "phi",
        "chi",
        "psi",
        "omega",
    ];
    let lowercase = [
        "alpha",
        "beta",
        "gamma",
        "delta",
        "epsilon",
        "zeta",
        "eta",
        "theta",
        "iota",
        "kappa",
        "lambda",
        "mu",
        "nu",
        "zai",
        "omicron",
        "pi",
        "rho",
        "词尾 sigma",
        "sigma",
        "tau",
        "upsilon",
        "phi",
        "chi",
        "psi",
        "omega",
    ];
    let variants = ["偏导数", "epsilon", "theta", "kappa", "phi", "rho", "pi"];
    let families = [
        (0xf408, 0xf421, 0xf422, 0xf43b, "粗体 ", "粗体纳布拉"),
        (0xf442, 0xf45b, 0xf45c, 0xf475, "", "斜体纳布拉"),
        (0xf47c, 0xf495, 0xf496, 0xf4af, "粗体 ", "粗斜体纳布拉"),
        (0xf4b6, 0xf4cf, 0xf4d0, 0xf4e9, "粗体 ", "粗体纳布拉"),
        (0xf4f0, 0xf509, 0xf50a, 0xf523, "粗体 ", "粗体纳布拉"),
    ];

    let check = |codepoint: u32, expected: &str| -> Result<()> {
        let expr = format!("<math><mi>&#x{codepoint:x};</mi></math>");
        test("zh-cn", "SimpleSpeak", &expr, expected)
            .map_err(|error| anyhow::anyhow!("U+{codepoint:04X}: {error}"))
    };

    check(0xf404, "无点 i")?;
    for (capital, nabla, small, variant, prefix, nabla_name) in families {
        for (offset, name) in capitals.iter().enumerate() {
            check(capital + offset as u32, &format!("{prefix}大写 {name}"))?;
        }
        check(nabla, nabla_name)?;
        for (offset, name) in lowercase.iter().enumerate() {
            check(small + offset as u32, &format!("{prefix}{name}"))?;
        }
        for (offset, name) in variants.iter().enumerate() {
            check(variant + offset as u32, &format!("{prefix}{name}"))?;
        }
    }
    for start in [0xf52e, 0xf54c] {
        for digit in 0..=9 {
            check(start + digit, &format!("粗体 {digit}"))?;
        }
    }
    Ok(())
}

#[test]
fn remaining_mathtype_and_unicode_slots_match_verified_sources() -> Result<()> {
    // Cover the remaining digit ranges and source-identified MathType PUA glyphs slot by slot.
    for start in [0xf556, 0x1d7e2, 0x1d7f6] {
        for digit in 0..=9 {
            let codepoint = start + digit;
            let expr = format!("<math><mi>&#x{codepoint:x};</mi></math>");
            test("zh-cn", "SimpleSpeak", &expr, &digit.to_string())
                .map_err(|error| anyhow::anyhow!("U+{codepoint:04X}: {error}"))?;
        }
    }

    let cases = [
        (0xf700, "未知字符"),
        (0xf726, "左下和右下三角形"),
        (0xf72d, "水平省略号延伸段"),
        (0xf72e, "中线水平省略号延伸段"),
        (0xf8e5, "根号延伸段"),
        (0xf8e6, "竖直箭头延伸段"),
        (0xf8e7, "水平箭头延伸段"),
        (0xf8e8, "无衬线体注册符号"),
        (0xf8e9, "无衬线体版权符号"),
        (0xf8ea, "无衬线体商标符号"),
        (0xf8eb, "左圆括号顶部"),
        (0xf8ec, "左圆括号延伸段"),
        (0xf8ed, "左圆括号底部"),
        (0xf8ee, "左方括号顶部"),
        (0xf8ef, "左方括号延伸段"),
        (0xf8f0, "左方括号底部"),
        (0xf8f1, "左花括号顶部"),
        (0xf8f2, "左花括号中部"),
        (0xf8f3, "左花括号底部"),
        (0xf8f4, "花括号延伸段"),
        (0xf8f5, "积分号延伸段"),
        (0xf8f6, "右圆括号顶部"),
        (0xf8f7, "右圆括号延伸段"),
        (0xf8f8, "右圆括号底部"),
        (0xf8f9, "右方括号顶部"),
        (0xf8fa, "右方括号延伸段"),
        (0xf8fb, "右方括号底部"),
        (0xf8fc, "右花括号顶部"),
        (0xf8fd, "右花括号中部"),
        (0xf8fe, "右花括号底部"),
        (0xf8ff, "苹果标志"),
    ];
    for (codepoint, expected) in cases {
        let expr = format!("<math><mo>&#x{codepoint:x};</mo></math>");
        test("zh-cn", "SimpleSpeak", &expr, expected)
            .map_err(|error| anyhow::anyhow!("U+{codepoint:04X}: {error}"))?;
    }
    Ok(())
}

#[test]
fn remaining_ligatures_combining_halves_and_small_forms_match_unicode() -> Result<()> {
    // Cover the compatibility and combining-form names not already checked by focused tests.
    let cases = [
        ("fb00", "ff 连字"),
        ("fb01", "fi 连字"),
        ("fb02", "fl 连字"),
        ("fb03", "ffi 连字"),
        ("fb04", "ffl 连字"),
        ("fb06", "st 连字"),
        ("fb29", "希伯来文替代加号"),
        ("fe20", "上加连字左半部"),
        ("fe21", "上加连字右半部"),
        ("fe22", "上加双波浪号左半部"),
        ("fe23", "上加双波浪号右半部"),
        ("fe24", "上加长音符左半部"),
        ("fe25", "上加长音符右半部"),
        ("fe26", "上加连接长音符"),
        ("fe61", "小型星号"),
        ("fe62", "小型加号"),
        ("fe63", "小型连字符减号"),
        ("fe66", "小型等号"),
    ];
    for (codepoint, expected) in cases {
        let expr = format!("<math><mo>&#x{codepoint};</mo></math>");
        test("zh-cn", "SimpleSpeak", &expr, expected)
            .map_err(|error| anyhow::anyhow!("U+{codepoint}: {error}"))?;
    }
    Ok(())
}

#[test]
fn remaining_fullwidth_and_replacement_characters_have_verified_readings() -> Result<()> {
    // Cover the final Unicode compatibility characters, including NVDA's established caret term.
    let cases = [
        ("ff0b", "全角加号"),
        ("ff1c", "小于"),
        ("ff1d", "等于"),
        ("ff1e", "大于"),
        ("ff3c", "反斜杠"),
        ("ff3e", "全角脱字符"),
        ("ff5c", "竖线"),
        ("ff5e", "波浪号"),
        ("ffe2", "非"),
        ("ffe9", "向左箭头"),
        ("ffea", "向上箭头"),
        ("ffeb", "向右箭头"),
        ("ffec", "向下箭头"),
        ("fffc", "未知或缺失的对象"),
        ("fffd", "缺失字符"),
    ];
    for (codepoint, expected) in cases {
        let expr = format!("<math><mo>&#x{codepoint};</mo></math>");
        test("zh-cn", "SimpleSpeak", &expr, expected)
            .map_err(|error| anyhow::anyhow!("U+{codepoint}: {error}"))?;
    }
    test(
        "zh-cn",
        "SimpleSpeak",
        "<math><mover accent='true'><mi>x</mi><mo>&#xff3e;</mo></mover></math>",
        "x 上方有 帽符",
    )?;
    Ok(())
}
