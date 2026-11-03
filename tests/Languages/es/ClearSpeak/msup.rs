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
    test_prefs("es", "ClearSpeak", vec![("Verbosity", "Medium"), ("ClearSpeak_Exponents", "Auto")], expr, "x cuadrado")?;
    test_prefs("es", "ClearSpeak", vec![("Verbosity", "Medium"), ("ClearSpeak_Exponents", "Ordinal")], expr, "x a la segundo")?;
    test_prefs("es", "ClearSpeak", vec![("Verbosity", "Medium"), ("ClearSpeak_Exponents", "OrdinalPower")], expr, "x a la segundo potencia")?;
    test_prefs("es", "ClearSpeak", vec![("Verbosity", "Medium"), ("ClearSpeak_Exponents", "AfterPower")], expr, "x elevado a la potencia 2")?;

    return Ok(());

}

// REVISAR (tercero): «x a la tercero»
// REVISAR (tercero): «x a la tercero potencia»
#[test]
#[ignore = "lectura en español pendiente de revisión"]
fn cubed() -> Result<()> {
  let expr = "<math>
                  <msup> <mi>x</mi> <mn>3</mn> </msup>
              </math>";
  test_prefs("es", "ClearSpeak", vec![("Verbosity", "Medium"), ("ClearSpeak_Exponents", "Auto")], expr, "x al cubo")?;
  test_prefs("es", "ClearSpeak", vec![("Verbosity", "Medium"), ("ClearSpeak_Exponents", "Ordinal")], expr, "x a la tercero")?;
  test_prefs("es", "ClearSpeak", vec![("Verbosity", "Medium"), ("ClearSpeak_Exponents", "OrdinalPower")], expr, "x a la tercero potencia")?;
  test_prefs("es", "ClearSpeak", vec![("Verbosity", "Medium"), ("ClearSpeak_Exponents", "AfterPower")], expr, "x elevado a la potencia 3")?;
  return Ok(());

}

// REVISAR (quinto potencia): «3 a la quinto potencia»
// REVISAR (quinto potencia): «3 a la quinto potencia»
#[test]
#[ignore = "lectura en español pendiente de revisión"]
fn ordinal_power() -> Result<()> {
  let expr = "<math>
                  <msup> <mn>3</mn> <mn>5</mn> </msup>
              </math>";
  test_prefs("es", "ClearSpeak", vec![("Verbosity", "Medium"), ("ClearSpeak_Exponents", "Auto")], expr, "3 a la quinto potencia")?;
  test_prefs("es", "ClearSpeak", vec![("Verbosity", "Medium"), ("ClearSpeak_Exponents", "Ordinal")], expr, "3 a la quinto")?;
  test_prefs("es", "ClearSpeak", vec![("Verbosity", "Medium"), ("ClearSpeak_Exponents", "OrdinalPower")], expr, "3 a la quinto potencia")?;
  test_prefs("es", "ClearSpeak", vec![("Verbosity", "Medium"), ("ClearSpeak_Exponents", "AfterPower")], expr, "3 elevado a la potencia 5")?;
  return Ok(());

}


#[test]
fn zero_power() -> Result<()> {
  let expr = "<math>
                    <msup> <mn>3</mn> <mn>0</mn> </msup>
                </math>";
  test_prefs("es", "ClearSpeak", vec![("Verbosity", "Medium"), ("ClearSpeak_Exponents", "Auto")], expr, "3 a la 0 potencia")?;
  test_prefs("es", "ClearSpeak", vec![("Verbosity", "Medium"), ("ClearSpeak_Exponents", "Ordinal")], expr, "3 a la 0")?;
  test_prefs("es", "ClearSpeak", vec![("Verbosity", "Medium"), ("ClearSpeak_Exponents", "OrdinalPower")], expr, "3 a la 0 potencia")?;
  test_prefs("es", "ClearSpeak", vec![("Verbosity", "Medium"), ("ClearSpeak_Exponents", "AfterPower")], expr, "3 elevado a la potencia 0")?;
  return Ok(());

}

