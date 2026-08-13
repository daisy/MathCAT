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
    test_prefs("pl", "ClearSpeak", vec![("Verbosity", "Medium"), ("ClearSpeak_Exponents", "Auto")], expr, "x do kwadratu")?;
    test_prefs("pl", "ClearSpeak", vec![("Verbosity", "Medium"), ("ClearSpeak_Exponents", "Ordinal")], expr, "x do potęgi drugiej")?;
    test_prefs("pl", "ClearSpeak", vec![("Verbosity", "Medium"), ("ClearSpeak_Exponents", "OrdinalPower")], expr, "x do potęgi drugiej")?;
    test_prefs("pl", "ClearSpeak", vec![("Verbosity", "Medium"), ("ClearSpeak_Exponents", "AfterPower")], expr, "x do potęgi 2")?;

    return Ok(());

}

#[test]
fn cubed() -> Result<()> {
  let expr = "<math>
                  <msup> <mi>x</mi> <mn>3</mn> </msup>
              </math>";
  test_prefs("pl", "ClearSpeak", vec![("Verbosity", "Medium"), ("ClearSpeak_Exponents", "Auto")], expr, "x do sześcianu")?;
  test_prefs("pl", "ClearSpeak", vec![("Verbosity", "Medium"), ("ClearSpeak_Exponents", "Ordinal")], expr, "x do potęgi trzeciej")?;
  test_prefs("pl", "ClearSpeak", vec![("Verbosity", "Medium"), ("ClearSpeak_Exponents", "OrdinalPower")], expr, "x do potęgi trzeciej")?;
  test_prefs("pl", "ClearSpeak", vec![("Verbosity", "Medium"), ("ClearSpeak_Exponents", "AfterPower")], expr, "x do potęgi 3")?;
  return Ok(());

}

#[test]
fn ordinal_power() -> Result<()> {
  let expr = "<math>
                  <msup> <mn>3</mn> <mn>5</mn> </msup>
              </math>";
  test_prefs("pl", "ClearSpeak", vec![("Verbosity", "Medium"), ("ClearSpeak_Exponents", "Auto")], expr, "3 do potęgi piątej")?;
  test_prefs("pl", "ClearSpeak", vec![("Verbosity", "Medium"), ("ClearSpeak_Exponents", "Ordinal")], expr, "3 do potęgi piątej")?;
  test_prefs("pl", "ClearSpeak", vec![("Verbosity", "Medium"), ("ClearSpeak_Exponents", "OrdinalPower")], expr, "3 do potęgi piątej")?;
  test_prefs("pl", "ClearSpeak", vec![("Verbosity", "Medium"), ("ClearSpeak_Exponents", "AfterPower")], expr, "3 do potęgi 5")?;
  return Ok(());

}


#[test]
fn zero_power() -> Result<()> {
  let expr = "<math>
                    <msup> <mn>3</mn> <mn>0</mn> </msup>
                </math>";
  test_prefs("pl", "ClearSpeak", vec![("Verbosity", "Medium"), ("ClearSpeak_Exponents", "Auto")], expr, "3 do potęgi 0")?;
  test_prefs("pl", "ClearSpeak", vec![("Verbosity", "Medium"), ("ClearSpeak_Exponents", "Ordinal")], expr, "3 do potęgi 0")?;
  test_prefs("pl", "ClearSpeak", vec![("Verbosity", "Medium"), ("ClearSpeak_Exponents", "OrdinalPower")], expr, "3 do potęgi 0")?;
  test_prefs("pl", "ClearSpeak", vec![("Verbosity", "Medium"), ("ClearSpeak_Exponents", "AfterPower")], expr, "3 do potęgi 0")?;
  return Ok(());

}

#[test]
fn simple_mi_power() -> Result<()> {
  let expr = "<math>
                    <msup> <mn>4</mn> <mi>x</mi> </msup>
                </math>";
  test_prefs("pl", "ClearSpeak", vec![("Verbosity", "Medium"), ("ClearSpeak_Exponents", "Auto")], expr, "4 do potęgi x")?;
  test_prefs("pl", "ClearSpeak", vec![("Verbosity", "Medium"), ("ClearSpeak_Exponents", "Ordinal")], expr, "4 do potęgi x")?;
  test_prefs("pl", "ClearSpeak", vec![("Verbosity", "Medium"), ("ClearSpeak_Exponents", "OrdinalPower")], expr, "4 do potęgi x")?;
  test_prefs("pl", "ClearSpeak", vec![("Verbosity", "Medium"), ("ClearSpeak_Exponents", "AfterPower")], expr, "4 do potęgi x")?;
  return Ok(());

}

