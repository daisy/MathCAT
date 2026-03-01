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
    test("sv", "SimpleSpeak", expr, "x kvadrat")?;
    return Ok(());

}

#[test]
fn cubed() -> Result<()> {
    let expr = "<math>
                    <msup> <mi>x</mi> <mn>3</mn> </msup>
                </math>";
    test("sv", "SimpleSpeak", expr, "x kubik")?;
    return Ok(());

}

#[test]
    fn ordinal_power() -> Result<()> {
        let expr = "<math>
                        <msup> <mi>x</mi> <mn>4</mn> </msup>
                    </math>";
        test("sv", "SimpleSpeak", expr, "x upphöjt till 4")?;
        return Ok(());

    }

#[test]
fn simple_mi_power() -> Result<()> {
    let expr = "<math>
                    <msup> <mi>x</mi> <mi>n</mi> </msup>
                </math>";
  test("sv", "SimpleSpeak", expr, "x upphöjt till n")?;
  return Ok(());

}

#[test]
fn zero_power() -> Result<()> {
    let expr = "<math>
                    <msup> <mi>x</mi> <mn>0</mn> </msup>
                </math>";
    test("sv", "SimpleSpeak", expr, "x upphöjt till 0")?;
    return Ok(());

}


#[test]
fn decimal_power() -> Result<()> {
    let expr = "<math>
                    <msup> <mi>x</mi> <mn>2,0</mn> </msup>
                </math>";
    test("sv", "SimpleSpeak", expr, "x upphöjt till 2,0")?;
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
    test("sv", "SimpleSpeak", expr, "3 upphöjt till y plus 2")?;
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
    test("sv", "SimpleSpeak", expr, "x upphöjt till minus 2")?;
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
  test("sv", "SimpleSpeak", expr, "x upphöjt till 1 tredjedel")?;
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
  test("sv", "SimpleSpeak", expr, "3 upphöjt till 2 x kvadrat")?;
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
  test("sv", "SimpleSpeak", expr, "3 upphöjt till minus 2 x kvadrat")?;
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
  test("sv", "SimpleSpeak", expr, "y upphöjt till 4 femtedelar kubik")?;
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
  test("sv", "SimpleSpeak", expr, "y upphöjt till minus 4 femtedelar kubik")?;
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
  test("sv", "SimpleSpeak", expr, "e upphöjt till 1 halv x kvadrat")?;
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
  test("sv", "SimpleSpeak", expr, "e upphöjt till minus 1 halv x kvadrat")?;
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
  test("sv", "SimpleSpeak", expr, "3 upphöjt till 3 upphöjt till 10")?;
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
  test("sv", "SimpleSpeak", expr, "3 upphöjt till vänster-parentes; x plus 1; höger-parentes kvadrat")?;
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
  test("sv", "SimpleSpeak", expr, "t upphöjt till 4 femtedelar upphöjt till n")?;
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
  test("sv", "SimpleSpeak", expr, "t upphöjt till 4 femtedelar upphöjt till n plus 1, slut exponent")?;
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
  test("sv", "SimpleSpeak", expr, "t upphöjt till 4 femtedelar upphöjt till minus 3; slut exponent")?;
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
  test("sv", "SimpleSpeak", expr, "e upphöjt till minus 1 halv gånger; vänster-parentes; division, x minus my, genom sigma, slut division; höger-parentes kvadrat")?;
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
  test("sv", "SimpleSpeak", expr, "t upphöjt till division, b plus 1, genom 3, slut division")?;
  return Ok(());

}
