// French braille tests from additional sources (generated candidates).
// Base reference: "NOTATION MATHÉMATIQUE BRAILLE" (Première édition janvier 2007)
//  https://www.avh.asso.fr/sites/default/files/notation_mathematique_braille2_0.pdf
// 4e, 5e, etc., refer to grade levels. Numbers refer to relevant sections in the spec.

use crate::common::*;
use anyhow::Result;

// --- lot1: digits, letters, sets ---

#[test]
fn digit_isolated_5_1_01() -> Result<()> {
    let expr= r#"<math><mn>5</mn></math>"#;
    test_braille("French", expr, "⠱")?;
    return Ok(());
}

#[test]
fn digit_multidigit_312_1_02() -> Result<()> {
    let expr= r#"<math><mn>312</mn></math>"#;
    test_braille("French", expr, "⠩⠡⠣")?;
    return Ok(());
}

#[test]
fn digit_decimal_3_14_1_03() -> Result<()> {
    let expr= r#"<math><mn>3,14</mn></math>"#;
    test_braille("French", expr, "⠩⠂⠡⠹")?;
    return Ok(());
}

#[test]
fn digit_negative_minus_7_1_04() -> Result<()> {
    let expr= r#"<math><mo>−</mo><mn>7</mn></math>"#;
    test_braille("French", expr, "⠤⠻")?;
    return Ok(());
}

#[test]
fn infinity_plus_1_05() -> Result<()> {
    let expr= r#"<math><mo>+</mo><mi>∞</mi></math>"#;
    test_braille("French", expr, "⠖⠘⠉")?;
    return Ok(());
}

#[test]
fn partial_derivative_round_d_2_1_01() -> Result<()> {
    let expr= r#"<math><mi>∂</mi></math>"#;
    test_braille("French", expr, "⠐⠙")?;
    return Ok(());
}

#[test]
fn set_naturals_blackboard_n_2_2_01() -> Result<()> {
    let expr= r#"<math><mi>ℕ</mi></math>"#;
    test_braille("French", expr, "⠨⠨⠝")?;
    return Ok(());
}

#[test]
fn greek_alpha_lower_2_3_01() -> Result<()> {
    let expr= r#"<math><mi>α</mi></math>"#;
    test_braille("French", expr, "⠘⠁")?;
    return Ok(());
}

#[test]
fn greek_delta_upper_2_3_02() -> Result<()> {
    let expr= r#"<math><mi>Δ</mi></math>"#;
    test_braille("French", expr, "⠨⠘⠙")?;
    return Ok(());
}

#[test]
fn hebrew_aleph_2_4_01() -> Result<()> {
    let expr= r#"<math><mi>ℵ</mi></math>"#;
    test_braille("French", expr, "⠘⠘⠁")?;
    return Ok(());
}

// --- lot2: operations, equality, parentheses ---

#[test]
fn addition_simple_3_01() -> Result<()> {
    let expr= r#"<math><mrow><mn>2</mn><mo>+</mo><mn>3</mn></mrow></math>"#;
    test_braille("French", expr, "⠣⠖⠩")?;
    return Ok(());
}

#[test]
fn multiplication_cross_3_02() -> Result<()> {
    let expr= r#"<math><mrow><mn>2</mn><mo>×</mo><mn>3</mn></mrow></math>"#;
    test_braille("French", expr, "⠣⠔⠩")?;
    return Ok(());
}

#[test]
fn plus_or_minus_3_03() -> Result<()> {
    let expr= r#"<math><mrow><mo>±</mo><mn>5</mn></mrow></math>"#;
    test_braille("French", expr, "⠖⠤⠱")?;
    return Ok(());
}

#[test]
fn function_composition_3_04() -> Result<()> {
    let expr= r#"<math><mrow><mi>f</mi><mo>∘</mo><mi>g</mi></mrow></math>"#;
    test_braille("French", expr, "⠋⠸⠼⠛")?;
    return Ok(());
}

#[test]
fn equality_arithmetic_4_01() -> Result<()> {
    let expr= r#"<math><mrow><mn>2</mn><mo>+</mo><mn>3</mn><mo>=</mo><mn>5</mn></mrow></math>"#;
    test_braille("French", expr, "⠣⠖⠩⠶⠱")?;
    return Ok(());
}

#[test]
fn inequality_less_than_4_02() -> Result<()> {
    let expr= r#"<math><mrow><mi>x</mi><mo>&lt;</mo><mn>5</mn></mrow></math>"#;
    test_braille("French", expr, "⠭⠐⠣⠱")?;
    return Ok(());
}

