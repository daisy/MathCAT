use crate::common::*;
use anyhow::Result;

// These tests have been brought over from the Finnish 6 dot math braille. They cover comprehensive school pretty well.

// The tests are based on the Finnish specification for 6 dot math braille on the braille authority's web page (https://www.pistekirjoitus.fi/julkaisut/matematiikka-ja-tietotekniikka/) titled "Matematiikan, fysiikan ja kemain pistemerkinnät". Roughly translates to "Braille for mathematics, physics and chemistry." These tests are based on the edition published in 2022.

// Grouping numbers
// No example in the specs
#[test]
fn p7_no_grouping_in_four_digit_numbers() -> Result<()> {
    let expr = r#"<math><mn>2000</mn></math>"#;
    test_braille("ASCIIMath_fi", expr, r"2000")?;
    return Ok(());
}

#[test]
fn p7_thousands_nbsp() -> Result<()> {
    let expr = r#"<math><mn>2 000 000</mn></math>"#;
    test_braille("ASCIIMath_fi", expr, r"2.000.000")?;
    return Ok(());
}

#[test]
fn p7_thousands_point() -> Result<()> {
    let expr = r#"<math><mn>2.000.000</mn></math>"#;
    test_braille("ASCIIMath_fi", expr, r"2.000.000")?;
    return Ok(());
}

#[test]
fn p7_thousands_space() -> Result<()> {
    let expr = r#"<math><mn>2 000 000</mn></math>"#;
    test_braille("ASCIIMath_fi", expr, r"2.000.000")?;
    return Ok(());
}

#[test]
fn p7_decimal_comma() -> Result<()> {
    let expr = r#"<math><mn>5,12575</mn></math>"#;
    test_braille("ASCIIMath_fi", expr, r"5,12575")?;
    return Ok(());
}


// When there is a decimal point in the text then the grouping character is the number character.
#[test]
fn p7_decimal_point() -> Result<()> {
    let expr = r#"<math><mn>5.12575</mn></math>"#;
    test_braille("ASCIIMath_fi", expr, r"5.12575")?;
    return Ok(());
}

#[test]
fn p7_decimal_point_endless_no_grouping() -> Result<()> {
    let expr = r#"<math><mn>1,234657234</mn><mi>...</mi></math>"#;
    test_braille("ASCIIMath_fi", expr, r"1,234657234⠄⠄⠄")?;
    return Ok(());
}


#[test]
fn p7_decimal_point_endless_with_grouping_space() -> Result<()> {
    let expr = r#"<math><mn>1,234 657 234</mn><mi>...</mi></math>"#;
    test_braille("ASCIIMath_fi", expr, r"1,234.657.234⠄⠄⠄")?;
    return Ok(());
}

#[test]
fn p7_decimal_point_endless_with_grouping_nbsp() -> Result<()> {
    let expr = r#"<math><mn>1,234 657 234</mn><mi>...</mi></math>"#;
    test_braille("ASCIIMath_fi", expr, r"1,234.657.234⠄⠄⠄")?;
    return Ok(());
}

#[test]
fn p7_decimal_point_endless_with_grouping_nbsp_ellipses() -> Result<()> {
    let expr = r#"<math><mn>1,234 657 234</mn><mi>…</mi></math>"#;
    test_braille("ASCIIMath_fi", expr, r"1,234.657.234⠄⠄⠄")?;
    return Ok(());
}

#[test]
fn p7_decimal_point_endless_with_grouping_nbsp_ellipses_mathtype() -> Result<()> {
    let expr = r#"<math><mn>1</mn><mo>,</mo><mn>234</mn><mo>&#xA0;</mo><mn>657</mn><mo>&#xA0;</mo><mn>234</mn><mo>&#x2026;</mo></math>"#;
    test_braille("ASCIIMath_fi", expr, r"1,234.657.234⠄⠄⠄")?;
    return Ok(());
}

#[test]
fn p7_decimal_point_endless_no_grouping_ellipses() -> Result<()> {
    let expr = r#"<math><mn>1,234657234</mn><mi>…</mi></math>"#;
    test_braille("ASCIIMath_fi", expr, r"1,234657234⠄⠄⠄")?;
    return Ok(());
}

#[test]
fn p7_decimal_point_endless_with_grouping_space_ellipses() -> Result<()> {
    let expr = r#"<math><mn>1,234 657 234…</mn></math>"#;
    test_braille("ASCIIMath_fi", expr, r"1,234.657.234⠄⠄⠄")?;
    return Ok(());
}

#[test]
fn p9_units_percent() -> Result<()> {
    let expr = r#"<math><mn>99</mn><mi>%</mi></math>"#;
    test_braille("ASCIIMath_fi", expr, r"99 %")?;
    return Ok(());
}

#[test]
fn p9_units_permille() -> Result<()> {
    let expr = r#"<math><mn>115</mn><mi>‰</mi></math>"#;
    test_braille("ASCIIMath_fi", expr, r"115 :%")?;
    return Ok(());
}

#[test]
fn p9_units_degrees() -> Result<()> {
    let expr = r#"<math><mn>100</mn><mi>˚</mi></math>"#;
    test_braille("ASCIIMath_fi", expr, r"100 ^@")?;
    return Ok(());
}

#[test]
fn p9_units_degrees_celsius() -> Result<()> {
    let expr = r#"<math><mn>37</mn><mi>˚</mi><mi>C</mi></math>"#;
    test_braille("ASCIIMath_fi", expr, r"37 ^@C")?;
    return Ok(());
}

#[test]
fn p9_units_degrees_fahrenheit() -> Result<()> {
    let expr = r#"<math><mn>−43</mn><mi>˚</mi><mi>F</mi></math>"#;
    test_braille("ASCIIMath_fi", expr, r"-43 ^@F")?;
    return Ok(());
}

#[test]
fn p10_currency_euro() -> Result<()> {
    let expr = r#"<math><mn>6</mn><mi>€</mi><mn>15</mn><mi>snt</mi></math>"#;
    test_braille("ASCIIMath_fi", expr, r"6 e 15 snt")?;
    return Ok(());
}

#[test]
fn p10_currency_dollar() -> Result<()> {
    let expr = r#"<math><mi>$</mi><mn>35</mn></math>"#;
    test_braille("ASCIIMath_fi", expr, r"$35")?;
    return Ok(());
}

#[test]
fn p10_currency_pound() -> Result<()> {
    let expr = r#"<math><mi>£</mi><mn>25</mn></math>"#;
    test_braille("ASCIIMath_fi", expr, r"£25")?;
    return Ok(());
}

#[test]
fn p11_mm_squared() -> Result<()> {
    let expr = r#"<math><msup><mi>mm</mi><mn>2</mn></msup></math>"#;
    test_braille("ASCIIMath_fi", expr, r"mm^2")?;
    return Ok(());
}

#[test]
fn p11_cm_cubed() -> Result<()> {
    let expr = r#"<math><msup><mi>cm</mi><mn>3</mn></msup></math>"#;
    test_braille("ASCIIMath_fi", expr, r"cm^3")?;
    return Ok(());
}

// Question mark is the same as in literary text.
#[test]
fn p13_times_with_question_mark() -> Result<()> {
    let expr = r#"<math><mn>27</mn><mo>·</mo><mn>3</mn><mo>=</mo><mi>?</mi></math>"#;
    test_braille("ASCIIMath_fi", expr, r"27 *3 =?")?;
    return Ok(());
}

#[test]
fn p13_parentheses_invisible_times() -> Result<()> {
    let expr = r#"<math><mo>(</mo><mn>3</mn><mo>&#8290;</mo><mi>x</mi><mo>+</mo><mn>2</mn><mo>&#8290;</mo><mi>y</mi><mo>)</mo>
                    <mo>&#8290;</mo><mo>(</mo><mn>2</mn><mo>&#8290;</mo><mi>x</mi><mo>−</mo><mn>1</mn><mo>)</mo></math>"#;
    test_braille("ASCIIMath_fi", expr, r"(3 x +2 y) (2 x -1)")?;
    return Ok(());
}

// No space required between variables.
#[test]
fn greater_than_with_more_variables() -> Result<()> {
    let expr = r#"<math>
    <mn>2</mn>
    <mi>x</mi>
    <mi>y</mi>
    <mo>&gt;</mo>
    <mn>6</mn>
</math>"#;
    test_braille("ASCIIMath_fi", expr, r"2 xy > 6")?;
    return Ok(());
}

#[test]
fn p15_greater_than_or_equal_to() -> Result<()> {
    let expr = r#"<math>
    <mn>2</mn>
    <mi>x</mi>
    <mo>≥</mo>
    <mn>6</mn>
</math>"#;
    test_braille("ASCIIMath_fi", expr, r"2 x >= 6")?;
    return Ok(());
}