// REVISAR (-th, inglés: th): «4 a la x-th potencia»
// REVISAR (-th, inglés: th): «4 a la x-th»
// REVISAR (-th, inglés: th): «4 a la x-th potencia»
#[test]
#[ignore = "lectura en español pendiente de revisión"]
fn simple_mi_power() -> Result<()> {
  let expr = "<math>
                    <msup> <mn>4</mn> <mi>x</mi> </msup>
                </math>";
  test_prefs("es", "ClearSpeak", vec![("Verbosity", "Medium"), ("ClearSpeak_Exponents", "Auto")], expr, "4 a la x-th potencia")?;
  test_prefs("es", "ClearSpeak", vec![("Verbosity", "Medium"), ("ClearSpeak_Exponents", "Ordinal")], expr, "4 a la x-th")?;
  test_prefs("es", "ClearSpeak", vec![("Verbosity", "Medium"), ("ClearSpeak_Exponents", "OrdinalPower")], expr, "4 a la x-th potencia")?;
  test_prefs("es", "ClearSpeak", vec![("Verbosity", "Medium"), ("ClearSpeak_Exponents", "AfterPower")], expr, "4 elevado a la potencia x")?;
  return Ok(());

}

#[test]
fn decimal_power() -> Result<()> {
  let expr = "<math>
                  <msup> <mn>3</mn> <mn>5.0</mn> </msup>
              </math>";
  test_prefs("es", "ClearSpeak", vec![("Verbosity", "Medium"), ("ClearSpeak_Exponents", "Auto")], expr, "3 elevado a la 50 potencia")?;
  test_prefs("es", "ClearSpeak", vec![("Verbosity", "Medium"), ("ClearSpeak_Exponents", "Ordinal")], expr, "3 elevado a la 50 potencia")?;
  test_prefs("es", "ClearSpeak", vec![("Verbosity", "Medium"), ("ClearSpeak_Exponents", "OrdinalPower")], expr, "3 elevado a la 50 potencia")?;
  test_prefs("es", "ClearSpeak", vec![("Verbosity", "Medium"), ("ClearSpeak_Exponents", "AfterPower")], expr, "3 elevado a la potencia 50")?;
  return Ok(());

}

#[test]
fn non_simple_power() -> Result<()> {
  let expr = "<math>
        <msup> <mn>3</mn>  <mrow> <mi>y</mi><mo>+</mo><mn>2</mn></mrow>  </msup>
    </math>";
  test_prefs("es", "ClearSpeak", vec![("Verbosity", "Medium"), ("ClearSpeak_Exponents", "Auto")], expr, "3 elevado a la y más 2 potencia")?;
  test_prefs("es", "ClearSpeak", vec![("Verbosity", "Medium"), ("ClearSpeak_Exponents", "Ordinal")], expr, "3 elevado a la y más 2 potencia")?;
  test_prefs("es", "ClearSpeak", vec![("Verbosity", "Medium"), ("ClearSpeak_Exponents", "OrdinalPower")], expr, "3 elevado a la y más 2 potencia")?;
  test_prefs("es", "ClearSpeak", vec![("Verbosity", "Medium"), ("ClearSpeak_Exponents", "AfterPower")], expr, "3 elevado a la potencia y más 2")?;
  return Ok(());

}

#[test]
fn negative_power() -> Result<()> {
  let expr = "<math>
                  <msup> <mn>3</mn> <mrow> <mo>-</mo> <mn>2</mn> </mrow> </msup>
              </math>";
  test_prefs("es", "ClearSpeak", vec![("Verbosity", "Medium"), ("ClearSpeak_Exponents", "Auto")], expr, "3 a la menos 2 potencia")?;
  test_prefs("es", "ClearSpeak", vec![("Verbosity", "Medium"), ("ClearSpeak_Exponents", "Ordinal")], expr, "3 a la menos 2")?;
  test_prefs("es", "ClearSpeak", vec![("Verbosity", "Medium"), ("ClearSpeak_Exponents", "OrdinalPower")], expr, "3 a la menos 2 potencia")?;
  test_prefs("es", "ClearSpeak", vec![("Verbosity", "Medium"), ("ClearSpeak_Exponents", "AfterPower")], expr, "3 elevado a la potencia menos 2")?;
  return Ok(());

}

