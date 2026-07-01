/// Tests for superscripts
///   simple superscripts
///   complex/nested superscripts
use crate::common::*;
use anyhow::Result;

#[test]
fn squared() -> Result<()> {
    let expr = "<math>
                    <msup> <mi>x</mi> <mn>2</mn> </msup>
                </math>";
    test_prefs("fr", "ClearSpeak", vec![("Verbosity", "Medium"), ("ClearSpeak_Exponents", "Auto")], expr, "x au carré")?;
    test_prefs("fr", "ClearSpeak", vec![("Verbosity", "Medium"), ("ClearSpeak_Exponents", "Ordinal")], expr, "x au deuxième")?;
    test_prefs("fr", "ClearSpeak", vec![("Verbosity", "Medium"), ("ClearSpeak_Exponents", "OrdinalPower")], expr, "x à la deuxième puissance")?;
    test_prefs("fr", "ClearSpeak", vec![("Verbosity", "Medium"), ("ClearSpeak_Exponents", "AfterPower")], expr, "x élevé à la puissance 2")?;

    return Ok(());

}

#[test]
fn cubed() -> Result<()> {
  let expr = "<math>
                  <msup> <mi>x</mi> <mn>3</mn> </msup>
              </math>";
  test_prefs("fr", "ClearSpeak", vec![("Verbosity", "Medium"), ("ClearSpeak_Exponents", "Auto")], expr, "x au cube")?;
  test_prefs("fr", "ClearSpeak", vec![("Verbosity", "Medium"), ("ClearSpeak_Exponents", "Ordinal")], expr, "x au troisième")?;
  test_prefs("fr", "ClearSpeak", vec![("Verbosity", "Medium"), ("ClearSpeak_Exponents", "OrdinalPower")], expr, "x à la troisième puissance")?;
  test_prefs("fr", "ClearSpeak", vec![("Verbosity", "Medium"), ("ClearSpeak_Exponents", "AfterPower")], expr, "x élevé à la puissance 3")?;
  return Ok(());

}

#[test]
fn ordinal_power() -> Result<()> {
  let expr = "<math>
                  <msup> <mn>3</mn> <mn>5</mn> </msup>
              </math>";
  test_prefs("fr", "ClearSpeak", vec![("Verbosity", "Medium"), ("ClearSpeak_Exponents", "Auto")], expr, "3 à la cinquième puissance")?;
  test_prefs("fr", "ClearSpeak", vec![("Verbosity", "Medium"), ("ClearSpeak_Exponents", "Ordinal")], expr, "3 au cinquième")?;
  test_prefs("fr", "ClearSpeak", vec![("Verbosity", "Medium"), ("ClearSpeak_Exponents", "OrdinalPower")], expr, "3 à la cinquième puissance")?;
  test_prefs("fr", "ClearSpeak", vec![("Verbosity", "Medium"), ("ClearSpeak_Exponents", "AfterPower")], expr, "3 élevé à la puissance 5")?;
  return Ok(());

}


#[test]
fn zero_power() -> Result<()> {
  let expr = "<math>
                    <msup> <mn>3</mn> <mn>0</mn> </msup>
                </math>";
  test_prefs("fr", "ClearSpeak", vec![("Verbosity", "Medium"), ("ClearSpeak_Exponents", "Auto")], expr, "3 à la puissance 0")?;
  test_prefs("fr", "ClearSpeak", vec![("Verbosity", "Medium"), ("ClearSpeak_Exponents", "Ordinal")], expr, "3 à la 0")?;
  test_prefs("fr", "ClearSpeak", vec![("Verbosity", "Medium"), ("ClearSpeak_Exponents", "OrdinalPower")], expr, "3 à la puissance 0")?;
  test_prefs("fr", "ClearSpeak", vec![("Verbosity", "Medium"), ("ClearSpeak_Exponents", "AfterPower")], expr, "3 élevé à la puissance 0")?;
  return Ok(());

}

