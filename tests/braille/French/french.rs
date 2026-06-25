// French braille tests for the basic mathml tags
// These tests are from the French braille authority's publication "NOTATION PIATHÉMATIQUE BRAILLE" (Première édition janvier 2007)
// Matrix formulas have been added to the initial release.  All examples from the reference document are taken into account.
//  https://www.avh.asso.fr/sites/default/files/notation_mathematique_braille2_0.pdf

use crate::common::*;
use anyhow::Result;

#[test]
fn binomial_nk_01() -> Result<()> {
    let expr= r#"<math><mrow><mo>(</mo><mfrac linethickness="0"><mi>n</mi><mi>k</mi></mfrac><mo>)</mo></mrow></math>"#;
    test_braille("French", expr, "⠘⠦⠝⠠⠜⠅⠘⠴")?;
    return Ok(());
}

#[test]
fn binomial_5_2_02() -> Result<()> {
    let expr= r#"<math><mrow><mo>(</mo><mfrac linethickness="0"><mn>5</mn><mn>2</mn></mfrac><mo>)</mo></mrow></math>"#;
    test_braille("French", expr, "⠘⠦⠱⠠⠜⠣⠘⠴")?;
    return Ok(());
}

#[test]
fn binomial_symmetry_03() -> Result<()> {
    let expr= r#"<math><mrow><mrow><mo>(</mo><mfrac linethickness="0"><mi>n</mi><mi>k</mi></mfrac><mo>)</mo></mrow><mo>=</mo><mrow><mo>(</mo><mfrac linethickness="0"><mi>n</mi><mrow><mi>n</mi><mo>−</mo><mi>k</mi></mrow></mfrac><mo>)</mo></mrow></mrow></math>"#;
    test_braille("French", expr, "⠘⠦⠝⠠⠜⠅⠘⠴⠶⠘⠦⠝⠠⠜⠝⠤⠅⠘⠴")?;
    return Ok(());
}

#[test]
fn binomial_pascal_04() -> Result<()> {
    let expr= r#"<math><mrow><mrow><mo>(</mo><mfrac linethickness="0"><mi>n</mi><mi>k</mi></mfrac><mo>)</mo></mrow><mo>=</mo><mrow><mo>(</mo><mfrac linethickness="0"><mrow><mi>n</mi><mo>−</mo><mn>1</mn></mrow><mrow><mi>k</mi><mo>−</mo><mn>1</mn></mrow></mfrac><mo>)</mo></mrow><mo>+</mo><mrow><mo>(</mo><mfrac linethickness="0"><mrow><mi>n</mi><mo>−</mo><mn>1</mn></mrow><mi>k</mi></mfrac><mo>)</mo></mrow></mrow></math>"#;
    test_braille("French", expr, "⠘⠦⠝⠠⠜⠅⠘⠴⠶⠘⠦⠝⠤⠡⠠⠜⠅⠤⠡⠘⠴⠖⠘⠦⠝⠤⠡⠠⠜⠅⠘⠴")?;
    return Ok(());
}


#[test]
fn set_of_elements_6_4_01() -> Result<()> {
    let expr= r#"<math><mrow><mi>E</mi><mo>=</mo><mrow><mo>{</mo><mi>a</mi><mo>,</mo><mi>b</mi><mo>,</mo><mi>c</mi><mo>,</mo><mi>d</mi><mo>}</mo></mrow></mrow></math>"#;
    test_braille("French", expr, "⠨⠑⠶⠨⠦⠁⠂⠃⠂⠉⠂⠙⠨⠴")?;
    return Ok(());
}

#[test]
fn set_r_definition_6_4_02() -> Result<()> {
    let expr= r#"<math><mrow><msup><mi>ℝ</mi><mo>*</mo></msup><mo>=</mo><mrow><mo>]</mo><mi>−</mi><mi>∞</mi><mo>,</mo><mn>0</mn><mo>[</mo></mrow><mo>∪</mo><mrow><mo>]</mo><mn>0</mn><mo>,</mo><mi>+</mi><mi>∞</mi><mo>[</mo></mrow></mrow></math>"#;
    test_braille("French", expr, "⠨⠨⠗⠈⠐⠔⠶⠾⠤⠘⠉⠂⠼⠷⠸⠖⠾⠼⠂⠖⠘⠉⠷")?;
    return Ok(());
}

#[test]
fn set_belonging_6_4_03() -> Result<()> {
    let expr= r#"<math><mrow><mi>x</mi><mo>∈</mo><mo stretchy="false">[</mo><mn>0</mn><mo>,</mo><mn>1</mn><mo>;</mo><mn>2</mn><mo>,</mo><mn>3</mn><mo stretchy="false">]</mo></mrow></math>"#;
    test_braille("French", expr, "⠭⠘⠡⠷⠼⠂⠡⠆⠣⠂⠩⠾")?;
    return Ok(());
}

#[test]
fn set_not_belong_tor_6_4_04() -> Result<()> {
    let expr= r#"<math><mrow><mi>x</mi><mo>∉</mo><msup><mi>ℝ</mi><mo>+</mo></msup></mrow></math>"#;
    test_braille("French", expr, "⠭⠘⠌⠨⠨⠗⠈⠖")?;
    return Ok(());
}

#[test]
fn set_included_6_4_05() -> Result<()> {
    let expr= r#"<math><mrow><mi>D</mi><mo>⊂</mo><mi>F</mi></mrow></math>"#;
    test_braille("French", expr, "⠨⠙⠨⠡⠨⠋")?;
    return Ok(());
}

#[test]
fn set_not_included_6_4_06() -> Result<()> {
    let expr= r#"<math><mrow><mi>D</mi><mo>⊄</mo><mi>E</mi></mrow></math>"#;
    test_braille("French", expr, "⠨⠙⠨⠌⠨⠑")?;
    return Ok(());
}

#[test]
fn set_included_or_equal_6_4_07() -> Result<()> {
    let expr= r#"<math><mrow><mi>A</mi><mo>⊆</mo><mi>B</mi></mrow></math>"#;
    test_braille("French", expr, "⠨⠁⠸⠨⠡⠨⠃")?;
    return Ok(());
}

#[test]
fn set_intersection_6_4_08() -> Result<()> {
    let expr= r#"<math><mrow><mi>A</mi><mo>∩</mo><mi>B</mi><mo>=</mo><mi>∅</mi></mrow></math>"#;
    test_braille("French", expr, "⠨⠁⠘⠖⠨⠃⠶⠘⠼")?;
    return Ok(());
}

#[test]
fn complement_of_6_4_09() -> Result<()> {
    let expr= r#"<math><mrow><msub><mi>∁</mi><mi>E</mi></msub><mi>F</mi></mrow></math>"#;
    test_braille("French", expr, "⠨⠩⠢⠨⠑⠨⠋")?;
    return Ok(());
}