// REVISAR (tercero): «x elevado a la 1 tercero potencia»
#[test]
#[ignore = "lectura en español pendiente de revisión"]
fn simple_fraction_power() -> Result<()> {
  let expr = "<math>
                    <msup>
                        <mi>x</mi> 
                        <mfrac><mn>1</mn><mn>3</mn></mfrac>
                    </msup>
                </math>";
  test("es", "ClearSpeak", expr, "x elevado a la 1 tercero potencia")?;
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
  test_prefs("es", "ClearSpeak", vec![("Verbosity", "Medium"), ("ClearSpeak_Exponents", "Auto")], expr, "3 elevado a la 2 x cuadrado potencia")?;
  test_prefs("es", "ClearSpeak", vec![("Verbosity", "Medium"), ("ClearSpeak_Exponents", "Ordinal")], expr, "3 elevado a la exponente, 2 x a la segundo, fin de la exponente")?;
  test_prefs("es", "ClearSpeak", vec![("Verbosity", "Medium"), ("ClearSpeak_Exponents", "OrdinalPower")], expr, "3 elevado a la exponente, 2 x a la segundo potencia, fin de la exponente")?;
  test_prefs("es", "ClearSpeak", vec![("Verbosity", "Medium"), ("ClearSpeak_Exponents", "AfterPower")], expr, "3 elevado a la exponente, 2 x elevado a la potencia 2; fin de la exponente")?;

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
  test("es", "ClearSpeak", expr, "3 elevado a la menos 2 x cuadrado potencia")?;
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
  test("es", "ClearSpeak", expr, "y elevado a 4 quintos al cubo potencia")?;
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
  test("es", "ClearSpeak", expr, "y elevado a la menos 4 quintos al cubo potencia")?;
  return Ok(());

}

// REVISAR (mitad): «e elevado a la 1 mitad x cuadrado potencia»
#[test]
#[ignore = "lectura en español pendiente de revisión"]
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
  test_prefs("es", "ClearSpeak", vec![("Verbosity", "Medium"), ("ClearSpeak_Exponents", "Auto")], expr, "e elevado a la 1 mitad x cuadrado potencia")?;
  return Ok(());

}

// REVISAR (mitad): «e elevado a la menos 1 mitad x cuadrado potencia»
// REVISAR (mitad): «e elevado a la exponente, menos 1 mitad x a la segundo, fin de la exponente»
// REVISAR (mitad): «e elevado a la exponente, menos 1 mitad x a la segundo potencia, fin de la exponente»
// REVISAR (mitad): «e elevado a la exponente, menos 1 mitad x elevado a la potencia 2; fin de la exponente»
#[test]
#[ignore = "lectura en español pendiente de revisión"]
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
  test_prefs("es", "ClearSpeak", vec![("Verbosity", "Medium"), ("ClearSpeak_Exponents", "Auto")], expr, "e elevado a la menos 1 mitad x cuadrado potencia")?;
  test_prefs("es", "ClearSpeak", vec![("Verbosity", "Medium"), ("ClearSpeak_Exponents", "Ordinal")], expr, "e elevado a la exponente, menos 1 mitad x a la segundo, fin de la exponente")?;
  test_prefs("es", "ClearSpeak", vec![("Verbosity", "Medium"), ("ClearSpeak_Exponents", "OrdinalPower")], expr, "e elevado a la exponente, menos 1 mitad x a la segundo potencia, fin de la exponente")?;
  test_prefs("es", "ClearSpeak", vec![("Verbosity", "Medium"), ("ClearSpeak_Exponents", "AfterPower")], expr, "e elevado a la exponente, menos 1 mitad x elevado a la potencia 2; fin de la exponente")?;
  return Ok(());

}

