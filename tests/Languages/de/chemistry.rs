/// Tests for rules shared between various speech styles:
/// *  modified var
use crate::common::*;
use anyhow::Result;

#[test]
fn salt() -> Result<()> {
  let expr = "<math><mi>Na</mi><mi>Cl</mi></math>";
  test_prefs("de", "SimpleSpeak", vec![("Verbosity", "Terse")], expr, "groß n a, groß c l")?;
  return Ok(());

}

#[test]
fn water() -> Result<()> {
  let expr = "<math><msub><mi>H</mi><mn>2</mn></msub><mi>O</mi></math>";
  test_prefs("de", "ClearSpeak", vec![("Verbosity", "Terse")], expr, "groß h, 2 groß o")?;
  test_prefs("de", "ClearSpeak", vec![("Verbosity", "Medium")], expr, "groß h, sub 2 groß o")?;
  test_prefs("de", "ClearSpeak", vec![("Verbosity", "Verbose")], expr, "groß h, index 2, groß o")?;
  return Ok(());

}

#[test]
fn carbon() -> Result<()> {
  let expr = "<math><mi>C</mi></math>";     // not enough to trigger recognition
  test_prefs("de", "SimpleSpeak", vec![("Verbosity", "Terse")], expr, "groß c")?;
  return Ok(());

}

#[test]
fn sulfate() -> Result<()> {
  let expr = "<math><mrow><msup>
          <mrow><mo>[</mo><mi>S</mi><msub><mi>O</mi><mn>4</mn></msub><mo>]</mo></mrow>
          <mrow><mn>2</mn><mo>&#x2212;</mo></mrow>
      </msup></mrow></math>";
  test_prefs("de", "ClearSpeak", vec![("Verbosity", "Medium")], expr, "offene eckige klammer, groß s, groß o, sub 4; schließende eckige klammer super 2 minus")?;
  return Ok(());

}

#[test]
fn chemical_state_names_are_german_adjectives() -> Result<()> {
  // Chemical state suffixes should use the standard German adjectival names.
  test_prefs(
    "de",
    "ClearSpeak",
    vec![("Verbosity", "Terse")],
    "<math><mi>Na</mi><mrow><mo>(</mo><mi mathvariant='normal'>s</mi><mo>)</mo></mrow></math>",
    "groß n a, fest",
  )?;
  test_prefs(
    "de",
    "ClearSpeak",
    vec![("Verbosity", "Terse")],
    "<math><msub><mi>H</mi><mn>2</mn></msub><mi>O</mi><mrow><mo>(</mo><mi mathvariant='normal'>l</mi><mo>)</mo></mrow></math>",
    "groß h, 2 groß o, flüssig",
  )?;
  test_prefs(
    "de",
    "ClearSpeak",
    vec![("Verbosity", "Terse")],
    "<math><mi>HCl</mi><mrow><mo>(</mo><mi mathvariant='normal'>g</mi><mo>)</mo></mrow></math>",
    "groß h, groß c l, gasförmig",
  )?;
  test_prefs(
    "de",
    "ClearSpeak",
    vec![("Verbosity", "Terse")],
    "<math><mi>HCl</mi><mrow><mo>(</mo><mi>aq</mi><mo>)</mo></mrow></math>",
    "groß h, groß c l, wässrig",
  )?;
  Ok(())
}

#[test]
fn reaction_arrow_uses_idiomatic_german() -> Result<()> {
  // A reaction arrow should describe the resulting products idiomatically in German.
  let expr = "<math><mrow>
      <mn>2</mn><mi>H</mi><mi>Cl</mi><mo>+</mo><mn>2</mn><mtext>Na</mtext>
      <mo>&#x2192;</mo>
      <mn>2</mn><mtext>Na</mtext><mi>Cl</mi><mo>+</mo>
      <msub><mi>H</mi><mn>2</mn></msub>
    </mrow></math>";
  test_prefs(
    "de",
    "SimpleSpeak",
    vec![("Verbosity", "Terse")],
    expr,
    "2, groß h, groß c l; plus 2 groß n a; ergibt; 2, groß n a, groß c l; plus groß h, 2",
  )?;
  test_prefs(
    "de",
    "SimpleSpeak",
    vec![("Verbosity", "Verbose")],
    expr,
    "2, groß h, groß c l; plus 2 groß n a; ergibt; 2, groß n a, groß c l; plus groß h, index 2",
  )?;
  Ok(())
}