#[test]
fn fraction_01_7_01() -> Result<()> {
    let expr= r#"<math><mrow><mfrac><mn>2</mn><mn>3</mn></mfrac><mo>=</mo><mfrac><mn>4</mn><mn>6</mn></mfrac></mrow></math>"#;
    test_braille("French", expr, "⠣⠌⠩⠶⠹⠌⠫")?;
    return Ok(());
}

#[test]
fn devided_by_7_02() -> Result<()> {
    let expr= r#"<math><mrow><mn>1</mn><mo>÷</mo><mn>2</mn><mo>=</mo><mn>0</mn><mo>,</mo><mn>5</mn></mrow></math>"#;
    test_braille("French", expr, "⠡⠒⠣⠶⠼⠂⠱")?;
    return Ok(());
}

#[test]
fn fraction_02_8_01() -> Result<()> {
    let expr= r#"<math display='inline'><mfrac><mrow><mi>a</mi><mo>+</mo><mi>b</mi></mrow><mi>a</mi></mfrac></math>"#;
    test_braille("French", expr, "⠠⠄⠰⠁⠖⠃⠆⠌⠁")?;
    return Ok(());
}

#[test]
fn fraction_03_8_02() -> Result<()> {
    let expr= r#"<math display='inline'><mfrac><mrow><mi>a</mi><mo>+</mo><mi>b</mi></mrow><mrow><mi>c</mi><mo>+</mo><mi>d</mi></mrow></mfrac></math>"#;
    test_braille("French", expr, "⠠⠄⠰⠁⠖⠃⠆⠌⠰⠉⠖⠙⠆")?;
    return Ok(());
}

#[test]
fn fraction_04_8_03() -> Result<()> {
    let expr= r#"<math display='inline'><mrow><mfrac><mi>a</mi><mi>b</mi></mfrac><mo>+</mo><mfrac><mi>c</mi><mi>d</mi></mfrac><mo>=</mo><mfrac><mrow><mi>a</mi><mi>d</mi><mo>+</mo><mi>b</mi><mi>c</mi></mrow><mrow><mi>b</mi><mi>d</mi></mrow></mfrac></mrow></math>"#;
    test_braille("French", expr, "⠠⠄⠁⠌⠃⠖⠉⠌⠙⠶⠰⠁⠙⠖⠃⠉⠆⠌⠃⠙")?;
    return Ok(());
}

#[test]
fn cardinal_8_04() -> Result<()> {
    let expr= r#"<math display='inline'><mrow><mi>C</mi><mi>a</mi><mi>r</mi><mi>d</mi><mspace width="0.333em"></mspace><mi>E</mi><mo>=</mo><mn>5</mn></mrow></math>"#;
    test_braille("French", expr, "⠠⠄⠨⠉⠁⠗⠙⠰⠨⠑⠆⠶⠱")?;
    return Ok(());
}

#[test]
fn exponential_01_8_05() -> Result<()> {
    let expr= r#"<math display='inline'><msup><mi>e</mi><mrow><mi>x</mi><mo>+</mo><mn>3</mn></mrow></msup></math>"#;
    test_braille("French", expr, "⠠⠄⠑⠈⠰⠭⠖⠩⠆")?;
    return Ok(());
}

#[test]
fn exponential_02_8_06() -> Result<()> {
    let expr= r#"<math display='inline'><msup><mi>e</mi><mn>7</mn></msup></math>"#;
    test_braille("French", expr, "⠠⠑⠈⠻")?;
    return Ok(());
}

#[test]
fn exponential_03_8_07() -> Result<()> {
    let expr= r#"<math display='inline'><msup><mi>e</mi><mrow><mi>−</mi><mi>x</mi></mrow></msup></math>"#;
    test_braille("French", expr, "⠠⠑⠈⠤⠭")?;
    return Ok(());
}

#[test]
fn exponential_04_8_08() -> Result<()> {
    let expr= r#"<math display='inline'><msup><mi>e</mi><mrow><mi>−</mi><mi>π</mi></mrow></msup></math>"#;
    test_braille("French", expr, "⠠⠑⠈⠤⠘⠏")?;
    return Ok(());
}

#[test]
fn power_negativ_one_8_09() -> Result<()> {
    let expr= r#"<math display='inline'><mrow><msup><mi>x</mi><mrow><mi>−</mi><mn>1</mn></mrow></msup><mo>=</mo><mfrac><mn>1</mn><mi>x</mi></mfrac></mrow></math>"#;
    test_braille("French", expr, "⠠⠭⠈⠤⠡⠶⠡⠌⠭")?;
    return Ok(());
}

#[test]
fn prime_9_1_01() -> Result<()> {
    let expr= r#"<math><msup><mi>a</mi><mo>′</mo></msup></math>"#;
    test_braille("French", expr, "⠁⠄")?;
    return Ok(());
}

#[test]
fn prime_dople_9_1_02() -> Result<()> {
    let expr= r#"<math><msup><mi>x</mi><mi>″</mi></msup></math>"#;
    test_braille("French", expr, "⠭⠄⠄")?;
    return Ok(());
}

#[test]
fn prime_dople_triple_9_1_03() -> Result<()> {
    let expr= r#"<math><mrow><msup><mi>a</mi><mo>′</mo></msup><mi>x</mi><mo>+</mo><msup><mi>a</mi><mi>″</mi></msup><mi>y</mi><mo>+</mo><msup><mi>a</mi><mi>‴</mi></msup><mi>z</mi></mrow></math>"#;
    test_braille("French", expr, "⠁⠄⠭⠖⠁⠄⠄⠽⠖⠁⠄⠄⠄⠵")?;
    return Ok(());
}

#[test]
fn sub_a_p_9_2_01() -> Result<()> {
    let expr= r#"<math><msub><mi>a</mi><mi>p</mi></msub></math>"#;
    test_braille("French", expr, "⠁⠢⠏")?;
    return Ok(());
}

#[test]
fn sub_x_0__9_2_02() -> Result<()> {
    let expr= r#"<math><msub><mi>x</mi><mn>0</mn></msub></math>"#;
    test_braille("French", expr, "⠭⠢⠼")?;
    return Ok(());
}

#[test]
fn sub_u_n_plus_one_9_2_03() -> Result<()> {
    let expr= r#"<math><msub><mi>u</mi><mrow><mi>n</mi><mo>+</mo><mn>1</mn></mrow></msub></math>"#;
    test_braille("French", expr, "⠥⠢⠰⠝⠖⠡⠆")?;
    return Ok(());
}

#[test]
fn sup_x_2_9_3_01() -> Result<()> {
    let expr= r#"<math><msup><mi>x</mi><mn>2</mn></msup></math>"#;
    test_braille("French", expr, "⠭⠈⠣")?;
    return Ok(());
}

#[test]
fn sup_x_p_plus_q_9_3_02() -> Result<()> {
    let expr= r#"<math><msup><mi>x</mi><mrow><mi>p</mi><mo>+</mo><mi>q</mi></mrow></msup></math>"#;
    test_braille("French", expr, "⠭⠈⠰⠏⠖⠟⠆")?;
    return Ok(());
}

