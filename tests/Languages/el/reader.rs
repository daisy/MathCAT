//! Native Greek Math Reader speech and structural regression tests.
use crate::common::*;
use anyhow::Result;

#[test]
fn squared_expression() -> Result<()> {
    // The add-on self-test expression must be Greek in the native speech style.
    test("el", "GreekMathReader", "<math><msup><mi>x</mi><mn>2</mn></msup><mo>+</mo><mn>1</mn></math>", "χι στο τετράγωνο, συν 1")
}

#[test]
fn simple_fraction() -> Result<()> {
    // Both operands remain audible in the concise fraction form.
    test("el", "GreekMathReader", "<math><mfrac><mn>2</mn><mn>3</mn></mfrac></math>", "2 διά 3")
}

#[test]
fn comma_decimal() -> Result<()> {
    // Use the Greek decimal separator without spelling the comma as punctuation.
    let expr = r#"<math><mn>3,14</mn></math>"#;
    test_prefs("el", "GreekMathReader", vec![("CapitalLetters_UseWord", "false")], expr,
        "3 κόμμα 14")
}

#[test]
fn grouped_decimal() -> Result<()> {
    // Remove grouping marks while preserving fractional trailing zeroes.
    let expr = r#"<math><mn>1.234,50</mn></math>"#;
    test_prefs("el", "GreekMathReader", vec![("CapitalLetters_UseWord", "false")], expr,
        "1234 κόμμα 50")
}

#[test]
fn cube_root() -> Result<()> {
    // Read the root index after MathCAT intent conversion in the correct order.
    let expr = r#"<math><mroot><mi>x</mi><mn>3</mn></mroot></math>"#;
    test_prefs("el", "GreekMathReader", vec![("CapitalLetters_UseWord", "false")], expr,
        "κυβική ρίζα του χι")
}

#[test]
fn fourth_root() -> Result<()> {
    // Preserve a non-cubic root index.
    let expr = r#"<math><mroot><mi>x</mi><mn>4</mn></mroot></math>"#;
    test_prefs("el", "GreekMathReader", vec![("CapitalLetters_UseWord", "false")], expr,
        "ρίζα τάξης 4 του χι")
}

#[test]
fn nested_fraction() -> Result<()> {
    // Both nested fraction boundaries must be audible.
    let expr = r#"<math><mfrac><mn>1</mn><mfrac><mn>2</mn><mn>3</mn></mfrac></mfrac></math>"#;
    test_prefs("el", "GreekMathReader", vec![("CapitalLetters_UseWord", "false")], expr,
        "κλάσμα με αριθμητή 1, και παρονομαστή κλάσμα με αριθμητή 2, και παρονομαστή 3, τέλος κλάσματος, τέλος κλάσματος")
}

#[test]
fn variable_power() -> Result<()> {
    // Terminate a compound exponent before returning to the surrounding expression.
    let expr = r#"<math><msup><mi>x</mi><mrow><mi>n</mi><mo>+</mo><mn>1</mn></mrow></msup></math>"#;
    test_prefs("el", "GreekMathReader", vec![("CapitalLetters_UseWord", "false")], expr,
        "χι στη δύναμη νι συν 1, τέλος εκθέτη")
}

#[test]
fn index() -> Result<()> {
    // Distinguish a subscript from an exponent.
    let expr = r#"<math><msub><mi>a</mi><mi>n</mi></msub></math>"#;
    test_prefs("el", "GreekMathReader", vec![("CapitalLetters_UseWord", "false")], expr,
        "έι δείκτης νι")
}

#[test]
fn partial_derivative() -> Result<()> {
    // Recognize a partial differential when its operator does not insert invisible multiplication.
    let expr = r#"<math><mfrac><mrow><mo>∂</mo><mi>f</mi></mrow><mrow><mo>∂</mo><mi>x</mi></mrow></mfrac></math>"#;
    test_prefs("el", "GreekMathReader", vec![("CapitalLetters_UseWord", "false")], expr,
        "μερική παράγωγος του εφ ως προς χι")
}

