/// Тесты для правил, общих для различных стилей речи:
/// * явный intent, русская локализация и грамматика аргументов
use crate::common::*;

use anyhow::Result;

fn all_russian_speech_modes(expr: &str, expected: &str) -> Result<()> {
    russian_speech_verbosities(expr, expected, expected)
}

fn russian_speech_verbosities(expr: &str, terse: &str, normal: &str) -> Result<()> {
    for style in ["ClearSpeak", "SimpleSpeak"] {
        for verbosity in ["Terse", "Medium", "Verbose"] {
            let expected = if verbosity == "Terse" { terse } else { normal };
            test_prefs("ru", style, vec![("Verbosity", verbosity), ("IntentErrorRecovery", "Error")], expr, expected)?;
        }
    }
    Ok(())
}

#[test]
fn w3c_point_dimensions() -> Result<()> {
    // W3C permits one or more coordinates; retain the established two-coordinate reading.
    all_russian_speech_modes("<math><mrow intent='point(1)'/></math>", "точка 1")?;
    all_russian_speech_modes("<math><mrow intent='point(1,2)'/></math>", "точка 1 запятая 2")?;
    all_russian_speech_modes("<math><mrow intent='point(1,2,3)'/></math>", "точка 1 запятая 2 запятая 3")
}

#[test]
fn w3c_point_compound_coordinate() -> Result<()> {
    // A coordinate expression remains a single coordinate, with the separator after the whole expression.
    all_russian_speech_modes("<math><mrow intent='point($c,3)'><mrow arg='c'><mn>1</mn><mo>+</mo><mn>2</mn></mrow></mrow></math>", "точка 1 плюс 2 запятая 3")
}

#[test]
fn w3c_pochhammer_function() -> Result<()> {
    // W3C's default function form keeps n=5 and k=2 in the Russian 'из n по k' order.
    all_russian_speech_modes("<math><mrow intent='pochhammer(5,2)'/></math>", "число размещений из 5 по 2")?;
    all_russian_speech_modes("<math><mrow intent='pochhammer:function(5,2)'/></math>", "число размещений из 5 по 2")
}

#[test]
fn w3c_pochhammer_infix_compatibility() -> Result<()> {
    // Existing inferred intents use infix fixity; their wording and argument order must remain unchanged.
    all_russian_speech_modes("<math><mrow intent='pochhammer:infix(5,2)'/></math>", "число размещений из 5 по 2")
}

#[test]
fn function_set_image() -> Result<()> {
    // An explicit image intent names the image, with a genitive capital-letter operand.
    all_russian_speech_modes("<math><mrow intent='image($r)'><mi>Im</mi><mi arg='r'>R</mi></mrow></math>", "образ заглавной эр")
}

#[test]
fn function_set_domain() -> Result<()> {
    // The domain is the set of allowed inputs, not an ordinary function applied with "от".
    all_russian_speech_modes("<math><mrow intent='domain($f)'><mi>Dom</mi><mi arg='f'>F</mi></mrow></math>", "область определения заглавной эф")
}

#[test]
fn function_set_codomain() -> Result<()> {
    // The codomain is the target set; it must remain distinct from the image.
    all_russian_speech_modes("<math><mrow intent='codomain($f)'><mi>Cod</mi><mi arg='f'>F</mi></mrow></math>", "целевое множество заглавной эф")
}

#[test]
fn function_set_nested_genitive() -> Result<()> {
    // Nested concept names themselves take the genitive, without an extra expression cue.
    all_russian_speech_modes("<math><mrow intent='image(domain($f))'><mi>Im</mi><mi arg='f'>F</mi></mrow></math>", "образ области определения заглавной эф")?;
    all_russian_speech_modes("<math><mrow intent='domain(codomain(image($f)))'><mi>Dom</mi><mi arg='f'>F</mi></mrow></math>", "область определения целевого множества образа заглавной эф")
}

#[test]
fn function_set_compound_operand() -> Result<()> {
    // A compound operand is introduced with genitive "выражения"; its internal letters stay nominative.
    all_russian_speech_modes("<math><mrow intent='image($f)'><mi>Im</mi><mrow arg='f'><mi>F</mi><mo>+</mo><mi>G</mi></mrow></mrow></math>", "образ выражения заглавная эф плюс заглавная гэ")
}

#[test]
fn function_set_name_only() -> Result<()> {
    // A standalone operator name has no operand to inflect and needs no connector.
    all_russian_speech_modes("<math><mi intent='image'>Im</mi></math>", "образ")?;
    all_russian_speech_modes("<math><mi intent='domain'>Dom</mi></math>", "область определения")?;
    all_russian_speech_modes("<math><mi intent='codomain'>Cod</mi></math>", "целевое множество")
}

#[test]
fn complex_parts_with_style_wrappers() -> Result<()> {
    // TeX4ht splits upright function names into mi children of mstyle; neither name may disappear.
    let expr = "<math><mi>a</mi><mo>=</mo><mstyle mathvariant='normal'><mi>R</mi><mi>e</mi></mstyle><mspace width='0.28em'/><mi>z</mi><mo>,</mo><mi>b</mi><mo>=</mo><mstyle mathvariant='normal'><mi>I</mi><mi>m</mi></mstyle><mspace width='0.28em'/><mi>z</mi></math>";
    test("ru", "ClearSpeak", expr, "а равно действительная часть от зэт запятая, бэ равно мнимая часть от зэт")?;
    test("ru", "SimpleSpeak", expr, "а равно действительная часть от зэт запятая, бэ равно мнимая часть от зэт")
}

#[test]
fn complex_parts_without_style_wrappers() -> Result<()> {
    // Access8Math removes mstyle wrappers; split Re and Im still denote functions of z.
    let expr = "<math><mi>a</mi><mo>=</mo><mi>R</mi><mi>e</mi><mspace width='0.28em'/><mi>z</mi><mo>,</mo><mi>b</mi><mo>=</mo><mi>I</mi><mi>m</mi><mspace width='0.28em'/><mi>z</mi></math>";
    test("ru", "ClearSpeak", expr, "а равно действительная часть от зэт запятая, бэ равно мнимая часть от зэт")?;
    test("ru", "SimpleSpeak", expr, "а равно действительная часть от зэт запятая, бэ равно мнимая часть от зэт")
}

#[test]
fn complex_parts_lowercase() -> Result<()> {
    // Lowercase re/im must be recognized just like Re/Im, without an explicit apply operator.
    let expr = "<math><mi>re</mi><mi>z</mi><mo>,</mo><mi>im</mi><mi>z</mi></math>";
    test("ru", "ClearSpeak", expr, "действительная часть от зэт запятая, мнимая часть от зэт")?;
    test("ru", "SimpleSpeak", expr, "действительная часть от зэт запятая, мнимая часть от зэт")
}

#[test]
fn imaginary_part_name() -> Result<()> {
    // The imaginary part is a real component of a complex number, not a "complex part".
    let expr = r#"<math><mi intent="imaginary-part">Im</mi></math>"#;
    test("ru", "SimpleSpeak", expr, "мнимая часть")?;
    test("ru", "ClearSpeak", expr, "мнимая часть")?;
    return Ok(());
}

#[test]
fn silent_intent() -> Result<()> {
    let expr = "<math> <mrow intent='testing:silent($arg1, $arg2)'><mn arg='arg1'>2</mn> <mi arg='arg2'>x</mi></mrow> </math>";
    test("ru", "SimpleSpeak", expr, "2 икс")?;
    test("ru", "LiteralSpeak", expr, "2 икс")?;
    return Ok(());
}

#[test]
fn prefix_intent() -> Result<()> {
    let expr = r#"<math><msup intent='testing:prefix($x)'> <mi arg='x'>x</mi> <mi>T</mi> </msup> </math>"#;
    test("ru", "SimpleSpeak", expr, "testing икс")?;
    return Ok(());
}

#[test]
fn postfix_intent() -> Result<()> {
    let expr = r#"<math><msup intent='testing:postfix($x)'> <mi arg='x'>x</mi> <mi>T</mi> </msup> </math>"#;
    test("ru", "SimpleSpeak", expr, "икс testing")?;
    return Ok(());
}

#[test]
fn infix_intent() -> Result<()> {
    let expr = r#"<math><mrow intent='testing:infix($x, $y, $z, 2)'>
        <mi arg='x'>x</mi>
        <mi arg='y'>y</mi>
        <mi arg='z'>z</mi>
    </mrow> </math>"#;
    test("ru", "SimpleSpeak", expr, "икс testing игрек testing зэт testing 2")?;
    return Ok(());
}

#[test]
fn infix_intent_no_args() -> Result<()> {
    // this is illegal intent, so it is just an mrow with one child
    let expr = r#"<math><mrow intent='testing:infix()'>
        <mi arg='x'>x</mi>
    </mrow> </math>"#;
    test("ru", "SimpleSpeak", expr, "икс")?;
    return Ok(());
}

#[test]
fn infix_intent_one_arg() -> Result<()> {
    let expr = r#"<math><mrow intent='testing:infix($x)'>
        <mi arg='x'>x</mi>
    </mrow> </math>"#;
    // Note: we say the intent name because there are infix plus/minus with a single arg due to continued rows or combined columns
    test("ru", "SimpleSpeak", expr, "testing икс")?;
    return Ok(());
}

#[test]
fn function_intent() -> Result<()> {
    let expr = r#"<math><mrow intent='testing:function($x, $y, $z, 2)'>
        <mi arg='x'>x</mi>
        <mi arg='y'>y</mi>
        <mi arg='z'>z</mi>
    </mrow> </math>"#;
    test("ru", "SimpleSpeak", expr, "testing от икс запятая; игрек запятая, зэт запятая, 2")?;
    return Ok(());
}

#[test]
fn function_no_args_intent() -> Result<()> {
    // this is illegal intent, so it is just an mrow with one child
    let expr = r#"<math><mrow intent='testing:function()'>
        <mi arg='x'>x</mi>
    </mrow> </math>"#;
    test("ru", "SimpleSpeak", expr, "икс")?;
    return Ok(());
}

#[test]
fn function_one_arg_intent() -> Result<()> {
    let expr = r#"<math><mrow intent='testing:function($x)'>
        <mi arg='x'>x</mi>
    </mrow> </math>"#;
    test("ru", "SimpleSpeak", expr, "testing от икс")?;
    return Ok(());
}