#[test]
fn simple_mi_power() -> Result<()> {
  let expr = "<math>
                    <msup> <mn>4</mn> <mi>x</mi> </msup>
                </math>";
  test_prefs("fr", "ClearSpeak", vec![("Verbosity", "Medium"), ("ClearSpeak_Exponents", "Auto")], expr, "4 à la x-ième puissance")?;
  test_prefs("fr", "ClearSpeak", vec![("Verbosity", "Medium"), ("ClearSpeak_Exponents", "Ordinal")], expr, "4 à la x-ième")?;
  test_prefs("fr", "ClearSpeak", vec![("Verbosity", "Medium"), ("ClearSpeak_Exponents", "OrdinalPower")], expr, "4 à la x-ième puissance")?;
  test_prefs("fr", "ClearSpeak", vec![("Verbosity", "Medium"), ("ClearSpeak_Exponents", "AfterPower")], expr, "4 élevé à la puissance x")?;
  return Ok(());

}

#[test]
fn decimal_power() -> Result<()> {
  let expr = "<math>
                  <msup> <mn>3</mn> <mn>5,0</mn> </msup>
              </math>";
  test_prefs("fr", "ClearSpeak", vec![("Verbosity", "Medium"), ("ClearSpeak_Exponents", "Auto")], expr, "3 à la puissance 5,0")?;
  test_prefs("fr", "ClearSpeak", vec![("Verbosity", "Medium"), ("ClearSpeak_Exponents", "Ordinal")], expr, "3 à la puissance 5,0")?;
  test_prefs("fr", "ClearSpeak", vec![("Verbosity", "Medium"), ("ClearSpeak_Exponents", "OrdinalPower")], expr, "3 à la puissance 5,0")?;
  test_prefs("fr", "ClearSpeak", vec![("Verbosity", "Medium"), ("ClearSpeak_Exponents", "AfterPower")], expr, "3 élevé à la puissance 5,0")?;
  return Ok(());

}

#[test]
fn non_simple_power() -> Result<()> {
  let expr = "<math>
        <msup> <mn>3</mn>  <mrow> <mi>y</mi><mo>+</mo><mn>2</mn></mrow>  </msup>
    </math>";
  test_prefs("fr", "ClearSpeak", vec![("Verbosity", "Medium"), ("ClearSpeak_Exponents", "Auto")], expr, "3 élevé à la puissance y plus 2")?;
  test_prefs("fr", "ClearSpeak", vec![("Verbosity", "Medium"), ("ClearSpeak_Exponents", "Ordinal")], expr, "3 élevé à la puissance y plus 2")?;
  test_prefs("fr", "ClearSpeak", vec![("Verbosity", "Medium"), ("ClearSpeak_Exponents", "OrdinalPower")], expr, "3 élevé à la puissance y plus 2")?;
  test_prefs("fr", "ClearSpeak", vec![("Verbosity", "Medium"), ("ClearSpeak_Exponents", "AfterPower")], expr, "3 élevé à la puissance y plus 2")?;
  return Ok(());

}

#[test]
fn negative_power() -> Result<()> {
  let expr = "<math>
                  <msup> <mn>3</mn> <mrow> <mo>-</mo> <mn>2</mn> </mrow> </msup>
              </math>";
  test_prefs("fr", "ClearSpeak", vec![("Verbosity", "Medium"), ("ClearSpeak_Exponents", "Auto")], expr, "3 à la puissance moins 2")?;
  test_prefs("fr", "ClearSpeak", vec![("Verbosity", "Medium"), ("ClearSpeak_Exponents", "Ordinal")], expr, "3 à la moins 2")?;
  test_prefs("fr", "ClearSpeak", vec![("Verbosity", "Medium"), ("ClearSpeak_Exponents", "OrdinalPower")], expr, "3 à la puissance moins 2")?;
  test_prefs("fr", "ClearSpeak", vec![("Verbosity", "Medium"), ("ClearSpeak_Exponents", "AfterPower")], expr, "3 élevé à la puissance moins 2")?;
  return Ok(());

}

