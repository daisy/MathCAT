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
    test_prefs("ru", "ClearSpeak", vec![("Verbosity", "Medium"), ("ClearSpeak_Exponents", "Auto")], expr, "икс в квадрате")?;
    test_prefs("ru", "ClearSpeak", vec![("Verbosity", "Medium"), ("ClearSpeak_Exponents", "Ordinal")], expr, "икс во второй")?;
    test_prefs("ru", "ClearSpeak", vec![("Verbosity", "Medium"), ("ClearSpeak_Exponents", "OrdinalPower")], expr, "икс во второй степени")?;
    test_prefs("ru", "ClearSpeak", vec![("Verbosity", "Medium"), ("ClearSpeak_Exponents", "AfterPower")], expr, "икс в степени 2")?;
    return Ok(());
}

#[test]
fn cubed() -> Result<()> {
  let expr = "<math>
                  <msup> <mi>x</mi> <mn>3</mn> </msup>
              </math>";
  test_prefs("ru", "ClearSpeak", vec![("Verbosity", "Medium"), ("ClearSpeak_Exponents", "Auto")], expr, "икс в кубе")?;
  test_prefs("ru", "ClearSpeak", vec![("Verbosity", "Medium"), ("ClearSpeak_Exponents", "Ordinal")], expr, "икс в третьей")?;
  test_prefs("ru", "ClearSpeak", vec![("Verbosity", "Medium"), ("ClearSpeak_Exponents", "OrdinalPower")], expr, "икс в третьей степени")?;
  test_prefs("ru", "ClearSpeak", vec![("Verbosity", "Medium"), ("ClearSpeak_Exponents", "AfterPower")], expr, "икс в степени 3")?;
  return Ok(());
}

#[test]
fn ordinal_power() -> Result<()> {
  let expr = "<math>
                  <msup> <mn>3</mn> <mn>5</mn> </msup>
              </math>";
  test_prefs("ru", "ClearSpeak", vec![("Verbosity", "Medium"), ("ClearSpeak_Exponents", "Auto")], expr, "3 в пятой степени")?;
  test_prefs("ru", "ClearSpeak", vec![("Verbosity", "Medium"), ("ClearSpeak_Exponents", "Ordinal")], expr, "3 в пятой")?;
  test_prefs("ru", "ClearSpeak", vec![("Verbosity", "Medium"), ("ClearSpeak_Exponents", "OrdinalPower")], expr, "3 в пятой степени")?;
  test_prefs("ru", "ClearSpeak", vec![("Verbosity", "Medium"), ("ClearSpeak_Exponents", "AfterPower")], expr, "3 в степени 5")?;
  return Ok(());
}

#[test]
fn zero_power() -> Result<()> {
  let expr = "<math>
                    <msup> <mn>3</mn> <mn>0</mn> </msup>
                </math>";
  test_prefs("ru", "ClearSpeak", vec![("Verbosity", "Medium"), ("ClearSpeak_Exponents", "Auto")], expr, "3 в нулевой степени")?;
  test_prefs("ru", "ClearSpeak", vec![("Verbosity", "Medium"), ("ClearSpeak_Exponents", "Ordinal")], expr, "3 в нулевой")?;
  test_prefs("ru", "ClearSpeak", vec![("Verbosity", "Medium"), ("ClearSpeak_Exponents", "OrdinalPower")], expr, "3 в нулевой степени")?;
  test_prefs("ru", "ClearSpeak", vec![("Verbosity", "Medium"), ("ClearSpeak_Exponents", "AfterPower")], expr, "3 в степени 0")?;
  return Ok(());
}

#[test]
fn simple_mi_power() -> Result<()> {
  let expr = "<math>
                    <msup> <mn>4</mn> <mi>x</mi> </msup>
                </math>";
  test_prefs("ru", "ClearSpeak", vec![("Verbosity", "Medium"), ("ClearSpeak_Exponents", "Auto")], expr, "4 в степени икс")?;
  test_prefs("ru", "ClearSpeak", vec![("Verbosity", "Medium"), ("ClearSpeak_Exponents", "Ordinal")], expr, "4 в степени икс")?;
  test_prefs("ru", "ClearSpeak", vec![("Verbosity", "Medium"), ("ClearSpeak_Exponents", "OrdinalPower")], expr, "4 в степени икс")?;
  test_prefs("ru", "ClearSpeak", vec![("Verbosity", "Medium"), ("ClearSpeak_Exponents", "AfterPower")], expr, "4 в степени икс")?;
  return Ok(());
}

