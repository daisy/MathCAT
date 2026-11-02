use crate::common::*;
use anyhow::Result;

// Check this language's W3C speech in both styles and at every verbosity.
fn test_w3c_speech(mathml: &str, expected: &str) -> Result<()> {
    for style in ["ClearSpeak", "SimpleSpeak"] {
        for verbosity in ["Terse", "Medium", "Verbose"] {
            test_prefs("en", style, vec![("Verbosity", verbosity), ("IntentErrorRecovery", "Error")], mathml, expected)?;
        }
    }
    Ok(())
}

#[test]
fn w3c_repeating_decimal_one_argument() -> Result<()> {
    // W3C supplies the period alone; a missing second child must never cause an XPath error.
    test_w3c_speech("<math><mover intent='repeating-decimal($repeat)'><mn arg='repeat'>45</mn><mo>_</mo></mover></math>", "repeating decimal 4 5")?;
    test_w3c_speech("<math><mrow intent='repeating-decimal(45)'/></math>", "repeating decimal 4 5")
}

#[test]
fn w3c_repeating_decimal_leading_zeroes() -> Result<()> {
    // A period is a digit sequence: 003 must retain both leading zeroes rather than become 3.
    test_w3c_speech("<math><mover intent='repeating-decimal:function($repeat)'><mn arg='repeat'>003</mn><mo>¯</mo></mover></math>", "repeating decimal 0 0 3")
}

#[test]
fn w3c_repeating_decimal_legacy_arguments() -> Result<()> {
    // Keep the existing prefix/period extension used by automatic inference.
    test_w3c_speech("<math><mrow intent='repeating-decimal($prefix,$repeat)'><mn arg='prefix'>12</mn><mn arg='repeat'>003</mn></mrow></math>", "12 with repeating digits 0 0 3")?;
    test_w3c_speech("<math><mn>12</mn><mover><mn>45</mn><mo>¯</mo></mover></math>", "12 with repeating digits 4 5")
}

#[test]
fn w3c_repeating_decimal_complete_w3c_example() -> Result<()> {
    // The W3C surrounding decimal must retain its prefix as well as both period digits.
    let mathml="<math><mn>0.0</mn><mover intent='repeating-decimal($repeat)'><mn arg='repeat'>45</mn><mo>_</mo></mover></math>";
    let (language,expected) = ("en","0.0 with repeating digits 4 5");
    for style in ["ClearSpeak","SimpleSpeak"] {
        for verbosity in ["Terse","Medium","Verbose"] {
            test_prefs(language,style,vec![("Verbosity",verbosity),("IntentErrorRecovery","Error"),("DecimalSeparators","."),("BlockSeparators",", ")],mathml,expected)?;
        }
    }
    Ok(())
}

#[test]
fn w3c_repeating_decimal_semantic_navigation() -> Result<()> {
    // Semantic reading must use the same one-argument rule when revisiting the current expression.
    let (language,expected) = ("en","repeating decimal 4 5");
    for style in ["ClearSpeak","SimpleSpeak"] {
        init_nav(language,"<math><mrow intent='repeating-decimal(45)'/></math>")?;
        set_preference("SpeechStyle",style)?;
        let speech=do_navigate_command("ReadCurrent")?;
        assert!(speech.contains(expected),"{language}/{style}: {speech}");
    }
    Ok(())
}

#[test]
fn w3c_blank_intent_representations() -> Result<()> {
    // The semantic blank is independent of its visible spelling, including a space-only blank.
    test_w3c_speech("<math><mi intent='blank'>__</mi></math>", "blank")?;
    test_w3c_speech("<math><mo intent='blank'>?</mo></math>", "blank")?;
    test_w3c_speech("<math><mspace intent='blank' width='3em'/></math>", "blank")
}

#[test]
fn w3c_invisible_times_keeps_operands() -> Result<()> {
    // W3C assigns silent fixity: suppress the operator name, never its operands.
    test_w3c_speech("<math><mrow intent='invisible-times($x,$y)'><mi arg='x'>x</mi><mi arg='y'>y</mi></mrow></math>", "x y")
}

#[test]
fn w3c_invisible_separator_keeps_operands() -> Result<()> {
    // A silent separator must not add the word 'comma' or drop any listed operand.
    test_w3c_speech("<math><mrow intent='invisible-separator($x,$y,$z)'><mi arg='x'>x</mi><mi arg='y'>y</mi><mi arg='z'>z</mi></mrow></math>", "x y z")
}

