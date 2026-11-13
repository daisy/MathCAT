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

#[test]
fn child_intent_blocks_ratio() -> Result<()> {
    // The intent on the ∶ says it is not a ratio colon, so the mrow must not be inferred as a ratio.
    let expr = "<math>
      <mrow>
        <mn>20</mn>
        <mo intent='_divides'>∶</mo>
        <mn>5</mn>
      </mrow>
    </math>";
    test("en", "SimpleSpeak", expr, "20 divides 5")?;
    return Ok(());
}

#[test]
fn child_intent_supplies_ratio() -> Result<()> {
    // The intent on the operator says it is a ∶, so the mrow is inferred as a ratio as if ∶ had been written.
    let expr = "<math>
      <mrow>
        <mn>20</mn>
        <mo intent='∶'>÷</mo>
        <mn>5</mn>
      </mrow>
    </math>";
    let target = "<math data-from-mathml='math'>
      <ratio data-from-mathml='mrow' data-intent-property=':function:'>
        <mn data-from-mathml='mn'>20</mn>
        <mn data-from-mathml='mn'>5</mn>
      </ratio>
    </math>";
    test_intent(expr, target, vec![])?;
    return Ok(());
}

#[test]
fn child_intent_supplies_number_set() -> Result<()> {
    // The intent on the base says it is ℝ, so the msup is inferred as a number set as if ℝ had been written.
    let expr = "<math><msup><mi intent='ℝ'>R</mi><mo>+</mo></msup></math>";
    test("en", "SimpleSpeak", expr, "the positive double-struck cap r")?;
    return Ok(());
}

#[test]
fn child_intent_blocks_number_set() -> Result<()> {
    // The intent on the base says it is not ℝ, so the msup must not be inferred as a number set.
    let expr = "<math><msup><mi intent='foo'>ℝ</mi><mo>+</mo></msup></math>";
    test("en", "SimpleSpeak", expr, "foo superscript plus")?;
    return Ok(());
}

#[test]
fn inferred_and_explicit_unit_same_tree() -> Result<()> {
    // "3 km" with km inferred as a unit (mtext after a number) gives the same tree as an explicit :unit.
    // The mrow holds a coefficient, so it is not itself marked as a unit.
    let target = "<math data-from-mathml='math'>
      <mrow data-from-mathml='mrow' data-changed='added'>
        <mn data-from-mathml='mn'>3</mn>
        <mo data-from-mathml='mo' data-changed='added'>&#x2062;</mo>
        <unit data-from-mathml='mtext' data-intent-property=':unit:'>km</unit>
      </mrow>
    </math>";
    test_intent("<math><mn>3</mn><mtext>km</mtext></math>", target, vec![])?;
    test_intent("<math><mn>3</mn><mtext intent=':unit'>km</mtext></math>", target, vec![])?;
    return Ok(());
}

#[test]
fn derived_unit_marks_containers() -> Result<()> {
    // :unit on the mrow for W/m² reaches the leaves; the power and the mrow joining units with '/' are marked as units.
    let expr = "<math>
      <mrow intent=':unit'>
        <mi>W</mi>
        <mo>/</mo>
        <msup><mi>m</mi><mn>2</mn></msup>
      </mrow>
    </math>";
    let target = "<math data-from-mathml='math'>
      <mrow data-from-mathml='mrow' data-intent-property=':unit:'>
        <unit data-from-mathml='mi' data-intent-property=':unit:'>W</unit>
        <mo data-from-mathml='mo'>/</mo>
        <power data-from-mathml='msup' data-intent-property=':unit:'>
          <unit data-from-mathml='mi' data-intent-property=':unit:'>m</unit>
          <mn data-from-mathml='mn'>2</mn>
        </power>
      </mrow>
    </math>";
    test_intent(expr, target, vec![])?;
    return Ok(());
}

#[test]
fn unit_product_mrow_marked() -> Result<()> {
    // Units joined by a multiplication dot form a unit mrow; a coefficient mrow around it is not marked.
    let expr = "<math>
      <mn>2</mn>
      <mo>&#x2062;</mo>
      <mrow><mi intent=':unit'>N</mi><mo>⋅</mo><mi intent=':unit'>m</mi></mrow>
    </math>";
    let target = "<math data-from-mathml='math'>
      <mrow data-from-mathml='mrow' data-changed='added'>
        <mn data-from-mathml='mn'>2</mn>
        <mo data-from-mathml='mo'>&#x2062;</mo>
        <mrow data-from-mathml='mrow' data-intent-property=':unit:'>
          <unit data-from-mathml='mi' data-intent-property=':unit:'>N</unit>
          <mo data-from-mathml='mo'>⋅</mo>
          <unit data-from-mathml='mi' data-intent-property=':unit:'>m</unit>
        </mrow>
      </mrow>
    </math>";
    test_intent(expr, target, vec![])?;
    return Ok(());
}