#[test]
fn simple_fraction_power() -> Result<()> {
  let expr = "<math>
                    <msup>
                        <mi>x</mi> 
                        <mfrac><mn>1</mn><mn>3</mn></mfrac>
                    </msup>
                </math>";
  test("fr", "ClearSpeak", expr, "x élevé à la puissance 1 tiers")?;
  return Ok(());

}

#[test]
fn nested_squared_power_with_coef() -> Result<()> {
  let expr = "<math>
      <mrow>
      <msup>
        <mn>3</mn>
        <mrow>
        <mn>2</mn>
        <msup>
          <mi>x</mi>
          <mn>2</mn>
        </msup>
        </mrow>
      </msup>
      </mrow>
      </math>";
  test_prefs("fr", "ClearSpeak", vec![("Verbosity", "Medium"), ("ClearSpeak_Exponents", "Auto")], expr, "3 élevé à la puissance 2 x au carré")?;
  test_prefs("fr", "ClearSpeak", vec![("Verbosity", "Medium"), ("ClearSpeak_Exponents", "Ordinal")], expr, "3 élevé à l'exposant, 2 x au deuxième, fin de l'exposant")?;
  test_prefs("fr", "ClearSpeak", vec![("Verbosity", "Medium"), ("ClearSpeak_Exponents", "OrdinalPower")], expr, "3 élevé à l'exposant, 2 x à la deuxième puissance, fin de l'exposant")?;
  test_prefs("fr", "ClearSpeak", vec![("Verbosity", "Medium"), ("ClearSpeak_Exponents", "AfterPower")], expr, "3 élevé à l'exposant, 2 x élevé à la puissance 2; fin de l'exposant")?;

  return Ok(());

}

#[test]
fn nested_squared_power_with_neg_coef() -> Result<()> {
    let expr = "<math>
      <mrow>
      <msup>
        <mn>3</mn>
        <mrow>
        <mo>-</mo>
        <mn>2</mn>
        <msup>
          <mi>x</mi>
          <mn>2</mn>
        </msup>
        </mrow>
      </msup>
      </mrow>
    </math>";
  test("fr", "ClearSpeak", expr, "3 élevé à la puissance moins 2 x au carré")?;
  return Ok(());

}


#[test]
fn nested_cubed_power() -> Result<()> {
    let expr = "<math>
      <msup>
      <mi>y</mi> 
      <msup>
          <mfrac><mn>4</mn><mn>5</mn></mfrac>
          <mn>3</mn>
      </msup>
    </msup>
  </math>";
  test("fr", "ClearSpeak", expr, "y élevé à la puissance 4 cinquièmes au cube")?;
  return Ok(());

}

#[test]
fn nested_cubed_power_with_neg_base() -> Result<()> {
    let expr = "<math>
      <msup>
      <mi>y</mi> 
        <mrow>
            <mo>-</mo>
            <msup>
                <mfrac><mn>4</mn><mn>5</mn></mfrac>
                <mn>3</mn>
            </msup>
        </mrow>
    </msup>
    </math>";
  test("fr", "ClearSpeak", expr, "y élevé à la puissance moins 4 cinquièmes au cube")?;
  return Ok(());

}

#[test]
fn nested_number_times_squared() -> Result<()> {
  let expr = "<math>
      <mrow>
      <msup>
        <mi>e</mi>
        <mrow>
        <mfrac>
          <mn>1</mn>
          <mn>2</mn>
          </mfrac>
          <msup>
          <mi>x</mi>
          <mn>2</mn>
          </msup>
        </mrow>
      </msup>
      </mrow>
      </math>";
  test_prefs("fr", "ClearSpeak", vec![("Verbosity", "Medium"), ("ClearSpeak_Exponents", "Auto")], expr, "e élevé à la puissance 1 demi x au carré")?;
  return Ok(());

}