#[test]
fn silent_intent_mi() -> Result<()> {
    let expr = "<math> <mn>2</mn> <mi intent=':silent'>x</mi></math>";
    test("ru", "SimpleSpeak", expr, "2")?;
    test("ru", "ClearSpeak", expr, "2")?;
    return Ok(());
}

#[test]
fn silent_intent_msup() -> Result<()> {
    let expr = "<math>
        <msup intent='index:silent($H,$n)'>
            <mi arg='H' mathvariant='normal'>H</mi>
            <mn arg='n'>2</mn>
        </msup></math>";
    test("ru", "SimpleSpeak", expr, "заглавная аш 2")?;
    test("ru", "ClearSpeak", expr, "заглавная аш 2")?;
    return Ok(());
}

#[test]
fn silent_intent_underscore() -> Result<()> {
    let expr = "<math>
        <msup intent='_($H,$n)'>
            <mi arg='H' mathvariant='normal'>H</mi>
            <mn arg='n'>2</mn>
        </msup></math>";
    test("ru", "SimpleSpeak", expr, "заглавная аш 2")?;
    test("ru", "ClearSpeak", expr, "заглавная аш 2")?;
    return Ok(());
}

#[test]
fn intent_prob_x() -> Result<()> {
    let expr = "<math>
    <msup intent='$op($arg)'>
        <mi arg='arg'>x</mi>
        <mi arg='op' intent='probability' mathvariant='normal'>P</mi>
    </msup></math>";
    // An indirectly referenced W3C name keeps its Russian noun grammar in every speech mode.
    all_russian_speech_modes(expr, "вероятность икс")?;
    return Ok(());
}

// Reviewed W3C names and argument-grammar regressions.
// W3C Core Concept List, reviewed 2026-10-30: https://w3c.github.io/mathml-docs/intent-core-concepts/
#[test]
fn w3c_core_adjugate() -> Result<()> {
    // W3C adjugate: localizes the concept and retains its argument, in both styles and all verbosities.
    let expr = "<math><mrow intent='adjugate($a0)'><mi arg='a0'>x</mi></mrow></math>";
    all_russian_speech_modes(expr, "присоединённая матрица икс")
}

#[test]
fn w3c_core_and() -> Result<()> {
    // W3C and: localizes the concept and retains both arguments, in both styles and all verbosities.
    let expr = "<math><mrow intent='and($a0,$a1)'><mi arg='a0'>x</mi><mi arg='a1'>y</mi></mrow></math>";
    all_russian_speech_modes(expr, "икс и игрек")
}

#[test]
fn w3c_core_angle() -> Result<()> {
    // W3C angle: localizes the concept and retains its argument, in both styles and all verbosities.
    let expr = "<math><mrow intent='angle($a0)'><mi arg='a0'>x</mi></mrow></math>";
    all_russian_speech_modes(expr, "угол икс")
}

#[test]
fn w3c_core_angle_measure() -> Result<()> {
    // W3C angle-measure: localizes the concept and retains its argument, in both styles and all verbosities.
    let expr = "<math><mrow intent='angle-measure($a0)'><mi arg='a0'>x</mi></mrow></math>";
    all_russian_speech_modes(expr, "мера угла икс")
}

#[test]
fn w3c_core_annotation() -> Result<()> {
    // W3C annotation: localizes the concept and retains both arguments, in both styles and all verbosities.
    let expr = "<math><mrow intent='annotation($a0,$a1)'><mi arg='a0'>x</mi><mi arg='a1'>y</mi></mrow></math>";
    all_russian_speech_modes(expr, "икс с пояснением, игрек")
}

#[test]
fn w3c_core_applied_to() -> Result<()> {
    // W3C applied-to: localizes the concept and retains both arguments, in both styles and all verbosities.
    let expr = "<math><mrow intent='applied-to($a0,$a1)'><mi arg='a0'>x</mi><mi arg='a1'>y</mi></mrow></math>";
    all_russian_speech_modes(expr, "икс от игрек")
}

#[test]
fn w3c_core_approximately() -> Result<()> {
    // W3C approximately: localizes the concept and retains both arguments, in both styles and all verbosities.
    let expr = "<math><mrow intent='approximately($a0,$a1)'><mi arg='a0'>x</mi><mi arg='a1'>y</mi></mrow></math>";
    all_russian_speech_modes(expr, "икс приблизительно равно, игрек")
}

#[test]
fn w3c_core_arc_hyperbolic_cosecant() -> Result<()> {
    // W3C arc-hyperbolic-cosecant: localizes the concept and retains its argument, in both styles and all verbosities.
    let expr = "<math><mrow intent='arc-hyperbolic-cosecant($a0)'><mi arg='a0'>x</mi></mrow></math>";
    russian_speech_verbosities(expr, "обратная функция гиперболического косеканса икс", "обратная функция гиперболического косеканса от икс")
}

#[test]
fn w3c_core_arc_hyperbolic_cosine() -> Result<()> {
    // W3C arc-hyperbolic-cosine: localizes the concept and retains its argument, in both styles and all verbosities.
    let expr = "<math><mrow intent='arc-hyperbolic-cosine($a0)'><mi arg='a0'>x</mi></mrow></math>";
    russian_speech_verbosities(expr, "обратная функция гиперболического косинуса икс", "обратная функция гиперболического косинуса от икс")
}

#[test]
fn w3c_core_arc_hyperbolic_cotangent() -> Result<()> {
    // W3C arc-hyperbolic-cotangent: localizes the concept and retains its argument, in both styles and all verbosities.
    let expr = "<math><mrow intent='arc-hyperbolic-cotangent($a0)'><mi arg='a0'>x</mi></mrow></math>";
    russian_speech_verbosities(expr, "обратная функция гиперболического котангенса икс", "обратная функция гиперболического котангенса от икс")
}

#[test]
fn w3c_core_arc_hyperbolic_secant() -> Result<()> {
    // W3C arc-hyperbolic-secant: localizes the concept and retains its argument, in both styles and all verbosities.
    let expr = "<math><mrow intent='arc-hyperbolic-secant($a0)'><mi arg='a0'>x</mi></mrow></math>";
    russian_speech_verbosities(expr, "обратная функция гиперболического секанса икс", "обратная функция гиперболического секанса от икс")
}

#[test]
fn w3c_core_arc_hyperbolic_sine() -> Result<()> {
    // W3C arc-hyperbolic-sine: localizes the concept and retains its argument, in both styles and all verbosities.
    let expr = "<math><mrow intent='arc-hyperbolic-sine($a0)'><mi arg='a0'>x</mi></mrow></math>";
    russian_speech_verbosities(expr, "обратная функция гиперболического синуса икс", "обратная функция гиперболического синуса от икс")
}

#[test]
fn w3c_core_arc_hyperbolic_tangent() -> Result<()> {
    // W3C arc-hyperbolic-tangent: localizes the concept and retains its argument, in both styles and all verbosities.
    let expr = "<math><mrow intent='arc-hyperbolic-tangent($a0)'><mi arg='a0'>x</mi></mrow></math>";
    russian_speech_verbosities(expr, "обратная функция гиперболического тангенса икс", "обратная функция гиперболического тангенса от икс")
}

#[test]
fn w3c_core_arccosecant() -> Result<()> {
    // W3C arccosecant: localizes the concept and retains its argument, in both styles and all verbosities.
    let expr = "<math><mrow intent='arccosecant($a0)'><mi arg='a0'>x</mi></mrow></math>";
    russian_speech_verbosities(expr, "арккосеканс икс", "арккосеканс от икс")
}

#[test]
fn w3c_core_arccosine() -> Result<()> {
    // W3C arccosine: localizes the concept and retains its argument, in both styles and all verbosities.
    let expr = "<math><mrow intent='arccosine($a0)'><mi arg='a0'>x</mi></mrow></math>";
    russian_speech_verbosities(expr, "арккосинус икс", "арккосинус от икс")
}

#[test]
fn w3c_core_arccotangent() -> Result<()> {
    // W3C arccotangent: localizes the concept and retains its argument, in both styles and all verbosities.
    let expr = "<math><mrow intent='arccotangent($a0)'><mi arg='a0'>x</mi></mrow></math>";
    russian_speech_verbosities(expr, "арккотангенс икс", "арккотангенс от икс")
}

#[test]
fn w3c_core_arcsecant() -> Result<()> {
    // W3C arcsecant: localizes the concept and retains its argument, in both styles and all verbosities.
    let expr = "<math><mrow intent='arcsecant($a0)'><mi arg='a0'>x</mi></mrow></math>";
    russian_speech_verbosities(expr, "арксеканс икс", "арксеканс от икс")
}

#[test]
fn w3c_core_arcsine() -> Result<()> {
    // W3C arcsine: localizes the concept and retains its argument, in both styles and all verbosities.
    let expr = "<math><mrow intent='arcsine($a0)'><mi arg='a0'>x</mi></mrow></math>";
    russian_speech_verbosities(expr, "арксинус икс", "арксинус от икс")
}

#[test]
fn w3c_core_arctangent() -> Result<()> {
    // W3C arctangent: localizes the concept and retains its argument, in both styles and all verbosities.
    let expr = "<math><mrow intent='arctangent($a0)'><mi arg='a0'>x</mi></mrow></math>";
    russian_speech_verbosities(expr, "арктангенс икс", "арктангенс от икс")
}

#[test]
fn w3c_core_area() -> Result<()> {
    // W3C area: localizes the concept and retains its argument, in both styles and all verbosities.
    let expr = "<math><mrow intent='area($a0)'><mi arg='a0'>x</mi></mrow></math>";
    all_russian_speech_modes(expr, "площадь икс")
}

#[test]
fn w3c_core_binomial_coefficient() -> Result<()> {
    // W3C binomial-coefficient: localizes the concept and retains both arguments, in both styles and all verbosities.
    let expr = "<math><mrow intent='binomial-coefficient($a0,$a1)'><mi arg='a0'>x</mi><mi arg='a1'>y</mi></mrow></math>";
    all_russian_speech_modes(expr, "число сочетаний из икс по игрек")
}