#[test]
fn approximately_equal_pi_4_03() -> Result<()> {
    let expr= r#"<math><mrow><mi>π</mi><mo>≈</mo><mn>3,14</mn></mrow></math>"#;
    test_braille("French", expr, "⠘⠏⠐⠶⠩⠂⠡⠹")?;
    return Ok(());
}

#[test]
fn parenthesis_simple_5_01() -> Result<()> {
    let expr= r#"<math><mrow><mo>(</mo><mi>a</mi><mo>+</mo><mi>b</mi><mo>)</mo></mrow></math>"#;
    test_braille("French", expr, "⠦⠁⠖⠃⠴")?;
    return Ok(());
}

#[test]
#[ignore = "awaiting feedback from French braille team"]
fn parenthesis_nested_imbrication_5_02() -> Result<()> {
    let expr= r#"<math><mrow><mi>f</mi><mo>(</mo><mi>g</mi><mo>(</mo><mi>x</mi><mo>)</mo><mo>)</mo></mrow></math>"#;
    test_braille("French", expr, "⠋⠐⠦⠛⠦⠭⠴⠐⠴")?;
    return Ok(());
}

#[test]
fn braces_set_with_condition_5_03() -> Result<()> {
    let expr= r#"<math><mrow><mo>{</mo><mi>x</mi><mo>:</mo><mi>x</mi><mo>></mo><mn>0</mn><mo>}</mo></mrow></math>"#;
    test_braille("French", expr, "⠨⠦⠭⠒⠭⠐⠜⠼⠨⠴")?;
    return Ok(());
}

// --- 4e ---

#[test]
fn linear_equation_4e_3_4_01() -> Result<()> {
    let expr= r#"<math><mrow><mn>2</mn><mi>x</mi><mo>+</mo><mn>3</mn><mo>=</mo><mn>7</mn></mrow></math>"#;
    test_braille("French", expr, "⠣⠭⠖⠩⠶⠻")?;
    return Ok(());
}

#[test]
fn inequality_4e_4_01() -> Result<()> {
    let expr= r#"<math><mrow><mn>2</mn><mi>x</mi><mo>−</mo><mn>1</mn><mo>&lt;</mo><mn>5</mn></mrow></math>"#;
    test_braille("French", expr, "⠣⠭⠤⠡⠐⠣⠱")?;
    return Ok(());
}

#[test]
fn pythagoras_4e_9_3_01() -> Result<()> {
    let expr= r#"<math><mrow><msup><mi>a</mi><mn>2</mn></msup><mo>+</mo><msup><mi>b</mi><mn>2</mn></msup><mo>=</mo><msup><mi>c</mi><mn>2</mn></msup></mrow></math>"#;
    test_braille("French", expr, "⠁⠈⠣⠖⠃⠈⠣⠶⠉⠈⠣")?;
    return Ok(());
}

#[test]
fn square_root_simple_4e_10_01() -> Result<()> {
    let expr= r#"<math><mrow><msqrt><mn>25</mn></msqrt><mo>=</mo><mn>5</mn></mrow></math>"#;
    test_braille("French", expr, "⠜⠣⠱⠶⠱")?;
    return Ok(());
}

#[test]
fn scientific_notation_4e_9_3_02() -> Result<()> {
    let expr= r#"<math><mrow><mn>3</mn><mo>×</mo><msup><mn>10</mn><mn>3</mn></msup></mrow></math>"#;
    test_braille("French", expr, "⠩⠔⠡⠼⠈⠩")?;
    return Ok(());
}

#[test]
fn negative_exponent_4e_9_3_03() -> Result<()> {
    let expr= r#"<math><mrow><msup><mn>10</mn><mrow><mo>−</mo><mn>2</mn></mrow></msup><mo>=</mo><mn>0,01</mn></mrow></math>"#;
    test_braille("French", expr, "⠡⠼⠈⠤⠣⠶⠼⠂⠼⠡")?;
    return Ok(());
}

#[test]
fn sin_function_full_4e_14_6_01() -> Result<()> {
    let expr= r#"<math><mrow><mi>sin</mi><mo>(</mo><mn>30</mn><mo>°</mo><mo>)</mo><mo>=</mo><mfrac><mn>1</mn><mn>2</mn></mfrac></mrow></math>"#;
    test_braille("French", expr, "⠎⠊⠝⠦⠩⠼⠐⠕⠴⠶⠡⠌⠣")?;
    return Ok(());
}

