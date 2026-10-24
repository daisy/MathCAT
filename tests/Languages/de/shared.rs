/// Tests for rules shared between various speech styles:
/// *  modified var
use crate::common::*;
use anyhow::Result;

#[test]
fn transpose_uses_grammatical_german_forms() -> Result<()> {
    // A superscript T uses the participle, while a function intent names the transpose.
    let postfix = "<math><msup><mi>A</mi><mi>T</mi></msup></math>";
    test("de", "ClearSpeak", postfix, "groß a transponiert")?;
    test("de", "SimpleSpeak", postfix, "groß a transponiert")?;

    let function = "<math><mrow intent='transpose:function($matrix)'><mi arg='matrix'>A</mi></mrow></math>";
    test("de", "ClearSpeak", function, "die Transponierte von groß a")?;
    Ok(())
}

#[test]
fn ordinary_integer_powers() -> Result<()> {
    // Ordinary integer exponents use the idiomatic German "hoch" construction.
    let fourth_power = "<math><msup><mi>x</mi><mn>4</mn></msup></math>";
    test("de", "ClearSpeak", fourth_power, "x hoch 4")?;
    test("de", "SimpleSpeak", fourth_power, "x hoch 4")?;

    let zeroth_power = "<math><msup><mi>x</mi><mn>0</mn></msup></math>";
    test("de", "ClearSpeak", zeroth_power, "x hoch 0")?;
    test("de", "SimpleSpeak", zeroth_power, "x hoch 0")?;
    Ok(())
}

#[test]
fn variable_powers() -> Result<()> {
    // Variable exponents should not acquire an English ordinal suffix or a second "hoch".
    let expr = "<math><msup><mn>2</mn><mi>x</mi></msup></math>";
    test("de", "ClearSpeak", expr, "2 hoch x")?;
    test("de", "SimpleSpeak", expr, "2 hoch x")?;
    Ok(())
}

#[test]
fn unary_signs_use_plus_and_minus() -> Result<()> {
    // Unary signs use their mathematical names rather than adjectival forms.
    let signed_numbers = "<math><mo>−</mo><mn>5</mn><mo>,</mo><mo>+</mo><mn>5</mn></math>";
    test("de", "ClearSpeak", signed_numbers, "minus 5 komma plus 5")?;
    test("de", "SimpleSpeak", signed_numbers, "minus 5 komma plus 5")?;

    let negative_root = "<math><mo>−</mo><msqrt><mi>x</mi></msqrt></math>";
    test("de", "ClearSpeak", negative_root, "minus die quadratwurzel von x")?;
    test("de", "SimpleSpeak", negative_root, "minus die quadratwurzel von x")?;
    Ok(())
}

#[test]
fn modified_vars() -> Result<()> {
    let expr = "<math> <mrow>
        <mover> <mi>a</mi> <mo>`</mo> </mover>
        <mover> <mi>b</mi> <mo>~</mo> </mover>
        <mover> <mi>c</mi> <mo>&#x0306;</mo> </mover>
        <mover> <mi>b</mi> <mo>&#x030c;</mo> </mover>
        <mover> <mi>c</mi> <mo>`</mo> </mover>  <mo>+</mo>
        <mover> <mi>r</mi> <mo>ˇ</mo> </mover>  <mo>+</mo>
        <mover> <mi>x</mi> <mo>.</mo> </mover>
        <mover> <mi>y</mi> <mo>&#x2D9;</mo> </mover>
        <mover> <mi>z</mi> <mo>&#x00A8;</mo> </mover>
        <mover> <mi>u</mi> <mo>&#x20DB;</mo> </mover>
        <mover> <mi>v</mi> <mo>&#x20DC;</mo> </mover> <mo>+</mo>
        <mover> <mi>x</mi> <mo>^</mo> </mover> <mo>+</mo>
        <mover> <mi>t</mi> <mo>→</mo> </mover>
        </mrow> </math>";
    test("de", "SimpleSpeak", expr,
        "a gravis akzent; b tilde, c brevis, b kombinierendes Caron; c gravis akzent; plus r caron plus; x punkt, y Punkt oberhalb; z diaeresis, u dreifacher punkt; v vierfacher punkt; plus x zirkumflex, plus vektor t")?;
        Ok(())

}

#[test]
fn limit() -> Result<()> {
    let expr = "<math>
            <munder>
            <mo>lim</mo>
            <mrow>  <mi>x</mi> <mo>&#x2192;</mo>  <mn>0</mn>  </mrow>
            </munder>
            <mrow>
            <mfrac>
                <mrow>  <mi>sin</mi>  <mo>&#x2061;</mo> <mi>x</mi> </mrow>
                <mi>x</mi>
            </mfrac>
            </mrow>
        </math>";
    test("de", "SimpleSpeak", expr, "der grenzwert bei x gegen 0; von, bruch, sinus von x, über x, ende des bruchs")?;
    test_prefs("de", "SimpleSpeak", vec![("Impairment", "LearningDisability")], expr,
            "der grenzwert bei x gegen 0; von; sinus von x, über x")?;
            return Ok(());

}