#[test]
fn sup_x_negative_number_9_3_03() -> Result<()> {
    let expr= r#"<math><msup><mi>a</mi><mrow><mi>−</mi><mn>5</mn><mo>,</mo><mn>3</mn></mrow></msup></math>"#;
    test_braille("French", expr, "⠁⠈⠤⠱⠂⠩")?;
    return Ok(());
}

#[test]
fn sup_a_pi_9_3_04() -> Result<()> {
    let expr= r#"<math><msup><mi>a</mi><mi>π</mi></msup></math>"#;
    test_braille("French", expr, "⠁⠈⠘⠏")?;
    return Ok(());
}

#[test]
fn sup_5_formula_block_1_9_3_05() -> Result<()> {
    let expr= r#"<math><mrow><msup><mn>5</mn><mrow><mn>2</mn></mrow></msup><msqrt><mn>3</mn></msqrt><mo>≠</mo><msup><mn>5</mn><mrow><mn>2</mn><msqrt><mn>3</mn></msqrt></mrow></msup></mrow></math>"#;
    test_braille("French", expr, "⠱⠈⠣⠜⠩⠨⠶⠱⠈⠰⠣⠜⠩⠆")?;
    return Ok(());
}

#[test]
fn sup_5_formula_block_0_9_3_06() -> Result<()> {
    let expr= r#"<math><mrow><msup><mi>e</mi><mrow><mi>−</mi><mn>5</mn><mi>x</mi></mrow></msup><mo>≠</mo><msup><mi>e</mi><mrow><mi>−</mi><mn>5</mn></mrow></msup><mi>x</mi></mrow></math>"#;
    test_braille("French", expr, "⠑⠈⠰⠤⠱⠭⠆⠨⠶⠰⠑⠈⠤⠱⠆⠭")?;
    return Ok(());
}

#[test]
fn sub_a_prime_0_9_4_01() -> Result<()> {
    let expr= r#"<math><msub><msup><mi>a</mi><mo>′</mo></msup><mn>0</mn></msub></math>"#;
    test_braille("French", expr, "⠁⠄⠢⠼")?;
    return Ok(());
}

#[test]
fn sub_mixt_0_9_4_02() -> Result<()> {
    let expr= r#"<math><msubsup><msup><mi>x</mi><mo>′</mo></msup><mn>0</mn><mn>2</mn></msubsup></math>"#;
    test_braille("French", expr, "⠭⠄⠢⠼⠈⠣")?;
    return Ok(());
}

#[test]
fn choose_9_4_03() -> Result<()> {
    let expr= r#"<math><mrow><msubsup><mi>C</mi><mi>m</mi><mi>p</mi></msubsup><mo>=</mo><msubsup><mi>C</mi><mi>m</mi><mrow><mi>m</mi><mo>−</mo><mi>p</mi></mrow></msubsup></mrow></math>"#;
    test_braille("French", expr, "⠨⠉⠢⠍⠈⠏⠶⠨⠉⠢⠍⠈⠰⠍⠤⠏⠆")?;
    return Ok(());
}

#[test]
fn sub_sup_mixed_formulaq_9_4_04() -> Result<()> {
    let expr= r#"<math><msubsup>
                    <msup><mi>a</mi><mo>′'</mo></msup>
                    <mrow><mi>m</mi><mo>+</mo><mi>n</mi></mrow>
                    <mrow><mi>p</mi><mo>+</mo><mi>q</mi></mrow>
                </msubsup></math>"#;
    test_braille("French", expr, "⠁⠄⠄⠢⠰⠍⠖⠝⠆⠈⠰⠏⠖⠟⠆")?;
    return Ok(());
}

#[test]
fn sub_sup_operators_9_4_05() -> Result<()> {
    let expr= r#"<math><msubsup><mi>ℝ</mi><mo>+</mo><mo>*</mo></msubsup></math>"#;
    test_braille("French", expr, "⠨⠨⠗⠢⠖⠈⠐⠔")?;
    return Ok(());
}

#[test]
fn left_sup_9_5_01() -> Result<()> {
    let expr= r#"<math><mrow><msubsup><mi></mi><mrow></mrow><mi>t</mi></msubsup><mi>A</mi></mrow></math>"#;
    test_braille("French", expr, "⠈⠞⠨⠁")?;
    return Ok(());
}

#[test]
fn left_sub_9_5_02() -> Result<()> {
    let expr= r#"<math><mrow><msubsup><mi></mi><mn>2</mn><mrow></mrow></msubsup><mi>w</mi></mrow></math>"#;
    test_braille("French", expr, "⠢⠣⠺")?;
    return Ok(());
}

#[test]
fn left_right_sup_9_5_03() -> Result<()> {
    let expr: &str= r#"<math><mmultiscripts><mi>A</mi><mprescripts/><mrow/><mi>t</mi></mmultiscripts><mmultiscripts><mi>B</mi><mprescripts/><mrow/><mi>t</mi></mmultiscripts></math>"#;
    test_braille("French", expr, "⠰⠈⠞⠨⠁⠆⠰⠈⠞⠨⠃⠆")?;
    return Ok(());
}

#[test]
fn left_sub_sup_test_9_5_04() -> Result<()> {
    let expr= r#"<math><mrow><msubsup><mi></mi><mn>8</mn><mn>16</mn></msubsup><mi>O</mi></mrow></math>"#;
    test_braille("French", expr, "⠨⠕⠠⠢⠳⠠⠈⠡⠫")?;
    return Ok(());
}

#[test]
fn bigsum_one_line_9_6_01() -> Result<()> {
    let expr= r#"<math><mrow><munderover><mo>∑</mo><mrow><mi>i</mi><mo>=</mo><mn>1</mn></mrow><mi>n</mi></munderover><msub><mi>u</mi><mi>i</mi></msub><mo>=</mo><msub><mi>u</mi><mn>1</mn></msub><mo>+</mo><msub><mi>u</mi><mn>2</mn></msub><mo>+</mo><mi>⋯</mi><mo>+</mo><msub><mi>u</mi><mi>n</mi></msub></mrow></math>"#;
    test_braille("French", expr, "⠨⠘⠎⠢⠢⠰⠊⠶⠡⠆⠈⠈⠝⠥⠢⠊⠶⠥⠢⠡⠖⠥⠢⠣⠖⠲⠲⠲⠖⠥⠢⠝")?;
    return Ok(());
}
#[test]
fn root_square_10_01() -> Result<()> {
    let expr= r#"<math><msqrt><mn>2</mn></msqrt></math>"#;
    test_braille("French", expr, "⠜⠣")?;
    return Ok(());
}

#[test]
fn root_fourth_10_02() -> Result<()> {
    let expr= r#"<math><mroot><mi>a</mi><mn>4</mn></mroot></math>"#;
    test_braille("French", expr, "⠈⠹⠜⠁")?;
    return Ok(());
}