#[test]
fn inferred_unit_product_chain() -> Result<()> {
    // Likely→confirm: kg · m without author :unit becomes a unit product mrow.
    let expr = r#"<math>
      <mi mathvariant="normal">kg</mi>
      <mo>&#xB7;</mo>
      <mi mathvariant="normal">m</mi>
    </math>"#;
    let target = "<math data-from-mathml='math'>
      <mrow data-from-mathml='mrow' data-changed='added' data-intent-property=':unit:'>
        <unit data-from-mathml='mi' mathvariant='normal' data-intent-property=':unit:'>kg</unit>
        <mo data-from-mathml='mo'>·</mo>
        <unit data-from-mathml='mi' mathvariant='normal' data-intent-property=':unit:'>m</unit>
      </mrow>
    </math>";
    test_intent(expr, target, vec![])?;
    return Ok(());
}

#[test]
fn inferred_unit_equality_and_power() -> Result<()> {
    // N = kg · m · s⁻²: LHS and RHS leaves confirmed; power and RHS product marked as units.
    let expr = r#"<math>
      <mi mathvariant="normal">N</mi>
      <mo>=</mo>
      <mi mathvariant="normal">kg</mi>
      <mo>&#xB7;</mo>
      <mi mathvariant="normal">m</mi>
      <mo>&#xB7;</mo>
      <msup>
        <mi mathvariant="normal">s</mi>
        <mrow><mo>-</mo><mn>2</mn></mrow>
      </msup>
    </math>"#;
    let target = "<math data-from-mathml='math'>
      <mrow data-from-mathml='mrow' data-changed='added'>
        <unit data-from-mathml='mi' mathvariant='normal' data-intent-property=':unit:'>N</unit>
        <mo data-from-mathml='mo'>=</mo>
        <mrow data-from-mathml='mrow' data-changed='added' data-intent-property=':unit:'>
          <unit data-from-mathml='mi' mathvariant='normal' data-intent-property=':unit:'>kg</unit>
          <mo data-from-mathml='mo'>·</mo>
          <unit data-from-mathml='mi' mathvariant='normal' data-intent-property=':unit:'>m</unit>
          <mo data-from-mathml='mo'>·</mo>
          <power data-from-mathml='msup' data-intent-property=':unit:'>
            <unit data-from-mathml='mi' mathvariant='normal' data-intent-property=':unit:'>s</unit>
            <minus data-from-mathml='mrow' data-intent-property=':prefix:'>
              <mn data-from-mathml='mn'>2</mn>
            </minus>
          </power>
        </mrow>
      </mrow>
    </math>";
    test_intent(expr, target, vec![])?;
    return Ok(());
}

#[test]
fn inferred_unit_fraction() -> Result<()> {
    // m/s as an mfrac of two likely unit leaves.
    let expr = r#"<math>
      <mfrac>
        <mi mathvariant="normal">m</mi>
        <mi mathvariant="normal">s</mi>
      </mfrac>
    </math>"#;
    let target = "<math data-from-mathml='math'>
      <fraction data-from-mathml='mfrac' data-intent-property=':unit:'>
        <unit data-from-mathml='mi' mathvariant='normal' data-intent-property=':unit:'>m</unit>
        <unit data-from-mathml='mi' mathvariant='normal' data-intent-property=':unit:'>s</unit>
      </fraction>
    </math>";
    test_intent(expr, target, vec![])?;
    return Ok(());
}

#[test]
fn lone_likely_unit_not_confirmed() -> Result<()> {
    // A lone normal known unit token stays an identifier (not promoted to unit).
    let expr = r#"<math><mi mathvariant="normal">m</mi></math>"#;
    let target = "<math data-from-mathml='math'>
      <mi data-from-mathml='mi' mathvariant='normal'>m</mi>
    </math>";
    test_intent(expr, target, vec![])?;
    return Ok(());
}