#[test]
fn w3c_core_braced_group() -> Result<()> {
    // W3C braced-group: localizes the concept and retains its argument, in both styles and all verbosities.
    let expr = "<math><mrow intent='braced-group($a0)'><mi arg='a0'>x</mi></mrow></math>";
    all_russian_speech_modes(expr, "группа икс")
}

#[test]
fn w3c_core_cardinality() -> Result<()> {
    // W3C cardinality: localizes the concept and retains its argument, in both styles and all verbosities.
    let expr = "<math><mrow intent='cardinality($a0)'><mi arg='a0'>x</mi></mrow></math>";
    all_russian_speech_modes(expr, "мощность икс")
}

#[test]
fn w3c_core_cartesian_coordinate() -> Result<()> {
    // W3C cartesian-coordinate: localizes the concept and retains both arguments, in both styles and all verbosities.
    let expr = "<math><mrow intent='cartesian-coordinate($a0,$a1)'><mi arg='a0'>x</mi><mi arg='a1'>y</mi></mrow></math>";
    all_russian_speech_modes(expr, "декартовы координаты икс запятая игрек")
}

#[test]
fn w3c_core_cartesian_product() -> Result<()> {
    // W3C cartesian-product: localizes the concept and retains both arguments, in both styles and all verbosities.
    let expr = "<math><mrow intent='cartesian-product($a0,$a1)'><mi arg='a0'>x</mi><mi arg='a1'>y</mi></mrow></math>";
    all_russian_speech_modes(expr, "икс декартово произведение; игрек")
}

#[test]
fn w3c_core_ceiling() -> Result<()> {
    // W3C ceiling: localizes the concept and retains its argument, in both styles and all verbosities.
    let expr = "<math><mrow intent='ceiling($a0)'><mi arg='a0'>x</mi></mrow></math>";
    all_russian_speech_modes(expr, "округление вверх икс")
}

#[test]
fn w3c_core_change() -> Result<()> {
    // W3C change: localizes the concept and retains its argument, in both styles and all verbosities.
    let expr = "<math><mrow intent='change($a0)'><mi arg='a0'>x</mi></mrow></math>";
    all_russian_speech_modes(expr, "изменение икс")
}

#[test]
fn w3c_core_complement() -> Result<()> {
    // W3C complement: localizes the concept and retains its argument, in both styles and all verbosities.
    let expr = "<math><mrow intent='complement($a0)'><mi arg='a0'>x</mi></mrow></math>";
    all_russian_speech_modes(expr, "дополнение икс")
}

#[test]
fn w3c_core_complex_arg() -> Result<()> {
    // W3C complex-arg: localizes the concept and retains its argument, in both styles and all verbosities.
    let expr = "<math><mrow intent='complex-arg($a0)'><mi arg='a0'>x</mi></mrow></math>";
    all_russian_speech_modes(expr, "аргумент икс")
}

#[test]
fn w3c_core_complex_conjugate() -> Result<()> {
    // W3C complex-conjugate: localizes the concept and retains its argument, in both styles and all verbosities.
    let expr = "<math><mrow intent='complex-conjugate($a0)'><mi arg='a0'>x</mi></mrow></math>";
    all_russian_speech_modes(expr, "комплексное сопряжение икс")
}

#[test]
fn w3c_core_composed_with() -> Result<()> {
    // W3C composed-with: localizes the concept and retains both arguments, in both styles and all verbosities.
    let expr = "<math><mrow intent='composed-with($a0,$a1)'><mi arg='a0'>x</mi><mi arg='a1'>y</mi></mrow></math>";
    all_russian_speech_modes(expr, "икс композиция с, игрек")
}

#[test]
fn w3c_core_conditional_probability() -> Result<()> {
    // W3C conditional-probability: localizes the concept and retains both arguments, in both styles and all verbosities.
    let expr = "<math><mrow intent='conditional-probability($a0,$a1)'><mi arg='a0'>x</mi><mi arg='a1'>y</mi></mrow></math>";
    all_russian_speech_modes(expr, "вероятность икс при условии игрек")
}

#[test]
fn w3c_core_congruent() -> Result<()> {
    // W3C congruent: localizes the concept and retains both arguments, in both styles and all verbosities.
    let expr = "<math><mrow intent='congruent($a0,$a1)'><mi arg='a0'>x</mi><mi arg='a1'>y</mi></mrow></math>";
    all_russian_speech_modes(expr, "икс конгруэнтно, игрек")
}

#[test]
fn w3c_core_constraint() -> Result<()> {
    // W3C constraint: localizes the concept and retains both arguments, in both styles and all verbosities.
    let expr = "<math><mrow intent='constraint($a0,$a1)'><mi arg='a0'>x</mi><mi arg='a1'>y</mi></mrow></math>";
    all_russian_speech_modes(expr, "икс при условии, игрек")
}

#[test]
fn w3c_core_cosecant() -> Result<()> {
    // W3C cosecant: localizes the concept and retains its argument, in both styles and all verbosities.
    let expr = "<math><mrow intent='cosecant($a0)'><mi arg='a0'>x</mi></mrow></math>";
    russian_speech_verbosities(expr, "косеканс икс", "косеканс от икс")
}

#[test]
fn w3c_core_cosine() -> Result<()> {
    // W3C cosine: localizes the concept and retains its argument, in both styles and all verbosities.
    let expr = "<math><mrow intent='cosine($a0)'><mi arg='a0'>x</mi></mrow></math>";
    russian_speech_verbosities(expr, "косинус икс", "косинус от икс")
}

#[test]
fn w3c_core_cotangent() -> Result<()> {
    // W3C cotangent: localizes the concept and retains its argument, in both styles and all verbosities.
    let expr = "<math><mrow intent='cotangent($a0)'><mi arg='a0'>x</mi></mrow></math>";
    russian_speech_verbosities(expr, "котангенс икс", "котангенс от икс")
}

#[test]
fn w3c_core_defined_as() -> Result<()> {
    // W3C defined-as: localizes the concept and retains both arguments, in both styles and all verbosities.
    let expr = "<math><mrow intent='defined-as($a0,$a1)'><mi arg='a0'>x</mi><mi arg='a1'>y</mi></mrow></math>";
    all_russian_speech_modes(expr, "икс определяется как, игрек")
}

#[test]
fn w3c_core_diameter() -> Result<()> {
    // W3C diameter: localizes the concept and retains its argument, in both styles and all verbosities.
    let expr = "<math><mrow intent='diameter($a0)'><mi arg='a0'>x</mi></mrow></math>";
    all_russian_speech_modes(expr, "диаметр икс")
}

#[test]
fn w3c_core_differential_d() -> Result<()> {
    // W3C differential-d: localizes the standalone concept name, in both styles and all verbosities.
    let expr = "<math><mi intent='differential-d'>x</mi></math>";
    all_russian_speech_modes(expr, "дэ")
}

#[test]
fn w3c_core_dimensional_product() -> Result<()> {
    // W3C dimensional-product: localizes the concept and retains both arguments, in both styles and all verbosities.
    let expr = "<math><mrow intent='dimensional-product($a0,$a1)'><mi arg='a0'>x</mi><mi arg='a1'>y</mi></mrow></math>";
    all_russian_speech_modes(expr, "икс на игрек")
}

#[test]
fn w3c_core_direct_product() -> Result<()> {
    // W3C direct-product: localizes the concept and retains both arguments, in both styles and all verbosities.
    let expr = "<math><mrow intent='direct-product($a0,$a1)'><mi arg='a0'>x</mi><mi arg='a1'>y</mi></mrow></math>";
    all_russian_speech_modes(expr, "икс прямое произведение, игрек")
}

#[test]
fn w3c_core_directed_line_segment() -> Result<()> {
    // W3C directed-line-segment: localizes the concept and retains both arguments, in both styles and all verbosities.
    let expr = "<math><mrow intent='directed-line-segment($a0,$a1)'><mi arg='a0'>x</mi><mi arg='a1'>y</mi></mrow></math>";
    all_russian_speech_modes(expr, "направленный отрезок икс игрек")
}

#[test]
fn w3c_core_distance() -> Result<()> {
    // W3C distance: localizes the concept and retains its argument, in both styles and all verbosities.
    let expr = "<math><mrow intent='distance($a0)'><mi arg='a0'>x</mi></mrow></math>";
    all_russian_speech_modes(expr, "расстояние икс")
}

#[test]
fn w3c_core_divided_by() -> Result<()> {
    // W3C divided-by: localizes the concept and retains both arguments, in both styles and all verbosities.
    let expr = "<math><mrow intent='divided-by($a0,$a1)'><mi arg='a0'>x</mi><mi arg='a1'>y</mi></mrow></math>";
    all_russian_speech_modes(expr, "икс разделить на, игрек")
}

#[test]
fn w3c_core_divides() -> Result<()> {
    // W3C divides: localizes the concept and retains both arguments, in both styles and all verbosities.
    let expr = "<math><mrow intent='divides($a0,$a1)'><mi arg='a0'>x</mi><mi arg='a1'>y</mi></mrow></math>";
    all_russian_speech_modes(expr, "икс делит игрек")
}

#[test]
fn w3c_core_does_not_belong_to() -> Result<()> {
    // W3C does-not-belong-to: localizes the concept and retains both arguments, in both styles and all verbosities.
    let expr = "<math><mrow intent='does-not-belong-to($a0,$a1)'><mi arg='a0'>x</mi><mi arg='a1'>y</mi></mrow></math>";
    all_russian_speech_modes(expr, "икс не принадлежит, игрек")
}

#[test]
fn w3c_core_does_not_divide() -> Result<()> {
    // W3C does-not-divide: localizes the concept and retains both arguments, in both styles and all verbosities.
    let expr = "<math><mrow intent='does-not-divide($a0,$a1)'><mi arg='a0'>x</mi><mi arg='a1'>y</mi></mrow></math>";
    all_russian_speech_modes(expr, "икс не делит игрек")
}

#[test]
fn w3c_core_downwards_diagonal_ellipsis() -> Result<()> {
    // W3C downwards-diagonal-ellipsis: localizes the concept and retains both arguments, in both styles and all verbosities.
    let expr = "<math><mrow intent='downwards-diagonal-ellipsis($a0,$a1)'><mi arg='a0'>x</mi><mi arg='a1'>y</mi></mrow></math>";
    all_russian_speech_modes(expr, "икс диагональное многоточие вниз; игрек")
}