#[test]
fn square_formula_1_10_03() -> Result<()> {
    let expr= r#"<math><mrow><mroot><mrow><mo stretchy="false">(</mo><mi>a</mi><mo>+</mo><mi>b</mi><msup><mo stretchy="false">)</mo><mn>3</mn></msup></mrow><mn>6</mn></mroot><mo>=</mo><msqrt><mrow><mi>a</mi><mo>+</mo><mi>b</mi></mrow></msqrt></mrow></math>"#;
    test_braille("French", expr, "⠈⠫⠜⠰⠦⠁⠖⠃⠴⠈⠩⠆⠶⠜⠰⠁⠖⠃⠆")?;
    return Ok(());
}

#[test]
fn square_formula_2_10_04() -> Result<()> {
    let expr= r#"<math><mroot><mrow><mi>a</mi><mo>+</mo><msqrt><mrow><mi>a</mi><mo>+</mo><mi>b</mi></mrow></msqrt></mrow><mn>3</mn></mroot></math>"#;
    test_braille("French", expr, "⠈⠩⠜⠰⠁⠖⠜⠰⠁⠖⠃⠆⠆")?;
    return Ok(());
}

#[test]
fn square_formula_3_10_05() -> Result<()> {
    let expr= r#"<math><mrow><mroot><mi>a</mi><mn>4</mn></mroot><mroot><mi>b</mi><mn>3</mn></mroot></mrow></math>"#;
    test_braille("French", expr, "⠰⠈⠹⠜⠁⠆⠰⠈⠩⠜⠃⠆")?;
    return Ok(());
}

#[test]
fn function_from_r_to_r_11_01() -> Result<()> {
    let expr= r#"<math><mrow><mi>f</mi><mo>:</mo><mi>ℝ</mi><mo>→</mo><mrow><mi>ℝ</mi><mo>,</mo><mspace width="0.222em"/></mrow><mspace width="0.222em"/><mi>x</mi><mo>↦</mo><mi>y</mi><mo>=</mo><mfrac><mn>1</mn><mi>x</mi></mfrac></mrow></math>"#;
    test_braille("French", expr, "⠋⠒⠨⠨⠗⠸⠱⠨⠨⠗⠭⠐⠱⠽⠶⠡⠌⠭")?;
    return Ok(());
}

#[test]
fn over_arrow_12_1_01() -> Result<()> {
    let expr= r#"<math><mover><mi>v</mi><mo accent="true">→</mo></mover></math>"#;
    test_braille("French", expr, "⠨⠒⠧")?;
    return Ok(());
}

#[test]
fn over_barre_12_1_02() -> Result<()> {
    let expr= r#"<math><mover><mrow><mi>A</mi><mi>B</mi></mrow><mo accent="true">¯</mo></mover></math>"#;
    test_braille("French", expr, "⠸⠒⠨⠁⠨⠃")?;
    return Ok(());
}

#[test]
fn over_arc_12_1_03() -> Result<()> {
    let expr= r#"<math><mover><mrow><mi>A</mi><mi>B</mi></mrow><mo accent="true">⏜</mo></mover></math>"#;
    test_braille("French", expr, "⠈⠒⠨⠁⠨⠃")?;
    return Ok(());
}

#[test]
fn over_large_hat_1_12_1_04() -> Result<()> {
    let expr= r#"<math><mover><mrow><mi>A</mi><mi>O</mi><mi>B</mi></mrow><mo accent="true">̂</mo></mover></math>"#;
    test_braille("French", expr, "⠘⠒⠨⠁⠨⠕⠨⠃")?;
    return Ok(());
}

#[test]
fn over_large_hat_2_12_1_05() -> Result<()> {
    let expr= r#"<math><mover><mrow><mo stretchy="false">(</mo><mi>O</mi><mi>x</mi><mo>,</mo><mi>O</mi><mi>y</mi><mo stretchy="false">)</mo></mrow><mo accent="true">̂</mo></mover></math>"#;
    test_braille("French", expr, "⠘⠒⠦⠨⠕⠭⠂⠨⠕⠽⠴")?;
    return Ok(());
}

#[test]
fn over_large_hat_3_12_1_06() -> Result<()> {
    let expr= r#"<math><mover><mrow><mo>(</mo><mover><mrow><mi>O</mi><mi>A</mi></mrow><mo accent="true">→</mo></mover><mo>,</mo><mover><mrow><mi>O</mi><mi>M</mi></mrow><mo accent="true">→</mo></mover><mo>)</mo></mrow><mo accent="true">̂</mo></mover></math>"#;
    test_braille("French", expr, "⠘⠒⠦⠨⠒⠨⠕⠨⠁⠂⠨⠒⠨⠕⠨⠍⠴")?;
    return Ok(());
}

#[test]
fn parallel_d_delta_12_3_01() -> Result<()> {
    let expr= r#"<math><mrow><mo stretchy="false">(</mo><mi>D</mi><mo stretchy="false">)</mo><mo>⫽</mo><mrow><mo>(</mo><mtext mathvariant="normal">Δ</mtext><mo>)</mo></mrow></mrow></math>"#;
    test_braille("French", expr, "⠦⠨⠙⠴⠸⠳⠦⠨⠘⠙⠴")?;
    return Ok(());
}

#[test]
fn perpendicular_12_3_02() -> Result<()> {
    let expr= r#"<math><mrow><mo stretchy="false">(</mo><mi>A</mi><mi>B</mi><mo stretchy="false">)</mo><mi>⊥</mi><mrow><mo>(</mo><msup><mi>A</mi><mo>′</mo></msup><msup><mi>B</mi><mo>′</mo></msup><mo>)</mo></mrow></mrow></math>"#;
    test_braille("French", expr, "⠦⠨⠁⠨⠃⠴⠘⠳⠦⠨⠁⠄⠨⠃⠄⠴")?;
    return Ok(());
}

#[test]
fn abs_13_2_01() -> Result<()> {
    let expr= r#"<math><mrow><mo>|</mo><msub><mi>x</mi><mn>0</mn></msub><mo>|</mo></mrow></math>"#;
    test_braille("French", expr, "⠿⠭⠢⠼⠿")?;
    return Ok(());
}

#[test]
fn modulus_13_2_02() -> Result<()> {
    let expr= r#"<math><mrow><mo stretchy="false">|</mo><mi>a</mi><mo>+</mo><mi>i</mi><mi>b</mi><mo stretchy="false">|</mo></mrow></math>"#;
    test_braille("French", expr, "⠿⠁⠖⠊⠃⠿")?;
    return Ok(());
}

#[test]
fn norm_k_u_vector_13_2_03() -> Result<()> {
    let expr= r#"<math><mrow><mrow><mo>∥</mo><mi>k</mi><mover><mi>u</mi><mo accent="true">→</mo></mover><mo>∥</mo></mrow><mo>=</mo><mo stretchy="false">|</mo><mi>k</mi><mo stretchy="false">|</mo><mrow><mo>∥</mo><mover><mi>u</mi><mo accent="true">→</mo></mover><mo>∥</mo></mrow></mrow></math>"#;
    test_braille("French", expr, "⠘⠿⠅⠨⠒⠥⠘⠿⠶⠿⠅⠿⠘⠿⠨⠒⠥⠘⠿")?;
    return Ok(());
}

