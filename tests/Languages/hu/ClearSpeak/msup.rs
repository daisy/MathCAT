/// Tests for superscripts
///   simple superscripts
///   complex/nested superscripts
use crate::common::*;
use anyhow::Result;

// AI generated
#[test]
fn squared() -> Result<()> {
    let expr = "<math>
                    <msup> <mi>x</mi> <mn>2</mn> </msup>
                </math>";
    test_prefs("hu", "ClearSpeak", vec![("Verbosity", "Medium"), ("ClearSpeak_Exponents", "Auto")], expr, "x a négyzeten")?;
    test_prefs("hu", "ClearSpeak", vec![("Verbosity", "Medium"), ("ClearSpeak_Exponents", "Ordinal")], expr, "x a(z) második hatványon")?;
    test_prefs("hu", "ClearSpeak", vec![("Verbosity", "Medium"), ("ClearSpeak_Exponents", "OrdinalPower")], expr, "x a(z) második hatványon")?;
    test_prefs("hu", "ClearSpeak", vec![("Verbosity", "Medium"), ("ClearSpeak_Exponents", "AfterPower")], expr, "x a következő hatványra emelve: 2")?;

    return Ok(());

}

// AI generated
#[test]
fn cubed() -> Result<()> {
  let expr = "<math>
                  <msup> <mi>x</mi> <mn>3</mn> </msup>
              </math>";
  test_prefs("hu", "ClearSpeak", vec![("Verbosity", "Medium"), ("ClearSpeak_Exponents", "Auto")], expr, "x a köbön")?;
  test_prefs("hu", "ClearSpeak", vec![("Verbosity", "Medium"), ("ClearSpeak_Exponents", "Ordinal")], expr, "x a(z) harmadik hatványon")?;
  test_prefs("hu", "ClearSpeak", vec![("Verbosity", "Medium"), ("ClearSpeak_Exponents", "OrdinalPower")], expr, "x a(z) harmadik hatványon")?;
  test_prefs("hu", "ClearSpeak", vec![("Verbosity", "Medium"), ("ClearSpeak_Exponents", "AfterPower")], expr, "x a következő hatványra emelve: 3")?;
  return Ok(());

}

// AI generated
#[test]
fn ordinal_power() -> Result<()> {
  let expr = "<math>
                  <msup> <mn>3</mn> <mn>5</mn> </msup>
              </math>";
  test_prefs("hu", "ClearSpeak", vec![("Verbosity", "Medium"), ("ClearSpeak_Exponents", "Auto")], expr, "3 a(z) ötödik hatványon")?;
  test_prefs("hu", "ClearSpeak", vec![("Verbosity", "Medium"), ("ClearSpeak_Exponents", "Ordinal")], expr, "3 a(z) ötödik hatványon")?;
  test_prefs("hu", "ClearSpeak", vec![("Verbosity", "Medium"), ("ClearSpeak_Exponents", "OrdinalPower")], expr, "3 a(z) ötödik hatványon")?;
  test_prefs("hu", "ClearSpeak", vec![("Verbosity", "Medium"), ("ClearSpeak_Exponents", "AfterPower")], expr, "3 a következő hatványra emelve: 5")?;
  return Ok(());

}


// AI generated
#[test]
fn zero_power() -> Result<()> {
  let expr = "<math>
                    <msup> <mn>3</mn> <mn>0</mn> </msup>
                </math>";
  test_prefs("hu", "ClearSpeak", vec![("Verbosity", "Medium"), ("ClearSpeak_Exponents", "Auto")], expr, "3 a(z) nulladik hatványon")?;
  test_prefs("hu", "ClearSpeak", vec![("Verbosity", "Medium"), ("ClearSpeak_Exponents", "Ordinal")], expr, "3 a(z) nulladik hatványon")?;
  test_prefs("hu", "ClearSpeak", vec![("Verbosity", "Medium"), ("ClearSpeak_Exponents", "OrdinalPower")], expr, "3 a(z) nulladik hatványon")?;
  test_prefs("hu", "ClearSpeak", vec![("Verbosity", "Medium"), ("ClearSpeak_Exponents", "AfterPower")], expr, "3 a következő hatványra emelve: 0")?;
  return Ok(());

}