#[test]
fn decimal_power() -> Result<()> {
  let expr = "<math>
                  <msup> <mn>3</mn> <mn>5.0</mn> </msup>
              </math>";
  test_prefs("pl", "ClearSpeak", vec![("Verbosity", "Medium"), ("ClearSpeak_Exponents", "Auto")], expr, "3 do potęgi 50")?;
  test_prefs("pl", "ClearSpeak", vec![("Verbosity", "Medium"), ("ClearSpeak_Exponents", "Ordinal")], expr, "3 do potęgi 50")?;
  test_prefs("pl", "ClearSpeak", vec![("Verbosity", "Medium"), ("ClearSpeak_Exponents", "OrdinalPower")], expr, "3 do potęgi 50")?;
  test_prefs("pl", "ClearSpeak", vec![("Verbosity", "Medium"), ("ClearSpeak_Exponents", "AfterPower")], expr, "3 do potęgi 50")?;
  return Ok(());

}

#[test]
fn non_simple_power() -> Result<()> {
  let expr = "<math>
        <msup> <mn>3</mn>  <mrow> <mi>y</mi><mo>+</mo><mn>2</mn></mrow>  </msup>
    </math>";
  test_prefs("pl", "ClearSpeak", vec![("Verbosity", "Medium"), ("ClearSpeak_Exponents", "Auto")], expr, "3 do potęgi y plus 2")?;
  test_prefs("pl", "ClearSpeak", vec![("Verbosity", "Medium"), ("ClearSpeak_Exponents", "Ordinal")], expr, "3 do potęgi y plus 2")?;
  test_prefs("pl", "ClearSpeak", vec![("Verbosity", "Medium"), ("ClearSpeak_Exponents", "OrdinalPower")], expr, "3 do potęgi y plus 2")?;
  test_prefs("pl", "ClearSpeak", vec![("Verbosity", "Medium"), ("ClearSpeak_Exponents", "AfterPower")], expr, "3 do potęgi y plus 2")?;
  return Ok(());

}

