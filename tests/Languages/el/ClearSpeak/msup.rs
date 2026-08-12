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
    test_prefs("el", "ClearSpeak", vec![("Verbosity", "Medium"), ("ClearSpeak_Exponents", "Auto")], expr, "x στο τετράγωνο")?;
    test_prefs("el", "ClearSpeak", vec![("Verbosity", "Medium"), ("ClearSpeak_Exponents", "Ordinal")], expr, "x στην δεύτερη")?;
    test_prefs("el", "ClearSpeak", vec![("Verbosity", "Medium"), ("ClearSpeak_Exponents", "OrdinalPower")], expr, "x στην δεύτερη δύναμη")?;
    test_prefs("el", "ClearSpeak", vec![("Verbosity", "Medium"), ("ClearSpeak_Exponents", "AfterPower")], expr, "x υψωμένο στη δύναμη 2")?;

    return Ok(());

}

#[test]
fn cubed() -> Result<()> {
  let expr = "<math>
                  <msup> <mi>x</mi> <mn>3</mn> </msup>
              </math>";
  test_prefs("el", "ClearSpeak", vec![("Verbosity", "Medium"), ("ClearSpeak_Exponents", "Auto")], expr, "x στον κύβο")?;
  test_prefs("el", "ClearSpeak", vec![("Verbosity", "Medium"), ("ClearSpeak_Exponents", "Ordinal")], expr, "x στην τρίτη")?;
  test_prefs("el", "ClearSpeak", vec![("Verbosity", "Medium"), ("ClearSpeak_Exponents", "OrdinalPower")], expr, "x στην τρίτη δύναμη")?;
  test_prefs("el", "ClearSpeak", vec![("Verbosity", "Medium"), ("ClearSpeak_Exponents", "AfterPower")], expr, "x υψωμένο στη δύναμη 3")?;
  return Ok(());

}

#[test]
fn ordinal_power() -> Result<()> {
  let expr = "<math>
                  <msup> <mn>3</mn> <mn>5</mn> </msup>
              </math>";
  test_prefs("el", "ClearSpeak", vec![("Verbosity", "Medium"), ("ClearSpeak_Exponents", "Auto")], expr, "3 στην πέμπτη δύναμη")?;
  test_prefs("el", "ClearSpeak", vec![("Verbosity", "Medium"), ("ClearSpeak_Exponents", "Ordinal")], expr, "3 στην πέμπτη")?;
  test_prefs("el", "ClearSpeak", vec![("Verbosity", "Medium"), ("ClearSpeak_Exponents", "OrdinalPower")], expr, "3 στην πέμπτη δύναμη")?;
  test_prefs("el", "ClearSpeak", vec![("Verbosity", "Medium"), ("ClearSpeak_Exponents", "AfterPower")], expr, "3 υψωμένο στη δύναμη 5")?;
  return Ok(());

}


#[test]
fn zero_power() -> Result<()> {
  let expr = "<math>
                    <msup> <mn>3</mn> <mn>0</mn> </msup>
                </math>";
  test_prefs("el", "ClearSpeak", vec![("Verbosity", "Medium"), ("ClearSpeak_Exponents", "Auto")], expr, "3 στην 0 δύναμη")?;
  test_prefs("el", "ClearSpeak", vec![("Verbosity", "Medium"), ("ClearSpeak_Exponents", "Ordinal")], expr, "3 στην 0")?;
  test_prefs("el", "ClearSpeak", vec![("Verbosity", "Medium"), ("ClearSpeak_Exponents", "OrdinalPower")], expr, "3 στην 0 δύναμη")?;
  test_prefs("el", "ClearSpeak", vec![("Verbosity", "Medium"), ("ClearSpeak_Exponents", "AfterPower")], expr, "3 υψωμένο στη δύναμη 0")?;
  return Ok(());

}