// AI generated
#[test]
fn simple_mi_power() -> Result<()> {
  let expr = "<math>
                    <msup> <mn>4</mn> <mi>x</mi> </msup>
                </math>";
  test_prefs("hu", "ClearSpeak", vec![("Verbosity", "Medium"), ("ClearSpeak_Exponents", "Auto")], expr, "4 a(z) x-edik hatványon")?;
  test_prefs("hu", "ClearSpeak", vec![("Verbosity", "Medium"), ("ClearSpeak_Exponents", "Ordinal")], expr, "4 a(z) x-edik hatványon")?;
  test_prefs("hu", "ClearSpeak", vec![("Verbosity", "Medium"), ("ClearSpeak_Exponents", "OrdinalPower")], expr, "4 a(z) x-edik hatványon")?;
  test_prefs("hu", "ClearSpeak", vec![("Verbosity", "Medium"), ("ClearSpeak_Exponents", "AfterPower")], expr, "4 a következő hatványra emelve: x")?;
  return Ok(());

}

// AI generated
#[test]
fn decimal_power() -> Result<()> {
  let expr = "<math>
                  <msup> <mn>3</mn> <mn>5.0</mn> </msup>
              </math>";
  test_prefs("hu", "ClearSpeak", vec![("Verbosity", "Medium"), ("ClearSpeak_Exponents", "Auto")], expr, "3 a(z) 50 hatványon")?;
  test_prefs("hu", "ClearSpeak", vec![("Verbosity", "Medium"), ("ClearSpeak_Exponents", "Ordinal")], expr, "3 a(z) 50 hatványon")?;
  test_prefs("hu", "ClearSpeak", vec![("Verbosity", "Medium"), ("ClearSpeak_Exponents", "OrdinalPower")], expr, "3 a(z) 50 hatványon")?;
  test_prefs("hu", "ClearSpeak", vec![("Verbosity", "Medium"), ("ClearSpeak_Exponents", "AfterPower")], expr, "3 a következő hatványra emelve: 50")?;
  return Ok(());

}

// AI generated
#[test]
fn non_simple_power() -> Result<()> {
  let expr = "<math>
        <msup> <mn>3</mn>  <mrow> <mi>y</mi><mo>+</mo><mn>2</mn></mrow>  </msup>
    </math>";
  test_prefs("hu", "ClearSpeak", vec![("Verbosity", "Medium"), ("ClearSpeak_Exponents", "Auto")], expr, "3 a(z) y plusz 2 hatványon")?;
  test_prefs("hu", "ClearSpeak", vec![("Verbosity", "Medium"), ("ClearSpeak_Exponents", "Ordinal")], expr, "3 a(z) y plusz 2 hatványon")?;
  test_prefs("hu", "ClearSpeak", vec![("Verbosity", "Medium"), ("ClearSpeak_Exponents", "OrdinalPower")], expr, "3 a(z) y plusz 2 hatványon")?;
  test_prefs("hu", "ClearSpeak", vec![("Verbosity", "Medium"), ("ClearSpeak_Exponents", "AfterPower")], expr, "3 a következő hatványra emelve: y plusz 2")?;
  return Ok(());

}

// AI generated
#[test]
fn negative_power() -> Result<()> {
  let expr = "<math>
                  <msup> <mn>3</mn> <mrow> <mo>-</mo> <mn>2</mn> </mrow> </msup>
              </math>";
  test_prefs("hu", "ClearSpeak", vec![("Verbosity", "Medium"), ("ClearSpeak_Exponents", "Auto")], expr, "3 a negatív 2 hatványon")?;
  test_prefs("hu", "ClearSpeak", vec![("Verbosity", "Medium"), ("ClearSpeak_Exponents", "Ordinal")], expr, "3 a negatív 2 hatványon")?;
  test_prefs("hu", "ClearSpeak", vec![("Verbosity", "Medium"), ("ClearSpeak_Exponents", "OrdinalPower")], expr, "3 a negatív 2 hatványon")?;
  test_prefs("hu", "ClearSpeak", vec![("Verbosity", "Medium"), ("ClearSpeak_Exponents", "AfterPower")], expr, "3 a következő hatványra emelve: negatív 2")?;
  return Ok(());

}