// REVISAR (décimo potencia): «3 elevado a la exponente, 3 a la décimo potencia, fin de la exponente»
// REVISAR (décimo potencia): «3 elevado a la exponente, 3 a la décimo potencia, fin de la exponente»
#[test]
#[ignore = "lectura en español pendiente de revisión"]
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
  test_prefs("es", "ClearSpeak", vec![("Verbosity", "Medium"), ("ClearSpeak_Exponents", "Auto")], expr, "3 elevado a la exponente, 3 a la décimo potencia, fin de la exponente")?;
  test_prefs("es", "ClearSpeak", vec![("Verbosity", "Medium"), ("ClearSpeak_Exponents", "Ordinal")], expr, "3 elevado a la exponente, 3 a la décimo, fin de la exponente")?;
  test_prefs("es", "ClearSpeak", vec![("Verbosity", "Medium"), ("ClearSpeak_Exponents", "OrdinalPower")], expr, "3 elevado a la exponente, 3 a la décimo potencia, fin de la exponente")?;
  test_prefs("es", "ClearSpeak", vec![("Verbosity", "Medium"), ("ClearSpeak_Exponents", "AfterPower")], expr, "3 elevado a la exponente, 3 elevado a la potencia 10; fin de la exponente")?;

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
  test_prefs("es", "ClearSpeak", vec![("Verbosity", "Medium"), ("ClearSpeak_Exponents", "Auto")], expr, "3 elevado a la exponente, se abren paréntesis, x más 1, se cierran paréntesis cuadrado, fin de la exponente")?;
  test_prefs("es", "ClearSpeak", vec![("Verbosity", "Medium"), ("ClearSpeak_Exponents", "Ordinal")], expr, "3 elevado a la exponente, se abren paréntesis, x más 1, se cierran paréntesis a la segundo, fin de la exponente")?;
  test_prefs("es", "ClearSpeak", vec![("Verbosity", "Medium"), ("ClearSpeak_Exponents", "OrdinalPower")], expr, "3 elevado a la exponente, se abren paréntesis, x más 1, se cierran paréntesis a la segundo potencia, fin de la exponente")?;
  test_prefs("es", "ClearSpeak", vec![("Verbosity", "Medium"), ("ClearSpeak_Exponents", "AfterPower")], expr, "3 elevado a la exponente, se abren paréntesis, x más 1, se cierran paréntesis elevado a la potencia 2; fin de la exponente")?;
  return Ok(());

}

// REVISAR (-th, inglés: th): «t elevado a la exponente, 4 quintos a la n-th potencia, fin de la exponente»
#[test]
#[ignore = "lectura en español pendiente de revisión"]
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
  test("es", "ClearSpeak", expr, "t elevado a la exponente, 4 quintos a la n-th potencia, fin de la exponente")?;
  return Ok(());

}

// REVISAR (mitad): «e elevado a la exponente, menos 1 mitad multiplicado por; se abren paréntesis; la fracción con numerador; x menos my; y denominador sigma; se cierran paréntesis cuadrado, fin de la exponente»
// REVISAR (mitad): «e elevado a la exponente, menos 1 mitad multiplicado por; se abren paréntesis; la fracción con numerador; x menos my; y denominador sigma; se cierran paréntesis a la segundo, fin de la exponente»
// REVISAR (mitad): «e elevado a la exponente, menos 1 mitad multiplicado por; se abren paréntesis; la fracción con numerador; x menos my; y denominador sigma; se cierran paréntesis a la segundo potencia, fin de la exponente»
// REVISAR (mitad): «e elevado a la exponente, menos 1 mitad multiplicado por; se abren paréntesis; la fracción con numerador; x menos my; y denominador sigma; se cierran paréntesis elevado a la potencia 2; fin de la exponente»
#[test]
#[ignore = "lectura en español pendiente de revisión"]
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
  test_prefs("es", "ClearSpeak", vec![("Verbosity", "Medium"), ("ClearSpeak_Exponents", "Auto")], expr,
       "e elevado a la exponente, menos 1 mitad multiplicado por; se abren paréntesis; la fracción con numerador; x menos my; y denominador sigma; se cierran paréntesis cuadrado, fin de la exponente")?;
  test_prefs("es", "ClearSpeak", vec![("Verbosity", "Medium"), ("ClearSpeak_Exponents", "Ordinal")], expr,
       "e elevado a la exponente, menos 1 mitad multiplicado por; se abren paréntesis; la fracción con numerador; x menos my; y denominador sigma; se cierran paréntesis a la segundo, fin de la exponente")?;
  test_prefs("es", "ClearSpeak", vec![("Verbosity", "Medium"), ("ClearSpeak_Exponents", "OrdinalPower")], expr,
       "e elevado a la exponente, menos 1 mitad multiplicado por; se abren paréntesis; la fracción con numerador; x menos my; y denominador sigma; se cierran paréntesis a la segundo potencia, fin de la exponente")?;
  test_prefs("es", "ClearSpeak", vec![("Verbosity", "Medium"), ("ClearSpeak_Exponents", "AfterPower")], expr,
       "e elevado a la exponente, menos 1 mitad multiplicado por; se abren paréntesis; la fracción con numerador; x menos my; y denominador sigma; se cierran paréntesis elevado a la potencia 2; fin de la exponente")?;
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
  test("es", "ClearSpeak", expr, "t elevado a la fracción con numerador; b más 1; y denominador 3; potencia")?;
  return Ok(());

}