#[test]
fn limit_from_below() -> Result<()> {
    let expr = "<math>
            <munder>
            <mo>lim</mo>
            <mrow>  <mi>x</mi> <mo>↗</mo>  <mn>0</mn>  </mrow>
            </munder>
            <mrow>
                <mrow>  <mi>sin</mi>  <mo>&#x2061;</mo> <mi>x</mi> </mrow>
            </mrow>
        </math>";
    test("de", "SimpleSpeak", expr, "der grenzwert bei x annähernd von unten 0; von sinus von x")?;
    return Ok(());

}


#[test]
fn binomial_mmultiscripts() -> Result<()> {
    let expr = "<math><mmultiscripts><mi>C</mi><mi>m</mi><none/><mprescripts/><mi>n</mi><none/></mmultiscripts></math>";
    test("de", "SimpleSpeak", expr, "n wählen m")?;
    return Ok(());

}

#[test]
fn binomial_mmultiscripts_other() -> Result<()> {
    let expr = "<math><mmultiscripts><mi>C</mi><mi>m</mi><none/><mprescripts/><none/><mi>n</mi></mmultiscripts></math>";
    test("de", "SimpleSpeak", expr, "n wählen m")?;
    return Ok(());

}

#[test]
fn binomial_subscript() -> Result<()> {  // C_{n,k}
    let expr = "<math><msub><mi>C</mi><mrow><mi>n</mi><mo>,</mo><mi>m</mi></mrow></msub></math>";
    test("de", "SimpleSpeak", expr, "n wählen m")?;
    return Ok(());

}

#[test]
fn permutation_mmultiscripts() -> Result<()> {
    let expr = "<math><mmultiscripts><mi>P</mi><mi>k</mi><none/><mprescripts/><mi>n</mi><none/></mmultiscripts></math>";
    test("de", "SimpleSpeak", expr, "k permutationen von n")?;
    return Ok(());

}

#[test]
fn permutation_mmultiscripts_sup() -> Result<()> {
    let expr = "<math><mmultiscripts><mi>P</mi><mi>k</mi><none/><mprescripts/><none/><mi>n</mi></mmultiscripts></math>";
    test("de", "SimpleSpeak", expr, "k permutationen von n")?;
    return Ok(());

}

#[test]
fn permutation_msubsup() -> Result<()> {
    let expr = "<math><msubsup><mi>P</mi><mi>k</mi><mi>n</mi></msubsup></math>";
    test("de", "SimpleSpeak", expr, "k permutationen von n")?;
    return Ok(());

}

#[test]
fn tensor_mmultiscripts() -> Result<()> {
    let expr = "<math><mmultiscripts>
            <mi>R</mi> <mi>i</mi><none/> <none/><mi>j</mi> <mi>k</mi><none/> <mi>l</mi><none/> 
        </mmultiscripts></math>";
    test_prefs("de", "SimpleSpeak", vec![("Verbosity", "Verbose")], expr,
            "groß r mit 4 Postskripten, Index i Hochstellung j Index k Index l")?;
    test_prefs("de", "SimpleSpeak", vec![("Verbosity", "Medium")], expr,
            "groß r mit 4 Postskripten, sub i super j sub k sub l")?;
            return Ok(());

}

#[test]
// idk really about that one, as we have "None" in the expected speech.
fn huge_num_mmultiscripts() -> Result<()> {
    let expr = "<math><mmultiscripts>
            <mi>R</mi> <mi>i</mi><none/> <none/><mi>j</mi> <mi>k</mi><none/> <mi>l</mi><none/> <mi>m</mi><none/>
            <mprescripts/> <mi>I</mi><none/> <none/><mi>J</mi> <mi>K</mi><none/> <mi>L</mi><none/>
        </mmultiscripts></math>";
    test_prefs("de", "SimpleSpeak", vec![("Verbosity", "Verbose")], expr,
            "groß r mit 4 Präskripten, vorangestellter Index groß i, vorangestellte Hochstellung groß j und abwechselnde präskripte groß k none groß l none ende der präskripte und mit 5 Postskripten, Index i Hochstellung j Index k Index l und wechselnde skripte m none ende der skripte")?;
            return Ok(());

}

#[test]
fn prime() -> Result<()> {
    let expr = "<math> <msup><mi>x</mi><mo >&#x2032;</mo></msup> </math>";
    test("de", "SimpleSpeak", expr, "x strich")?;
    return Ok(());

}