#[test]
fn restriction_of_f_to_13_2_04() -> Result<()> {
    let expr= r#"<math><mrow><mi>f</mi><mrow><mo>|</mo><mi>E</mi></mrow><mspace width="0.222em"></mspace></mrow></math>"#;
    test_braille("French", expr, "⠋⠿⠨⠑")?;
    return Ok(());
}

// #[test]
// fn determinant_tabular_13_3_01() -> Result<()> {
//     let expr= r#"<math><mrow><mi>I</mi><mo>=</mo><mrow><mo stretchy="true" form="prefix">|</mo><mtable><mtr><mtd columnalign="center" style="text-align: center"><mn>1</mn></mtd><mtd columnalign="center" style="text-align: center"><mn>0</mn></mtd></mtr><mtr><mtd columnalign="center" style="text-align: center"><mn>2</mn></mtd><mtd columnalign="center" style="text-align: center"><mn>3</mn></mtd></mtr></mtable><mo stretchy="true" form="postfix">|</mo></mrow><mo>=</mo><mn>3</mn></mrow></math>"#;
//     test_braille("French", expr, "not correct⠨⠊⠶⠿⠡⠀⠼⠿   ⠿⠣⠀⠩⠿⠶⠩")?;
//     return Ok(());
// }
#[test]
fn matrix_linear_13_3_02() -> Result<()> {
    let expr= r#"<math><mrow><mi>J</mi><mo>=</mo><mrow><mo stretchy="true" form="prefix">(</mo><mtable><mtr><mtd columnalign="center" style="text-align: center"><mn>1</mn></mtd><mtd columnalign="center" style="text-align: center"></mtd><mtd columnalign="center" style="text-align: center"></mtd><mtd columnalign="center" style="text-align: center"><mn>4</mn></mtd></mtr><mtr><mtd columnalign="center" style="text-align: center"><mn>2</mn></mtd><mtd columnalign="center" style="text-align: center"><mn>3</mn></mtd><mtd columnalign="center" style="text-align: center"><mn>5</mn></mtd><mtd columnalign="center" style="text-align: center"><mn>6</mn></mtd></mtr><mtr><mtd columnalign="center" style="text-align: center"><mo>−</mo><mn>7</mn></mtd><mtd columnalign="center" style="text-align: center"><mn>8</mn></mtd><mtd columnalign="center" style="text-align: center"></mtd><mtd columnalign="center" style="text-align: center"><mn>9</mn></mtd></mtr></mtable><mo stretchy="true" form="postfix">)</mo></mrow></mrow></math>"#;
    test_braille("French", expr, "⠨⠚⠶⠘⠦⠡⠀⠐⠂⠀⠐⠂⠀⠹⠀⠠⠜⠀⠣⠀⠩⠀⠱⠀⠫⠀⠠⠜⠀⠤⠻⠀⠳⠀⠐⠂⠀⠪⠘⠴")?;
    return Ok(());
}
#[test]
fn matrix_with_equations_one_lines_13_3_03() -> Result<()> {
    let expr= r#"<math><mrow><mi>K</mi><mo>=</mo><mrow><mo stretchy="true" form="prefix">[</mo><mtable><mtr><mtd columnalign="center" style="text-align: center"><mn>1</mn></mtd><mtd columnalign="center" style="text-align: center"><mfrac><mrow><msup><mi>x</mi><mn>2</mn></msup><mo>+</mo><mn>3</mn><mi>x</mi><mo>−</mo><mn>9</mn></mrow><mrow><mn>6</mn><msup><mi>x</mi><mn>3</mn></msup><mo>−</mo><msup><mi>x</mi><mn>2</mn></msup><mo>+</mo><mn>1</mn></mrow></mfrac></mtd></mtr><mtr><mtd columnalign="center" style="text-align: center"><mfrac><mrow><mi>x</mi><mo>+</mo><mn>3</mn></mrow><mrow><mi>y</mi><mo>+</mo><mn>5</mn></mrow></mfrac></mtd><mtd columnalign="center" style="text-align: center"><mi>g</mi><mrow><mo stretchy="true" form="prefix">(</mo><mi>x</mi><mo stretchy="true" form="postfix">)</mo></mrow></mtd></mtr></mtable><mo stretchy="true" form="postfix">]</mo></mrow></mrow></math>"#;
    test_braille("French", expr, "⠨⠅⠶⠘⠷⠡⠀⠰⠭⠈⠣⠖⠩⠭⠤⠪⠆⠌⠰⠫⠭⠈⠩⠤⠭⠈⠣⠖⠡⠆⠀⠠⠜⠀⠰⠭⠖⠩⠆⠌⠰⠽⠖⠱⠆⠀⠛⠦⠭⠴⠘⠾")?;
    return Ok(());
}
#[test]
fn sytem_equation_linéaraire_13_4_01() -> Result<()> {
    let expr= r#"<math><mrow><mrow><mo stretchy="true" form="prefix">{</mo><mtable><mtr><mtd columnalign="right" style="text-align: right"><mn>2</mn><mi>x</mi><mo>+</mo><mn>3</mn><mi>y</mi><mo>=</mo><mn>5</mn></mtd></mtr><mtr><mtd columnalign="right" style="text-align: right"><mn>3</mn><mi>x</mi><mo>−</mo><mn>7</mn><mi>y</mi><mo>=</mo><mn>8</mn></mtd></mtr></mtable></mrow><mspace width="0.222em"></mspace></mrow></math>"#;
    test_braille("French", expr, "⠸⠦⠣⠭⠖⠩⠽⠶⠱⠀⠠⠜⠀⠩⠭⠤⠻⠽⠶⠳")?;
    return Ok(());
}
#[test]
fn function_f_of_x_14_1_01() -> Result<()> {
    let expr= r#"<math><mrow><mi>f</mi><mo stretchy="false">(</mo><mi>x</mi><mo stretchy="false">)</mo></mrow></math>"#;
    test_braille("French", expr, "⠋⠦⠭⠴")?;
    return Ok(());
}

#[test]
fn function_g_prime_14_1_02() -> Result<()> {
    let expr= r#"<math><mrow><msup><mi>g</mi><mo>′</mo></msup><mo stretchy="false">(</mo><mi>x</mi><mo stretchy="false">)</mo><mo>=</mo><mn>3</mn><mi>x</mi><mo>+</mo><mn>4</mn></mrow></math>"#;
    test_braille("French", expr, "⠛⠄⠦⠭⠴⠶⠩⠭⠖⠹")?;
    return Ok(());
}