#[test]
fn w3c_core_element_of() -> Result<()> {
    // W3C element-of: localizes the concept and retains both arguments, in both styles and all verbosities.
    let expr = "<math><mrow intent='element-of($a0,$a1)'><mi arg='a0'>x</mi><mi arg='a1'>y</mi></mrow></math>";
    all_russian_speech_modes(expr, "икс принадлежит, игрек")
}

#[test]
fn w3c_core_ellipsis() -> Result<()> {
    // W3C ellipsis: localizes the concept and retains both arguments, in both styles and all verbosities.
    let expr = "<math><mrow intent='ellipsis($a0,$a1)'><mi arg='a0'>x</mi><mi arg='a1'>y</mi></mrow></math>";
    all_russian_speech_modes(expr, "икс многоточие, игрек")
}

#[test]
fn w3c_core_embellished_name() -> Result<()> {
    // W3C embellished-name: localizes the concept and retains both arguments, in both styles and all verbosities.
    let expr = "<math><mrow intent='embellished-name($a0,$a1)'><mi arg='a0'>x</mi><mi arg='a1'>y</mi></mrow></math>";
    all_russian_speech_modes(expr, "икс с обозначением, игрек")
}

#[test]
fn w3c_core_empty_set() -> Result<()> {
    // W3C empty-set: localizes the standalone concept name, in both styles and all verbosities.
    let expr = "<math><mi intent='empty-set'>x</mi></math>";
    all_russian_speech_modes(expr, "пустое множество")
}

#[test]
fn w3c_core_equals() -> Result<()> {
    // W3C equals: localizes the concept and retains both arguments, in both styles and all verbosities.
    let expr = "<math><mrow intent='equals($a0,$a1)'><mi arg='a0'>x</mi><mi arg='a1'>y</mi></mrow></math>";
    all_russian_speech_modes(expr, "икс равно игрек")
}

#[test]
fn w3c_core_equivalent_to() -> Result<()> {
    // W3C equivalent-to: localizes the concept and retains both arguments, in both styles and all verbosities.
    let expr = "<math><mrow intent='equivalent-to($a0,$a1)'><mi arg='a0'>x</mi><mi arg='a1'>y</mi></mrow></math>";
    all_russian_speech_modes(expr, "икс эквивалентно, игрек")
}

#[test]
fn w3c_core_evaluated_at() -> Result<()> {
    // W3C evaluated-at: localizes the concept and retains both arguments, in both styles and all verbosities.
    let expr = "<math><mrow intent='evaluated-at($a0,$a1)'><mi arg='a0'>x</mi><mi arg='a1'>y</mi></mrow></math>";
    all_russian_speech_modes(expr, "икс вычисленное при, игрек")
}

#[test]
fn w3c_core_evaluates_to() -> Result<()> {
    // W3C evaluates-to: localizes the concept and retains both arguments, in both styles and all verbosities.
    let expr = "<math><mrow intent='evaluates-to($a0,$a1)'><mi arg='a0'>x</mi><mi arg='a1'>y</mi></mrow></math>";
    all_russian_speech_modes(expr, "икс вычисляется как, игрек")
}

#[test]
fn w3c_core_exponential() -> Result<()> {
    // W3C exponential: localizes the concept and retains its argument, in both styles and all verbosities.
    let expr = "<math><mrow intent='exponential($a0)'><mi arg='a0'>x</mi></mrow></math>";
    russian_speech_verbosities(expr, "экспонента икс", "экспонента от икс")
}

#[test]
fn w3c_core_exponential_e() -> Result<()> {
    // W3C exponential-e: localizes the standalone concept name, in both styles and all verbosities.
    let expr = "<math><mi intent='exponential-e'>x</mi></math>";
    all_russian_speech_modes(expr, "число е")
}

#[test]
fn w3c_core_factorial() -> Result<()> {
    // W3C factorial: localizes the concept and retains its argument, in both styles and all verbosities.
    let expr = "<math><mrow intent='factorial($a0)'><mi arg='a0'>x</mi></mrow></math>";
    all_russian_speech_modes(expr, "икс факториал")
}

#[test]
fn w3c_core_fenced_group() -> Result<()> {
    // W3C fenced-group: localizes the concept and retains its argument, in both styles and all verbosities.
    let expr = "<math><mrow intent='fenced-group($a0)'><mi arg='a0'>x</mi></mrow></math>";
    all_russian_speech_modes(expr, "выражение в скобках икс")
}

#[test]
fn w3c_core_floor() -> Result<()> {
    // W3C floor: localizes the concept and retains its argument, in both styles and all verbosities.
    let expr = "<math><mrow intent='floor($a0)'><mi arg='a0'>x</mi></mrow></math>";
    all_russian_speech_modes(expr, "округление вниз икс")
}

#[test]
fn w3c_core_for_all() -> Result<()> {
    // W3C for-all: localizes the concept and retains its argument, in both styles and all verbosities.
    let expr = "<math><mrow intent='for-all($a0)'><mi arg='a0'>x</mi></mrow></math>";
    all_russian_speech_modes(expr, "для всех икс")
}

#[test]
fn w3c_core_fractional_part() -> Result<()> {
    // W3C fractional-part: localizes the concept and retains its argument, in both styles and all verbosities.
    let expr = "<math><mrow intent='fractional-part($a0)'><mi arg='a0'>x</mi></mrow></math>";
    all_russian_speech_modes(expr, "дробная часть икс")
}

#[test]
fn w3c_core_given() -> Result<()> {
    // W3C given: localizes the concept and retains both arguments, in both styles and all verbosities.
    let expr = "<math><mrow intent='given($a0,$a1)'><mi arg='a0'>x</mi><mi arg='a1'>y</mi></mrow></math>";
    all_russian_speech_modes(expr, "икс при условии, игрек")
}

#[test]
fn w3c_core_golden_ratio() -> Result<()> {
    // W3C golden-ratio: localizes the standalone concept name, in both styles and all verbosities.
    let expr = "<math><mi intent='golden-ratio'>x</mi></math>";
    all_russian_speech_modes(expr, "золотое сечение")
}

#[test]
fn w3c_core_greater_than() -> Result<()> {
    // W3C greater-than: localizes the concept and retains both arguments, in both styles and all verbosities.
    let expr = "<math><mrow intent='greater-than($a0,$a1)'><mi arg='a0'>x</mi><mi arg='a1'>y</mi></mrow></math>";
    all_russian_speech_modes(expr, "икс больше игрек")
}

#[test]
fn w3c_core_greater_than_or_equal_to() -> Result<()> {
    // W3C greater-than-or-equal-to: localizes the concept and retains both arguments, in both styles and all verbosities.
    let expr = "<math><mrow intent='greater-than-or-equal-to($a0,$a1)'><mi arg='a0'>x</mi><mi arg='a1'>y</mi></mrow></math>";
    all_russian_speech_modes(expr, "икс больше или равно, игрек")
}

#[test]
fn w3c_core_highlight() -> Result<()> {
    // W3C highlight: localizes the concept and retains its argument, in both styles and all verbosities.
    let expr = "<math><mrow intent='highlight($a0)'><mi arg='a0'>x</mi></mrow></math>";
    all_russian_speech_modes(expr, "икс выделено")
}

#[test]
fn w3c_core_hyperbolic_cosecant() -> Result<()> {
    // W3C hyperbolic-cosecant: localizes the concept and retains its argument, in both styles and all verbosities.
    let expr = "<math><mrow intent='hyperbolic-cosecant($a0)'><mi arg='a0'>x</mi></mrow></math>";
    russian_speech_verbosities(expr, "гиперболический косеканс икс", "гиперболический косеканс от икс")
}

#[test]
fn w3c_core_hyperbolic_cosine() -> Result<()> {
    // W3C hyperbolic-cosine: localizes the concept and retains its argument, in both styles and all verbosities.
    let expr = "<math><mrow intent='hyperbolic-cosine($a0)'><mi arg='a0'>x</mi></mrow></math>";
    russian_speech_verbosities(expr, "гиперболический косинус икс", "гиперболический косинус от икс")
}

#[test]
fn w3c_core_hyperbolic_cotangent() -> Result<()> {
    // W3C hyperbolic-cotangent: localizes the concept and retains its argument, in both styles and all verbosities.
    let expr = "<math><mrow intent='hyperbolic-cotangent($a0)'><mi arg='a0'>x</mi></mrow></math>";
    russian_speech_verbosities(expr, "гиперболический котангенс икс", "гиперболический котангенс от икс")
}

#[test]
fn w3c_core_hyperbolic_secant() -> Result<()> {
    // W3C hyperbolic-secant: localizes the concept and retains its argument, in both styles and all verbosities.
    let expr = "<math><mrow intent='hyperbolic-secant($a0)'><mi arg='a0'>x</mi></mrow></math>";
    russian_speech_verbosities(expr, "гиперболический секанс икс", "гиперболический секанс от икс")
}

#[test]
fn w3c_core_hyperbolic_sine() -> Result<()> {
    // W3C hyperbolic-sine: localizes the concept and retains its argument, in both styles and all verbosities.
    let expr = "<math><mrow intent='hyperbolic-sine($a0)'><mi arg='a0'>x</mi></mrow></math>";
    russian_speech_verbosities(expr, "гиперболический синус икс", "гиперболический синус от икс")
}

#[test]
fn w3c_core_hyperbolic_tangent() -> Result<()> {
    // W3C hyperbolic-tangent: localizes the concept and retains its argument, in both styles and all verbosities.
    let expr = "<math><mrow intent='hyperbolic-tangent($a0)'><mi arg='a0'>x</mi></mrow></math>";
    russian_speech_verbosities(expr, "гиперболический тангенс икс", "гиперболический тангенс от икс")
}

#[test]
fn w3c_core_identically_equals() -> Result<()> {
    // W3C identically-equals: localizes the concept and retains both arguments, in both styles and all verbosities.
    let expr = "<math><mrow intent='identically-equals($a0,$a1)'><mi arg='a0'>x</mi><mi arg='a1'>y</mi></mrow></math>";
    all_russian_speech_modes(expr, "икс тождественно равно, игрек")
}

#[test]
fn w3c_core_identity_matrix() -> Result<()> {
    // W3C identity-matrix: localizes the standalone concept name, in both styles and all verbosities.
    let expr = "<math><mi intent='identity-matrix'>x</mi></math>";
    all_russian_speech_modes(expr, "единичная матрица")
}