#[test]
fn w3c_invisible_operator_leaves_are_silent() -> Result<()> {
    // Explicit annotations on the Unicode operator itself have no spoken name or argument.
    test_w3c_speech("<math><mo intent='invisible-times'>&#x2062;</mo></math>", "")?;
    test_w3c_speech("<math><mo intent='invisible-separator'>&#x2063;</mo></math>", "")
}

#[test]
fn w3c_definite_integral_domain() -> Result<()> {
    // The single W3C argument is an integration domain, not an integrand or integral sign.
    test_w3c_speech("<math><msub intent='definite-integral($d)'><mo>∫</mo><mi arg='d'>x</mi></msub></math>", "integral over x")
}

#[test]
fn w3c_definite_integral_bounds() -> Result<()> {
    // Two W3C arguments are lower and upper bounds, in that order; neither may disappear.
    test_w3c_speech("<math><msubsup intent='definite-integral($lo,$hi)'><mo>∫</mo><mn arg='lo'>0</mn><mn arg='hi'>1</mn></msubsup></math>", "integral from 0 to 1")
}

#[test]
fn w3c_definite_integral_infinite_bounds() -> Result<()> {
    // Infinite bounds must retain both signs and endpoint values.
    let mathml="<math><msubsup intent='definite-integral($lo,$hi)'><mo>∫</mo><mrow arg='lo'><mo>−</mo><mi>∞</mi></mrow><mi arg='hi'>∞</mi></msubsup></math>";
    test_w3c_speech(mathml, "integral from negative infinity to infinity")
}

#[test]
fn w3c_derivative_leibniz_second_order() -> Result<()> {
    // The three arguments denote function, variable, and order, regardless of fraction layout.
    let mathml = "<math><mfrac intent='derivative($f,$x,$n)'><mrow><msup><mi>d</mi><mn arg='n'>2</mn></msup><mi arg='f'>y</mi></mrow><mrow><mi>d</mi><msup><mi arg='x'>x</mi><mn>2</mn></msup></mrow></mfrac></math>";
    test_w3c_speech(mathml, "the second derivative of y with respect to x")
}

#[test]
fn w3c_derivative_prime_first_order() -> Result<()> {
    // Prime notation needs the same semantic meaning, without assuming a spoken Leibniz fraction.
    let mathml = "<math><mrow intent='derivative($f,$x,1)'><msup><mi arg='f'>f</mi><mo>′</mo></msup><mo>(</mo><mi arg='x'>x</mi><mo>)</mo></mrow></math>";
    test_w3c_speech(mathml, "the derivative of f with respect to x")
}

#[test]
fn w3c_derivative_symbolic_order() -> Result<()> {
    // Symbolic orders need an order phrase, not a numeric-ordinal conversion of the letter n.
    let mathml="<math><mrow intent='derivative($f,$x,$n)'><mi arg='f'>f</mi><mi arg='x'>x</mi><mi arg='n'>n</mi></mrow></math>";
    test_w3c_speech(mathml, "the derivative of order n of f with respect to x")
}

#[test]
fn w3c_calculus_intent_read_current() -> Result<()> {
    // Semantic navigation must use the same integral and derivative readings as full speech.
    let (language, integral, derivative) = ("en", "integral from 0 to 1", "the derivative of f with respect to x");
    for style in ["ClearSpeak", "SimpleSpeak"] {
        init_nav(language, "<math><msubsup intent='definite-integral(0,1)'><mo>∫</mo><mn>0</mn><mn>1</mn></msubsup></math>")?;
        set_preference("SpeechStyle", style)?;
        let speech=do_navigate_command("ReadCurrent")?.split_whitespace().collect::<Vec<_>>().join(" ");
        assert!(speech.contains(integral), "{language}/{style}: {speech}");
        init_nav(language, "<math><msup intent='derivative($f,x,1)'><mi arg='f'>f</mi><mo>′</mo></msup></math>")?;
        set_preference("SpeechStyle", style)?;
        let speech=do_navigate_command("ReadCurrent")?.split_whitespace().collect::<Vec<_>>().join(" ");
        assert!(speech.contains(derivative), "{language}/{style}: {speech}");
    }
    Ok(())
}

#[test]
fn w3c_binomial_coefficient() -> Result<()> {
    // The W3C alias reuses binomial speech without changing inferred semantics.
    test_w3c_speech("<math><mrow intent='binomial-coefficient($n,$k)'><mn arg='n'>7</mn><mn arg='k'>3</mn></mrow></math>", "7 choose 3")
}