#[test]
fn function_composition_14_1_03() -> Result<()> {
    let expr= r#"<math><mrow><mi>f</mi><mo>∘</mo><mi>g</mi><mo stretchy="false">(</mo><mi>x</mi><mo stretchy="false">)</mo><mo>=</mo><mi>f</mi><mrow><mo>[</mo><mi>g</mi><mo stretchy="false">(</mo><mi>x</mi><mo stretchy="false">)</mo><mo>]</mo></mrow></mrow></math>"#;
    test_braille("French", expr, "⠋⠸⠼⠛⠦⠭⠴⠶⠋⠷⠛⠦⠭⠴⠾")?;
    return Ok(());
}

#[test]
fn function_partial_derivative_14_1_04() -> Result<()> {
    let expr= r#"<math><mrow><mfrac><mrow><mi>∂</mi><mi>f</mi><mo stretchy="false">(</mo><mi>x</mi><mo>,</mo><mi>y</mi><mo stretchy="false">)</mo></mrow><mrow><mi>∂</mi><mi>x</mi></mrow></mfrac><mo>=</mo><mn>5</mn><mi>x</mi><mi>y</mi><mo>−</mo><mn>7</mn><mi>x</mi></mrow></math>"#;
    test_braille("French", expr, "⠐⠙⠋⠦⠭⠂⠽⠴⠌⠐⠙⠭⠶⠱⠭⠽⠤⠻⠭")?;
    return Ok(());
}

#[test]
fn limit_x_tends_1_14_2_01_corrected() -> Result<()> {
    let expr= r#"<math><mrow><munder><mi>lim</mi><mrow><mi>x</mi><mo>→</mo><mi>+</mi><mi>∞</mi></mrow></munder><mi>f</mi><mo stretchy="false">(</mo><mi>x</mi><mo stretchy="false">)</mo><mo>=</mo><mn>0</mn></mrow></math>"#;
    test_braille("French", expr, "⠇⠊⠍⠢⠢⠰⠭⠸⠱⠖⠘⠉⠆⠋⠦⠭⠴⠶⠼")?;
    return Ok(());
}

#[test]
fn limit_x_tends_and_x_lt_1_14_2_02_corrected_1() -> Result<()> {
    let expr= r#"<math><mrow><munder><mi>lim</mi><mtable><mtr><mtd columnalign="right" style="text-align: right"><mi>x</mi><mo>→</mo><mn>4</mn></mtd></mtr><mtr><mtd columnalign="right" style="text-align: right"><mi>x</mi><mo>&gt;</mo><mn>4</mn></mtd></mtr></mtable></munder><mi>f</mi><mo stretchy="false">(</mo><mi>x</mi><mo stretchy="false">)</mo><mo>=</mo><mi>−</mi><mi>∞</mi></mrow></math>"#;
    test_braille("French", expr, "⠇⠊⠍⠢⠢⠰⠭⠸⠱⠹⠂⠭⠐⠜⠹⠆⠋⠦⠭⠴⠶⠤⠘⠉")?;
    return Ok(());
}

#[test]
fn limit_x_tends_and_x_lt_2_14_2_02_corrected_2() -> Result<()> {
    let expr= r#"<math><mrow><munder><munder><mi>lim</mi><mrow><mi>x</mi><mo>→</mo><mn>4</mn></mrow></munder><mrow><mi>x</mi><mo>&gt;</mo><mn>4</mn></mrow></munder><mi>f</mi><mo stretchy="false">(</mo><mi>x</mi><mo stretchy="false">)</mo><mo>=</mo><mi>−</mi><mi>∞</mi></mrow></math>"#;
    test_braille("French", expr, "⠇⠊⠍⠢⠢⠰⠭⠸⠱⠹⠂⠭⠐⠜⠹⠆⠋⠦⠭⠴⠶⠤⠘⠉")?;
    return Ok(());
}

#[test]
fn limit_x_tends_of_formula_lt_2_14_2_03_corrected() -> Result<()> {
    let expr= r#"<math><mrow><munder><mi>lim</mi><mrow><mi>x</mi><mo>→</mo><mi>+</mi><mi>∞</mi></mrow></munder><mfrac><mrow><mi>ln</mi><mo>⁡</mo><mo stretchy="false">(</mo><mi>x</mi><mo>+</mo><mn>2</mn><mo stretchy="false">)</mo></mrow><mi>x</mi></mfrac><mo>=</mo><mn>0</mn></mrow></math>"#;
    test_braille("French", expr, "⠇⠊⠍⠢⠢⠰⠭⠸⠱⠖⠘⠉⠆⠰⠇⠝⠦⠭⠖⠣⠴⠌⠭⠆⠶⠼")?;
    return Ok(());
}

// #[test]
// fn sign_chart_14_3_01() -> Result<()> {
//     let expr= r#"<math><mtable><mtr><mtd columnalign="center" style="text-align: center"><mi>x</mi></mtd><mtd columnalign="center" style="text-align: center"><mo>−</mo><mi>∞</mi></mtd><mtd columnalign="center" style="text-align: center"></mtd><mtd columnalign="center" style="text-align: center"><mfrac><mn>3</mn><mn>2</mn></mfrac></mtd><mtd columnalign="center" style="text-align: center"></mtd><mtd columnalign="center" style="text-align: center"><mo>+</mo><mi>∞</mi></mtd></mtr><mtr><mtd columnalign="center" style="text-align: center"><mn>2</mn><mi>x</mi><mo>−</mo><mn>3</mn></mtd><mtd columnalign="center" style="text-align: center"></mtd><mtd columnalign="center" style="text-align: center"><mo>−</mo></mtd><mtd columnalign="center" style="text-align: center"><mn>0</mn></mtd><mtd columnalign="center" style="text-align: center"><mo>+</mo></mtd><mtd columnalign="center" style="text-align: center"></mtd></mtr></mtable></math>"#;
//     test_braille("French", expr, "
// ⠭⠀⠀⠀⠸⠤⠘⠉⠀⠀⠩⠌⠣⠀⠀⠀⠖⠘⠉
// ⠒⠒⠒⠒⠺⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒
// ⠣⠭⠤⠩⠸⠀⠀⠀⠤⠀⠀⠼⠀⠀⠖")?;
//     return Ok(());
// }