#[test]
fn w3c_core_if_and_only_if() -> Result<()> {
    // W3C if-and-only-if: localizes the concept and retains both arguments, in both styles and all verbosities.
    let expr = "<math><mrow intent='if-and-only-if($a0,$a1)'><mi arg='a0'>x</mi><mi arg='a1'>y</mi></mrow></math>";
    all_russian_speech_modes(expr, "икс тогда и только тогда когда; игрек")
}

#[test]
fn w3c_core_imaginary_i() -> Result<()> {
    // W3C imaginary-i: localizes the standalone concept name, in both styles and all verbosities.
    let expr = "<math><mi intent='imaginary-i'>x</mi></math>";
    all_russian_speech_modes(expr, "мнимая единица")
}

#[test]
fn w3c_core_implies() -> Result<()> {
    // W3C implies: localizes the concept and retains both arguments, in both styles and all verbosities.
    let expr = "<math><mrow intent='implies($a0,$a1)'><mi arg='a0'>x</mi><mi arg='a1'>y</mi></mrow></math>";
    all_russian_speech_modes(expr, "икс влечёт игрек")
}

#[test]
fn w3c_core_inner_product() -> Result<()> {
    // W3C inner-product: localizes the concept and retains both arguments, in both styles and all verbosities.
    let expr = "<math><mrow intent='inner-product($a0,$a1)'><mi arg='a0'>x</mi><mi arg='a1'>y</mi></mrow></math>";
    all_russian_speech_modes(expr, "икс скалярное произведение; игрек")
}

#[test]
fn w3c_core_intersection() -> Result<()> {
    // W3C intersection: localizes the concept and retains both arguments, in both styles and all verbosities.
    let expr = "<math><mrow intent='intersection($a0,$a1)'><mi arg='a0'>x</mi><mi arg='a1'>y</mi></mrow></math>";
    all_russian_speech_modes(expr, "икс пересечение, игрек")
}

#[test]
fn w3c_core_inverse() -> Result<()> {
    // W3C inverse: localizes the concept and retains its argument, in both styles and all verbosities.
    let expr = "<math><mrow intent='inverse($a0)'><mi arg='a0'>x</mi></mrow></math>";
    all_russian_speech_modes(expr, "обратный к икс")
}

#[test]
fn w3c_core_least_common_denominator() -> Result<()> {
    // W3C least-common-denominator: localizes the concept and retains both arguments, in both styles and all verbosities.
    let expr = "<math><mrow intent='least-common-denominator($a0,$a1)'><mi arg='a0'>x</mi><mi arg='a1'>y</mi></mrow></math>";
    all_russian_speech_modes(expr, "наименьший общий знаменатель икс и игрек")
}

#[test]
fn w3c_core_length() -> Result<()> {
    // W3C length: localizes the concept and retains its argument, in both styles and all verbosities.
    let expr = "<math><mrow intent='length($a0)'><mi arg='a0'>x</mi></mrow></math>";
    all_russian_speech_modes(expr, "длина икс")
}

#[test]
fn w3c_core_less_than() -> Result<()> {
    // W3C less-than: localizes the concept and retains both arguments, in both styles and all verbosities.
    let expr = "<math><mrow intent='less-than($a0,$a1)'><mi arg='a0'>x</mi><mi arg='a1'>y</mi></mrow></math>";
    all_russian_speech_modes(expr, "икс меньше игрек")
}

#[test]
fn w3c_core_less_than_or_equal_to() -> Result<()> {
    // W3C less-than-or-equal-to: localizes the concept and retains both arguments, in both styles and all verbosities.
    let expr = "<math><mrow intent='less-than-or-equal-to($a0,$a1)'><mi arg='a0'>x</mi><mi arg='a1'>y</mi></mrow></math>";
    all_russian_speech_modes(expr, "икс меньше или равно, игрек")
}

#[test]
fn w3c_core_line() -> Result<()> {
    // W3C line: localizes the concept and retains both arguments, in both styles and all verbosities.
    let expr = "<math><mrow intent='line($a0,$a1)'><mi arg='a0'>x</mi><mi arg='a1'>y</mi></mrow></math>";
    all_russian_speech_modes(expr, "прямая икс игрек")
}

#[test]
fn w3c_core_list() -> Result<()> {
    // W3C list: localizes the standalone concept name, in both styles and all verbosities.
    let expr = "<math><mi intent='list'>x</mi></math>";
    all_russian_speech_modes(expr, "список")
}

#[test]
fn w3c_core_list_separator() -> Result<()> {
    // W3C list-separator: localizes the concept and retains both arguments, in both styles and all verbosities.
    let expr = "<math><mrow intent='list-separator($a0,$a1)'><mi arg='a0'>x</mi><mi arg='a1'>y</mi></mrow></math>";
    all_russian_speech_modes(expr, "икс запятая игрек")
}

#[test]
fn w3c_core_logarithm() -> Result<()> {
    // W3C logarithm: localizes the concept and retains both arguments, in both styles and all verbosities.
    let expr = "<math><mrow intent='logarithm($a0,$a1)'><mi arg='a0'>x</mi><mi arg='a1'>y</mi></mrow></math>";
    all_russian_speech_modes(expr, "логарифм по основанию игрек от икс")
}

#[test]
fn w3c_core_maps_to() -> Result<()> {
    // W3C maps-to: localizes the concept and retains both arguments, in both styles and all verbosities.
    let expr = "<math><mrow intent='maps-to($a0,$a1)'><mi arg='a0'>x</mi><mi arg='a1'>y</mi></mrow></math>";
    all_russian_speech_modes(expr, "икс отображается в, игрек")
}

#[test]
fn w3c_core_max() -> Result<()> {
    // W3C max: localizes the concept and retains its argument, in both styles and all verbosities.
    let expr = "<math><mrow intent='max($a0)'><mi arg='a0'>x</mi></mrow></math>";
    all_russian_speech_modes(expr, "максимум икс")
}

#[test]
fn w3c_core_mean() -> Result<()> {
    // W3C mean: localizes the concept and retains its argument, in both styles and all verbosities.
    let expr = "<math><mrow intent='mean($a0)'><mi arg='a0'>x</mi></mrow></math>";
    all_russian_speech_modes(expr, "среднее икс")
}

#[test]
fn w3c_core_measured_angle() -> Result<()> {
    // W3C measured-angle: localizes the concept and retains its argument, in both styles and all verbosities.
    let expr = "<math><mrow intent='measured-angle($a0)'><mi arg='a0'>x</mi></mrow></math>";
    all_russian_speech_modes(expr, "измеренный угол икс")
}

#[test]
fn w3c_core_median() -> Result<()> {
    // W3C median: localizes the concept and retains its argument, in both styles and all verbosities.
    let expr = "<math><mrow intent='median($a0)'><mi arg='a0'>x</mi></mrow></math>";
    all_russian_speech_modes(expr, "медиана икс")
}

#[test]
fn w3c_core_member_of() -> Result<()> {
    // W3C member-of: localizes the concept and retains both arguments, in both styles and all verbosities.
    let expr = "<math><mrow intent='member-of($a0,$a1)'><mi arg='a0'>x</mi><mi arg='a1'>y</mi></mrow></math>";
    all_russian_speech_modes(expr, "икс принадлежит, игрек")
}

#[test]
fn w3c_core_min() -> Result<()> {
    // W3C min: localizes the concept and retains its argument, in both styles and all verbosities.
    let expr = "<math><mrow intent='min($a0)'><mi arg='a0'>x</mi></mrow></math>";
    all_russian_speech_modes(expr, "минимум икс")
}

#[test]
fn w3c_core_minus_or_plus() -> Result<()> {
    // W3C minus-or-plus: localizes the concept and retains both arguments, in both styles and all verbosities.
    let expr = "<math><mrow intent='minus-or-plus($a0,$a1)'><mi arg='a0'>x</mi><mi arg='a1'>y</mi></mrow></math>";
    all_russian_speech_modes(expr, "икс минус плюс, игрек")
}

#[test]
fn w3c_core_mixed_fraction() -> Result<()> {
    // W3C mixed-fraction: localizes the concept and retains both arguments, in both styles and all verbosities.
    let expr = "<math><mrow intent='mixed-fraction($a0,$a1)'><mi arg='a0'>x</mi><mi arg='a1'>y</mi></mrow></math>";
    all_russian_speech_modes(expr, "икс и игрек")
}

#[test]
fn w3c_core_mode() -> Result<()> {
    // W3C mode: localizes the concept and retains its argument, in both styles and all verbosities.
    let expr = "<math><mrow intent='mode($a0)'><mi arg='a0'>x</mi></mrow></math>";
    all_russian_speech_modes(expr, "мода икс")
}

#[test]
fn w3c_core_not() -> Result<()> {
    // W3C not: localizes the concept and retains its argument, in both styles and all verbosities.
    let expr = "<math><mrow intent='not($a0)'><mi arg='a0'>x</mi></mrow></math>";
    all_russian_speech_modes(expr, "не икс")
}

#[test]
fn w3c_core_not_equal_to() -> Result<()> {
    // W3C not-equal-to: localizes the concept and retains both arguments, in both styles and all verbosities.
    let expr = "<math><mrow intent='not-equal-to($a0,$a1)'><mi arg='a0'>x</mi><mi arg='a1'>y</mi></mrow></math>";
    all_russian_speech_modes(expr, "икс не равно игрек")
}

#[test]
fn w3c_core_not_member_of() -> Result<()> {
    // W3C not-member-of: localizes the concept and retains both arguments, in both styles and all verbosities.
    let expr = "<math><mrow intent='not-member-of($a0,$a1)'><mi arg='a0'>x</mi><mi arg='a1'>y</mi></mrow></math>";
    all_russian_speech_modes(expr, "икс не принадлежит, игрек")
}

#[test]
fn w3c_core_not_parallel_to() -> Result<()> {
    // W3C not-parallel-to: localizes the concept and retains both arguments, in both styles and all verbosities.
    let expr = "<math><mrow intent='not-parallel-to($a0,$a1)'><mi arg='a0'>x</mi><mi arg='a1'>y</mi></mrow></math>";
    all_russian_speech_modes(expr, "икс не параллельно, игрек")
}