#[test]
fn nested_negative_number_times_squared() -> Result<()> {
  let expr = "<math>
      <mrow>
      <msup>
        <mi>e</mi>
        <mrow>
        <mo>&#x2212;</mo><mfrac>
          <mn>1</mn>
          <mn>2</mn>
        </mfrac>
        <msup>
          <mi>x</mi>
          <mn>2</mn>
        </msup>
        </mrow>
      </msup>
      </mrow>
      </math>";
  test_prefs("fr", "ClearSpeak", vec![("Verbosity", "Medium"), ("ClearSpeak_Exponents", "Auto")], expr, "e élevé à la puissance moins 1 demi x au carré")?;
  test_prefs("fr", "ClearSpeak", vec![("Verbosity", "Medium"), ("ClearSpeak_Exponents", "Ordinal")], expr, "e élevé à l'exposant, moins 1 demi x au deuxième, fin de l'exposant")?;
  test_prefs("fr", "ClearSpeak", vec![("Verbosity", "Medium"), ("ClearSpeak_Exponents", "OrdinalPower")], expr, "e élevé à l'exposant, moins 1 demi x à la deuxième puissance, fin de l'exposant")?;
  test_prefs("fr", "ClearSpeak", vec![("Verbosity", "Medium"), ("ClearSpeak_Exponents", "AfterPower")], expr, "e élevé à l'exposant, moins 1 demi x élevé à la puissance 2; fin de l'exposant")?;
  return Ok(());

}

#[test]
fn nested_expr_to_tenth() -> Result<()> {
  let expr = "<math>
      <mrow>
      <msup>
        <mn>3</mn>
        <mrow>
        <msup>
          <mn>3</mn>
          <mrow>
          <mn>10</mn></mrow>
        </msup>
        </mrow>
      </msup>
      </mrow>
      </math>";
  test_prefs("fr", "ClearSpeak", vec![("Verbosity", "Medium"), ("ClearSpeak_Exponents", "Auto")], expr, "3 élevé à l'exposant, 3 à la dixième puissance, fin de l'exposant")?;
  test_prefs("fr", "ClearSpeak", vec![("Verbosity", "Medium"), ("ClearSpeak_Exponents", "Ordinal")], expr, "3 élevé à l'exposant, 3 au dixième, fin de l'exposant")?;
  test_prefs("fr", "ClearSpeak", vec![("Verbosity", "Medium"), ("ClearSpeak_Exponents", "OrdinalPower")], expr, "3 élevé à l'exposant, 3 à la dixième puissance, fin de l'exposant")?;
  test_prefs("fr", "ClearSpeak", vec![("Verbosity", "Medium"), ("ClearSpeak_Exponents", "AfterPower")], expr, "3 élevé à l'exposant, 3 élevé à la puissance 10; fin de l'exposant")?;

  return Ok(());

}

#[test]
fn nested_non_simple_squared_exp() -> Result<()> {
  let expr = "<math>
      <mrow>
      <msup>
        <mn>3</mn>
        <mrow>
        <msup>
          <mrow>
          <mrow><mo>(</mo>
            <mrow>
            <mi>x</mi><mo>+</mo><mn>1</mn></mrow>
          <mo>)</mo></mrow></mrow>
          <mn>2</mn>
        </msup>
        </mrow>
      </msup>
      </mrow>
      </math>";
  test_prefs("fr", "ClearSpeak", vec![("Verbosity", "Medium"), ("ClearSpeak_Exponents", "Auto")], expr, "3 élevé à l'exposant, parenthèse gauche, x plus 1, parenthèse droite au carré, fin de l'exposant")?;
  test_prefs("fr", "ClearSpeak", vec![("Verbosity", "Medium"), ("ClearSpeak_Exponents", "Ordinal")], expr, "3 élevé à l'exposant, parenthèse gauche, x plus 1, parenthèse droite au deuxième, fin de l'exposant")?;
  test_prefs("fr", "ClearSpeak", vec![("Verbosity", "Medium"), ("ClearSpeak_Exponents", "OrdinalPower")], expr, "3 élevé à l'exposant, parenthèse gauche, x plus 1, parenthèse droite à la deuxième puissance, fin de l'exposant")?;
  test_prefs("fr", "ClearSpeak", vec![("Verbosity", "Medium"), ("ClearSpeak_Exponents", "AfterPower")], expr, "3 élevé à l'exposant, parenthèse gauche, x plus 1, parenthèse droite élevé à la puissance 2; fin de l'exposant")?;
  return Ok(());

}

