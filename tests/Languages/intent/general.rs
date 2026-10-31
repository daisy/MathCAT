/// Tests for:
/// *  functions including trig functions, logs, and functions to powers
/// *  implied times/functional call and explicit times/function call
/// *  parens
/// These are all intertwined, so they are in one file
use crate::common::*;
use anyhow::Result;

#[test]
fn binomial() -> Result<()> {
  let mathml = "<math><mrow>
        <mo>(</mo>
        <mfrac linethickness='0'> <mn arg='n'>7</mn> <mn arg='m'>3</mn> </mfrac>
        <mo>)</mo>
    </mrow></math>";
  let intent = "<math data-from-mathml='math'>
        <binomial data-from-mathml='mrow' data-intent-property=':infix:'>
          <mn data-from-mathml='mn' arg='n'>7</mn>
          <mn data-from-mathml='mn' arg='m'>3</mn>
        </binomial>
    </math>";
  test_intent(mathml, intent, vec![])?;
  // The W3C alias reuses binomial speech without changing inferred binomial semantics.
  test_core_speech("<math><mrow intent='binomial-coefficient($n,$k)'><mn arg='n'>7</mn><mn arg='k'>3</mn></mrow></math>", "7 choose 3", "число сочетаний из 7 по 3")?;
  return Ok(());

}

#[test]
fn closed_interval() -> Result<()> {
    let expr = r#"<math>
      <mo stretchy="false">[</mo>
      <mi>a</mi>
      <mo>,</mo>
      <mi>b</mi>
      <mo stretchy="false">]</mo>
    </math>"#;
    let target = "<math data-from-mathml='math'>
      <closed-interval data-from-mathml='mrow' data-changed='added'>
        <mi data-from-mathml='mi'>a</mi>
        <mi data-from-mathml='mi'>b</mi>
      </closed-interval>
    </math>";
    test_intent(expr, target, vec![])?;
    return Ok(());

}

#[test]
fn nested_interval_bug_329() -> Result<()> {
    let expr = r#"<math>
      <mo stretchy="false">[</mo>
      <mi>A</mi>
      <mo>,</mo>
      <mo stretchy="false">[</mo>
      <mi>B</mi>
      <mo>,</mo>
      <mi>C</mi>
      <mo stretchy="false">]</mo>
      <mo stretchy="false">]</mo>
    </math>"#;
    let target = "<math data-from-mathml='math'>
    <mrow data-from-mathml='mrow' data-changed='added'>
      <mo data-from-mathml='mo' stretchy='false'>[</mo>
      <mrow data-from-mathml='mrow' data-changed='added'>
        <mi data-from-mathml='mi'>A</mi>
        <mo data-from-mathml='mo'>,</mo>
        <mrow data-from-mathml='mrow' data-changed='added'>
          <mo data-from-mathml='mo' stretchy='false'>[</mo>
          <mrow data-from-mathml='mrow' data-changed='added'>
            <mi data-from-mathml='mi'>B</mi>
            <mo data-from-mathml='mo'>,</mo>
            <mi data-from-mathml='mi'>C</mi>
          </mrow>
          <mo data-from-mathml='mo' stretchy='false'>]</mo>
        </mrow>
      </mrow>
      <mo data-from-mathml='mo' stretchy='false'>]</mo>
    </mrow>
   </math>";
    test_intent(expr, target, vec![])?;
    return Ok(());

}

#[test]
fn evaluated_at() -> Result<()> {
    let expr = r#"<math>
          <msubsup>
              <mrow>
                  <mo>[</mo>
                  <msup><mi>x</mi><mn>2</mn></msup>
                  <mo>+</mo>
                  <mi>x</mi>
                  <mo>]</mo>
              </mrow>
              <mn>0</mn>
              <mn>1</mn>
          </msubsup>
        </math>"#;
    let target = "<math data-from-mathml='math'>
        <evaluate data-from-mathml='msubsup'>
          <mrow data-from-mathml='mrow' data-changed='added'>
            <power data-from-mathml='msup'>
              <mi data-from-mathml='mi'>x</mi>
              <mn data-from-mathml='mn'>2</mn>
            </power>
            <mo data-from-mathml='mo'>+</mo>
            <mi data-from-mathml='mi'>x</mi>
          </mrow>
          <mn data-from-mathml='mn'>0</mn>
          <mn data-from-mathml='mn'>1</mn>
      </evaluate>
    </math>";
    test_intent(expr, target, vec![])?;
    return Ok(());

}

// W3C core speech cases complement the inference checks above.
use super::test_core_speech;

