/// Tests for:
/// *  calculus-related inference
use crate::common::*;
use anyhow::Result;


#[test]
fn laplacian() -> Result<()> {
  let mathml = r#"<math>
        <msup> <mo>∇</mo> <mn>2</mn> </msup>
        <mi>&#x3C8;</mi>
    </math>"#;
  let intent = r#"<math data-from-mathml='math' >
      <laplacian data-from-mathml='mrow' data-changed='added' data-intent-property=':function:'>
        <mi data-from-mathml='mi'>ψ</mi>
      </laplacian>
   </math>"#;
    test_intent(mathml, intent, vec![])?;
    return Ok(());

}

#[test]
fn laplacian_as_vector() -> Result<()> {
  let mathml = r#"<math>
        <msup> <mover><mo>∇</mo><mo>&#x2192;</mo></mover> <mn>2</mn> </msup>
        <mi>&#x3C8;</mi>
    </math>"#;
  let intent = r#"<math data-from-mathml='math' >
      <laplacian data-from-mathml='mrow' data-changed='added' data-intent-property=':function:'>
        <mi data-from-mathml='mi'>ψ</mi>
      </laplacian>
   </math>"#;
    test_intent(mathml, intent, vec![])?;
    return Ok(());

}

#[test]
fn laplacian_as_operator() -> Result<()> {
  let mathml = r#"<math>
    <msup> <mo>𝛁</mo> <mn>2</mn> </msup>
    </math>"#;
  let intent = r#"<math data-from-mathml='math' >
      <laplacian data-from-mathml='msup' />
   </math>"#;
    test_intent(mathml, intent, vec![])?;
    return Ok(());

}

#[test]
fn simple_gradient() -> Result<()> {
  let mathml = r#"<math>
        <mi mathvariant="normal">∇</mi>
        <mi>f</mi>
    </math>"#;
  let intent = r#"<math data-from-mathml='math'>
        <gradient data-from-mathml='mrow' data-changed='added' data-intent-property=':function:'>
            <mi data-from-mathml='mi'>f</mi>
        </gradient>
    </math>"#;
    test_intent(mathml, intent, vec![])?;
    return Ok(());

}

#[test]
fn simple_bold_gradient() -> Result<()> {
  let mathml = r#"<math>
        <mi mathvariant="bold">∇</mi>
        <mi>f</mi>
    </math>"#;
  let intent = r#"<math data-from-mathml='math'>
        <gradient data-from-mathml='mrow' data-changed='added' data-intent-property=':function:'>
            <mi data-from-mathml='mi'>f</mi>
        </gradient>
       </math>"#;
    test_intent(mathml, intent, vec![])?;
    return Ok(());

}

#[test]
fn simple_div() -> Result<()> {
  let mathml = r#"<math>
        <mi mathvariant="normal">∇</mi>
        <mo>⋅</mo>
        <mi mathvariant="bold">f</mi>
     </math>"#;
  let intent = r#"<math data-from-mathml='math'>
    <divergence data-from-mathml='mrow' data-changed='added' data-intent-property=':function:'>
        <mi data-from-mathml='mi' mathvariant='bold'>𝐟</mi>
    </divergence> 
   </math>"#;
    test_intent(mathml, intent, vec![])?;
    return Ok(());

}

#[test]
fn simple_curl() -> Result<()> {
  let mathml = r#"<math>
        <mi mathvariant="normal">∇</mi>
        <mo>&#xD7;</mo>
        <mi mathvariant="bold">f</mi>
    </math>"#;
  let intent = r#"<math data-from-mathml='math'>
        <curl data-from-mathml='mrow' data-changed='added' data-intent-property=':function:'>
            <mi data-from-mathml='mi' mathvariant='bold'>𝐟</mi>
        </curl>
    </math>"#;
    test_intent(mathml, intent, vec![])?;
    return Ok(());

}

#[test]
fn curl_in_mrow() -> Result<()> {
  let mathml = r#"<math>
        <mi>r</mi>
        <mi mathvariant="normal">&#x2207;</mi>
        <mo>&#xD7;</mo>
        <mrow >
            <mi mathvariant="bold">A</mi>
        </mrow>
    </math>"#;
  let intent = r#"<math data-from-mathml='math'>
    <mrow data-from-mathml='mrow' data-changed='added'>
        <mi data-from-mathml='mi'>r</mi>
        <mo data-from-mathml='mo' data-changed='added'>&#x2062;</mo>
        <curl data-from-mathml='mrow' data-changed='added' data-intent-property=':function:'>
        <mi data-from-mathml='mi' mathvariant='bold'>𝐀</mi>
        </curl>
    </mrow>
    </math>"#;
    test_intent(mathml, intent, vec![])?;
    return Ok(());

}

