use crate::common::*;
use anyhow::Result;

/// Verifies that basic arithmetic operators use the seeded Japanese names.
#[test]
fn arithmetic_operators() -> Result<()> {
    let expr = "<math><mn>1</mn><mo>+</mo><mn>2</mn><mo>&#x2212;</mo><mn>3</mn><mo>=</mo><mn>0</mn></math>";
    test("ja", "ClearSpeak", expr, "1 プラス 2 マイナス 3; イコール 0")?;
    return Ok(());
}

/// A fraction of two numbers is read denominator-first in Japanese:
/// 21/22 is "22 分の 21", literally "of 22, 21". Reading it the other way round
/// says 22/21.
#[test]
fn simple_fraction() -> Result<()> {
    let expr = "<math><mfrac><mn>21</mn><mn>22</mn></mfrac></math>";
    test("ja", "ClearSpeak", expr, "22 分の 21")?;
    test("ja", "SimpleSpeak", expr, "22 分の 21")?;
    return Ok(());
}

/// The denominator-first pattern is not limited to the small numbers that English
/// has ordinals for ("three fourths"); it is how any two numbers are read.
#[test]
fn numeric_fraction_large_denominator() -> Result<()> {
    let expr = "<math><mfrac><mn>3</mn><mn>128</mn></mfrac></math>";
    test("ja", "ClearSpeak", expr, "128 分の 3")?;
    return Ok(());
}

/// When the parts are not plain numbers, Japanese keeps the written order and
/// borrows the English preposition as "オーバー" instead (Yamaguchi et al. 1996).
#[test]
fn fraction_of_variables() -> Result<()> {
    let expr = "<math><mfrac><mi>x</mi><mi>y</mi></mfrac></math>";
    test("ja", "ClearSpeak", expr, "x オーバー y")?;
    test("ja", "SimpleSpeak", expr, "x オーバー y")?;
    return Ok(());
}

/// Verifies that a square root uses the seeded Japanese root wording.
#[test]
fn square_root() -> Result<()> {
    let expr = "<math><msqrt><mn>9</mn></msqrt></math>";
    test("ja", "ClearSpeak", expr, "平方根 オブ 9")?;
    return Ok(());
}

/// An exponent is read "<base> の <exponent> 乗"; 乗 closes it. Japanese has no
/// ordinal form here, so 2 is 2 and not "second".
#[test]
fn squared() -> Result<()> {
    let expr = "<math><msup><mn>3</mn><mn>2</mn></msup></math>";
    test("ja", "ClearSpeak", expr, "3 の 2 乗")?;
    test("ja", "SimpleSpeak", expr, "3 の 2 乗")?;
    return Ok(());
}

/// Same shape for a cube.
#[test]
fn cubed() -> Result<()> {
    let expr = "<math><msup><mn>5</mn><mn>3</mn></msup></math>";
    test("ja", "ClearSpeak", expr, "5 の 3 乗")?;
    test("ja", "SimpleSpeak", expr, "5 の 3 乗")?;
    return Ok(());
}

/// The pattern does not change for exponents above three.
#[test]
fn integer_exponent() -> Result<()> {
    let expr = "<math><msup><mi>x</mi><mn>5</mn></msup></math>";
    test("ja", "ClearSpeak", expr, "x の 5 乗")?;
    test("ja", "SimpleSpeak", expr, "x の 5 乗")?;
    return Ok(());
}

/// A variable exponent is read the same way (English adds "-th" here; Japanese does not).
#[test]
fn variable_exponent() -> Result<()> {
    let expr = "<math><msup><mi>x</mi><mi>n</mi></msup></math>";
    test("ja", "ClearSpeak", expr, "x の n 乗")?;
    test("ja", "SimpleSpeak", expr, "x の n 乗")?;
    return Ok(());
}

/// A complex exponent is read as a superscript instead, with an explicit close:
/// 「の上付き … 上付き終了」. Reading it as 乗 would end a nested exponent 「… 乗 乗」.
#[test]
fn complex_exponent() -> Result<()> {
    let expr = "<math><msup><mi>x</mi><mrow><mi>y</mi><mo>+</mo><mn>1</mn></mrow></msup></math>";
    test("ja", "SimpleSpeak", expr, "x の上付き y プラス 1 上付き終了")?;
    return Ok(());
}

/// A decimal exponent is read with 乗 like an integer one, as SimpleSpeak already reads it.
/// ClearSpeak read x^2.0 as a superscript (x の上付き 2.0 上付き終了), because the English rule
/// keeps decimals away from its ordinals; Japanese has no ordinal here, so all Exponents values
/// say 乗. The outer exponent of a nested one is still read as a superscript.
#[test]
fn decimal_exponent() -> Result<()> {
    let expr = "<math><msup><mi>x</mi><mn>2.0</mn></msup></math>";
    test("ja", "ClearSpeak", expr, "x の 2.0 乗")?;
    test("ja", "SimpleSpeak", expr, "x の 2.0 乗")?;
    let expr = "<math><msup><mi>x</mi><msup><mi>y</mi><mn>2.5</mn></msup></msup></math>";
    test("ja", "ClearSpeak", expr, "x の上付き, y の 2.5 乗, 上付き終了")?;
    let expr = "<math><msup><mn>3</mn><mn>5.0</mn></msup></math>";
    test_ClearSpeak("ja", "ClearSpeak_Exponents", "Ordinal", expr, "3 の 5.0 乗")?;
    test_ClearSpeak("ja", "ClearSpeak_Exponents", "OrdinalPower", expr, "3 の 5.0 乗")?;
    test_ClearSpeak("ja", "ClearSpeak_Exponents", "AfterPower", expr, "3 の 5.0 乗")?;
    return Ok(());
}

/// A leading minus is read マイナス, the same word as the binary operator.
/// 負の / 正の name the *kind* of number (負の数 = "the negative numbers") and are
/// not how −5 is read aloud.
#[test]
fn negative_number() -> Result<()> {
    let expr = "<math><mo>&#x2212;</mo><mn>5</mn></math>";
    test("ja", "ClearSpeak", expr, "マイナス 5")?;
    test("ja", "SimpleSpeak", expr, "マイナス 5")?;
    return Ok(());
}

/// Verifies both Japanese gradient readings selected by verbosity.
#[test]
fn gradient() -> Result<()> {
    let expr = "<math><mo>&#x2207;</mo><mi mathvariant='normal'>F</mi></math>";
    test_prefs("ja", "SimpleSpeak", vec![("Verbosity", "Terse")], expr, "デル おおもじ f")?;
    test_prefs("ja", "SimpleSpeak", vec![("Verbosity", "Verbose")], expr, "勾配 オブ おおもじ f")?;
    return Ok(());
}

/// These four reach the generic function-application rule through the
/// function= entries in definitions.yaml, so they take オブ like any other
/// function. SharedRules/calculus.yaml also names them, but neither ja nor en
/// includes that file today, so it is not the path under test here.
#[test]
fn vector_calculus_operators() -> Result<()> {
    for (intent, expected) in [
        ("curl", "回転 オブ x"),
        ("divergence", "発散 オブ x"),
        ("gradient", "勾配 オブ x"),
        ("laplacian", "ラプラシアン オブ x"),
    ] {
        let expr = format!("<math><mrow intent='{intent}($x)'><mi arg='x'>x</mi></mrow></math>");
        test("ja", "ClearSpeak", &expr, expected)?;
    }
    return Ok(());
}

/// Verifies that multiplication and division symbols use Japanese operator names.
#[test]
fn multiplication_and_division() -> Result<()> {
    let expr = "<math><mn>6</mn><mo>&#x00D7;</mo><mn>4</mn><mo>&#x00F7;</mo><mn>2</mn></math>";
    test("ja", "ClearSpeak", expr, "6 掛ける 4 わる 2")?;
    return Ok(());
}