#[test]
fn w3c_repeating_decimal_one_argument() -> Result<()> {
    // W3C supplies the period alone; a missing second child must never cause an XPath error.
    test_core_speech("<math><mover intent='repeating-decimal($repeat)'><mn arg='repeat'>45</mn><mo>_</mo></mover></math>", "repeating decimal 4 5", "период 4 5")?;
    test_core_speech("<math><mrow intent='repeating-decimal(45)'/></math>", "repeating decimal 4 5", "период 4 5")
}

#[test]
fn w3c_repeating_decimal_leading_zeroes() -> Result<()> {
    // A period is a digit sequence: 003 must retain both leading zeroes rather than become 3.
    test_core_speech("<math><mover intent='repeating-decimal:function($repeat)'><mn arg='repeat'>003</mn><mo>¯</mo></mover></math>", "repeating decimal 0 0 3", "период 0 0 3")
}

#[test]
fn w3c_repeating_decimal_legacy_arguments() -> Result<()> {
    // Keep the existing prefix/period extension used by automatic inference.
    test_core_speech("<math><mrow intent='repeating-decimal($prefix,$repeat)'><mn arg='prefix'>12</mn><mn arg='repeat'>003</mn></mrow></math>", "12 with repeating digits 0 0 3", "12 с повторяющимися цифрами 0 0 3")?;
    test_core_speech("<math><mn>12</mn><mover><mn>45</mn><mo>¯</mo></mover></math>", "12 with repeating digits 4 5", "12 с повторяющимися цифрами 4 5")
}

#[test]
fn w3c_repeating_decimal_complete_w3c_example() -> Result<()> {
    // The W3C surrounding decimal must retain its prefix as well as both period digits.
    let mathml="<math><mn>0.0</mn><mover intent='repeating-decimal($repeat)'><mn arg='repeat'>45</mn><mo>_</mo></mover></math>";
    for (language,expected) in [("en","0.0 with repeating digits 4 5"),("ru","0.0 с повторяющимися цифрами 4 5")] {
        for style in ["ClearSpeak","SimpleSpeak"] {
            for verbosity in ["Terse","Medium","Verbose"] {
                test_prefs(language,style,vec![("Verbosity",verbosity),("IntentErrorRecovery","Error"),("DecimalSeparators","."),("BlockSeparators",", ")],mathml,expected)?;
            }
        }
    }
    Ok(())
}

#[test]
fn w3c_repeating_decimal_semantic_navigation() -> Result<()> {
    // Semantic reading must use the same one-argument rule when revisiting the current expression.
    for (language,expected) in [("en","repeating decimal 4 5"),("ru","период 4 5")] {
        for style in ["ClearSpeak","SimpleSpeak"] {
            init_nav(language,"<math><mrow intent='repeating-decimal(45)'/></math>")?;
            set_preference("SpeechStyle",style)?;
            let speech=do_navigate_command("ReadCurrent")?;
            assert!(speech.contains(expected),"{language}/{style}: {speech}");
        }
    }
    Ok(())
}

#[test]
fn w3c_blank_intent_representations() -> Result<()> {
    // The semantic blank is independent of its visible spelling, including a space-only blank.
    test_core_speech("<math><mi intent='blank'>__</mi></math>", "blank", "пустое место")?;
    test_core_speech("<math><mo intent='blank'>?</mo></math>", "blank", "пустое место")?;
    test_core_speech("<math><mspace intent='blank' width='3em'/></math>", "blank", "пустое место")
}

#[test]
fn w3c_invisible_times_keeps_operands() -> Result<()> {
    // W3C assigns silent fixity: suppress the operator name, never its operands.
    test_core_speech("<math><mrow intent='invisible-times($x,$y)'><mi arg='x'>x</mi><mi arg='y'>y</mi></mrow></math>", "x y", "икс игрек")
}

#[test]
fn w3c_invisible_separator_keeps_operands() -> Result<()> {
    // A silent separator must not add the word 'comma' or drop any listed operand.
    test_core_speech("<math><mrow intent='invisible-separator($x,$y,$z)'><mi arg='x'>x</mi><mi arg='y'>y</mi><mi arg='z'>z</mi></mrow></math>", "x y z", "икс игрек зэт")
}

#[test]
fn w3c_invisible_operator_leaves_are_silent() -> Result<()> {
    // Explicit annotations on the Unicode operator itself have no spoken name or argument.
    test_core_speech("<math><mo intent='invisible-times'>&#x2062;</mo></math>", "", "")?;
    test_core_speech("<math><mo intent='invisible-separator'>&#x2063;</mo></math>", "", "")
}