// AI generated
#[test]
fn simple_fraction_power() -> Result<()> {
  let expr = "<math>
                    <msup>
                        <mi>x</mi> 
                        <mfrac><mn>1</mn><mn>3</mn></mfrac>
                    </msup>
                </math>";
  test("hu", "ClearSpeak", expr, "x a(z) 1 harmad hatványon")?;
  return Ok(());

}

// AI generated
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
  test_prefs("hu", "ClearSpeak", vec![("Verbosity", "Medium"), ("ClearSpeak_Exponents", "Auto")], expr, "3 emelve a(z) 2 x a négyzeten hatványra")?;
  test_prefs("hu", "ClearSpeak", vec![("Verbosity", "Medium"), ("ClearSpeak_Exponents", "Ordinal")], expr, "3 emelve a következő kitevőre:, 2 x a(z) második hatványon, kitevő vége")?;
  test_prefs("hu", "ClearSpeak", vec![("Verbosity", "Medium"), ("ClearSpeak_Exponents", "OrdinalPower")], expr, "3 emelve a következő kitevőre:, 2 x a(z) második hatványon, kitevő vége")?;
  test_prefs("hu", "ClearSpeak", vec![("Verbosity", "Medium"), ("ClearSpeak_Exponents", "AfterPower")], expr, "3 a következő kitevőre emelve:, 2, x a következő hatványra emelve: 2; kitevő vége")?;

  return Ok(());

}

// AI generated
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
  test("hu", "ClearSpeak", expr, "3 emelve a negatív 2 x a négyzeten hatványra")?;
  return Ok(());

}


// AI generated
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
  test("hu", "ClearSpeak", expr, "y emelve a(z) 4 ötöd a köbön hatványra")?;
  return Ok(());

}

// AI generated
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
  test("hu", "ClearSpeak", expr, "y a(z) negatív 4 ötöd a köbön hatványon")?;
  return Ok(());

}

// AI generated
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
  test_prefs("hu", "ClearSpeak", vec![("Verbosity", "Medium"), ("ClearSpeak_Exponents", "Auto")], expr, "e emelve a(z) 1 ketted x a négyzeten hatványra")?;
  return Ok(());

}

// AI generated
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
  test_prefs("hu", "ClearSpeak", vec![("Verbosity", "Medium"), ("ClearSpeak_Exponents", "Auto")], expr, "e emelve a negatív 1 ketted x a négyzeten hatványra")?;
  test_prefs("hu", "ClearSpeak", vec![("Verbosity", "Medium"), ("ClearSpeak_Exponents", "Ordinal")], expr, "e emelve a következő kitevőre:, negatív 1 ketted x a(z) második hatványon, kitevő vége")?;
  test_prefs("hu", "ClearSpeak", vec![("Verbosity", "Medium"), ("ClearSpeak_Exponents", "OrdinalPower")], expr, "e emelve a következő kitevőre:, negatív 1 ketted x a(z) második hatványon, kitevő vége")?;
  test_prefs("hu", "ClearSpeak", vec![("Verbosity", "Medium"), ("ClearSpeak_Exponents", "AfterPower")], expr, "e a következő kitevőre emelve:, negatív 1 ketted, x a következő hatványra emelve: 2; kitevő vége")?;
  return Ok(());

}