/// A "/" between units said 割る ("divided by"); en says per here. It is now 毎, and before an
/// hour or minute unit it is 毎時 or 毎分 and the unit itself is not spoken (毎 時間 is "every hour").
/// A "/" that is not next to a unit still says わる.
#[test]
fn slash_between_units() -> Result<()> {
    for (expr, expected) in [
        (r#"<math><mn>5</mn><mi intent=":unit">m</mi><mo>/</mo><mi intent=":unit">h</mi></math>"#, "5 メートル 毎時"),
        (r#"<math><mn>10</mn><mi intent=":unit">m</mi><mo>/</mo><mi intent=":unit">min</mi></math>"#, "10 メートル 毎分"),
        (r#"<math><mn>3</mn><mi intent=":unit">m</mi><mo>/</mo><mi intent=":unit">s</mi></math>"#, "3, メートル 毎 秒"),
        (r#"<math><mrow><mi>€</mi><mn>5</mn><mo>/</mo><mi mathvariant="normal">l</mi></mrow></math>"#, "5 ユーロ 毎 リットル"),
        (r#"<math><mi>x</mi><mo>/</mo><mi>h</mi></math>"#, "x わる h"),
        // the hour is still spoken when it is squared
        (r#"<math><mn>5</mn><mi intent=":unit">m</mi><mo>/</mo><msup><mi intent=":unit">h</mi><mn>2</mn></msup></math>"#, "5, メートル 毎 時間 の 2 乗"),
        (r#"<math><mn>5</mn><mi intent=":unit">m</mi><mo>/</mo><mi intent=":unit">hr</mi></math>"#, "5 メートル 毎時"),
        // the division slash U+2215 is read the same way
        (r#"<math><mn>5</mn><mi intent=":unit">m</mi><mo>&#x2215;</mo><mi intent=":unit">h</mi></math>"#, "5 メートル 毎時"),
        (r#"<math><mi>a</mi><mo>&#x2215;</mo><mi>b</mi></math>"#, "エー わる b"),
        (r#"<math><mrow intent=":literal"><mi>a</mi><mo>&#x2215;</mo><mi>h</mi></mrow></math>"#, "エー スラッシュ h"),
    ] {
        test("ja", "SimpleSpeak", expr, expected)?;
        test("ja", "ClearSpeak", expr, expected)?;
    }
    return Ok(());
}

/// Verifies that explicit parentheses retain Japanese opening and closing cues.
#[test]
fn parenthesized_expression() -> Result<()> {
    let expr = "<math><mrow><mo>(</mo><mn>1</mn><mo>+</mo><mn>2</mn><mo>)</mo></mrow></math>";
    test("ja", "ClearSpeak", expr, "丸カッコ, 1 プラス 2, 丸カッコ閉じ")?;
    return Ok(());
}

/// Square and curly brackets follow the same pattern: the opening bracket names
/// the shape and the closing one adds the postposed cue (Yamaguchi et al. 1996).
#[test]
fn square_and_curly_brackets() -> Result<()> {
    let square = "<math><mrow><mo>[</mo><mn>1</mn><mo>+</mo><mn>2</mn><mo>]</mo></mrow></math>";
    test("ja", "ClearSpeak", square, "角かっこ, 1 プラス 2, 角かっこ閉じ")?;
    let curly = "<math><mrow><mo>{</mo><mn>1</mn><mo>+</mo><mn>2</mn><mo>}</mo></mrow></math>";
    test("ja", "ClearSpeak", curly, "中カッコ, 1 プラス 2, 中カッコ閉じ")?;
    return Ok(());
}

/// Verifies the Japanese ClearSpeak wording for an absolute value.
#[test]
fn absolute_value() -> Result<()> {
    let expr = "<math><mrow><mo>|</mo><mi>x</mi><mo>|</mo></mrow></math>";
    test("ja", "ClearSpeak", expr, "絶対値 x")?;
    return Ok(());
}

/// With AbsEnd the closing cue is spoken after the contents, not before it, so
/// the bar that ends the group is heard where it actually is.
#[test]
fn absolute_value_abs_end() -> Result<()> {
    let expr = "<math><mrow><mo>|</mo><mi>x</mi><mo>|</mo></mrow></math>";
    test_ClearSpeak("ja", "ClearSpeak_AbsoluteValue", "AbsEnd", expr, "絶対値 x, 絶対値 閉じ")?;
    return Ok(());
}

/// The four bounded interval forms. English says "not including c or d"; a slot-by-slot
/// translation would use または and change the meaning, because negation does not
/// distribute over a Japanese disjunction. Each endpoint takes its own verb instead,
/// with the first in the continuative form.
#[test]
fn interval_open_open() -> Result<()> {
    let expr = "<math><mrow><mo>(</mo><mrow><mi>c</mi><mo>,</mo><mi>d</mi></mrow><mo>)</mo></mrow></math>";
    test_ClearSpeak("ja", "ClearSpeak_Paren", "Interval", expr,
        "区間 c から d まで, c を含まず d を含まない")?;
    return Ok(());
}

#[test]
fn interval_closed_closed() -> Result<()> {
    let expr = "<math><mrow><mo>[</mo><mrow><mi>c</mi><mo>,</mo><mi>d</mi></mrow><mo>]</mo></mrow></math>";
    test_ClearSpeak("ja", "ClearSpeak_Paren", "Interval", expr,
        "区間 c から d まで, c を含み d を含む")?;
    return Ok(());
}

#[test]
fn interval_closed_open() -> Result<()> {
    let expr = "<math><mrow><mo>[</mo><mrow><mi>c</mi><mo>,</mo><mi>d</mi></mrow><mo>)</mo></mrow></math>";
    test_ClearSpeak("ja", "ClearSpeak_Paren", "Interval", expr,
        "区間 c から d まで, c を含み d を含まない")?;
    return Ok(());
}

#[test]
fn interval_open_closed() -> Result<()> {
    let expr = "<math><mrow><mo>(</mo><mrow><mi>c</mi><mo>,</mo><mi>d</mi></mrow><mo>]</mo></mrow></math>";
    test_ClearSpeak("ja", "ClearSpeak_Paren", "Interval", expr,
        "区間 c から d まで, c を含まず d を含む")?;
    return Ok(());
}

/// SimpleSpeak names the kind of interval. The name was read in English
/// ("open interval から c に d"), and から and まで now follow the endpoints.
/// (a, b] is 左開右閉区間 and [a, b) is 左閉右開区間; 左半開区間 is avoided because
/// authors differ on which side it means. 開 is written かい because Open JTalk
/// reads 開区間 as ひらきくかん.
#[test]
fn simplespeak_interval_names_are_japanese() -> Result<()> {
    let cases = [
        ("open-interval", "(", ")", "かい区間"),
        ("closed-interval", "[", "]", "閉区間"),
        ("open-closed-interval", "(", "]", "左かい右閉区間"),
        ("closed-open-interval", "[", ")", "左閉右かい区間"),
    ];
    for (intent, open, close, name) in cases {
        let expr = format!("<math><mrow intent='{intent}($start, $end)'><mo>{open}</mo><mi arg='start'>c</mi><mo>,</mo><mi arg='end'>d</mi><mo>{close}</mo></mrow></math>");
        test("ja", "SimpleSpeak", &expr, &format!("{name} c から d まで"))?;
        test_prefs("ja", "SimpleSpeak", vec![("Verbosity", "Terse")], &expr, &format!("{name} c コンマ d"))?;
    }
    return Ok(());
}

/// 循環節 is the dictionary term for the repeating digits of a recurring decimal; Open JTalk reads
/// the kanji ジュンカンブシ, so 節 is written せつ.
#[test]
fn repeating_decimal_says_junkansetsu() -> Result<()> {
    let expr = "<math><mn>0.1</mn><mo>&#x2062;</mo><mover><mn>23</mn><mo>¯</mo></mover></math>";
    test("ja", "SimpleSpeak", expr, "0.1 循環せつ 2 3")?;
    test("ja", "ClearSpeak", expr, "0.1 循環せつ 2 3")?;
    return Ok(());
}

/// Cov is 共分散 at every verbosity, as exp is 指数関数 at every verbosity.
#[test]
fn covariance_is_kyoubunsan() -> Result<()> {
    let expr = "<math><mrow><mi>Cov</mi><mi>x</mi></mrow></math>";
    test("ja", "SimpleSpeak", expr, "共分散 x")?;
    test_prefs("ja", "SimpleSpeak", vec![("Verbosity", "Terse")], expr, "共分散 x")?;
    return Ok(());
}

/// Verifies that an indexed cube root receives the Japanese cube-root cue.
#[test]
fn cube_root() -> Result<()> {
    let expr = "<math><mroot><mn>8</mn><mn>3</mn></mroot></math>";
    test("ja", "ClearSpeak", expr, "立方根 オブ 8")?;
    return Ok(());
}

/// An n-th root is 「n 乗根」 -- the index, then 乗根. English builds an ordinal
/// ("the fifth root"); Japanese has no such form and reads the number as it is.
/// 乗根 is written in kana: Open JTalk does not know the word and reads the kanji
/// ジョーネ (also in 4乗根, with no space), while じょうこん is read ジョーコン.
#[test]
fn nth_root() -> Result<()> {
    let expr = "<math><mroot><mi>x</mi><mn>5</mn></mroot></math>";
    test("ja", "ClearSpeak", expr, "5 じょうこん オブ x")?;
    test("ja", "SimpleSpeak", expr, "5 じょうこん オブ x")?;
    return Ok(());
}

/// The same holds when the index is a variable (English appends "-th" here).
#[test]
fn variable_index_root() -> Result<()> {
    let expr = "<math><mroot><mi>x</mi><mi>n</mi></mroot></math>";
    test("ja", "ClearSpeak", expr, "n じょうこん オブ x")?;
    test("ja", "SimpleSpeak", expr, "n じょうこん オブ x")?;
    return Ok(());
}

/// The fourth-root sign ∜ is read like an mroot of index 4, with オブ as for √.
#[test]
fn fourth_root_sign() -> Result<()> {
    let expr = "<math><mo>&#x221C;</mo><mi>x</mi></math>";
    test("ja", "ClearSpeak", expr, "4 じょうこん オブ x")?;
    test("ja", "SimpleSpeak", expr, "4 じょうこん オブ x")?;
    return Ok(());
}

/// The overview rules (the "describe" command) have their own root rule.
#[test]
fn overview_nth_root() -> Result<()> {
    set_rules_dir(abs_rules_dir_path())?;
    set_preference("Language", "ja")?;
    set_preference("SpeechStyle", "SimpleSpeak")?;
    set_preference("Verbosity", "Medium")?;
    set_mathml("<math><mroot><mi>x</mi><mn>5</mn></mroot></math>")?;
    let spoken = get_overview_text()?;
    assert_eq!("5 じょうこん オブ x", spoken.trim_end_matches([' ', ',', ';']));
    return Ok(());
}

/// The overview rules (the "describe" command) shorten a root, a fraction and a long
/// row: a root whose radicand is not simple is just 平方根, a fraction whose parts are
/// not simple is just 分数, and a long row stops after five items with 以下同様.
#[test]
fn overview_short_forms() -> Result<()> {
    for (expr, expected) in [
        ("<math><msqrt><mi>x</mi><mo>+</mo><mn>1</mn></msqrt></math>", "平方根"),
        ("<math><mfrac><mrow><mi>x</mi><mo>+</mo><mn>1</mn></mrow><mn>2</mn></mfrac></math>", "分数"),
        ("<math><mi>a</mi><mo>+</mo><mi>b</mi><mo>+</mo><mi>c</mi><mo>+</mo><mi>d</mi></math>", "エー プラス b プラス c 以下同様"),
    ] {
        set_rules_dir(abs_rules_dir_path())?;
        set_preference("Language", "ja")?;
        set_preference("SpeechStyle", "SimpleSpeak")?;
        set_preference("Verbosity", "Medium")?;
        set_mathml(expr)?;
        let spoken = get_overview_text()?;
        assert_eq!(expected, spoken.trim_end_matches([' ', ',', ';']));
    }
    return Ok(());
}

/// Verifies the basic Japanese SimpleSpeak subscript pattern.
#[test]
fn subscript() -> Result<()> {
    let expr = "<math><msub><mi>x</mi><mn>1</mn></msub></math>";
    test("ja", "SimpleSpeak", expr, "x 下付き 1")?;
    return Ok(());
}

/// Verifies that a common trigonometric function is spoken in Japanese SimpleSpeak.
#[test]
fn sine_function() -> Result<()> {
    let expr = "<math><mi>sin</mi><mo>&#x2061;</mo><mi>x</mi></math>";
    test("ja", "SimpleSpeak", expr, "サイン オブ x")?;
    return Ok(());
}

/// Verifies Japanese speech for the less-than comparison operator.
/// No は before the relation: = is already x イコール 5, and 小なり on its own means "less than".
#[test]
fn less_than() -> Result<()> {
    let expr = "<math><mi>x</mi><mo>&lt;</mo><mn>5</mn></math>";
    test("ja", "ClearSpeak", expr, "x 小なり 5")?;
    return Ok(());
}

/// 小なり / 大なり are the pair the source gives for < and >. より大きい stranded the
/// より, which needs its comparand in front of it, so "x は より大きい 5" read as
/// "x is a bigger 5"; and it did not match the 小なり already used for <.
#[test]
fn greater_than() -> Result<()> {
    let expr = "<math><mi>x</mi><mo>&gt;</mo><mn>5</mn></math>";
    test("ja", "ClearSpeak", expr, "x 大なり 5")?;
    return Ok(());
}

/// ≠ is ノット・イコール, the partner of the イコール already used for =.
#[test]
fn not_equal() -> Result<()> {
    let expr = "<math><mi>x</mi><mo>&#x2260;</mo><mn>5</mn></math>";
    test("ja", "ClearSpeak", expr, "x ノット イコール 5")?;
    return Ok(());
}

/// ∈ is 要素オブ. 要素の attaches backwards -- 要素の A is "A of an element" -- and
/// the rules for ∈ and ∊ did not even agree with each other.
#[test]
fn element_of() -> Result<()> {
    for ch in ["&#x2208;", "&#x220a;"] {
        let expr = format!("<math><mi>x</mi><mo>{ch}</mo><mi>A</mi></math>");
        test("ja", "SimpleSpeak", &expr, "x 要素オブ おおもじ エー")?;
        test_ClearSpeak("ja", "ClearSpeak_SetMemberSymbol", "Element", &expr,
            "x 要素オブ おおもじ エー")?;
    }
    return Ok(());
}

/// Inside a set the ClearSpeak options take a different branch of the same rule.
/// 中へ is "into", a direction of motion, not membership. The shape is the one
/// en/ClearSpeak/sets.rs uses for set-builder notation.
#[test]
fn set_builder_member_symbol() -> Result<()> {
    for ch in ["&#x2208;", "&#x220a;"] {
        let expr = format!(
            "<math><mo>{{</mo><mi>x</mi><mo>{ch}</mo><mi>&#x2124;</mi><mo>:</mo>             <mi>x</mi><mo>&#x003E;</mo><mn>5</mn><mo>}}</mo></math>"
        );
        test_ClearSpeak("ja", "ClearSpeak_SetMemberSymbol", "In", &expr,
            "集合 すべての x イン 整数 ただし x 大なり 5")?;
        test_ClearSpeak("ja", "ClearSpeak_SetMemberSymbol", "Element", &expr,
            "集合 すべての x 要素オブ 整数 ただし x 大なり 5")?;
    }
    return Ok(());
}

/// ∉ is the negation of ∈, and 要素ではない ends the sentence, so the set that
/// follows is stranded behind it: "x は 要素ではない A" reads as "x is not an element"
/// and then an A. The source gives ノット・イコール for ≠, so the negation is that same
/// ノット in front of whatever the positive form says.
#[test]
fn not_element_of() -> Result<()> {
    let expr = "<math><mi>x</mi><mo>&#x2209;</mo><mi>A</mi></math>";
    test("ja", "SimpleSpeak", expr, "x ノット 要素オブ, おおもじ エー")?;
    test_ClearSpeak("ja", "ClearSpeak_SetMemberSymbol", "Element", expr,
        "x ノット 要素オブ, おおもじ エー")?;
    return Ok(());
}

/// The Member option said 元 on its own, which is the noun "element" with no
/// preposition, so the set after it was stranded the same way 要素の was for ∈.
/// 元オブ then kept the noun the seed chose, but Open JTalk reads it もとおぶ, so
/// 元 is now written げん; the Element option keeps 要素オブ. It keeps the オブ
/// the source introduces for English prepositions. に含まれる attaches to what comes before it, which
/// reverses the relation, so the In option uses the イン already used inside a set.
#[test]
fn member_and_in_options_work_between_the_operands() -> Result<()> {
    for ch in ["&#x2208;", "&#x220a;"] {
        let expr = format!("<math><mi>x</mi><mo>{ch}</mo><mi>A</mi></math>");
        test_ClearSpeak("ja", "ClearSpeak_SetMemberSymbol", "Member", &expr,
            "x げんオブ おおもじ エー")?;
        test_ClearSpeak("ja", "ClearSpeak_SetMemberSymbol", "In", &expr,
            "x イン おおもじ エー")?;
    }
    return Ok(());
}

/// The source gives 部分集合オブ for ⊂ and 部分集合（オブ）・オア・イコール for ⊆,
/// where the オア イコール is the one already used for ≤. The seed dropped the オブ,
/// leaving 部分集合 to strand the set after it, and wrote the negated forms as
/// でない, which ends the sentence in front of that set. The source gives no
/// reading for the superset forms, so those keep the noun the seed chose.
#[test]
fn subset_and_superset() -> Result<()> {
    for (ch, expected) in [
        ("&#x2282;", "x 部分集合オブ y"),
        ("&#x2283;", "x 上位集合オブ y"),
        ("&#x2284;", "x ノット 部分集合オブ y"),
        ("&#x2285;", "x ノット 上位集合オブ y"),
        ("&#x2286;", "x 部分集合オブ オア イコール y"),
        ("&#x2287;", "x 上位集合オブ オア イコール y"),
    ] {
        let expr = format!("<math><mi>x</mi><mo>{ch}</mo><mi>y</mi></math>");
        test("ja", "ClearSpeak", &expr, expected)?;
    }
    return Ok(());
}

/// ∾ is "most positive" in the numeric sense; 最も肯定的な is "most affirmative".
#[test]
fn most_positive() -> Result<()> {
    let expr = "<math><mi>x</mi><mo>&#x223e;</mo><mi>y</mi></math>";
    test("ja", "ClearSpeak", expr, "x 最も正の y")?;
    return Ok(());
}

/// A hat over a variable is ハット. 帽子 is the thing you wear, and it was said
/// twice.
#[test]
fn accent_hat() -> Result<()> {
    let expr = "<math><mover><mi>x</mi><mo>^</mo></mover></math>";
    test("ja", "ClearSpeak", expr, "x ハット")?;
    return Ok(());
}

/// A tilde is チルダ. チルド is the loanword for "chilled" (as in chilled food).
#[test]
fn accent_tilde() -> Result<()> {
    let expr = "<math><mover><mi>x</mi><mo>~</mo></mover></math>";
    test("ja", "ClearSpeak", expr, "x チルダ")?;
    return Ok(());
}

/// The degree sign is 度. キーワード ("keyword") is not a unit of angle.
#[test]
fn degree_sign() -> Result<()> {
    let expr = "<math><mn>90</mn><mo>&#xb0;</mo></math>";
    test("ja", "ClearSpeak", expr, "90 度")?;
    return Ok(());
}

/// The SI prefix was joined to the unit with "- - -", so 5 km was read キロ、、、メートル.
/// Open JTalk reads a lone 分, 日 or 年 after a space as ブン, ヒ and トシ, so after a
/// number they are joined to it (3日 is read みっか) and after a letter they are written
/// in kana. Per hour and per minute are the words 毎時 and 毎分 (毎 時間 is "every hour").
#[test]
fn units() -> Result<()> {
    for (n, unit, expected) in [
        ("5", "km", "5 キロメートル"),
        ("3", "mg", "3 ミリグラム"),
        ("5", "&#xB5;s", "5 マイクロ秒"),
        ("5", "min", "5分"),
        ("3", "d", "3日"),
        ("2", "yr", "2年"),
    ] {
        let expr = format!("<math><mn>{n}</mn><mi intent=':unit'>{unit}</mi></math>");
        test("ja", "SimpleSpeak", &expr, expected)?;
    }
    let expr = "<math><mi>n</mi><mi intent=':unit'>d</mi></math>";
    test("ja", "SimpleSpeak", expr, "n にち")?;
    let expr = "<math><mn>15</mn><mi intent=':unit'>arcmin</mi></math>";
    test("ja", "SimpleSpeak", expr, "15 ふんかく")?;
    // h is only 毎時 when it is marked as a unit; with Fractions=Per a plain h stays h
    let expr = "<math><mfrac><mi>d</mi><mi>h</mi></mfrac></math>";
    test_ClearSpeak("ja", "ClearSpeak_Fractions", "Per", expr, "d 毎 h")?;
    for (unit, expected) in [("h", "5 キロメートル 毎時"), ("min", "5 キロメートル 毎分"), ("s", "5 キロメートル 毎 秒")] {
        let expr = format!("<math><mfrac><mrow><mn>5</mn><mi intent=':unit'>km</mi></mrow><mi intent=':unit'>{unit}</mi></mfrac></math>");
        test("ja", "SimpleSpeak", &expr, expected)?;
        test("ja", "ClearSpeak", &expr, expected)?;
    }
    // the Greek letter mu (U+03BC) is a micro prefix too, as in en
    let expr = "<math><mn>3</mn><mi intent=':unit'>&#x3BC;g</mi></math>";
    test("ja", "ClearSpeak", expr, "3 マイクログラム")?;
    // a currency amount over a unit is a rate (5 euros per litre), as in en
    for (cur, unit, expected) in [("€", "l", "5 ユーロ 毎 リットル"), ("$", "h", "5 ドル 毎時")] {
        let expr = format!("<math><mfrac><mrow><mi>{cur}</mi><mn>5</mn></mrow><mi mathvariant='normal'>{unit}</mi></mfrac></math>");
        test("ja", "SimpleSpeak", &expr, expected)?;
        test("ja", "ClearSpeak", &expr, expected)?;
    }
    return Ok(());
}

/// rpm was 1分あたりの回転数 ("the number of turns per one minute"), so 1 rpm was read
/// 1 1分あたりの回転数 with two numbers in a row. 毎分回転数 is the word used in Japanese
/// (Open JTalk: マイフンカイテンスー); 回転毎分 is not used, it is read カイテンゴトブン.
#[test]
fn rpm_unit() -> Result<()> {
    let expr = "<math><mn>1</mn><mi intent=':unit'>rpm</mi></math>";
    test("ja", "SimpleSpeak", expr, "1 毎分回転数")?;
    test("ja", "ClearSpeak", expr, "1 毎分回転数")?;
    let expr = "<math><mn>3000</mn><mi intent=':unit'>rpm</mi></math>";
    test("ja", "SimpleSpeak", expr, "3000 毎分回転数")?;
    return Ok(());
}

/// The circled katakana and ideographs, the parenthesized ideographs, ⑲, ㉐ and ㋿ had no entry, so the
/// character itself was passed on and Open JTalk said nothing. A lone ヘ is read as the particle (エ),
/// so ㋬ is written 丸、ヘ, which Open JTalk reads マル、ヘ.
#[test]
fn enclosed_characters() -> Result<()> {
    let expr = "<math><mi>&#x2472;</mi></math>";
    test("ja", "SimpleSpeak", expr, "丸 19")?;
    let expr = "<math><mi>&#x32d0;</mi></math>";
    test("ja", "SimpleSpeak", expr, "丸 ア")?;
    let expr = "<math><mi>&#x32ec;</mi></math>";
    test("ja", "SimpleSpeak", expr, "丸、ヘ")?;
    let expr = "<math><mi>&#x3299;</mi></math>";
    test("ja", "SimpleSpeak", expr, "丸 秘")?;
    let expr = "<math><mi>&#x3231;</mi></math>";
    test("ja", "SimpleSpeak", expr, "カッコ 株")?;
    let expr = "<math><mi>&#x32ff;</mi></math>";
    test("ja", "SimpleSpeak", expr, "令和")?;
    return Ok(());
}

/// The arrows in U+2190-21FF use the words of their neighbours: TO BAR is 棒へ向かう like ⤒, a corner is 隅 like ⌜,
/// SMALL CIRCLE is 小さい円 like ⥈, and ↴ is said like ⤵ because 角が is read カクガ.
#[test]
fn arrows_to_bar_and_corner() -> Result<()> {
    let expr = "<math><mi>&#x21e5;</mi></math>";
    test("ja", "SimpleSpeak", expr, "棒へ向かう右向き矢印")?;
    let expr = "<math><mi>&#x21f1;</mi></math>";
    test("ja", "SimpleSpeak", expr, "隅へ向かう北西矢印")?;
    let expr = "<math><mi>&#x21f4;</mi></math>";
    test("ja", "SimpleSpeak", expr, "小さい円付き右向き矢印")?;
    let expr = "<math><mi>&#x21b4;</mi></math>";
    test("ja", "SimpleSpeak", expr, "右へ進んで下へ折れる矢印")?;
    let expr = "<math><mi>&#x214e;</mi></math>";
    test("ja", "SimpleSpeak", expr, "逆さエフ")?;
    return Ok(());
}

/// The MathType characters in U+E900-E9FF use the words of the standard symbols they look like:
/// ⨫ for e919, the space of ⊓ ⊔ for e920, ⧴ for e94d. 差 異体 was read サ イタイ, so e96d is 差の変形.
#[test]
fn mathtype_relation_symbols() -> Result<()> {
    let expr = "<math><mi>&#xe919;</mi></math>";
    test("ja", "SimpleSpeak", expr, "右さがりドット付きマイナス")?;
    let expr = "<math><mi>&#xe920;</mi></math>";
    test("ja", "SimpleSpeak", expr, "二重四角 ユニオン")?;
    let expr = "<math><mi>&#xe94d;</mi></math>";
    test("ja", "SimpleSpeak", expr, "遅延規則")?;
    let expr = "<math><mi>&#xe96d;</mi></math>";
    test("ja", "SimpleSpeak", expr, "差の変形")?;
    let expr = "<math><mi>&#xe90a;</mi></math>";
    test("ja", "SimpleSpeak", expr, "要素として含む オア イコール")?;
    return Ok(());
}

/// The dingbats and arrows in U+2700-27FF use the words of their neighbours: ★ is 黒星, ⟨ is 山カッコ,
/// ⇚ writes 3重 with a digit, and ⓫ is 黒丸 11. 6つ角 was read ムッツカク, so ✶ says 先が6つ.
#[test]
fn dingbats_and_supplemental_arrows() -> Result<()> {
    let expr = "<math><mi>&#x2736;</mi></math>";
    test("ja", "SimpleSpeak", expr, "先が6つの黒星")?;
    let expr = "<math><mi>&#x27ea;</mi></math>";
    test("ja", "SimpleSpeak", expr, "二重の山カッコ")?;
    let expr = "<math><mi>&#x27f0;</mi></math>";
    test("ja", "SimpleSpeak", expr, "上向き4重矢印")?;
    let expr = "<math><mi>&#x2776;</mi></math>";
    test("ja", "SimpleSpeak", expr, "黒丸 1")?;
    let expr = "<math><mi>&#x27d5;</mi></math>";
    test("ja", "SimpleSpeak", expr, "左外部結合")?;
    return Ok(());
}

/// The squared katakana words (U+3300-3357), the era names and ㍿ had no entry, so the character itself
/// was passed on and Open JTalk said nothing for most of them (㌔, ㍿); ㏂ was read アトメートル, and
/// 毎 written with no space between two units was read ゴト.
#[test]
fn cjk_compatibility_squares() -> Result<()> {
    let expr = "<math><mn>10</mn><mi>&#x3314;</mi></math>";
    test("ja", "SimpleSpeak", expr, "10 キロ")?;
    let expr = "<math><mi>&#x337b;</mi><mn>31</mn></math>";
    test("ja", "SimpleSpeak", expr, "平成 31")?;
    let expr = "<math><mi>&#x337f;</mi></math>";
    test("ja", "SimpleSpeak", expr, "株式会社")?;
    let expr = "<math><mi>&#x33c2;</mi></math>";
    test("ja", "SimpleSpeak", expr, "午前")?;
    let expr = "<math><mn>3</mn><mi>&#x33de;</mi></math>";
    test("ja", "SimpleSpeak", expr, "3 ボルト 毎 メートル")?;
    return Ok(());
}

/// The MathType arrows in U+EB00-EBFF use the words of the standard arrows they look like:
/// 棒へ向かう and 棒から出る like ⤒ and ⥠, and eb13-eb15 say 折れる like ↴, because 角 was read カク.
#[test]
fn mathtype_arrows() -> Result<()> {
    let expr = "<math><mi>&#xeb13;</mi></math>";
    test("ja", "SimpleSpeak", expr, "左へ進んで下へ折れる矢印")?;
    let expr = "<math><mi>&#xeb24;</mi></math>";
    test("ja", "SimpleSpeak", expr, "棒から出るうえかえし左向きハープーン")?;
    let expr = "<math><mi>&#xeb28;</mi></math>";
    test("ja", "SimpleSpeak", expr, "棒へ向かう左かえし上向きハープーン")?;
    let expr = "<math><mi>&#xeb2c;</mi></math>";
    test("ja", "SimpleSpeak", expr, "棒から出る左かえし上向きハープーン")?;
    let expr = "<math><mi>&#xeb30;</mi></math>";
    test("ja", "SimpleSpeak", expr, "棒へ向かう上向き矢印")?;
    return Ok(());
}

/// The MathType negated relations in U+EA00-EAFF use the words of the standard relations they look like:
/// バックスラッシュ 部分集合オブ like ⟈, ノット 上にドット付き要素オブ like ⋵ and ∉, さらにきわめて like ⋘,
/// 凹凸付き like ⪮ and ノット 四角の像 like ⋢, and contain is 要素として含む like ∋. A variant says 変形 like ⦜,
/// because 異体 was read イタイ.
#[test]
fn mathtype_negated_relations() -> Result<()> {
    let expr = "<math><mi>&#xea02;</mi></math>";
    test("ja", "SimpleSpeak", expr, "ノット きわめて小なり 変形")?;
    let expr = "<math><mi>&#xea0a;</mi></math>";
    test("ja", "SimpleSpeak", expr, "ノット 要素として含む オア イコール")?;
    let expr = "<math><mi>&#xea0d;</mi></math>";
    test("ja", "SimpleSpeak", expr, "バックスラッシュ 部分集合オブ")?;
    let expr = "<math><mi>&#xea22;</mi></math>";
    test("ja", "SimpleSpeak", expr, "ノット 上にドット付き要素オブ")?;
    let expr = "<math><mi>&#xea48;</mi></math>";
    test("ja", "SimpleSpeak", expr, "ノット さらにきわめて小なり")?;
    let expr = "<math><mi>&#xea58;</mi></math>";
    test("ja", "SimpleSpeak", expr, "ノット 凹凸付きイコール")?;
    let expr = "<math><mi>&#xea60;</mi></math>";
    test("ja", "SimpleSpeak", expr, "ノット 四角の像")?;
    return Ok(());
}

/// The telegraph symbols for months, hours and days had no entry. The hours decompose to 点, which is
/// "o'clock" in Chinese but てん ("point") in Japanese, so 0点 would be れいてん; they are spoken with 時.
/// Open JTalk reads 1日 alone as イチニチ ("one day"), so the first day is ついたち.
#[test]
fn telegraph_symbols() -> Result<()> {
    let expr = "<math><mi>&#x32c3;</mi></math>";
    test("ja", "SimpleSpeak", expr, "4月")?;
    let expr = "<math><mi>&#x3358;</mi></math>";
    test("ja", "SimpleSpeak", expr, "零時")?;
    let expr = "<math><mi>&#x3366;</mi></math>";
    test("ja", "SimpleSpeak", expr, "14時")?;
    let expr = "<math><mi>&#x33e0;</mi></math>";
    test("ja", "SimpleSpeak", expr, "ついたち")?;
    let expr = "<math><mi>&#x33f3;</mi></math>";
    test("ja", "SimpleSpeak", expr, "20日")?;
    return Ok(());
}

/// The yin and yang departing tone marks write 陰 and 陽 in kana, because Open JTalk read 陰去声 as
/// カゲキョショー and 陽去声 as ヒキョショー. ʶ says 小型 like en "small", because the spelled R already
/// adds おおもじ and 小型大文字 said the capital twice.
#[test]
fn modifier_tone_marks() -> Result<()> {
    let expr = "<math><mi>&#x02ea;</mi></math>";
    test("ja", "SimpleSpeak", expr, "修飾文字 いん去声記号")?;
    let expr = "<math><mi>&#x02eb;</mi></math>";
    test("ja", "SimpleSpeak", expr, "修飾文字 よう去声記号")?;
    let expr = "<math><mi>&#x02b6;</mi></math>";
    test("ja", "SimpleSpeak", expr, "修飾文字 小型 逆さ おおもじ r")?;
    return Ok(());
}

/// ※ is こめじるし because Open JTalk reads 米印 as ベーシルシ. ‸ is キャレット like ^, since の alone
/// says nothing. ⃦ is an overlay like ⃒ and ⃥, so it is 重ね二重縦線装飾.
#[test]
fn general_punctuation_and_currency() -> Result<()> {
    let expr = "<math><mi>&#x203b;</mi></math>";
    test("ja", "SimpleSpeak", expr, "こめじるし")?;
    let expr = "<math><mi>&#x2038;</mi></math>";
    test("ja", "SimpleSpeak", expr, "キャレット")?;
    let expr = "<math><mi>&#x20e6;</mi></math>";
    test("ja", "SimpleSpeak", expr, "重ね二重縦線装飾")?;
    let expr = "<math><mi>&#x2031;</mi></math>";
    test("ja", "SimpleSpeak", expr, "パーミリアド")?;
    let expr = "<math><mi>&#x20b4;</mi></math>";
    test("ja", "SimpleSpeak", expr, "フリヴニャ")?;
    return Ok(());
}

/// The negation sign is ノット. The seed said コメントはありません -- "there are no
/// comments" -- because the English source word is "not".
#[test]
fn logical_not() -> Result<()> {
    let expr = "<math><mo>&#xac;</mo><mi>p</mi></math>";
    test("ja", "ClearSpeak", expr, "ノット p")?;
    return Ok(());
}

/// Geometry has settled Japanese terms: 線分, 半直線, 弧, 点. The katakana
/// transliterations of the English words are not used for these.
#[test]
fn geometry_terms() -> Result<()> {
    for (intent, expected) in [
        ("line-segment", "線分 x y"),
        ("directed-line-segment", "有向線分 x y"),
        ("line", "直線 x y"),
        ("ray", "半直線 x y"),
        ("arc", "弧 x y"),
    ] {
        let expr = format!(
            "<math><mrow intent='{intent}($x,$y)'><mi arg='x'>x</mi><mi arg='y'>y</mi></mrow></math>"
        );
        test("ja", "ClearSpeak", &expr, expected)?;
    }
    return Ok(());
}

/// A point is 点, not ポイント.
#[test]
fn geometry_point() -> Result<()> {
    let expr = "<math><mrow intent='point($x,$y,$z)'><mi arg='x'>x</mi><mi arg='y'>y</mi><mi arg='z'>z</mi></mrow></math>";
    test("ja", "ClearSpeak", expr, "点 x y z")?;
    return Ok(());
}

/// Verbose keeps the head term first, as the reference asks, but a ray and a
/// segment differ in what the second point is: a segment stops at it, a ray only
/// passes through it. 「まで」 would claim the ray ends at B.
#[test]
fn geometry_verbose_from_to() -> Result<()> {
    let seg = "<math><mrow intent='line-segment($x,$y)'><mi arg='x'>x</mi><mi arg='y'>y</mi></mrow></math>";
    test_prefs("ja", "ClearSpeak", vec![("Verbosity", "Verbose")], seg, "線分 x から y まで")?;
    let ray = "<math><mrow intent='ray($x,$y)'><mi arg='x'>x</mi><mi arg='y'>y</mi></mrow></math>";
    test_prefs("ja", "ClearSpeak", vec![("Verbosity", "Verbose")], ray, "半直線 x を始点として y を通る")?;
    return Ok(());
}

/// The measure of an angle is 「角 ABC の大きさ」: Japanese names the angle first. 角 is written
/// かく because Open JTalk reads 角 before a pause as スミ (∠ABC = 90° was スミ, ...).
/// 「角の大きさ ABC」 kept the English order.
#[test]
fn measure_of_angle() -> Result<()> {
    let expr = "<math><mrow intent='measure-of-angle($x,$y,$z)'><mi arg='x'>x</mi><mi arg='y'>y</mi><mi arg='z'>z</mi></mrow></math>";
    test("ja", "ClearSpeak", expr, "かく x y z の大きさ")?;
    test_prefs("ja", "ClearSpeak", vec![("Verbosity", "Verbose")], expr, "かく x y z の大きさ")?;
    test("ja", "SimpleSpeak", expr, "かく x y z の大きさ")?;
    return Ok(());
}

/// ∠ABC = 90° with the letters not grouped: ∠ is followed by a pause, where 角 was read スミ.
/// Two MathType names also ended in a lone 角: the angle one is かく too, and the synthetic
/// division corner is 隅, like the corner brackets ⌜ 左上隅.
#[test]
fn angle_sign_before_a_pause() -> Result<()> {
    let expr = "<math><mo>∠</mo><mi>A</mi><mi>B</mi><mi>C</mi><mo>=</mo><mn>90</mn><mo>°</mo></math>";
    test("ja", "ClearSpeak", expr, "かく, おおもじ エー, おおもじ b おおもじ c; イコール 90 度")?;
    test("ja", "ClearSpeak", "<math><mo>&#xEA2C;</mo></math>", "縦線付きノット かく")?;
    test("ja", "ClearSpeak", "<math><mo>&#xEC92;</mo></math>", "組立除法 隅")?;
    return Ok(());
}

/// The angle-measure intent is said the same way: かく A の大きさ.
#[test]
fn angle_measure_intent() -> Result<()> {
    let expr = "<math><mrow intent='angle-measure($x)'><mi arg='x'>x</mi></mrow></math>";
    test("ja", "ClearSpeak", expr, "かく x の大きさ")?;
    test("ja", "SimpleSpeak", expr, "かく x の大きさ")?;
    // m∠ABC: the operand already says かく, so it is not said twice.
    let angle_arg = "<math><mrow intent='angle-measure($x)'><mrow arg='x' intent='angle($p)'><mi arg='p'>x</mi></mrow></mrow></math>";
    test("ja", "ClearSpeak", angle_arg, "かく x の大きさ")?;
    test("ja", "SimpleSpeak", angle_arg, "かく x の大きさ")?;
    let sign_arg = "<math><mrow intent='angle-measure($x)'><mi>m</mi><mrow arg='x'><mo>&#x2220;</mo><mi>x</mi></mrow></mrow></math>";
    test("ja", "ClearSpeak", sign_arg, "かく x の大きさ")?;
    test("ja", "SimpleSpeak", sign_arg, "かく x の大きさ")?;
    // A silent angle does not say かく itself, so it is said here.
    let silent_arg = "<math><mrow intent='angle-measure($x)'><mrow arg='x' intent='angle:silent($p)'><mi arg='p'>x</mi></mrow></mrow></math>";
    test("ja", "ClearSpeak", silent_arg, "かく x の大きさ")?;
    test("ja", "SimpleSpeak", silent_arg, "かく x の大きさ")?;
    // A silent angle-measure says only its operand.
    let silent_measure = "<math><mrow intent='angle-measure:silent($x)'><mi arg='x'>x</mi></mrow></math>";
    test("ja", "ClearSpeak", silent_measure, "x")?;
    test("ja", "SimpleSpeak", silent_measure, "x")?;
    return Ok(());
}

/// The trigonometric functions have settled Japanese names. 接線 and 割線 are the
/// tangent *line* and the secant *line* -- curves, not the functions -- and 探す is
/// the verb "to search", so none of them can be spoken for tan/sec.
#[test]
fn trigonometric_function_names() -> Result<()> {
    for (name, expected) in [
        ("cos", "コサイン オブ x"),
        ("tan", "タンジェント オブ x"),
        ("sec", "セカント オブ x"),
        ("csc", "コセカント オブ x"),
        ("cot", "コタンジェント, オブ x"),
    ] {
        let expr = format!("<math><mi>{name}</mi><mo>&#x2061;</mo><mi>x</mi></math>");
        test("ja", "SimpleSpeak", &expr, expected)?;
    }
    return Ok(());
}

/// The six hyperbolic functions are 双曲線正弦 through 双曲線余接. Open JTalk reads 割
/// in 双曲線正割 and 双曲線余割 as わり and 双曲線余接 as そうきょくせんあまりせっ, so
/// those three are written 双曲線せいかつ, 双曲線よかつ and 双曲線よせつ.
#[test]
fn hyperbolic_function_names() -> Result<()> {
    for (name, expected) in [
        ("sinh", "双曲線正弦 オブ x"),
        ("cosh", "双曲線余弦 オブ x"),
        ("tanh", "双曲線正接 オブ x"),
        ("sech", "双曲線せいかつ, オブ x"),
        ("csch", "双曲線よかつ オブ x"),
        ("coth", "双曲線よせつ オブ x"),
    ] {
        let expr = format!("<math><mi>{name}</mi><mo>&#x2061;</mo><mi>x</mi></math>");
        test("ja", "SimpleSpeak", &expr, expected)?;
    }
    return Ok(());
}

/// Terse reads the abbreviation aloud instead of the formal name.
#[test]
fn hyperbolic_function_terse() -> Result<()> {
    let expr = "<math><mi>tanh</mi><mo>&#x2061;</mo><mi>x</mi></math>";
    test_prefs("ja", "SimpleSpeak", vec![("Verbosity", "Terse")], expr, "ハイパーボリックタンジェント, x")?;
    return Ok(());
}

/// The parts of a complex number are 実部 and 虚部, and its conjugate is 複素共役.
/// 実際の部分 ("the actual portion") and 想像上の部分 ("an imagined portion") are
/// the everyday senses of "real" and "imaginary", not the mathematical ones.
#[test]
fn complex_number_parts() -> Result<()> {
    for (intent, expected) in [
        ("real-part", "実部 オブ x"),
        ("imaginary-part", "虚部 オブ x"),
        ("complex-conjugate", "複素共役 オブ x"),
        ("complex-arg", "偏角 オブ x"),
    ] {
        let expr = format!("<math><mrow intent='{intent}($x)'><mi arg='x'>x</mi></mrow></math>");
        test("ja", "ClearSpeak", &expr, expected)?;
    }
    return Ok(());
}

/// The inverse trigonometric functions are 逆正弦 through 逆余接. 逆余割 is read
/// ぎゃくよわり, so it is written 逆よかつ; 逆正割 and 逆余接 are read correctly.
#[test]
fn inverse_trigonometric_names() -> Result<()> {
    for (intent, expected) in [
        ("arcsine", "逆正弦 オブ x"),
        ("arccosine", "逆余弦 オブ x"),
        ("arctangent", "逆正接 オブ x"),
        ("arcsecant", "逆正割 オブ x"),
        ("arccosecant", "逆よかつ オブ x"),
        ("arccotangent", "逆余接 オブ x"),
    ] {
        let expr = format!("<math><mrow intent='{intent}($x)'><mi arg='x'>x</mi></mrow></math>");
        test("ja", "ClearSpeak", &expr, expected)?;
    }
    return Ok(());
}

/// Verifies that common lowercase Greek letters use their Japanese names.
#[test]
fn greek_letters() -> Result<()> {
    let expr = "<math><mi>&#x03B1;</mi><mo>+</mo><mi>&#x03B2;</mi></math>";
    test("ja", "ClearSpeak", expr, "アルファ プラス ベータ")?;
    return Ok(());
}

/// Verifies Japanese SimpleSpeak wording for set membership.
#[test]
fn set_membership() -> Result<()> {
    let expr = "<math><mi>x</mi><mo>&#x2208;</mo><mi mathvariant='double-struck'>R</mi></math>";
    test("ja", "SimpleSpeak", expr, "x 要素オブ 実数")?;
    return Ok(());
}

/// The seed read the variable a as イーグル -- an eagle. The English rule says
/// "eigh" only because the letter a and the article "a" sound alike in English;
/// the note to translators at the top of unicode.yaml says most languages do not
/// need this. Japanese reads the letter as エー, and 大文字 エー when capital.
#[test]
fn letter_a() -> Result<()> {
    let expr = "<math><mi>a</mi><mo>+</mo><mi>b</mi></math>";
    test("ja", "ClearSpeak", expr, "エー プラス b")?;
    return Ok(());
}

/// Five Greek letters were wrong and ξ was silent: it had an empty string, so a
/// formula using it simply skipped the variable. ゼタ is the SI prefix zetta,
/// オクタ is "octa", and プッシー is an offensive English word.
/// ξ is グザイ: クサイ is said the same as 臭い ("smelly").
#[test]
fn greek_letters_that_were_wrong() -> Result<()> {
    for (letter, expected) in [
        ("&#x3b6;", "ゼータ"),
        ("&#x3b9;", "イオタ"),
        ("&#x3be;", "グザイ"),
        ("&#x3c5;", "ウプシロン"),
        ("&#x3c8;", "プサイ"),
    ] {
        let expr = format!("<math><mi>{letter}</mi></math>");
        test("ja", "ClearSpeak", &expr, expected)?;
    }
    return Ok(());
}

/// ∂ said 部分的な派生物 -- "a partially derived object". The symbol is read
/// ラウンド and the concept is 偏微分.
#[test]
fn partial_derivative_symbol() -> Result<()> {
    let expr = "<math><mo>&#x2202;</mo></math>";
    test("ja", "ClearSpeak", expr, "偏微分")?;
    return Ok(());
}

/// Set membership was worded as club membership: メンバー, 会員でない
/// ("not a club member") and 所属団体 ("the organization one belongs to").
/// The Auto option lands on the Member branch, which then said a bare 元でない;
/// it now takes the ノット of ≠ in front of the げんオブ the positive form uses.
#[test]
fn set_non_membership() -> Result<()> {
    let expr = "<math><mi>x</mi><mo>&#x2209;</mo><mi>y</mi></math>";
    test("ja", "ClearSpeak", expr, "x ノット げんオブ y")?;
    return Ok(());
}

/// 平行 and 垂直 are the geometric relations. The seed used 平行へ and 垂直へ,
/// which attach a direction particle that cannot follow a noun this way, and
/// wrote the negative form with 並行 -- a different word, meaning "concurrent".
#[test]
fn parallel_and_perpendicular() -> Result<()> {
    for (op, expected) in [
        ("&#x2225;", "x 平行 y"),
        ("&#x2226;", "x ノット 平行 y"),
    ] {
        let expr = format!("<math><mi>x</mi><mo>{op}</mo><mi>y</mi></math>");
        test("ja", "ClearSpeak", &expr, expected)?;
    }
    return Ok(());
}

/// ∽ is the sign for similar figures in Japanese school mathematics and ≒ is the
/// usual Japanese sign for "nearly equal", but both were spoken with names taken
/// from English (reversed tilde, approximately equal to or the image of). Three
/// other words were misread by Open JTalk (原像 as はらぞう, 三重 as みえ and
/// 左正規 as ひだりまさき) and are now written so they are read correctly; 三重
/// is written 3重 everywhere in the ja rules, as in 3重結合.
#[test]
fn operator_words_in_unicode_full() -> Result<()> {
    for (ch, expected) in [
        ("&#x223D;", "x 相似 y"),
        ("&#x2252;", "x ニアリーイコール y"),
        ("&#x2290;", "x 四角のげんぞう y"),
        ("&#x22AA;", "x 3重縦棒右向きターンスタイル y"),
        ("&#x22C9;", "x 左の正規因子半直積 y"),
        ("&#x22CA;", "x 右の正規因子半直積 y"),
    ] {
        let expr = format!("<math><mi>x</mi><mo>{ch}</mo><mi>y</mi></math>");
        test("ja", "SimpleSpeak", &expr, expected)?;
    }
    return Ok(());
}

/// ∻ (homothetic) was 位似, which Open JTalk reads くらいに, so it is 相似の位置 like ∽ 相似,
/// and ∛ says 立方根 オブ like √ 平方根 オブ (∜ is in fourth_root_sign).
#[test]
fn more_operator_words_in_unicode_full() -> Result<()> {
    for (ch, expected) in [
        ("&#x223B;", "x 相似の位置 y"),
        ("&#x22F2;", "x 長い横線付き要素オブ y"),
    ] {
        let expr = format!("<math><mi>x</mi><mo>{ch}</mo><mi>y</mi></math>");
        test("ja", "SimpleSpeak", &expr, expected)?;
    }
    let expr = "<math><mo>&#x221B;</mo><mi>x</mi></math>";
    test("ja", "SimpleSpeak", expr, "立方根 オブ x")?;
    return Ok(());
}

/// Open JTalk reads 波形 at the start of a word as なみかた and 対の as たいの
/// ("versus"), so the wave arrows are 波形の … and the paired arrows are 一対の ….
#[test]
fn arrow_words_in_unicode_full() -> Result<()> {
    for (ch, expected) in [
        ("&#x21AD;", "x 波形の左右矢印 y"),
        ("&#x21C9;", "x 一対の右向き矢印 y"),
        ("&#x21DD;", "x 波形の右向き矢印 y"),
    ] {
        let expr = format!("<math><mi>x</mi><mo>{ch}</mo><mi>y</mi></math>");
        test("ja", "SimpleSpeak", &expr, expected)?;
    }
    return Ok(());
}

/// Open JTalk reads these kanji words wrongly when they stand after a space, so the
/// misread part is written in kana: 正割 is read せいわり and 余割 よわり (now せいかつ and
/// よかつ), 割る after a letter or number is read われる (now わる), 負の is read まけの
/// (now ふの), デカルト積 デカルトつもる, 線形包 せんけいつつみ, 丸めた値 まるめたね, and
/// 空集合 そらしゅうごう. codomain said 共同ドメイン ("joint domain"); it is 終域, which
/// Open JTalk reads おわりいき, so it is written しゅう域.
#[test]
fn kanji_words_written_so_they_read_right() -> Result<()> {
    let cases = [
        ("<mrow intent='secant($x)'><mi arg='x'>x</mi></mrow>", "せいかつ オブ x"),
        ("<mrow intent='cosecant($x)'><mi arg='x'>x</mi></mrow>", "よかつ オブ x"),
        ("<mrow intent='codomain($x)'><mi arg='x'>x</mi></mrow>", "しゅう域 オブ x"),
        ("<mrow intent='quotient($a,$b)'><mi arg='a'>x</mi><mi arg='b'>y</mi></mrow>", "整数部 オブ x わる y"),
        ("<mrow intent='cartesian-product($a,$b)'><mi arg='a'>x</mi><mi arg='b'>y</mi></mrow>", "x デカルトせき y"),
        ("<mrow intent='span($x)'><mi arg='x'>x</mi></mrow>", "線形ほう オブ x"),
        ("<mrow><mi>x</mi><mo>÷</mo><mi>y</mi></mrow>", "x わる y"),
        ("<mrow><mi>x</mi><mo>/</mo><mi>y</mi></mrow>", "x わる y"),
        ("<msup><mi>ℤ</mi><mo>-</mo></msup>", "ふの 整数"),
    ];
    for (body, expected) in cases {
        test("ja", "SimpleSpeak", &format!("<math>{body}</math>"), expected)?;
    }
    return Ok(());
}

/// ≤ said より少しまたは等しい ("a little more, or equal") and ≥ ended in the
/// particle へ. They then said より小さいか等しい / より大きいか等しい, which strand the
/// より the same way > did; the source gives 小なり・オア・イコール.
#[test]
fn comparison_with_equality() -> Result<()> {
    for (op, expected) in [
        ("&#x2264;", "x 小なり オア イコール 5"),
        ("&#x2265;", "x 大なり オア イコール 5"),
    ] {
        let expr = format!("<math><mi>x</mi><mo>{op}</mo><mn>5</mn></math>");
        test("ja", "ClearSpeak", &expr, expected)?;
    }
    return Ok(());
}

/// A factorial is 階乗. ファクシャル is not a Japanese word.
#[test]
fn factorial() -> Result<()> {
    let expr = "<math><mn>5</mn><mo>!</mo></math>";
    test("ja", "ClearSpeak", expr, "5 階乗")?;
    return Ok(());
}

/// Verifies the seeded Japanese cues for a summation with limits.
#[test]
fn summation() -> Result<()> {
    let expr = "<math><munderover><mo>&#x2211;</mo><mrow><mi>i</mi><mo>=</mo><mn>1</mn></mrow><mi>n</mi></munderover><mi>i</mi></math>";
    test("ja", "SimpleSpeak", expr, "サム i イコール 1 から, n まで オブ i")?;
    return Ok(());
}

/// A large operator with only a lower limit takes the postposed "over" cue.
#[test]
fn summation_lower_limit_only() -> Result<()> {
    let expr = "<math><munder><mo>&#x2211;</mo><mrow><mi>i</mi><mo>=</mo><mn>1</mn></mrow></munder><mi>i</mi></math>";
    test("ja", "SimpleSpeak", expr, "サム i イコール 1 にわたる オブ i")?;
    return Ok(());
}

/// When the argument is not simple, an intent ends with an end word. It is 〇〇終了, as in 分数終了,
/// or 〇〇 閉じ for the bars of an absolute value or a norm, as in 丸カッコ閉じ.
/// 端次元 was read ハシジゲン, and 絶対値終了 written as one word is read ゼッタイネ.
/// A subscript says 下付き … 下付き終了 as the msub rules do; it was サブ … 末尾サブ (マツビサブ).
#[test]
fn intent_end_words() -> Result<()> {
    for (name, expected) in [
        ("trace", "トレース オブ エー プラス b トレース終了; イコール 3"),
        ("dimension", "次元 オブ エー プラス b 次元終了; イコール 3"),
        ("homomorphism", "準同型 オブ エー プラス b 準同型終了; イコール 3"),
        ("kernel", "カーネル オブ エー プラス b カーネル終了; イコール 3"),
        ("chemistry-concentration", "濃度 オブ エー プラス b 濃度終了; イコール 3"),
        ("norm", "ノルム オブ エー プラス b ノルム閉じ; イコール 3"),
        ("absolute-value", "絶対値 オブ エー プラス b 絶対値 閉じ; イコール 3"),
    ] {
        let expr = format!(r#"<math><mrow intent="{name}($x)"><mo>(</mo><mrow arg="x"><mi>a</mi><mo>+</mo><mi>b</mi></mrow><mo>)</mo></mrow><mo>=</mo><mn>3</mn></math>"#);
        test("ja", "SimpleSpeak", &expr, expected)?;
    }
    let expr = "<math><mrow><mo>|</mo><mi>a</mi><mo>+</mo><mi>b</mi><mo>|</mo></mrow><mo>=</mo><mn>3</mn></math>";
    test("ja", "SimpleSpeak", expr, "絶対値 オブ エー プラス b 絶対値 閉じ; イコール 3")?;
    let expr = r#"<math><mrow intent="binomial($n,$x)"><mi arg="n">n</mi><mo>,</mo><mrow arg="x"><mi>a</mi><mo>+</mo><mi>b</mi></mrow></mrow><mo>=</mo><mn>3</mn></math>"#;
    test("ja", "SimpleSpeak", expr, "n シー エー プラス b 二項係数終了; イコール 3")?;
    let expr = "<math><msub><mi>x</mi><mrow><mi>i</mi><mo>+</mo><mn>1</mn></mrow></msub></math>";
    test("ja", "SimpleSpeak", expr, "x 下付き i プラス 1 下付き終了")?;
    test_prefs("ja", "SimpleSpeak", vec![("Verbosity", "Verbose")], expr, "x 下付き文字, i プラス 1 下付き文字終了")?;
    return Ok(());
}

/// The same shape is used for the other large operators.
#[test]
fn product_with_limits() -> Result<()> {
    let expr = "<math><munderover><mo>&#x220F;</mo><mrow><mi>i</mi><mo>=</mo><mn>1</mn></mrow><mi>n</mi></munderover><mi>i</mi></math>";
    test("ja", "SimpleSpeak", expr, "プロダクト i イコール 1 から, n まで オブ i")?;
    return Ok(());
}

/// A sum or product written as an intent is said in the order the large-op symbol uses:
/// the limits come first with から, まで or にわたる after them, and the product is プロダクト as for the symbol.
#[test]
fn sum_and_product_intents() -> Result<()> {
    for (name, symbol, word) in [("sum", "&#x2211;", "サム"), ("product", "&#x220F;", "プロダクト")] {
        let one = format!(r#"<math><mrow intent="{name}($x)"><mo>{symbol}</mo><mi arg="x">x</mi></mrow></math>"#);
        test("ja", "SimpleSpeak", &one, &format!("{word} オブ x"))?;
        let two = format!(r#"<math><mrow intent="{name}($i,$x)"><mo>{symbol}</mo><mi arg="i">i</mi><mi arg="x">x</mi></mrow></math>"#);
        test("ja", "SimpleSpeak", &two, &format!("{word} i にわたる オブ x"))?;
        let three = format!(r#"<math><mrow intent="{name}($i,$n,$x)"><mo>{symbol}</mo><mi arg="i">i</mi><mi arg="n">n</mi><mi arg="x">x</mi></mrow></math>"#);
        test("ja", "SimpleSpeak", &three, &format!("{word} i から, n まで オブ x"))?;
        test("ja", "ClearSpeak", &three, &format!("{word} i から, n まで オブ x"))?;
    }
    let expr = r#"<math><mrow intent="sum($l,$u,$b)"><munderover><mo>&#x2211;</mo><mrow arg="l"><mi>i</mi><mo>=</mo><mn>1</mn></mrow><mi arg="u">n</mi></munderover><msub arg="b"><mi>a</mi><mi>i</mi></msub></mrow></math>"#;
    test("ja", "SimpleSpeak", expr, "サム i イコール 1 から, n まで オブ エー 下付き i")?;
    return Ok(());
}

/// Verifies the seeded Japanese cues for a definite integral.
#[test]
fn definite_integral() -> Result<()> {
    let expr = "<math><msubsup><mo>&#x222B;</mo><mn>0</mn><mn>1</mn></msubsup><mi>x</mi><mo>&#x2146;</mo><mi>x</mi></math>";
    test("ja", "SimpleSpeak", expr, "インテグラル 0 から, 1 まで オブ; x d x")?;
    return Ok(());
}

/// Characters that only unicode-full.yaml knows. The seeded file read ¥ as 入会金
/// ("entrance fee"), ∅ as 空のセット, ∋ as 会員を含む ("contains a club member")
/// and ≺ as プリセット; these are the readings a listener actually hears.
#[test]
fn unicode_full_symbol_names() -> Result<()> {
    for (ch, expected) in [
        ("&#x00A5;", "円"),
        ("&#x2205;", "くう集合"),
        ("&#x2295;", "丸 プラス"),
    ] {
        let expr = format!("<math><mo>{ch}</mo></math>");
        test("ja", "ClearSpeak", &expr, expected)?;
    }
    for (op, expected) in [
        ("&#x220B;", "x 要素として含む y"),
        ("&#x227A;", "x 先行 y"),
    ] {
        let expr = format!("<math><mi>x</mi><mo>{op}</mo><mi>y</mi></math>");
        test("ja", "ClearSpeak", &expr, expected)?;
    }
    return Ok(());
}

/// Open JTalk read the vulgar fractions as minutes (4分の1 was ヨンプンノイチ and
/// 2分の1 ニフンノイチ), so they are spaced the way the fraction rule says them;
/// ¥ is a currency like $ and £ (it was 円記号 500); 節記号 was フシキゴー.
#[test]
fn latin1_fractions_and_yen() -> Result<()> {
    for (expr, expected) in [
        ("<math><mn>&#x00BC;</mn></math>", "4 分の 1"),
        ("<math><mi>x</mi><mo>+</mo><mn>&#x00BE;</mn></math>", "x プラス 4 分の 3"),
        ("<math><mo>&#x2E39;</mo></math>", "上半分のセクション記号"),
        ("<math><mo>&#x00A5;</mo><mn>500</mn></math>", "500 円"),
    ] {
        test("ja", "ClearSpeak", expr, expected)?;
        test("ja", "SimpleSpeak", expr, expected)?;
    }
    return Ok(());
}

/// Names that Open JTalk misread: 角カッコ was スミカッコ (上角カッコ ウエスミカッコ),
/// 太白 was タイハク and 右下がり was ミギシタガリ.
#[test]
fn bracket_and_arrow_names_read_correctly() -> Result<()> {
    for (ch, expected) in [
        ("&#x23B4;", "上側角かっこ"),
        ("&#x27E6;", "白角かっこ"),
        ("&#x27AD;", "右下に影の太い白抜き右向き矢印"),
        ("&#x27CD;", "数学用右さがり斜線"),
    ] {
        let expr = format!("<math><mo>{ch}</mo></math>");
        test("ja", "ClearSpeak", &expr, expected)?;
    }
    return Ok(());
}

/// Open JTalk read 硬音記号 as カタオトキゴー and 軟音記号 as ヤワオトキゴー, so the sound
/// part is in kana; ы uses its Russian name ゥイ instead of the old name yeru.
#[test]
fn cyrillic_letter_names() -> Result<()> {
    for (ch, expected) in [
        ("&#x044A;", "こうおん記号"),
        ("&#x044B;", "ゥイ"),
        ("&#x044C;", "なんおん記号"),
        ("&#x042B;", "おおもじ ゥイ"),
    ] {
        let expr = format!("<math><mi>{ch}</mi></math>");
        test("ja", "ClearSpeak", &expr, expected)?;
        test("ja", "SimpleSpeak", &expr, expected)?;
    }
    return Ok(());
}

/// Shape names that Open JTalk misread: 中くらい was ナカクライ, 四分円 was ヨンプンエン
/// (with only 四分円 in kana, 右上 became ミギジョー, 左上 ヒダリジョー and 左下 ヒダリクダ,
/// so the whole name is kana) and 角丸 was スミマル (also in the menclose roundedbox);
/// 中カッコ after ちゅうくらいの also became ナカカッコ.
#[test]
fn shape_names_read_correctly() -> Result<()> {
    for (ch, expected) in [
        ("&#x25FB;", "ちゅうくらいの白四角"),
        ("&#x25F7;", "みぎうえしぶんえん付き白丸"),
        ("&#x25A2;", "かど丸の白四角"),
        ("&#x2774;", "ちゅうくらいのちゅうカッコ飾り"),
        ("&#x25B3;", "三角形"),
    ] {
        let expr = format!("<math><mo>{ch}</mo></math>");
        test("ja", "ClearSpeak", &expr, expected)?;
    }
    return Ok(());
}

/// ◩ and ◪ are halved on the diagonal, so they say 斜め like ⬔ 右上側斜め半分黒の四角, and ▧ says
/// 左上側から like ▨ 右上側から. ◫ says 縦の, since its bisecting line is vertical.
#[test]
fn geometric_shapes() -> Result<()> {
    for (ch, expected) in [
        ("&#x25E9;", "左上側斜め半分黒の四角"),
        ("&#x25EA;", "右下側斜め半分黒の四角"),
        ("&#x25A7;", "左上側から右下への斜線塗りの四角"),
        ("&#x25EB;", "縦の二等分線付き白四角"),
    ] {
        let expr = format!("<math><mo>{ch}</mo></math>");
        test("ja", "ClearSpeak", &expr, expected)?;
    }
    return Ok(());
}

/// Symbol names that Open JTalk misread, found by speaking the zh-cn tests' symbols in ja:
/// 環 was タマキ, 魚の尾 ギョノオ, 凹辺 オーベ, 太 before a noun フトシ, 小文字 after a space
/// コモンジ, 形 after 小なり カタチ, 白 before すみ ハク and before 羽根 シラ, 真直度 マッスグタビ
/// and 辺 after 開いた アタリ.
#[test]
fn misread_symbol_names() -> Result<()> {
    for (ch, expected) in [
        ("&#x232C;", "ベンゼンかん"),
        ("&#x297C;", "左向きさかなの尾"),
        ("&#x27E1;", "おうへん白ひし形"),
        ("&#x2795;", "太いプラス"),
        ("&#x2B58;", "太い丸"),
        ("&#x02B0;", "修飾文字 こもじ エイチ"),
        ("&#xEC48;", "小なりがたの全角結合"),
        ("&#x3016;", "白いすみ付きカッコ"),
        ("&#x27B3;", "白い羽根付き右向き矢印"),
        ("&#x23E4;", "しんちょく度"),
        ("&#x29A8;", "開いたへんが上と右を指す矢印で終わる測定角"),
    ] {
        let expr = format!("<math><mo>{ch}</mo></math>");
        test("ja", "ClearSpeak", &expr, expected)?;
    }
    return Ok(());
}

/// The white arrows in U+2B00-2BFF put 白 after the direction like ⇦ 左向き白矢印 and
/// ⇳ 上下白矢印, and ⬕ says 左下側 like ⬔ 右上側.
#[test]
fn misc_symbols_and_arrows() -> Result<()> {
    for (ch, expected) in [
        ("&#x2B00;", "北東白矢印"),
        ("&#x2B04;", "左右白矢印"),
        ("&#x2B15;", "左下側斜め半分黒の四角"),
    ] {
        let expr = format!("<math><mo>{ch}</mo></math>");
        test("ja", "ClearSpeak", &expr, expected)?;
    }
    return Ok(());
}

/// U+23E6 (AC current) had spell: "ac", which is the XPath for a child element named ac,
/// not the string, so the whole expression failed to speak. It is 交流 in Japanese.
#[test]
fn ac_current_symbol() -> Result<()> {
    let expr = "<math><mi>q</mi><mo>&#x23E6;</mo><mi>q</mi></math>";
    test("ja", "ClearSpeak", expr, "q 交流 q")?;
    return Ok(());
}

/// main reads a∶b as the function intent ratio(a, b); ja gives it a name and glue word.
#[test]
fn ratio_as_function() -> Result<()> {
    let expr = "<math><mi>x</mi><mo>&#x2236;</mo><mi>y</mi></math>";
    test("ja", "ClearSpeak", expr, "比 オブ x 対 y")?;
    return Ok(());
}

/// The infix form of ratio is read x 対 y, the word the function form already uses after its
/// first argument; 比率 names a ratio as a quantity and is not how "x to y" is said.
#[test]
fn ratio_infix() -> Result<()> {
    let expr = "<math><mrow intent='ratio($a,$b)'><mi arg='a'>x</mi><mi arg='b'>y</mi></mrow></math>";
    test("ja", "ClearSpeak", expr, "x 対 y")?;
    test("ja", "SimpleSpeak", expr, "x 対 y")?;
    return Ok(());
}

/// Set-theory names. セット/空のセット are the loanword for a set of objects; the
/// mathematical term is 集合.
#[test]
fn set_terminology() -> Result<()> {
    let expr = "<math><mrow intent='set($x,$y)'><mi arg='x'>x</mi><mi arg='y'>y</mi></mrow></math>";
    test("ja", "ClearSpeak", expr, "集合 オブ x コンマ, y")?;
    return Ok(());
}

/// 規範 is a social norm; the norm of a vector is ノルム.
#[test]
fn norm_terminology() -> Result<()> {
    let expr = "<math><mrow intent='norm($x)'><mi arg='x'>x</mi></mrow></math>";
    test("ja", "ClearSpeak", expr, "ノルム オブ x")?;
    return Ok(());
}

/// 限界 is a bound or a ceiling and 傾向がある is "has a tendency"; neither is how
/// x → a is read. The mathematical terms are 極限 and 近づく. It is 近づく rather
/// than に近づく, because the に would attach to the x in front of it.
#[test]
fn limit_terminology() -> Result<()> {
    let expr = "<math><mrow intent='tends-to($x,$y)'><mi arg='x'>x</mi><mi arg='y'>y</mi></mrow></math>";
    test("ja", "ClearSpeak", expr, "x 近づく y")?;
    return Ok(());
}

/// A limit is followed directly by what it is the limit of. It was 極限 x 近づく 0; オブ, …,
/// with the オブ that English says there ("the limit as x approaches 0, of …"); Japanese has no
/// glue word in that place, and the pause after 0 already marks where the limit ends.
#[test]
fn limit_without_of() -> Result<()> {
    let expr = "<math><munder><mo>lim</mo><mrow><mi>x</mi><mo>&#x2192;</mo><mn>0</mn></mrow></munder><mfrac><mrow><mi>sin</mi><mo>&#x2061;</mo><mi>x</mi></mrow><mi>x</mi></mfrac></math>";
    test("ja", "ClearSpeak", expr, "極限 x 近づく 0; サイン x オーバー x")?;
    test("ja", "SimpleSpeak", expr, "極限 x 近づく 0; 分数, サイン オブ x, オーバー x, 分数終了")?;
    let expr = "<math><munder><mo>lim</mo><mrow><mi>x</mi><mo>&#x2197;</mo><mn>0</mn></mrow></munder><mrow><mi>sin</mi><mo>&#x2061;</mo><mi>x</mi></mrow></math>";
    test("ja", "ClearSpeak", expr, "極限 x 下から近づく 0; サイン x")?;
    return Ok(());
}

/// The Intent rules give lim sup and lim inf the names limit-sup and limit-inf. ja only defined
/// lim-sup and lim-inf, so the English name was spoken, the arrow was 右向き矢印
/// and the オブ of U+2061 stayed. They now read like the plain limit above.
#[test]
fn lim_sup_and_lim_inf() -> Result<()> {
    let expr = "<math><mrow><munder><mi>lim sup</mi><mrow><mi>n</mi><mo>&#x2192;</mo><mi>&#x221E;</mi></mrow></munder><mo>&#x2061;</mo><msub><mi>a</mi><mi>n</mi></msub></mrow></math>";
    test("ja", "SimpleSpeak", expr, "じょう極限 n 近づく 無限大; エー 下付き n")?;
    test("ja", "ClearSpeak", expr, "じょう極限 n 近づく 無限大; エー 下付き n")?;
    test_prefs("ja", "SimpleSpeak", vec![("Verbosity", "Terse")], expr, "じょう極限 n 近づく 無限大; エー 下付き n")?;
    let expr = "<math><mrow><munder><mi>lim inf</mi><mrow><mi>n</mi><mo>&#x2192;</mo><mi>&#x221E;</mi></mrow></munder><mo>&#x2061;</mo><msub><mi>a</mi><mi>n</mi></msub></mrow></math>";
    test("ja", "SimpleSpeak", expr, "か極限 n 近づく 無限大; エー 下付き n")?;
    test("ja", "ClearSpeak", expr, "か極限 n 近づく 無限大; エー 下付き n")?;
    let expr = "<math><mrow><munder><mi>lim inf</mi><mrow><mi>x</mi><mo>&#x2197;</mo><mn>0</mn></mrow></munder><mo>&#x2061;</mo><mrow><mi>f</mi><mo>&#x2061;</mo><mrow><mo>(</mo><mi>x</mi><mo>)</mo></mrow></mrow></mrow></math>";
    test("ja", "SimpleSpeak", expr, "か極限 x 下から近づく 0; f オブ x")?;
    test("ja", "ClearSpeak", expr, "か極限 x 下から近づく 0; f オブ x")?;
    return Ok(());
}

/// MathJax writes \limsup and \liminf as an mo with U+2006 (six-per-em space) between the words. That
/// form was not in FunctionNames, so the U+2061 after it was still read as オブ. It now reads
/// like lim sup and lim inf above.
#[test]
fn lim_sup_and_lim_inf_from_mathjax() -> Result<()> {
    let expr = r#"<math><munder><mo movablelimits="true">lim&#x2006;sup</mo><mrow><mi>n</mi><mo stretchy="false">&#x2192;</mo><mi mathvariant="normal">&#x221E;</mi></mrow></munder><mo>&#x2061;</mo><msub><mi>a</mi><mi>n</mi></msub></math>"#;
    test("ja", "SimpleSpeak", expr, "じょう極限 n 近づく 無限大; エー 下付き n")?;
    test("ja", "ClearSpeak", expr, "じょう極限 n 近づく 無限大; エー 下付き n")?;
    let expr = r#"<math><munder><mo movablelimits="true">lim&#x2006;inf</mo><mrow><mi>x</mi><mo stretchy="false">&#x2192;</mo><mn>0</mn></mrow></munder><mo>&#x2061;</mo><mi>f</mi><mo stretchy="false">(</mo><mi>x</mi><mo stretchy="false">)</mo></math>"#;
    test("ja", "SimpleSpeak", expr, "か極限 x 近づく 0; f オブ x")?;
    test("ja", "ClearSpeak", expr, "か極限 x 近づく 0; f オブ x")?;
    return Ok(());
}

/// Japanese puts the object in front of 比例する, 属する or に先行する, but these
/// words are spoken where the symbol is, so x ≺ y (precedes) came out as
/// x に先行する y, "the y that precedes x". The symbols and intents now use names
/// that are read in symbol order, as 先行 for ≺ and 要素オブ for ∈ already are.
/// maps-to, obtained-from and applied-to have no such name and are spoken in
/// Japanese order instead (x は y に写る).
#[test]
fn relation_words_in_symbol_order() -> Result<()> {
    for (ch, expected) in [
        ("&#x221D;", "x 比例 y"),
        ("&#x2261;", "x 合同 y"),
        ("&#x2262;", "x ノット 合同 y"),
        ("&#x225E;", "x 測定によりイコール y"),
        ("&#x22AD;", "x ノット 真 y"),
    ] {
        let expr = format!("<math><mi>x</mi><mo>{ch}</mo><mi>y</mi></math>");
        test("ja", "SimpleSpeak", &expr, expected)?;
    }
    for (ch, expected) in [
        ("&#x2208;", "集合 すべての x 属する おおもじ エー ただし x 大なり 0"),
        ("&#x2209;", "集合 すべての x 属さない おおもじ エー ただし x 大なり 0"),
        ("&#x220A;", "集合 すべての x 属する おおもじ エー ただし x 大なり 0"),
    ] {
        let expr = format!("<math><mrow><mo>{{</mo><mrow><mi>x</mi><mo>{ch}</mo><mi>A</mi></mrow><mo>|</mo><mrow><mi>x</mi><mo>&gt;</mo><mn>0</mn></mrow><mo>}}</mo></mrow></math>");
        test_ClearSpeak("ja", "ClearSpeak_SetMemberSymbol", "Belongs", &expr, expected)?;
    }
    for (intent, expected) in [
        ("precedes", "x 先行 y"),
        ("succeeds", "x 後続 y"),
        ("equivalent-to", "x 同等 y"),
        ("proportional", "x 比例 y"),
        ("identically-equals", "x 恒等的にイコール y"),
        ("defined-as", "x とは y"),
        ("evaluates-to", "x 評価すると y"),
        ("maps-to", "x は y に写る"),
        ("obtained-from", "x は y から得られる"),
        ("applied-to", "x は y に適用される"),
        ("rate", "x 毎 y"),
        ("does-not-divide", "x 割り切らない y"),
        ("evaluated-at", "x 次の点で評価 y"),
    ] {
        let expr = format!("<math><mrow intent='{intent}($x,$y)'><mi arg='x'>x</mi><mi arg='y'>y</mi></mrow></math>");
        test("ja", "ClearSpeak", &expr, expected)?;
    }
    let expr = "<math><mrow intent='not($p)'><mi arg='p'>p</mi></mrow></math>";
    test("ja", "ClearSpeak", expr, "ノット p")?;
    return Ok(());
}

/// inj lim and proj lim kept their English names and were read like any other
/// script (inj lim 付き i 下 オブ ..., proj lim 下付き i; オブ ...). They are now read
/// like a limit: 帰納極限 or 射影極限, then what is under them, with no オブ before what follows.
#[test]
fn inj_lim_and_proj_lim() -> Result<()> {
    let expr = r#"<math><munder><mo>inj lim</mo><mi>i</mi></munder><mo>&#x2061;</mo><msub><mi>A</mi><mi>i</mi></msub></math>"#;
    test("ja", "SimpleSpeak", expr, "帰納極限 i; おおもじ エー 下付き i")?;
    test("ja", "ClearSpeak", expr, "帰納極限 i; おおもじ エー 下付き i")?;
    test_prefs("ja", "SimpleSpeak", vec![("Verbosity", "Terse")], expr, "帰納極限 i; おおもじ エー 下付き i")?;
    // a subscript instead of an underscript (msub reaches the speech rules as indexed-by)
    let expr = r#"<math><msub><mo>proj lim</mo><mi>i</mi></msub><mo>&#x2061;</mo><msub><mi>A</mi><mi>i</mi></msub></math>"#;
    test("ja", "SimpleSpeak", expr, "射影極限 i; おおもじ エー 下付き i")?;
    let expr = r#"<math><munder><mo>inj lim</mo><mrow><mi>i</mi><mo>&#x2208;</mo><mi>I</mi></mrow></munder><mo>&#x2061;</mo><msub><mi>M</mi><mi>i</mi></msub></math>"#;
    test("ja", "SimpleSpeak", expr, "帰納極限 i 要素オブ おおもじ i; おおもじ m 下付き i")?;
    // an mi and no U+2061 after it
    let expr = r#"<math><munder><mi>proj lim</mi><mi>i</mi></munder><msub><mi>A</mi><mi>i</mi></msub></math>"#;
    test("ja", "ClearSpeak", expr, "射影極限 i; おおもじ エー 下付き i")?;
    return Ok(());
}

/// モード is the loanword; the statistical term is 最頻値.
#[test]
fn statistics_mode() -> Result<()> {
    let expr = "<math><mrow intent='mode($x)'><mi arg='x'>x</mi></mrow></math>";
    test("ja", "ClearSpeak", expr, "最頻値 オブ x")?;
    return Ok(());
}

/// A matrix is 行列, not the loanword マトリクス, and the dimension separator is
/// かける, not によって ("by means of"). A row matrix was "1 かける 2 行 行列": the
/// size already says it is one row, and 行 standing alone is read クダリ or アルキ
/// by Open JTalk, so the extra word is gone.
#[test]
fn matrix_terminology() -> Result<()> {
    let expr = "<math><mrow><mo>[</mo><mtable><mtr><mtd><mn>1</mn></mtd><mtd><mn>2</mn></mtd></mtr></mtable><mo>]</mo></mrow></math>";
    test("ja", "ClearSpeak", expr, "1 かける 2 行列; 1, 2")?;
    return Ok(());
}

fn mat(rows: &[&[&str]], open: &str, close: &str, attrs: &str) -> String {
    let rows: String = rows.iter()
        .map(|r| format!("<mtr>{}</mtr>", r.iter().map(|c| format!("<mtd>{c}</mtd>")).collect::<String>()))
        .collect();
    format!("<math><mrow><mo>{open}</mo><mtable {attrs}>{rows}</mtable><mo>{close}</mo></mrow></math>")
}

/// The matrix rules not covered above say the size as m かける n too: the special
/// matrices, a matrix too big for the short form, and the one-row and one-column
/// forms (which no longer add 行 or 列 before 行列 or ベクトル). A long column read
/// as a vector ends with ベクトル終了, like the short column and row forms.
#[test]
fn matrix_sizes() -> Result<()> {
    let (n0, n1, n2, n3, n4) = ("<mn>0</mn>", "<mn>1</mn>", "<mn>2</mn>", "<mn>3</mn>", "<mn>4</mn>");
    let x2 = "<msup><mi>x</mi><mn>2</mn></msup>";
    test("ja", "SimpleSpeak", &mat(&[&[n0, n0], &[n0, n0]], "[", "]", ""), "2 かける 2 零行列")?;
    test("ja", "SimpleSpeak", &mat(&[&[n1, n0, n0], &[n0, n1, n0], &[n0, n0, n1]], "(", ")", ""),
        "3 かける 3 単位行列")?;
    test_prefs("ja", "SimpleSpeak", vec![("Verbosity", "Terse")],
        &mat(&[&[n2, n0, n0], &[n0, n1, n0], &[n0, n0, x2]], "(", ")", ""),
        "3 かける 3 対角行列; 列 1; 2; 列 2; 1; 列 3; x の 2 乗")?;
    test("ja", "ClearSpeak", &mat(&[&[n1, n2, n3, n4], &[n4, n3, n2, n1]], "(", ")", ""),
        "2 かける 4 行列; ぎょう 1; 列 1; 1, 列 2; 2, 列 3; 3, 列 4; 4; ぎょう 2; 列 1; 4, 列 2; 3, 列 3; 2, 列 4; 1")?;
    test("ja", "ClearSpeak", &mat(&[&[n1, n2, n3, n4], &[n4, n3, n2, n1]], "(", ")", "columnlines='none none solid'"),
        "2 かける 4 拡大行列; ぎょう 1; 列 1; 1, 列 2; 2, 列 3; 3, 列区切り, 列 4; 4; ぎょう 2; 列 1; 4, 列 2; 3, 列 3; 2, 列区切り, 列 4; 1")?;
    test("ja", "SimpleSpeak", &mat(&[&[n1], &[n2], &[n3]], "(", ")", ""), "3 かける 1 行列; 1; 2; 3")?;
    let column = mat(&[&[n1], &[n2], &[n3], &[n4]], "(", ")", "");
    let row = mat(&[&[n1, n2, n3, n4]], "(", ")", "");
    test("ja", "ClearSpeak", &column, "4 かける 1 行列; ぎょう 1; 1; ぎょう 2; 2; ぎょう 3; 3; ぎょう 4; 4")?;
    test("ja", "ClearSpeak", &row, "1 かける 4 行列; 列 1; 1, 列 2; 2, 列 3; 3, 列 4; 4")?;
    // ClearSpeak_Matrix stays set for the rest of this test, so the Vector cases come last.
    test_ClearSpeak("ja", "ClearSpeak_Matrix", "Vector", &column,
        "4 かける 1 ベクトル; ぎょう 1; 1; ぎょう 2; 2; ぎょう 3; 3; ぎょう 4; 4")?;
    test_ClearSpeak("ja", "ClearSpeak_Matrix", "Vector", &row,
        "1 かける 4 ベクトル; 列 1; 1, 列 2; 2, 列 3; 3, 列 4; 4")?;
    test_ClearSpeak("ja", "ClearSpeak_Matrix", "EndMatrix", &column,
        "4 かける 1 行列; ぎょう 1; 1; ぎょう 2; 2; ぎょう 3; 3; ぎょう 4; 4; 行列終了")?;
    test_ClearSpeak("ja", "ClearSpeak_Matrix", "EndVector", &column,
        "4 かける 1 ベクトル; ぎょう 1; 1; ぎょう 2; 2; ぎょう 3; 3; ぎょう 4; 4; ベクトル終了")?;
    test_ClearSpeak("ja", "ClearSpeak_Matrix", "EndMatrix", &row,
        "1 かける 4 行列; 列 1; 1, 列 2; 2, 列 3; 3, 列 4; 4; 行列終了")?;
    test_ClearSpeak("ja", "ClearSpeak_Matrix", "EndVector", &row,
        "1 かける 4 ベクトル; 列 1; 1, 列 2; 2, 列 3; 3, 列 4; 4; ベクトル終了")?;
    return Ok(());
}

/// A line between columns of a table was announced as セパレータ, the English word
/// "separator" in katakana. It is 列区切り now, the partner of ぎょう区切り for a line between rows.
#[test]
fn table_lines_are_column_and_row_separators() -> Result<()> {
    let expr = "<math><mrow><mo>[</mo><mtable rowlines='dashed' columnlines='solid'>
                <mtr><mtd><mn>1</mn></mtd><mtd><mn>2</mn></mtd></mtr>
                <mtr><mtd><mn>3</mn></mtd><mtd><mn>4</mn></mtd></mtr>
                </mtable><mo>]</mo></mrow></math>";
    test("ja", "ClearSpeak", expr,
        "2 かける 2 拡大行列; ぎょう 1; 1, 列区切り, 2, ぎょう区切り; ぎょう 2; 3, 列区切り, 4")?;
    test("ja", "SimpleSpeak", expr,
        "2 かける 2 拡大行列; ぎょう 1; 1, 列区切り, 2, ぎょう区切り; ぎょう 2; 3, 列区切り, 4")?;
    return Ok(());
}

/// Scripts written before and after a symbol (mmultiscripts) were read with the English
/// words pre sub, pre super, sub and super. They use the same words as msub, msup and
/// navigate.yaml now: 前置きの下付き, 前置きの上付き, 下付き and 上付き, with 文字 added when Verbose.
#[test]
fn prescripts_and_postscripts_are_japanese_words() -> Result<()> {
    let expr = "<math><mmultiscripts><mi>x</mi><mi>a</mi><mi>b</mi><mprescripts/><mi>c</mi><mi>d</mi></mmultiscripts></math>";
    test("ja", "ClearSpeak", expr, "x 前置きの下付き c および 前置きの上付き d, 下付き エー および 上付き b")?;
    test_prefs("ja", "SimpleSpeak", vec![("Verbosity", "Verbose")], expr,
        "x 前置きの下付き文字 c および 前置きの上付き文字 d, 下付き文字 エー および 上付き文字 b")?;
    return Ok(());
}

/// Every number set, not just ℝ, has a settled Japanese name. ℝ was the only one
/// with a test, which is how 合理的な数字 ("reasonable numbers") and 整数者
/// ("integer person") survived in the other branches of the same rule.
#[test]
fn number_set_names() -> Result<()> {
    for (letter, expected) in [
        ("C", "x 要素オブ 複素数"),
        ("N", "x 要素オブ 自然数"),
        ("Q", "x 要素オブ 有理数"),
        ("R", "x 要素オブ 実数"),
        ("Z", "x 要素オブ 整数"),
    ] {
        let expr = format!(
            "<math><mi>x</mi><mo>&#x2208;</mo><mi mathvariant='double-struck'>{letter}</mi></math>"
        );
        test("ja", "SimpleSpeak", &expr, expected)?;
    }
    return Ok(());
}

/// In geometry a translation is 平行移動. 翻訳 is translation between languages.
#[test]
fn geometry_translation() -> Result<()> {
    let expr = "<math><mrow intent='translation($x)'><mi arg='x'>x</mi></mrow></math>";
    test("ja", "ClearSpeak", expr, "平行移動 オブ x")?;
    return Ok(());
}

/// The corner brackets were 左上角 and so on, and Open JTalk read 角 there inconsistently (カク, スミ, スミ, カク);
/// 隅 is read スミ every time and is already used for a corner position, as in ⦍ 上隅短線付き角かっこ.
#[test]
fn corner_and_floor_brackets() -> Result<()> {
    for (expr, expected) in [
        ("<math><mrow><mo>&#x231C;</mo><mi>A</mi><mo>&#x231D;</mo></mrow></math>", "左上隅, おおもじ エー, 右上隅"),
        ("<math><mrow><mo>&#x231E;</mo><mi>A</mi><mo>&#x231F;</mo></mrow></math>", "左下隅, おおもじ エー, 右下隅"),
        ("<math><mrow><mo>&#x230A;</mo><mi>x</mi><mo>&#x230B;</mo></mrow></math>", "左床 x 右床"),
    ] {
        test("ja", "ClearSpeak", expr, expected)?;
        test("ja", "SimpleSpeak", expr, expected)?;
    }
    return Ok(());
}

/// 分割する is to split something into parts; divisibility is 割り切る. Similar
/// figures are 相似, not と同様 ("the same as").
#[test]
fn divides_and_similar() -> Result<()> {
    for (intent, expected) in [
        ("divides", "x 割り切る y"),
        ("similar", "x 相似 y"),
    ] {
        let expr = format!(
            "<math><mrow intent='{intent}($a,$b)'><mi arg='a'>x</mi><mi arg='b'>y</mi></mrow></math>"
        );
        test("ja", "ClearSpeak", &expr, expected)?;
    }
    return Ok(());
}

/// nPk permutation notation is read with the standard Japanese school phrase
/// and in display order: (superscript 5) P (subscript 2) is
/// "5 個から 2 個取る順列" ("permutations taking 2 out of 5").
/// en reverses the arguments ("2 permutations of 5"); Japanese does not need to.
#[test]
fn permutation_p_notation() -> Result<()> {
    let expr = "<math><msubsup><mi>P</mi><mn>2</mn><mn>5</mn></msubsup></math>";
    test("ja", "ClearSpeak", expr, "5 個から 2 個取る順列")?;
    test("ja", "SimpleSpeak", expr, "5 個から 2 個取る順列")?;
    return Ok(());
}

/// Multi-line labels: a system of equations labelled as cases is announced as
/// "2 ケース" and each line as "ケース 1", "ケース 2" (the seed doubled the word
/// and used 設備 "equipment" for equation).
#[test]
fn multiline_case_label() -> Result<()> {
    let expr = "<math>
     <mrow>
      <mtable>
       <mtr> <mtd> <mrow> <mi>x</mi><mo>+</mo><mi>y</mi></mrow> </mtd>  <mtd><mo>=</mo> </mtd>  <mtd><mn>7</mn></mtd>  </mtr>
       <mtr> <mtd> <mrow> <mn>2</mn><mi>x</mi><mo>+</mo><mn>3</mn><mi>y</mi></mrow></mtd>  <mtd><mo>=</mo></mtd>  <mtd><mrow><mn>17</mn></mrow></mtd> </mtr>
      </mtable></mrow>
    </math>";
    // SimpleSpeak reaches the same labels through SharedRules/general.yaml, and
    // classifies this as a system of equations rather than cases: 2 式, 式 1, 式 2.
    test("ja", "SimpleSpeak", expr, "2 式; 式 1; x プラス y, イコール 7; 式 2; 2 x プラス 3 y; イコール 17")?;
    test_ClearSpeak("ja", "ClearSpeak_MultiLineLabel", "Case", expr,
        "2 ケース; ケース 1; x プラス y, イコール 7; ケース 2; 2 x プラス 3 y; イコール 17")?;
    return Ok(());
}

/// menclose with a line on one side: the side comes first and the noun last,
/// "左に 線" ("a line on the left"); the seed said ライン アクセス for line-on-right.
#[test]
fn menclose_line_on_left() -> Result<()> {
    let expr = "<math>
                    <menclose notation='left'>  <mfrac><mn>3</mn><mn>2</mn></mfrac> </menclose>
                </math>";
    test("ja", "ClearSpeak", expr, "左に 線, 囲み 2 分の 3 囲み終了")?;
    return Ok(());
}

/// The letter names in the dimension form of a number set (N squared) are the
/// katakana letter names. ネクタイ is a necktie.
#[test]
fn number_set_dimension_letters() -> Result<()> {
    for (letter, expected) in [
        ("ℂ", "シー 2"),
        ("ℕ", "エヌ 2"),
        ("ℚ", "キュー 2"),
        ("ℝ", "アール 2"),
        ("ℤ", "ゼット 2"),
    ] {
        let expr = format!("<math><msup><mi>{letter}</mi><mn>2</mn></msup></math>");
        test("ja", "ClearSpeak", &expr, expected)?;
    }
    return Ok(());
}

/// A vector is ベクトル; コンテンツ is "content". The end marker is one word with
/// the noun first, matching 行列終了 already used in the same file.
#[test]
fn column_vector_end_vector() -> Result<()> {
    let expr = "<math display='block'>
        <mrow><mo>(</mo><mrow><mtable>
          <mtr><mtd><mn>1</mn></mtd></mtr>
          <mtr><mtd><mn>2</mn></mtd></mtr>
          <mtr><mtd><mn>3</mn></mtd></mtr>
        </mtable></mrow><mo>)</mo></mrow></math>";
    test_ClearSpeak("ja", "ClearSpeak_Matrix", "EndVector",
        expr, "3 かける 1 ベクトル; 1; 2; 3; ベクトル終了")?;
    return Ok(());
}

/// The row form, and the matrix form of the same end marker: 行列終了, not
/// リリース 行列 ("release matrix").
#[test]
fn row_vector_and_matrix_end_markers() -> Result<()> {
    let row = "<math display='block'>
        <mrow><mo>[</mo><mrow><mtable>
          <mtr><mtd><mn>1</mn></mtd><mtd><mn>2</mn></mtd></mtr>
        </mtable></mrow><mo>]</mo></mrow></math>";
    test_ClearSpeak("ja", "ClearSpeak_Matrix", "EndVector",
        row, "1 かける 2 ベクトル; 1, 2; ベクトル終了")?;
    let square = "<math display='block'>
        <mrow><mo>(</mo><mrow><mtable>
          <mtr><mtd><mn>2</mn></mtd><mtd><mn>1</mn></mtd></mtr>
          <mtr><mtd><mn>7</mn></mtd><mtd><mn>5</mn></mtd></mtr>
        </mtable></mrow><mo>)</mo></mrow></math>";
    test_ClearSpeak("ja", "ClearSpeak_Matrix", "EndMatrix",
        square, "2 かける 2 行列; ぎょう 1; 2, 1; ぎょう 2; 7, 5; 行列終了")?;
    return Ok(());
}

/// Chemical states use the terms taught in Japanese chemistry: 固体 液体 気体
/// 水溶液. ガスレンジ is a gas cooker and アキュース is not a word.
#[test]
fn chemical_states() -> Result<()> {
    for (state, expected) in [
        ("s", "おおもじ n エー; 固体"),
        ("l", "おおもじ n エー; 液体"),
        ("g", "おおもじ n エー; 気体"),
        ("aq", "おおもじ n エー; 水溶液"),
    ] {
        let expr = format!(
            "<math><mrow><mi>Na</mi><mrow><mo>(</mo><mrow><mi>{state}</mi></mrow><mo>)</mo></mrow></mrow></math>"
        );
        test_prefs("ja", "SimpleSpeak", vec![("Verbosity", "Terse")], &expr, expected)?;
    }
    return Ok(());
}

/// MathCAT says the arrow word between the two sides, so the word has to fit
/// there: A が反応して B, A から B (terse), A と平衡にある B.
#[test]
fn chemical_arrows() -> Result<()> {
    for (arrow, verbosity, expected) in [
        ("&#x2192;", "Verbose", "2, おおもじ h; おおもじ c l; プラス, 2 おおもじ n エー; が反応して; 2, おおもじ n エー; おおもじ c l; プラス, おおもじ h, 下付き文字 2"),
        ("&#x2192;", "Terse", "2, おおもじ h; おおもじ c l; プラス, 2 おおもじ n エー; から; 2, おおもじ n エー; おおもじ c l; プラス おおもじ h, 2"),
        ("&#x21CC;", "Verbose", "2, おおもじ h; おおもじ c l; プラス, 2 おおもじ n エー; と平衡にある; 2, おおもじ n エー; おおもじ c l; プラス, おおもじ h, 下付き文字 2"),
        ("&#x1F8D3;", "Verbose", "2, おおもじ h; おおもじ c l; プラス, 2 おおもじ n エー; と左に偏った平衡にある; 2, おおもじ n エー; おおもじ c l; プラス, おおもじ h, 下付き文字 2"),
        ("&#x1F8D2;", "Verbose", "2, おおもじ h; おおもじ c l; プラス, 2 おおもじ n エー; と右に偏った平衡にある; 2, おおもじ n エー; おおもじ c l; プラス, おおもじ h, 下付き文字 2"),
        // MathJax PUA arrows must preserve the corresponding Unicode reaction reading.
        ("&#xE429;", "Verbose", "2, おおもじ h; おおもじ c l; プラス, 2 おおもじ n エー; が反応して; 2, おおもじ n エー; おおもじ c l; プラス, おおもじ h, 下付き文字 2"),
        ("&#xE42D;", "Verbose", "2, おおもじ h; おおもじ c l; プラス, 2 おおもじ n エー; が反応して; 2, おおもじ n エー; おおもじ c l; プラス, おおもじ h, 下付き文字 2"),
        ("&#xE429;", "Terse", "2, おおもじ h; おおもじ c l; プラス, 2 おおもじ n エー; から; 2, おおもじ n エー; おおもじ c l; プラス おおもじ h, 2"),
        ("&#xE42D;", "Terse", "2, おおもじ h; おおもじ c l; プラス, 2 おおもじ n エー; から; 2, おおもじ n エー; おおもじ c l; プラス おおもじ h, 2"),
        ("&#x1F8D1;", "Verbose", "2, おおもじ h; おおもじ c l; プラス, 2 おおもじ n エー; と平衡にある; 2, おおもじ n エー; おおもじ c l; プラス, おおもじ h, 下付き文字 2"),
        ("&#xE408;", "Verbose", "2, おおもじ h; おおもじ c l; プラス, 2 おおもじ n エー; と平衡にある; 2, おおもじ n エー; おおもじ c l; プラス, おおもじ h, 下付き文字 2"),
        ("&#xE40A;", "Verbose", "2, おおもじ h; おおもじ c l; プラス, 2 おおもじ n エー; と左に偏った平衡にある; 2, おおもじ n エー; おおもじ c l; プラス, おおもじ h, 下付き文字 2"),
        ("&#xE409;", "Verbose", "2, おおもじ h; おおもじ c l; プラス, 2 おおもじ n エー; と右に偏った平衡にある; 2, おおもじ n エー; おおもじ c l; プラス, おおもじ h, 下付き文字 2"),
    ] {
        let expr = format!("<math><mrow><mn>2</mn><mi>H</mi><mi>Cl</mi><mo>+</mo><mn>2</mn><mtext>Na</mtext>
            <mo>{arrow}</mo><mn>2</mn><mtext>Na</mtext><mi>Cl</mi><mo>+</mo><msub><mi>H</mi><mn>2</mn></msub></mrow></math>");
        test_prefs("ja", "SimpleSpeak", vec![("Verbosity", verbosity)], &expr, expected)?;
    }
    return Ok(());
}

/// Bond names are 単結合 二重結合 3重結合 四重結合, not transliterations;
/// 四倍の結束 is "quadruple solidarity", not a chemistry term.
#[test]
fn chemical_bonds() -> Result<()> {
    let ethylene = "<math><mrow>
          <msub><mi>H</mi><mn>2</mn></msub><mi>C</mi>
          <mo>=</mo>
          <mi>C</mi><msub><mi>H</mi><mn>2</mn></msub>
      </mrow></math>";
    test_prefs("ja", "SimpleSpeak", vec![("Verbosity", "Terse")], ethylene,
        "おおもじ h, 2, おおもじ c; 二重結合 おおもじ c; おおもじ h, 2")?;
    // 三重結合 would be read みえけつごう, so the triple bond is written 3重結合.
    let acetylene = "<math><mrow><mi>H</mi><mi>C</mi><mo>&#x2261;</mo><mi>C</mi><mi>H</mi></mrow></math>";
    test_prefs("ja", "SimpleSpeak", vec![("Verbosity", "Terse")], acetylene,
        "おおもじ h; おおもじ c; 3重結合 おおもじ c; おおもじ h")?;
    return Ok(());
}

/// ⸮ is the reversed question mark. It said 逆疑問符 like ¿, the inverted question mark, so the two
/// could not be told apart; it now says 左右反転疑問符.
#[test]
fn supplemental_punctuation() -> Result<()> {
    let expr = "<math><mi>&#x2e2e;</mi></math>";
    test("ja", "SimpleSpeak", expr, "左右反転疑問符")?;
    let expr = "<math><mi>&#xbf;</mi></math>";
    test("ja", "SimpleSpeak", expr, "逆疑問符")?;
    return Ok(());
}

/// The short readings of sub and superscript are 下付き and 上付き, the same
/// stems as the verbose 下付き文字 / 上付き文字 in the branch beside them.
/// サブサブ is the word "sub" doubled and スーパー is a supermarket.
#[test]
fn sub_and_superscript_medium() -> Result<()> {
    let sulfate = "<math><mrow><msup>
          <mrow><mo>[</mo><mi>S</mi><msub><mi>O</mi><mn>4</mn></msub><mo>]</mo></mrow>
          <mrow><mn>2</mn><mo>&#x2212;</mo></mrow>
      </msup></mrow></math>";
    test_prefs("ja", "ClearSpeak", vec![("Verbosity", "Medium")], sulfate,
        "角かっこ, おおもじ s; おおもじ o, 下付き 4; 角かっこ閉じ 上付き 2 マイナス")?;
    return Ok(());
}

/// A determinant ends with 行列式終了, built the same way as 行列終了.
#[test]
fn determinant_end_marker() -> Result<()> {
    let expr = "<math><mrow><mrow><mo>|</mo>
        <mtable>
          <mtr><mtd><mn>2</mn></mtd><mtd><mn>1</mn></mtd></mtr>
          <mtr><mtd><mn>7</mn></mtd><mtd><mn>5</mn></mtd></mtr>
        </mtable>
      <mo>|</mo></mrow></mrow></math>";
    test_ClearSpeak("ja", "ClearSpeak_Matrix", "EndMatrix",
        expr, "2 かける 2 行列式; ぎょう 1; 2, 1; ぎょう 2; 7, 5; 行列式終了")?;
    return Ok(());
}

/// The other determinant forms: one entry, a table too big for the short form,
/// and the determinant of a single expression, whose end marker was 末端の 行列式
/// ("the terminal determinant") and is now 行列式終了 like the others.
/// The scalar form says 行列式 オブ x, like 平方根 オブ x; 行列式 の x would mean
/// "the x of the determinant".
#[test]
fn determinant_sizes_and_scalar_end_marker() -> Result<()> {
    let (n1, n2, n3, n4) = ("<mn>1</mn>", "<mn>2</mn>", "<mn>3</mn>", "<mn>4</mn>");
    test("ja", "ClearSpeak", &mat(&[&["<mi>x</mi>"]], "|", "|", ""), "1 かける 1 行列式 成分 x")?;
    test("ja", "ClearSpeak", &mat(&[&[n1, n2, n3, n4], &[n4, n3, n2, n1]], "|", "|", ""),
        "2 かける 4 行列式; ぎょう 1; 列 1; 1, 列 2; 2, 列 3; 3, 列 4; 4; ぎょう 2; 列 1; 4, 列 2; 3, 列 3; 2, 列 4; 1")?;
    let scalar = "<math><mrow intent='determinant($a)'><mrow arg='a'><mi>x</mi><mo>+</mo><mi>y</mi></mrow></mrow></math>";
    test("ja", "ClearSpeak", scalar, "行列式 オブ x プラス y 行列式終了")?;
    return Ok(());
}

/// LiteralSpeak reads what is written, and Japanese writes the denominator
/// first: 3/4 is "4 分の 3". "3 分の 4" is the other number, 4/3.
#[test]
fn literal_speak_simple_fraction() -> Result<()> {
    let expr = "<math><mfrac><mn>3</mn><mn>4</mn></mfrac></math>";
    test("ja", "LiteralSpeak", expr, "4 分の 3")?;
    return Ok(());
}

/// A fraction that is not two leaves is bracketed instead, and then the written
/// order is kept with the borrowed オーバー, as ClearSpeak already does.
#[test]
fn literal_speak_bracketed_fraction() -> Result<()> {
    let expr = "<math><mfrac><mrow><mi>x</mi><mo>+</mo><mn>1</mn></mrow><mn>2</mn></mfrac></math>";
    test("ja", "LiteralSpeak", expr, "分数, x プラス 1, オーバー 2, 分数終了")?;
    return Ok(());
}

/// The overview rules (what the "describe" navigation command speaks) have their
/// own fraction rule, and it has to say the denominator first as well.
#[test]
fn overview_fraction_is_denominator_first() -> Result<()> {
    set_rules_dir(abs_rules_dir_path())?;
    set_preference("Language", "ja")?;
    set_preference("SpeechStyle", "SimpleSpeak")?;
    set_preference("Verbosity", "Medium")?;
    set_mathml("<math><mfrac><mn>3</mn><mn>4</mn></mfrac></math>")?;
    let spoken = get_overview_text()?;
    assert_eq!("4 分の 3", spoken.trim_end_matches([' ', ',', ';']));
    return Ok(());
}

/// The overview rules say a matrix size the same way as the full speech.
#[test]
fn overview_matrix_size() -> Result<()> {
    let (n1, n2, n3, n4) = ("<mn>1</mn>", "<mn>2</mn>", "<mn>3</mn>", "<mn>4</mn>");
    for (expr, expected) in [
        (mat(&[&[n1, n2], &[n3, n4]], "(", ")", ""), "2 かける 2 行列"),
        (mat(&[&[n1, n2], &[n3, n4]], "|", "|", ""), "2 かける 2 行列式"),
        (mat(&[&[n1, n2, n3], &[n3, n4, n1]], "[", "]", ""), "2 かける 3 行列"),
    ] {
        set_rules_dir(abs_rules_dir_path())?;
        set_preference("Language", "ja")?;
        set_preference("SpeechStyle", "SimpleSpeak")?;
        set_preference("Verbosity", "Medium")?;
        set_mathml(&expr)?;
        let spoken = get_overview_text()?;
        assert_eq!(expected, spoken.trim_end_matches([' ', ',', ';']), "overview of {expr}");
    }
    return Ok(());
}

/// A strike names its direction and then the mark. 対角形 ("diagonal shape") does
/// not say which diagonal, and クロスアウト is the English "cross out".
#[test]
fn menclose_strikes() -> Result<()> {
    for (notation, expected) in [
        ("updiagonalstrike", "右上がり方向, 取り消し線, 囲み x 囲み終了"),
        ("downdiagonalstrike", "右さがり方向, 取り消し線, 囲み x 囲み終了"),
    ] {
        let expr = format!("<math><menclose notation='{notation}'><mi>x</mi></menclose></math>");
        test("ja", "ClearSpeak", &expr, expected)?;
    }
    return Ok(());
}

/// The shapes and the long division mark were transliterations of the English.
#[test]
fn menclose_shapes_and_long_division() -> Result<()> {
    for (notation, expected) in [
        ("circle", "円, 囲み x 囲み終了"),
        ("roundedbox", "かど丸の四角, 囲み x 囲み終了"),
        ("longdiv", "割り算の筆算の記号, 囲み x 囲み終了"),
    ] {
        let expr = format!("<math><menclose notation='{notation}'><mi>x</mi></menclose></math>");
        test("ja", "ClearSpeak", &expr, expected)?;
    }
    return Ok(());
}

/// 上下矢印 is an arrow with a head at each end, so it was the wrong name for the
/// single up arrow, which is 上向き矢印 (Yamaguchi et al. 1996: → is 右向き矢印). The double ended arrows had the English "double ended" in them;
/// unicode-full.yaml already calls ↕ 上下矢印 and ⤢ 北東・南西矢印.
#[test]
fn menclose_arrows() -> Result<()> {
    for (notation, expected) in [
        ("uparrow", "上向き矢印, 囲み x 囲み終了"),
        ("updownarrow", "上下矢印, 囲み x 囲み終了"),
        ("northeastsouthwestarrow", "北東・南西矢印, 囲み x 囲み終了"),
    ] {
        let expr = format!("<math><menclose notation='{notation}'><mi>x</mi></menclose></math>");
        test("ja", "ClearSpeak", &expr, expected)?;
    }
    return Ok(());
}

/// 詳しくはこちら is the web phrase "click here for details". The word wanted here
/// is "above", the partner of the 下 in the rule next to it.
#[test]
fn something_above_and_below_an_expression() -> Result<()> {
    let over = "<math><mover><mrow><mi>x</mi><mo>+</mo><mn>1</mn></mrow><mi>z</mi></mover></math>";
    test("ja", "ClearSpeak", over, "数量 x プラス 1 付き z 上")?;
    let both = "<math><munderover><mrow><mi>x</mi><mo>+</mo><mn>1</mn></mrow><mi>y</mi><mi>z</mi></munderover></math>";
    test("ja", "ClearSpeak", both, "数量 x プラス 1 付き y 下および z 上")?;
    return Ok(());
}

/// ノルダム is Notre-Dame. The Japanese for a norm is ノルム.
/// It says オブ before the argument, like ノルム オブ x; with の, "2 ノルム の x"
/// would mean "the x of the 2-norm".
#[test]
fn subscripted_norm() -> Result<()> {
    let expr = "<math><msub><mrow><mo>&#x2225;</mo><mi>x</mi><mo>&#x2225;</mo></mrow><mn>2</mn></msub></math>";
    test("ja", "ClearSpeak", expr, "2 ノルム オブ x")?;
    return Ok(());
}

/// A subscripted base of a power used to end with a pause, so x_1^2 was read
/// x 下付き 1, の 2 乗, which cuts the phrase before its particle の. The pause is
/// left out when the subscript is the base of a power. In Medium and Verbose a
/// following term still gets its pause after の 2 乗; Terse has none there.
#[test]
fn no_pause_between_subscript_and_power() -> Result<()> {
    let expr = "<math><msubsup><mi>x</mi><mn>1</mn><mn>2</mn></msubsup><mo>+</mo><mi>y</mi></math>";
    test("ja", "SimpleSpeak", expr, "x 下付き 1 の 2 乗, プラス y")?;
    test("ja", "ClearSpeak", expr, "x 下付き 1 の 2 乗, プラス y")?;
    test_prefs("ja", "SimpleSpeak", vec![("Verbosity", "Verbose")], expr, "x 下付き文字 1 の 2 乗, プラス y")?;
    test_prefs("ja", "SimpleSpeak", vec![("Verbosity", "Terse")], expr, "x 1 の 2 乗 プラス y")?;
    return Ok(());
}

/// A matrix has 成分, not エントリー.
#[test]
fn matrix_entry() -> Result<()> {
    let matrix = "<math><mrow><mo>(</mo><mtable><mtr><mtd><mi>x</mi></mtd></mtr></mtable><mo>)</mo></mrow></math>";
    test("ja", "ClearSpeak", matrix, "1 かける 1 行列 成分 x")?;
    return Ok(());
}

/// A coordinate is a 点, the word geometry.yaml already uses.
#[test]
fn coordinate_point() -> Result<()> {
    let coordinate = "<math><mrow intent='point($x,$y)'><mn arg='x'>1</mn><mo>,</mo><mn arg='y'>2</mn></mrow></math>";
    test("ja", "ClearSpeak", coordinate, "点 1 コンマ 2")?;
    return Ok(());
}

/// Letters from \mathbf, \boldsymbol, \mathcal, \mathfrak, \mathbb, \mathbfscr, \mathbffrak, \mathbfsf and
/// \mathbfsfit, written as MathJax writes them (a script L is ℒ, read from the letterlike entries, so A is used).
/// 太字 comes first, as in 太字 イタリック ナブラ, so a bold script letter is 太字 スクリプト (it was スクリプト 太字).
#[test]
fn bold_script_fraktur_and_double_struck_letters() -> Result<()> {
    for (expr, expected) in [
        ("<math><mrow><mi mathvariant=\"bold\">v</mi><mo>=</mo><mn>2</mn><mi mathvariant=\"bold\">u</mi></mrow></math>", "太字 v イコール, 2 太字 u"),
        ("<math><mrow><mi data-mjx-variant=\"-tex-calligraphic\" mathvariant=\"script\">A</mi><mo>,</mo><mi mathvariant=\"script\">l</mi></mrow></math>", "スクリプト おおもじ エー, コンマ, スクリプト l"),
        ("<math><mrow><mi mathvariant=\"fraktur\">g</mi><mo>,</mo><mi mathvariant=\"fraktur\">A</mi></mrow></math>", "フラクトゥール g, コンマ; フラクトゥール おおもじ エー"),
        ("<math><mrow><mi mathvariant=\"double-struck\">k</mi><mo>,</mo><mi mathvariant=\"double-struck\">A</mi><mo>,</mo><mn mathvariant=\"double-struck\">1</mn></mrow></math>", "黒板太字 k コンマ; 黒板太字 おおもじ エー, コンマ, 黒板太字 1"),
        ("<math><mrow><mi mathvariant=\"bold-italic\">&#x3B1;</mi><mo>+</mo><mi mathvariant=\"bold-italic\">x</mi><mo>+</mo><mi mathvariant=\"bold\">&#x393;</mi><mo>+</mo><mi mathvariant=\"bold-italic\">&#x3B2;</mi></mrow></math>", "太字 アルファ プラス 太字 x プラス, 太字 おおもじ ガンマ, プラス 太字 ベータ"),
        ("<math><mrow><mi mathvariant=\"bold-script\">A</mi><mo>,</mo><mi mathvariant=\"bold-fraktur\">A</mi></mrow></math>", "太字 スクリプト おおもじ エー; コンマ; 太字 フラクトゥール おおもじ エー"),
        ("<math><mrow><mi mathvariant=\"bold\">&#x2207;</mi><mo>,</mo><mi mathvariant=\"bold-italic\">&#x2202;</mi></mrow></math>", "太字 ナブラ コンマ, 太字 偏微分"),
        ("<math><mrow><mi mathvariant=\"bold-sans-serif\">A</mi><mo>,</mo><mi mathvariant=\"sans-serif-bold-italic\">x</mi><mo>,</mo><mn mathvariant=\"bold\">7</mn><mo>,</mo><mn mathvariant=\"bold-sans-serif\">7</mn></mrow></math>", "太字 おおもじ エー, コンマ, 太字 x コンマ, 太字 7 コンマ, 太字 7"),
    ] {
        test("ja", "ClearSpeak", expr, expected)?;
        test("ja", "SimpleSpeak", expr, expected)?;
    }
    return Ok(());
}

/// そのようなこと is "such a thing": it translates the words of "such that" and
/// none of its work, which is to attach a condition to what was just named.
/// ただし is the word Japanese mathematical prose uses for that. 集合 stays in
/// front of the variable, where a listener learns what they are hearing before
/// the first argument runs long.
#[test]
fn set_builder_says_the_condition_is_a_condition() -> Result<()> {
    let expr = "<math><mo>{</mo><mrow><mi>x</mi><mo>|</mo><mi>x</mi><mo>&#x003E;</mo><mn>2</mn></mrow><mo>}</mo></math>";
    test("ja", "SimpleSpeak", expr, "集合 すべての x ただし x 大なり 2")?;
    return Ok(());
}

/// 与えられた is the past participle "given", which in Japanese modifies the noun
/// after it -- so P(A | B) came out as "A, given B" with B as the thing being
/// described rather than the condition.
#[test]
fn the_vertical_line_names_the_condition() -> Result<()> {
    let expr = "<math><mn>1</mn><mo>|</mo><mn>2</mn></math>";
    test_ClearSpeak("ja", "ClearSpeak_VerticalLine", "Given", expr, "1 条件は 2")?;
    return Ok(());
}

/// ラベルを使って is "using a label", which says the row does something with a
/// label. The label is simply what the row is called.
#[test]
fn a_labelled_row_names_its_label() -> Result<()> {
    let expr = "<math><mtable><mlabeledtr><mtd><mn>1</mn></mtd><mtd><mi>x</mi></mtd></mlabeledtr></mtable></math>";
    test("ja", "ClearSpeak", expr, "1 ぎょう および 1 列 の表; ぎょう 1 ラベルは 1; 列 1; x")?;
    return Ok(());
}

/// A table with nothing in front of it gets the array intent. It is read as a
/// table, like the plain mtable, and its rows are not joined by コンマ (#890).
#[test]
fn an_unfenced_array_is_read_as_a_table() -> Result<()> {
    let expr = "<math><mtable><mtr><mtd><mi>a</mi></mtd><mtd><mi>b</mi></mtd></mtr>
                       <mtr><mtd><mi>c</mi></mtd><mtd><mi>d</mi></mtd></mtr></mtable></math>";
    test("ja", "ClearSpeak", expr, "2 ぎょう および 2 列 の表; ぎょう 1; 列 1; エー, 列 2; b; ぎょう 2; 列 1; c, 列 2; d")?;
    test("ja", "SimpleSpeak", expr, "2 ぎょう および 2 列 の表; ぎょう 1; 列 1; エー, 列 2; b; ぎょう 2; 列 1; c, 列 2; d")?;
    return Ok(());
}

/// 評価される is the passive "is evaluated", with nothing to say what it is
/// evaluated at, and 同じ式が評価されるマイナス put the minus last, so the
/// subtraction arrived after the thing being subtracted.
#[test]
fn an_evaluation_bar_says_where_it_is_evaluated() -> Result<()> {
    let two = "<math><mrow intent='evaluate($f,$a)'><mi arg='f'>f</mi><mn arg='a'>1</mn></mrow></math>";
    test("ja", "ClearSpeak", two, "f 次の点で評価 1")?;
    let three = "<math><mrow intent='evaluate($f,$a,$b)'><mi arg='f'>f</mi><mn arg='a'>1</mn><mn arg='b'>2</mn></mrow></math>";
    test("ja", "ClearSpeak", three, "f 次の点で評価 2 マイナス 同じ式を次の点で評価 1")?;
    return Ok(());
}

/// ラウンド値 is "round" in katakana with 値 stuck on the end, and
/// フェンスグループ is an English term that has no Japanese meaning at all.
#[test]
fn the_function_names_are_japanese_words() -> Result<()> {
    for (intent, expected) in [
        ("round", "丸めたあたい オブ x"),
        ("fenced-group", "括弧のまとまり オブ x"),
    ] {
        let expr = format!("<math><mrow intent='{intent}($x)'><mi arg='x'>x</mi></mrow></math>");
        test("ja", "ClearSpeak", &expr, expected)?;
    }
    return Ok(());
}

/// sin⁻¹ x is read "サイン インバース" with インバース after the function name.
/// The TrigInverse setting already put it there; the default and SimpleSpeak
/// put it in front, which is the English word order.
#[test]
fn inverse_trig_puts_inverse_after_the_function() -> Result<()> {
    let expr = "<math><msup><mi>sin</mi><mrow><mo>-</mo><mn>1</mn></mrow></msup><mi>x</mi></math>";
    test("ja", "ClearSpeak", expr, "サイン インバース, オブ x")?;
    test_ClearSpeak("ja", "ClearSpeak_Trig", "TrigInverse", expr, "サイン インバース, オブ x")?;
    test("ja", "SimpleSpeak", expr, "サイン インバース, オブ x")?;
    return Ok(());
}

/// ln is 自然対数 in Japanese; 自然ログ is a word-for-word rendering of "natural log".
#[test]
fn natural_log_is_shizen_taisuu() -> Result<()> {
    let expr = "<math><mi>ln</mi><mi>x</mi></math>";
    test_ClearSpeak("ja", "ClearSpeak_Log", "LnAsNaturalLog", expr, "自然対数 x")?;
    test("ja", "SimpleSpeak", expr, "自然対数 オブ x")?;
    return Ok(());
}

/// Log z (capital L) is the principal value of the logarithm, 対数の主値.
/// 主値 is written in kana: Open JTalk reads the kanji シュネ or アルジチ.
/// The comma is the automatic pause: src/tts.rs skips it before a two-byte "of",
/// but オブ is six bytes in UTF-8, so a long function name gets a short pause.
#[test]
fn principal_value_log() -> Result<()> {
    let expr = "<math><mrow><mi>Log</mi><mi>z</mi></mrow></math>";
    test("ja", "ClearSpeak", expr, "対数のしゅち オブ z")?;
    test("ja", "SimpleSpeak", expr, "対数のしゅち オブ z")?;
    return Ok(());
}

/// log_b x is read ログ b の x, the way it is read in Japanese schools; it was ログベース b; オブ x,
/// the English "log base b, of x" word for word. 底 ("base") is not used: Open JTalk reads it ソコ.
/// A simple base is not followed by a pause, which would split ログ b from its の.
#[test]
fn logarithm_with_base() -> Result<()> {
    let expr = "<math><msub><mi>log</mi><mi>b</mi></msub><mo>&#x2061;</mo><mi>x</mi></math>";
    test("ja", "ClearSpeak", expr, "ログ b の x")?;
    test("ja", "SimpleSpeak", expr, "ログ b の x")?;
    test_prefs("ja", "SimpleSpeak", vec![("Verbosity", "Verbose")], expr, "ログ b の x")?;
    test_prefs("ja", "SimpleSpeak", vec![("Verbosity", "Terse")], expr, "ログ b の x")?;
    let expr = "<math><msub><mi>log</mi><mi>b</mi></msub><mo>&#x2061;</mo><mrow><mo>(</mo><mi>x</mi><mo>+</mo><mi>y</mi><mo>)</mo></mrow></math>";
    test("ja", "ClearSpeak", expr, "ログ b の, 丸カッコ, x プラス y, 丸カッコ閉じ")?;
    let expr = "<math><msub><mi>log</mi><mi>b</mi></msub></math>";
    test("ja", "ClearSpeak", expr, "ログ b")?;
    // a base that is not simple keeps the pause that marks where it ends
    let expr = "<math><msub><mi>log</mi><mrow><mi>a</mi><mo>+</mo><mi>b</mi></mrow></msub><mo>&#x2061;</mo><mi>x</mi></math>";
    test("ja", "ClearSpeak", expr, "ログ エー プラス b; の x")?;
    return Ok(());
}

/// A binomial coefficient is read like ₃C₂, "3 シー 2". "3 選ぶ 2" was a
/// word-for-word translation of "3 choose 2".
#[test]
fn binomial_coefficient_is_read_like_ncr() -> Result<()> {
    let expr = "<math><mo>(</mo><mrow><mtable><mtr><mtd><mn>3</mn></mtd></mtr><mtr><mtd><mn>2</mn></mtd></mtr></mtable></mrow><mo>)</mo></math>";
    test_ClearSpeak("ja", "ClearSpeak_Matrix", "Combinatorics", expr, "3 シー 2")?;
    // (3 over 2) with no fraction line, and ₃C₂ itself, go through the binomial intent.
    let frac = "<math><mo>(</mo><mfrac linethickness='0'><mn>3</mn><mn>2</mn></mfrac><mo>)</mo></math>";
    test("ja", "SimpleSpeak", frac, "3 シー 2")?;
    test("ja", "ClearSpeak", frac, "3 シー 2")?;
    let ncr = "<math><mmultiscripts><mi>C</mi><mn>2</mn><none/><mprescripts/><mn>3</mn><none/></mmultiscripts></math>";
    test("ja", "SimpleSpeak", ncr, "3 シー 2")?;
    return Ok(());
}

/// The U+24xx, U+29xx and U+2Exx symbols MathJax writes from TeX: \circledS (the uppercase circled
/// letters now stop at Ⓩ instead of running on into ⓐ-ⓩ), \blacklozenge (黒ロゼンジ as ◊ is ロゼンジ),
/// \notChar and \textinterrobangdown, each in the element MathJax puts it in.
#[test]
fn circled_letters_and_the_last_tex_symbols() -> Result<()> {
    for (expr, expected) in [
        ("<math><mi mathvariant=\"normal\">&#x24C8;</mi></math>", "丸 おおもじ s"),
        ("<math><mrow><mi>&#x24D0;</mi><mo>+</mo><mi>&#x24B7;</mi></mrow></math>", "丸 エー プラス 丸 おおもじ b"),
        ("<math><mrow><mi>x</mi><mo>=</mo><mi>&#x29EB;</mi></mrow></math>", "x イコール 黒ロゼンジ"),
        ("<math><mrow><mi>a</mi><mo>&#x29F8;</mo><mi>b</mi></mrow></math>", "エー 大きいスラッシュ b"),
        ("<math><mtext>&#x2E18;</mtext></math>", "逆さインターロバング"),
    ] {
        test("ja", "ClearSpeak", expr, expected)?;
        test("ja", "SimpleSpeak", expr, expected)?;
    }
    return Ok(());
}

/// 行 for a row or a line was read クダリ or アルキ by Open JTalk, the synthesizer in
/// NVDA's Japanese edition: as a word of its own (行 1, 3 行) and also inside one
/// string (行区切り, 前の行に移動). Written ぎょう it is read ギョウ. 行列 and 行列式
/// are read correctly and stay as they are.
#[test]
fn row_and_line_are_written_in_kana() -> Result<()> {
    let expr = "<math>
     <mrow>
      <mtable>
       <mtr> <mtd> <mrow> <mi>x</mi><mo>+</mo><mi>y</mi></mrow> </mtd>  <mtd><mo>=</mo> </mtd>  <mtd><mn>7</mn></mtd>  </mtr>
       <mtr> <mtd> <mrow> <mn>2</mn><mi>x</mi><mo>+</mo><mn>3</mn><mi>y</mi></mrow></mtd>  <mtd><mo>=</mo></mtd>  <mtd><mrow><mn>17</mn></mrow></mtd> </mtr>
      </mtable></mrow>
    </math>";
    test_ClearSpeak("ja", "ClearSpeak_MultiLineLabel", "Line", expr, "2 ぎょう; ぎょう 1; x プラス y, イコール 7; ぎょう 2; 2 x プラス 3 y; イコール 17")?;
    test_ClearSpeak("ja", "ClearSpeak_MultiLineLabel", "Row", expr, "2 ぎょう; ぎょう 1; x プラス y, イコール 7; ぎょう 2; 2 x プラス 3 y; イコール 17")?;
    // SimpleSpeak reads the two equations as equations (式), so its line word needs
    // a table that is not one: one marked as lines.
    let lines = "<math><mtable intent=':lines'><mtr><mtd><mn>3</mn></mtd></mtr><mtr><mtd><mn>2</mn></mtd></mtr></mtable></math>";
    test("ja", "SimpleSpeak", lines, "2 ぎょう; ぎょう 1; 3; ぎょう 2; 2")?;
    return Ok(());
}

/// ASCII symbols said with their everyday Japanese names: # is シャープ, the star in A* is
/// スター as in English, and the literal ! is 感嘆符 even when terse (バング is not Japanese).
/// An empty part of a fraction or matrix is 空欄, because Open JTalk reads a lone 空 as ソラ (sky).
#[test]
fn ascii_symbol_names() -> Result<()> {
    test("ja", "ClearSpeak", "<math><mo>#</mo><mi>A</mi></math>", "シャープ おおもじ エー")?;
    test("ja", "ClearSpeak", "<math><msup><mi>A</mi><mo>*</mo></msup></math>", "おおもじ エー スター")?;
    test_prefs("ja", "SimpleSpeak", vec![("Verbosity", "Terse")],
        "<math><mi>n</mi><mo intent=':literal'>!</mo></math>", "n 感嘆符")?;
    test("ja", "ClearSpeak", "<math><mfrac><mrow></mrow><mn>2</mn></mfrac></math>", "分数; 空欄; オーバー 2")?;
    let cell = "<math><mrow><mo>(</mo><mtable><mtr><mtd><mn>1</mn></mtd><mtd><mtext> </mtext></mtd></mtr>\
        <mtr><mtd><mn>3</mn></mtd><mtd><mn>4</mn></mtd></mtr></mtable><mo>)</mo></mrow></math>";
    test("ja", "ClearSpeak", cell, "2 かける 2 行列; ぎょう 1; 列 1; 1, 列 2; 空欄; ぎょう 2; 列 1; 3, 列 2; 4")?;
    return Ok(());
}

/// The angle brackets ⟨ ⟩ that \langle and \rangle give are 山括弧 in Japanese (JIS calls 〈 始め山括弧);
/// they were called 三角カッコ. The long arrows keep their names.
#[test]
fn angle_brackets_and_long_arrows() -> Result<()> {
    for (expr, expected) in [
        ("<math><mrow><mo>&#x27E8;</mo><mi>a</mi><mo>,</mo><mi>b</mi><mo>&#x27E9;</mo></mrow></math>",
            "山カッコ, エー コンマ, b, 山カッコ閉じ"),
        ("<math><mi>x</mi><mo>&#x27FC;</mo><msup><mi>x</mi><mn>2</mn></msup></math>",
            "x 棒から出る長い右向き矢印, x の 2 乗"),
        ("<math><mi>p</mi><mo>&#x27F8;</mo><mi>q</mi></math>", "p 長い左向き二重矢印 q"),
    ] {
        test("ja", "ClearSpeak", expr, expected)?;
        test("ja", "SimpleSpeak", expr, expected)?;
    }
    return Ok(());
}

/// A mixed number is read 2 と 4分の3 in Japanese, whether the invisible plus is
/// written or inferred; ± and ∓ are プラスマイナス and マイナスプラス, not the
/// English "plus or minus" in katakana.
#[test]
fn mixed_number_and_plus_minus() -> Result<()> {
    let implicit = "<math><mn>2</mn><mfrac><mn>3</mn><mn>4</mn></mfrac></math>";
    test("ja", "SimpleSpeak", implicit, "2 と 4 分の 3")?;
    let explicit = "<math><mn>2</mn><mo>&#x2064;</mo><mfrac><mn>3</mn><mn>4</mn></mfrac></math>";
    test("ja", "SimpleSpeak", explicit, "2 と 4 分の 3")?;
    let plus_minus = "<math><mi>x</mi><mo>&#xB1;</mo><mn>1</mn></math>";
    test("ja", "SimpleSpeak", plus_minus, "x プラスマイナス 1")?;
    let minus_plus = "<math><mi>x</mi><mo>&#x2213;</mo><mn>1</mn></math>";
    test("ja", "SimpleSpeak", minus_plus, "x マイナスプラス 1")?;
    // The same words when an author writes the intent.
    for (intent, expected) in [
        ("mixed-fraction", "x と y"),
        ("plus-or-minus", "x プラスマイナス y"),
        ("minus-or-plus", "x マイナスプラス y"),
    ] {
        let expr = format!(
            "<math><mrow intent='{intent}($x,$y)'><mi arg='x'>x</mi><mi arg='y'>y</mi></mrow></math>"
        );
        test("ja", "SimpleSpeak", &expr, expected)?;
    }
    return Ok(());
}

/// ℃ is the unit 摂氏度 (like ℉ 華氏度); 摂氏温度 is the name of the quantity.
#[test]
fn degrees_celsius() -> Result<()> {
    let character = "<math><mn>25</mn><mi>&#x2103;</mi></math>";
    test("ja", "SimpleSpeak", character, "25 摂氏度")?;
    let unit = "<math><mn>25</mn><mi intent=':unit'>°C</mi></math>";
    test("ja", "SimpleSpeak", unit, "25 摂氏度")?;
    return Ok(());
}

/// ⅇ and ⅈ are the exponential e and the imaginary i; they used to be read 微分 e and 微分 i.
#[test]
fn double_struck_italic_letters() -> Result<()> {
    let expr = "<math><msup><mi>&#x2147;</mi><mrow><mi>&#x2148;</mi><mi>&#x3B8;</mi></mrow></msup></math>";
    test("ja", "SimpleSpeak", expr, "e の上付き i シータ 上付き終了")?;
    return Ok(());
}

/// The card suits keep 黒 and 白 as en keeps black and white (♡ is not ♥), ★ is 黒星 as ☆ is 白星,
/// and the music signs are the words used in Japanese music lessons. MathJax writes all of them as mi.
#[test]
fn card_suits_and_music_signs() -> Result<()> {
    for (expr, expected) in [
        ("<math><mrow><mi mathvariant=\"normal\">&#x2660;</mi><mo>,</mo><mi mathvariant=\"normal\">&#x2661;</mi><mo>,</mo><mi mathvariant=\"normal\">&#x2662;</mi><mo>,</mo><mi mathvariant=\"normal\">&#x2663;</mi></mrow></math>", "黒スペード コンマ, 白ハート コンマ, 白ダイヤ コンマ, 黒クラブ"),
        ("<math><mrow><mi>C</mi><mi mathvariant=\"normal\">&#x266F;</mi><mo>,</mo><mi>B</mi><mi mathvariant=\"normal\">&#x266D;</mi><mo>,</mo><mi>F</mi><mi mathvariant=\"normal\">&#x266E;</mi></mrow></math>", "おおもじ c シャープ; コンマ; おおもじ b フラット; コンマ; おおもじ f ナチュラル"),
        ("<math><msup><mi>A</mi><mi>&#x2605;</mi></msup></math>", "おおもじ エー の 黒星 乗"),
    ] {
        test("ja", "ClearSpeak", expr, expected)?;
        test("ja", "SimpleSpeak", expr, expected)?;
    }
    return Ok(());
}

/// Between two units a ·, ⋅, ∙ or × is not spoken (en does the same: newton metres), so N·m is
/// ニュートン メートル. The dot is still said between other things and inside :literal.
#[test]
fn product_of_units() -> Result<()> {
    for (expr, expected) in [
        (r#"<math><mn>5</mn><mi intent=":unit">N</mi><mo>·</mo><mi intent=":unit">m</mi></math>"#, "5, ニュートン メートル"),
        (r#"<math><mrow intent=":unit"><mi>N</mi><mo>⋅</mo><mi>m</mi></mrow></math>"#, "ニュートン メートル"),
        (r#"<math><mn>5</mn><mi intent=":unit">N</mi><mo>&#x2219;</mo><mi intent=":unit">m</mi></math>"#, "5, ニュートン メートル"),
        (r#"<math><mn>3</mn><mi intent=":unit">m</mi><mo>×</mo><mi intent=":unit">m</mi></math>"#, "3 メートル メートル"),
        (r#"<math><mo>(</mo><mi intent=":unit">N</mi><mo>·</mo><mi intent=":unit">m</mi><mo>)</mo><mo>/</mo><mi intent=":unit">s</mi></math>"#, "ニュートン メートル; 毎 秒"),
        (r#"<math><mrow intent=":literal"><mi intent=":unit">N</mi><mo>·</mo><mi intent=":unit">m</mi></mrow></math>"#, "おおもじ n ドット m"),
        (r#"<math><mi>u</mi><mo>×</mo><mi>v</mi></math>"#, "u 掛ける v"),
    ] {
        test("ja", "SimpleSpeak", expr, expected)?;
        test("ja", "ClearSpeak", expr, expected)?;
    }
    // the dot between ordinary letters still depends on the style
    let expr = r#"<math><mi>a</mi><mo>·</mo><mi>b</mi></math>"#;
    test("ja", "SimpleSpeak", expr, "エー ドット b")?;
    test("ja", "ClearSpeak", expr, "エー 掛ける b")?;
    return Ok(());
}

/// After another unit in a product the hour is 時 said together with that unit (ワット時, as in
/// 計量単位令), not 時間. Open JTalk reads a lone 時 after a space or a pause as トキ, so the unit
/// before it says 時 and the hour itself says nothing. The hour alone, after a power, after "/"
/// (毎時) and inside :literal is unchanged.
#[test]
fn hour_after_another_unit() -> Result<()> {
    for (expr, expected) in [
        (r#"<math><mn>2</mn><mi intent=":unit">W</mi><mo>·</mo><mi intent=":unit">h</mi></math>"#, "2 ワット時"),
        (r#"<math><mrow intent=":unit"><mi>W</mi><mi>h</mi></mrow></math>"#, "ワット時"),
        (r#"<math><mn>5</mn><mi intent=":unit">V</mi><mo>·</mo><mi intent=":unit">A</mi><mo>·</mo><mi intent=":unit">h</mi></math>"#, "5, ボルト アンペア時"),
        (r#"<math><mn>2</mn><mi intent=":unit">kW</mi><mo>·</mo><mi intent=":unit">h</mi></math>"#, "2, キロワット時"),
        (r#"<math><mn>3000</mn><mi intent=":unit">mA</mi><mo>&#x22C5;</mo><mi intent=":unit">h</mi></math>"#, "3000, ミリアンペア時"),
        (r#"<math><mrow intent=":literal"><mi intent=":unit">W</mi><mo>·</mo><mi intent=":unit">h</mi></mrow></math>"#, "おおもじ w ドット h"),
        (r#"<math><mn>2</mn><mi intent=":unit">W</mi><mo>×</mo><mi intent=":unit">h</mi></math>"#, "2 ワット時"),
        (r#"<math><mn>2</mn><mi intent=":unit">W</mi><mo>&#x2219;</mo><mi intent=":unit">h</mi></math>"#, "2 ワット時"),
        (r#"<math><mn>3</mn><mi intent=":unit">h</mi></math>"#, "3 時間"),
        (r#"<math><mn>60</mn><mi intent=":unit">km</mi><mo>/</mo><mi intent=":unit">h</mi></math>"#, "60, キロメートル, 毎時"),
        (r#"<math><mn>5</mn><mi intent=":unit">kg</mi><mo>/</mo><mo>(</mo><msup><mi intent=":unit">m</mi><mn>2</mn></msup><mo>·</mo><mi intent=":unit">h</mi><mo>)</mo></math>"#, "5; キログラム, 毎; メートル の 2 乗, 時間"),
    ] {
        test("ja", "SimpleSpeak", expr, expected)?;
        test("ja", "ClearSpeak", expr, expected)?;
    }
    return Ok(());
}

/// In terse speech a one-letter hour is spelled and hr stays 時間; the unit before it does not add 時.
#[test]
fn hour_after_another_unit_terse() -> Result<()> {
    for (expr, expected) in [
        (r#"<math><mn>2</mn><mi intent=":unit">kW</mi><mo>·</mo><mi intent=":unit">h</mi></math>"#, "2, キロワット, h"),
        (r#"<math><mn>2</mn><mi intent=":unit">kW</mi><mo>·</mo><mi intent=":unit">hr</mi></math>"#, "2, キロワット, 時間"),
    ] {
        test_prefs("ja", "SimpleSpeak", vec![("Verbosity", "Terse")], expr, expected)?;
    }
    return Ok(());
}

/// 開 in 開部分集合 is read ひらき by Open JTalk, so it is written かい (as in かい区間).
#[test]
fn open_subset_superset() -> Result<()> {
    test("ja", "SimpleSpeak", "<math><mi>x</mi><mo>&#x27C3;</mo><mi>y</mi></math>", "x かい部分集合オブ y")?;
    test("ja", "SimpleSpeak", "<math><mi>x</mi><mo>&#x27C4;</mo><mi>y</mi></math>", "x かい上位集合オブ y")?;
    return Ok(());
}

/// A leaf intent (no arguments) is spoken with its ja IntentMappings word instead of its
/// English intent name; a name that is not there is still spoken as written.
#[test]
fn leaf_intent_uses_intent_mappings() -> Result<()> {
    for (intent, expected) in [
        ("plus", "エー プラス b"),
        ("member-of", "エー げんオブ b"),
        ("factorial", "エー 階乗 b"),
        ("angle", "エー かく b"),
        ("gradient", "エー 勾配 b"),
        ("not-in-mappings", "エー not in mappings b"),
    ] {
        let expr = format!("<math><mrow><mi>a</mi><mo intent='{intent}'>&#x2217;</mo><mi>b</mi></mrow></math>");
        test("ja", "SimpleSpeak", &expr, expected)?;
    }
    return Ok(());
}

/// Under LiteralSpeak a leaf intent is not interpreted: the character itself is read
/// (so ⊥ is the name of the sign, not 垂直), even though other leaf intents now use
/// their IntentMappings word.
#[test]
fn literal_leaf_intent_reads_the_character() -> Result<()> {
    let expr = "<math><mrow><mover><mi>A</mi><mo stretchy='false'>→</mo></mover><mo intent='perpendicular'>⊥</mo><mover><mi>B</mi><mo stretchy='false'>→</mo></mover></mrow></math>";
    test("ja", "LiteralSpeak", expr, "おおもじ エー 右向き矢印, 上向きタック, おおもじ b 右向き矢印")?;
    return Ok(());
}

/// The Terse form of a leaf intent's IntentMappings word is used when Verbosity is Terse.
#[test]
fn leaf_intent_terse() -> Result<()> {
    let expr = "<math><mrow><mi>a</mi><mo intent='cross-product'>×</mo><mi>b</mi></mrow></math>";
    test_prefs("ja", "SimpleSpeak", vec![("Verbosity", "Terse")], expr, "エー クロス b")?;
    return Ok(());
}

/// · between vectors becomes the leaf intent dot-product (Rules/Intent/linear-algebra.yaml).
#[test]
fn vector_dot_product() -> Result<()> {
    let expr = "<math><mrow><mover><mi>a</mi><mo>→</mo></mover><mo>·</mo><mover><mi>b</mi><mo>→</mo></mover></mrow></math>";
    test("ja", "SimpleSpeak", expr, "ベクトル エー; 内積 ベクトル b")?;
    return Ok(());
}

/// × between vectors becomes the leaf intent cross-product (Rules/Intent/linear-algebra.yaml).
#[test]
fn vector_cross_product() -> Result<()> {
    let expr = "<math><mrow><mover><mi>a</mi><mo>→</mo></mover><mo>×</mo><mover><mi>b</mi><mo>→</mo></mover></mrow></math>";
    test("ja", "SimpleSpeak", expr, "ベクトル エー; 外積 ベクトル b")?;
    return Ok(());
}

/// Terms and operators before an element with an empty `ot: ""` (such as a square root
/// or absolute value) must not be dropped inside nested expressions.
#[test]
fn terms_before_root_in_nested_expressions() -> Result<()> {
    let frac_num = "<math><mfrac><mrow><mn>1</mn><mo>+</mo><msqrt><mn>5</mn></msqrt></mrow><mn>2</mn></mfrac></math>";
    test("ja", "ClearSpeak", frac_num, "分数; 1 プラス 平方根 オブ 5; オーバー 2")?;
    test("ja", "SimpleSpeak", frac_num, "分数, 1 プラス 平方根 オブ 5; オーバー 2, 分数終了")?;

    let frac_den = "<math><mfrac><mn>2</mn><mrow><mn>1</mn><mo>+</mo><msqrt><mn>5</mn></msqrt></mrow></mfrac></math>";
    test("ja", "ClearSpeak", frac_den, "分数 2; オーバー 1 プラス 平方根 オブ 5")?;

    let parens = "<math><mrow><mo>(</mo><mn>1</mn><mo>+</mo><msqrt><mn>5</mn></msqrt><mo>)</mo></mrow></math>";
    test("ja", "ClearSpeak", parens, "丸カッコ, 1 プラス 平方根 オブ 5; 丸カッコ閉じ")?;

    let sup = "<math><msup><mi>x</mi><mrow><mn>1</mn><mo>+</mo><msqrt><mn>5</mn></msqrt></mrow></msup></math>";
    test("ja", "ClearSpeak", sup, "x の上付き 1 プラス 平方根 オブ 5, 上付き終了")?;

    let abs_frac = "<math><mfrac><mrow><mn>1</mn><mo>+</mo><mo>|</mo><mi>x</mi><mo>|</mo></mrow><mn>2</mn></mfrac></math>";
    test("ja", "ClearSpeak", abs_frac, "分数; 1 プラス 絶対値 x; オーバー 2")?;
    return Ok(());
}

// ClearSpeak preference values that the en tests cover and ja had no test for.
// The inputs are taken from the en tests that use the same preference.

/// MoreImpliedTimes speaks the invisible times as 掛ける, where Auto leaves it silent
/// (t x, and 2x inside parentheses that are squared).
#[test]
fn clearspeak_pref_implied_times_more_implied_times() -> Result<()> {
    let expr1 = "<math><mrow><mi>t</mi><mo>&#x2062;</mo><mrow><mi>x</mi></mrow></mrow></math>";
    let expr2 = "<math><mrow><mrow><msup><mrow><mrow><mo>(</mo><mrow><mn>2</mn><mi>x</mi></mrow><mo>)</mo></mrow></mrow><mn>2</mn></msup></mrow></mrow></math>";
    test_ClearSpeak("ja", "ClearSpeak_ImpliedTimes", "MoreImpliedTimes", expr1,
        "t 掛ける x")?;
    test_ClearSpeak("ja", "ClearSpeak_ImpliedTimes", "MoreImpliedTimes", expr2,
        "丸カッコ, 2 掛ける x, 丸カッコ閉じ の 2 乗")?;
    return Ok(());
}

/// ImpliedTimes=None drops the 掛ける that Auto speaks next to a parenthesized factor;
/// in 2[3], which Auto reads as 2 掛ける 3, the brackets are then spoken.
#[test]
fn clearspeak_pref_implied_times_none() -> Result<()> {
    let expr1 = "<math><mo>(</mo><mn>2</mn><mo>&#x2212;</mo><mn>1</mn><mo>)</mo><mi>x</mi></math>";
    let expr2 = "<math><mn>2</mn><mo>[</mo><mn>3</mn><mo>]</mo></math>";
    test_ClearSpeak("ja", "ClearSpeak_ImpliedTimes", "None", expr1,
        "丸カッコ, 2 マイナス 1, 丸カッコ閉じ; x")?;
    test_ClearSpeak("ja", "ClearSpeak_ImpliedTimes", "None", expr2,
        "2, 角かっこ 3 角かっこ閉じ")?;
    return Ok(());
}

/// Functions=None stops treating f(...) as a function call: f(x+y) says 掛ける instead
/// of オブ, and B(2,6), which Auto reads with nothing between B and the parentheses, gets 掛ける.
/// With ImpliedTimes=None as well, f(x) gets neither オブ nor 掛ける.
#[test]
fn clearspeak_pref_functions_none() -> Result<()> {
    let expr1 = "<math><mi>log</mi><mo>&#x2061;</mo><mrow><mo>(</mo><mrow><mi>x</mi><mo>+</mo><mi>y</mi></mrow><mo>)</mo></mrow><mo>+</mo><mi>f</mi><mo>&#x2061;</mo><mrow><mo>(</mo><mrow><mi>x</mi><mo>+</mo><mi>y</mi></mrow><mo>)</mo></mrow></math>";
    let expr2 = "<math><mi>B</mi><mrow><mo>(</mo><mrow><mn>2</mn><mo>,</mo><mn>6</mn></mrow><mo>)</mo></mrow></math>";
    let expr3 = "<math><mi>f</mi><mo>(</mo><mi>x</mi><mo>)</mo><mo>=</mo><msup><mi>x</mi><mn>2</mn></msup><mrow><mo>(</mo><mrow><mi>x</mi><mo>+</mo><mn>1</mn></mrow><mo>)</mo></mrow></math>";
    test_ClearSpeak("ja", "ClearSpeak_Functions", "None", expr1,
        "ログ オブ, 丸カッコ, x プラス y, 丸カッコ閉じ; プラス; f 掛ける, 丸カッコ, x プラス y, 丸カッコ閉じ")?;
    test_ClearSpeak("ja", "ClearSpeak_Functions", "None", expr2,
        "おおもじ b 掛ける, 丸カッコ, 2 コンマ, 6, 丸カッコ閉じ")?;
    test_ClearSpeak_prefs("ja", vec![("ClearSpeak_ImpliedTimes", "None"), ("ClearSpeak_Functions", "None")], expr3,
        "f, 丸カッコ x 丸カッコ閉じ; イコール; x の 2 乗, 丸カッコ, x プラス 1, 丸カッコ閉じ")?;
    return Ok(());
}

/// ArcTrig puts アーク before the function name; Auto says インバース after it.
#[test]
fn clearspeak_pref_trig_arc_trig() -> Result<()> {
    let expr = "<math><msup><mi>cosh</mi><mrow><mo>-</mo><mn>1</mn></mrow></msup><mi>x</mi></math>";
    test_ClearSpeak("ja", "ClearSpeak_Trig", "ArcTrig", expr,
        "アーク 双曲線余弦, オブ x")?;
    return Ok(());
}

/// RootEnd adds 根号終了 after the radicand so the end of the root is heard.
#[test]
fn clearspeak_pref_roots_root_end() -> Result<()> {
    let expr1 = "<math><msqrt><mi>x</mi></msqrt></math>";
    let expr2 = "<math><mroot><mrow><mi>x</mi><mo>+</mo><mi>y</mi></mrow><mn>21</mn></mroot></math>";
    test_ClearSpeak("ja", "ClearSpeak_Roots", "RootEnd", expr1,
        "平方根 オブ x, 根号終了")?;
    test_ClearSpeak("ja", "ClearSpeak_Roots", "RootEnd", expr2,
        "21 じょうこん オブ x プラス y, 根号終了")?;
    return Ok(());
}

/// PosNegSqRoot says プラス before a root that is not negated (the positive root);
/// a negated root already says マイナス, as it does with Auto.
#[test]
fn clearspeak_pref_roots_pos_neg_sq_root() -> Result<()> {
    let expr1 = "<math><msqrt><mi>x</mi></msqrt></math>";
    let expr2 = "<math><mo>-</mo><mroot><mi>x</mi><mn>3</mn></mroot><mo>-</mo><msqrt><mi>x</mi></msqrt></math>";
    test_ClearSpeak("ja", "ClearSpeak_Roots", "PosNegSqRoot", expr1,
        "プラス 平方根 オブ x")?;
    test_ClearSpeak("ja", "ClearSpeak_Roots", "PosNegSqRoot", expr2,
        "マイナス 立方根 オブ x; マイナス, プラス 平方根 オブ x")?;
    return Ok(());
}

/// PosNegSqRootEnd does both: プラス before the root and 根号終了 after it.
#[test]
fn clearspeak_pref_roots_pos_neg_sq_root_end() -> Result<()> {
    let expr1 = "<math><msqrt><mi>x</mi></msqrt></math>";
    let expr2 = "<math><mroot><mi>x</mi><mi>t</mi></mroot></math>";
    test_ClearSpeak("ja", "ClearSpeak_Roots", "PosNegSqRootEnd", expr1,
        "プラス 平方根 オブ x, 根号終了")?;
    test_ClearSpeak("ja", "ClearSpeak_Roots", "PosNegSqRootEnd", expr2,
        "プラス t じょうこん オブ x, 根号終了")?;
    return Ok(());
}

/// Auto numbers the lines (ぎょう 1, ぎょう 2), or the cases (ケース 1) of a piecewise function;
/// None keeps the count (2 ぎょう) but drops the per-line labels; Equation, Constraint and Step
/// use 式, 制約条件 and ステップ.
#[test]
fn clearspeak_pref_multiline_label_values() -> Result<()> {
    let expr1 = "<math><mrow><mtable><mtr><mtd><mrow><mi>x</mi><mo>+</mo><mi>y</mi></mrow></mtd><mtd><mo>=</mo></mtd><mtd><mn>7</mn></mtd></mtr><mtr><mtd><mrow><mn>2</mn><mi>x</mi><mo>+</mo><mn>3</mn><mi>y</mi></mrow></mtd><mtd><mo>=</mo></mtd><mtd><mrow><mn>17</mn></mrow></mtd></mtr></mtable></mrow></math>";
    let expr2 = "<math><mi>f</mi><mrow><mo>(</mo><mi>x</mi><mo>)</mo></mrow><mo>=</mo><mrow><mo stretchy='true'>{</mo><mtable><mtr><mtd><mo>-</mo><mn>1</mn></mtd><mtd><mtext>if</mtext></mtd><mtd><mi>x</mi><mo>&lt;</mo><mn>0</mn></mtd></mtr><mtr><mtd><mn>0</mn></mtd><mtd><mtext>if</mtext></mtd><mtd><mi>x</mi><mo>=</mo><mn>0</mn></mtd></mtr><mtr><mtd><mn>1</mn></mtd><mtd><mtext>if</mtext></mtd><mtd><mi>x</mi><mo>&gt;</mo><mn>0</mn></mtd></mtr></mtable></mrow></math>";
    test_ClearSpeak("ja", "ClearSpeak_MultiLineLabel", "Auto", expr1,
        "2 ぎょう; ぎょう 1; x プラス y, イコール 7; ぎょう 2; 2 x プラス 3 y; イコール 17")?;
    test_ClearSpeak("ja", "ClearSpeak_MultiLineLabel", "Auto", expr2,
        "f オブ x, イコール; 3 ケース; ケース 1; マイナス 1 if x 小なり 0; ケース 2; 0 if x イコール 0; ケース 3; 1 if x 大なり 0")?;
    test_ClearSpeak("ja", "ClearSpeak_MultiLineLabel", "None", expr1,
        "2 ぎょう; x プラス y, イコール 7; 2 x プラス 3 y; イコール 17")?;
    test_ClearSpeak("ja", "ClearSpeak_MultiLineLabel", "Equation", expr1,
        "2 式; 式 1; x プラス y, イコール 7; 式 2; 2 x プラス 3 y; イコール 17")?;
    test_ClearSpeak("ja", "ClearSpeak_MultiLineLabel", "Constraint", expr1,
        "2 制約条件; 制約条件 1; x プラス y, イコール 7; 制約条件 2; 2 x プラス 3 y; イコール 17")?;
    test_ClearSpeak("ja", "ClearSpeak_MultiLineLabel", "Step", expr1,
        "2 ステップ; ステップ 1; x プラス y, イコール 7; ステップ 2; 2 x プラス 3 y; イコール 17")?;
    return Ok(());
}

/// SilentBracket drops the 集合 that Auto says before a list of elements in braces.
#[test]
fn clearspeak_pref_sets_silent_bracket() -> Result<()> {
    let expr = "<math><mo>{</mo><mn>5</mn><mo>,</mo><mn>10</mn><mo>,</mo><mn>15</mn><mo>}</mo></math>";
    test_ClearSpeak("ja", "ClearSpeak_Sets", "SilentBracket", expr,
        "5 コンマ, 10 コンマ, 15")?;
    return Ok(());
}

/// woAll drops the すべての that Auto says in set-builder notation.
#[test]
fn clearspeak_pref_sets_wo_all() -> Result<()> {
    let expr = "<math><mo>{</mo><mi>x</mi><mo>∈</mo><mi>ℤ</mi><mo>:</mo><mi>x</mi><mo>&#x003E;</mo><mn>5</mn><mo>}</mo></math>";
    test_ClearSpeak_prefs("ja", vec![("ClearSpeak_SetMemberSymbol", "Member"), ("ClearSpeak_Sets", "woAll")], expr,
        "集合 x げんオブ 整数 ただし x 大なり 5")?;
    return Ok(());
}

/// Auto reads … as テンテンテン and keeps the commas. AndSoOn at the end of a list
/// says 以下同様 and drops the comma before it.
#[test]
fn clearspeak_pref_ellipses_auto_and_so_on() -> Result<()> {
    let expr1 = "<math><mn>1</mn><mo>,</mo><mn>2</mn><mo>,</mo><mn>3</mn><mo>,</mo><mi>…</mi></math>";
    let expr2 = "<math><mrow><mn>1</mn><mo>,</mo><mn>2</mn><mo>,</mo><mn>3</mn><mo>,</mo><mi>…</mi><mo>,</mo><mn>20</mn></mrow></math>";
    test_ClearSpeak("ja", "ClearSpeak_Ellipses", "Auto", expr1,
        "1 コンマ, 2 コンマ, 3 コンマ, テンテンテン")?;
    test_ClearSpeak("ja", "ClearSpeak_Ellipses", "Auto", expr2,
        "1 コンマ, 2 コンマ, 3 コンマ, テンテンテン, コンマ, 20")?;
    test_ClearSpeak("ja", "ClearSpeak_Ellipses", "AndSoOn", expr1,
        "1 コンマ, 2 コンマ, 3 以下同様")?;
    return Ok(());
}

/// AndSoOn in the middle of a list says 以下同様に before the last item and まで after it
/// (en: "3 and so on up to 20"); 以下同様に 20 alone does not say the list runs up to 20.
/// A sum such as x₁ + x₂ + … + xₙ is not a list, so it gets no まで, and neither does a list
/// that ends in a relation (x₁, …, xₙ = 0), where まで would come after the 0.
#[test]
fn clearspeak_pref_ellipses_and_so_on_up_to() -> Result<()> {
    let list = "<math><mrow><mn>1</mn><mo>,</mo><mn>2</mn><mo>,</mo><mn>3</mn><mo>,</mo><mi>…</mi><mo>,</mo><mn>20</mn></mrow></math>";
    let subscripts = "<math><msub><mi>x</mi><mn>1</mn></msub><mo>,</mo><msub><mi>x</mi><mn>2</mn></msub><mo>,</mo><mo>…</mo><mo>,</mo><msub><mi>x</mi><mi>n</mi></msub></math>";
    let sum = "<math><msub><mi>x</mi><mn>1</mn></msub><mo>+</mo><msub><mi>x</mi><mn>2</mn></msub><mo>+</mo><mo>…</mo><mo>+</mo><msub><mi>x</mi><mi>n</mi></msub></math>";
    let relation = "<math><msub><mi>x</mi><mn>1</mn></msub><mo>,</mo><mo>…</mo><mo>,</mo><msub><mi>x</mi><mi>n</mi></msub><mo>=</mo><mn>0</mn></math>";
    test_ClearSpeak("ja", "ClearSpeak_Ellipses", "AndSoOn", list,
        "1 コンマ, 2 コンマ, 3 以下同様に 20 まで")?;
    test_ClearSpeak("ja", "ClearSpeak_Ellipses", "AndSoOn", subscripts,
        "x 下付き 1; コンマ; x 下付き 2; 以下同様に x 下付き n, まで")?;
    test_ClearSpeak("ja", "ClearSpeak_Ellipses", "AndSoOn", sum,
        "x 下付き 1; プラス x 下付き 2; プラス 以下同様に プラス x 下付き n")?;
    test_ClearSpeak("ja", "ClearSpeak_Ellipses", "AndSoOn", relation,
        "x 下付き 1; 以下同様に, x 下付き n; イコール 0")?;
    return Ok(());
}

/// Cross reads × as クロス; Auto reads it as 掛ける.
#[test]
fn clearspeak_pref_mult_symbol_x_cross() -> Result<()> {
    let expr = "<math><mi>u</mi><mo>×</mo><mi>v</mi></math>";
    test_ClearSpeak("ja", "ClearSpeak_MultSymbolX", "Cross", expr,
        "u クロス v")?;
    return Ok(());
}

/// With Auto a bar between two numbers is divisibility, 割り切る (SuchThat would say ただし
/// and Given 条件は).
#[test]
fn clearspeak_pref_vertical_line_auto() -> Result<()> {
    let expr = "<math><mn>3</mn><mo>|</mo><mn>6</mn></math>";
    test_ClearSpeak("ja", "ClearSpeak_VerticalLine", "Auto", expr,
        "3 割り切る 6")?;
    return Ok(());
}

/// SpeakColNum says the column numbers (列 1, 列 2) even when the entries are simple numbers,
/// which Auto reads without them.
#[test]
fn clearspeak_pref_matrix_speak_col_num() -> Result<()> {
    let expr1 = "<math display='block' xmlns='http://www.w3.org/1998/Math/MathML'><mrow><mrow><mo>(</mo><mrow><mtable><mtr><mtd><mn>2</mn></mtd><mtd><mn>1</mn></mtd></mtr><mtr><mtd><mn>7</mn></mtd><mtd><mn>5</mn></mtd></mtr></mtable></mrow><mo>)</mo></mrow></mrow></math>";
    let expr2 = "<math display='block' xmlns='http://www.w3.org/1998/Math/MathML'><mrow><mrow><mo>[</mo><mrow><mtable><mtr><mtd><mn>1</mn></mtd><mtd><mn>2</mn></mtd></mtr></mtable></mrow><mo>]</mo></mrow></mrow></math>";
    test_ClearSpeak("ja", "ClearSpeak_Matrix", "SpeakColNum", expr1,
        "2 かける 2 行列; ぎょう 1; 列 1; 2, 列 2; 1; ぎょう 2; 列 1; 7, 列 2; 5")?;
    test_ClearSpeak("ja", "ClearSpeak_Matrix", "SpeakColNum", expr2,
        "1 かける 2 行列; 列 1; 1, 列 2; 2")?;
    return Ok(());
}

/// SilentColNum drops the column numbers that Auto says when the entries are not simple.
#[test]
fn clearspeak_pref_matrix_silent_col_num() -> Result<()> {
    let expr = "<math><mrow><mrow><mo>(</mo><mrow><mtable><mtr><mtd><mrow><msub><mi>b</mi><mrow><mn>1</mn><mn>1</mn></mrow></msub></mrow></mtd><mtd><mrow><msub><mi>b</mi><mrow><mn>1</mn><mn>2</mn></mrow></msub></mrow></mtd></mtr><mtr><mtd><mrow><msub><mi>b</mi><mrow><mn>2</mn><mn>1</mn></mrow></msub></mrow></mtd><mtd><mrow><msub><mi>b</mi><mrow><mn>2</mn><mn>2</mn></mrow></msub></mrow></mtd></mtr></mtable></mrow><mo>)</mo></mrow></mrow></math>";
    test_ClearSpeak("ja", "ClearSpeak_Matrix", "SilentColNum", expr,
        "2 かける 2 行列; ぎょう 1; b 下付き 1 1; b 下付き 1 2; ぎょう 2; b 下付き 2 1; b 下付き 2 2")?;
    return Ok(());
}

/// ⁿ was written エヌ乗, which Open JTalk reads as エヌノ. With a space before 乗 it is
/// read as a power (エヌ ジョー); ⁱ gets the same space so the two letters match.
#[test]
fn superscript_letter_powers() -> Result<()> {
    let expr = "<math><mi>x</mi><mi>ⁿ</mi></math>";
    test("ja", "SimpleSpeak", expr, "x エヌ 乗")?;
    let expr = "<math><mi>x</mi><mi>ⁱ</mi></math>";
    test("ja", "SimpleSpeak", expr, "x アイ 乗")?;
    return Ok(());
}

/// ⨅ and ⨆ are written with a space like ⊓ and ⊔, ⫋ gets the オブ of ⫌, and a symbol above is
/// 上に…付き as in the other verified names.
#[test]
fn supplemental_operators() -> Result<()> {
    let expr = "<math><mi>&#x2a05;</mi></math>";
    test("ja", "SimpleSpeak", expr, "四角 交わり")?;
    let expr = "<math><mi>&#x2a06;</mi></math>";
    test("ja", "SimpleSpeak", expr, "四角 ユニオン")?;
    let expr = "<math><mi>&#x2acb;</mi></math>";
    test("ja", "SimpleSpeak", expr, "ノット イコールの上に部分集合オブ")?;
    let expr = "<math><mi>&#x2a7b;</mi></math>";
    test("ja", "SimpleSpeak", expr, "上に疑問符付き小なり")?;
    return Ok(());
}

/// ⧓ uses the ボウタイ of ⋈, ⥈ the 小さい円 of SMALL CIRCLE, and ⦦ ⧖ ⥌ avoid words that Open JTalk
/// misreads (斜角 as ハスカク, 白砂時計 as ハクシャトケー, 下端 as カタン).
#[test]
fn miscellaneous_math_symbols_b() -> Result<()> {
    let expr = "<math><mi>&#x29d3;</mi></math>";
    test("ja", "SimpleSpeak", expr, "黒ボウタイ")?;
    let expr = "<math><mi>&#x2948;</mi></math>";
    test("ja", "SimpleSpeak", expr, "小さい円を貫く左右矢印")?;
    let expr = "<math><mi>&#x29a6;</mi></math>";
    test("ja", "SimpleSpeak", expr, "上に開いた斜めの角")?;
    let expr = "<math><mi>&#x29d6;</mi></math>";
    test("ja", "SimpleSpeak", expr, "白い砂時計")?;
    let expr = "<math><mi>&#x294c;</mi></math>";
    test("ja", "SimpleSpeak", expr, "うわはし右かえし したはし左かえし 上下ハープーン")?;
    return Ok(());
}

/// The MathType pieces in U+EC00-ECFF use the words of the standard symbols they are drawn with:
/// 下側中カッコ like ⏟, 上側丸カッコ like ⏜, 筆算の割り算 like ⟌, 左天井床 like ⌈ and ⌊, and 左上側 like the
/// top right rectangle ec90. 下部 was read カブ ("turnip"), so the bottom pieces here, in U+EBxx and U+F8xx
/// and ⌡ say 下の部分, after a space because 下 right after インテグラル was read カ.
#[test]
fn mathtype_bracket_pieces() -> Result<()> {
    let expr = "<math><mi>&#xec00;</mi></math>";
    test("ja", "SimpleSpeak", expr, "下側中カッコ 左端")?;
    let expr = "<math><mi>&#xeca1;</mi></math>";
    test("ja", "SimpleSpeak", expr, "上側丸カッコ 左端")?;
    let expr = "<math><mi>&#xec1f;</mi></math>";
    test("ja", "SimpleSpeak", expr, "根号 下の部分")?;
    let expr = "<math><mi>&#xec32;</mi></math>";
    test("ja", "SimpleSpeak", expr, "筆算の割り算")?;
    let expr = "<math><mi>&#xec95;</mi></math>";
    test("ja", "SimpleSpeak", expr, "左天井床 延長部")?;
    let expr = "<math><mi>&#xec80;</mi></math>";
    test("ja", "SimpleSpeak", expr, "左上側の長方形")?;
    let expr = "<math><mi>&#xeb82;</mi></math>";
    test("ja", "SimpleSpeak", expr, "下向きハープーンの左に上向きハープーン 下の部分")?;
    let expr = "<math><mi>&#xf8f0;</mi></math>";
    test("ja", "SimpleSpeak", expr, "角かっこ 下の部分")?;
    let expr = "<math><mi>&#x2321;</mi></math>";
    test("ja", "SimpleSpeak", expr, "インテグラル 下の部分")?;
    return Ok(());
}

/// ⌇ is なみ線 because Open JTalk read 波線 as ハセン. ⌔ is おうぎ形, the way school math books write
/// the sector. ⌠ says インテグラル 上部 with a space, like ⌡ インテグラル 下の部分 and the other top pieces.
/// ⌐ and ⌙ say ノット記号 like ⫬, and ⌴ is ざぐり, the way JIS drafting writes the counterbore.
#[test]
fn misc_technical_names() -> Result<()> {
    let expr = "<math><mi>&#x2307;</mi></math>";
    test("ja", "SimpleSpeak", expr, "なみ線")?;
    let expr = "<math><mi>&#x2314;</mi></math>";
    test("ja", "SimpleSpeak", expr, "おうぎ形")?;
    let expr = "<math><mi>&#x2320;</mi></math>";
    test("ja", "SimpleSpeak", expr, "インテグラル 上部")?;
    let expr = "<math><mi>&#x2310;</mi></math>";
    test("ja", "SimpleSpeak", expr, "逆ノット記号")?;
    let expr = "<math><mi>&#x2319;</mi></math>";
    test("ja", "SimpleSpeak", expr, "逆さノット記号")?;
    let expr = "<math><mi>&#x2334;</mi></math>";
    test("ja", "SimpleSpeak", expr, "ざぐり")?;
    return Ok(());
}
