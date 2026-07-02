use crate::common::*;
use anyhow::Result;

#[test]
fn script_grouping_regressions() -> Result<()> {
    let cases = vec![
        ("sup_x", r#"<math><msup><mi>a</mi><mi>x</mi></msup></math>"#),
        ("sup_zero", r#"<math><msup><mi>x</mi><mn>0</mn></msup></math>"#),
        ("sup_ten", r#"<math><msup><mi>x</mi><mn>10</mn></msup></math>"#),
        ("sup_minus_x", r#"<math><msup><mi>a</mi><mrow><mo>-</mo><mi>x</mi></mrow></msup></math>"#),
        ("sup_minus_2", r#"<math><msup><mi>a</mi><mrow><mo>-</mo><mn>2</mn></mrow></msup></math>"#),
        ("sup_x_plus_1", r#"<math><msup><mi>a</mi><mrow><mi>x</mi><mo>+</mo><mn>1</mn></mrow></msup></math>"#),
        ("sup_sub_x2", r#"<math><msup><mi>a</mi><msub><mi>x</mi><mn>2</mn></msub></msup></math>"#),
        ("sup_nested", r#"<math><msup><mi>a</mi><msup><mi>x</mi><mn>2</mn></msup></msup></math>"#),
        ("sup_frac", r#"<math><msup><mi>a</mi><mfrac><mn>1</mn><mn>2</mn></mfrac></msup></math>"#),
        ("sup_complex_frac", r#"<math><msup><mi>a</mi><mfrac><mrow><mi>x</mi><mo>+</mo><mn>1</mn></mrow><mi>y</mi></mfrac></msup></math>"#),
        ("sup_sqrt", r#"<math><msup><mi>a</mi><msqrt><mi>x</mi></msqrt></msup></math>"#),
        ("sup_follow_letter", r#"<math><mrow><msup><mi>x</mi><mn>2</mn></msup><mi>y</mi></mrow></math>"#),
        ("sup_follow_number", r#"<math><mrow><msup><mi>x</mi><mn>2</mn></msup><mn>3</mn></mrow></math>"#),
        ("sub_x", r#"<math><msub><mi>a</mi><mi>x</mi></msub></math>"#),
        ("sub_zero", r#"<math><msub><mi>x</mi><mn>0</mn></msub></math>"#),
        ("sub_ten", r#"<math><msub><mi>x</mi><mn>10</mn></msub></math>"#),
        ("sub_digits_sequence", r#"<math><msub><mi>D</mi><mrow><mn>1</mn><mn>3</mn></mrow></msub></math>"#),
        ("sub_infinity", r#"<math><msub><mi>t</mi><mi>&#x221E;</mi></msub></math>"#),
        ("sub_minus_x", r#"<math><msub><mi>a</mi><mrow><mo>-</mo><mi>x</mi></mrow></msub></math>"#),
        ("sub_minus_2", r#"<math><msub><mi>a</mi><mrow><mo>-</mo><mn>2</mn></mrow></msub></math>"#),
        ("sub_x_plus_1", r#"<math><msub><mi>a</mi><mrow><mi>x</mi><mo>+</mo><mn>1</mn></mrow></msub></math>"#),
        ("sub_nested", r#"<math><msub><mi>a</mi><msub><mi>x</mi><mn>2</mn></msub></msub></math>"#),
        ("sub_follow_letter", r#"<math><mrow><msub><mi>x</mi><mn>2</mn></msub><mi>y</mi></mrow></math>"#),
        ("sub_follow_number", r#"<math><mrow><msub><mi>x</mi><mn>2</mn></msub><mn>3</mn></mrow></math>"#),
        ("subsup_numeric", r#"<math><msubsup><mi>x</mi><mn>1</mn><mn>2</mn></msubsup></math>"#),
        ("subsup_i_n", r#"<math><msubsup><mi>x</mi><mi>i</mi><mi>n</mi></msubsup></math>"#),
        ("subsup_grouped_sub", r#"<math><msubsup><mi>x</mi><mrow><mi>i</mi><mo>+</mo><mn>1</mn></mrow><mn>2</mn></msubsup></math>"#),
        ("subsup_grouped_sup", r#"<math><msubsup><mi>x</mi><mn>0</mn><mrow><mi>n</mi><mo>-</mo><mn>1</mn></mrow></msubsup></math>"#),
        ("subsup_nested_sup", r#"<math><msubsup><mi>x</mi><mn>0</mn><msup><mi>n</mi><mn>2</mn></msup></msubsup></math>"#),
        ("negative_base_sup", r#"<math><msup><mrow><mo>-</mo><mi>x</mi></mrow><mn>2</mn></msup></math>"#),
        ("paren_base_sup", r#"<math><msup><mrow><mo>(</mo><mi>x</mi><mo>+</mo><mn>1</mn><mo>)</mo></mrow><mn>2</mn></msup></math>"#),
        ("frac_base_sup", r#"<math><msup><mfrac><mn>1</mn><mi>x</mi></mfrac><mn>2</mn></msup></math>"#),
        ("root_base_sup", r#"<math><msup><msqrt><mi>x</mi></msqrt><mn>2</mn></msup></math>"#),
        ("sup_on_function", r#"<math><mrow><msup><mi>sin</mi><mn>2</mn></msup><mi>x</mi></mrow></math>"#),
        ("log_sub_sup", r#"<math><mrow><msubsup><mi>log</mi><mn>2</mn><mn>3</mn></msubsup><mi>x</mi></mrow></math>"#),
        ("root_index_group", r#"<math><mroot><mi>x</mi><mrow><mi>n</mi><mo>+</mo><mn>1</mn></mrow></mroot></math>"#),
        ("root_index_sub", r#"<math><mroot><mi>x</mi><msub><mi>n</mi><mn>2</mn></msub></mroot></math>"#),
        ("frac_num_group", r#"<math><mfrac><mrow><mi>x</mi><mo>+</mo><mn>1</mn></mrow><mi>y</mi></mfrac></math>"#),
        ("frac_den_group", r#"<math><mfrac><mi>x</mi><mrow><mi>y</mi><mo>+</mo><mn>1</mn></mrow></mfrac></math>"#),
        ("nested_frac", r#"<math><mfrac><mfrac><mn>1</mn><mi>x</mi></mfrac><mfrac><mn>1</mn><mi>y</mi></mfrac></mfrac></math>"#),
        ("sup_after_fraction", r#"<math><msup><mfrac><mrow><mi>x</mi><mo>+</mo><mn>1</mn></mrow><mrow><mi>y</mi><mo>-</mo><mn>1</mn></mrow></mfrac><mn>2</mn></msup></math>"#),
        ("sub_after_fraction", r#"<math><msub><mfrac><mrow><mi>x</mi><mo>+</mo><mn>1</mn></mrow><mrow><mi>y</mi><mo>-</mo><mn>1</mn></mrow></mfrac><mi>i</mi></msub></math>"#),
        ("tensor_like", r#"<math><mrow><msubsup><mi>T</mi><mi>i</mi><mi>j</mi></msubsup><msubsup><mi>x</mi><mi>j</mi><mi>k</mi></msubsup></mrow></math>"#),
        ("mmultiscripts_left_sub", r#"<math><mmultiscripts><mi>F</mi><mprescripts/><mi>k</mi><none/></mmultiscripts></math>"#),
        ("mmultiscripts_left_sup", r#"<math><mmultiscripts><mi>W</mi><mprescripts/><none/><mn>4</mn></mmultiscripts></math>"#),
        ("mmultiscripts_both_sides", r#"<math><mmultiscripts><mi>T</mi><mi>i</mi><mi>j</mi><mprescripts/><mi>r</mi><mi>s</mi></mmultiscripts></math>"#),
        ("pre_negative_power", r#"<math><mrow><mn>2</mn><msup><mi>x</mi><mrow><mo>-</mo><mn>1</mn></mrow></msup></mrow></math>"#),
        ("power_of_power_follow", r#"<math><mrow><msup><msup><mi>x</mi><mn>2</mn></msup><mn>3</mn></msup><mi>y</mi></mrow></math>"#),
        ("subscripted_power_follow", r#"<math><mrow><msup><msub><mi>x</mi><mn>2</mn></msub><mn>3</mn></msup><mi>y</mi></mrow></math>"#),
        ("power_subscript_follow", r#"<math><mrow><msub><msup><mi>x</mi><mn>2</mn></msup><mn>3</mn></msub><mi>y</mi></mrow></math>"#),
    ];

    let expected = std::collections::HashMap::from([
        ("sup_x", "⠠⠁⠌⠭⠱"),
        ("sup_zero", "⠠⠭⠌⠴"),
        ("sup_ten", "⠠⠭⠌⠂⠴"),
        ("sup_minus_x", "⠠⠁⠌⠀⠤⠭⠱"),
        ("sup_minus_2", "⠠⠁⠌⠤⠆"),
        ("sup_x_plus_1", "⠠⠁⠌⠭⠀⠖⠼⠁⠱"),
        ("sup_sub_x2", "⠠⠁⠌⠭⠡⠆⠱"),
        ("sup_nested", "⠠⠁⠌⠭⠌⠆⠱"),
        ("sup_frac", "⠠⠁⠌⠼⠁⠆⠱"),
        ("sup_complex_frac", "⠠⠁⠌⠐⠆⠭⠀⠖⠼⠁⠀⠳⠠⠽⠰⠱"),
        ("sup_sqrt", "⠠⠁⠌⠩⠱⠭⠹⠱"),
        ("sup_follow_letter", "⠠⠭⠌⠆⠽"),
        ("sup_follow_number", "⠠⠭⠌⠆⠼⠉"),
        ("sub_x", "⠠⠁⠡⠭⠱"),
        ("sub_zero", "⠠⠭⠡⠴"),
        ("sub_ten", "⠠⠭⠡⠂⠴"),
        ("sub_digits_sequence", "⠨⠙⠡⠂⠒"),
        ("sub_infinity", "⠠⠞⠡⠻"),
        ("sub_minus_x", "⠠⠁⠡⠀⠤⠭⠱"),
        ("sub_minus_2", "⠠⠁⠡⠤⠆"),
        ("sub_x_plus_1", "⠠⠁⠡⠐⠭⠀⠖⠼⠁⠱"),
        ("sub_nested", "⠠⠁⠡⠭⠡⠆⠱"),
        ("sub_follow_letter", "⠠⠭⠡⠆⠽"),
        ("sub_follow_number", "⠠⠭⠡⠆⠼⠉"),
        ("subsup_numeric", "⠠⠭⠡⠼⠁⠌⠼⠃⠱"),
        ("subsup_i_n", "⠠⠭⠡⠊⠌⠝⠱"),
        ("subsup_grouped_sub", "⠠⠭⠡⠐⠊⠀⠖⠼⠁⠌⠼⠃⠱"),
        ("subsup_grouped_sup", "⠠⠭⠡⠼⠚⠌⠠⠝⠀⠤⠼⠁⠱"),
        ("subsup_nested_sup", "⠠⠭⠡⠼⠚⠌⠠⠝⠌⠆⠱"),
        ("negative_base_sup", "⠤⠠⠭⠌⠆"),
        ("paren_base_sup", "⠣⠠⠭⠀⠖⠼⠁⠜⠌⠆"),
        ("frac_base_sup", "⠼⠁⠳⠠⠭⠌⠆"),
        ("root_base_sup", "⠩⠱⠠⠭⠹⠌⠆"),
        ("sup_on_function", "⠫⠎⠌⠆⠠⠭"),
        ("log_sub_sup", "⠫⠇⠡⠼⠃⠌⠼⠉⠱⠠⠭"),
        ("root_index_group", "⠩⠠⠝⠀⠖⠼⠁⠱⠠⠭⠹"),
        ("root_index_sub", "⠩⠠⠝⠡⠆⠱⠭⠹"),
        ("frac_num_group", "⠆⠠⠭⠀⠖⠼⠁⠀⠳⠠⠽⠰"),
        ("frac_den_group", "⠆⠠⠭⠀⠳⠽⠀⠖⠼⠁⠰"),
        ("nested_frac", "⠆⠆⠼⠁⠀⠳⠠⠭⠰⠀⠳⠆⠼⠁⠀⠳⠠⠽⠰⠰"),
        ("sup_after_fraction", "⠆⠠⠭⠀⠖⠼⠁⠀⠳⠠⠽⠀⠤⠼⠁⠰⠌⠆"),
        ("sub_after_fraction", "⠆⠠⠭⠀⠖⠼⠁⠀⠳⠠⠽⠀⠤⠼⠁⠰⠡⠠⠊⠱"),
        ("tensor_like", "⠨⠞⠡⠠⠊⠌⠚⠱⠭⠡⠚⠌⠅⠱"),
        ("mmultiscripts_left_sub", "⠡⠠⠅⠱⠨⠋"),
        ("mmultiscripts_left_sup", "⠌⠲⠨⠺"),
        ("mmultiscripts_both_sides", "⠡⠠⠗⠱⠌⠎⠱⠨⠞⠡⠠⠊⠱⠌⠚⠱"),
        ("pre_negative_power", "⠼⠃⠠⠭⠌⠤⠂"),
        ("power_of_power_follow", "⠠⠭⠌⠆⠌⠒⠽"),
        ("subscripted_power_follow", "⠠⠭⠡⠆⠌⠒⠽"),
        ("power_subscript_follow", "⠠⠭⠌⠆⠡⠒⠽"),
    ]);

    for (label, expr) in cases {
        test_braille("Russian", expr, expected[label])?;
    }
    return Ok(());
}

#[test]
fn numbers_and_operators() -> Result<()> {
    let expr = r#"<math><mrow><mn>5</mn><mo>+</mo><mn>12</mn><mo>=</mo><mn>17</mn></mrow></math>"#;
    test_braille("Russian", expr, "⠼⠑⠀⠖⠼⠁⠃⠀⠶⠼⠁⠛")?;
    return Ok(());
}

#[test]
fn source_general_math_text_rules() -> Result<()> {
    let expr = r#"<math><mrow><mo>&#x2116;</mo><mn>5</mn><mo>,</mo><mo>&#xA7;</mo><mn>2</mn></mrow></math>"#;
    test_braille("Russian", expr, "⠝⠼⠑⠠⠂⠬⠼⠃")?;

    let expr = r#"<math><mrow><mn>1</mn><mo>+</mo><mn>2</mn><mo>&#x21B5;</mo><mn>3</mn><mo>+</mo><mo>&#x2026;</mo><mo>+</mo><mn>10</mn><mo>&#x23CE;</mo><mo>=</mo><mn>55</mn><mo>.</mo></mrow></math>"#;
    test_braille("Russian", expr, "⠼⠁⠀⠖⠼⠃⠐⠼⠉⠀⠖⠠⠄⠀⠖⠼⠁⠚⠠⠀⠶⠼⠑⠑⠠⠲")?;

    let expr = r#"<math><mrow><mi>x</mi><mo>&#x2370;</mo><mfrac><mi>y</mi><mn>2</mn></mfrac></mrow></math>"#;
    test_braille("Russian", expr, "⠠⠭⠐⠽⠳⠆")?;

    let expr = r#"<math><mrow><mn>1</mn><mo>+</mo><mn>2</mn><mo>&#x21B5;</mo><mn>3</mn><mo>+</mo><mn>4</mn></mrow></math>"#;
    test_braille("Russian", expr, "⠼⠁⠀⠖⠼⠃⠐⠼⠉⠀⠖⠼⠙")?;

    let expr = r#"<math><mrow><mn>1</mn><mo>+</mo><mn>2</mn><mo>&#x23CE;</mo><mn>3</mn><mo>+</mo><mn>4</mn></mrow></math>"#;
    test_braille("Russian", expr, "⠼⠁⠀⠖⠼⠃⠠⠼⠉⠀⠖⠼⠙")?;
    return Ok(());
}

#[test]
fn percent_and_special_marks() -> Result<()> {
    let expr = r#"<math><mrow><mn>25</mn><mo>%</mo></mrow></math>"#;
    test_braille("Russian", expr, "⠼⠃⠑⠼⠍⠴")?;

    let expr = r#"<math><mn>0,56</mn></math>"#;
    test_braille("Russian", expr, "⠼⠚⠂⠑⠋")?;
    return Ok(());
}

#[test]
fn labeled_table_rows() -> Result<()> {
    let expr = r#"<math><mtable><mlabeledtr><mtd><mtext>(1)</mtext></mtd><mtd><mi>x</mi></mtd><mtd><mo>=</mo></mtd><mtd><mn>0</mn></mtd></mlabeledtr></mtable></math>"#;
    test_braille("Russian", expr, "⠍⠑⠞⠅⠁⠀⠎⠞⠗⠕⠅⠊⠀⠲⠣⠼⠁⠜⠀⠠⠭⠀⠶⠀⠼⠚")?;
    return Ok(());
}

#[test]
fn fraction() -> Result<()> {
    let expr = r#"<math><mfrac><mi>x</mi><mn>2</mn></mfrac></math>"#;
    test_braille("Russian", expr, "⠠⠭⠳⠆")?;
    return Ok(());
}

#[test]
fn scripts_and_root() -> Result<()> {
    let expr = r#"<math><mrow><msup><mi>x</mi><mn>2</mn></msup><mo>+</mo><msqrt><mi>y</mi></msqrt></mrow></math>"#;
    test_braille("Russian", expr, "⠠⠭⠌⠆⠀⠖⠩⠱⠽⠹")?;
    return Ok(());
}

#[test]
fn cyrillic_text() -> Result<()> {
    let expr = r#"<math><mtext>угол</mtext></math>"#;
    test_braille("Russian", expr, "⠥⠛⠕⠇")?;
    return Ok(());
}

#[test]
fn wikipedia_linear_parens_flat() -> Result<()> {
    let expr = r#"<math><mrow><mn>3</mn><mo>&#x22C5;</mo><mrow><mo>(</mo><mn>9</mn><mo>-</mo><mn>7</mn><mo>)</mo></mrow><mo>=</mo><mn>6</mn></mrow></math>"#;
    test_braille("Russian", expr, "⠼⠉⠄⠣⠼⠊⠀⠤⠼⠛⠜⠀⠶⠼⠋")?;
    return Ok(());
}

#[test]
fn source_arithmetic_examples() -> Result<()> {
    let expr = r#"<math><mrow><mn>24</mn><mo>&#x22C5;</mo><mn>81</mn><mo>=</mo><mn>1944</mn></mrow></math>"#;
    test_braille("Russian", expr, "⠼⠃⠙⠄⠼⠓⠁⠀⠶⠼⠁⠊⠙⠙")?;

    let expr = r#"<math><mrow><mn>783</mn><mo>:</mo><mn>9</mn><mo>=</mo><mn>87</mn></mrow></math>"#;
    test_braille("Russian", expr, "⠼⠛⠓⠉⠀⠲⠼⠊⠀⠶⠼⠓⠛")?;

    let expr = r#"<math><mrow><mn>12</mn><mo>&#xD7;</mo><mn>35</mn><mo>=</mo><mn>420</mn></mrow></math>"#;
    test_braille("Russian", expr, "⠼⠁⠃⠀⠦⠼⠉⠑⠀⠶⠼⠙⠃⠚")?;

    let expr = r#"<math><mtable intent=":column-arithmetic"><mtr><mtd><mn>7456</mn></mtd></mtr><mtr><mtd><mrow><mo>+</mo><mn>5623</mn></mrow></mtd></mtr><mtr><mtd><mrow><mo>=</mo><mn>13079</mn></mrow></mtd></mtr></mtable></math>"#;
    test_braille("Russian", expr, "⠼⠛⠙⠑⠋⠨⠳⠖⠑⠋⠃⠉⠨⠳⠶⠁⠉⠚⠛⠊")?;

    let expr = r#"<math><mtable intent=":column-arithmetic"><mtr><mtd><mn>78650</mn></mtd></mtr><mtr><mtd><mrow><mo>-</mo><mn>1952</mn></mrow></mtd></mtr><mtr><mtd><mrow><mo>=</mo><mn>76698</mn></mrow></mtd></mtr></mtable></math>"#;
    test_braille("Russian", expr, "⠼⠛⠓⠋⠑⠚⠨⠳⠤⠁⠊⠑⠃⠨⠳⠶⠛⠋⠋⠊⠓")?;

    let expr = r#"<math><mtable intent=":column-arithmetic"><mtr><mtd><mn>327</mn></mtd></mtr><mtr><mtd><mrow><mo>&#xD7;</mo><mn>548</mn></mrow></mtd></mtr><mtr><mtd><mn>2616</mn></mtd></mtr><mtr><mtd><mn>1308</mn></mtd></mtr><mtr><mtd><mrow><mo>=</mo><mn>179196</mn></mrow></mtd></mtr></mtable></math>"#;
    test_braille("Russian", expr, "⠼⠉⠃⠛⠨⠳⠦⠑⠙⠓⠨⠳⠃⠋⠁⠋⠨⠳⠁⠉⠚⠓⠨⠳⠶⠁⠛⠊⠁⠊⠋")?;
    return Ok(());
}

#[test]
fn nested_fraction_and_root() -> Result<()> {
    let expr = r#"<math><mfrac><mrow><mi>x</mi><mo>+</mo><msqrt><mfrac><mn>1</mn><mi>y</mi></mfrac></msqrt></mrow><mrow><mi>x</mi><mo>-</mo><mi>y</mi></mrow></mfrac></math>"#;
    test_braille("Russian", expr, "⠆⠠⠭⠀⠖⠩⠱⠼⠁⠳⠠⠽⠹⠀⠳⠭⠀⠤⠽⠰")?;
    return Ok(());
}

#[test]
fn source_simple_fractions_scripts_roots() -> Result<()> {
    let expr = r#"<math><mfrac><mn>1</mn><mn>2</mn></mfrac></math>"#;
    test_braille("Russian", expr, "⠼⠁⠆")?;

    let expr = r#"<math><mfrac><mi>a</mi><mn>3</mn></mfrac></math>"#;
    test_braille("Russian", expr, "⠠⠁⠳⠒")?;

    let expr = r#"<math><msub><mi>b</mi><mn>7</mn></msub></math>"#;
    test_braille("Russian", expr, "⠠⠃⠡⠶")?;

    let expr = r#"<math><mroot><mi>x</mi><mn>3</mn></mroot></math>"#;
    test_braille("Russian", expr, "⠩⠒⠱⠠⠭⠹")?;
    return Ok(());
}

#[test]
fn source_thin_fraction_rules() -> Result<()> {
    let expr = r#"<math><mfrac><mi>a</mi><mn>3,2</mn></mfrac></math>"#;
    test_braille("Russian", expr, "⠠⠁⠳⠼⠉⠂⠃")?;

    let expr = r#"<math><mfrac><mi>a</mi><mrow><mo>-</mo><mn>3</mn></mrow></mfrac></math>"#;
    test_braille("Russian", expr, "⠠⠁⠳⠤⠒")?;

    let expr = r#"<math><mrow><mn>3</mn><mo>&#x22C5;</mo><mfrac><mrow><mi>x</mi><mo>+</mo><mi>y</mi></mrow><mi>z</mi></mfrac></mrow></math>"#;
    test_braille("Russian", expr, "⠼⠉⠄⠆⠠⠭⠀⠖⠽⠀⠳⠵⠰")?;

    let expr = r#"<math><mrow><mfrac><mi>x</mi><mi>y</mi></mfrac><mo>&#x22C5;</mo><mi>z</mi></mrow></math>"#;
    test_braille("Russian", expr, "⠠⠭⠳⠽⠄⠵")?;

    let expr = r#"<math><msup><mi>x</mi><mfrac><mrow><mi>a</mi><mo>+</mo><mi>b</mi></mrow><mrow><mi>a</mi><mo>-</mo><mi>b</mi></mrow></mfrac></msup></math>"#;
    test_braille("Russian", expr, "⠠⠭⠌⠐⠆⠁⠀⠖⠃⠀⠳⠁⠀⠤⠃⠰⠱")?;
    return Ok(());
}

#[test]
fn source_index_rules() -> Result<()> {
    let expr = r#"<math><msub><mi>D</mi><mrow><mn>1</mn><mo>,</mo><mn>3</mn></mrow></msub></math>"#;
    test_braille("Russian", expr, "⠨⠙⠡⠼⠁⠠⠂⠼⠉⠱")?;

    let expr = r#"<math><msub><mi>P</mi><mrow><mi>s</mi><mo>,</mo><mn>1</mn></mrow></msub></math>"#;
    test_braille("Russian", expr, "⠨⠏⠡⠠⠎⠠⠂⠼⠁⠱")?;

    let expr = r#"<math><msubsup><mi>a</mi><mrow><mn>2</mn><mi>k</mi></mrow><mrow><mi>n</mi><mo>+</mo><mn>1</mn></mrow></msubsup></math>"#;
    test_braille("Russian", expr, "⠠⠁⠡⠼⠃⠠⠅⠱⠌⠝⠀⠖⠼⠁⠱")?;

    let expr = r#"<math><msub><mi>Z</mi><mrow><mo>+</mo><mi>&#x221E;</mi></mrow></msub></math>"#;
    test_braille("Russian", expr, "⠨⠵⠡⠐⠀⠖⠻⠱")?;

    let expr = r#"<math><msub><mi>a</mi><mrow><mo>-</mo><mfrac><mrow><mi>n</mi><mo>+</mo><mn>1</mn></mrow><mn>2</mn></mfrac></mrow></msub></math>"#;
    test_braille("Russian", expr, "⠠⠁⠡⠐⠀⠤⠆⠝⠀⠖⠼⠁⠀⠳⠼⠃⠰⠱")?;
    return Ok(());
}

#[test]
fn source_gost_numbers_fractions_and_sets() -> Result<()> {
    let expr = r#"<math><mn>0,56</mn></math>"#;
    test_braille("Russian", expr, "⠼⠚⠂⠑⠋")?;

    let expr = r#"<math><mrow><mn>2</mn><mo>/</mo><mn>3</mn></mrow></math>"#;
    test_braille("Russian", expr, "⠼⠃⠠⠌⠼⠉")?;

    let expr = r#"<math><mrow><mi>a</mi><mo>/</mo><mi>b</mi></mrow></math>"#;
    test_braille("Russian", expr, "⠠⠁⠠⠌⠃")?;

    let expr = r#"<math><mrow><mi>км</mi><mo>/</mo><mi>ч</mi></mrow></math>"#;
    test_braille("Russian", expr, "⠅⠍⠠⠌⠟")?;

    let expr = r#"<math><mrow><mn>25</mn><mtext>кг</mtext></mrow></math>"#;
    test_braille("Russian", expr, "⠼⠃⠑⠅⠛")?;

    let expr = r#"<math><mrow><mn>12</mn><mo intent=":blank">?</mo><mn>4</mn></mrow></math>"#;
    test_braille("Russian", expr, "⠼⠁⠃⠬⠙")?;

    let expr = r#"<math><mrow><mo>[</mo><mn>1</mn><mo>,</mo><mn>4</mn><mo>]</mo><mo>\</mo><mo>{</mo><mn>4</mn><mo>}</mo><mo>=</mo><mo>[</mo><mn>1</mn><mo>,</mo><mn>4</mn><mo>)</mo></mrow></math>"#;
    test_braille("Russian", expr, "⠷⠼⠁⠠⠂⠼⠙⠾⠀⠰⠤⠪⠼⠙⠕⠀⠶⠷⠼⠁⠠⠂⠼⠙⠜")?;

    let expr = r#"<math><mrow><mn>5</mn><mo>|</mo><mi>x</mi></mrow></math>"#;
    test_braille("Russian", expr, "⠼⠑⠸⠠⠭")?;

    let expr = r#"<math><mrow><mi>x</mi><mo>&#x2208;</mo><mi>A</mi></mrow></math>"#;
    test_braille("Russian", expr, "⠠⠭⠀⠐⠪⠀⠨⠁")?;

    let expr = r#"<math><mrow><mi>x</mi><mo>&#x2209;</mo><mi>A</mi></mrow></math>"#;
    test_braille("Russian", expr, "⠠⠭⠘⠪⠨⠁")?;

    let expr = r#"<math><mrow><mi>A</mi><mo>&#x2209;</mo><mi>B</mi></mrow></math>"#;
    test_braille("Russian", expr, "⠨⠁⠘⠪⠃")?;

    let expr = r#"<math><mrow><mi>A</mi><mo>&#x2282;</mo><mi>B</mi></mrow></math>"#;
    test_braille("Russian", expr, "⠨⠁⠀⠯⠀⠃")?;

    let expr = r#"<math><mrow><mi>D</mi><mo>&#x2284;</mo><mi>E</mi></mrow></math>"#;
    test_braille("Russian", expr, "⠨⠙⠈⠯⠑")?;

    let expr = r#"<math><mrow><mi>A</mi><mo>&#x2284;</mo><mi>B</mi></mrow></math>"#;
    test_braille("Russian", expr, "⠨⠁⠈⠯⠃")?;

    let expr = r#"<math><mrow><mi>A</mi><mo>&#x2229;</mo><mi>B</mi></mrow></math>"#;
    test_braille("Russian", expr, "⠨⠁⠀⠰⠲⠃")?;

    let expr = r#"<math><mrow><mi>A</mi><mo>&#x222A;</mo><mi>B</mi></mrow></math>"#;
    test_braille("Russian", expr, "⠨⠁⠀⠰⠴⠃")?;

    let expr = r#"<math><mrow><mi>A</mi><mo>&#x222A;</mo><mi>B</mi><mo>\</mo><mi>C</mi><mo>&#x2229;</mo><mi>D</mi></mrow></math>"#;
    test_braille("Russian", expr, "⠨⠁⠀⠰⠴⠃⠀⠰⠤⠉⠀⠰⠲⠙")?;

    let expr = r#"<math><mrow><mo>(</mo><mn>3</mn><mo>,</mo><mn>5</mn><mo>]</mo><mo>&#x2229;</mo><mo>[</mo><mn>7</mn><mo>,</mo><mi>&#x221E;</mi><mo>)</mo><mo>=</mo><mi>&#x2205;</mi></mrow></math>"#;
    test_braille("Russian", expr, "⠣⠼⠉⠠⠂⠼⠑⠾⠀⠰⠲⠷⠼⠛⠠⠂⠻⠜⠀⠶⠈⠴")?;

    let expr = r#"<math><mrow><mn>36,6</mn><mo>&#x2103;</mo></mrow></math>"#;
    test_braille("Russian", expr, "⠼⠉⠋⠂⠋⠨⠴⠨⠉")?;
    return Ok(());
}

#[test]
fn source_mixed_numbers_and_periodic_decimals() -> Result<()> {
    let expr = r#"<math><mn>0,4(71)</mn></math>"#;
    test_braille("Russian", expr, "⠼⠚⠂⠙⠣⠛⠁⠜")?;

    let expr = r#"<math><mn>1,(523)</mn></math>"#;
    test_braille("Russian", expr, "⠼⠁⠂⠣⠑⠃⠉⠜")?;

    let expr = r#"<math><mrow><mn>0,4</mn><mo>(</mo><mn>71</mn><mo>)</mo></mrow></math>"#;
    test_braille("Russian", expr, "⠼⠚⠂⠙⠣⠛⠁⠜")?;

    let expr = r#"<math><mrow><mn>5</mn><mfrac><mn>3</mn><mn>8</mn></mfrac></mrow></math>"#;
    test_braille("Russian", expr, "⠼⠑⠼⠉⠦")?;

    let expr = r#"<math><mrow><mn>2</mn><mfrac><mrow><mn>24</mn><mo>+</mo><mn>4</mn><mo>-</mo><mn>15</mn></mrow><mn>24</mn></mfrac></mrow></math>"#;
    test_braille("Russian", expr, "⠼⠃⠐⠆⠼⠃⠙⠀⠖⠼⠙⠀⠤⠼⠁⠑⠀⠳⠼⠃⠙⠰")?;
    return Ok(());
}

#[test]
fn latin_alphabet_indicators() -> Result<()> {
    let expr = r#"<math><mrow><mi>x</mi><mo>+</mo><mi>A</mi><mo>+</mo><mi>y</mi><mo>+</mo><mi>B</mi><mo>=</mo><mi>x</mi><mo>+</mo><mi>y</mi><mo>+</mo><mi>A</mi><mo>+</mo><mi>B</mi></mrow></math>"#;
    test_braille("Russian", expr, "⠠⠭⠀⠖⠨⠁⠀⠖⠠⠽⠀⠖⠨⠃⠀⠶⠠⠭⠀⠖⠽⠀⠖⠨⠁⠀⠖⠃")?;

    let expr = r#"<math><mi>MCDLXIV</mi></math>"#;
    test_braille("Russian", expr, "⠨⠍⠉⠙⠇⠭⠊⠧")?;
    return Ok(());
}

#[test]
fn alphabet_indicators_after_numbers_and_greek() -> Result<()> {
    let expr = r#"<math><mrow><mn>2</mn><mo>&#x22C5;</mo><mi>x</mi><mo>+</mo><mn>15</mn><mo>=</mo><mn>23</mn></mrow></math>"#;
    test_braille("Russian", expr, "⠼⠃⠄⠠⠭⠀⠖⠼⠁⠑⠀⠶⠼⠃⠉")?;

    let expr = r#"<math><mrow><mi>L</mi><mo>=</mo><mn>2</mn><mi>&#x3C0;</mi><mi>r</mi></mrow></math>"#;
    test_braille("Russian", expr, "⠨⠇⠀⠶⠼⠃⠰⠏⠠⠗")?;

    let expr = r#"<math><mrow><mi>&#x3B1;</mi><mo>+</mo><mi>&#x3B2;</mi><mo>=</mo><mi>&#x391;</mi><mo>+</mo><mi>&#x392;</mi></mrow></math>"#;
    test_braille("Russian", expr, "⠰⠁⠀⠖⠃⠀⠶⠸⠁⠀⠖⠃")?;
    return Ok(());
}

#[test]
fn source_typeform_and_mathvariant_indicators() -> Result<()> {
    let expr = r#"<math><mi mathvariant="bold">x</mi></math>"#;
    test_braille("Russian", expr, "⠻⠠⠭⠻")?;

    let expr = r#"<math><mi mathvariant="italic">y</mi></math>"#;
    test_braille("Russian", expr, "⠸⠠⠽⠸")?;

    let expr = r#"<math><mi mathvariant="bold-italic">z</mi></math>"#;
    test_braille("Russian", expr, "⠻⠸⠠⠵⠸⠻")?;

    let expr = r#"<math><mi mathvariant="bold">AB</mi></math>"#;
    test_braille("Russian", expr, "⠻⠨⠁⠃⠻")?;

    let expr = r#"<math><mrow><mi mathvariant="bold">x</mi><mo>+</mo><mi>y</mi></mrow></math>"#;
    test_braille("Russian", expr, "⠻⠠⠭⠻⠀⠖⠽")?;

    let expr = r#"<math><mi>&#x1D431;</mi></math>"#;
    test_braille("Russian", expr, "⠻⠠⠭⠻")?;

    let expr = r#"<math><mi>&#x1D465;</mi></math>"#;
    test_braille("Russian", expr, "⠸⠠⠭⠸")?;
    return Ok(());
}

#[test]
fn wikipedia_times_divide() -> Result<()> {
    let expr = r#"<math><mn>6</mn><mo>&#xD7;</mo><mn>7</mn><mo>:</mo><mn>14</mn><mo>=</mo><mn>3</mn></math>"#;
    test_braille("Russian", expr, "⠼⠋⠀⠦⠼⠛⠀⠲⠼⠁⠙⠀⠶⠼⠉")?;
    return Ok(());
}

#[test]
fn wikipedia_linear_parens() -> Result<()> {
    let expr = r#"<math><mn>3</mn><mo>&#xB7;</mo><mo>(</mo><mn>9</mn><mo>&#x2212;</mo><mn>7</mn><mo>)</mo><mo>=</mo><mn>6</mn></math>"#;
    test_braille("Russian", expr, "⠼⠉⠄⠣⠼⠊⠀⠤⠼⠛⠜⠀⠶⠼⠋")?;
    return Ok(());
}

#[test]
fn wikipedia_sqrt() -> Result<()> {
    let expr = r#"<math><msqrt><mn>10000</mn></msqrt><mo>&lt;</mo><mn>101</mn></math>"#;
    test_braille("Russian", expr, "⠩⠱⠼⠁⠚⠚⠚⠚⠹⠀⠪⠀⠼⠁⠚⠁")?;

    let expr = r#"<math><mrow><msqrt><mi>a</mi></msqrt><mo>&#x22C5;</mo><mi>b</mi></mrow></math>"#;
    test_braille("Russian", expr, "⠩⠱⠠⠁⠹⠄⠃")?;
    return Ok(());
}

#[test]
fn source_functions_logs_derivatives() -> Result<()> {
    let expr = r#"<math><mrow><mi>cos</mi><mi>&#x3B1;</mi></mrow></math>"#;
    test_braille("Russian", expr, "⠫⠉⠰⠁")?;

    let expr = r#"<math><mrow><mi>tg</mi><mi>x</mi><mo>&#x22C5;</mo><mi>ctg</mi><mi>x</mi><mo>=</mo><mn>1</mn></mrow></math>"#;
    test_braille("Russian", expr, "⠫⠞⠠⠭⠄⠫⠉⠞⠠⠭⠀⠶⠼⠁")?;

    let expr = r#"<math><mrow><msup><mi>sin</mi><mn>2</mn></msup><mi>x</mi></mrow></math>"#;
    test_braille("Russian", expr, "⠫⠎⠌⠆⠠⠭")?;

    let expr = r#"<math><mrow><msup><mi>tg</mi><mn>3</mn></msup><mfrac><mrow><mi>&#x3B1;</mi><mo>+</mo><mi>&#x3B2;</mi></mrow><mn>2</mn></mfrac></mrow></math>"#;
    test_braille("Russian", expr, "⠫⠞⠌⠒⠐⠆⠰⠁⠀⠖⠃⠀⠳⠼⠃⠰")?;

    let expr = r#"<math><msup><mrow><mo>(</mo><mi>ctg</mi><mi>&#x3B2;</mi><mo>)</mo></mrow><mrow><mn>2</mn><mi>m</mi><mo>-</mo><mi>n</mi><mo>+</mo><mn>3</mn></mrow></msup></math>"#;
    test_braille("Russian", expr, "⠣⠫⠉⠞⠰⠃⠜⠌⠼⠃⠠⠍⠀⠤⠝⠀⠖⠼⠉⠱")?;

    let expr = r#"<math><mrow><msup><mi>arcsin</mi><mn>2</mn></msup><mi>x</mi></mrow></math>"#;
    test_braille("Russian", expr, "⠫⠁⠎⠌⠆⠠⠭")?;

    let expr = r#"<math><mrow><msup><mi>arccos</mi><mrow><mi>n</mi><mo>+</mo><mn>1</mn></mrow></msup><mi>x</mi></mrow></math>"#;
    test_braille("Russian", expr, "⠫⠁⠉⠌⠠⠝⠀⠖⠼⠁⠱⠠⠭")?;

    let expr = r#"<math><mrow><msup><mi>arctg</mi><mn>3</mn></msup><mfrac><mrow><mi>x</mi><mo>+</mo><mi>y</mi></mrow><mn>2</mn></mfrac></mrow></math>"#;
    test_braille("Russian", expr, "⠫⠁⠞⠌⠒⠐⠆⠠⠭⠀⠖⠽⠀⠳⠼⠃⠰")?;

    let expr = r#"<math><mrow><mi>tan</mi><mi>x</mi><mo>+</mo><mi>cot</mi><mi>x</mi></mrow></math>"#;
    test_braille("Russian", expr, "⠫⠞⠠⠭⠀⠖⠫⠉⠞⠠⠭")?;

    let expr = r#"<math><mrow><msub><mi>log</mi><mi>x</mi></msub><mi>y</mi><mo>&#x22C5;</mo><msub><mi>log</mi><mi>y</mi></msub><mi>x</mi><mo>=</mo><mn>1</mn></mrow></math>"#;
    test_braille("Russian", expr, "⠫⠇⠡⠠⠭⠱⠽⠄⠫⠇⠡⠠⠽⠱⠭⠀⠶⠼⠁")?;

    let expr = r#"<math><mrow><mi>sh</mi><mi>x</mi><mo>+</mo><mi>ch</mi><mi>x</mi><mo>=</mo><mi>sinh</mi><mi>x</mi><mo>+</mo><mi>cosh</mi><mi>x</mi></mrow></math>"#;
    test_braille("Russian", expr, "⠫⠎⠓⠠⠭⠀⠖⠫⠉⠓⠠⠭⠀⠶⠫⠎⠓⠠⠭⠀⠖⠫⠉⠓⠠⠭")?;

    let expr = r#"<math><mrow><mi>th</mi><mi>x</mi><mo>&#x22C5;</mo><mi>cth</mi><mi>x</mi><mo>=</mo><mn>1</mn></mrow></math>"#;
    test_braille("Russian", expr, "⠫⠞⠓⠠⠭⠄⠫⠉⠞⠓⠠⠭⠀⠶⠼⠁")?;

    let expr = r#"<math><mrow><mi>arg</mi><mi>z</mi><mo>=</mo><mn>0</mn><mo>,</mo><mi>sgn</mi><mi>x</mi></mrow></math>"#;
    test_braille("Russian", expr, "⠫⠁⠗⠛⠠⠵⠀⠶⠼⠚⠠⠂⠫⠎⠛⠝⠠⠭")?;

    let expr = r#"<math><mrow><mi>det</mi><mi>A</mi><mo>+</mo><mi>rank</mi><mi>A</mi><mo>=</mo><mi>rg</mi><mi>A</mi></mrow></math>"#;
    test_braille("Russian", expr, "⠫⠙⠑⠞⠨⠁⠀⠖⠫⠗⠁⠝⠅⠨⠁⠀⠶⠫⠗⠛⠨⠁")?;

    let expr = r#"<math><mrow><mi>Re</mi><mi>z</mi><mo>+</mo><mi>Im</mi><mi>z</mi></mrow></math>"#;
    test_braille("Russian", expr, "⠫⠗⠑⠠⠵⠀⠖⠫⠊⠍⠠⠵")?;

    let expr = r#"<math><mrow><mi>grad</mi><mi>&#x3C6;</mi><mo>+</mo><mi>rot</mi><mi>F</mi><mo>+</mo><mi>div</mi><mi>F</mi></mrow></math>"#;
    test_braille("Russian", expr, "⠫⠛⠗⠁⠙⠰⠋⠀⠖⠫⠗⠕⠞⠨⠋⠀⠖⠫⠙⠊⠧⠨⠋")?;

    let expr = r#"<math><mrow><mi>gcd</mi><mrow><mo>(</mo><mi>a</mi><mo>,</mo><mi>b</mi><mo>)</mo></mrow><mo>=</mo><mi>lcm</mi><mrow><mo>(</mo><mi>c</mi><mo>,</mo><mi>d</mi><mo>)</mo></mrow></mrow></math>"#;
    test_braille("Russian", expr, "⠫⠛⠉⠙⠣⠠⠁⠠⠂⠃⠜⠀⠶⠫⠇⠉⠍⠣⠠⠉⠠⠂⠙⠜")?;

    let expr = r#"<math><mrow><msup><mi>y</mi><mo>&#x2032;</mo></msup><mrow><mo>(</mo><mi>x</mi><mo>)</mo></mrow><mo>=</mo><mi>f</mi><mrow><mo>(</mo><mi>x</mi><mo>)</mo></mrow></mrow></math>"#;
    test_braille("Russian", expr, "⠠⠽⠔⠣⠭⠜⠀⠶⠋⠣⠭⠜")?;

    let expr = r#"<math><mrow><msup><mi>y</mi><mo>&#x2032;</mo></msup><mrow><mo>(</mo><mi>x</mi><mo>)</mo></mrow><mo>=</mo><munder><mi>lim</mi><mrow><mi>x</mi><mo>&#x2192;</mo><msub><mi>x</mi><mn>0</mn></msub></mrow></munder><mfrac><mrow><mi>y</mi><mrow><mo>(</mo><mi>x</mi><mo>)</mo></mrow><mo>-</mo><mi>y</mi><mrow><mo>(</mo><msub><mi>x</mi><mn>0</mn></msub><mo>)</mo></mrow></mrow><mrow><mi>x</mi><mo>-</mo><msub><mi>x</mi><mn>0</mn></msub></mrow></mfrac></mrow></math>"#;
    test_braille("Russian", expr, "⠠⠽⠔⠣⠭⠜⠀⠶⠫⠇⠍⠨⠡⠠⠭⠀⠒⠕⠭⠡⠴⠱⠆⠽⠣⠭⠜⠀⠤⠽⠣⠭⠡⠴⠜⠀⠳⠭⠀⠤⠭⠡⠴⠰")?;

    let expr = r#"<math><mrow><msub><mi>x</mi><mi>n</mi></msub><mo>&#x2192;</mo><mi>a</mi><mspace width="1em"/><mtext>при</mtext><mspace width="1em"/><mi>n</mi><mo>&#x2192;</mo><mo>+</mo><mi>&#x221E;</mi></mrow></math>"#;
    test_braille("Russian", expr, "⠠⠭⠡⠝⠱⠀⠒⠕⠁⠀⠏⠗⠊⠀⠝⠀⠒⠕⠀⠖⠻")?;
    return Ok(());
}

#[test]
fn source_large_operator_limits() -> Result<()> {
    let expr = r#"<math><mrow><msubsup><mo>&#x222B;</mo><mn>1</mn><mn>4</mn></msubsup><msup><mi>x</mi><mn>2</mn></msup><mi>d</mi><mi>x</mi></mrow></math>"#;
    test_braille("Russian", expr, "⠮⠡⠂⠌⠲⠠⠭⠌⠆⠙⠭")?;

    let expr = r#"<math><mrow><munderover><mo>&#x2211;</mo><mrow><mi>i</mi><mo>=</mo><mn>1</mn></mrow><mi>n</mi></munderover><msub><mi>a</mi><mi>i</mi></msub></mrow></math>"#;
    test_braille("Russian", expr, "⠸⠎⠨⠡⠠⠊⠀⠶⠼⠁⠱⠨⠌⠠⠝⠱⠁⠡⠊⠱")?;

    let expr = r#"<math><mrow><msubsup><mo>&#x220F;</mo><mn>0</mn><mi>n</mi></msubsup><msub><mi>b</mi><mi>k</mi></msub></mrow></math>"#;
    test_braille("Russian", expr, "⠸⠏⠡⠴⠌⠠⠝⠱⠃⠡⠅⠱")?;
    return Ok(());
}

#[test]
fn source_geometry_matrix_chemistry() -> Result<()> {
    let expr = r#"<math><mrow><mo>&#x2220;</mo><mi>A</mi><mi>B</mi><mi>C</mi><mo>=</mo><mn>15</mn><mo>&#xB0;</mo><mn>30</mn><mo>&#x2032;</mo><mn>12</mn><mo>&#x2033;</mo></mrow></math>"#;
    test_braille("Russian", expr, "⠸⠪⠨⠁⠃⠉⠀⠶⠼⠁⠑⠨⠴⠼⠉⠚⠨⠔⠼⠁⠃⠨⠔⠔")?;

    let expr = r#"<math><mrow><mo>&#x2220;</mo><mrow><mi>A</mi><mo>&#x2032;</mo><mi>B</mi><mo>&#x2032;</mo><mi>C</mi><mo>&#x2032;</mo></mrow></mrow></math>"#;
    test_braille("Russian", expr, "⠸⠪⠨⠁⠔⠃⠔⠉⠔")?;

    let expr = r#"<math><mrow><mo>&#x25B3;</mo><msub><mi>A</mi><mn>1</mn></msub><msub><mi>B</mi><mn>1</mn></msub><msub><mi>C</mi><mn>1</mn></msub></mrow></math>"#;
    test_braille("Russian", expr, "⠸⠙⠨⠁⠡⠂⠃⠡⠂⠉⠡⠂")?;

    let expr = r#"<math><mrow><mn>2</mn><mo>&#x2220;</mo><mi>A</mi><mi>B</mi><mi>C</mi></mrow></math>"#;
    test_braille("Russian", expr, "⠼⠃⠄⠸⠪⠨⠁⠃⠉")?;

    let expr = r#"<math><mrow><mn>3</mn><mo>&#x25B3;</mo><mi>A</mi><mi>B</mi><mi>C</mi></mrow></math>"#;
    test_braille("Russian", expr, "⠼⠉⠄⠸⠙⠨⠁⠃⠉")?;

    let expr = r#"<math><mrow><mn>4</mn><mo>&#x222A;</mo><mi>E</mi><mi>F</mi></mrow></math>"#;
    test_braille("Russian", expr, "⠼⠙⠄⠸⠜⠨⠑⠋")?;

    let expr = r#"<math><mrow><mo>&#x25B3;</mo><mi>K</mi><mi>L</mi><mi>M</mi><mo>&#x223C;</mo><mo>&#x25B3;</mo><mi>P</mi><mi>Q</mi><mi>R</mi></mrow></math>"#;
    test_braille("Russian", expr, "⠸⠙⠨⠅⠇⠍⠀⠢⠸⠙⠨⠏⠟⠗")?;

    let expr = r#"<math><mrow><mi>A</mi><mi>B</mi><mo>&#x2225;</mo><mi>C</mi><mi>D</mi></mrow></math>"#;
    test_braille("Russian", expr, "⠨⠁⠃⠸⠸⠨⠉⠙")?;

    let expr = r#"<math><mrow><mi>a</mi><mo>&#x22A5;</mo><mi>b</mi></mrow></math>"#;
    test_braille("Russian", expr, "⠠⠁⠼⠄⠃")?;

    let expr = r#"<math><mrow><mi>A</mi><mi>B</mi><mo>&#x22A5;</mo><mi>C</mi><mi>D</mi></mrow></math>"#;
    test_braille("Russian", expr, "⠨⠁⠃⠼⠄⠉⠙")?;

    let expr = r#"<math><mrow><mover accent="true"><mi>a</mi><mo stretchy="true">&#x2192;</mo></mover><mo>=</mo><mn>2</mn></mrow></math>"#;
    test_braille("Russian", expr, "⠠⠁⠒⠂⠀⠶⠼⠃")?;

    let expr = r#"<math><mrow><mover accent="true"><mrow><mi>A</mi><mi>B</mi></mrow><mo stretchy="true">&#x2192;</mo></mover><mo>+</mo><mover accent="true"><mrow><mi>B</mi><mi>C</mi></mrow><mo stretchy="true">&#x2192;</mo></mover><mo>=</mo><mover accent="true"><mrow><mi>A</mi><mi>C</mi></mrow><mo stretchy="true">&#x2192;</mo></mover></mrow></math>"#;
    test_braille("Russian", expr, "⠨⠁⠃⠨⠒⠂⠀⠖⠃⠉⠨⠒⠂⠀⠶⠁⠉⠨⠒⠂")?;

    let expr = r#"<math><mrow><mover accent="true"><mrow><mi>K</mi><mi>L</mi></mrow><mo stretchy="true">&#xAF;</mo></mover><mo>=</mo><mn>4</mn><mo>&#x22C5;</mo><mover accent="true"><mrow><mi>P</mi><mi>Q</mi></mrow><mo stretchy="true">&#xAF;</mo></mover></mrow></math>"#;
    test_braille("Russian", expr, "⠨⠅⠇⠨⠒⠀⠶⠼⠙⠄⠨⠏⠟⠨⠒")?;

    let expr = r#"<math><mrow><mover accent="true"><mrow><mi>A</mi><mi>B</mi></mrow><mo stretchy="true">&#x2192;</mo></mover><mo>&#x2225;</mo><mover accent="true"><mrow><mi>C</mi><mi>D</mi></mrow><mo stretchy="true">&#x2192;</mo></mover></mrow></math>"#;
    test_braille("Russian", expr, "⠨⠁⠃⠨⠒⠂⠸⠸⠨⠉⠙⠨⠒⠂")?;

    let expr = r#"<math><mrow><mover accent="true"><mrow><mi>E</mi><mi>F</mi></mrow><mo stretchy="true">&#x23DC;</mo></mover><mo>=</mo><mover accent="true"><mrow><mi>K</mi><mi>L</mi></mrow><mo stretchy="true">&#x23DC;</mo></mover></mrow></math>"#;
    test_braille("Russian", expr, "⠸⠜⠨⠑⠋⠀⠶⠸⠜⠨⠅⠇")?;

    let expr = r#"<math><mrow><mo>(</mo><mtable><mtr><mtd><mi>a</mi></mtd><mtd><mi>b</mi></mtd></mtr><mtr><mtd><mi>c</mi></mtd><mtd><mi>d</mi></mtd></mtr></mtable><mo>)</mo></mrow></math>"#;
    test_braille("Russian", expr, "⠣⠠⠁⠀⠃⠨⠳⠉⠀⠙⠜")?;

    let expr = r#"<math><mrow><mo>{</mo><mtable><mtr><mtd><mrow><mi>x</mi><mo>+</mo><mi>y</mi><mo>+</mo><mi>z</mi><mo>=</mo><mn>1</mn></mrow></mtd></mtr><mtr><mtd><mrow><mi>x</mi><mo>+</mo><mn>2</mn><mi>y</mi><mo>+</mo><mn>3</mn><mi>z</mi><mo>=</mo><mn>2</mn></mrow></mtd></mtr><mtr><mtd><mrow><mi>x</mi><mo>+</mo><mn>3</mn><mi>y</mi><mo>+</mo><mn>2</mn><mi>z</mi><mo>=</mo><mn>5</mn></mrow></mtd></mtr></mtable></mrow></math>"#;
    test_braille("Russian", expr, "⠏⠀⠠⠭⠀⠖⠽⠀⠖⠵⠀⠶⠼⠁⠨⠳⠇⠀⠠⠭⠀⠖⠼⠃⠠⠽⠀⠖⠼⠉⠠⠵⠀⠶⠼⠃⠨⠳⠧⠀⠠⠭⠀⠖⠼⠉⠠⠽⠀⠖⠼⠃⠠⠵⠀⠶⠼⠑")?;

    let expr = r#"<math><mtable><mtr><mtd><mrow><mi>x</mi><mo>+</mo><mi>y</mi><mo>=</mo><mn>7</mn></mrow></mtd></mtr><mtr><mtd><mrow><mn>2</mn><mi>x</mi><mo>+</mo><mn>3</mn><mi>y</mi><mo>=</mo><mn>17</mn></mrow></mtd></mtr></mtable></math>"#;
    test_braille("Russian", expr, "⠠⠭⠀⠖⠽⠀⠶⠼⠛⠨⠳⠼⠃⠠⠭⠀⠖⠼⠉⠠⠽⠀⠶⠼⠁⠛")?;

    let expr = r#"<math><mrow><mi>f</mi><mrow><mo>(</mo><mi>x</mi><mo>)</mo></mrow><mo>=</mo><mrow><mo>{</mo><mtable><mtr><mtd><mn>0</mn></mtd><mtd><mrow><mi>x</mi><mo>&lt;</mo><mn>0</mn></mrow></mtd></mtr><mtr><mtd><mi>x</mi></mtd><mtd><mrow><mi>x</mi><mo>&#x2265;</mo><mn>0</mn></mrow></mtd></mtr></mtable></mrow></mrow></math>"#;
    test_braille("Russian", expr, "⠠⠋⠣⠭⠜⠀⠶⠏⠀⠼⠚⠀⠠⠭⠀⠪⠀⠼⠚⠨⠳⠧⠀⠠⠭⠀⠭⠀⠕⠶⠼⠚")?;

    let expr = r#"<math><mrow><mo>|</mo><mtable><mtr><mtd><mi>a</mi></mtd><mtd><mi>b</mi></mtd></mtr><mtr><mtd><mi>c</mi></mtd><mtd><mi>d</mi></mtd></mtr></mtable><mo>|</mo></mrow></math>"#;
    test_braille("Russian", expr, "⠸⠠⠁⠀⠃⠨⠳⠉⠀⠙⠸")?;

    let expr = r#"<math><msub><mi>H</mi><mn>2</mn></msub><mi>O</mi></math>"#;
    test_braille("Russian", expr, "⠨⠓⠡⠆⠕")?;

    let expr = r#"<math><mrow><msub><mi>H</mi><mn>2</mn></msub><mi>C</mi><mo>=</mo><mi>C</mi><msub><mi>H</mi><mn>2</mn></msub></mrow></math>"#;
    test_braille("Russian", expr, "⠨⠓⠡⠆⠉⠦⠉⠓⠡⠆")?;
    return Ok(());
}

#[test]
fn source_label_marks() -> Result<()> {
    let expr = r#"<math><mover accent="true"><mi>x</mi><mo>~</mo></mover></math>"#;
    test_braille("Russian", expr, "⠠⠭⠢")?;

    let expr = r#"<math><mover accent="true"><mn>7</mn><mo>~</mo></mover></math>"#;
    test_braille("Russian", expr, "⠼⠛⠢")?;

    let expr = r#"<math><mover accent="true"><mi>x</mi><mo>^</mo></mover></math>"#;
    test_braille("Russian", expr, "⠠⠭⠲")?;

    let expr = r#"<math><mover accent="true"><mi>x</mi><mo>&#x02C7;</mo></mover></math>"#;
    test_braille("Russian", expr, "⠠⠭⠰⠔")?;

    let expr = r#"<math><mover accent="true"><mi>x</mi><mo>&#x2217;</mo></mover></math>"#;
    test_braille("Russian", expr, "⠠⠭⠘⠆")?;

    let expr = r#"<math><msup><mi>x</mi><mo>&#x2032;</mo></msup></math>"#;
    test_braille("Russian", expr, "⠠⠭⠔")?;

    let expr = r#"<math><munder accentunder="true"><mi>x</mi><mo>~</mo></munder></math>"#;
    test_braille("Russian", expr, "⠠⠭⠰⠢")?;

    let expr = r#"<math><msup><mi>x</mi><mo>&#x2217;</mo></msup></math>"#;
    test_braille("Russian", expr, "⠠⠭⠆")?;

    let expr = r#"<math><msup><mi>x</mi><mo>&#x2218;</mo></msup></math>"#;
    test_braille("Russian", expr, "⠠⠭⠨⠴")?;

    let expr = r#"<math><msub><mi>x</mi><mo>&#x25A1;</mo></msub></math>"#;
    test_braille("Russian", expr, "⠠⠭⠸⠶")?;

    let expr = r#"<math><msub><mi>x</mi><mo>&#x00D7;</mo></msub></math>"#;
    test_braille("Russian", expr, "⠠⠭⠸⠦")?;

    let expr = r#"<math><msup><mi>x</mi><mrow><mo>&#x2217;</mo><mo>&#x2217;</mo></mrow></msup></math>"#;
    test_braille("Russian", expr, "⠠⠭⠆⠆")?;

    let expr = r#"<math><mover accent="true"><mrow><mi>x</mi><mo>+</mo><mi>y</mi></mrow><mo>~</mo></mover></math>"#;
    test_braille("Russian", expr, "⠯⠠⠭⠀⠖⠽⠽⠢")?;

    let expr = r#"<math><mmultiscripts><mi>x</mi><mprescripts/><mo>&#x25A1;</mo><none/></mmultiscripts></math>"#;
    test_braille("Russian", expr, "⠸⠶⠠⠭")?;

    let expr = r#"<math><mmultiscripts><mi>x</mi><mprescripts/><none/><mo>&#x2217;</mo></mmultiscripts></math>"#;
    test_braille("Russian", expr, "⠨⠆⠠⠭")?;

    let expr = r#"<math><mrow><mn>2</mn><mo>&#x2370;</mo><mmultiscripts><mi>x</mi><mprescripts/><mo>&#x2217;</mo><none/></mmultiscripts></mrow></math>"#;
    test_braille("Russian", expr, "⠼⠃⠐⠸⠆⠠⠭")?;

    let expr = r#"<math><mrow><mi>a</mi><mo>&#x2370;</mo><mmultiscripts><mi>x</mi><mprescripts/><none/><mo>&#x25A1;</mo></mmultiscripts></mrow></math>"#;
    test_braille("Russian", expr, "⠠⠁⠐⠨⠶⠭")?;

    let expr = r#"<math><msubsup><mover accent="true"><mi>x</mi><mo>~</mo></mover><mn>1</mn><mn>2</mn></msubsup></math>"#;
    test_braille("Russian", expr, "⠠⠭⠢⠡⠼⠁⠌⠼⠃⠱")?;
    return Ok(());
}

#[test]
fn source_chemical_reactions_and_charges() -> Result<()> {
    let expr = r#"<math intent=":chemical-equation"><mrow><mn>2</mn><mi>HCl</mi><mo>+</mo><mn>2</mn><mi>Na</mi><mo>&#x2192;</mo><mn>2</mn><mi>NaCl</mi><mo>+</mo><msub><mi>H</mi><mn>2</mn></msub></mrow></math>"#;
    test_braille("Russian", expr, "⠼⠃⠨⠓⠉⠠⠇⠀⠖⠼⠃⠨⠝⠠⠁⠀⠒⠕⠼⠃⠨⠝⠠⠁⠨⠉⠠⠇⠀⠖⠨⠓⠡⠆")?;

    let expr = r#"<math intent=":chemical-equation"><mrow><msub><mi>H</mi><mn>2</mn></msub><mo>(</mo><mi>g</mi><mo>)</mo><mo>+</mo><msub><mi>I</mi><mn>2</mn></msub><mo>(</mo><mi>g</mi><mo>)</mo><mo>&#x21CC;</mo><mn>2</mn><mi>HI</mi><mo>(</mo><mi>g</mi><mo>)</mo></mrow></math>"#;
    test_braille("Russian", expr, "⠨⠓⠡⠆⠣⠠⠛⠜⠀⠖⠨⠊⠡⠆⠣⠠⠛⠜⠀⠒⠕⠀⠦⠶⠼⠃⠨⠓⠊⠣⠠⠛⠜")?;

    let expr = r#"<math intent=":chemical-equation"><mrow><mn>2</mn><mi>Al</mi><mo>&#x2192;</mo><mn>2</mn><msup><mi>Al</mi><mrow><mn>3</mn><mo>+</mo></mrow></msup><mo>+</mo><mn>6</mn><msup><mi>e</mi><mo>-</mo></msup></mrow></math>"#;
    test_braille("Russian", expr, "⠼⠃⠨⠁⠠⠇⠀⠒⠕⠼⠃⠨⠁⠠⠇⠌⠒⠖⠀⠖⠼⠋⠠⠑⠌⠤")?;

    let expr = r#"<math><msubsup><mi>SO</mi><mn>4</mn><mrow><mn>2</mn><mo>-</mo></mrow></msubsup></math>"#;
    test_braille("Russian", expr, "⠨⠎⠕⠡⠲⠌⠆⠤")?;

    let expr = r#"<math><msup><msub><mi>HPO</mi><mn>4</mn></msub><mrow><mo>-</mo><mo>-</mo></mrow></msup></math>"#;
    test_braille("Russian", expr, "⠨⠓⠏⠕⠡⠲⠌⠆⠤")?;

    let expr = r#"<math intent=":chemical-equation"><mrow><mi>CNO</mi></mrow></math>"#;
    test_braille("Russian", expr, "⠸⠉⠝⠕")?;

    let expr = r#"<math intent=":chemical-equation"><mrow><mi>N</mi><mover><mo>&#x2192;</mo><mn>300</mn></mover><mi>N</mi></mrow></math>"#;
    test_braille("Russian", expr, "⠨⠝⠀⠒⠕⠨⠌⠼⠉⠚⠚⠱⠨⠝")?;

    let expr = r#"<math intent=":chemical-equation"><mrow><mi>N</mi><munder><mo>&#x2192;</mo><mn>300</mn></munder><mi>N</mi></mrow></math>"#;
    test_braille("Russian", expr, "⠨⠝⠀⠒⠕⠨⠡⠼⠉⠚⠚⠱⠨⠝")?;

    let expr = r#"<math intent=":chemical-equation"><mrow><mi>N</mi><munderover><mo>&#x2192;</mo><mn>300</mn><mi>Pt</mi></munderover><mi>N</mi></mrow></math>"#;
    test_braille("Russian", expr, "⠨⠝⠀⠒⠕⠨⠡⠼⠉⠚⠚⠱⠨⠌⠨⠏⠠⠞⠱⠨⠝")?;

    let expr = r#"<math intent=":chemical-equation"><mrow><mi>NaCl</mi><mo>(</mo><mi>aq</mi><mo>)</mo><mo>+</mo><mi>H</mi><mo>(</mo><mi>l</mi><mo>)</mo><mo>+</mo><mi>C</mi><mo>(</mo><mi>s</mi><mo>)</mo></mrow></math>"#;
    test_braille("Russian", expr, "⠨⠝⠠⠁⠨⠉⠠⠇⠣⠁⠟⠜⠀⠖⠨⠓⠣⠠⠇⠜⠀⠖⠨⠉⠣⠠⠎⠜")?;

    let expr = r#"<math intent=":chemical-equation"><mrow><msup><mi>e</mi><mo>+</mo></msup><mo>+</mo><msup><mi>e</mi><mrow><mo>-</mo><mo>-</mo></mrow></msup></mrow></math>"#;
    test_braille("Russian", expr, "⠠⠑⠌⠖⠀⠖⠑⠌⠆⠤")?;
    return Ok(());
}

#[test]
fn source_gost_logic_arrows() -> Result<()> {
    let expr = r#"<math><mrow><mi>A</mi><mo>&#x21D2;</mo><mi>B</mi></mrow></math>"#;
    test_braille("Russian", expr, "⠨⠁⠀⠶⠜⠃")?;

    let expr = r#"<math><mrow><mi>B</mi><mo>&#x21D0;</mo><mi>A</mi></mrow></math>"#;
    test_braille("Russian", expr, "⠨⠃⠀⠦⠶⠁")?;

    let expr = r#"<math><mrow><mi>C</mi><mo>&#x21D4;</mo><mi>D</mi></mrow></math>"#;
    test_braille("Russian", expr, "⠨⠉⠀⠦⠶⠜⠙")?;

    let expr = r#"<math><mrow><mi>&#x2200;</mi><mi>x</mi><mo>:</mo><mi>P</mi><mo>(</mo><mi>x</mi><mo>)</mo></mrow></math>"#;
    test_braille("Russian", expr, "⠫⠄⠠⠭⠀⠲⠨⠏⠣⠠⠭⠜")?;

    let expr = r#"<math><mrow><mi>&#x2203;</mi><mi>x</mi><mo>:</mo><mi>P</mi><mo>(</mo><mi>x</mi><mo>)</mo></mrow></math>"#;
    test_braille("Russian", expr, "⠫⠢⠠⠭⠀⠲⠨⠏⠣⠠⠭⠜")?;
    return Ok(());
}