#[test]
fn gradient_vector() -> Result<()> {
  let mathml = r#"<math>
        <mover>
            <mi mathvariant="normal">&#x2207;</mi>
            <mo stretchy="false">&#x2192;</mo>
        </mover>
        <mi>f</mi>
    </math>"#;
  let intent = r#"<math data-from-mathml='math'>
        <gradient data-from-mathml='mrow' data-changed='added' data-intent-property=':function:'>
            <mi data-from-mathml='mi'>f</mi>
        </gradient>
    </math>"#;
    test_intent(mathml, intent, vec![])?;
    return Ok(());

}

#[test]
fn curl_vector() -> Result<()> {
  let mathml = r#"<math>
        <mover>
            <mi mathvariant="normal">&#x2207;</mi>
            <mo stretchy="false">&#x2192;</mo>
        </mover>
        <mo>&#xD7;</mo>
        <mover>
            <mi>f</mi>
            <mo stretchy="false">&#x2192;</mo>
        </mover>
    </math>"#;
  let intent = r#"<math data-from-mathml='math'>
        <curl data-from-mathml='mrow' data-changed='added' data-intent-property=':function:'>
            <vector data-from-mathml='mover' data-intent-property=':prefix:'>
                <mi data-from-mathml='mi'>f</mi>
            </vector>
        </curl>
    </math>"#;
    test_intent(mathml, intent, vec![])?;
    return Ok(());

}

// W3C core speech cases complement the inference checks above.
use super::test_core_speech;

#[test]
fn w3c_definite_integral_domain() -> Result<()> {
    // The single W3C argument is an integration domain, not an integrand or integral sign.
    test_core_speech("<math><msub intent='definite-integral($d)'><mo>∫</mo><mi arg='d'>x</mi></msub></math>", "integral over x", "интеграл по икс")
}

#[test]
fn w3c_definite_integral_bounds() -> Result<()> {
    // Two W3C arguments are lower and upper bounds, in that order; neither may disappear.
    test_core_speech("<math><msubsup intent='definite-integral($lo,$hi)'><mo>∫</mo><mn arg='lo'>0</mn><mn arg='hi'>1</mn></msubsup></math>", "integral from 0 to 1", "интеграл от 0 до 1")
}

#[test]
fn w3c_definite_integral_infinite_bounds() -> Result<()> {
    // Infinite bounds need the Russian genitive after both 'от' and 'до'.
    let mathml="<math><msubsup intent='definite-integral($lo,$hi)'><mo>∫</mo><mrow arg='lo'><mo>−</mo><mi>∞</mi></mrow><mi arg='hi'>∞</mi></msubsup></math>";
    test_core_speech(mathml, "integral from negative infinity to infinity", "интеграл от минус бесконечности до бесконечности")
}

#[test]
fn w3c_derivative_leibniz_second_order() -> Result<()> {
    // The three arguments denote function, variable, and order, regardless of fraction layout.
    let mathml = "<math><mfrac intent='derivative($f,$x,$n)'><mrow><msup><mi>d</mi><mn arg='n'>2</mn></msup><mi arg='f'>y</mi></mrow><mrow><mi>d</mi><msup><mi arg='x'>x</mi><mn>2</mn></msup></mrow></mfrac></math>";
    test_core_speech(mathml, "the second derivative of y with respect to x", "производная порядка 2 игрек по икс")
}

#[test]
fn w3c_derivative_prime_first_order() -> Result<()> {
    // Prime notation needs the same semantic meaning, without assuming a spoken Leibniz fraction.
    let mathml = "<math><mrow intent='derivative($f,$x,1)'><msup><mi arg='f'>f</mi><mo>′</mo></msup><mo>(</mo><mi arg='x'>x</mi><mo>)</mo></mrow></math>";
    test_core_speech(mathml, "the derivative of f with respect to x", "производная эф по икс")
}

#[test]
fn w3c_derivative_symbolic_order() -> Result<()> {
    // Symbolic orders need an order phrase, not a numeric-ordinal conversion of the letter n.
    let mathml="<math><mrow intent='derivative($f,$x,$n)'><mi arg='f'>f</mi><mi arg='x'>x</mi><mi arg='n'>n</mi></mrow></math>";
    test_core_speech(mathml, "the derivative of order n of f with respect to x", "производная порядка эн эф по икс")
}

#[test]
fn w3c_calculus_intent_read_current() -> Result<()> {
    // Semantic navigation must use the same integral and derivative readings as full speech.
    for (language, integral, derivative) in [
        ("en", "integral from 0 to 1", "the derivative of f with respect to x"),
        ("ru", "интеграл от 0 до 1", "производная эф по икс"),
    ] {
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
    }
    Ok(())
}