#[test]
fn w3c_core_not_subset() -> Result<()> {
    // W3C not-subset: localizes the concept and retains both arguments, in both styles and all verbosities.
    let expr = "<math><mrow intent='not-subset($a0,$a1)'><mi arg='a0'>x</mi><mi arg='a1'>y</mi></mrow></math>";
    all_russian_speech_modes(expr, "икс не является подмножеством; игрек")
}

#[test]
fn w3c_core_not_superset() -> Result<()> {
    // W3C not-superset: localizes the concept and retains both arguments, in both styles and all verbosities.
    let expr = "<math><mrow intent='not-superset($a0,$a1)'><mi arg='a0'>x</mi><mi arg='a1'>y</mi></mrow></math>";
    all_russian_speech_modes(expr, "икс не является надмножеством; игрек")
}

#[test]
fn w3c_core_number_of() -> Result<()> {
    // W3C number-of: localizes the concept and retains its argument, in both styles and all verbosities.
    let expr = "<math><mrow intent='number-of($a0)'><mi arg='a0'>x</mi></mrow></math>";
    all_russian_speech_modes(expr, "число икс")
}

#[test]
fn w3c_core_obtained_from() -> Result<()> {
    // W3C obtained-from: localizes the concept and retains both arguments, in both styles and all verbosities.
    let expr = "<math><mrow intent='obtained-from($a0,$a1)'><mi arg='a0'>x</mi><mi arg='a1'>y</mi></mrow></math>";
    all_russian_speech_modes(expr, "икс получено из, игрек")
}

#[test]
fn w3c_core_or() -> Result<()> {
    // W3C or: localizes the concept and retains both arguments, in both styles and all verbosities.
    let expr = "<math><mrow intent='or($a0,$a1)'><mi arg='a0'>x</mi><mi arg='a1'>y</mi></mrow></math>";
    all_russian_speech_modes(expr, "икс или игрек")
}

#[test]
fn w3c_core_ordered_pair() -> Result<()> {
    // W3C ordered-pair: localizes the concept and retains both arguments, in both styles and all verbosities.
    let expr = "<math><mrow intent='ordered-pair($a0,$a1)'><mi arg='a0'>x</mi><mi arg='a1'>y</mi></mrow></math>";
    all_russian_speech_modes(expr, "упорядоченная пара икс и игрек")
}

#[test]
fn w3c_core_outer_product() -> Result<()> {
    // W3C outer-product: localizes the concept and retains both arguments, in both styles and all verbosities.
    let expr = "<math><mrow intent='outer-product($a0,$a1)'><mi arg='a0'>x</mi><mi arg='a1'>y</mi></mrow></math>";
    all_russian_speech_modes(expr, "икс внешнее произведение, игрек")
}

#[test]
fn w3c_core_parallel_to() -> Result<()> {
    // W3C parallel-to: localizes the concept and retains both arguments, in both styles and all verbosities.
    let expr = "<math><mrow intent='parallel-to($a0,$a1)'><mi arg='a0'>x</mi><mi arg='a1'>y</mi></mrow></math>";
    all_russian_speech_modes(expr, "икс параллельно, игрек")
}

#[test]
fn w3c_core_partial_derivative() -> Result<()> {
    // W3C partial-derivative: localizes the concept and retains all 3 arguments, in both styles and all verbosities.
    let expr = "<math><mrow intent='partial-derivative($a0,$a1,$a2)'><mi arg='a0'>x</mi><mi arg='a1'>y</mi><mi arg='a2'>z</mi></mrow></math>";
    all_russian_speech_modes(expr, "частная производная икс по игрек порядка зэт")
}

#[test]
fn w3c_core_percent() -> Result<()> {
    // W3C percent: localizes the concept and retains its argument, in both styles and all verbosities.
    let expr = "<math><mrow intent='percent($a0)'><mi arg='a0'>x</mi></mrow></math>";
    all_russian_speech_modes(expr, "икс процентов")
}

#[test]
fn w3c_core_permutation_cycle() -> Result<()> {
    // W3C permutation-cycle: localizes the concept and retains both arguments, in both styles and all verbosities.
    let expr = "<math><mrow intent='permutation-cycle($a0,$a1)'><mi arg='a0'>x</mi><mi arg='a1'>y</mi></mrow></math>";
    all_russian_speech_modes(expr, "цикл перестановки икс запятая игрек")
}

#[test]
fn w3c_core_plus_or_minus() -> Result<()> {
    // W3C plus-or-minus: localizes the concept and retains both arguments, in both styles and all verbosities.
    let expr = "<math><mrow intent='plus-or-minus($a0,$a1)'><mi arg='a0'>x</mi><mi arg='a1'>y</mi></mrow></math>";
    all_russian_speech_modes(expr, "икс плюс минус, игрек")
}

#[test]
fn w3c_core_polar_coordinate() -> Result<()> {
    // W3C polar-coordinate: localizes the concept and retains both arguments, in both styles and all verbosities.
    let expr = "<math><mrow intent='polar-coordinate($a0,$a1)'><mi arg='a0'>x</mi><mi arg='a1'>y</mi></mrow></math>";
    all_russian_speech_modes(expr, "полярные координаты икс запятая игрек")
}

#[test]
fn w3c_core_precedes() -> Result<()> {
    // W3C precedes: localizes the concept and retains both arguments, in both styles and all verbosities.
    let expr = "<math><mrow intent='precedes($a0,$a1)'><mi arg='a0'>x</mi><mi arg='a1'>y</mi></mrow></math>";
    all_russian_speech_modes(expr, "икс предшествует, игрек")
}

#[test]
fn w3c_core_probability() -> Result<()> {
    // W3C probability: localizes the concept and retains its argument, in both styles and all verbosities.
    let expr = "<math><mrow intent='probability($a0)'><mi arg='a0'>x</mi></mrow></math>";
    all_russian_speech_modes(expr, "вероятность икс")
}

#[test]
fn w3c_core_product() -> Result<()> {
    // W3C product: localizes the concept and retains all 3 arguments, in both styles and all verbosities.
    let expr = "<math><mrow intent='product($a0,$a1,$a2)'><mi arg='a0'>x</mi><mi arg='a1'>y</mi><mi arg='a2'>z</mi></mrow></math>";
    all_russian_speech_modes(expr, "произведение от икс до игрек выражения зэт")
}

#[test]
fn w3c_core_proportional() -> Result<()> {
    // W3C proportional: localizes the concept and retains both arguments, in both styles and all verbosities.
    let expr = "<math><mrow intent='proportional($a0,$a1)'><mi arg='a0'>x</mi><mi arg='a1'>y</mi></mrow></math>";
    all_russian_speech_modes(expr, "икс пропорционально, игрек")
}

#[test]
fn w3c_core_quotient() -> Result<()> {
    // W3C quotient: localizes the concept and retains both arguments, in both styles and all verbosities.
    let expr = "<math><mrow intent='quotient($a0,$a1)'><mi arg='a0'>x</mi><mi arg='a1'>y</mi></mrow></math>";
    all_russian_speech_modes(expr, "целая часть результата деления икс на игрек")
}

#[test]
fn w3c_core_radius() -> Result<()> {
    // W3C radius: localizes the concept and retains its argument, in both styles and all verbosities.
    let expr = "<math><mrow intent='radius($a0)'><mi arg='a0'>x</mi></mrow></math>";
    all_russian_speech_modes(expr, "радиус икс")
}

#[test]
fn w3c_core_range_separator() -> Result<()> {
    // W3C range-separator: localizes the concept and retains both arguments, in both styles and all verbosities.
    let expr = "<math><mrow intent='range-separator($a0,$a1)'><mi arg='a0'>x</mi><mi arg='a1'>y</mi></mrow></math>";
    all_russian_speech_modes(expr, "икс до игрек")
}

#[test]
fn w3c_core_rate() -> Result<()> {
    // W3C rate: localizes the concept and retains both arguments, in both styles and all verbosities.
    let expr = "<math><mrow intent='rate($a0,$a1)'><mi arg='a0'>x</mi><mi arg='a1'>y</mi></mrow></math>";
    all_russian_speech_modes(expr, "икс на игрек")
}

#[test]
fn w3c_core_ratio() -> Result<()> {
    // W3C ratio: localizes the concept and retains both arguments, in both styles and all verbosities.
    let expr = "<math><mrow intent='ratio($a0,$a1)'><mi arg='a0'>x</mi><mi arg='a1'>y</mi></mrow></math>";
    all_russian_speech_modes(expr, "отношение икс к игрек")
}

#[test]
fn w3c_core_remainder() -> Result<()> {
    // W3C remainder: localizes the concept and retains both arguments, in both styles and all verbosities.
    let expr = "<math><mrow intent='remainder($a0,$a1)'><mi arg='a0'>x</mi><mi arg='a1'>y</mi></mrow></math>";
    all_russian_speech_modes(expr, "остаток от деления икс на игрек")
}

#[test]
fn w3c_core_right_angle() -> Result<()> {
    // W3C right-angle: localizes the concept and retains its argument, in both styles and all verbosities.
    let expr = "<math><mrow intent='right-angle($a0)'><mi arg='a0'>x</mi></mrow></math>";
    all_russian_speech_modes(expr, "прямой угол икс")
}

#[test]
fn w3c_core_round() -> Result<()> {
    // W3C round: localizes the concept and retains its argument, in both styles and all verbosities.
    let expr = "<math><mrow intent='round($a0)'><mi arg='a0'>x</mi></mrow></math>";
    all_russian_speech_modes(expr, "округлённое значение икс")
}

#[test]
fn w3c_core_secant() -> Result<()> {
    // W3C secant: localizes the concept and retains its argument, in both styles and all verbosities.
    let expr = "<math><mrow intent='secant($a0)'><mi arg='a0'>x</mi></mrow></math>";
    russian_speech_verbosities(expr, "секанс икс", "секанс от икс")
}

#[test]
fn w3c_core_set_difference() -> Result<()> {
    // W3C set-difference: localizes the concept and retains both arguments, in both styles and all verbosities.
    let expr = "<math><mrow intent='set-difference($a0,$a1)'><mi arg='a0'>x</mi><mi arg='a1'>y</mi></mrow></math>";
    all_russian_speech_modes(expr, "разность множеств икс и игрек")
}