#[test]
fn literal_derivative() -> Result<()> {
    // An explicit literal intent prevents differential interpretation.
    let expr = r#"<math><mfrac intent=":literal"><mrow><mi>d</mi><mi>f</mi></mrow><mrow><mi>d</mi><mi>x</mi></mrow></mfrac></math>"#;
    test_prefs("el", "GreekMathReader", vec![("CapitalLetters_UseWord", "false")], expr,
        "αρχή, ντί εφ, προς, ντί χι, τέλος κλάσματος")
}

#[test]
fn literal_expectation() -> Result<()> {
    // An author can opt out of the conventional statistical reading.
    let expr = r#"<math><mrow intent=":literal"><mi>E</mi><mo>[</mo><mi>X</mi><mo>|</mo><mi>Y</mi><mo>]</mo></mrow></math>"#;
    test_prefs("el", "GreekMathReader", vec![("CapitalLetters_UseWord", "false")], expr,
        "ί; αριστερή αγκύλη; χι κάθετη γραμμη, γουάι; δεξιά αγκύλη")
}

#[test]
fn literal_bra_ket() -> Result<()> {
    // Literal vertical bars must not be converted into an inner product.
    let expr = r#"<math><mrow intent=":literal"><mo>⟨</mo><mi>ψ</mi><mo>|</mo><mi>φ</mi><mo>⟩</mo></mrow></math>"#;
    test_prefs("el", "GreekMathReader", vec![("CapitalLetters_UseWord", "false")], expr,
        "αριστερή γωνιακή αγκύλη; ψι κάθετη γραμμη, φι; δεξιά γωνιακή αγκύλη")
}

#[test]
fn literal_product() -> Result<()> {
    // Literal dot notation retains the dot name.
    let expr = r#"<math><mrow intent=":literal"><mi>a</mi><mo>·</mo><mi>b</mi></mrow></math>"#;
    test_prefs("el", "GreekMathReader", vec![("CapitalLetters_UseWord", "false")], expr,
        "έι τελεία μπί")
}

#[test]
fn standalone_differential() -> Result<()> {
    // Do not identify ordinary d times x as a differential outside calculus notation.
    let expr = r#"<math><mi>d</mi><mi>x</mi></math>"#;
    test_prefs("el", "GreekMathReader", vec![("CapitalLetters_UseWord", "false")], expr,
        "ντί χι")
}

#[test]
fn extended_expectation() -> Result<()> {
    // Non-binary conditional notation retains all operands through structural speech.
    let expr = r#"<math><mi>E</mi><mo>[</mo><mi>X</mi><mo>|</mo><mi>Y</mi><mo>|</mo><mi>Z</mi><mo>]</mo></math>"#;
    test_prefs("el", "GreekMathReader", vec![("CapitalLetters_UseWord", "false")], expr,
        "αναμενόμενη τιμή του χι, η απόλυτη τιμή του γουάι; ζήτα")
}

#[test]
fn extended_intent() -> Result<()> {
    // Unsupported arity must preserve all four arguments rather than truncating them.
    let expr = r#"<math><mrow intent="matrix-element($a,$b,$c,$d)"><mn arg="a">2</mn><mn arg="b">3</mn><mn arg="c">4</mn><mn arg="d">5</mn></mrow></math>"#;
    test_prefs("el", "GreekMathReader", vec![("CapitalLetters_UseWord", "false")], expr,
        "στοιχείο πίνακα του 2 κόμμα, 3 κόμμα, 4 κόμμα, 5")
}

#[test]
fn empty_intent() -> Result<()> {
    // A zero-argument named concept still has a Greek name.
    let expr = r#"<math><mrow intent="expectation"><mi>E</mi></mrow></math>"#;
    test_prefs("el", "GreekMathReader", vec![("CapitalLetters_UseWord", "false")], expr,
        "αναμενόμενη τιμή")
}

