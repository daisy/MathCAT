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
    test("hu", "SimpleSpeak", expr, "x a négyzeten")?;
    return Ok(());

}

// AI generated
#[test]
fn cubed() -> Result<()> {
    let expr = "<math>
                    <msup> <mi>x</mi> <mn>3</mn> </msup>
                </math>";
    test("hu", "SimpleSpeak", expr, "x a köbön")?;
    return Ok(());

}

// AI generated
#[test]
    fn ordinal_power() -> Result<()> {
        let expr = "<math>
                        <msup> <mi>x</mi> <mn>4</mn> </msup>
                    </math>";
        test("hu", "SimpleSpeak", expr, "x a(z) negyedik hatványon")?;
        return Ok(());

    }

// AI generated
#[test]
fn simple_mi_power() -> Result<()> {
    let expr = "<math>
                    <msup> <mi>x</mi> <mi>n</mi> </msup>
                </math>";
  test("hu", "SimpleSpeak", expr, "x a(z) n-edik hatványon")?;
  return Ok(());

}

// AI generated
#[test]
fn zero_power() -> Result<()> {
    let expr = "<math>
                    <msup> <mi>x</mi> <mn>0</mn> </msup>
                </math>";
    test("hu", "SimpleSpeak", expr, "x a(z) nulladik hatványon")?;
    return Ok(());

}


// AI generated
#[test]
fn decimal_power() -> Result<()> {
    let expr = "<math>
                    <msup> <mi>x</mi> <mn>2.0</mn> </msup>
                </math>";
    test("hu", "SimpleSpeak", expr, "x a(z) 20 hatványon")?;
    return Ok(());

}

// AI generated
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
    test("hu", "SimpleSpeak", expr, "3 a(z) y plusz 2 hatványon")?;
    return Ok(());

}

// AI generated
#[test]
fn negative_power() -> Result<()> {
    let expr = "<math>
                    <msup>
                        <mi>x</mi>
                        <mrow> <mo>-</mo> <mn>2</mn> </mrow>
                    </msup>
                </math>";
    test("hu", "SimpleSpeak", expr, "x a negatív 2 hatványon")?;
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
  test("hu", "SimpleSpeak", expr, "x a(z) 1 harmad hatványon")?;
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
  test("hu", "SimpleSpeak", expr, "3 emelve a(z) 2 x a négyzeten hatványra")?;
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
  test("hu", "SimpleSpeak", expr, "3 emelve a negatív 2 x a négyzeten hatványra")?;
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
  test("hu", "SimpleSpeak", expr, "y emelve a(z) 4 ötöd a köbön hatványra")?;
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
  test("hu", "SimpleSpeak", expr, "y a(z) negatív 4 ötöd a köbön hatványon")?;
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
  test("hu", "SimpleSpeak", expr, "e emelve a(z) 1 ketted x a négyzeten hatványra")?;
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
  test("hu", "SimpleSpeak", expr, "e emelve a negatív 1 ketted, x a négyzeten hatványra")?;
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
  test("hu", "SimpleSpeak", expr, "3 emelve a következő kitevőre:, 3 a(z) tizedik hatványon, kitevő vége")?;
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
  test("hu", "SimpleSpeak", expr, "3 emelve a következő kitevőre:, nyitott zárójel, x plusz 1, zárt zárójel a négyzeten, kitevő vége")?;
  return Ok(());

}

// AI generated
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
  test("hu", "SimpleSpeak", expr, "t emelve a következő kitevőre:, 4 ötöd a(z) n-edik hatványon, kitevő vége")?;
  return Ok(());

}

// AI generated
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
  test("hu", "SimpleSpeak", expr, "t emelve a következő kitevőre:, 4 ötöd a(z) n plusz 1 hatványon, kitevő vége")?;
  test_prefs("hu", "SimpleSpeak", vec![("Impairment", "LearningDisability")], expr,
  "t emelve a következő kitevőre:, 4 ötöd a(z) n plusz 1 hatványon, kitevő vége")?;
  return Ok(());

}

// AI generated
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
  test("hu", "SimpleSpeak", expr, "t emelve a következő kitevőre:, 4 ötöd a negatív 3 hatványon, kitevő vége")?;
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
  test("hu", "SimpleSpeak", expr, "e emelve a következő kitevőre:, negatív 1 ketted, szorozva; nyitott zárójel; tört, x mínusz mű, per szigma, tört vége; zárt zárójel a négyzeten, kitevő vége")?;
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
  test("hu", "SimpleSpeak", expr, "t a(z) tört, b plusz 1, per 3, tört vége; hatványon")?;
  return Ok(());

}
