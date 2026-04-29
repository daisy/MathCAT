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
    test("ru", "SimpleSpeak", expr, "икс в квадрате")?;
    return Ok(());
}

#[test]
fn cubed() -> Result<()> {
    let expr = "<math>
                    <msup> <mi>x</mi> <mn>3</mn> </msup>
                </math>";
    test("ru", "SimpleSpeak", expr, "икс в кубе")?;
    return Ok(());
}

#[test]
fn ordinal_power() -> Result<()> {
    let expr = "<math>
                        <msup> <mi>x</mi> <mn>4</mn> </msup>
                    </math>";
    test("ru", "SimpleSpeak", expr, "икс в четвёртой степени")?;
    return Ok(());
}

#[test]
fn simple_mi_power() -> Result<()> {
    let expr = "<math>
                    <msup> <mi>x</mi> <mi>n</mi> </msup>
                </math>";
    test("ru", "SimpleSpeak", expr, "икс в степени эн")?;
    return Ok(());
}

#[test]
fn zero_power() -> Result<()> {
    let expr = "<math>
                    <msup> <mi>x</mi> <mn>0</mn> </msup>
                </math>";
    test("ru", "SimpleSpeak", expr, "икс в нулевой степени")?;
    return Ok(());
}

#[test]
fn decimal_power() -> Result<()> {
    let expr = "<math>
                    <msup> <mi>x</mi> <mn>2.0</mn> </msup>
                </math>";
    test("ru", "SimpleSpeak", expr, "икс в степени 2.0")?;
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
    test("ru", "SimpleSpeak", expr, "3 в степени игрек плюс 2")?;
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
    test("ru", "SimpleSpeak", expr, "икс в степени минус 2")?;
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
    test("ru", "SimpleSpeak", expr, "икс в степени одна третья")?;
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
    test("ru", "SimpleSpeak", expr, "3 в степени 2 икс в квадрате")?;
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
    test("ru", "SimpleSpeak", expr, "3 в степени минус 2 икс в квадрате")?;
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
    test("ru", "SimpleSpeak", expr, "игрек в степени четыре пятых в кубе")?;
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
    test("ru", "SimpleSpeak", expr, "игрек в степени минус четыре пятых в кубе")?;
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
    test("ru", "SimpleSpeak", expr, "e в степени одна вторая икс в квадрате")?;
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
    test("ru", "SimpleSpeak", expr, "e в степени минус одна вторая икс в квадрате")?;
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
    test("ru", "SimpleSpeak", expr, "3 в степени 3 в десятой степени")?;
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
    test("ru", "SimpleSpeak", expr, "3 в степени скобка открывается икс плюс 1, скобка закрывается в квадрате")?;
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
    test("ru", "SimpleSpeak", expr, "тэ в степени четыре пятых в степени эн")?;
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
    test("ru", "SimpleSpeak", expr, "тэ в степени четыре пятых в степени n плюс 1, конец показателя")?;
    test_prefs("ru", "SimpleSpeak", vec![("Impairment", "LearningDisability")], expr,
               "тэ в степени четыре пятых в степени n плюс 1")?;
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
    test("ru", "SimpleSpeak", expr, "тэ в степени четыре пятых в степени минус 3, конец показателя")?;
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
    test("ru", "SimpleSpeak", expr,
         "e в степени минус одна вторая умножить на; скобка открывается, дробь, числитель: икс минус мю, знаменатель: сигма, конец дроби; скобка закрывается в квадрате")?;
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
    test("ru", "SimpleSpeak", expr, "тэ в степени дробь, числитель: бэ плюс 1, знаменатель: 3, конец дроби")?;
    return Ok(());
}