// #[test]
// fn sign_chart_14_3_02() -> Result<()> {
//     let expr= r#"<math><mtable><mtr><mtd columnalign="center" style="text-align: center"><mi>x</mi></mtd><mtd columnalign="center" style="text-align: center"><mo>−</mo><mi>∞</mi></mtd><mtd columnalign="center" style="text-align: center"></mtd><mtd columnalign="center" style="text-align: center"><mn>7</mn></mtd><mtd columnalign="center" style="text-align: center"></mtd><mtd columnalign="center" style="text-align: center"><mo>+</mo><mi>∞</mi></mtd></mtr><mtr><mtd columnalign="center" style="text-align: center"><mi>f</mi><mi>′</mi><mrow><mo stretchy="true" form="prefix">(</mo><mi>x</mi><mo stretchy="true" form="postfix">)</mo></mrow></mtd><mtd columnalign="center" style="text-align: center"></mtd><mtd columnalign="center" style="text-align: center"><mo>+</mo></mtd><mtd columnalign="center" style="text-align: center"><mn>0</mn></mtd><mtd columnalign="center" style="text-align: center"><mo>−</mo></mtd><mtd columnalign="center" style="text-align: center"></mtd></mtr><mtr><mtd columnalign="center" style="text-align: center"><mi>f</mi><mrow><mo stretchy="true" form="prefix">(</mo><mi>x</mi><mo stretchy="true" form="postfix">)</mo></mrow></mtd><mtd columnalign="center" style="text-align: center"><mo>−</mo><mi>∞</mi></mtd><mtd columnalign="center" style="text-align: center"><mo>↗</mo></mtd><mtd columnalign="center" style="text-align: center"><mo>−</mo><mn>1</mn></mtd><mtd columnalign="center" style="text-align: center"><mo>↘</mo></mtd><mtd columnalign="center" style="text-align: center"><mo>−</mo><mi>∞</mi></mtd></mtr></mtable></math>"#;
//     test_braille("French", expr, "
// ⠀⠭⠀⠀⠀⠀⠸⠤⠘⠉⠀⠀⠀⠀⠀⠻⠀⠀⠀⠀⠀⠖⠘
// ⠒⠒⠒⠒⠒⠺⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒
// ⠋⠄⠦⠭⠴⠸⠀⠀⠀⠀⠖⠀⠀⠀⠀⠼⠀⠀⠀⠀⠤
// ⠋⠦⠭⠴⠀⠸⠤⠘⠉⠀⠘⠱⠀⠤⠡⠀⠨⠱⠀⠤⠘⠉")?;
//     return Ok(());
// }
#[test]
fn integral_from_to_1_14_4_01() -> Result<()> {
    let expr= r#"<math><mrow><msubsup><mo>∫</mo><mi>a</mi><mi>b</mi></msubsup><mrow><mi>f</mi><mo stretchy="false">(</mo><mi>x</mi><mo stretchy="false">)</mo><mi>d</mi><mi>x</mi></mrow></mrow></math>"#;
    test_braille("French", expr, "⠯⠢⠁⠈⠃⠋⠦⠭⠴⠙⠭")?;
    return Ok(());
}

#[test]
fn integral_two_lines_14_4_02() -> Result<()> {
    let expr= r#"<math><mrow><msubsup><mo>∫</mo><mn>0</mn><mfrac><mn>5</mn><mn>2</mn></mfrac></msubsup><mrow><mi>f</mi><mrow><mo stretchy="true" form="prefix">(</mo><mi>x</mi><mo stretchy="true" form="postfix">)</mo></mrow><mi>d</mi><mi>x</mi><mo>=</mo><mi>F</mi><mrow><mo stretchy="true" form="prefix">(</mo><mfrac><mn>5</mn><mn>2</mn></mfrac><mo stretchy="true" form="postfix">)</mo></mrow><mo>−</mo><mi>F</mi><mrow><mo stretchy="true" form="prefix">(</mo><mn>0</mn><mo stretchy="true" form="postfix">)</mo></mrow></mrow></mrow></math>"#;
    test_braille("French", expr, "⠯⠢⠼⠈⠰⠱⠌⠣⠆⠋⠦⠭⠴⠙⠭⠶⠨⠋⠦⠱⠌⠣⠴⠤⠨⠋⠦⠼⠴")?;
    return Ok(());
}
#[test]
fn integral_one_lines14_4_02() -> Result<()> {
    let expr= r#"<math><mrow><msubsup><mo>∫</mo><mn>0</mn><mfrac><mn>5</mn><mn>2</mn></mfrac></msubsup><mrow><mi>f</mi><mrow><mo stretchy="true" form="prefix">(</mo><mi>x</mi><mo stretchy="true" form="postfix">)</mo></mrow><mi>d</mi><mi>x</mi><mo>=</mo><mi>F</mi><mrow><mo stretchy="true" form="prefix">(</mo><mfrac><mn>5</mn><mn>2</mn></mfrac><mo stretchy="true" form="postfix">)</mo></mrow><mo>−</mo><mi>F</mi><mrow><mo stretchy="true" form="prefix">(</mo><mn>0</mn><mo stretchy="true" form="postfix">)</mo></mrow></mrow></mrow></math>"#;
    test_braille("French", expr, "⠯⠢⠼⠈⠰⠱⠌⠣⠆⠋⠦⠭⠴⠙⠭⠶⠨⠋⠦⠱⠌⠣⠴⠤⠨⠋⠦⠼⠴")?;
    return Ok(());
}

#[test]
fn integral_from_to_2_14_4_03() -> Result<()> {
    let expr= r#"<math><mrow><msubsup><mo>∫</mo><mn>1</mn><mn>2</mn></msubsup><mrow><mn>5</mn><mi>x</mi><mi>d</mi><mi>x</mi></mrow></mrow></math>"#;
    test_braille("French", expr, "⠯⠢⠡⠈⠣⠰⠱⠭⠆⠙⠭")?;
    return Ok(());
}

#[test]
fn integral_over_curve_14_4_04() -> Result<()> {
    let expr= r#"<math><mrow><msubsup><mo>∫</mo><mrow><mo stretchy="false">(</mo><mi>C</mi><mo stretchy="false">)</mo></mrow><mrow></mrow></msubsup><mrow><mi>f</mi><mo stretchy="false">(</mo><mi>x</mi><mo stretchy="false">)</mo><mi>d</mi><mi>x</mi></mrow><mspace width="0.27em"></mspace></mrow></math>"#;
    test_braille("French", expr, "⠯⠢⠦⠨⠉⠴⠋⠦⠭⠴⠙⠭")?;
    return Ok(());
}

#[test]
fn integral_double_over_surface_14_4_05() -> Result<()> {
    let expr= r#"<math><mrow><msubsup><mo>∯</mo><mi>S</mi><mrow></mrow></msubsup><mrow><mspace width="0.27em"></mspace><mi>f</mi><mo stretchy="false">(</mo><mi>x</mi><mo>,</mo><mi>y</mi><mo stretchy="false">)</mo><mi>d</mi><mi>x</mi><mi>d</mi><mi>y</mi></mrow></mrow></math>"#;
    test_braille("French", expr, "⠨⠯⠯⠢⠨⠎⠋⠦⠭⠂⠽⠴⠙⠭⠙⠽")?;
    return Ok(());
}

#[test]
fn logarithm_natural_fraction_14_5_01() -> Result<()> {
    let expr= r#"<math><mrow><mi>ln</mi><mo>⁡</mo><mrow><mo>(</mo><mfrac><mi>a</mi><mi>b</mi></mfrac><mo>)</mo></mrow></mrow></math>"#;
    test_braille("French", expr, "⠇⠝⠦⠁⠌⠃⠴")?;
    return Ok(());
}