#[test]
fn w3c_core_set_of_complex_numbers() -> Result<()> {
    // W3C set-of-complex-numbers: localizes the standalone concept name, in both styles and all verbosities.
    let expr = "<math><mi intent='set-of-complex-numbers'>x</mi></math>";
    all_russian_speech_modes(expr, "комплексные числа")
}

#[test]
fn w3c_core_set_of_integers() -> Result<()> {
    // W3C set-of-integers: localizes the standalone concept name, in both styles and all verbosities.
    let expr = "<math><mi intent='set-of-integers'>x</mi></math>";
    all_russian_speech_modes(expr, "целые числа")
}

#[test]
fn w3c_core_set_of_natural_numbers() -> Result<()> {
    // W3C set-of-natural-numbers: localizes the standalone concept name, in both styles and all verbosities.
    let expr = "<math><mi intent='set-of-natural-numbers'>x</mi></math>";
    all_russian_speech_modes(expr, "натуральные числа")
}

#[test]
fn w3c_core_set_of_primes() -> Result<()> {
    // W3C set-of-primes: localizes the standalone concept name, in both styles and all verbosities.
    let expr = "<math><mi intent='set-of-primes'>x</mi></math>";
    all_russian_speech_modes(expr, "простые числа")
}

#[test]
fn w3c_core_set_of_rationals() -> Result<()> {
    // W3C set-of-rationals: localizes the standalone concept name, in both styles and all verbosities.
    let expr = "<math><mi intent='set-of-rationals'>x</mi></math>";
    all_russian_speech_modes(expr, "рациональные числа")
}

#[test]
fn w3c_core_set_of_reals() -> Result<()> {
    // W3C set-of-reals: localizes the standalone concept name, in both styles and all verbosities.
    let expr = "<math><mi intent='set-of-reals'>x</mi></math>";
    all_russian_speech_modes(expr, "действительные числа")
}

#[test]
fn w3c_core_similar() -> Result<()> {
    // W3C similar: localizes the concept and retains both arguments, in both styles and all verbosities.
    let expr = "<math><mrow intent='similar($a0,$a1)'><mi arg='a0'>x</mi><mi arg='a1'>y</mi></mrow></math>";
    all_russian_speech_modes(expr, "икс подобно игрек")
}

#[test]
fn w3c_core_sine() -> Result<()> {
    // W3C sine: localizes the concept and retains its argument, in both styles and all verbosities.
    let expr = "<math><mrow intent='sine($a0)'><mi arg='a0'>x</mi></mrow></math>";
    russian_speech_verbosities(expr, "синус икс", "синус от икс")
}

#[test]
fn w3c_core_span() -> Result<()> {
    // W3C span: localizes the concept and retains its argument, in both styles and all verbosities.
    let expr = "<math><mrow intent='span($a0)'><mi arg='a0'>x</mi></mrow></math>";
    all_russian_speech_modes(expr, "линейная оболочка икс")
}

#[test]
fn w3c_core_spherical_coordinate() -> Result<()> {
    // W3C spherical-coordinate: localizes the concept and retains all 3 arguments, in both styles and all verbosities.
    let expr = "<math><mrow intent='spherical-coordinate($a0,$a1,$a2)'><mi arg='a0'>x</mi><mi arg='a1'>y</mi><mi arg='a2'>z</mi></mrow></math>";
    all_russian_speech_modes(expr, "сферические координаты икс запятая игрек запятая зэт")
}

#[test]
fn w3c_core_square_root_of() -> Result<()> {
    // W3C square-root-of: localizes the concept and retains its argument, in both styles and all verbosities.
    let expr = "<math><mrow intent='square-root-of($a0)'><mi arg='a0'>x</mi></mrow></math>";
    all_russian_speech_modes(expr, "квадратный корень из икс")
}

#[test]
fn w3c_core_standard_deviation() -> Result<()> {
    // W3C standard-deviation: localizes the concept and retains its argument, in both styles and all verbosities.
    let expr = "<math><mrow intent='standard-deviation($a0)'><mi arg='a0'>x</mi></mrow></math>";
    all_russian_speech_modes(expr, "стандартное отклонение икс")
}

#[test]
fn w3c_core_subset() -> Result<()> {
    // W3C subset: localizes the concept and retains both arguments, in both styles and all verbosities.
    let expr = "<math><mrow intent='subset($a0,$a1)'><mi arg='a0'>x</mi><mi arg='a1'>y</mi></mrow></math>";
    all_russian_speech_modes(expr, "икс является подмножеством; игрек")
}

#[test]
fn w3c_core_subset_or_equal() -> Result<()> {
    // W3C subset-or-equal: localizes the concept and retains both arguments, in both styles and all verbosities.
    let expr = "<math><mrow intent='subset-or-equal($a0,$a1)'><mi arg='a0'>x</mi><mi arg='a1'>y</mi></mrow></math>";
    all_russian_speech_modes(expr, "икс является подмножеством или равно; игрек")
}

#[test]
fn w3c_core_succeeds() -> Result<()> {
    // W3C succeeds: localizes the concept and retains both arguments, in both styles and all verbosities.
    let expr = "<math><mrow intent='succeeds($a0,$a1)'><mi arg='a0'>x</mi><mi arg='a1'>y</mi></mrow></math>";
    all_russian_speech_modes(expr, "икс следует за, игрек")
}

#[test]
fn w3c_core_such_that() -> Result<()> {
    // W3C such-that: localizes the concept and retains both arguments, in both styles and all verbosities.
    let expr = "<math><mrow intent='such-that($a0,$a1)'><mi arg='a0'>x</mi><mi arg='a1'>y</mi></mrow></math>";
    all_russian_speech_modes(expr, "икс таких что игрек")
}

#[test]
fn w3c_core_sum() -> Result<()> {
    // W3C sum: localizes the concept and retains all 3 arguments, in both styles and all verbosities.
    let expr = "<math><mrow intent='sum($a0,$a1,$a2)'><mi arg='a0'>x</mi><mi arg='a1'>y</mi><mi arg='a2'>z</mi></mrow></math>";
    all_russian_speech_modes(expr, "сумма от икс до игрек выражения зэт")
}

#[test]
fn w3c_core_superset() -> Result<()> {
    // W3C superset: localizes the concept and retains both arguments, in both styles and all verbosities.
    let expr = "<math><mrow intent='superset($a0,$a1)'><mi arg='a0'>x</mi><mi arg='a1'>y</mi></mrow></math>";
    all_russian_speech_modes(expr, "икс является надмножеством; игрек")
}

#[test]
fn w3c_core_superset_or_equal() -> Result<()> {
    // W3C superset-or-equal: localizes the concept and retains both arguments, in both styles and all verbosities.
    let expr = "<math><mrow intent='superset-or-equal($a0,$a1)'><mi arg='a0'>x</mi><mi arg='a1'>y</mi></mrow></math>";
    all_russian_speech_modes(expr, "икс является надмножеством или равно; игрек")
}

#[test]
fn w3c_core_tangent() -> Result<()> {
    // W3C tangent: localizes the concept and retains its argument, in both styles and all verbosities.
    let expr = "<math><mrow intent='tangent($a0)'><mi arg='a0'>x</mi></mrow></math>";
    russian_speech_verbosities(expr, "тангенс икс", "тангенс от икс")
}

#[test]
fn w3c_core_tends_to() -> Result<()> {
    // W3C tends-to: localizes the concept and retains both arguments, in both styles and all verbosities.
    let expr = "<math><mrow intent='tends-to($a0,$a1)'><mi arg='a0'>x</mi><mi arg='a1'>y</mi></mrow></math>";
    all_russian_speech_modes(expr, "икс стремится к, игрек")
}

#[test]
fn w3c_core_tends_to_from_above() -> Result<()> {
    // W3C tends-to-from-above: localizes the concept and retains both arguments, in both styles and all verbosities.
    let expr = "<math><mrow intent='tends-to-from-above($a0,$a1)'><mi arg='a0'>x</mi><mi arg='a1'>y</mi></mrow></math>";
    all_russian_speech_modes(expr, "икс стремится сверху к, игрек")
}

#[test]
fn w3c_core_tends_to_from_below() -> Result<()> {
    // W3C tends-to-from-below: localizes the concept and retains both arguments, in both styles and all verbosities.
    let expr = "<math><mrow intent='tends-to-from-below($a0,$a1)'><mi arg='a0'>x</mi><mi arg='a1'>y</mi></mrow></math>";
    all_russian_speech_modes(expr, "икс стремится снизу к, игрек")
}

#[test]
fn w3c_core_there_does_not_exist() -> Result<()> {
    // W3C there-does-not-exist: localizes the concept and retains its argument, in both styles and all verbosities.
    let expr = "<math><mrow intent='there-does-not-exist($a0)'><mi arg='a0'>x</mi></mrow></math>";
    all_russian_speech_modes(expr, "не существует икс")
}

#[test]
fn w3c_core_there_exists() -> Result<()> {
    // W3C there-exists: localizes the concept and retains its argument, in both styles and all verbosities.
    let expr = "<math><mrow intent='there-exists($a0)'><mi arg='a0'>x</mi></mrow></math>";
    all_russian_speech_modes(expr, "существует икс")
}

#[test]
fn w3c_core_tilde() -> Result<()> {
    // W3C tilde: localizes the concept and retains both arguments, in both styles and all verbosities.
    let expr = "<math><mrow intent='tilde($a0,$a1)'><mi arg='a0'>x</mi><mi arg='a1'>y</mi></mrow></math>";
    all_russian_speech_modes(expr, "икс тильда игрек")
}

#[test]
fn w3c_core_time_separator() -> Result<()> {
    // W3C time-separator: localizes the concept and retains both arguments, in both styles and all verbosities.
    let expr = "<math><mrow intent='time-separator($a0,$a1)'><mi arg='a0'>x</mi><mi arg='a1'>y</mi></mrow></math>";
    all_russian_speech_modes(expr, "икс игрек")
}

#[test]
fn w3c_core_times() -> Result<()> {
    // W3C times: localizes the concept and retains both arguments, in both styles and all verbosities.
    let expr = "<math><mrow intent='times($a0,$a1)'><mi arg='a0'>x</mi><mi arg='a1'>y</mi></mrow></math>";
    all_russian_speech_modes(expr, "икс умножить на, игрек")
}