#[test]
fn p15_less_than_or_equal_to() -> Result<()> {
    let expr = r#"<math>
    <mi>x</mi>
    <mo>≤</mo>
    <mn>18</mn>
</math>"#;
    test_braille("ASCIIMath_fi", expr, r"x <= 18")?;
    return Ok(());
}

#[test]
fn p16_division() -> Result<()> {
    let expr = r#"<math>
    <mo>(</mo>
    <mn>6</mn>
    <mi>x</mi>
    <mo>+</mo>
    <mn>3</mn>
    <mi>x</mi>
    <mo>)</mo>
    <mo>:</mo>
    <mn>3</mn>
</math>"#;
    test_braille("ASCIIMath_fi", expr, r"(6 x +3 x) :3")?;
    return Ok(());
}

#[test]
fn p16_fractional() -> Result<()> {
    let expr = r#"<math>
  <mfrac>
    <mrow>
      <mn>6</mn>
      <mi>x</mi>
      <mo>+</mo>
      <mn>3</mn>
      <mi>x</mi>
    </mrow>
    <mn>3</mn>
  </mfrac>
</math>"#;
    test_braille("ASCIIMath_fi", expr, r"(6 x +3 x) /3")?;
    return Ok(());
}

#[test]
fn p16_absolute_value() -> Result<()> {
    let expr = r#"<math>
      <mrow>
        <mrow>
          <mo>|</mo>
          <mrow>
            <mo>-</mo>
            <mrow><mo>(</mo><mrow><mn>2</mn><mo>+</mo><mn>5</mn></mrow><mo>)</mo></mrow>
          </mrow>
          <mo>|</mo>
        </mrow>
        <mo>=</mo>
        <mrow><mo>|</mo><mrow><mo>-</mo><mn>7</mn></mrow><mo>|</mo></mrow>
        <mo>=</mo>
        <mn>7</mn>
      </mrow>
    </math>"#;
    test_braille("ASCIIMath_fi", expr, r"|-(2 +5)| =|-7| =7")?;
    return Ok(());
}

#[test]
fn p16_parens_and_brackets() -> Result<()> {
    let expr = r#"<math>
  <mrow>
    <mn>92</mn>
    <mo>+</mo>
    <mo>[</mo>
    <mo>−</mo>
    <mn>2</mn>
    <mo>⋅</mo>
    <mo>(</mo>
    <mn>18</mn>
    <mo>+</mo>
    <mn>5</mn>
    <mo>)</mo>
    <mo>]</mo>
  </mrow>
</math>"#;
    test_braille("ASCIIMath_fi", expr, r"92 +[-2 *(18 +5)]")?;
    return Ok(());
}

#[test]
fn p17_multiple_operations() -> Result<()> {
    let expr = r#"<math>
    <mn>5</mn>
    <mo>⋅</mo>
    <msup>
      <mn>3</mn>
      <mn>2</mn>
    </msup>
    <mo>+</mo>
    <mo>(</mo>
    <mn>81</mn>
    <mo>−</mo>
    <mn>60</mn>
    <mo>)</mo>
    <mo>:</mo>
    <mn>3</mn>
    <mo>−</mo>
    <mo>(</mo>
    <mn>2</mn>
    <mo>+</mo>
    <mn>4</mn>
    <msup>
      <mo>)</mo>
      <mn>2</mn>
    </msup>
</math>"#;
    test_braille("ASCIIMath_fi", expr, r"5 *3^2 +(81 -60) :3 -(2 +4)^2")?;
    return Ok(());
}

#[test]
fn p19_fractions_sum() -> Result<()> {
    let expr = r#"<math>
  <mrow>
    <mfrac>
      <mn>3</mn>
      <mn>4</mn>
    </mfrac>
    <mo>+</mo>
    <mfrac>
      <mn>1</mn>
      <mn>4</mn>
    </mfrac>
    <mo>=</mo>
    <mn>1</mn>
  </mrow>
</math>"#;
    test_braille("ASCIIMath_fi", expr, r"3/4 +1/4 =1")?;
    return Ok(());
}

#[test]
fn p19_fractions_mixed_operations() -> Result<()> {
    let expr = r#"<math>
  <mrow>
    <mo>(</mo>
    <mfrac>
      <mn>3</mn>
      <mn>4</mn>
    </mfrac>
    <mo>+</mo>
    <mfrac>
      <mn>5</mn>
      <mn>6</mn>
    </mfrac>
    <mo>)</mo>
    <mo>−</mo>
    <mo>(</mo>
    <mfrac>
      <mn>3</mn>
      <mn>5</mn>
    </mfrac>
    <mo>+</mo>
    <mfrac>
      <mn>1</mn>
      <mn>2</mn>
    </mfrac>
    <mo>)</mo>
  </mrow>
</math>"#;
    test_braille("ASCIIMath_fi", expr, r"(3/4 +5/6) -(3/5 +1/2)")?;
    return Ok(());
}

// This is Finnish notation where the numerator and denomator is multiplied by the same expression that is in the pre-supercript. Structure is always: **[multiplier]
#[test]
fn p19_fraction_multiplication_Finnish_notation_multiscripts() -> Result<()> {
    let expr = r#"<math>
  <mrow>
    <mmultiscripts>
      <mfrac>
        <mi>a</mi>
        <mi>c</mi>
      </mfrac>
      <mprescripts></mprescripts>
      <none></none>
      <mrow>
        <mi>b</mi>
        <mo>)</mo>
      </mrow>
    </mmultiscripts>
    <mo>=</mo>
    <mfrac>
      <mrow>
        <mi>a</mi>
        <mi>b</mi>
      </mrow>
      <mrow>
        <mi>b</mi>
        <mi>c</mi>
      </mrow>
    </mfrac>
  </mrow>
</math>"#;
    test_braille("ASCIIMath_fi", expr, r"**b a/c =ab/(bc)")?;
    return Ok(());
}

// This is Finnish notation where the numerator and denomator is multiplied by the same expression that is in the pre-supercript. Structure is always: **[multiplier]
#[test]
fn p19_fraction_multiplication_Finnish_notation_multiscripts_2() -> Result<()> {
    let expr = r#"<math>
  <mrow>
    <mmultiscripts>
      <mfrac>
        <mn>1</mn>
        <mn>2</mn>
      </mfrac>
      <mprescripts></mprescripts>
      <none></none>
      <mrow>
        <mn>6</mn>
        <mo>)</mo>
      </mrow>
    </mmultiscripts>
  </mrow>
</math>"#;
    test_braille("ASCIIMath_fi", expr, r"**6 1/2")?;
    return Ok(());
}

