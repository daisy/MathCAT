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
        ("sup_sqrt", r#"<math><msup><mi>a</mi><msqrt><mi>x</mi></msqrt></msup></math>"#),
        ("sup_follow_letter", r#"<math><mrow><msup><mi>x</mi><mn>2</mn></msup><mi>y</mi></mrow></math>"#),
        ("sup_follow_number", r#"<math><mrow><msup><mi>x</mi><mn>2</mn></msup><mn>3</mn></mrow></math>"#),
        ("sub_x", r#"<math><msub><mi>a</mi><mi>x</mi></msub></math>"#),
        ("sub_zero", r#"<math><msub><mi>x</mi><mn>0</mn></msub></math>"#),
        ("sub_ten", r#"<math><msub><mi>x</mi><mn>10</mn></msub></math>"#),
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
        ("sup_sqrt", "⠠⠁⠌⠩⠱⠭⠹⠱"),
        ("sup_follow_letter", "⠠⠭⠌⠆⠽"),
        ("sup_follow_number", "⠠⠭⠌⠆⠼⠉"),
        ("sub_x", "⠠⠁⠡⠭⠱"),
        ("sub_zero", "⠠⠭⠡⠴"),
        ("sub_ten", "⠠⠭⠡⠂⠴"),
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
fn latin_alphabet_indicators() -> Result<()> {
    let expr = r#"<math><mrow><mi>x</mi><mo>+</mo><mi>A</mi><mo>+</mo><mi>y</mi><mo>+</mo><mi>B</mi><mo>=</mo><mi>x</mi><mo>+</mo><mi>y</mi><mo>+</mo><mi>A</mi><mo>+</mo><mi>B</mi></mrow></math>"#;
    test_braille("Russian", expr, "⠠⠭⠀⠖⠨⠁⠀⠖⠠⠽⠀⠖⠨⠃⠀⠶⠠⠭⠀⠖⠽⠀⠖⠨⠁⠀⠖⠃")?;
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

    let expr = r#"<math><mrow><mi mathvariant="bold">x</mi></mrow></math>"#;
    test_braille("Russian", expr, "⠻⠠⠭⠻")?;

    let expr = r#"<math><mrow><mi mathvariant="bold">x</mi><mi mathvariant="bold">y</mi></mrow></math>"#;
    test_braille("Russian", expr, "⠻⠠⠭⠽⠻")?;

    let expr = r#"<math><mrow><mi mathvariant="italic">y</mi></mrow></math>"#;
    test_braille("Russian", expr, "⠸⠠⠽⠸")?;
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
    return Ok(());
}

#[test]
fn source_functions_logs_derivatives() -> Result<()> {
    let expr = r#"<math><mrow><mi>cos</mi><mi>&#x3B1;</mi></mrow></math>"#;
    test_braille("Russian", expr, "⠫⠉⠰⠁")?;

    let expr = r#"<math><mrow><mi>tg</mi><mi>x</mi><mo>&#x22C5;</mo><mi>ctg</mi><mi>x</mi><mo>=</mo><mn>1</mn></mrow></math>"#;
    test_braille("Russian", expr, "⠫⠞⠠⠭⠄⠫⠉⠞⠠⠭⠀⠶⠼⠁")?;

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
    test_braille("Russian", expr, "⠸⠪⠨⠁⠃⠉⠀⠶⠼⠁⠑⠨⠴⠼⠉⠚⠔⠼⠁⠃⠔⠔")?;

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

    let expr = r#"<math><mrow><mover accent="true"><mrow><mi>E</mi><mi>F</mi></mrow><mo stretchy="true">&#x23DC;</mo></mover><mo>=</mo><mover accent="true"><mrow><mi>K</mi><mi>L</mi></mrow><mo stretchy="true">&#x23DC;</mo></mover></mrow></math>"#;
    test_braille("Russian", expr, "⠸⠜⠨⠑⠋⠀⠶⠸⠜⠨⠅⠇")?;

    let expr = r#"<math><mrow><mo>(</mo><mtable><mtr><mtd><mi>a</mi></mtd><mtd><mi>b</mi></mtd></mtr><mtr><mtd><mi>c</mi></mtd><mtd><mi>d</mi></mtd></mtr></mtable><mo>)</mo></mrow></math>"#;
    test_braille("Russian", expr, "⠣⠠⠁⠀⠃⠨⠳⠉⠀⠙⠜")?;

    let expr = r#"<math><mrow><mo>{</mo><mtable><mtr><mtd><mrow><mi>x</mi><mo>+</mo><mi>y</mi><mo>+</mo><mi>z</mi><mo>=</mo><mn>1</mn></mrow></mtd></mtr><mtr><mtd><mrow><mi>x</mi><mo>+</mo><mn>2</mn><mi>y</mi><mo>+</mo><mn>3</mn><mi>z</mi><mo>=</mo><mn>2</mn></mrow></mtd></mtr><mtr><mtd><mrow><mi>x</mi><mo>+</mo><mn>3</mn><mi>y</mi><mo>+</mo><mn>2</mn><mi>z</mi><mo>=</mo><mn>5</mn></mrow></mtd></mtr></mtable></mrow></math>"#;
    test_braille("Russian", expr, "⠏⠀⠠⠭⠀⠖⠽⠀⠖⠵⠀⠶⠼⠁⠨⠳⠇⠀⠠⠭⠀⠖⠼⠃⠠⠽⠀⠖⠼⠉⠠⠵⠀⠶⠼⠃⠨⠳⠧⠀⠠⠭⠀⠖⠼⠉⠠⠽⠀⠖⠼⠃⠠⠵⠀⠶⠼⠑")?;

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
    test_braille("Russian", expr, "⠩⠱⠼⠁⠚⠚⠚⠚⠀⠪⠀⠼⠁⠚⠁")?;
    return Ok(());
}