#[test]
fn nested_default_power() -> Result<()> {
  let expr = "<math>
    <msup>
    <mi>t</mi> 
    <msup>
        <mfrac><mn>4</mn><mn>5</mn></mfrac>
        <mi>n</mi>
    </msup>
  </msup>
</math>";
  test("fr", "ClearSpeak", expr, "t élevé à l'exposant, 4 cinquièmes à la n-ième puissance, fin de l'exposant")?;
  return Ok(());

}

#[test]
fn nested_complex_power() -> Result<()> {
  let expr = "<math>
      <mrow>
      <msup>
        <mi>e</mi>
        <mrow>
        <mo>&#x2212;</mo><mfrac>
          <mn>1</mn>
          <mn>2</mn>
        </mfrac>
        <msup>
          <mrow>
          <mrow><mo>(</mo>
            <mrow>
            <mfrac>
              <mrow>
              <mi>x</mi><mo>&#x2212;</mo><mi>&#x03BC;</mi></mrow>
              <mi>&#x03C3;</mi>
            </mfrac>
            </mrow>
          <mo>)</mo></mrow></mrow>
          <mn>2</mn>
        </msup>
        </mrow>
      </msup>
      </mrow>
      </math>";
  test_prefs("fr", "ClearSpeak", vec![("Verbosity", "Medium"), ("ClearSpeak_Exponents", "Auto")], expr,
       "e élevé à l'exposant, moins 1 demi fois; parenthèse gauche; la fraction avec numérateur; x moins mû; et dénominateur sigma; parenthèse droite au carré, fin de l'exposant")?;
  test_prefs("fr", "ClearSpeak", vec![("Verbosity", "Medium"), ("ClearSpeak_Exponents", "Ordinal")], expr,
       "e élevé à l'exposant, moins 1 demi fois; parenthèse gauche; la fraction avec numérateur; x moins mû; et dénominateur sigma; parenthèse droite au deuxième, fin de l'exposant")?;
  test_prefs("fr", "ClearSpeak", vec![("Verbosity", "Medium"), ("ClearSpeak_Exponents", "OrdinalPower")], expr,
       "e élevé à l'exposant, moins 1 demi fois; parenthèse gauche; la fraction avec numérateur; x moins mû; et dénominateur sigma; parenthèse droite à la deuxième puissance, fin de l'exposant")?;
  test_prefs("fr", "ClearSpeak", vec![("Verbosity", "Medium"), ("ClearSpeak_Exponents", "AfterPower")], expr,
       "e élevé à l'exposant, moins 1 demi fois; parenthèse gauche; la fraction avec numérateur; x moins mû; et dénominateur sigma; parenthèse droite élevé à la puissance 2; fin de l'exposant")?;
       return Ok(());

}

#[test]
fn default_power() -> Result<()> {
    let expr = "<math>
      <msup>
      <mi>t</mi> 
      <mfrac>
          <mrow><mi>b</mi><mo>+</mo><mn>1</mn></mrow>
          <mn>3</mn>
      </mfrac>
    </msup>
  </math>";
  test("fr", "ClearSpeak", expr, "t élevé à la puissance la fraction avec numérateur; b plus 1; et dénominateur 3")?;
  return Ok(());

}