// This is Finnish notation for dividing the numerator and denominator by the same expression. It is always of the form: ::[divisor]. The notation comes after the fraction.
#[test]
fn p22_fraction_division_Finnish_notation_msup() -> Result<()> {
    let expr = r#"<math>
  <mrow>
    <msup>
      <mfrac>
        <mrow>
          <mi>a</mi>
          <mi>b</mi>
          <mi>c</mi>
        </mrow>
        <mrow>
          <mi>a</mi>
          <mi>d</mi>
        </mrow>
      </mfrac>
      <mrow>
        <mo>(</mo>
        <mi>a</mi>
      </mrow>
    </msup>
    <mo>=</mo>
    <mfrac>
      <mrow>
        <mi>b</mi>
        <mi>c</mi>
      </mrow>
      <mi>d</mi>
    </mfrac>
  </mrow>
</math>"#;
    test_braille("ASCIIMath_fi", expr, r"abc/(ad) ::a =bc/d")?;
    return Ok(());
}

// This is Finnish notation for dividing the numerator and denominator by the same expression. It is always of the form: ::[divisor]. The notation comes after the fraction.
#[test]
fn p22_fraction_division_Finnish_notation_msup_2() -> Result<()> {
    let expr = r#"<math>
  <msup>
    <mfrac>
      <mn>2</mn>
      <mn>4</mn>
    </mfrac>
    <mrow>
      <mo>(</mo>
      <mn>2</mn>
    </mrow>
  </msup>
</math>"#;
    test_braille("ASCIIMath_fi", expr, r"2/4 ::2")?;
    return Ok(());
}

// It could possible to have the same structure as a mmultiscript as before, so here is a test for that. Output is the same as before.
#[test]
fn p22_fraction_division_Finnish_notation_mmultiscripts() -> Result<()> {
    let expr = r#"<math>
  <mmultiscripts>
    <mfrac>
      <mn>2</mn>
      <mn>4</mn>
    </mfrac>
    <mrow></mrow>
    <mrow>
      <mo>(</mo>
      <mn>2</mn>
    </mrow>
  </mmultiscripts>
</math>"#;
    test_braille("ASCIIMath_fi", expr, r"2/4 ::2")?;
    return Ok(());
}

#[test]
fn p23_equation_1_no_invisible_multiplication() -> Result<()> {
    let expr = r#"<math>
    <mn>2</mn>
    <mi>x</mi>
    <mo>+</mo>
    <mn>3</mn>
    <mo>=</mo>
    <mo>−</mo>
    <mn>3</mn>
    <mi>x</mi>
    <mo>−</mo>
    <mn>7</mn>
</math>"#;
    test_braille("ASCIIMath_fi", expr, r"2 x +3 =-3 x -7")?;
    return Ok(());
}

#[test]
fn p23_equation_1_with_invisible_multiplication() -> Result<()> {
    let expr = r#"<math>
    <mn>2</mn>
    <mo>&#8290;</mo>
    <mi>x</mi>
    <mo>+</mo>
    <mn>3</mn>
    <mo>=</mo>
    <mo>−</mo>
    <mn>3</mn>
    <mo>&#8290;</mo>
    <mi>x</mi>
    <mo>−</mo>
    <mn>7</mn>
</math>"#;
    test_braille("ASCIIMath_fi", expr, r"2 x +3 =-3 x -7")?;
    return Ok(());
}

// This is Finnish notation. The bar | is used to denote that an operation is done on both sides of the equation.
// Here the equation is divided by five on both sides.
#[test]
fn p22_equation_solving_notation() -> Result<()> {
    let expr = r#"<math>
    <mn>5</mn>
    <mi>x</mi>
    <mo>=</mo>
    <mo>−</mo>
    <mn>10</mn>
    <mo>|</mo>
    <mo>:</mo>
    <mn>5</mn>
</math>"#;
    test_braille("ASCIIMath_fi", expr, r"5 x =-10 | :5")?; 
    
    return Ok(());
}

// This is Finnish notation. The bar | is used to denote that an operation is done on both sides of the equation. This is not a real life example, but a string of mark up that should produce the output in the test: [space][vertical bar][space][operation].
#[test]
fn operate_by_on_both_sides() -> Result<()> {
    let expr = r#"<math>
    <mo>|</mo>
    <mo>+</mo>
    <mn>5</mn>
    <mo>|</mo>
    <mo>-</mo>
    <mn>5</mn>
    <mo>|</mo>
    <mo>−</mo>
    <mn>5</mn>
    <mo>|</mo>
    <mo>·</mo>
    <mn>5</mn>
    <mo>|</mo>
    <mo>×</mo>
    <mn>5</mn>
    <mo>|</mo>
    <msup>
    <mrow>
    <mo>(</mo>
    <mo>)</mo>
    </mrow>
    <mn>2</mn>
    </msup>
    <mo>|</mo>
    <mo>/</mo>
    <mn>5</mn>
</math>"#;
    test_braille("ASCIIMath_fi", expr, r" | +5 | -5 | *5 | xx 5 | ()^2 | /5")?;
    return Ok(());
  }

// Sometimes also two bars || are used.
#[test]
fn p22_equation_solving_notation_double_bars() -> Result<()> {
    let expr = r#"<math>
    <mn>5</mn>
    <mi>x</mi>
    <mo>=</mo>
    <mo>−</mo>
    <mn>10</mn>
    <mo>|</mo>
    <mo>|</mo>
    <mo>:</mo>
    <mn>5</mn>
</math>"#;
    test_braille("ASCIIMath_fi", expr, r"5 x =-10 || :5")?;
    
    return Ok(());
}

// Sometimes also two bars || are used. This is not a real life example, but a string of mark up that should produce the output in the test: [space][vertical bar][vertical bar][operation done on both sides].
#[test]
fn operate_by_on_both_sides_double_bars() -> Result<()> {
    let expr = r#"<math>
    <mo>|</mo>
    <mo>|</mo>
    <mo>+</mo>
    <mn>5</mn>
    <mo>|</mo>
    <mo>|</mo>
    <mo>-</mo>
    <mn>5</mn>
    <mo>|</mo>
    <mo>|</mo>
    <mo>−</mo>
    <mn>5</mn>
    <mo>|</mo>
    <mo>|</mo>
    <mo>·</mo>
    <mn>5</mn>
    <mo>|</mo>
    <mo>|</mo>
    <mo>×</mo>
    <mn>5</mn>
    <mo>|</mo>
    <mo>|</mo>
    <msup>
    <mrow>
    <mo>(</mo>
    <mo>)</mo>
    </mrow>
    <mn>2</mn>
    </msup>
    <mo>|</mo>
    <mo>|</mo>
    <mo>/</mo>
    <mn>5</mn>
</math>"#;
    test_braille("ASCIIMath_fi", expr, r" || +5 || -5 || *5 || xx 5 || ()^2 || /5")?;
    return Ok(());
  }

#[test]
fn p24_fraction() -> Result<()> {
    let expr = r#"<math>
  <mfrac>
    <mrow>
      <mn>5</mn>
      <mo>+</mo>
      <mn>7</mn>
    </mrow>
    <mrow>
      <mn>2</mn>
      <mo>⋅</mo>
      <mn>3</mn>
    </mrow>
  </mfrac>
</math>"#;
    test_braille("ASCIIMath_fi", expr, r"(2 +7) /(2 *3)")?;
    return Ok(());
}

#[test]
fn p24_fraction_2() -> Result<()> {
    let expr = r#"<math>
  <mfrac>
    <mrow>
      <mn>4</mn>
      <mo>&#8290;</mo>
      <mi>x</mi>
    </mrow>
    <mrow>
      <mn>6</mn>
      <mo>&#8290;</mo>
      <mo>(</mo>
      <mn>1</mn>
      <mo>−</mo>
      <mi>x</mi>
      <mo>)</mo>
    </mrow>
  </mfrac>
</math>"#;
    test_braille("ASCIIMath_fi", expr, r"4 x /(6 (1 -x))")?;
    return Ok(());
}

// Rule fix: Fractions' division requires double parentheses so that the order of operations is respected.
#[test]
fn p25_long_fraction() -> Result<()> {
    let expr = r#"<math>
  <mrow>
    <mfrac>
      <mrow>
        <msup>
          <mi>x</mi>
          <mn>2</mn>
        </msup>
        <mo>−</mo>
        <mn>7</mn>
        <mi>x</mi>
        <mo>+</mo>
        <mn>12</mn>
      </mrow>
      <mrow>
        <mn>4</mn>
        <mi>x</mi>
        <mo>−</mo>
        <mn>20</mn>
      </mrow>
    </mfrac>
    <mo>:</mo>
    <mfrac>
      <mrow>
        <msup>
          <mi>x</mi>
          <mn>2</mn>
        </msup>
        <mo>−</mo>
        <mn>8</mn>
        <mi>x</mi>
        <mo>+</mo>
        <mn>15</mn>
      </mrow>
      <mrow>
        <mn>4</mn>
        <mi>x</mi>
        <mo>−</mo>
        <mn>16</mn>
      </mrow>
    </mfrac>
  </mrow>
</math>"#;
    test_braille("ASCIIMath_fi", expr, r"((x^2 -7 x +12) /(4 x -20)) / ((x^2 -8 x +15) /(4 x -16))")?;
    return Ok(());
}

#[test]
fn p25_fraction_in_fraction() -> Result<()> {
    let expr = r#"<math>
  <mfrac>
    <mrow>
      <mfrac>
        <mn>1</mn>
        <mn>2</mn>
      </mfrac>
      <mo>+</mo>
      <mfrac>
        <mn>1</mn>
        <mn>3</mn>
      </mfrac>
    </mrow>
    <mrow>
      <mfrac>
        <mn>1</mn>
        <mn>4</mn>
      </mfrac>
      <mo>−</mo>
      <mfrac>
        <mn>1</mn>
        <mn>5</mn>
      </mfrac>
    </mrow>
  </mfrac>
</math>"#;
    test_braille("ASCIIMath_fi", expr, r"(1/2 +1/3) /(1/4 -1/5)")?;
    return Ok(());
}

#[test]
fn p26_exponent_multiplication_with_parens() -> Result<()> {
    let expr = r#"<math>
  <mrow>
    <msup>
      <mn>2</mn>
      <mn>3</mn>
    </msup>
    <mo>⋅</mo>
    <msup>
      <mn>2</mn>
      <mn>5</mn>
    </msup>
    <mo>=</mo>
    <msup>
      <mn>2</mn>
      <mrow>
        <mo>(</mo>
        <mn>3</mn>
        <mo>+</mo>
        <mn>5</mn>
        <mo>)</mo>
      </mrow>
    </msup>
    <mo>=</mo>
    <msup>
      <mn>2</mn>
      <mn>8</mn>
    </msup>
  </mrow>
</math>"#;
    test_braille("ASCIIMath_fi", expr, r"2^3 *2^5 =2^(3 +5) =2^8")?;
    return Ok(());
}

#[test]
fn p26_exponent_multiplication_no_parens() -> Result<()> {
    let expr = r#"<math>
  <mrow>
    <msup>
      <mn>2</mn>
      <mn>3</mn>
    </msup>
    <mo>⋅</mo>
    <msup>
      <mn>2</mn>
      <mn>5</mn>
    </msup>
    <mo>=</mo>
    <msup>
      <mn>2</mn>
      <mrow>
        <mn>3</mn>
        <mo>+</mo>
        <mn>5</mn>
      </mrow>
    </msup>
    <mo>=</mo>
    <msup>
      <mn>2</mn>
      <mn>8</mn>
    </msup>
  </mrow>
</math>"#;
    test_braille("ASCIIMath_fi", expr, r"2^3 *2^5 =2^(3 +5) =2^8")?;
    return Ok(());
}

#[test]
fn p27_exponent_to_large_number() -> Result<()> {
    let expr = r#"<math>
  <mrow>
    <msup>
      <mn>2</mn>
      <mn>20</mn>
    </msup>
    <mo>=</mo>
    <mn>1048576</mn>
  </mrow>
</math>"#;
    test_braille("ASCIIMath_fi", expr, r"2^20 =1.048.576")?;
    return Ok(());
}

#[test]
fn p27_sqrt() -> Result<()> {
    let expr = r#"<math>
  <mrow>
    <msup>
      <mn>4</mn>
      <mfrac>
        <mn>1</mn>
        <mn>2</mn>
      </mfrac>
    </msup>
    <mo>=</mo>
    <msqrt>
      <mn>4</mn>
    </msqrt>
    <mo>=</mo>
    <mn>2</mn>
  </mrow>
</math>"#;
    test_braille("ASCIIMath_fi", expr, r"4^(1/2) =sqrt(4) =2")?;
    return Ok(());
}

#[test]
fn p31_trig_tan_infinity() -> Result<()> {
    let expr = r#"<math>
      <mi>tan</mi>
      <mo>⁡</mo>
      <mn>90</mn>
      <mi>°</mi>
      <mo>=</mo>
      <mi>∞</mi>
</math>"#;
    test_braille("ASCIIMath_fi", expr, r"tan(90 ^@) =oo")?;
    return Ok(());
}

// The degree sign ^@ has a whitespace before it, but the ' or '' don't.
#[test]
fn p31_degrees_minutes_seconds_primes() -> Result<()> {
    let expr = r#"<math>
  <mrow>
    <mn>30</mn>
    <mi>°</mi>
    <msup>
      <mn>45</mn>
      <mo>′</mo>
    </msup>
    <msup>
      <mn>20</mn>
      <mrow>
        <mo>′′</mo>
      </mrow>
    </msup>
  </mrow>
</math>"#;
    test_braille("ASCIIMath_fi", expr, r"30 ^@ 45' 20''")?;
    return Ok(());
}

// The degree sign ^@ has a whitespace before it, but the ' or '' don't.
#[test]
fn p31_degrees_minutes_seconds_alternative() -> Result<()> {
    let expr = r#"<math>
  <mrow>
    <mn>30</mn>
    <mi>°</mi>
    <msup>
      <mn>45</mn>
      <mo>'</mo>
    </msup>
    <msup>
      <mn>20</mn>
      <mrow>
        <mo>''</mo>
      </mrow>
    </msup>
  </mrow>
</math>"#;
    test_braille("ASCIIMath_fi", expr, r"30 ^@ 45' 20''")?;
    return Ok(());
}

#[test]
fn msub_singular() -> Result<()> {
  init_logger();
    let expr = r#"<math>
    <msub>
    <mi>x</mi>
    <mn>1</mn>
    </msub>
    <mo>+</mo>
    <mn>1</mn>
</math>"#;
    test_braille("ASCIIMath_fi", expr, r"x_1 +1")?;
    return Ok(());
}

#[test]
fn msub_parentheses() -> Result<()> {
    let expr = r#"<math>
    <msub>
    <mi>a</mi>
    <mrow>
    <mi>x</mi>
    <mo>+</mo>
    <mn>1</mn>
    </mrow>
    </msub>
    <mo>+</mo>
    <mn>1</mn>
</math>"#;
    test_braille("ASCIIMath_fi", expr, r"a_(x +1) +1")?;
    return Ok(());
}

// Overline denotes a vector here.
#[test]
fn msub_with_vector() -> Result<()> {
    let expr = r#"<math>
      <msub>
      <mover>
        <mi>a</mi>
        <mo>¯</mo>
      </mover>
        <mi>b</mi>
      </msub>
    <mo>+</mo>
    <mn>1</mn>
</math>"#;
    test_braille("ASCIIMath_fi", expr, r"vec a_b +1")?;
    return Ok(());
}

// Even when the vector is marked with an arrow, it should output the same as vec.
#[test]
fn msub_with_vector_arrow() -> Result<()> {
    let expr = r#"<math>
      <msub>
      <mover>
        <mi>a</mi>
        <mo>→</mo>
      </mover>
        <mi>b</mi>
      </msub>
    <mo>+</mo>
    <mn>1</mn>
</math>"#;
    test_braille("ASCIIMath_fi", expr, r"vec a_b +1")?;
    return Ok(());
}

#[test]
fn msub_with_vector_with_sub_text() -> Result<()> {
    let expr = r#"<math>
  <mrow data-changed='added'>
      <msub>
        <mover>
          <mi>a</mi>
          <mo>¯</mo>
        </mover>
        <mtext>tuki</mtext>
      </msub>
    <mo>+</mo>
    <mn>1</mn>
  </mrow>
 </math>"#;
    test_braille("ASCIIMath_fi", expr, r"vec a_tuki +1")?;
    return Ok(());
}

#[test]
fn msubsup_with_vector() -> Result<()> {
    let expr = r#"<math>
  <mrow>
      <msubsup>
        <mover>
          <mi>a</mi>
          <mo>¯</mo>
        </mover>
        <mn>5</mn>
        <mn>0</mn>
      </msubsup>
    <mo>+</mo>
    <mn>1</mn>
  </mrow>
 </math>"#;
    test_braille("ASCIIMath_fi", expr, r"vec a_5^0 +1")?;
    return Ok(());
}

#[test]
fn msubsup_with_vector_complex_with_parentheses() -> Result<()> {
    let expr = r#"<math>
  <mrow>
      <msubsup>
        <mover>
          <mi>a</mi>
          <mo>¯</mo>
        </mover>
        <mrow>
          <mo>(</mo>
          <mn>5</mn>
          <mo>+</mo>
          <mn>2</mn>
          <mo>)</mo>
        </mrow>
        <mrow>
          <mo>(</mo>
          <mn>3</mn>
          <mo>+</mo>
          <mi>x</mi>
          <mo>)</mo>
        </mrow>
      </msubsup>
    <mo>+</mo>
    <mn>1</mn>
  </mrow>
 </math>"#;
    test_braille("ASCIIMath_fi", expr, r"vec a_(5 +2)^(3 +x) +1")?;
    return Ok(());
}

#[test]
fn msubsup() -> Result<()> {
    let expr = r#"<math>
    <msubsup>
    <mi>a</mi>
    <mn>1</mn>
    <mn>2</mn>
    </msubsup>
    <mo>+</mo>
    <mn>1</mn>
</math>"#;
    test_braille("ASCIIMath_fi", expr, r"a_1^2 +1")?;
    return Ok(());
}

#[test]
fn msubsup_parentheses_in_sub_and_sup() -> Result<()> {
    let expr = r#"<math>
    <msubsup>
    <mi>a</mi>
    <mrow>
    <mi>x</mi>
    <mo>+</mo>
    <mn>1</mn>
    </mrow>
    <mrow>
    <mi>x</mi>
    <mo>−</mo>
    <mn>10</mn>
    </mrow>
    </msubsup>
    <mo>+</mo>
    <mn>1</mn>
</math>"#;
    test_braille("ASCIIMath_fi", expr, r"a_(x +1)^(x -10) +1")?;
    return Ok(());
}

#[test]
fn p32_expected_value() -> Result<()> {
    let expr = r#"<math>
    <mi>E</mi>
    <mo>(</mo>
    <mi>x</mi>
    <mo>)</mo>
    <mo>=</mo>
    <mi>μ</mi>
    <mo>=</mo>
    <msub>
      <mo>∑</mo>
      <mi>i</mi>
    </msub>
    <msub>
      <mi>p</mi>
      <mi>i</mi>
    </msub>
    <msub>
      <mi>x</mi>
      <mi>i</mi>
    </msub>
</math>"#;
    test_braille("ASCIIMath_fi", expr, r"E(x) =~m =sum_i (p_i x_i)")?;
    return Ok(());
}

#[test]
fn p33_vector() -> Result<()> {
    let expr = r#"<math>
  <mrow>
    <mover>
        <mi>AB</mi>
        <mo>‾</mo>
    </mover>
    <mo>=</mo>
    <mn>3</mn>
    <mover>
      <mi>i</mi>
      <mo>‾</mo>
    </mover>
    <mo>−</mo>
    <mn>4</mn>
    <mover>
      <mi>j</mi>
      <mo>‾</mo>
    </mover>
  </mrow>
</math>"#;
    test_braille("ASCIIMath_fi", expr, r"vec AB =3 vec i -4 vec j")?;
    return Ok(());
}

#[test]
fn p33_parallel_vectors() -> Result<()> {
    let expr = r#"<math>
  <mrow>
    <mover>
      <mi>a</mi>
      <mo>‾</mo>
    </mover>
    <mo>∥</mo>
    <mover>
      <mi>b</mi>
      <mo>‾</mo>
    </mover>
  </mrow>
</math>"#;
    test_braille("ASCIIMath_fi", expr, r"vec a || vec b")?;
    return Ok(());
}

// Dot product is denoted with the center dot (multiplication), but it has whitespace on both sides of the operation.
#[test]
fn p34_vector_dot_product() -> Result<()> {
    let expr = r#"<math>
  <mrow>
    <mover>
      <mi>v</mi>
      <mo>‾</mo>
    </mover>
    <mo>⋅</mo>
    <mover>
      <mi>u</mi>
      <mo>‾</mo>
    </mover>
  </mrow>
</math>"#;
    test_braille("ASCIIMath_fi", expr, r"vec v * vec u")?;
    return Ok(());
}

#[test]
fn p34_vector_cross_product() -> Result<()> {
    let expr = r#"<math>
  <mrow>
    <mover>
      <mi>a</mi>
      <mo>‾</mo>
    </mover>
      <mo>×</mo>
    <mover>
      <mi>b</mi>
      <mo>‾</mo>
    </mover>
  </mrow>
</math>"#;
    test_braille("ASCIIMath_fi", expr, r"vec a xx vec b")?;
    return Ok(());
}

#[test]
fn p39_not_in_set() -> Result<()> {
    let expr = r#"<math>
    <mn>3</mn>
    <mo>∉</mo>
    <mi>B</mi>
</math>"#;
    test_braille("ASCIIMath_fi", expr, r"3 !in B")?;
    return Ok(());
}

#[test]
fn p39_subset() -> Result<()> {
    let expr = r#"<math>
  <mrow>
    <mi>B</mi>
    <mo>⊂</mo>
    <mi>A</mi>
  </mrow>
</math>"#;
    test_braille("ASCIIMath_fi", expr, r"B sube A")?;
    return Ok(());
}

#[test]
fn p39_not_subset() -> Result<()> {
    let expr = r#"<math>
    <mi>F</mi>
    <mo>⊄</mo>
    <mi>E</mi>
</math>"#;
    test_braille("ASCIIMath_fi", expr, r"F !sub E")?;
    return Ok(());
}

#[test]
fn p39_equal_sets_with_mtext() -> Result<()> {
    let expr = r#"<math>
  <mrow>
    <mi>C</mi>
    <mo>⊆</mo>
    <mi>D</mi>
    <mtext> ja </mtext>
    <mi>D</mi>
    <mo>⊆</mo>
    <mi>C</mi>
    <mo>⇔</mo>
    <mi>C</mi>
    <mo>=</mo>
    <mi>D</mi>
  </mrow>
</math>"#;
    test_braille("ASCIIMath_fi", expr, r"C sube D ja D sube C <--> C =D")?;
    return Ok(());
}

#[test]
fn p39_set_of_points_in_R2() -> Result<()> {
    let expr = r#"<math>
  <mrow>
    <mi>A</mi>
    <mo>×</mo>
    <mi>B</mi>
    <mo>=</mo>
    <mo>{</mo>
    <mo>(</mo>
    <mn>1</mn>
    <mo>,</mo>
    <mn>3</mn>
    <mo>)</mo>
    <mo>,</mo>
    <mo>(</mo>
    <mn>1</mn>
    <mo>,</mo>
    <mn>4</mn>
    <mo>)</mo>
    <mo>,</mo>
    <mo>(</mo>
    <mn>2</mn>
    <mo>,</mo>
    <mn>3</mn>
    <mo>)</mo>
    <mo>,</mo>
    <mo>(</mo>
    <mn>2</mn>
    <mo>,</mo>
    <mn>4</mn>
    <mo>)</mo>
    <mo>}</mo>
  </mrow>
</math>"#;
    test_braille("ASCIIMath_fi", expr, r"A xx B ={(1, 3), (1, 4), (2, 3), (2, 4)}")?;
    return Ok(());
}

#[test]
fn p40_not_p() -> Result<()> {
    let expr = r#"<math>
    <mo>¬</mo>
    <mi>p</mi>
</math>"#;
    test_braille("ASCIIMath_fi", expr, r"not p")?;
    return Ok(());
}

#[test]
fn p40_p_and_q() -> Result<()> {
    let expr = r#"<math>
    <mi>p</mi>
    <mo>∧</mo>
    <mi>q</mi>
</math>"#;
    test_braille("ASCIIMath_fi", expr, r"p ^^ q")?;
    return Ok(());
}

#[test]
fn p40_p_or_q() -> Result<()> {
    let expr = r#"<math>
    <mi>p</mi>
    <mo>∨</mo>
    <mi>q</mi>
</math>"#;
    test_braille("ASCIIMath_fi", expr, r"p vv q")?;
    return Ok(());
}

// The colon : should be written directly after the function f (and similar). This is different compared to it being used as a division sign and as a ratio.
#[test]
fn p41_function_definition() -> Result<()> {
    let expr = r#"<math>
    <mi>f</mi>
    <mo>:</mo>
    <mi>x</mi>
    <mo>→</mo>
    <mi>f</mi>
    <mo>(</mo>
    <mi>x</mi>
    <mo>)</mo>
</math>"#;
    test_braille("ASCIIMath_fi", expr, r"f: x -> f(x)")?;
    return Ok(());
}

// Here the bar | should have whitespace on both sides of the character.
#[test]
fn p41_inverse_function() -> Result<()> {
    let expr = r#"<math>
    <msup>
      <mi>f</mi>
      <mrow>
        <mo>−</mo>
        <mn>1</mn>
      </mrow>
    </msup>
    <mo>=</mo>
  <mrow>
    <mo>{</mo>
    <mo>(</mo>
    <mi>y</mi>
    <mo>,</mo>
    <mi>x</mi>
    <mo>)</mo>
    <mo>∈</mo>
    <mi>B</mi>
    <mo>×</mo>
    <mi>A</mi>
    <mo>|</mo>
    <mi>y</mi>
    <mo>=</mo>
    <mi>f</mi>
    <mo>(</mo>
    <mi>x</mi>
    <mo>)</mo>
    <mo>}</mo>
  </mrow>
</math>"#;
    test_braille("ASCIIMath_fi", expr, r"f^-1 ={(y, x) in (B xx A) | y =f(x)}")?;
    return Ok(());
}


#[test]
fn p41_function_with_parts_with_arrow() -> Result<()> {
    let expr = r#"<math>
      <mi>f</mi>
      <mo>:</mo>
      <mi>x</mi>
      <mo>→</mo>
      <mrow>
         <mo>{</mo>
         <mrow>
            <mtable>
               <mtr>
                  <mtd>
                     <mrow>
                        <mn>&#x2212;1</mn>
                        <mo>,</mo>
                        <mi>x</mi>
                        <mo>≤</mo>
                        <mn>&#x2212;1</mn>
                     </mrow>
                  </mtd>
               </mtr>
               <mtr>
                  <mtd>
                     <mrow>
                        <msup>
                           <mi>x</mi>
                           <mn>2</mn>
                        </msup>
                        <mo>+</mo>
                        <mn>2</mn>
                        <mi>x</mi>
                        <mo>,</mo>
                        <mn>&#x2212;1</mn>
                        <mo>&lt;</mo>
                        <mi>x</mi>
                        <mo>≤</mo>
                        <mn>1</mn>
                     </mrow>
                  </mtd>
               </mtr>
               <mtr>
                  <mtd>
                     <mrow>
                        <mo>&#x2212;</mo>
                        <mi>x</mi>
                        <mo>+</mo>
                        <mn>3</mn>
                        <mo>,</mo>
                        <mi>x</mi>
                        <mo>&#x003E;</mo>
                        <mn>1</mn>
                     </mrow>
                  </mtd>
               </mtr>
            </mtable>
         </mrow>
      </mrow>
</math>
"#;
    test_braille("ASCIIMath_fi", expr, r"⠋⠒⠀⠭⠀⠤⠱⠀⠫⠤⠼⠁⠂⠀⠭⠀⠣⠶⠀⠤⠼⠁⠀⠐⠐⠀⠭⠬⠀⠖⠼⠃⠀⠭⠂⠀⠤⠼⠁⠀⠣⠀⠭⠀⠣⠶⠀⠼⠁⠀⠐⠐⠀⠤⠭⠀⠖⠼⠉⠂⠀⠭⠀⠱⠀⠼⠁⠻")?;
    return Ok(());
}

// This notation differs from the actual specification and resembles how matrices are marked up.
// Correct ouput would require a multiline braille display.
#[test]
fn p40_function_with_parts() -> Result<()> {
  let expr = r#"<math>
      <mi>f</mi>
      <mo>(</mo>
      <mi>x</mi>
      <mo>)</mo>
      <mo>=</mo>
      <mrow>
         <mo>{</mo>
         <mrow>
            <mtable>
               <mtr>
                  <mtd>
                     <mrow>
                        <mn>&#x2212;1</mn>
                        <mo>,</mo>
                        <mi>x</mi>
                        <mo>≤</mo>
                        <mn>&#x2212;1</mn>
                     </mrow>
                  </mtd>
               </mtr>
               <mtr>
                  <mtd>
                     <mrow>
                        <msup>
                           <mi>x</mi>
                           <mn>2</mn>
                        </msup>
                        <mo>+</mo>
                        <mn>2</mn>
                        <mi>x</mi>
                        <mo>,</mo>
                        <mn>&#x2212;1</mn>
                        <mo>&lt;</mo>
                        <mi>x</mi>
                        <mo>≤</mo>
                        <mn>1</mn>
                     </mrow>
                  </mtd>
               </mtr>
               <mtr>
                  <mtd>
                     <mrow>
                        <mo>&#x2212;</mo>
                        <mi>x</mi>
                        <mo>+</mo>
                        <mn>3</mn>
                        <mo>,</mo>
                        <mi>x</mi>
                        <mo>&#x003E;</mo>
                        <mn>1</mn>
                     </mrow>
                  </mtd>
               </mtr>
            </mtable>
         </mrow>
      </mrow>
</math>"#;
  test_braille("ASCIIMath_fi", expr, r"f: x -> {-1, x <= -1}, {x^2 +2 x, -1 < x <= 1}, {-x +3, x > 1}")?;
    return Ok(());
}

// Empty cells are marked up with :
#[test]
fn matrix_with_empty_cells() -> Result<()> {
    let expr = r#"<math>
<mrow>
<mo>(</mo>
<mtable>
	<mtr>
    <mtd>
    <mn>1</mn>
    </mtd>
    <mtd>
    </mtd>
    <mtd>
    </mtd>
    <mtd>
    <mn>1</mn>
    </mtd>
    </mtr>
	<mtr>
    <mtd>
    </mtd>
    <mtd>
    <mn>1</mn>
    </mtd>
    <mtd>
    </mtd>
    <mtd>
    </mtd>
    </mtr>
	<mtr>
    <mtd>
    </mtd>
    <mtd>
    </mtd>
    <mtd>
    <mn>1</mn>
    </mtd>
    <mtd>
    </mtd>
    </mtr>
	<mtr>
    <mtd>
    <mn>1</mn>
    </mtd>
    <mtd>
    </mtd>
    <mtd>
    </mtd>
    <mtd>
    <mn>1</mn>
    </mtd>
    </mtr>
</mtable>
<mo>)</mo>
</mrow>
</math>"#;
    test_braille("ASCIIMath_fi", expr, r"((1 : : 1), (: 1 : :), (: : 1 :), (1 : : 1))")?;
    return Ok(());
}

#[test]
fn open_interval() -> Result<()> {
    let expr = r#"<math> 
  <mo>]</mo>
    <mi>c</mi>
    <mo>,</mo>
    <mi>d</mi>
    <mo>[</mo>
</math>"#;
    test_braille("ASCIIMath_fi", expr, r"]c, d[")?;
    return Ok(());
}

#[test]
fn open_interval_alt() -> Result<()> {
    let expr = r#"<math> 
  <mo>(</mo>
    <mi>c</mi>
    <mo>,</mo>
    <mi>d</mi>
    <mo>)</mo>
</math>"#;
    test_braille("ASCIIMath_fi", expr, r"(c, d)")?;
    return Ok(());
}

#[test]
fn half_open_interval_left() -> Result<()> {
    let expr = r#"<math> 
  <mo>]</mo>
    <mi>c</mi>
    <mo>,</mo>
    <mi>d</mi>
    <mo>]</mo>
</math>"#;
    test_braille("ASCIIMath_fi", expr, r"]c, d]")?;
    return Ok(());
}

#[test]
fn half_open_interval_left_alt() -> Result<()> {
    let expr = r#"<math> 
  <mo>(</mo>
    <mi>c</mi>
    <mo>,</mo>
    <mi>d</mi>
    <mo>]</mo>
</math>"#;
    test_braille("ASCIIMath_fi", expr, r"(c, d]")?;
    return Ok(());
}

#[test]
fn half_open_interval_right() -> Result<()> {
    let expr = r#"<math> 
  <mo>[</mo>
    <mi>c</mi>
    <mo>,</mo>
    <mi>d</mi>
    <mo>[</mo>
</math>"#;
    test_braille("ASCIIMath_fi", expr, r"[c, d]")?;
    return Ok(());
}

#[test]
fn half_open_interval_right_alt() -> Result<()> {
    let expr = r#"<math> 
  <mo>[</mo>
    <mi>c</mi>
    <mo>,</mo>
    <mi>d</mi>
    <mo>)</mo>
</math>"#;
    test_braille("ASCIIMath_fi", expr, r"[c, d)")?;
    return Ok(());
}

#[test]
fn closed_interval() -> Result<()> {
    let expr = r#"<math> 
  <mo>[</mo>
    <mi>c</mi>
    <mo>,</mo>
    <mi>d</mi>
    <mo>]</mo>
</math>"#;
    test_braille("ASCIIMath_fi", expr, r"[c, d]")?;
    return Ok(());
}

// The bar over 'arc' is signified as the 'arc' being in parentheses.
#[test]
fn p45_arcsin_with_bar_over() -> Result<()> {
    let expr = r#"<math>
<mover>
<mi>arc</mi>
<mo>‾</mo>
</mover>
<mi>sin</mi>
<mi>x</mi>
</math>"#;
    test_braille("ASCIIMath_fi", expr, r"bar (arc) sin x")?;
    return Ok(());
}

#[test]
fn p46_first_order_derivative_alt() -> Result<()> {
    let expr = r#"<math><mi>f</mi><mo>’</mo><mo>(</mo><mi>x</mi><mo>)</mo></math>"#;
    test_braille("ASCIIMath_fi", expr, r"f'(x)")?;
    return Ok(());
}

#[test]
fn second_order_derivative_alt_1() -> Result<()> {
    let expr = r#"<math><mi>f</mi><mo>’’</mo><mo>(</mo><mi>x</mi><mo>)</mo></math>"#;
    test_braille("ASCIIMath_fi", expr, r"f''(x)")?;
    return Ok(());
}

#[test]
fn second_order_derivative_alt_2() -> Result<()> {
    let expr = r#"<math><mi>f</mi><mo>”</mo><mo>(</mo><mi>x</mi><mo>)</mo></math>"#;
    test_braille("ASCIIMath_fi", expr, r"f''(x)")?;
    return Ok(());
}

#[test]
fn nth_order_derivative() -> Result<()> {
    let expr = r#"<math><msup><mi>f</mi><mrow><mo>(</mo><mi>n</mi><mo>)</mo></mrow></msup><mo>(</mo><mi>x</mi><mo>)</mo></math>"#;
    test_braille("ASCIIMath_fi", expr, r"f^(n) (x)")?;
    return Ok(());
}

// Underline
#[test]
fn p46_random_variable() -> Result<()> {
    let expr = r#"<math>
<munder>
<mi>s</mi>
<mo>_</mo>
</munder>
</math>"#;
    test_braille("ASCIIMath_fi", expr, r"ul s")?;
    return Ok(());
}

#[test]
fn line_under_terminated_by_space() -> Result<()> {
    let expr = r#"<math>
<munder>
<mi>s</mi>
<mo>_</mo>
</munder>
<mo>+</mo>
<mn>1</mn>
</math>"#;
    test_braille("ASCIIMath_fi", expr, r"ul s +1")?;
    return Ok(());
}

// Needs parentheses so that grouping is clear.
#[test]
fn line_under_parentheses() -> Result<()> {
    let expr = r#"<math>
<munder>
<mrow>
<mi>s</mi>
<mo>/</mo>
<mn>2</mn>
</mrow>
<mo>_</mo>
</munder>
<mo>+</mo>
<mn>1</mn>
</math>"#;
    test_braille("ASCIIMath_fi", expr, r"ul (s/2) +1")?;
    return Ok(());
}

#[test]
fn limit_approaches_from_below() -> Result<()> {
    let expr = r#"<math>
<munder>
<mo>lim</mo>
<mrow>
<mi>x</mi>
<mo>↗</mo>
<mn>0</mn>
</mrow>
</munder>
<mrow>
<mrow>
<mi>f</mi>
<mo>(</mo>
<mi>x</mi>
<mo>)</mo>
</mrow>
</mrow>
</math>"#;
    test_braille("ASCIIMath_fi", expr, r"lim_(x *> 0) f(x)")?;
    return Ok(());
}

// Finnish integral substitution notation. The intent might be wrong, but that is what the notation means.
#[test]
fn p49_integral_with_Finnish_notation_for_substitution() -> Result<()> {
    let expr = r#"<math>
  <mrow>
    <msubsup>
      <mo>∫</mo>
      <mn>0</mn>
      <mn>4</mn>
    </msubsup>
    <msqrt>
      <mi>x</mi>
    </msqrt>
    <mi>d</mi>
    <mi>x</mi>
    <mo>=</mo>
    <msubsup>
      <mo intent='substitution'>⧸</mo>
      <mn>0</mn>
      <mn>4</mn>
    </msubsup>
    <mfrac>
      <mn>2</mn>
      <mn>3</mn>
    </mfrac>
    <mi>x</mi>
    <msqrt>
      <mi>x</mi>
    </msqrt>
  </mrow>
</math>"#;
    test_braille("ASCIIMath_fi", expr, r"int_0^4 sqrt(x) dx =sij_0^4 2/3 x sqrt(x)")?;
    return Ok(());
}

#[test]
fn p49_summation_with_lower_upper_bounds() -> Result<()> {
    let expr = r#"<math>
<munderover>
<mo>∑</mo>
<mrow>
<mi>i</mi>
<mo>=</mo>
<mn>0</mn>
</mrow>
<mi>n</mi>
</munderover>
<msub>
<mi>f</mi>
<mi>i</mi>
</msub>
<msub>
<mi>x</mi>
<mi>i</mi>
</msub>
</math>"#;
    test_braille("ASCIIMath_fi", expr, r"sum_(i =0)^n (f_i x_i)")?;
    return Ok(());
}

#[test]
fn p49_union_with_lower_upper_bounds() -> Result<()> {
    let expr = r#"<math>
<munderover>
<mo>∪</mo>
<mrow>
<mi>i</mi>
<mo>=</mo>
<mn>1</mn>
</mrow>
<mi>n</mi>
</munderover>
<msub>
<mi>A</mi>
<mi>i</mi>
</msub>
</math>"#;
    test_braille("ASCIIMath_fi", expr, r"uu_(i =1)^n A_i")?;
    return Ok(());
}

#[test]
fn p49_sequence_with_lower_upper_bounds() -> Result<()> {
    let expr = r#"<math>
<msubsup>
<mrow>
<mo>(</mo>
<msub>
<mi>x</mi>
<mi>n</mi>
</msub>
<mo>)</mo>
</mrow>
<mrow>
<mi>n</mi>
<mo>=</mo>
<mn>1</mn>
</mrow>
<mi>∞</mi>
</msubsup>
</math>"#;
    test_braille("ASCIIMath_fi", expr, r"(x_n)_(n =1)^oo")?;
    return Ok(());
}

#[test]
fn p50_such_that_y_greater_than_x() -> Result<()> {
    let expr = r#"<math>
    <mi>∀</mi>
    <mi>x</mi>
    <mo>∈</mo>
    <mi>ℝ</mi>
    <mo>,</mo>
    <mi>∃</mi>
    <mi>y</mi>
    <mo>∈</mo>
    <mi>ℝ</mi>
    <mo>;</mo>
    <mi>y</mi>
    <mo>&gt;</mo>
    <mi>x</mi>
</math>"#;
    test_braille("ASCIIMath_fi", expr, r"AA x in RR, EE y in RR; y > x")?;
    return Ok(());
}

#[test]
fn p50_cube_root() -> Result<()> {
    let expr = r#"<math>
  <mroot>
    <mrow>
      <mi>x</mi>
      <mo>+</mo>
      <mn>5</mn>
    </mrow>
    <mn>3</mn>
  </mroot>
</math>"#;
    test_braille("ASCIIMath_fi", expr, r"root3(x +5)")?;
    return Ok(());
}

#[test]
fn p50_binomial_with_factorial() -> Result<()> {
    let expr = r#"<math>
  <mrow>
    <mrow>
      <mo>(</mo>
      <mfrac linethickness="0px">
        <mn>6</mn>
        <mn>4</mn>
      </mfrac>
      <mo>)</mo>
    </mrow>
    <mo>=</mo>
    <mfrac>
      <mrow>
        <mn>6</mn>
        <mo>!</mo>
      </mrow>
      <mrow>
        <mn>4</mn>
        <mo>!</mo>
        <mo>(</mo>
        <mn>6</mn>
        <mo>−</mo>
        <mn>4</mn>
        <mo>)</mo>
        <mo>!</mo>
      </mrow>
    </mfrac>
  </mrow>
</math>"#;
    test_braille("ASCIIMath_fi", expr, r"((6), (4)) =6! /[4! (6 -4)!] ")?;
    return Ok(());
}

#[test]
fn p50_conditional_probability() -> Result<()> {
    let expr = r#"<math>
  <mrow>
    <mi>P</mi>
    <mo>(</mo>
    <mi>B</mi>
    <mi>|</mi>
    <mi>A</mi>
    <mo>)</mo>
    <mo>=</mo>
    <mfrac>
      <mrow>
        <mi>P</mi>
        <mo>(</mo>
        <mi>A</mi>
        <mi>B</mi>
        <mo>)</mo>
      </mrow>
      <mrow>
        <mi>P</mi>
        <mo>(</mo>
        <mi>A</mi>
        <mo>)</mo>
      </mrow>
    </mfrac>
  </mrow>
</math>"#;
    test_braille("ASCIIMath_fi", expr, r"P(B | A) =P(AB) /P(A)")?;
    return Ok(());
}

#[test]
fn p50_alternative_derivative_1() -> Result<()> {
    let expr = r#"<math>
  <mfrac>
    <mrow>
      <mi>d</mi>
      <mi>ln</mi>
      <mi>x</mi>
    </mrow>
    <mrow>
      <mi>d</mi>
      <mi>x</mi>
    </mrow>
  </mfrac>
</math>"#;
    test_braille("ASCIIMath_fi", expr, r"d ln x /dx")?;
    return Ok(());
}

// Not sure about this MathML markup.
#[test]
fn p50_alternative_derivative_2() -> Result<()> {
    let expr = r#"<math>
  <mrow>
    <mfrac>
      <mi>d</mi>
      <mrow>
        <mi>d</mi>
        <mi>x</mi>
      </mrow>
    </mfrac>
    <mi>ln</mi>
    <mi>x</mi>
  </mrow>
</math>"#;
    test_braille("ASCIIMath_fi", expr, r"d/dx ln x")?;
    return Ok(());
}

#[test]
fn p51_set_with_closure() -> Result<()> {
    let expr = r#"<math>
<mi>𝜕</mi>
<mi>A</mi> 
<mo>=</mo>
<mover>
<mi>A</mi>
<mo>‾</mo>
</mover>
<mo>∩</mo>
<mover>
<mrow>
<mo>(</mo>
<mi>X</mi>
<mo>∖</mo>
<mi>A</mi>
<mo>)</mo>
</mrow>
<mo>‾</mo>
</mover>
</math>"#;
    test_braille("ASCIIMath_fi", expr, r"del A =bar A nn bar (X -A)")?;
    return Ok(());
}


#[test]
fn p52_right_circular_cone_volume() -> Result<()> {
  let expr = r#"<math>
    <mi>V</mi>
    <mo>=</mo>
  <mrow>
    <mfrac>
      <mn>1</mn>
      <mn>3</mn>
    </mfrac>
    <mi>π</mi>
    <msup>
      <mi>r</mi>
      <mn>2</mn>
    </msup>
    <mi>h</mi>
  </mrow>
</math>"#;
    test_braille("ASCIIMath_fi", expr, r"V =1/3 ~p r^2 h")?;
    return Ok(());
}

#[test]
fn p52_circular_cone_area_equation() -> Result<()> {
    let expr = r#"<math>
    <mi>A</mi>
    <mo>=</mo>
    <mi>π</mi>
    <msup>
      <mi>r</mi>
      <mn>2</mn>
    </msup>
    <mo>+</mo>
  <mrow>
    <mi>π</mi>
    <mi>r</mi>
    <msqrt>
      <mrow>
        <msup>
          <mi>r</mi>
          <mn>2</mn>
        </msup>
        <mo>+</mo>
        <msup>
          <mi>h</mi>
          <mn>2</mn>
        </msup>
      </mrow>
    </msqrt>
  </mrow>
</math>"#;
    test_braille("ASCIIMath_fi", expr, r"A =~p r^2 +~p r sqrt(r^2 +h^2)")?;
    return Ok(());
}

#[test]
fn p63_O16() -> Result<()> {
    // From MathType
    let expr = r#"<math><mmultiscripts><mi mathvariant="normal">O</mi><mprescripts/><mn>8</mn><mn>16</mn></mmultiscripts></math>"#;
    test_braille("ASCIIMath_fi", expr, r"_8^16O")?;
    return Ok(());
}

#[test]
fn p64_chemical_equation_1() -> Result<()> {
    // From MathType
    let expr = r#"<math><msub><mtext>C</mtext><mn>2</mn></msub><msub><mtext>H</mtext><mn>5</mn></msub><mtext>OH</mtext></math>"#;
    test_braille("ASCIIMath_fi", expr, r"C_2H_5OH")?;
    return Ok(());
}

// Whitespace before the ion's minus sign.
#[test]
fn p64_chemistry_1() -> Result<()> {
    let expr = r#"<math><msubsup><mrow><mi>S</mi><mi>O</mi></mrow><mn>4</mn><mrow><mn>2</mn><mo>-</mo></mrow></msubsup></math>"#;
    test_braille("ASCIIMath_fi", expr, r"SO_4^(2 -)")?;
    return Ok(());
}

#[test]
fn p64_chemistry_bond_1() -> Result<()> {
    let expr = r#"<math intent=':chemical-formula'><mi>H</mi><mo>-</mo><mi>H</mi></math>"#;
    test_braille("ASCIIMath_fi", expr, r"H;H")?;
    return Ok(());
}

// No whitespace.
#[test]
fn p64_chemistry_bond_1_alternative() -> Result<()> {
    let expr = r#"<math intent=':chemical-formula'><mi>H</mi><mo>:</mo><mi>H</mi></math>"#;
    test_braille("ASCIIMath_fi", expr, r"H:H")?;
    return Ok(());
}

// No whitespace.
#[test]
fn p64_chemistry_bond_2() -> Result<()> {
    let expr = r#"<math intent=':chemical-formula'><mi>O</mi><mo>=</mo><mi>O</mi></math>"#;
    test_braille("ASCIIMath_fi", expr, r"O=O")?;
    return Ok(());
}

// No whitespace.
#[test]
fn p64_chemistry_bond_2_alternative() -> Result<()> {
    let expr = r#"<math intent=':chemical-formula'><mi>O</mi><mo>∷</mo><mi>O</mi></math>"#;
    test_braille("ASCIIMath_fi", expr, r"O::O")?;
    return Ok(());
}

// No whitespace.
#[test]
fn p64_chemistry_bond_3() -> Result<()> {
    let expr = r#"<math intent=':chemical-formula'><mi>N</mi><mo>≡</mo><mi>N</mi></math>"#;
    test_braille("ASCIIMath_fi", expr, r"N;=N")?;
    return Ok(());
}

#[test]
fn chemical_formula_parentheses_1() -> Result<()> {
    let expr = r#"<math>
  <mi>Ca</mi>
  <msub>
    <mrow>
    <mo>(</mo>
    <mi>O</mi>
    <mi>H</mi>
    <mo>)</mo>
    </mrow>
  <mn>2</mn>
  </msub>
</math>"#;
    test_braille("ASCIIMath_fi", expr, r"Ca(OH)_2")?;
    return Ok(());
}

#[test]
fn chemical_formula_parentheses_2() -> Result<()> {
    let expr = r#"<math>
  <msub>
    <mi>Ca</mi>
    <mn>3</mn>
  </msub>
  <msub>
    <mrow>
    <mo>(</mo>
    <mi>P</mi>
      <msub>
      <mi>O</mi>
      <mn>4</mn>
      </msub>
    <mo>)</mo>
    </mrow>
  <mn>2</mn>
  </msub>
</math>"#;
    test_braille("ASCIIMath_fi", expr, r"Ca_3(PO_4)_2")?;
    return Ok(());
}

#[test]
fn approx_1() -> Result<()> {
    let expr = r#"<math>
  <mrow>
    <mn>230</mn>
    <mo>≈</mo>
    <mn>200</mn>
  </mrow>
</math>"#;
    test_braille("ASCIIMath_fi", expr, r"230 ~~200")?;
    return Ok(());
}

#[test]
fn approx_2() -> Result<()> {
    let expr = r#"<math>
  <mrow>
    <mn>80</mn>
    <mo>+</mo>
    <mn>60</mn>
    <mo>≈</mo>
    <mn>150</mn>
  </mrow>
</math>"#;
    test_braille("ASCIIMath_fi", expr, r"80 +60 ~~150")?;
    return Ok(());
}

#[test]
fn large_numbers_1() -> Result<()> {
    let expr = r#"<math>
  <mrow>
    <mn>20.000</mn>
    <mo>+</mo>
    <mn>3000</mn>
    <mo>=</mo>
    <mn>23.000</mn>
  </mrow>
</math>"#;
    test_braille("ASCIIMath_fi", expr, r"20.000 +3000 =23.000")?;
    return Ok(());
}

#[test]
fn large_numbers_2() -> Result<()> {
    let expr = r#"<math>
  <mrow>
    <mn>88.888</mn>
    <mo>−</mo>
    <mn>8080</mn>
    <mo>=</mo>
    <mn>80.808</mn>
  </mrow>
</math>"#;
    test_braille("ASCIIMath_fi", expr, r"88.888 -8080 =80.808")?;
    return Ok(());
}

#[test]
fn comparison_multiple_numbers() -> Result<()> {
    let expr = r#"<math>
  <mrow>
    <mn>1000</mn>
    <mo>&lt;</mo>
    <mn>9000</mn>
    <mo>&lt;</mo>
    <mn>10.000</mn>
  </mrow>
</math>"#;
    test_braille("ASCIIMath_fi", expr, r"1000 < 9000 < 10.000")?;
    return Ok(());
}

#[test]
fn division_slash_1() -> Result<()> {
    let expr = r#"<math>
  <mrow>
    <mn>9</mn>
    <mo>/</mo>
    <mn>3</mn>
    <mo>=</mo>
    <mn>3</mn>
  </mrow>
</math>"#;
    test_braille("ASCIIMath_fi", expr, r"9/3 =3")?;
    return Ok(());
}

#[test]
fn division_slash_2() -> Result<()> {
    let expr = r#"<math>
  <mrow>
    <mn>100</mn>
    <mo>/</mo>
    <mn>2</mn>
    <mo>=</mo>
    <mn>50</mn>
  </mrow>
</math>"#;
    test_braille("ASCIIMath_fi", expr, r"100/2 =50")?;
    return Ok(());
}

#[test]
fn units_grams() -> Result<()> {
    let expr = r#"<math>
  <mn>380</mn>
  <mi>g</mi>
</math>"#;
    test_braille("ASCIIMath_fi", expr, r"380 g")?;
    return Ok(());
}

#[test]
fn units_kilograms_and_grams() -> Result<()> {
    let expr = r#"<math>
    <mn>4</mn>
    <mi>kg</mi>
    <mo>−</mo>
    <mn>150</mn>
    <mi>g</mi>
    <mo>=</mo>
    <mn>3,850</mn>
    <mi>kg</mi>
</math>"#;
    test_braille("ASCIIMath_fi", expr, r"4 kg -150 g =3,850 kg")?;
    return Ok(());
}