#[test]
fn silent_tokens() -> Result<()> {
    // Custom token rules must honor author-supplied silence, including numbers and equality.
    let expr = r#"<math><mi intent=":silent">x</mi><mo intent=":silent">=</mo><mn intent=":silent">3,14</mn><mn>2</mn></math>"#;
    test_prefs("el", "GreekMathReader", vec![("CapitalLetters_UseWord", "false")], expr,
        "2")
}

#[test]
fn currency_one() -> Result<()> {
    // Use the Greek singular currency name.
    let expr = r#"<math><mo>$</mo><mn>1</mn></math>"#;
    test_prefs("el", "GreekMathReader", vec![("CapitalLetters_UseWord", "false")], expr,
        "1 δολάριο")
}

#[test]
fn currency_two() -> Result<()> {
    // Use a Greek plural, not an English s suffix.
    let expr = r#"<math><mo>$</mo><mn>2</mn></math>"#;
    test_prefs("el", "GreekMathReader", vec![("CapitalLetters_UseWord", "false")], expr,
        "2 δολάρια")
}

#[test]
fn currency_plus() -> Result<()> {
    // Do not consume a plus sign as if it were invisible multiplication.
    let expr = r#"<math><mo>$</mo><mo>+</mo><mn>2</mn></math>"#;
    test_prefs("el", "GreekMathReader", vec![("CapitalLetters_UseWord", "false")], expr,
        "δολάριο συν 2")
}

#[test]
fn units() -> Result<()> {
    // Reuse native MathCAT unit intent and plural handling.
    let expr = r#"<math><mn>3</mn><mi intent=":unit">kg</mi></math>"#;
    test_prefs("el", "GreekMathReader", vec![("CapitalLetters_UseWord", "false")], expr,
        "3 κιλά")
}

#[test]
fn matrix() -> Result<()> {
    // Preserve dimensions, row positions, and all four matrix entries.
    let expr = r#"<math><mo>(</mo><mtable><mtr><mtd><mn>1</mn></mtd><mtd><mn>2</mn></mtd></mtr><mtr><mtd><mn>3</mn></mtd><mtd><mn>4</mn></mtd></mtr></mtable><mo>)</mo></math>"#;
    test_prefs("el", "GreekMathReader", vec![("CapitalLetters_UseWord", "false")], expr,
        "2 επί 2 πίνακας; γραμμή 1; στήλη 1; 1, στήλη 2; 2; γραμμή 2; στήλη 1; 3, στήλη 2; 4")
}

#[test]
fn cases() -> Result<()> {
    // Keep each branch and its condition audible.
    let expr = r#"<math><mo>{</mo><mtable><mtr><mtd><mi>x</mi></mtd><mtd><mtext>αν</mtext><mi>x</mi><mo>&gt;</mo><mn>0</mn></mtd></mtr><mtr><mtd><mn>0</mn></mtd><mtd><mtext>αλλιώς</mtext></mtd></mtr></mtable></math>"#;
    test_prefs("el", "GreekMathReader", vec![("CapitalLetters_UseWord", "false")], expr,
        "2 περιπτώσεις; περίπτωση 1; χι, αν χι, είναι μεγαλύτερο από 0; περίπτωση 2; 0 αλλιώς")
}

#[test]
fn limit() -> Result<()> {
    // Reuse native limit and function application handling.
    let expr = r#"<math><munder><mi>lim</mi><mrow><mi>x</mi><mo>→</mo><mn>0</mn></mrow></munder><mi>f</mi><mo>(</mo><mi>x</mi><mo>)</mo></math>"#;
    test_prefs("el", "GreekMathReader", vec![("CapitalLetters_UseWord", "false")], expr,
        "το όριο όταν χι προσεγγίζει 0; του εφ του χι")
}

#[test]
fn chemistry() -> Result<()> {
    // Preserve chemical subscripts and both element letters.
    let expr = r#"<math><msub><mi>H</mi><mn>2</mn></msub><mi>O</mi></math>"#;
    test_prefs("el", "GreekMathReader", vec![("CapitalLetters_UseWord", "false")], expr,
        "έιτς, κάτω 2, όμικρον")
}

