//! Presentation MathML document examples from Greek Math Reader, adapted to native MathCAT conventions.
use crate::common::*;
use anyhow::Result;

#[test]
fn content_mathml_interval() -> Result<()> {
    // Content MathML interval is converted to supported Presentation MathML with explicit intent.
    let expr = r#"<math><mrow intent="open-closed-interval($a,$b)"><mn arg="a">0</mn><mn arg="b">1</mn></mrow></math>"#;
    test_prefs("el", "GreekMathReader", vec![("CapitalLetters_UseWord", "false")], expr,
        "διάστημα ανοιχτό αριστερά κλειστό δεξιά από 0 έως 1")
}

#[test]
fn mathjax_semantics() -> Result<()> {
    // Read the presentation branch and ignore the TeX annotation, preserving nested boundaries.
    let expr = r#"<math xmlns="http://www.w3.org/1998/Math/MathML">
  <semantics>
    <mrow>
      <mi>x</mi><mo>=</mo>
      <mfrac>
        <mrow><mo>−</mo><mi>b</mi><mo>±</mo><msqrt><msup><mi>b</mi><mn>2</mn></msup><mo>−</mo><mn>4</mn><mi>a</mi><mi>c</mi></msqrt></mrow>
        <mrow><mn>2</mn><mi>a</mi></mrow>
      </mfrac>
    </mrow>
    <annotation encoding="application/x-tex">x=\frac{-b\pm\sqrt{b^2-4ac}}{2a}</annotation>
  </semantics>
</math>"#;
    test_prefs("el", "GreekMathReader", vec![("CapitalLetters_UseWord", "false")], expr,
        "χι ίσον; κλάσμα με αριθμητή μείον μπί, συν πλην; τετραγωνική ρίζα του μπί στο τετράγωνο, μείον, 4 έι σί, τέλος ρίζας, και παρονομαστή 2 έι, τέλος κλάσματος")
}

#[test]
fn mathtype_annotation_xml() -> Result<()> {
    // Use the presentation annotation when it is the available MathML branch.
    let expr = r#"<math xmlns="http://www.w3.org/1998/Math/MathML">
  <semantics>
    <annotation encoding="application/x-tex">\int_0^1 x^2\,dx</annotation>
    <annotation-xml encoding="MathML-Presentation">
      <mrow>
        <msubsup><mo>∫</mo><mn>0</mn><mn>1</mn></msubsup>
        <msup><mi>x</mi><mn>2</mn></msup><mi>d</mi><mi>x</mi>
      </mrow>
    </annotation-xml>
  </semantics>
</math>"#;
    test_prefs("el", "GreekMathReader", vec![("CapitalLetters_UseWord", "false")], expr,
        "ολοκλήρωμα από 0 ως 1 του; χι στο τετράγωνο ως προς χι")
}

#[test]
fn quantum_matrix_element() -> Result<()> {
    // Paired bars inside angle brackets identify all three matrix-element operands.
    let expr = r#"<math xmlns="http://www.w3.org/1998/Math/MathML">
  <mo>⟨</mo><mi>ψ</mi><mo>|</mo><mi>H</mi><mo>|</mo><mi>φ</mi><mo>⟩</mo>
</math>"#;
    test_prefs("el", "GreekMathReader", vec![("CapitalLetters_UseWord", "false")], expr,
        "στοιχείο πίνακα με μπρα ψι έιτς κετ φι")
}

#[test]
fn statistics_expectation() -> Result<()> {
    // Keep both the random variable and conditioning event audible.
    let expr = r#"<math xmlns="http://www.w3.org/1998/Math/MathML">
  <mi>E</mi><mrow><mo>[</mo><mi>X</mi><mo>|</mo><mi>Y</mi><mo>]</mo></mrow>
</math>"#;
    test_prefs("el", "GreekMathReader", vec![("CapitalLetters_UseWord", "false")], expr,
        "αναμενόμενη τιμή του χι δεδομένου του γουάι")
}

#[test]
fn tagged_pdf_formula() -> Result<()> {
    // Latin E remains distinct from Greek epsilon, including outside statistical functions.
    let expr = r#"<math xmlns="http://www.w3.org/1998/Math/MathML">
  <mrow>
    <mi>E</mi><mo>=</mo><mi>m</mi><msup><mi>c</mi><mn>2</mn></msup>
  </mrow>
</math>"#;
    test_prefs("el", "GreekMathReader", vec![("CapitalLetters_UseWord", "false")], expr,
        "ί ίσον, μι σί στο τετράγωνο")
}

#[test]
fn tensor_indices() -> Result<()> {
    // Preserve left and right tensor indices through native multiscript handling.
    let expr = r#"<math xmlns="http://www.w3.org/1998/Math/MathML">
  <mmultiscripts><mi>T</mi><mi>j</mi><none/><mprescripts/><none/><mi>i</mi></mmultiscripts>
</math>"#;
    test_prefs("el", "GreekMathReader", vec![("CapitalLetters_UseWord", "false")], expr,
        "ταυ προτεταγμένος εκθέτης άι, δείκτης τζέι")
}