// AI generated
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
  test_prefs("hu", "ClearSpeak", vec![("Verbosity", "Medium"), ("ClearSpeak_Exponents", "Auto")], expr, "3 emelve a következő kitevőre:, 3 a(z) tizedik hatványon, kitevő vége")?;
  test_prefs("hu", "ClearSpeak", vec![("Verbosity", "Medium"), ("ClearSpeak_Exponents", "Ordinal")], expr, "3 emelve a következő kitevőre:, 3 a(z) tizedik hatványon, kitevő vége")?;
  test_prefs("hu", "ClearSpeak", vec![("Verbosity", "Medium"), ("ClearSpeak_Exponents", "OrdinalPower")], expr, "3 emelve a következő kitevőre:, 3 a(z) tizedik hatványon, kitevő vége")?;
  test_prefs("hu", "ClearSpeak", vec![("Verbosity", "Medium"), ("ClearSpeak_Exponents", "AfterPower")], expr, "3 a következő kitevőre emelve:, 3 a következő hatványra emelve: 10; kitevő vége")?;

  return Ok(());

}

// AI generated
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
  test_prefs("hu", "ClearSpeak", vec![("Verbosity", "Medium"), ("ClearSpeak_Exponents", "Auto")], expr, "3 emelve a következő kitevőre:, nyitott zárójel, x plusz 1, zárt zárójel a négyzeten, kitevő vége")?;
  test_prefs("hu", "ClearSpeak", vec![("Verbosity", "Medium"), ("ClearSpeak_Exponents", "Ordinal")], expr, "3 emelve a következő kitevőre:, nyitott zárójel, x plusz 1, zárt zárójel a(z) második hatványon, kitevő vége")?;
  test_prefs("hu", "ClearSpeak", vec![("Verbosity", "Medium"), ("ClearSpeak_Exponents", "OrdinalPower")], expr, "3 emelve a következő kitevőre:, nyitott zárójel, x plusz 1, zárt zárójel a(z) második hatványon, kitevő vége")?;
  test_prefs("hu", "ClearSpeak", vec![("Verbosity", "Medium"), ("ClearSpeak_Exponents", "AfterPower")], expr, "3 a következő kitevőre emelve:, nyitott zárójel, x plusz 1, zárt zárójel a következő hatványra emelve: 2; kitevő vége")?;
  return Ok(());

}

// AI generated
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
  test("hu", "ClearSpeak", expr, "t emelve a következő kitevőre:, 4 ötöd a(z) n-edik hatványon, kitevő vége")?;
  return Ok(());

}

// AI generated
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
  test_prefs("hu", "ClearSpeak", vec![("Verbosity", "Medium"), ("ClearSpeak_Exponents", "Auto")], expr,
       "e emelve a következő kitevőre:, negatív 1 ketted szorozva; nyitott zárójel; a tört, melynek számlálója; x mínusz mű; nevezője pedig szigma; zárt zárójel a négyzeten, kitevő vége")?;
  test_prefs("hu", "ClearSpeak", vec![("Verbosity", "Medium"), ("ClearSpeak_Exponents", "Ordinal")], expr,
       "e emelve a következő kitevőre:, negatív 1 ketted szorozva; nyitott zárójel; a tört, melynek számlálója; x mínusz mű; nevezője pedig szigma; zárt zárójel a(z) második hatványon, kitevő vége")?;
  test_prefs("hu", "ClearSpeak", vec![("Verbosity", "Medium"), ("ClearSpeak_Exponents", "OrdinalPower")], expr,
       "e emelve a következő kitevőre:, negatív 1 ketted szorozva; nyitott zárójel; a tört, melynek számlálója; x mínusz mű; nevezője pedig szigma; zárt zárójel a(z) második hatványon, kitevő vége")?;
  test_prefs("hu", "ClearSpeak", vec![("Verbosity", "Medium"), ("ClearSpeak_Exponents", "AfterPower")], expr,
       "e a következő kitevőre emelve:, negatív 1 ketted szorozva; nyitott zárójel; a tört, melynek számlálója; x mínusz mű; nevezője pedig szigma; zárt zárójel a következő hatványra emelve: 2; kitevő vége")?;
       return Ok(());

}

// AI generated
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
  test("hu", "ClearSpeak", expr, "t a(z) a tört, melynek számlálója; b plusz 1; nevezője pedig 3; hatványon")?;
  return Ok(());

}