#[test]
fn greek_capital_psi() -> Result<()> {
    // Capital Greek letters use the same phonetic spelling through the native capital handler.
    let expr = r#"<math><mi>Ψ</mi></math>"#;
    test_prefs("el", "GreekMathReader", vec![("CapitalLetters_UseWord", "false")], expr,
        "ψι")
}

#[test]
fn authored_absolute_in_angle() -> Result<()> {
    // Explicit absolute-value intent must not be replaced by a guessed matrix element.
    let expr = r#"<math><mo>⟨</mo><mi>ψ</mi><mrow intent="absolute-value($h)"><mi arg="h">H</mi></mrow><mi>φ</mi><mo>⟩</mo></math>"#;
    test_prefs("el", "GreekMathReader", vec![("CapitalLetters_UseWord", "false")], expr,
        "αριστερή γωνιακή αγκύλη; ψι, η απόλυτη τιμή του έιτς; φι; δεξιά γωνιακή αγκύλη")
}

#[test]
fn dot_decimal_preference() -> Result<()> {
    // Input using a decimal point remains readable when the user selects that convention.
    test_prefs("el", "GreekMathReader", vec![("DecimalSeparator", ".")],
        "<math><mn>3.14</mn></math>", "3 κόμμα 14")
}

#[test]
fn style_is_discoverable_and_preserves_braille() -> Result<()> {
    // Hosts discover the native style without a plugin, and its speech rules must not affect UEB.
    test("el", "GreekMathReader", "<math><mn>2</mn></math>", "2")?;
    assert!(get_supported_speech_styles("el")?.contains(&"GreekMathReader".to_string()));
    set_preference("BrailleCode", "UEB")?;
    set_preference("BrailleNavHighlight", "Off")?;
    set_mathml("<math><mfrac><mn>1</mn><mi>x</mi></mfrac></math>")?;
    let reader_braille = get_braille("")?;
    assert!(!reader_braille.is_empty());
    set_preference("SpeechStyle", "SimpleSpeak")?;
    assert_eq!(reader_braille, get_braille("")?);
    set_preference("SpeechStyle", "ClearSpeak")?;
    assert_eq!(reader_braille, get_braille("")?);
    Ok(())
}

#[test]
fn shared_semantics_work_in_existing_styles() -> Result<()> {
    // The exported semantic vocabulary is available to both established Greek styles.
    let expr = r#"<math><mrow intent="conditional-expectation($x,$y)"><mn arg="x">2</mn><mn arg="y">3</mn></mrow></math>"#;
    test("el", "SimpleSpeak", expr, "δεσμευμένη αναμενόμενη τιμή του 2 δεδομένου του 3")?;
    test("el", "ClearSpeak", expr, "δεσμευμένη αναμενόμενη τιμή του 2 δεδομένου του 3")
}

#[test]
fn authored_silent_intent_preserves_only_its_operands() -> Result<()> {
    // Greek grammar must not override an author who explicitly silences the concept name.
    test("el", "GreekMathReader",
        r#"<math><mrow intent="covariance:silent($x,$y)"><mn arg="x">2</mn><mn arg="y">3</mn></mrow></math>"#,
        "2 3")
}

#[test]
fn terse_and_verbose_semantic_readings_remain_greek() -> Result<()> {
    // Both verbosity extremes use the native vocabulary and preserve their arguments.
    let expr = r#"<math><mrow intent="conditional-expectation($x,$y)"><mn arg="x">2</mn><mn arg="y">3</mn></mrow></math>"#;
    test_prefs("el", "GreekMathReader", vec![("Verbosity", "Terse")], expr,
        "δεσμευμένη αναμενόμενη τιμή του 2 δεδομένου του 3")?;
    test_prefs("el", "GreekMathReader", vec![("Verbosity", "Verbose")], expr,
        "δεσμευμένη αναμενόμενη τιμή του 2 δεδομένου του 3")
}