#[test]
fn negative_power() -> Result<()> {
  let expr = "<math>
                  <msup> <mn>3</mn> <mrow> <mo>-</mo> <mn>2</mn> </mrow> </msup>
              </math>";
  test_prefs("pl", "ClearSpeak", vec![("Verbosity", "Medium"), ("ClearSpeak_Exponents", "Auto")], expr, "3 do potęgi minus 2")?;
  test_prefs("pl", "ClearSpeak", vec![("Verbosity", "Medium"), ("ClearSpeak_Exponents", "Ordinal")], expr, "3 do potęgi minus 2")?;
  test_prefs("pl", "ClearSpeak", vec![("Verbosity", "Medium"), ("ClearSpeak_Exponents", "OrdinalPower")], expr, "3 do potęgi minus 2")?;
  test_prefs("pl", "ClearSpeak", vec![("Verbosity", "Medium"), ("ClearSpeak_Exponents", "AfterPower")], expr, "3 do potęgi minus 2")?;
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
  test("pl", "ClearSpeak", expr, "x do potęgi jedna trzecia")?;
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
  test_prefs("pl", "ClearSpeak", vec![("Verbosity", "Medium"), ("ClearSpeak_Exponents", "Auto")], expr, "3 do potęgi 2 x do kwadratu")?;
  test_prefs("pl", "ClearSpeak", vec![("Verbosity", "Medium"), ("ClearSpeak_Exponents", "Ordinal")], expr, "3 do wykładnika, 2 x do potęgi drugiej, koniec wykładnika")?;
  test_prefs("pl", "ClearSpeak", vec![("Verbosity", "Medium"), ("ClearSpeak_Exponents", "OrdinalPower")], expr, "3 do wykładnika, 2 x do potęgi drugiej, koniec wykładnika")?;
  test_prefs("pl", "ClearSpeak", vec![("Verbosity", "Medium"), ("ClearSpeak_Exponents", "AfterPower")], expr, "3 do wykładnika, 2 x do potęgi 2; koniec wykładnika")?;

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
  test("pl", "ClearSpeak", expr, "3 do potęgi minus 2 x do kwadratu")?;
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
  test("pl", "ClearSpeak", expr, "y do potęgi 4 piąte do sześcianu")?;
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
  test("pl", "ClearSpeak", expr, "y do potęgi minus 4 piąte do sześcianu")?;
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
  test_prefs("pl", "ClearSpeak", vec![("Verbosity", "Medium"), ("ClearSpeak_Exponents", "Auto")], expr, "e do potęgi jedna druga x do kwadratu")?;
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
  test_prefs("pl", "ClearSpeak", vec![("Verbosity", "Medium"), ("ClearSpeak_Exponents", "Auto")], expr, "e do potęgi minus jedna druga x do kwadratu")?;
  test_prefs("pl", "ClearSpeak", vec![("Verbosity", "Medium"), ("ClearSpeak_Exponents", "Ordinal")], expr, "e do wykładnika, minus jedna druga x do potęgi drugiej, koniec wykładnika")?;
  test_prefs("pl", "ClearSpeak", vec![("Verbosity", "Medium"), ("ClearSpeak_Exponents", "OrdinalPower")], expr, "e do wykładnika, minus jedna druga x do potęgi drugiej, koniec wykładnika")?;
  test_prefs("pl", "ClearSpeak", vec![("Verbosity", "Medium"), ("ClearSpeak_Exponents", "AfterPower")], expr, "e do wykładnika, minus jedna druga x do potęgi 2; koniec wykładnika")?;
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
  test_prefs("pl", "ClearSpeak", vec![("Verbosity", "Medium"), ("ClearSpeak_Exponents", "Auto")], expr, "3 do wykładnika, 3 do potęgi dziesiątej, koniec wykładnika")?;
  test_prefs("pl", "ClearSpeak", vec![("Verbosity", "Medium"), ("ClearSpeak_Exponents", "Ordinal")], expr, "3 do wykładnika, 3 do potęgi dziesiątej, koniec wykładnika")?;
  test_prefs("pl", "ClearSpeak", vec![("Verbosity", "Medium"), ("ClearSpeak_Exponents", "OrdinalPower")], expr, "3 do wykładnika, 3 do potęgi dziesiątej, koniec wykładnika")?;
  test_prefs("pl", "ClearSpeak", vec![("Verbosity", "Medium"), ("ClearSpeak_Exponents", "AfterPower")], expr, "3 do wykładnika, 3 do potęgi 10; koniec wykładnika")?;

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
  test_prefs("pl", "ClearSpeak", vec![("Verbosity", "Medium"), ("ClearSpeak_Exponents", "Auto")], expr, "3 do wykładnika, nawias otwierający, x plus 1, nawias zamykający do kwadratu, koniec wykładnika")?;
  test_prefs("pl", "ClearSpeak", vec![("Verbosity", "Medium"), ("ClearSpeak_Exponents", "Ordinal")], expr, "3 do wykładnika, nawias otwierający, x plus 1, nawias zamykający do potęgi drugiej, koniec wykładnika")?;
  test_prefs("pl", "ClearSpeak", vec![("Verbosity", "Medium"), ("ClearSpeak_Exponents", "OrdinalPower")], expr, "3 do wykładnika, nawias otwierający, x plus 1, nawias zamykający do potęgi drugiej, koniec wykładnika")?;
  test_prefs("pl", "ClearSpeak", vec![("Verbosity", "Medium"), ("ClearSpeak_Exponents", "AfterPower")], expr, "3 do wykładnika, nawias otwierający, x plus 1, nawias zamykający do potęgi 2; koniec wykładnika")?;
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
  test("pl", "ClearSpeak", expr, "t do wykładnika, 4 piąte do potęgi n, koniec wykładnika")?;
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
  test_prefs("pl", "ClearSpeak", vec![("Verbosity", "Medium"), ("ClearSpeak_Exponents", "Auto")], expr,
       "e do wykładnika, minus jedna druga razy; nawias otwierający; ułamek z licznikiem; x minus mi; i mianownikiem sigma; nawias zamykający do kwadratu, koniec wykładnika")?;
  test_prefs("pl", "ClearSpeak", vec![("Verbosity", "Medium"), ("ClearSpeak_Exponents", "Ordinal")], expr,
       "e do wykładnika, minus jedna druga razy; nawias otwierający; ułamek z licznikiem; x minus mi; i mianownikiem sigma; nawias zamykający do potęgi drugiej, koniec wykładnika")?;
  test_prefs("pl", "ClearSpeak", vec![("Verbosity", "Medium"), ("ClearSpeak_Exponents", "OrdinalPower")], expr,
       "e do wykładnika, minus jedna druga razy; nawias otwierający; ułamek z licznikiem; x minus mi; i mianownikiem sigma; nawias zamykający do potęgi drugiej, koniec wykładnika")?;
  test_prefs("pl", "ClearSpeak", vec![("Verbosity", "Medium"), ("ClearSpeak_Exponents", "AfterPower")], expr,
       "e do wykładnika, minus jedna druga razy; nawias otwierający; ułamek z licznikiem; x minus mi; i mianownikiem sigma; nawias zamykający do potęgi 2; koniec wykładnika")?;
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
  test("pl", "ClearSpeak", expr, "t do potęgi ułamek z licznikiem; b plus 1; i mianownikiem 3")?;
  return Ok(());

}