#[test]
fn logarithm_natural_product_14_5_02() -> Result<()> {
    let expr= r#"<math><mrow><mi>ln</mi><mo>⁡</mo><mspace width="0.333em"></mspace><mi>x</mi><mi>y</mi><mo>=</mo><mi>ln</mi><mo>⁡</mo><mspace width="0.333em"></mspace><mi>x</mi><mo>+</mo><mi>ln</mi><mo>⁡</mo><mspace width="0.333em"></mspace><mi>y</mi></mrow></math>"#;
    test_braille("French", expr, "⠇⠝⠰⠭⠽⠆⠶⠇⠝⠰⠭⠆⠖⠇⠝⠰⠽⠆")?;
    return Ok(());
}

#[test]
fn logarthm_change_base_14_5_03() -> Result<()> {
    let expr: &str= r#"<math><mrow><mi>l</mi><mi>o</mi><msub><mi>g</mi><mn>7</mn></msub><mspace width="0.333em"></mspace><mi>x</mi><mo>=</mo><mfrac><mrow><mi>ln</mi><mo>⁡</mo><mspace width="0.333em"></mspace><mi>x</mi></mrow><mrow><mi>ln</mi><mo>⁡</mo><mspace width="0.333em"></mspace><mn>7</mn></mrow></mfrac></mrow></math>"#;
    test_braille("French", expr, "⠇⠕⠛⠢⠻⠰⠭⠆⠶⠇⠝⠰⠭⠆⠌⠇⠝⠰⠻⠆")?;
    return Ok(());
}

#[test]
fn cos_of_sum_1_14_6_01() -> Result<()> {
    let expr= r#"<math><mrow><mi>cos</mi><mo>(</mo><mi>a</mi><mo>+</mo><mi>b</mi><mo>)</mo><mo>=</mo><mi>cos</mi><mi>a</mi><mi>cos</mi><mi>b</mi><mo>−</mo><mi>sin</mi><mi>a</mi><mi>sin</mi><mi>b</mi></mrow></math>"#;
    test_braille("French", expr, "⠉⠕⠎⠦⠁⠖⠃⠴⠶⠉⠕⠎⠰⠁⠆⠉⠕⠎⠰⠃⠆⠤⠎⠊⠝⠰⠁⠆⠎⠊⠝⠰⠃⠆")?;
    return Ok(());
}

#[test]
fn cos_of_sum_2_14_6_01b() -> Result<()> {
    let expr= r#"<math><mrow><mi>cos</mi><mo>(</mo><mi>a</mi><mo>+</mo><mi>b</mi><mo>)</mo><mo>=</mo><mi>cos</mi><mi>a</mi><mi>cos</mi><mi>b</mi><mo>−</mo><mi>sin</mi><mi>a</mi><mi>sin</mi><mi>b</mi></mrow></math>"#;
    test_braille_prefs("French", vec![("French_UseShortForm", "true")], expr, "⠨⠬⠦⠁⠖⠃⠴⠶⠨⠬⠁⠨⠬⠃⠤⠬⠁⠬⠃")?;
    return Ok(());
}
#[test]
fn cosh_sinh_14_7_01() -> Result<()> {
    let expr= r#"<math><mrow><msup><mrow><mi>cosh</mi></mrow><mn>2</mn></msup><mi>x</mi><mo>−</mo><msup><mrow><mi>sinh</mi></mrow><mn>2</mn></msup><mi>x</mi><mo>=</mo><mn>1</mn></mrow></math>"#;
    test_braille_prefs("French", vec![("French_UseShortForm", "true")], expr, "⠉⠓⠈⠣⠰⠭⠆⠤⠎⠓⠈⠣⠰⠭⠆⠶⠡")?;

    return Ok(());
}

#[test]
fn exists_element_of_15_01() -> Result<()> {
    let expr= r#"<math><mrow><mo>∃</mo><mi>x</mi><mo>:</mo><mspace width="0.333em"></mspace><mi>x</mi><mo>∈</mo><mi>A</mi></mrow></math>"#;
    test_braille("French", expr, "⠸⠡⠭⠒⠭⠘⠡⠨⠁")?;
    return Ok(());
}

#[test]
fn set_formula_15_02() -> Result<()> {
    let expr= r#"<math><mrow><mo>∀</mo><mi>ε</mi><mo>&gt;</mo><mn>0</mn><mspace width="0.40em"></mspace><mo>∃</mo><mi>η</mi><mo>&gt;</mo><mn>0</mn><mspace width="0.167em"></mspace><mo>:</mo><mspace width="0.40em"></mspace><mrow><mo stretchy="true" form="prefix">|</mo><msub><mi>x</mi><mn>2</mn></msub><mo>−</mo><msub><mi>x</mi><mn>1</mn></msub><mo stretchy="true" form="postfix">|</mo></mrow><mo>≤</mo><mi>η</mi><mspace width="0.40em"></mspace><mo>⇒</mo><mspace width="0.40em"></mspace><mrow><mo stretchy="true" form="prefix">|</mo><mi>f</mi><mrow><mo stretchy="true" form="prefix">(</mo><msub><mi>x</mi><mn>2</mn></msub><mo stretchy="true" form="postfix">)</mo></mrow><mo>−</mo><mi>f</mi><mrow><mo stretchy="true" form="prefix">(</mo><msub><mi>x</mi><mn>1</mn></msub><mo stretchy="true" form="postfix">)</mo></mrow><mo stretchy="true" form="postfix">|</mo></mrow><mo>≤</mo><mi>ε</mi></mrow></math>"#;
    test_braille("French", expr, "⠸⠌⠘⠑⠐⠜⠼⠀⠸⠡⠘⠓⠐⠜⠼⠒⠀⠿⠭⠢⠣⠤⠭⠢⠡⠿⠘⠣⠘⠓⠀⠒⠂⠀⠿⠋⠦⠭⠢⠣⠴⠤⠋⠦⠭⠢⠡⠴⠿⠘⠣⠘⠑")?;
    return Ok(());
}
#[test]
fn power_ordinal_a_16_4_07() -> Result<()> {
    let expr= r#"<math><mrow><mo stretchy="false">(</mo><mi>p</mi><mo>+</mo><mi>q</mi><msup><mo stretchy="false">)</mo><mrow><mi>i</mi><mi>è</mi><mi>m</mi><mi>e</mi></mrow></msup></mrow></math>"#;
    test_braille("French", expr, "⠦⠏⠖⠟⠴⠈⠊⠮⠍⠑")?;
    return Ok(());
}

#[test]
fn power_ordinal_b_16_4_07() -> Result<()> {
    let expr= r#"<math><mrow><mo stretchy="false">(</mo><mi>p</mi><mo>+</mo><mi>q</mi><msup><mo stretchy="false">)</mo><mrow><mi>ième</mi></mrow></msup></mrow></math>"#;
    test_braille("French", expr, "⠦⠏⠖⠟⠴⠈⠊⠮⠍⠑")?;
    return Ok(());
}