#[test]
fn w3c_core_translation() -> Result<()> {
    // W3C translation: localizes the concept and retains both arguments, in both styles and all verbosities.
    let expr = "<math><mrow intent='translation($a0,$a1)'><mi arg='a0'>x</mi><mi arg='a1'>y</mi></mrow></math>";
    all_russian_speech_modes(expr, "перенос на икс запятая игрек")
}

#[test]
fn w3c_core_tuple() -> Result<()> {
    // W3C tuple: localizes the standalone concept name, in both styles and all verbosities.
    let expr = "<math><mi intent='tuple'>x</mi></math>";
    all_russian_speech_modes(expr, "кортеж")
}

#[test]
fn w3c_core_union() -> Result<()> {
    // W3C union: localizes the concept and retains both arguments, in both styles and all verbosities.
    let expr = "<math><mrow intent='union($a0,$a1)'><mi arg='a0'>x</mi><mi arg='a1'>y</mi></mrow></math>";
    all_russian_speech_modes(expr, "икс объединение, игрек")
}

#[test]
fn w3c_core_unit_vector() -> Result<()> {
    // W3C unit-vector: localizes the concept and retains its argument, in both styles and all verbosities.
    let expr = "<math><mrow intent='unit-vector($a0)'><mi arg='a0'>x</mi></mrow></math>";
    all_russian_speech_modes(expr, "единичный вектор икс")
}

#[test]
fn w3c_core_upwards_diagonal_ellipsis() -> Result<()> {
    // W3C upwards-diagonal-ellipsis: localizes the concept and retains both arguments, in both styles and all verbosities.
    let expr = "<math><mrow intent='upwards-diagonal-ellipsis($a0,$a1)'><mi arg='a0'>x</mi><mi arg='a1'>y</mi></mrow></math>";
    all_russian_speech_modes(expr, "икс диагональное многоточие вверх; игрек")
}

#[test]
fn w3c_core_variance() -> Result<()> {
    // W3C variance: localizes the concept and retains its argument, in both styles and all verbosities.
    let expr = "<math><mrow intent='variance($a0)'><mi arg='a0'>x</mi></mrow></math>";
    all_russian_speech_modes(expr, "дисперсия икс")
}

#[test]
fn w3c_core_vertical_ellipsis() -> Result<()> {
    // W3C vertical-ellipsis: localizes the concept and retains both arguments, in both styles and all verbosities.
    let expr = "<math><mrow intent='vertical-ellipsis($a0,$a1)'><mi arg='a0'>x</mi><mi arg='a1'>y</mi></mrow></math>";
    all_russian_speech_modes(expr, "икс вертикальное многоточие; игрек")
}

#[test]
fn w3c_core_volume() -> Result<()> {
    // W3C volume: localizes the concept and retains its argument, in both styles and all verbosities.
    let expr = "<math><mrow intent='volume($a0)'><mi arg='a0'>x</mi></mrow></math>";
    all_russian_speech_modes(expr, "объём икс")
}

#[test]
fn w3c_core_xor() -> Result<()> {
    // W3C xor: localizes the concept and retains both arguments, in both styles and all verbosities.
    let expr = "<math><mrow intent='xor($a0,$a1)'><mi arg='a0'>x</mi><mi arg='a1'>y</mi></mrow></math>";
    all_russian_speech_modes(expr, "икс исключающее или, игрек")
}

#[test]
fn w3c_logarithm_argument_and_base() -> Result<()> {
    // W3C orders arguments as value, base; Russian speech says 'по основанию base от value'.
    all_russian_speech_modes("<math><mrow intent='logarithm($value,$base)'><msub><mi>log</mi><mn arg='base'>2</mn></msub><mn arg='value'>8</mn></mrow></math>","логарифм по основанию 2 от 8")?;
    for style in ["ClearSpeak", "SimpleSpeak"] {
        for verbosity in ["Terse", "Medium", "Verbose"] {
            let expected=if verbosity=="Terse" {"логарифм 8"} else {"логарифм от 8"};
            test_prefs("ru",style,vec![("Verbosity",verbosity)],"<math><mrow intent='logarithm(8)'/></math>",expected)?;
        }
    }
    Ok(())
}

#[test]
fn w3c_sum_arities() -> Result<()> {
    // Domain/bound arguments precede the summand in the two- and three-argument W3C forms.
    all_russian_speech_modes("<math><mrow intent='sum(2)'/></math>","сумма 2")?;
    all_russian_speech_modes("<math><mrow intent='sum(2,3)'/></math>","сумма по 2 выражения 3")?;
    all_russian_speech_modes("<math><mrow intent='sum(2,3,5)'/></math>","сумма от 2 до 3 выражения 5")
}

#[test]
fn w3c_product_arities() -> Result<()> {
    // The last argument is the factor expression, never an upper bound or an extra factor.
    all_russian_speech_modes("<math><mrow intent='product(2)'/></math>","произведение 2")?;
    all_russian_speech_modes("<math><mrow intent='product(2,3)'/></math>","произведение по 2 выражения 3")?;
    all_russian_speech_modes("<math><mrow intent='product(2,3,5)'/></math>","произведение от 2 до 3 выражения 5")
}

#[test]
fn w3c_partial_derivative_mixed_orders() -> Result<()> {
    // Every variable stays paired with its own order, including an implicit first-order variable.
    all_russian_speech_modes("<math><mrow intent='partial-derivative($f,$x,2,$y,1)'><mi arg='f'>f</mi><mi arg='x'>x</mi><mi arg='y'>y</mi></mrow></math>","частная производная эф по икс порядка 2 и по игрек")?;
    all_russian_speech_modes("<math><mrow intent='partial-derivative($f,x,$n,y,3)'><mi arg='f'>f</mi><mi arg='n'>n</mi></mrow></math>","частная производная эф по икс порядка эн и по игрек порядка 3")
}

#[test]
fn w3c_variadic_lists_and_coordinates() -> Result<()> {
    // Every entry/coordinate of a longer list must be spoken once; arity-zero names are in the inventory test.
    all_russian_speech_modes("<math><mrow intent='list(1,2,3)'/></math>","список 1 запятая 2 запятая 3")?;
    all_russian_speech_modes("<math><mrow intent='tuple(1,2,3)'/></math>","кортеж 1 запятая 2 запятая 3")?;
    all_russian_speech_modes("<math><mrow intent='cartesian-coordinate(1,2,3,4)'/></math>","декартовы координаты 1 запятая 2 запятая 3 запятая 4")
}

#[test]
fn w3c_variadic_extrema() -> Result<()> {
    // max/min accept more than one value and must not drop the final value.
    all_russian_speech_modes("<math><mrow intent='max(1,2,3)'/></math>","максимум из 1 запятая 2 запятая 3")?;
    all_russian_speech_modes("<math><mrow intent='min(1,2,3)'/></math>","минимум из 1 запятая 2 запятая 3")
}

#[test]
fn w3c_nested_noun_cases() -> Result<()> {
    // Nested noun heads take genitive forms, in either order of old and newly supported concepts.
    all_russian_speech_modes("<math><mrow intent='cardinality(image($r))'><mi>Im</mi><mi arg='r'>R</mi></mrow></math>","мощность образа заглавной эр")?;
    all_russian_speech_modes("<math><mrow intent='image(cardinality($r))'><mi>Im</mi><mi arg='r'>R</mi></mrow></math>","образ мощности заглавной эр")?;
    all_russian_speech_modes("<math><mrow intent='mean(list(1,2,3))'/></math>","среднее списка 1 запятая 2 запятая 3")
}

#[test]
fn w3c_division_and_condition_argument_order() -> Result<()> {
    // Non-symmetric numeric operands expose accidental reversal of dividend/divisor or event/condition.
    all_russian_speech_modes("<math><mrow intent='quotient(7,3)'/></math>","целая часть результата деления 7 на 3")?;
    all_russian_speech_modes("<math><mrow intent='remainder(7,3)'/></math>","остаток от деления 7 на 3")?;
    all_russian_speech_modes("<math><mrow intent='ratio(7,3)'/></math>","отношение 7 к 3")?;
    all_russian_speech_modes("<math><mrow intent='conditional-probability(7,3)'/></math>","вероятность 7 при условии 3")
}

#[test]
fn w3c_relation_and_operation_cases() -> Result<()> {
    // Check grammatical words independently of automatically generated pauses.
    for style in ["ClearSpeak","SimpleSpeak"] {
        for (name,operator,expected) in [
            ("equals","=","заглавная ку равно заглавной эр"),
            ("element-of","∈","заглавная ку принадлежит заглавной эр"),
            ("times","×","заглавная ку умножить на заглавную эр"),
            ("implies","⇒","заглавная ку влечёт заглавную эр"),
        ] {
            let mathml=format!("<math><mrow intent='{name}($q,$r)'><mi arg='q'>Q</mi><mo>{operator}</mo><mi arg='r'>R</mi></mrow></math>");
            init_nav("ru",&mathml)?;
            set_preference("SpeechStyle",style)?;
            set_preference("SpeechOverrides_CapitalLetters","")?;
            set_preference("Impairment","Blindness")?;
            let actual=get_spoken_text()?.replace([',',';'],"").split_whitespace().collect::<Vec<_>>().join(" ");
            assert_eq!(actual,expected,"{style}/{name}");
        }
    }
    Ok(())
}

#[test]
fn w3c_read_named_logarithm_and_relation() -> Result<()> {
    // Semantic ReadCurrent must retain both argument order and Russian relation wording.
    for style in ["ClearSpeak","SimpleSpeak"] {
        init_nav("ru","<math><mrow intent='logarithm(8,2)'/></math>")?;
        set_preference("SpeechStyle",style)?;
        let speech=do_navigate_command("ReadCurrent")?.split_whitespace().collect::<Vec<_>>().join(" ");
        assert!(speech.contains("логарифм по основанию 2 от 8"),"{style}: {speech}");
        init_nav("ru","<math><mrow intent='equals(2,3)'/></math>")?;
        set_preference("SpeechStyle",style)?;
        let speech=do_navigate_command("ReadCurrent")?.split_whitespace().collect::<Vec<_>>().join(" ");
        assert!(speech.contains("2 равно 3"),"{style}: {speech}");
    }
    Ok(())
}