#[test]
fn binomial_square_4e_9_3_5_01() -> Result<()> {
    let expr= r#"<math><mrow><msup><mrow><mo>(</mo><mi>a</mi><mo>+</mo><mi>b</mi><mo>)</mo></mrow><mn>2</mn></msup><mo>=</mo><msup><mi>a</mi><mn>2</mn></msup><mo>+</mo><mn>2</mn><mi>a</mi><mi>b</mi><mo>+</mo><msup><mi>b</mi><mn>2</mn></msup></mrow></math>"#;
    test_braille("French", expr, "⠦⠁⠖⠃⠴⠈⠣⠶⠁⠈⠣⠖⠣⠁⠃⠖⠃⠈⠣")?;
    return Ok(());
}

#[test]
fn square_root_complex_4e_10_8_01() -> Result<()> {
    let expr= r#"<math><msqrt><mrow><msup><mi>a</mi><mn>2</mn></msup><mo>+</mo><msup><mi>b</mi><mn>2</mn></msup></mrow></msqrt></math>"#;
    test_braille("French", expr, "⠜⠰⠁⠈⠣⠖⠃⠈⠣⠆")?;
    return Ok(());
}

#[test]
fn probability_4e_5_7_01() -> Result<()> {
    let expr= r#"<math><mrow><mi>P</mi><mo>(</mo><mi>A</mi><mo>)</mo><mo>=</mo><mfrac><mn>1</mn><mn>4</mn></mfrac></mrow></math>"#;
    test_braille("French", expr, "⠨⠏⠦⠨⠁⠴⠶⠡⠌⠹")?;
    return Ok(());
}

// --- 5e ---

#[test]
fn fraction_add_simple_5e_7_3_01() -> Result<()> {
    let expr= r#"<math><mrow><mfrac><mn>1</mn><mn>3</mn></mfrac><mo>+</mo><mfrac><mn>1</mn><mn>4</mn></mfrac><mo>=</mo><mfrac><mn>7</mn><mn>12</mn></mfrac></mrow></math>"#;
    test_braille("French", expr, "⠡⠌⠩⠖⠡⠌⠹⠶⠻⠌⠡⠣")?;
    return Ok(());
}

#[test]
fn negative_add_5e_3_01() -> Result<()> {
    let expr= r#"<math><mrow><mo>−</mo><mn>5</mn><mo>+</mo><mn>3</mn><mo>=</mo><mo>−</mo><mn>2</mn></mrow></math>"#;
    test_braille("French", expr, "⠤⠱⠖⠩⠶⠤⠣")?;
    return Ok(());
}

#[test]
fn literal_expression_5e_3_01() -> Result<()> {
    let expr= r#"<math><mrow><mn>3</mn><mi>a</mi><mo>+</mo><mn>2</mn><mi>b</mi></mrow></math>"#;
    test_braille("French", expr, "⠩⠁⠖⠣⠃")?;
    return Ok(());
}

#[test]
fn percentage_5e_16_4_01() -> Result<()> {
    let expr= r#"<math><mrow><mn>15</mn><mo>%</mo><mo>×</mo><mn>60</mn><mo>=</mo><mn>9</mn></mrow></math>"#;
    test_braille("French", expr, "⠡⠱⠐⠬⠔⠫⠼⠶⠪")?;
    return Ok(());
}

#[test]
fn power_two_digit_base_5e_9_3_01() -> Result<()> {
    let expr= r#"<math><mrow><msup><mn>10</mn><mn>2</mn></msup><mo>=</mo><mn>100</mn></mrow></math>"#;
    test_braille("French", expr, "⠡⠼⠈⠣⠶⠡⠼⠼")?;
    return Ok(());
}

#[test]
fn volume_cube_formula_5e_9_3_02() -> Result<()> {
    let expr= r#"<math><mrow><mi>V</mi><mo>=</mo><msup><mi>a</mi><mn>3</mn></msup></mrow></math>"#;
    test_braille("French", expr, "⠨⠧⠶⠁⠈⠩")?;
    return Ok(());
}

#[test]
fn fraction_complex_numerator_5e_7_8_01() -> Result<()> {
    let expr= r#"<math><mfrac><mrow><mi>a</mi><mo>+</mo><mi>b</mi></mrow><mn>2</mn></mfrac></math>"#;
    test_braille("French", expr, "⠰⠁⠖⠃⠆⠌⠣")?;
    return Ok(());
}

#[test]
fn triangle_area_5e_7_8_02() -> Result<()> {
    let expr= r#"<math><mrow><mi>A</mi><mo>=</mo><mfrac><mrow><mi>b</mi><mo>×</mo><mi>h</mi></mrow><mn>2</mn></mfrac></mrow></math>"#;
    test_braille("French", expr, "⠨⠁⠶⠰⠃⠔⠓⠆⠌⠣")?;
    return Ok(());
}