#[test]
fn simple_mi_power() -> Result<()> {
  let expr = "<math>
                    <msup> <mn>4</mn> <mi>x</mi> </msup>
                </math>";
  test_prefs("el", "ClearSpeak", vec![("Verbosity", "Medium"), ("ClearSpeak_Exponents", "Auto")], expr, "4 στην x οστή δύναμη")?;
  test_prefs("el", "ClearSpeak", vec![("Verbosity", "Medium"), ("ClearSpeak_Exponents", "Ordinal")], expr, "4 στην x οστή")?;
  test_prefs("el", "ClearSpeak", vec![("Verbosity", "Medium"), ("ClearSpeak_Exponents", "OrdinalPower")], expr, "4 στην x οστή δύναμη")?;
  test_prefs("el", "ClearSpeak", vec![("Verbosity", "Medium"), ("ClearSpeak_Exponents", "AfterPower")], expr, "4 υψωμένο στη δύναμη x")?;
  return Ok(());

}

#[test]
fn decimal_power() -> Result<()> {
  let expr = "<math>
                  <msup> <mn>3</mn> <mn>5.0</mn> </msup>
              </math>";
  test_prefs("el", "ClearSpeak", vec![("Verbosity", "Medium"), ("ClearSpeak_Exponents", "Auto")], expr, "3 υψωμένο στην 5.0 τέλος δύναμης")?;
  test_prefs("el", "ClearSpeak", vec![("Verbosity", "Medium"), ("ClearSpeak_Exponents", "Ordinal")], expr, "3 υψωμένο στην 5.0 τέλος δύναμης")?;
  test_prefs("el", "ClearSpeak", vec![("Verbosity", "Medium"), ("ClearSpeak_Exponents", "OrdinalPower")], expr, "3 υψωμένο στην 5.0 τέλος δύναμης")?;
  test_prefs("el", "ClearSpeak", vec![("Verbosity", "Medium"), ("ClearSpeak_Exponents", "AfterPower")], expr, "3 υψωμένο στην 5.0")?;
  return Ok(());
//theodora. fails because 5.0 is somehow converted to 50, probably due to handling decimals
// The issue is at src/prefs (set_separator, decimal_separator)
}

#[test]
fn non_simple_power() -> Result<()> {
  let expr = "<math>
        <msup> <mn>3</mn>  <mrow> <mi>y</mi><mo>+</mo><mn>2</mn></mrow>  </msup>
    </math>";
  test_prefs("el", "ClearSpeak", vec![("Verbosity", "Medium"), ("ClearSpeak_Exponents", "Auto")], expr, "3 υψωμένο στην y συν 2 τέλος δύναμης")?;
  test_prefs("el", "ClearSpeak", vec![("Verbosity", "Medium"), ("ClearSpeak_Exponents", "Ordinal")], expr, "3 υψωμένο στην y συν 2 τέλος δύναμης")?;
  test_prefs("el", "ClearSpeak", vec![("Verbosity", "Medium"), ("ClearSpeak_Exponents", "OrdinalPower")], expr, "3 υψωμένο στην y συν 2 τέλος δύναμης")?;
  test_prefs("el", "ClearSpeak", vec![("Verbosity", "Medium"), ("ClearSpeak_Exponents", "AfterPower")], expr, "3 υψωμένο στη δύναμη y συν 2")?;
  return Ok(());

}