#[test]
fn decimal_power() -> Result<()> {
  let expr = "<math>
                  <msup> <mn>3</mn> <mn>5.0</mn> </msup>
              </math>";
  test_prefs("ru", "ClearSpeak", vec![("Verbosity", "Medium"), ("ClearSpeak_Exponents", "Auto")], expr, "3 в степени 5.0")?;
  test_prefs("ru", "ClearSpeak", vec![("Verbosity", "Medium"), ("ClearSpeak_Exponents", "Ordinal")], expr, "3 в степени 5.0")?;
  test_prefs("ru", "ClearSpeak", vec![("Verbosity", "Medium"), ("ClearSpeak_Exponents", "OrdinalPower")], expr, "3 в степени 5.0")?;
  test_prefs("ru", "ClearSpeak", vec![("Verbosity", "Medium"), ("ClearSpeak_Exponents", "AfterPower")], expr, "3 в степени 5.0")?;
  return Ok(());
}

#[test]
fn non_simple_power() -> Result<()> {
  let expr = "<math>
        <msup> <mn>3</mn>  <mrow> <mi>y</mi><mo>+</mo><mn>2</mn></mrow>  </msup>
    </math>";
  test_prefs("ru", "ClearSpeak", vec![("Verbosity", "Medium"), ("ClearSpeak_Exponents", "Auto")], expr, "3 в степени игрек плюс 2")?;
  test_prefs("ru", "ClearSpeak", vec![("Verbosity", "Medium"), ("ClearSpeak_Exponents", "Ordinal")], expr, "3 в степени игрек плюс 2")?;
  test_prefs("ru", "ClearSpeak", vec![("Verbosity", "Medium"), ("ClearSpeak_Exponents", "OrdinalPower")], expr, "3 в степени игрек плюс 2")?;
  test_prefs("ru", "ClearSpeak", vec![("Verbosity", "Medium"), ("ClearSpeak_Exponents", "AfterPower")], expr, "3 в степени игрек плюс 2")?;
  return Ok(());
}

