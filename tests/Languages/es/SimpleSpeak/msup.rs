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
    test("es", "SimpleSpeak", expr, "x al cuadrado")?;
    return Ok(());

}

// REVISAR (x cúbica): «x cúbica»
#[test]
#[ignore = "lectura en español pendiente de revisión"]
fn cubed() -> Result<()> {
    let expr = "<math>
                    <msup> <mi>x</mi> <mn>3</mn> </msup>
                </math>";
    test("es", "SimpleSpeak", expr, "x cúbica")?;
    return Ok(());

}

#[test]
    fn ordinal_power() -> Result<()> {
        let expr = "<math>
                        <msup> <mi>x</mi> <mn>4</mn> </msup>
                    </math>";
        test("es", "SimpleSpeak", expr, "x a la cuarto")?;
        return Ok(());

    }

// REVISAR (-th, inglés: th): «x a la n-th»
#[test]
#[ignore = "lectura en español pendiente de revisión"]
fn simple_mi_power() -> Result<()> {
    let expr = "<math>
                    <msup> <mi>x</mi> <mi>n</mi> </msup>
                </math>";
  test("es", "SimpleSpeak", expr, "x a la n-th")?;
  return Ok(());

}

#[test]
fn zero_power() -> Result<()> {
    let expr = "<math>
                    <msup> <mi>x</mi> <mn>0</mn> </msup>
                </math>";
    test("es", "SimpleSpeak", expr, "x a la 0")?;
    return Ok(());

}


#[test]
fn decimal_power() -> Result<()> {
    let expr = "<math>
                    <msup> <mi>x</mi> <mn>2.0</mn> </msup>
                </math>";
    test("es", "SimpleSpeak", expr, "x a la 20")?;
    return Ok(());

}

#[test]
fn non_simple_power() -> Result<()> {
    let expr = "<math>
      <mrow>
      <msup>
        <mn>3</mn>
        <mrow>
        <mi>y</mi><mo>+</mo><mn>2</mn></mrow>
      </msup>
      </mrow>
                </math>";
    test("es", "SimpleSpeak", expr, "3 elevado a la y más 2 potencia")?;
    return Ok(());

}

#[test]
fn negative_power() -> Result<()> {
    let expr = "<math>
                    <msup>
                        <mi>x</mi>
                        <mrow> <mo>-</mo> <mn>2</mn> </mrow>
                    </msup>
                </math>";
    test("es", "SimpleSpeak", expr, "x a la menos 2")?;
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
  test("es", "SimpleSpeak", expr, "x elevado a la 1 tercero potencia")?;
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
  test("es", "SimpleSpeak", expr, "3 elevado a la 2 x al cuadrado potencia")?;
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
  test("es", "SimpleSpeak", expr, "3 elevado a la menos 2 x al cuadrado potencia")?;
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
  test("es", "SimpleSpeak", expr, "y elevado a la 4 quintos cúbica potencia")?;
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
  test("es", "SimpleSpeak", expr, "y elevado a la menos 4 quintos cúbica potencia")?;
  return Ok(());

}

// REVISAR (mitad): «e elevado a la 1 mitad x al cuadrado potencia»
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
  test("es", "SimpleSpeak", expr, "e elevado a la 1 mitad x al cuadrado potencia")?;
  return Ok(());

}

// REVISAR (mitad): «e elevado a la menos 1 mitad x al cuadrado potencia»
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
  test("es", "SimpleSpeak", expr, "e elevado a la menos 1 mitad x al cuadrado potencia")?;
  return Ok(());

}

// REVISAR (décimo potencia): «3 elevado a la 3 a la décimo potencia»
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
  test("es", "SimpleSpeak", expr, "3 elevado a la 3 a la décimo potencia")?;
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
  test("es", "SimpleSpeak", expr, "3 elevado a la se abren paréntesis, x más 1, se cierran paréntesis al cuadrado potencia")?;
  return Ok(());

}

// REVISAR (-th, inglés: th): «t elevado a la 4 quintos a la n-th potencia»
#[test]
#[ignore = "lectura en español pendiente de revisión"]
fn nested_simple_power() -> Result<()> {
    let expr = "<math>
      <msup>
      <mi>t</mi> 
      <msup>
          <mfrac><mn>4</mn><mn>5</mn></mfrac>
          <mi>n</mi>
      </msup>
    </msup>
  </math>";
  test("es", "SimpleSpeak", expr, "t elevado a la 4 quintos a la n-th potencia")?;
  return Ok(());

}

#[test]
fn nested_end_exponent_power() -> Result<()> {
    let expr = "<math>
      <msup>
      <mi>t</mi> 
      <msup>
          <mfrac><mn>4</mn><mn>5</mn></mfrac>
          <mrow><mi>n</mi><mo>+</mo><mn>1</mn></mrow>
      </msup>
    </msup>
  </math>";
  test("es", "SimpleSpeak", expr, "t elevado a 4 quintos elevado a la n más 1 potencia, fin de la exponente")?;
  test_prefs("es", "SimpleSpeak", vec![("Impairment", "LearningDisability")], expr,
  "t elevado a 4 quintos elevado a la n más 1 potencia, fin de la exponente")?;
  return Ok(());

}

#[test]
fn nested_end_exponent_neg_power() -> Result<()> {
    let expr = "<math>
      <msup>
      <mi>t</mi> 
      <msup>
          <mfrac><mn>4</mn><mn>5</mn></mfrac>
          <mrow><mo>-</mo><mn>3</mn></mrow>
      </msup>
    </msup>
  </math>";
  test("es", "SimpleSpeak", expr, "t elevado a 4 quintos a la menos 3, fin de la exponente")?;
  return Ok(());

}

// REVISAR (mitad): «e elevado a la menos 1 mitad por; se abren paréntesis; fracción, x menos my, partido por sigma, fin de fracción; se cierran paréntesis al cuadrado potencia»
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
  test("es", "SimpleSpeak", expr, "e elevado a la menos 1 mitad por; se abren paréntesis; fracción, x menos my, partido por sigma, fin de fracción; se cierran paréntesis al cuadrado potencia")?;
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
  test("es", "SimpleSpeak", expr, "t elevado a la fracción, b más 1, partido por 3, fin de fracción; potencia")?;
  return Ok(());

}
