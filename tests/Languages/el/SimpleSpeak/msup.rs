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
    test("el", "SimpleSpeak", expr, "x στο τετράγωνο")?; // x στο τετράγωνο
    return Ok(());

}

#[test]
fn cubed() -> Result<()> {
    let expr = "<math>
                    <msup> <mi>x</mi> <mn>3</mn> </msup>
                </math>";
    test("el", "SimpleSpeak", expr, "x στον κύβο")?; // x στον κύβο
    return Ok(());

}

#[test]
    fn ordinal_power() -> Result<()> {
        let expr = "<math>
                        <msup> <mi>x</mi> <mn>4</mn> </msup>
                    </math>";
        test("el", "SimpleSpeak", expr, "x στην τέταρτη")?;
        return Ok(());

    }

#[test]
fn simple_mi_power() -> Result<()> {
    let expr = "<math>
                    <msup> <mi>x</mi> <mi>n</mi> </msup>
                </math>";
  test("el", "SimpleSpeak", expr, "x στην n οστή")?;
  return Ok(());

}

#[test]
fn zero_power() -> Result<()> {
    let expr = "<math>
                    <msup> <mi>x</mi> <mn>0</mn> </msup>
                </math>";
    test("el", "SimpleSpeak", expr, "x στην 0")?;
    return Ok(());

}


#[test]
fn decimal_power() -> Result<()> {
    let expr = "<math>
                    <msup> <mi>x</mi> <mn>2.0</mn> </msup>
                </math>";
    test("el", "SimpleSpeak", expr, "x στην 2.0")?;
    return Ok(());
    //theodora. fails because of 2.0 -> 20
    // The issue is at src/prefs (set_separator, decimal_separator)

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
    test("el", "SimpleSpeak", expr, "3 που υψώνεται στη y συν 2 τέλος δύναμης")?;
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
    test("el", "SimpleSpeak", expr, "x στην μείον 2")?;
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
  test("el", "SimpleSpeak", expr, "x που υψώνεται στη 1 τρίτο τέλος δύναμης")?;
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
  test("el", "SimpleSpeak", expr, "3 που υψώνεται στη 2 x στο τετράγωνο τέλος δύναμης")?;
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
  test("el", "SimpleSpeak", expr, "3 που υψώνεται στη μείον 2 x στο τετράγωνο τέλος δύναμης")?;
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
  test("el", "SimpleSpeak", expr, "y που υψώνεται στη 4 πέμπτα στον κύβο τέλος δύναμης")?;
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
  test("el", "SimpleSpeak", expr, "y που υψώνεται στη μείον 4 πέμπτα στον κύβο τέλος δύναμης")?;
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
  test("el", "SimpleSpeak", expr, "e που υψώνεται στη 1 δεύτερο x στο τετράγωνο τέλος δύναμης")?;
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
  test("el", "SimpleSpeak", expr, "e που υψώνεται στη μείον 1 δεύτερο, x στο τετράγωνο τέλος δύναμης")?;
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
  test("el", "SimpleSpeak", expr, "3 που υψώνεται στη 3 στην δέκατη τέλος δύναμης")?;
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
  test("el", "SimpleSpeak", expr, "3 που υψώνεται στη ανοίγει παρένθεση, x συν 1, κλείνει παρένθεση στο τετράγωνο τέλος δύναμης")?;
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
  test("el", "SimpleSpeak", expr, "t που υψώνεται στη 4 πέμπτα στην n οστή τέλος δύναμης")?;
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
  test("el", "SimpleSpeak", expr, "t που υψώνεται στη 4 πέμπτα που υψώνεται στη n συν 1 τέλος δύναμης; τέλος εκθέτη")?;
  test_prefs("el", "SimpleSpeak", vec![("Impairment", "LearningDisability")], expr,
  "t που υψώνεται στη 4 πέμπτα που υψώνεται στη n συν 1 τέλος δύναμης")?;
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
  test("el", "SimpleSpeak", expr, "t που υψώνεται στη 4 πέμπτα στην μείον 3, τέλος εκθέτη")?;
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
  test("el", "SimpleSpeak", expr, "e που υψώνεται στη μείον 1 δεύτερο; ανοίγει παρένθεση; κλάσμα, x μείον μί, προς σίγμα, τέλος κλάσματος; κλείνει παρένθεση στο τετράγωνο τέλος δύναμης")?; 
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
  test("el", "SimpleSpeak", expr, "t που υψώνεται στη κλάσμα, b συν 1, προς 3, τέλος κλάσματος; τέλος δύναμης")?; // t που υψώνεται στη κλάσμα, b συν 1, προς 3, τέλος κλάσματος; τέλος δύναμης
  return Ok(());

}
