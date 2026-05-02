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
    test("en", "SimpleSpeak", expr, "x squared")?;
    return Ok(());

}

#[test]
fn cubed() -> Result<()> {
    let expr = "<math>
                    <msup> <mi>x</mi> <mn>3</mn> </msup>
                </math>";
    test("en", "SimpleSpeak", expr, "x cubed")?;
    return Ok(());

}

#[test]
    fn ordinal_power() -> Result<()> {
        let expr = "<math>
                        <msup> <mi>x</mi> <mn>4</mn> </msup>
                    </math>";
        test("en", "SimpleSpeak", expr, "x to the fourth")?;
        return Ok(());

    }

#[test]
fn simple_mi_power() -> Result<()> {
    let expr = "<math>
                    <msup> <mi>x</mi> <mi>n</mi> </msup>
                </math>";
  test("en", "SimpleSpeak", expr, "x to the n-th")?;
  return Ok(());

}

#[test]
fn zero_power() -> Result<()> {
    let expr = "<math>
                    <msup> <mi>x</mi> <mn>0</mn> </msup>
                </math>";
    test("en", "SimpleSpeak", expr, "x to the 0")?;
    return Ok(());

}


#[test]
fn decimal_power() -> Result<()> {
    let expr = "<math>
                    <msup> <mi>x</mi> <mn>2.0</mn> </msup>
                </math>";
    test("en", "SimpleSpeak", expr, "x to the 2.0")?;
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
    test("en", "SimpleSpeak", expr, "3 raised to the y plus 2 power")?;
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
    test("en", "SimpleSpeak", expr, "x to the negative 2")?;
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
  test("en", "SimpleSpeak", expr, "x raised to the 1 third power")?;
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
  test("en", "SimpleSpeak", expr, "3 raised to the 2 x squared power")?;
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
  test("en", "SimpleSpeak", expr, "3 raised to the negative 2 x squared power")?;
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
  test("en", "SimpleSpeak", expr, "y raised to the 4 fifths cubed power")?;
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
  test("en", "SimpleSpeak", expr, "y raised to the negative 4 fifths cubed power")?;
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
  test("en", "SimpleSpeak", expr, "e raised to the 1 half x squared power")?;
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
  test("en", "SimpleSpeak", expr, "e raised to the negative 1 half x squared power")?;
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
  test("en", "SimpleSpeak", expr, "3 raised to the 3 to the tenth power")?;
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
  test("en", "SimpleSpeak", expr, "3 raised to the open paren x plus 1, close paren squared power")?;
  return Ok(());

}

#[test]
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
  test("en", "SimpleSpeak", expr, "t raised to the 4 fifths to the n-th power")?;
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
  test("en", "SimpleSpeak", expr, "t raised to the 4 fifths raised to the n plus 1 power; end exponent")?;
  test_prefs("en", "SimpleSpeak", vec![("Impairment", "LearningDisability")], expr,
  "t raised to the 4 fifths raised to the n plus 1 power")?;
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
  test("en", "SimpleSpeak", expr, "t raised to the 4 fifths to the negative 3, end exponent")?;
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
  test("en", "SimpleSpeak", expr, "e raised to the negative 1 half times; open paren, fraction, x minus mu, over sigma, end fraction; close paren squared power")?;
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
  test("en", "SimpleSpeak", expr, "t raised to the fraction, b plus 1, over 3, end fraction; power")?;
  return Ok(());

}