#[test]
fn vector_calculus() -> Result<()> {
    // Inherit vector-calculus intent and preserve the bold-typeface cue.
    let expr = r#"<math xmlns="http://www.w3.org/1998/Math/MathML">
  <mo>∇</mo><mo>·</mo><mi mathvariant="bold">F</mi><mo>=</mo><mn>0</mn>
</math>"#;
    test_prefs("el", "GreekMathReader", vec![("CapitalLetters_UseWord", "false")], expr,
        "απόκλιση του έντονο εφ; ίσον 0")
}

#[test]
fn word_uia_prefixed() -> Result<()> {
    // Accept a correctly declared MathML prefix from a Word-style expression.
    let expr = r#"<mml:math xmlns:mml="http://www.w3.org/1998/Math/MathML">
  <mml:mrow>
    <mml:munderover>
      <mml:mo>∑</mml:mo>
      <mml:mrow><mml:mi>n</mml:mi><mml:mo>=</mml:mo><mml:mn>1</mml:mn></mml:mrow>
      <mml:mi>∞</mml:mi>
    </mml:munderover>
    <mml:mfrac><mml:mn>1</mml:mn><mml:msup><mml:mi>n</mml:mi><mml:mn>2</mml:mn></mml:msup></mml:mfrac>
  </mml:mrow>
</mml:math>"#;
    test_prefs("el", "GreekMathReader", vec![("CapitalLetters_UseWord", "false")], expr,
        "άθροισμα από νι ίσον 1 ως άπειρο του; κλάσμα με αριθμητή 1, και παρονομαστή νι στο τετράγωνο, τέλος κλάσματος")
}

#[test]
fn ordinary_derivative() -> Result<()> {
    // Recognize matching differentials in a first derivative.
    let expr = r#"<math><mfrac><mrow><mi>d</mi><mi>f</mi></mrow><mrow><mi>d</mi><mi>x</mi></mrow></mfrac></math>"#;
    test_prefs("el", "GreekMathReader", vec![("CapitalLetters_UseWord", "false")], expr,
        "παράγωγος του εφ ως προς χι")
}

#[test]
fn conditional_probability() -> Result<()> {
    // Recognize a conditional probability without discarding its event.
    let expr = r#"<math><mi>P</mi><mo>(</mo><mi>A</mi><mo>|</mo><mi>B</mi><mo>)</mo></math>"#;
    test_prefs("el", "GreekMathReader", vec![("CapitalLetters_UseWord", "false")], expr,
        "πιθανότητα του έι δεδομένου του μπί")
}

#[test]
fn expectation_parens() -> Result<()> {
    // Recognize the conventional expectation function with parentheses.
    let expr = r#"<math><mi>E</mi><mo>(</mo><mi>X</mi><mo>)</mo></math>"#;
    test_prefs("el", "GreekMathReader", vec![("CapitalLetters_UseWord", "false")], expr,
        "αναμενόμενη τιμή του χι")
}

#[test]
fn variance_function() -> Result<()> {
    // Use the Greek variance head for the named function.
    let expr = r#"<math><mi>Var</mi><mo>(</mo><mi>X</mi><mo>)</mo></math>"#;
    test_prefs("el", "GreekMathReader", vec![("CapitalLetters_UseWord", "false")], expr,
        "διακύμανση του χι")
}

#[test]
fn bare_unknown() -> Result<()> {
    // Unrecognized identifiers must remain audible through the native fallback.
    let expr = r#"<math><mo>⧖</mo><mi>fooBar</mi></math>"#;
    test_prefs("el", "GreekMathReader", vec![("CapitalLetters_UseWord", "false")], expr,
        "λευκή κλεψύδρα, fooBar")
}

#[test]
fn scalar_product() -> Result<()> {
    // Read scalar multiplication without falsely assigning a vector dot-product intent.
    let expr = r#"<math><mi>a</mi><mo>·</mo><mi>b</mi></math>"#;
    test_prefs("el", "GreekMathReader", vec![("CapitalLetters_UseWord", "false")], expr,
        "έι επί μπί")
}

#[test]
fn covariance() -> Result<()> {
    // Join both covariance operands with Greek grammar.
    let expr = r#"<math><mi>Cov</mi><mo>(</mo><mi>X</mi><mo>,</mo><mi>Y</mi><mo>)</mo></math>"#;
    test_prefs("el", "GreekMathReader", vec![("CapitalLetters_UseWord", "false")], expr,
        "συνδιακύμανση των χι και γουάι")
}

#[test]
fn bra_ket() -> Result<()> {
    // Preserve both states in the inner product.
    let expr = r#"<math><mo>⟨</mo><mi>ψ</mi><mo>|</mo><mi>φ</mi><mo>⟩</mo></math>"#;
    test_prefs("el", "GreekMathReader", vec![("CapitalLetters_UseWord", "false")], expr,
        "εσωτερικό γινόμενο ψι με φι")
}

#[test]
fn quantum_dagger() -> Result<()> {
    // A dagger alone is ambiguous; do not silently assert quantum adjoint semantics.
    let expr = r#"<math><msup><mi>A</mi><mo>†</mo></msup></math>"#;
    test_prefs("el", "GreekMathReader", vec![("CapitalLetters_UseWord", "false")], expr,
        "έι οβελίσκος")
}