#[test]
fn angles_sum_greek_5e_2_3_16_01() -> Result<()> {
    let expr= r#"<math><mrow><mi>α</mi><mo>+</mo><mi>β</mi><mo>=</mo><mn>90</mn><mo>°</mo></mrow></math>"#;
    test_braille("French", expr, "⠘⠁⠖⠘⠃⠶⠪⠼⠐⠕")?;
    return Ok(());
}

#[test]
fn proportionality_ratio_5e_7_4_01() -> Result<()> {
    let expr= r#"<math><mrow><mfrac><mi>a</mi><mi>b</mi></mfrac><mo>=</mo><mfrac><mi>c</mi><mi>d</mi></mfrac></mrow></math>"#;
    test_braille("French", expr, "⠁⠌⠃⠶⠉⠌⠙")?;
    return Ok(());
}

// --- 6e ---

#[test]
fn addition_6e_3_01() -> Result<()> {
    let expr= r#"<math><mrow><mn>12</mn><mo>+</mo><mn>7</mn><mo>=</mo><mn>19</mn></mrow></math>"#;
    test_braille("French", expr, "⠡⠣⠖⠻⠶⠡⠪")?;
    return Ok(());
}

#[test]
fn multiplication_6e_3_02() -> Result<()> {
    let expr= r#"<math><mrow><mn>6</mn><mo>×</mo><mn>8</mn><mo>=</mo><mn>48</mn></mrow></math>"#;
    test_braille("French", expr, "⠫⠔⠳⠶⠹⠳")?;
    return Ok(());
}

#[test]
fn division_6e_3_03() -> Result<()> {
    let expr= r#"<math><mrow><mn>15</mn><mo>÷</mo><mn>3</mn><mo>=</mo><mn>5</mn></mrow></math>"#;
    test_braille("French", expr, "⠡⠱⠒⠩⠶⠱")?;
    return Ok(());
}

#[test]
fn priority_no_parens_6e_3_4_01() -> Result<()> {
    let expr= r#"<math><mrow><mn>3</mn><mo>+</mo><mn>4</mn><mo>×</mo><mn>2</mn><mo>=</mo><mn>11</mn></mrow></math>"#;
    test_braille("French", expr, "⠩⠖⠹⠔⠣⠶⠡⠡")?;
    return Ok(());
}

#[test]
fn parentheses_priority_6e_5_01() -> Result<()> {
    let expr= r#"<math><mrow><mrow><mo>(</mo><mn>3</mn><mo>+</mo><mn>4</mn><mo>)</mo></mrow><mo>×</mo><mn>2</mn><mo>=</mo><mn>14</mn></mrow></math>"#;
    test_braille("French", expr, "⠦⠩⠖⠹⠴⠔⠣⠶⠡⠹")?;
    return Ok(());
}

#[test]
fn fraction_simple_6e_7_01() -> Result<()> {
    let expr= r#"<math><mfrac><mn>3</mn><mn>4</mn></mfrac></math>"#;
    test_braille("French", expr, "⠩⠌⠹")?;
    return Ok(());
}

#[test]
fn fraction_equality_6e_7_4_01() -> Result<()> {
    let expr= r#"<math><mrow><mfrac><mn>1</mn><mn>2</mn></mfrac><mo>=</mo><mfrac><mn>2</mn><mn>4</mn></mfrac></mrow></math>"#;
    test_braille("French", expr, "⠡⠌⠣⠶⠣⠌⠹")?;
    return Ok(());
}

#[test]
fn fraction_decimal_6e_7_1_01() -> Result<()> {
    let expr= r#"<math><mrow><mfrac><mn>3</mn><mn>4</mn></mfrac><mo>=</mo><mn>0,75</mn></mrow></math>"#;
    test_braille("French", expr, "⠩⠌⠹⠶⠼⠂⠻⠱")?;
    return Ok(());
}

#[test]
fn square_exponent_6e_9_3_01() -> Result<()> {
    let expr= r#"<math><mrow><msup><mn>5</mn><mn>2</mn></msup><mo>=</mo><mn>25</mn></mrow></math>"#;
    test_braille("French", expr, "⠱⠈⠣⠶⠣⠱")?;
    return Ok(());
}

#[test]
fn perpendicular_lines_6e_12_3_01() -> Result<()> {
    let expr= r#"<math><mrow><mo>(</mo><mi>A</mi><mi>B</mi><mo>)</mo><mo>⊥</mo><mo>(</mo><mi>C</mi><mi>D</mi><mo>)</mo></mrow></math>"#;
    test_braille("French", expr, "⠦⠨⠁⠨⠃⠴⠘⠳⠦⠨⠉⠨⠙⠴")?;
    return Ok(());
}