#[test]
fn negative_power() -> Result<()> {
  let expr = "<math>
                  <msup> <mn>3</mn> <mrow> <mo>-</mo> <mn>2</mn> </mrow> </msup>
              </math>";
  test_prefs("ru", "ClearSpeak", vec![("Verbosity", "Medium"), ("ClearSpeak_Exponents", "Auto")], expr, "3 в степени минус 2")?;
  test_prefs("ru", "ClearSpeak", vec![("Verbosity", "Medium"), ("ClearSpeak_Exponents", "Ordinal")], expr, "3 в степени минус 2")?;
  test_prefs("ru", "ClearSpeak", vec![("Verbosity", "Medium"), ("ClearSpeak_Exponents", "OrdinalPower")], expr, "3 в степени минус 2")?;
  test_prefs("ru", "ClearSpeak", vec![("Verbosity", "Medium"), ("ClearSpeak_Exponents", "AfterPower")], expr, "3 в степени минус 2")?;
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
  test("ru", "ClearSpeak", expr, "икс в степени одна третья")?;
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
  test_prefs("ru", "ClearSpeak", vec![("Verbosity", "Medium"), ("ClearSpeak_Exponents", "Auto")], expr, "3 в степени 2 икс в квадрате")?;
  test_prefs("ru", "ClearSpeak", vec![("Verbosity", "Medium"), ("ClearSpeak_Exponents", "Ordinal")], expr, "3 в степени (показатель: 2 икс во второй, конец показателя)")?;
  test_prefs("ru", "ClearSpeak", vec![("Verbosity", "Medium"), ("ClearSpeak_Exponents", "OrdinalPower")], expr, "3 в степени (показатель: 2 икс во второй степени, конец показателя)")?;
  test_prefs("ru", "ClearSpeak", vec![("Verbosity", "Medium"), ("ClearSpeak_Exponents", "AfterPower")], expr, "3 в степени (показатель: 2 икс в степени 2, конец показателя)")?;
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
  test("ru", "ClearSpeak", expr, "3 в степени минус 2 икс в квадрате")?;
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
  test("ru", "ClearSpeak", expr, "игрек в степени четыре пятых в кубе")?;
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
  test("ru", "ClearSpeak", expr, "игрек в степени минус четыре пятых в кубе")?;
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
  test_prefs("ru", "ClearSpeak", vec![("Verbosity", "Medium"), ("ClearSpeak_Exponents", "Auto")], expr, "e в степени одна вторая икс в квадрате")?;
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
  test_prefs("ru", "ClearSpeak", vec![("Verbosity", "Medium"), ("ClearSpeak_Exponents", "Auto")], expr, "e в степени минус одна вторая икс в квадрате")?;
  test_prefs("ru", "ClearSpeak", vec![("Verbosity", "Medium"), ("ClearSpeak_Exponents", "Ordinal")], expr, "e в степени (показатель: минус одна вторая икс во второй, конец показателя)")?;
  test_prefs("ru", "ClearSpeak", vec![("Verbosity", "Medium"), ("ClearSpeak_Exponents", "OrdinalPower")], expr, "e в степени (показатель: минус одна вторая икс во второй степени, конец показателя)")?;
  test_prefs("ru", "ClearSpeak", vec![("Verbosity", "Medium"), ("ClearSpeak_Exponents", "AfterPower")], expr, "e в степени (показатель: минус одна вторая икс в степени 2, конец показателя)")?;
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
  test_prefs("ru", "ClearSpeak", vec![("Verbosity", "Medium"), ("ClearSpeak_Exponents", "Auto")], expr, "3 в степени (показатель: 3 в десятой степени, конец показателя)")?;
  test_prefs("ru", "ClearSpeak", vec![("Verbosity", "Medium"), ("ClearSpeak_Exponents", "Ordinal")], expr, "3 в степени (показатель: 3 в десятой, конец показателя)")?;
  test_prefs("ru", "ClearSpeak", vec![("Verbosity", "Medium"), ("ClearSpeak_Exponents", "OrdinalPower")], expr, "3 в степени (показатель: 3 в десятой степени, конец показателя)")?;
  test_prefs("ru", "ClearSpeak", vec![("Verbosity", "Medium"), ("ClearSpeak_Exponents", "AfterPower")], expr, "3 в степени (показатель: 3 в степени 10, конец показателя)")?;
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
  test_prefs("ru", "ClearSpeak", vec![("Verbosity", "Medium"), ("ClearSpeak_Exponents", "Auto")], expr, "3 в степени (показатель: скобка открывается икс плюс 1, скобка закрывается в квадрате, конец показателя)")?;
  test_prefs("ru", "ClearSpeak", vec![("Verbosity", "Medium"), ("ClearSpeak_Exponents", "Ordinal")], expr, "3 в степени (показатель: скобка открывается икс плюс 1, скобка закрывается во второй, конец показателя)")?;
  test_prefs("ru", "ClearSpeak", vec![("Verbosity", "Medium"), ("ClearSpeak_Exponents", "OrdinalPower")], expr, "3 в степени (показатель: скобка открывается икс плюс 1, скобка закрывается во второй степени, конец показателя)")?;
  test_prefs("ru", "ClearSpeak", vec![("Verbosity", "Medium"), ("ClearSpeak_Exponents", "AfterPower")], expr, "3 в степени (показатель: скобка открывается икс плюс 1, скобка закрывается в степени 2, конец показателя)")?;
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
  test("ru", "ClearSpeak", expr, "тэ в степени (показатель: четыре пятых в степени эн, конец показателя)")?;
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
  test_prefs("ru", "ClearSpeak", vec![("Verbosity", "Medium"), ("ClearSpeak_Exponents", "Auto")], expr,
       "e в степени (показатель: минус одна вторая умножить на; скобка открывается; дробь, числитель: икс минус мю; знаменатель: сигма; скобка закрывается в квадрате, конец показателя)")?;
  test_prefs("ru", "ClearSpeak", vec![("Verbosity", "Medium"), ("ClearSpeak_Exponents", "Ordinal")], expr,
       "e в степени (показатель: минус одна вторая умножить на; скобка открывается; дробь, числитель: икс минус мю; знаменатель: сигма; скобка закрывается во второй, конец показателя)")?;
  test_prefs("ru", "ClearSpeak", vec![("Verbosity", "Medium"), ("ClearSpeak_Exponents", "OrdinalPower")], expr,
       "e в степени (показатель: минус одна вторая умножить на; скобка открывается; дробь, числитель: икс минус мю; знаменатель: сигма; скобка закрывается во второй степени, конец показателя)")?;
  test_prefs("ru", "ClearSpeak", vec![("Verbosity", "Medium"), ("ClearSpeak_Exponents", "AfterPower")], expr,
       "e в степени (показатель: минус одна вторая умножить на; скобка открывается; дробь, числитель: икс минус мю; знаменатель: сигма; скобка закрывается в степени 2, конец показателя)")?;
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
  test("ru", "ClearSpeak", expr, "тэ в степени дробь, числитель: бэ плюс 1; знаменатель: 3")?;
  return Ok(());
}