#[test]
fn negative_power() -> Result<()> {
  let expr = "<math>
                  <msup> <mn>3</mn> <mrow> <mo>-</mo> <mn>2</mn> </mrow> </msup>
              </math>";
  test_prefs("el", "ClearSpeak", vec![("Verbosity", "Medium"), ("ClearSpeak_Exponents", "Auto")], expr, "3 στην μείον 2 δύναμη")?;
  test_prefs("el", "ClearSpeak", vec![("Verbosity", "Medium"), ("ClearSpeak_Exponents", "Ordinal")], expr, "3 στην μείον 2")?;
  test_prefs("el", "ClearSpeak", vec![("Verbosity", "Medium"), ("ClearSpeak_Exponents", "OrdinalPower")], expr, "3 στην μείον 2 δύναμη")?;
  test_prefs("el", "ClearSpeak", vec![("Verbosity", "Medium"), ("ClearSpeak_Exponents", "AfterPower")], expr, "3 υψωμένο στη δύναμη μείον 2")?;
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
  test("el", "ClearSpeak", expr, "x υψωμένο στην 1 τρίτο τέλος δύναμης")?;
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
  test_prefs("el", "ClearSpeak", vec![("Verbosity", "Medium"), ("ClearSpeak_Exponents", "Auto")], expr, "3 υψωμένο στην 2 x στο τετράγωνο δύναμη")?;
  test_prefs("el", "ClearSpeak", vec![("Verbosity", "Medium"), ("ClearSpeak_Exponents", "Ordinal")], expr, "3 υψωμένο στον εκθέτη, 2 x στην δεύτερη, τέλος εκθέτη")?;
  test_prefs("el", "ClearSpeak", vec![("Verbosity", "Medium"), ("ClearSpeak_Exponents", "OrdinalPower")], expr, "3 υψωμένο στον εκθέτη, 2, x στην δεύτερη δύναμη, τέλος εκθέτη")?;
  test_prefs("el", "ClearSpeak", vec![("Verbosity", "Medium"), ("ClearSpeak_Exponents", "AfterPower")], expr, "3 υψωμένο στον εκθέτη, 2, x υψωμένο στη δύναμη 2; τέλος εκθέτη")?;

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
  test("el", "ClearSpeak", expr, "3 υψωμένο στην μείον 2 x στο τετράγωνο δύναμη")?;
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
  test("el", "ClearSpeak", expr, "y υψωμένο στην 4 πέμπτα στον κύβο δύναμη")?;
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
  test("el", "ClearSpeak", expr, "y υψωμένο στην μείον 4 πέμπτα στον κύβο τέλος δύναμης")?;
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
  test_prefs("el", "ClearSpeak", vec![("Verbosity", "Medium"), ("ClearSpeak_Exponents", "Auto")], expr, "e υψωμένο στην 1 δεύτερο x στο τετράγωνο δύναμη")?;
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
  test_prefs("el", "ClearSpeak", vec![("Verbosity", "Medium"), ("ClearSpeak_Exponents", "Auto")], expr, "e υψωμένο στην μείον 1 δεύτερο, x στο τετράγωνο δύναμη")?;
  test_prefs("el", "ClearSpeak", vec![("Verbosity", "Medium"), ("ClearSpeak_Exponents", "Ordinal")], expr, "e υψωμένο στον εκθέτη, μείον 1 δεύτερο, x στην δεύτερη, τέλος εκθέτη")?;
  test_prefs("el", "ClearSpeak", vec![("Verbosity", "Medium"), ("ClearSpeak_Exponents", "OrdinalPower")], expr, "e υψωμένο στον εκθέτη, μείον 1 δεύτερο; x στην δεύτερη δύναμη, τέλος εκθέτη")?;
  test_prefs("el", "ClearSpeak", vec![("Verbosity", "Medium"), ("ClearSpeak_Exponents", "AfterPower")], expr, "e υψωμένο στον εκθέτη, μείον 1 δεύτερο; x υψωμένο στη δύναμη 2; τέλος εκθέτη")?;
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
  test_prefs("el", "ClearSpeak", vec![("Verbosity", "Medium"), ("ClearSpeak_Exponents", "Auto")], expr, "3 υψωμένο στον εκθέτη, 3 στην δέκατη δύναμη, τέλος εκθέτη")?;
  test_prefs("el", "ClearSpeak", vec![("Verbosity", "Medium"), ("ClearSpeak_Exponents", "Ordinal")], expr, "3 υψωμένο στον εκθέτη, 3 στην δέκατη, τέλος εκθέτη")?;
  test_prefs("el", "ClearSpeak", vec![("Verbosity", "Medium"), ("ClearSpeak_Exponents", "OrdinalPower")], expr, "3 υψωμένο στον εκθέτη, 3 στην δέκατη δύναμη, τέλος εκθέτη")?;
  test_prefs("el", "ClearSpeak", vec![("Verbosity", "Medium"), ("ClearSpeak_Exponents", "AfterPower")], expr, "3 υψωμένο στον εκθέτη, 3 υψωμένο στη δύναμη 10; τέλος εκθέτη")?;

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
  test_prefs("el", "ClearSpeak", vec![("Verbosity", "Medium"), ("ClearSpeak_Exponents", "Auto")], expr, "3 υψωμένο στον εκθέτη, ανοίγει παρένθεση, x συν 1, κλείνει παρένθεση στο τετράγωνο, τέλος εκθέτη")?;
  test_prefs("el", "ClearSpeak", vec![("Verbosity", "Medium"), ("ClearSpeak_Exponents", "Ordinal")], expr, "3 υψωμένο στον εκθέτη, ανοίγει παρένθεση, x συν 1, κλείνει παρένθεση στην δεύτερη, τέλος εκθέτη")?;
  test_prefs("el", "ClearSpeak", vec![("Verbosity", "Medium"), ("ClearSpeak_Exponents", "OrdinalPower")], expr, "3 υψωμένο στον εκθέτη, ανοίγει παρένθεση, x συν 1, κλείνει παρένθεση στην δεύτερη δύναμη, τέλος εκθέτη")?;
  test_prefs("el", "ClearSpeak", vec![("Verbosity", "Medium"), ("ClearSpeak_Exponents", "AfterPower")], expr, "3 υψωμένο στον εκθέτη, ανοίγει παρένθεση, x συν 1, κλείνει παρένθεση υψωμένο στη δύναμη 2; τέλος εκθέτη")?;
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
  test("el", "ClearSpeak", expr, "t υψωμένο στον εκθέτη, 4 πέμπτα στην n οστή δύναμη, τέλος εκθέτη")?;
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
  test_prefs("el", "ClearSpeak", vec![("Verbosity", "Medium"), ("ClearSpeak_Exponents", "Auto")], expr,
       "e υψωμένο στον εκθέτη, μείον 1 δεύτερο; ανοίγει παρένθεση; το κλάσμα με αριθμητή; x μείον μί; και παρονομαστή σίγμα; κλείνει παρένθεση στο τετράγωνο, τέλος εκθέτη")?;
  test_prefs("el", "ClearSpeak", vec![("Verbosity", "Medium"), ("ClearSpeak_Exponents", "Ordinal")], expr,
       "e υψωμένο στον εκθέτη, μείον 1 δεύτερο; ανοίγει παρένθεση; το κλάσμα με αριθμητή; x μείον μί; και παρονομαστή σίγμα; κλείνει παρένθεση στην δεύτερη, τέλος εκθέτη")?;
  test_prefs("el", "ClearSpeak", vec![("Verbosity", "Medium"), ("ClearSpeak_Exponents", "OrdinalPower")], expr,
       "e υψωμένο στον εκθέτη, μείον 1 δεύτερο; ανοίγει παρένθεση; το κλάσμα με αριθμητή; x μείον μί; και παρονομαστή σίγμα; κλείνει παρένθεση στην δεύτερη δύναμη, τέλος εκθέτη")?;
  test_prefs("el", "ClearSpeak", vec![("Verbosity", "Medium"), ("ClearSpeak_Exponents", "AfterPower")], expr,
       "e υψωμένο στον εκθέτη, μείον 1 δεύτερο; ανοίγει παρένθεση; το κλάσμα με αριθμητή; x μείον μί; και παρονομαστή σίγμα; κλείνει παρένθεση υψωμένο στη δύναμη 2; τέλος εκθέτη")?;
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
  test("el", "ClearSpeak", expr, "t υψωμένο στην το κλάσμα με αριθμητή; b συν 1; και παρονομαστή 3; τέλος δύναμης")?;
  return Ok(());

}
