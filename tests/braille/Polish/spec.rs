
// Polish tests
// Most of these tests come from 
//   https://ore.edu.pl/images/files/pdf/Brajlowska%20notacja%20matematyczna%20fizyczna%20chemiczna%20wyd%20II.pdf
use crate::common::*;
use anyhow::Result;


#[test]
fn Intro_1() -> Result<()> {
    let expr = r#"<math><msqrt><mn>16</mn></msqrt></math>"#;
    test_braille("Polish", expr, "⠩⠼⠁⠋")?;
    return Ok(());
}

#[test]
fn Intro_2() -> Result<()> {
    let expr = r#"<math><msqrt><mn>81</mn></msqrt><mo>=</mo><mn>9</mn></math>"#;
    test_braille("Polish", expr, "⠩⠼⠓⠁⠀⠶⠼⠊")?;
    return Ok(());
}

#[test]
fn Intro_3() -> Result<()> {
    let expr = r#"<math><mroot><mn>27</mn><mn>3</mn></mroot><mo>=</mo><mn>3</mn></math>"#;
    test_braille("Polish", expr, "⠌⠒⠩⠼⠃⠛⠀⠶⠼⠉")?;
    return Ok(());
}

#[test]
fn decimal_numbers_p5_1() -> Result<()> {
    let expr = r#"<math><mn>7</mn><mo>,</mo><mn>29</mn></math>"#;
    test_braille("Polish", expr, "⠼⠛⠂⠃⠊")?;
    return Ok(());
}

#[test]
fn decimal_numbers_p5_2() -> Result<()> {
    let expr = r#"<math><mn>0</mn><mo>,</mo><mn>072</mn></math>"#;
    test_braille("Polish", expr, "⠼⠚⠂⠚⠛⠃")?;
    return Ok(());
}

#[test]
fn decimal_numbers_p5_3() -> Result<()> {
    let expr = r#"<math><mn>50</mn><mo>,</mo><mn>347</mn><mo>.</mo><mn>296</mn></math>"#;
    test_braille("Polish", expr, "⠼⠑⠚⠂⠉⠙⠛⠄⠃⠊⠋")?;
    return Ok(());
}

#[test]
fn decimal_numbers_p5_4() -> Result<()> {
    // FIX: wasn't able to repair this MathML
    // let expr = r#"<math><mn>0</mn><mo>,</mo><mn>333</mn><mo>.</mo><mo>.</mo><mo>.</mo><mo>=</mo><mn>0</mn><mo>,</mo><mo>(</mo><mn>3</mn><mo>)</mo></math>"#;
    let expr = r#"<math><mn>0,333</mn><mo>.</mo><mo>.</mo><mo>.</mo><mo>=</mo><mn>0</mn><mo>,</mo><mo>(</mo><mn>3</mn><mo>)</mo></math>"#;
    test_braille("Polish", expr, "⠼⠚⠂⠉⠉⠉⠄⠄⠄⠀⠶⠼⠚⠂⠣⠼⠉⠜")?;
    return Ok(());
}

#[test]
fn percents_and_promiles_1() -> Result<()> {
    let expr = r#"<math><mn>0</mn><mo>,</mo><mn>25</mn><mo>=</mo><mn>25</mn><mo>%</mo></math>"#;
    test_braille("Polish", expr, "⠼⠚⠂⠃⠑⠀⠶⠼⠃⠑⠼⠚⠴")?;
    return Ok(());
}

#[test]
fn percents_and_promiles_2() -> Result<()> {
    let expr = r#"<math><mn>48</mn><mo>%</mo><mo>=</mo><mn>480</mn><mo>&#x2030;</mo></math>"#;
    test_braille("Polish", expr, "⠼⠙⠓⠼⠚⠴⠀⠶⠼⠙⠓⠚⠼⠚⠴⠴")?;
    return Ok(());
}

#[test]
fn units_of_measurement_p8_1() -> Result<()> {
    let expr = r#"<math><mn>1</mn><mi intent=":unit">m</mi></math>"#;
    test_braille("Polish", expr, "⠼⠁⠻⠍")?;
    return Ok(());
}

#[test]
fn units_of_measurement_p8_2() -> Result<()> {
    let expr = r#"<math><mn>1</mn><mi intent=":unit">km</mi></math>"#;
    test_braille("Polish", expr, "⠼⠁⠻⠅⠍")?;
    return Ok(());
}

#[test]
fn units_of_measurement_p8_3() -> Result<()> {
    let expr = r#"<math><mn>5</mn><mfrac><mi intent=":unit">m</mi><mi intent=":unit">s</mi></mfrac></math>"#;
    test_braille("Polish", expr, "⠼⠑⠻⠍⠳⠎")?;
    return Ok(());
}

#[test]
fn units_of_measurement_p8_4() -> Result<()> {
    let expr = r#"<math><mn>230</mn><mi intent=":unit">V</mi></math>"#;
    test_braille("Polish", expr, "⠼⠃⠉⠚⠻⠨⠧")?;
    return Ok(());
}

#[test]
fn units_of_measurement_p9_5() -> Result<()> {
    let expr = r#"<math><mn>2</mn><mi intent=":unit">min</mi></math>"#;
    test_braille("Polish", expr, "⠼⠃⠻⠍⠊⠝")?;
    return Ok(());
}

#[test]
fn units_of_measurement_p9_6() -> Result<()> {
    let expr = r#"<math><mn>5</mn><mi intent=":unit">N</mi></math>"#;
    test_braille("Polish", expr, "⠼⠑⠻⠨⠝")?;
    return Ok(());
}

#[test]
fn units_of_measurement_p9_7() -> Result<()> {
    let expr = r#"<math><mn>2</mn><mo>,</mo><mn>5</mn><msup><mi intent=":unit">m</mi><mn>2</mn></msup></math>"#;
    test_braille("Polish", expr, "⠼⠃⠂⠑⠻⠍⠬⠆")?;
    return Ok(());
}

#[test]
fn currency_p9_1() -> Result<()> {
    let expr = r#"<math><mn>2</mn><mo>&#xA0;</mo><mi>z&#x142;</mi></math>"#;
    test_braille("Polish", expr, "⠼⠃⠀⠵⠣")?;
    return Ok(());
}

#[test]
fn currency_p9_2() -> Result<()> {
    let expr = r#"<math><mn>2</mn><mo>&#xA0;</mo><mi>PLN</mi></math>"#;
    test_braille("Polish", expr, "⠼⠃⠀⠨⠏⠨⠇⠨⠝")?;
    return Ok(());
}

#[test]
fn currency_p9_3() -> Result<()> {
    let expr = r#"<math><mn>5</mn><mo>&#xA0;</mo><mi>z&#x142;</mi><mo>&#xA0;</mo><mn>50</mn><mo>&#xA0;</mo><mi>gr</mi></math>"#;
    test_braille("Polish", expr, "⠼⠑⠀⠵⠣⠀⠼⠑⠚⠀⠛⠗")?;
    return Ok(());
}

#[test]
fn currency_p9_4() -> Result<()> {
    let expr = r#"<math><mn>5</mn><mo>,</mo><mn>50</mn><mo>&#xA0;</mo><mi>z&#x142;</mi></math>"#;
    test_braille("Polish", expr, "⠼⠑⠂⠑⠚⠀⠵⠣")?;
    return Ok(());
}

#[test]
fn currency_p9_5() -> Result<()> {
    let expr = r#"<math><mn>2</mn><mo>&#xA0;</mo><mo>&#x20AC;</mo></math>"#;
    test_braille("Polish", expr, "⠼⠃⠀⠈⠑")?;
    return Ok(());
}

#[test]
fn date_p9_1() -> Result<()> {
    let expr = r#"<math><mn>15</mn><mo>.</mo><mn>03</mn><mo>.</mo><mn>2002</mn></math>"#;
    test_braille("Polish", expr, "⠼⠁⠑⠄⠚⠉⠄⠃⠚⠚⠃")?;
    return Ok(());
}

#[test]
fn date_p9_2() -> Result<()> {
    let expr = r#"<math><mn>98</mn><mo>/</mo><mn>08</mn><mo>/</mo><mn>26</mn></math>"#;
    test_braille("Polish", expr, "⠼⠊⠓⠲⠚⠓⠲⠃⠋")?;
    return Ok(());
}

#[test]
fn date_p9_3() -> Result<()> {
    let expr = r#"<math><mn>2002</mn><mo>-</mo><mn>03</mn><mo>-</mo><mn>15</mn></math>"#;
    test_braille("Polish", expr, "⠼⠃⠚⠚⠃⠤⠚⠉⠤⠁⠑")?;
    return Ok(());
}

#[test]
fn date_p9_4() -> Result<()> {
    let expr = r#"<math intent=":date"><mn>15</mn><mo>&#xA0;</mo><mi>II</mi><mo>&#xA0;</mo><mn>2011</mn></math>"#;
    test_braille("Polish", expr, "⠼⠁⠑⠀⠨⠊⠊⠀⠼⠃⠚⠁⠁")?;
    return Ok(());
}

#[test]
fn time_p9_1() -> Result<()> {
    let expr = r#"<math><msup intent=":time"><mn>0</mn><mn>20</mn></msup></math>"#;
    test_braille("Polish", expr, "⠼⠚⠄⠃⠚")?;
    return Ok(());
}

#[test]
fn time_p9_2() -> Result<()> {
    let expr = r#"<math intent=":time"><mn>05</mn><mo>:</mo><mn>40</mn></math>"#;
    test_braille("Polish", expr, "⠼⠚⠑⠒⠙⠚")?;
    return Ok(());
}

#[test]
fn time_p9_3() -> Result<()> {
    let expr = r#"<math intent=":time"><mn>18</mn><mo>.</mo><mn>25</mn></math>"#;
    test_braille("Polish", expr, "⠼⠁⠓⠄⠃⠑")?;
    return Ok(());
}

#[test]
fn signs_of_action_p2_1() -> Result<()> {
    let expr = r#"<math><mn>5</mn><mo>+</mo><mi>x</mi></math>"#;
    test_braille("Polish", expr, "⠼⠑⠀⠖⠠⠭")?;
    return Ok(());
}

#[test]
fn signs_of_action_p2_2() -> Result<()> {
    let expr = r#"<math><mn>67</mn><mo>:</mo><mn>14</mn></math>"#;
    test_braille("Polish", expr, "⠼⠋⠛⠀⠲⠼⠁⠙")?;
    return Ok(());
}

#[test]
fn signs_of_action_p2_3() -> Result<()> {
    let expr = r#"<math><mn>24</mn><mo>,</mo><mn>6</mn><mo>+</mo><mn>2</mn><mo>-</mo><mn>4</mn><mo>,</mo><mn>8</mn></math>"#;
    test_braille("Polish", expr, "⠼⠃⠙⠂⠋⠀⠖⠼⠃⠀⠤⠼⠙⠂⠓")?;
    return Ok(());
}

#[test]
fn signs_of_action_p2_4a() -> Result<()> {
    let expr = r#"<math><mn>12</mn><mo>&#xB7;</mo><mn>3</mn></math>"#;
    test_braille("Polish", expr, "⠼⠁⠃⠄⠼⠉")?;
    return Ok(());
}

#[test]
#[ignore = "uses space before multiplication dot when not needed"]
fn signs_of_action_p2_4b() -> Result<()> {
    let expr = r#"<math><mn>12</mn><mo>&#xB7;</mo><mn>3</mn></math>"#;
    test_braille_prefs("Polish", vec![("Polish_BrailleLevel", "Advanced")], expr, r"⠼⠁⠃⠀⠄⠼⠉")?;
    return Ok(());
}

#[test]
fn relations_p11_1() -> Result<()> {
    let expr = r#"<math><mn>7</mn><mo>-</mo><mn>4</mn><mo>=</mo><mn>3</mn></math>"#;
    test_braille("Polish", expr, "⠼⠛⠀⠤⠼⠙⠀⠶⠼⠉")?;
    return Ok(());
}

#[test]
fn relations_p11_2() -> Result<()> {
    let expr = r#"<math><mn>2</mn><mo>&#xB7;</mo><mn>5</mn><mo>&lt;</mo><mn>47</mn><mo>:</mo><mn>3</mn></math>"#;
    test_braille("Polish", expr, "⠼⠃⠄⠼⠑⠀⠪⠄⠼⠙⠛⠀⠲⠼⠉")?;
    return Ok(());
}

#[test]
fn brackets_p11_1() -> Result<()> {
    let expr = r#"<math><mo>(</mo><mn>14</mn><mo>-</mo><mn>5</mn><mo>)</mo><mo>+</mo><mn>7</mn><mo>&gt;</mo><mo>-</mo><mo>[</mo><mn>1</mn><mo>+</mo><mn>2</mn><mo>(</mo><mn>5</mn><mo>+</mo><mn>3</mn><mo>)</mo><mo>]</mo></math>"#;
    test_braille("Polish", expr, "⠣⠼⠁⠙⠀⠤⠼⠑⠜⠀⠖⠼⠛⠀⠕⠂⠤⠷⠼⠁⠀⠖⠼⠃⠣⠼⠑⠀⠖⠼⠉⠜⠾")?;
    return Ok(());
}

#[test]
fn brackets_p11_2() -> Result<()> {
    let expr = r#"<math><mn>2</mn><mo>+</mo><mo>{</mo><mn>4</mn><mo>-</mo><mo>[</mo><mn>5</mn><mo>+</mo><mo>(</mo><mn>6</mn><mo>-</mo><mn>2</mn><mo>)</mo><mo>]</mo><mo>+</mo><mn>3</mn><mo>(</mo><mn>6</mn><mo>+</mo><mn>4</mn><mo>)</mo><mo>}</mo><mo>=</mo><mn>2</mn><mo>+</mo><mo>{</mo><mn>4</mn><mo>-</mo><mo>[</mo><mn>5</mn><mo>+</mo><mn>4</mn><mo>]</mo><mo>+</mo><mn>30</mn><mo>}</mo></math>"#;
    test_braille("Polish", expr, "⠼⠃⠀⠖⠪⠼⠙⠀⠤⠷⠼⠑⠀⠖⠣⠼⠋⠀⠤⠼⠃⠜⠾⠀⠖⠼⠉⠣⠼⠋⠀⠖⠼⠙⠜⠕⠀⠶⠼⠃⠀⠖⠪⠼⠙⠀⠤⠷⠼⠑⠀⠖⠼⠙⠾⠀⠖⠼⠉⠚⠕")?;
    return Ok(());
}

#[test]
fn brackets_p12_3() -> Result<()> {
    let expr = r#"<math><mn>5</mn><mo>-</mo><mn>3</mn><mo>=</mo><mn>2</mn><mo>&#xA0;</mo><mo>(</mo><mi>bo</mi><mo>&#xA0;</mo><mn>2</mn><mo>+</mo><mn>3</mn><mo>=</mo><mn>5</mn><mo>)</mo></math>"#;
    test_braille("Polish", expr, "⠼⠑⠀⠤⠼⠉⠀⠶⠼⠃⠀⠠⠶⠃⠕⠀⠼⠃⠀⠖⠼⠉⠀⠶⠼⠑⠠⠶")?;
    return Ok(());
}

#[test]
fn absolute_value_p12_1() -> Result<()> {
    let expr = r#"<math><mfenced open="|" close="|"><mrow><mo>-</mo><mn>5</mn></mrow></mfenced><mo>=</mo><mn>5</mn></math>"#;
    test_braille("Polish", expr, "⠈⠇⠤⠼⠑⠸⠀⠶⠼⠑")?;
    return Ok(());
}

#[test]
fn absolute_value_p12_2() -> Result<()> {
    let expr = r#"<math><mo>-</mo><mfenced open="|" close="|"><mrow><mo>-</mo><mo>(</mo><mn>7</mn><mo>+</mo><mn>4</mn><mo>)</mo></mrow></mfenced><mo>&lt;</mo><mfenced open="|" close="|"><mrow><mo>(</mo><mn>4</mn><mo>-</mo><mn>7</mn><mo>)</mo></mrow></mfenced></math>"#;
    test_braille("Polish", expr, "⠤⠈⠇⠤⠣⠼⠛⠀⠖⠼⠙⠜⠸⠀⠪⠄⠈⠇⠣⠼⠙⠀⠤⠼⠛⠜⠸")?;
    return Ok(());
}

#[test]
fn number_divisors_p12_1() -> Result<()> {
    let expr = r#"<math><mn>5</mn><mo>|</mo><mn>25</mn></math>"#;
    test_braille("Polish", expr, "⠼⠑⠈⠇⠼⠃⠑")?;
    return Ok(());
}

#[test]
fn number_divisors_p12_2() -> Result<()> {
    let expr = r#"<math><mn>5</mn><mo>&#x2224;</mo><mn>27</mn></math>"#;
    test_braille("Polish", expr, "⠼⠑⠀⠔⠈⠇⠼⠃⠛")?;
    return Ok(());
}

#[test]
fn number_separation_p13_1() -> Result<()> {
    let expr = r#"<math><mn>12</mn><mo>;</mo></math>"#;
    test_braille("Polish", expr, "⠼⠁⠃⠠⠆")?;
    return Ok(());
}

#[test]
fn number_separation_p13_2() -> Result<()> {
    let expr = r#"<math><mn>12</mn><mo>?</mo></math>"#;
    test_braille("Polish", expr, "⠼⠁⠃⠠⠢")?;
    return Ok(());
}

#[test]
#[ignore = "there is no way to tell this is not a factorial"]
fn number_separation_p13_3() -> Result<()> {
    let expr = r#"<math><mn>12</mn><mo>!</mo></math>"#;
    test_braille("Polish", expr, "⠼⠁⠃⠠⠖")?;
    return Ok(());
}

#[test]
fn number_separation_p13_4() -> Result<()> {
    let expr = r#"<math><mi>P</mi><mo>=</mo><mo>(</mo><mn>3</mn><mo>,</mo><mn>5</mn><mo>)</mo></math>"#;
    test_braille("Polish", expr, "⠨⠏⠀⠶⠣⠼⠉⠠⠂⠀⠼⠑⠜")?;
    return Ok(());
}

#[test]
fn number_separation_p13_5() -> Result<()> {
    let expr = r#"<math><mi>X</mi><mo>=</mo><mo>{</mo><mn>1</mn><mo>,</mo><mo>&#xA0;</mo><mn>2</mn><mo>,</mo><mo>&#xA0;</mo><mn>3</mn><mo>,</mo><mo>&#xA0;</mo><mn>4</mn><mo>}</mo></math>"#;
    test_braille("Polish", expr, "⠨⠭⠀⠶⠪⠼⠁⠠⠂⠀⠼⠃⠠⠂⠀⠼⠉⠠⠂⠀⠼⠙⠕")?;
    return Ok(());
}

#[test]
fn number_separation_p13_6() -> Result<()> {
    let expr = r#"<math><mo>(</mo><mn>3,2</mn><mo>&#xA0;</mo><mo>;</mo><mo>&#xA0;</mo><mn>5</mn><mo>,</mo><mn>4</mn><mo>⟩</mo></math>"#;
    test_braille("Polish", expr, "⠣⠼⠉⠂⠃⠠⠆⠀⠼⠑⠂⠙⠠⠾")?;
    return Ok(());
}

#[test]
fn number_separation_p13_7() -> Result<()> {
    let expr = r#"<math><mi>A</mi><mo>=</mo><mo>〈</mo><mn>2</mn><mo>,</mo><mo>&#xA0;</mo><mn>5</mn><mo>)</mo></math>"#;
    test_braille("Polish", expr, "⠨⠁⠀⠶⠷⠄⠼⠃⠠⠂⠀⠼⠑⠜")?;
    return Ok(());
}

#[test]
fn number_separation_p13_8() -> Result<()> {
    let expr = r#"<math><mi>B</mi><mo>=</mo><mo>{</mo><mn>2</mn><mo>,</mo><mo>&#xA0;</mo><mn>5</mn><mo>,</mo><mo>&#xA0;</mo><mn>6</mn><mo>}</mo><mo>,</mo></math>"#;
    test_braille("Polish", expr, "⠨⠃⠀⠶⠪⠼⠃⠠⠂⠀⠼⠑⠠⠂⠀⠼⠋⠕⠠⠂")?;
    return Ok(());
}

// I'have skipped "Line divisions" chapter. I have no idea how to enter the multiline expression into the format.

#[test]
fn algebraic_expressions_1() -> Result<()> {
    let expr = r#"<math><mn>3</mn><mi>a</mi><mo>+</mo><mi>b</mi><mo>+</mo><mi>c</mi></math>"#;
    test_braille("Polish", expr, "⠼⠉⠠⠁⠀⠖⠃⠀⠖⠉")?;
    return Ok(());
}

#[test]
fn algebraic_expressions_2() -> Result<()> {
    let expr = r#"<math><mn>3</mn><mi>x</mi><mo>+</mo><mi>y</mi><mo>+</mo><mi>z</mi></math>"#;
    test_braille("Polish", expr, "⠼⠉⠠⠭⠀⠖⠽⠀⠖⠵")?;
    return Ok(());
}

#[test]
fn algebraic_expressions_3() -> Result<()> {
    let expr = r#"<math><mn>3</mn><mi>a</mi><mo>+</mo><mn>2</mn><mi>b</mi><mo>+</mo><mn>4</mn><mi>c</mi></math>"#;
    test_braille("Polish", expr, "⠼⠉⠠⠁⠀⠖⠼⠃⠠⠃⠀⠖⠼⠙⠠⠉")?;
    return Ok(());
}

#[test]
fn algebraic_expressions_4() -> Result<()> {
    let expr = r#"<math><mn>3</mn><mi>x</mi><mo>+</mo><mn>2</mn><mi>y</mi><mo>+</mo><mn>4</mn><mi>z</mi></math>"#;
    test_braille("Polish", expr, "⠼⠉⠠⠭⠀⠖⠼⠃⠽⠀⠖⠼⠙⠵")?;
    return Ok(());
}

#[test]
fn algebraic_expressions_5() -> Result<()> {
    let expr = r#"<math><mn>2</mn><mi>a</mi><mi>C</mi><mo>+</mo><mn>4</mn><mi>B</mi><mi>c</mi><mo>-</mo><mn>3</mn><mo>,</mo><mn>2</mn><mi>B</mi><mi>D</mi></math>"#;
    test_braille("Polish", expr, "⠼⠃⠠⠁⠨⠉⠀⠖⠼⠙⠨⠃⠠⠉⠀⠤⠼⠉⠂⠃⠨⠃⠙")?;
    return Ok(());
}

#[test]
fn algebraic_expressions_6() -> Result<()> {
    let expr = r#"<math><mn>2</mn><mi>a</mi><mi>C</mi><mo>+</mo><mn>4</mn><mi>B</mi><mi>c</mi><mo>-</mo><mn>3</mn><mo>,</mo><mn>2</mn><mi>B</mi><mi>D</mi></math>"#;
    test_braille("Polish", expr, "⠼⠃⠠⠁⠨⠉⠀⠖⠼⠙⠨⠃⠠⠉⠀⠤⠼⠉⠂⠃⠨⠃⠙")?;
    return Ok(());
}

#[test]
fn algebraic_expressions_7() -> Result<()> {
    let expr = r#"<math><mn>2</mn><mi>a</mi><mi>b</mi><mi>c</mi><mo>-</mo><mn>3</mn><mi>B</mi><mi>C</mi><mi>d</mi><mo>-</mo><mi>b</mi><mi>d</mi></math>"#;
    test_braille("Polish", expr, "⠼⠃⠠⠁⠃⠉⠀⠤⠼⠉⠨⠃⠉⠠⠙⠀⠤⠃⠙")?;
    return Ok(());
}

#[test]
#[ignore = "decided that we should always generate Greek letter indicators"]
fn algebraic_expressions_8() -> Result<()> {
    let expr = r#"<math><mn>2</mn><mi>a</mi><mi>b</mi><mi>c</mi><mo>-</mo><mn>3</mn><mi>&#x3B1;</mi><mi>&#x3B2;</mi><mi>&#x3B3;</mi></math>"#;
    test_braille("Polish", expr, "⠼⠃⠠⠁⠃⠉⠀⠤⠼⠉⠰⠁⠃⠛")?;
    return Ok(());
}

// I have skipped alternative notations of algebraic_expressions with big letters

#[test]
fn sets_p17_1() -> Result<()> {
    let expr = r#"<math><mi>ℕ</mi><mo>&#x2282;</mo><mi>ℂ</mi><mo>&#x2282;</mo><mi>𝕎</mi><mo>&#x2282;</mo><mi>ℝ</mi></math>"#;
    test_braille("Polish", expr, "⠨⠨⠝⠀⠣⠄⠨⠨⠉⠀⠣⠄⠨⠨⠺⠀⠣⠄⠨⠨⠗")?;
    return Ok(());
}

#[test]
fn sets_p17_2() -> Result<()> {
    let expr = r#"<math><mi>𝕎</mi><mo>&#x2284;</mo><mi>𝕀</mi><mi>𝕎</mi></math>"#;
    test_braille("Polish", expr, "⠨⠨⠺⠀⠔⠣⠄⠨⠨⠊⠺")?;
    return Ok(());
}

#[test]
fn sets_p17_3() -> Result<()> {
    let expr = r#"<math><mi>ℝ</mi><mo>&#x2283;</mo><mi>ℕ</mi></math>"#;
    test_braille("Polish", expr, "⠨⠨⠗⠜⠂⠨⠨⠝")?;
    return Ok(());
}

#[test]
fn sets_p17_4() -> Result<()> {
    let expr = r#"<math><mn>2</mn><mo>&#x2208;</mo><msup><mi>ℂ</mi><mo>+</mo></msup></math>"#;
    test_braille("Polish", expr, "⠼⠃⠀⠈⠑⠨⠨⠉⠖")?;
    return Ok(());
}

#[test]
fn sets_p17_5() -> Result<()> {
    let expr = r#"<math><mi>C</mi><mo>=</mo><mi>A</mi><mo>&#x222A;</mo><mi>C</mi></math>"#;
    test_braille("Polish", expr, "⠨⠉⠀⠶⠨⠁⠀⠩⠄⠨⠉")?;
    return Ok(());
}

#[test]
fn sets_p17_6() -> Result<()> {
    let expr = r#"<math><mi>B</mi><mo>=</mo><mi>D</mi><mo>&#x2216;</mo><mi>C</mi></math>"#;
    test_braille("Polish", expr, "⠨⠃⠀⠶⠨⠙⠀⠡⠄⠨⠉")?;
    return Ok(());
}

#[test]
fn sets_p18_1() -> Result<()> {
    let expr = r#"<math><mi>B</mi><mo>=</mo><mo>{</mo><mi>x</mi><mo>:</mo><mo>&#xA0;</mo><mi>x</mi><mo>&#x2208;</mo><mi>ℕ</mi><mo>&#xA0;</mo><mi>i</mi><mo>&#xA0;</mo><mi>x</mi><mo>&lt;</mo><mn>7</mn><mo>}</mo></math>"#;
    test_braille("Polish", expr, "⠨⠃⠀⠶⠪⠠⠭⠒⠀⠭⠀⠈⠑⠨⠨⠝⠀⠊⠀⠠⠭⠀⠪⠄⠼⠛⠕")?;
    return Ok(());
}

#[test]
fn sets_p18_2() -> Result<()> {
    let expr = r#"<math><mi>B</mi><mo>=</mo><mo>{</mo><mi>x</mi><mo>:</mo><mo>&#xA0;</mo><mi>x</mi><mo>&#x2208;</mo><mi>ℕ</mi><mo>&#xA0;</mo><mo>&#x2227;</mo><mo>&#xA0;</mo><mi>x</mi><mo>&lt;</mo><mn>7</mn><mo>}</mo></math>"#;
    test_braille("Polish", expr, "⠨⠃⠀⠶⠪⠠⠭⠒⠀⠭⠀⠈⠑⠨⠨⠝⠀⠬⠂⠠⠭⠀⠪⠄⠼⠛⠕")?;
    return Ok(());
}

#[test]
fn sets_p18_3() -> Result<()> {
    let expr = r#"<math><mi>𝕎</mi><mo>=</mo><mo>{</mo><mi>x</mi><mo>:</mo><mo>&#xA0;</mo><mi>x</mi><mo>=</mo><mfrac><mi>p</mi><mi>q</mi></mfrac><mo>&#x2227;</mo><mi>p</mi><mo>&#x2208;</mo><mi>ℂ</mi><mo>&#x2227;</mo><mi>q</mi><mo>&#x2208;</mo><mi>ℂ</mi><mo>&#x2216;</mo><mo>{</mo><mn>0</mn><mo>}</mo><mo>}</mo></math>"#;
    test_braille("Polish", expr, "⠨⠨⠺⠀⠶⠪⠠⠭⠒⠀⠭⠀⠶⠏⠳⠟⠀⠬⠂⠏⠀⠈⠑⠨⠨⠉⠀⠬⠂⠠⠟⠀⠈⠑⠨⠨⠉⠀⠡⠄⠪⠼⠚⠕⠕")?;
    return Ok(());
}

#[test]
fn sets_p18_4() -> Result<()> {
    let expr = r#"<math><mo>(</mo><mo>-</mo><mo>&#x221E;</mo><mo>;</mo><mo>&#xA0;</mo><mi>a</mi><mo>)</mo><mo>=</mo><mo>{</mo><mi>x</mi><mo>:</mo><mo>&#xA0;</mo><mi>x</mi><mo>&#x2208;</mo><mi>ℝ</mi><mo>&#xA0;</mo><mi>i</mi><mo>&#xA0;</mo><mi>x</mi><mo>&lt;</mo><mi>a</mi><mo>}</mo></math>"#;
    test_braille("Polish", expr, "⠣⠤⠼⠿⠆⠀⠠⠁⠜⠀⠶⠪⠭⠒⠀⠭⠀⠈⠑⠨⠨⠗⠀⠊⠀⠠⠭⠀⠪⠄⠁⠕")?;
    return Ok(());
}

#[test]
fn sets_p18_5() -> Result<()> {
    let expr = r#"<math><mi>x</mi><mo>&#x2208;</mo><mi>A</mi><mo>&#x2216;</mo><mi>B</mi><mtext>&#xA0;wtedy i tylko wtedy gdy </mtext><mo>(</mo><mi>x</mi><mo>&#x2208;</mo><mi>A</mi><mo>&#xA0;</mo><mi>i</mi><mo>&#xA0;</mo><mi>x</mi><mo>&#x2209;</mo><mi>B</mi><mo>)</mo></math>"#;
    test_braille("Polish", expr, r"⠠⠭⠀⠈⠑⠨⠁⠀⠡⠄⠨⠃⠀⠺⠞⠑⠙⠽⠀⠊⠀⠞⠽⠇⠅⠕⠀⠺⠞⠑⠙⠽⠀⠛⠙⠽⠀⠣⠠⠭⠀⠈⠑⠨⠁⠀⠊⠀⠠⠭⠀⠔⠈⠑⠨⠃⠜")?;
    return Ok(());
}

#[test]
fn sets_p18_6() -> Result<()> {
    let expr = r#"<math><mi>x</mi><mo>&#x2208;</mo><mi>A</mi><mo>&#x2216;</mo><mi>B</mi><mo>&#xA0;</mo><mo>&#x21D4;</mo><mo>(</mo><mi>x</mi><mo>&#x2208;</mo><mi>A</mi><mo>&#xA0;</mo><mo>∧</mo><mo>&#xA0;</mo><mi>x</mi><mo>&#x2209;</mo><mi>B</mi><mo>)</mo></math>"#;
    test_braille("Polish", expr, r"⠠⠭⠀⠈⠑⠨⠁⠀⠡⠄⠨⠃⠀⠐⠶⠂⠣⠠⠭⠀⠈⠑⠨⠁⠀⠬⠂⠠⠭⠀⠔⠈⠑⠨⠃⠜")?;
    return Ok(());
}

#[test]
fn sets_p18_7() -> Result<()> {
    let expr = r#"<math><mi>A</mi><mo>=</mo><mo>{</mo><mi>x</mi><mo>:</mo><mo>&#xA0;</mo><mi>x</mi><mo>&#x2208;</mo><mi>ℂ</mi><mo>&#xA0;</mo><mi>i</mi><mo>&#xA0;</mo><mn>0</mn><mo>&lt;</mo><mi>x</mi><mo>&lt;</mo><mn>5</mn><mo>}</mo></math>"#;
    test_braille("Polish", expr, r"⠨⠁⠀⠶⠪⠠⠭⠒⠀⠭⠀⠈⠑⠨⠨⠉⠀⠊⠀⠼⠚⠀⠪⠄⠭⠀⠪⠄⠼⠑⠕")?;
    return Ok(());
}

#[test]
fn sets_p19_1() -> Result<()> {
    let expr = r#"<math><mo>(</mo><mi>a</mi><mo>,</mo><mo>&#xA0;</mo><mi>b</mi><mo>)</mo><mo>&#x2208;</mo><mo>&#xA0;</mo><mi>A</mi><mo>&#xD7;</mo><mi>B</mi><mo>&#xA0;</mo><mo>&#x21D4;</mo><mo>&#xA0;</mo><mo>(</mo><mi>a</mi><mo>&#x2208;</mo><mo>&#xA0;</mo><mi>A</mi><mo>&#x2227;</mo><mo>&#xA0;</mo><mi>b</mi><mo>&#x2208;</mo><mi>B</mi><mo>)</mo></math>"#;
    test_braille("Polish", expr, r"⠣⠠⠁⠂⠀⠃⠜⠀⠈⠑⠨⠁⠀⠦⠨⠃⠀⠐⠶⠂⠣⠠⠁⠀⠈⠑⠨⠁⠀⠬⠂⠠⠃⠀⠈⠑⠨⠃⠜")?;
    return Ok(());
}

#[test]
fn sets_p19_2() -> Result<()> {
    let expr = r#"<math><mi>A</mi><mo>&#x2282;</mo><mi>X</mi></math>"#;
    test_braille("Polish", expr, r"⠨⠁⠀⠣⠄⠨⠭")?;
    return Ok(());
}

#[test]
fn sets_p19_3() -> Result<()> {
    let expr = r#"<math><mi>A</mi><mo>'</mo><mo>=</mo><mi>X</mi><mo>&#x2216;</mo><mo>&#xA0;</mo><mi>A</mi></math>"#;
    test_braille("Polish", expr, r"⠨⠁⠔⠀⠶⠨⠭⠀⠡⠄⠨⠁")?;
    return Ok(());
}

#[test]
fn sets_p19_4() -> Result<()> {
    let expr = r#"<math><mtext>Je&#x17C;eli&#xA0;</mtext><mi>X</mi><mo>=</mo><mo>{</mo><mn>1</mn><mo>,</mo><mo>&#xA0;</mo><mn>2</mn><mo>,</mo><mo>&#xA0;</mo><mn>3</mn><mo>,</mo><mo>&#xA0;</mo><mn>4</mn><mo>,</mo><mo>&#xA0;</mo><mn>5</mn><mo>,</mo><mo>&#xA0;</mo><mn>6</mn><mo>}</mo><mo>,</mo></math>"#;
    test_braille("Polish", expr, "⠨⠚⠑⠯⠑⠇⠊⠀⠨⠭⠀⠶⠪⠼⠁⠠⠂⠀⠼⠃⠠⠂⠀⠼⠉⠠⠂⠀⠼⠙⠠⠂⠀⠼⠑⠠⠂⠀⠼⠋⠕⠠⠂")?;
    return Ok(());
}

#[test]
fn sets_p19_5() -> Result<()> {
    let expr = r#"<math><mi>A</mi><mo>&#x2282;</mo><mi>X</mi><mo>&#xA0;</mo><mi>i</mi><mo>&#xA0;</mo><mi>B</mi><mo>&#x2282;</mo><mi>X</mi><mo>,</mo></math>"#;
    test_braille_prefs("Polish", vec![("Polish_BrailleLevel", "Advanced")], expr, r"⠨⠁⠀⠣⠄⠨⠭⠀⠊⠀⠨⠃⠀⠣⠄⠨⠭⠂")?;
    return Ok(());
}

#[test]
fn sets_p19_6() -> Result<()> {
    let expr = r#"<math><mi>A</mi><mo>=</mo><mo>{</mo><mn>1</mn><mo>,</mo><mo>&#xA0;</mo><mn>2</mn><mo>,</mo><mo>&#xA0;</mo><mn>3</mn><mo>,</mo><mo>&#xA0;</mo><mn>4</mn><mo>,</mo><mo>&#xA0;</mo><mn>5</mn><mo>}</mo><mo>&#xA0;</mo><mi>i</mi><mo>&#xA0;</mo><mi>B</mi><mo>=</mo><mo>{</mo><mn>4</mn><mo>,</mo><mo>&#xA0;</mo><mn>5</mn><mo>}</mo><mo>,</mo><mo>&#xA0;</mo><mi>t</mi><mi>o</mi><mo>:</mo></math>"#;
    test_braille_prefs("Polish", vec![("Polish_BrailleLevel", "Advanced")], expr, r"⠨⠁⠀⠶⠪⠼⠁⠠⠂⠀⠼⠃⠠⠂⠀⠼⠉⠠⠂⠀⠼⠙⠠⠂⠀⠼⠑⠕⠀⠊⠀⠨⠃⠀⠶⠪⠼⠙⠠⠂⠀⠼⠑⠕⠠⠂⠀⠞⠕⠒")?;
    return Ok(());
}

#[test]
fn sets_p19_7() -> Result<()> {
    let expr = r#"<math><mi>A</mi><mo>&#x222A;</mo><mi>B</mi><mo>=</mo><mo>{</mo><mn>1</mn><mo>,</mo><mo>&#xA0;</mo><mn>2</mn><mo>,</mo><mo>&#xA0;</mo><mn>3</mn><mo>,</mo><mo>&#xA0;</mo><mn>4</mn><mo>,</mo><mo>&#xA0;</mo><mn>5</mn><mo>}</mo><mo>,</mo></math>"#;
    test_braille_prefs("Polish", vec![("Polish_BrailleLevel", "Advanced")], expr, r"⠨⠁⠀⠩⠄⠨⠃⠀⠶⠪⠼⠁⠠⠂⠀⠼⠃⠠⠂⠀⠼⠉⠠⠂⠀⠼⠙⠠⠂⠀⠼⠑⠕⠠⠂")?;
    return Ok(());
}

#[test]
fn sets_p19_8() -> Result<()> {
    let expr = r#"<math><mo>(</mo><mi>A</mi><mo>&#x222A;</mo><mi>B</mi><mo>)</mo><mo>'</mo><mo>=</mo><mo>{</mo><mn>6</mn><mo>}</mo><mo>,</mo></math>"#;
    test_braille_prefs("Polish", vec![("Polish_BrailleLevel", "Advanced")], expr, r"⠣⠨⠁⠀⠩⠄⠨⠃⠜⠔⠀⠶⠪⠼⠋⠕⠠⠂")?;
    return Ok(());
}

#[test]
fn sets_p19_9() -> Result<()> {
    let expr = r#"<math><mo>(</mo><mi>A</mi><mo>&#x2216;</mo><mi>B</mi><mo>)</mo><mo>'</mo><mo>=</mo><mo>{</mo><mn>4</mn><mo>,</mo><mo>&#xA0;</mo><mn>5</mn><mo>,</mo><mo>&#xA0;</mo><mn>6</mn><mo>}</mo><mo>,</mo></math>"#;
    test_braille_prefs("Polish", vec![("Polish_BrailleLevel", "Advanced")], expr, r"⠣⠨⠁⠀⠡⠄⠨⠃⠜⠔⠀⠶⠪⠼⠙⠠⠂⠀⠼⠑⠠⠂⠀⠼⠋⠕⠠⠂")?;
    return Ok(());
}

#[test]
fn sets_p19_10() -> Result<()> {
    let expr = r#"<math><mi>B</mi><mo>&#x2216;</mo><mi>A</mi><mo>=</mo><mi>&#xD8;</mi><mo>,</mo></math>"#;
    test_braille_prefs("Polish", vec![("Polish_BrailleLevel", "Advanced")], expr, r"⠨⠃⠀⠡⠄⠨⠁⠀⠶⠯⠕⠠⠂")?;
    return Ok(());
}

#[test]
fn sets_p19_11() -> Result<()> {
    let expr = r#"<math><mi>A</mi><mo>&#x2229;</mo><mi>B</mi><mo>=</mo><mo>{</mo><mn>4</mn><mo>,</mo><mn>5</mn><mo>}</mo><mo>.</mo></math>"#;
    test_braille("Polish", expr, r"⠨⠁⠀⠬⠄⠨⠃⠀⠶⠪⠼⠙⠠⠂⠀⠼⠑⠕⠄")?;
    return Ok(());
}

// I've skipped chapter on Graphical representation of intervals, as they require multiline representation.

#[test]
#[ignore = "uses whitespace instead of terminator due to multiplication dot"]
fn simple_projectors_p22_1() -> Result<()> {
    let expr = r#"<math><msup><mi>a</mi><mrow><mi>n</mi><mo>+</mo><mi>m</mi></mrow></msup><mo>=</mo><msup><mi>a</mi><mi>n</mi></msup><mo>&#xB7;</mo><msup><mi>a</mi><mi>m</mi></msup></math>"#;
    test_braille("Polish", expr, "⠠⠁⠬⠝⠈⠖⠍⠀⠶⠁⠬⠝⠱⠄⠁⠬⠍")?;
    return Ok(());
}

#[test]
fn simple_projectors_p22_2() -> Result<()> {
    let expr = r#"<math><msub><mi>a</mi><mi>n</mi></msub><mo>=</mo><mo>&#xA0;</mo><mstyle displaystyle="false"><mfrac><mrow><msub><mi>a</mi><mrow><mi>n</mi><mo>-</mo><mn>1</mn></mrow></msub><mo>+</mo><msub><mi>a</mi><mrow><mi>n</mi><mo>+</mo><mn>1</mn></mrow></msub></mrow><mn>2</mn></mfrac></mstyle></math>"#;
    test_braille("Polish", expr, "⠠⠁⠡⠝⠀⠶⠆⠁⠡⠝⠈⠤⠼⠁⠱⠈⠖⠁⠡⠝⠈⠖⠼⠁⠳⠆")?;
    return Ok(());
}

#[test]
fn simple_projectors_p22_3() -> Result<()> {
    let expr = r#"<math><msup><mrow><mo>(</mo><msub><mi>a</mi><mi>k</mi></msub><mo>)</mo></mrow><mn>2</mn></msup><mo>=</mo><mo>&#xA0;</mo><msub><mi>a</mi><mi>k</mi></msub><mo>&#xB7;</mo><msub><mi>a</mi><mi>k</mi></msub></math>"#;
    test_braille("Polish", expr, "⠠⠁⠡⠅⠬⠆⠀⠶⠁⠡⠅⠀⠄⠁⠡⠅")?;
    return Ok(());
}

#[test]
fn simple_projectors_p23_1() -> Result<()> {
    let expr = r#"<math><mfrac><msqrt><mn>3</mn></msqrt><mn>2</mn></mfrac><mo>=</mo><mfrac><mn>1</mn><mn>2</mn></mfrac><mo>&#xB7;</mo><msqrt><mn>3</mn></msqrt></math>"#;
    test_braille("Polish", expr, "⠩⠼⠉⠳⠆⠀⠶⠼⠁⠆⠄⠩⠼⠉")?;
    return Ok(());
}

#[test]
fn simple_projectors_p23_2() -> Result<()> {
    let expr = r#"<math><mo>(</mo><mn>3</mn><mo>+</mo><msub><mi>a</mi><mrow><mi>n</mi><mo>+</mo><mn>2</mn></mrow></msub><mo>)</mo><mo>&#xB7;</mo><mn>5</mn></math>"#;
    test_braille("Polish", expr, "⠣⠼⠉⠀⠖⠠⠁⠡⠝⠈⠖⠼⠃⠜⠄⠼⠑")?;
    return Ok(());
}

#[test]
fn simple_projectors_p23_3() -> Result<()> {
    let expr = r#"<math><msub><mi>f</mi><mi>n</mi></msub><mo>(</mo><mi>x</mi><mo>)</mo><mo>=</mo><mi>n</mi><mi>x</mi></math>"#;
    test_braille("Polish", expr, "⠠⠋⠡⠝⠱⠣⠭⠜⠀⠶⠝⠭")?;
    return Ok(());
}

#[test]
fn compound_projectors_p23_1() -> Result<()> {
    let expr = r#"<math><msqrt><mfrac><mi>x</mi><mi>y</mi></mfrac></msqrt><mo>=</mo><mfrac><msqrt><mi>x</mi></msqrt><msqrt><mi>y</mi></msqrt></mfrac></math>"#;
    test_braille("Polish", expr, "⠐⠩⠠⠭⠳⠽⠀⠶⠩⠭⠳⠩⠽")?;
    return Ok(());
}

#[test]
fn compound_projectors_p23_2() -> Result<()> {
    let expr = r#"<math><msub><mi>u</mi><mi>n</mi></msub><mo>=</mo><mroot><mrow><msup><mn>3</mn><mi>n</mi></msup><mo>+</mo><msup><mn>2</mn><mi>n</mi></msup></mrow><mi>n</mi></mroot></math>"#;
    test_braille("Polish", expr, "⠠⠥⠡⠝⠀⠶⠌⠝⠐⠩⠼⠉⠬⠝⠱⠈⠖⠼⠃⠬⠝")?;
    return Ok(());
}

#[test]
fn compound_projectors_p23_3() -> Result<()> {
    let expr = r#"<math><msup><mi>e</mi><mfrac><mi>x</mi><mn>2</mn></mfrac></msup><mo>=</mo><msqrt><msup><mi>e</mi><mi>x</mi></msup></msqrt></math>"#;
    test_braille("Polish", expr, "⠠⠑⠐⠬⠭⠳⠆⠀⠶⠐⠩⠑⠬⠭")?;
    return Ok(());
}

#[test]
fn detailed_projectors_p23_1() -> Result<()> {
    let expr = r#"<math><msup><mi>e</mi><mrow><mroot><mrow><mi>x</mi><mo>+</mo><mn>1</mn></mrow><mn>3</mn></mroot><mo>+</mo><mn>2</mn></mrow></msup><mo>&#xB7;</mo><msup><mi>e</mi><mi>x</mi></msup></math>"#;
    test_braille_prefs("Polish", vec![("Polish_BrailleLevel", "Advanced")], expr, r"⠠⠑⠨⠬⠌⠒⠩⠭⠈⠖⠼⠁⠀⠖⠼⠃⠨⠱⠄⠑⠬⠭")?;
    return Ok(());
}

#[test]
fn detailed_projectors_p23_2() -> Result<()> {
    let expr = r#"<math><mi>u</mi><mo>=</mo><mroot><mrow><msup><mi>x</mi><mn>2</mn></msup><mo>+</mo><msqrt><msup><mi>x</mi><mn>2</mn></msup><mo>+</mo><mn>1</mn></msqrt><mo>+</mo><mn>8</mn></mrow><mn>3</mn></mroot></math>"#;
    test_braille("Polish", expr, "⠠⠥⠀⠶⠌⠒⠨⠩⠭⠬⠆⠀⠖⠐⠩⠭⠬⠆⠈⠖⠼⠁⠀⠖⠼⠓⠨⠱")?;
    return Ok(());
}

#[test]
fn detailed_projectors_p24_1() -> Result<()> {
    let expr = r#"<math><mn>3</mn><mi>x</mi><mo>-</mo><mn>7</mn></math>"#;
    test_braille("Polish", expr, "⠼⠉⠠⠭⠀⠤⠼⠛")?;
    return Ok(());
}

#[test]
fn detailed_projectors_p24_2() -> Result<()> {
    let expr = r#"<math><mfrac><mn>2</mn><mn>3</mn></mfrac><mi>x</mi><mo>+</mo><mn>1</mn><mfrac><mn>2</mn><mn>3</mn></mfrac></math>"#;
    test_braille("Polish", expr, "⠼⠃⠒⠠⠭⠀⠖⠼⠁⠼⠃⠒")?;
    return Ok(());
}

#[test]
fn detailed_projectors_p24_3() -> Result<()> {
    let expr = r#"<math><mn>2</mn><mi>&#x3C0;</mi><mi>r</mi></math>"#;
    test_braille("Polish", expr, "⠼⠃⠰⠏⠠⠗")?;
    return Ok(());
}

#[test]
fn detailed_projectors_p24_4() -> Result<()> {
    let expr = r#"<math><msup><mrow><mo>(</mo><mi>x</mi><mo>+</mo><mi>y</mi><mo>)</mo></mrow><mn>2</mn></msup></math>"#;
    test_braille("Polish", expr, "⠣⠠⠭⠀⠖⠽⠜⠬⠆")?;
    return Ok(());
}

#[test]
fn detailed_projectors_p24_5() -> Result<()> {
    let expr = r#"<math><msup><mi>e</mi><mfrac><mi>x</mi><mn>2</mn></mfrac></msup></math>"#;
    test_braille("Polish", expr, "⠠⠑⠐⠬⠭⠳⠆")?;
    return Ok(());
}

#[test]
fn detailed_projectors_p24_6() -> Result<()> {
    let expr = r#"<math><msub><mi>a</mi><msub><mi>i</mi><mi>j</mi></msub></msub></math>"#;
    test_braille("Polish", expr, "⠠⠁⠐⠡⠊⠡⠚")?;
    return Ok(());
}

#[test]
fn detailed_projectors_p24_7() -> Result<()> {
    let expr = r#"<math><mfrac><mrow><mn>1</mn><mo>+</mo><mstyle displaystyle="true"><mfrac><mi>b</mi><mi>a</mi></mfrac></mstyle></mrow><mrow><mn>1</mn><mo>-</mo><mstyle displaystyle="true"><mfrac><mi>b</mi><mi>a</mi></mfrac></mstyle></mrow></mfrac></math>"#;
    test_braille("Polish", expr, "⠆⠼⠁⠀⠖⠆⠠⠃⠳⠁⠀⠳⠀⠼⠁⠀⠤⠆⠃⠳⠁⠰")?;
    return Ok(());
}

#[test]
fn detailed_projectors_p24_8() -> Result<()> {
    let expr = r#"<math><msqrt><mfrac><mi>x</mi><mi>y</mi></mfrac></msqrt></math>"#;
    test_braille("Polish", expr, "⠐⠩⠠⠭⠳⠽")?;
    return Ok(());
}

//Fractions

#[test]
fn fractions_p25_1() -> Result<()> {
    let expr = r#"<math><mfrac><mrow><mn>2</mn><mi>x</mi><mo>+</mo><mn>3</mn><mi>y</mi><mo>-</mo><mn>4</mn></mrow><mrow><mn>4</mn><mi>x</mi><mo>-</mo><mn>5</mn><mi>y</mi></mrow></mfrac></math>"#;
    test_braille_prefs("Polish", vec![("Polish_BrailleLevel", "Beginner")], expr, r"⠆⠼⠃⠠⠭⠀⠖⠼⠉⠽⠀⠤⠼⠙⠀⠳⠀⠼⠙⠭⠀⠤⠼⠑⠽⠰")?;
    return Ok(());
}

#[test]
fn fractions_p25_2a() -> Result<()> {
    let expr = r#"<math><mfrac><mn>2</mn><mn>3</mn></mfrac></math>"#;
    // corrected expected output as per email for Advanced
    test_braille_prefs("Polish", vec![("Polish_BrailleLevel", "Advanced")], expr, r"⠼⠃⠒")?;
    return Ok(());
}

#[test]
fn fractions_p25_2b() -> Result<()> {
    let expr = r#"<math><mfrac><mn>2</mn><mn>3</mn></mfrac></math>"#;
    test_braille_prefs("Polish", vec![("Polish_BrailleLevel", "Beginner")], expr, r"⠆⠼⠃⠀⠳⠀⠼⠉⠰")?;
    return Ok(());
}

#[test]
fn fractions_p25_3a() -> Result<()> {
    let expr = r#"<math><mfrac><mi>x</mi><mi>y</mi></mfrac></math>"#;
    test_braille("Polish", expr, "⠠⠭⠳⠽")?;
    return Ok(());
}

#[test]
fn fractions_p25_3b() -> Result<()> {
    let expr = r#"<math><mfrac><mi>x</mi><mi>y</mi></mfrac></math>"#;
    test_braille_prefs("Polish", vec![("Polish_BrailleLevel", "Beginner")], expr, r"⠆⠠⠭⠀⠳⠀⠽⠰")?;
    return Ok(());
}

#[test]
fn fractions_p25_4() -> Result<()> {
    let expr = r#"<math><mfrac><mrow><mn>3</mn><mi>x</mi></mrow><mn>4</mn></mfrac></math>"#;
    test_braille("Polish", expr, "⠼⠉⠠⠭⠳⠲")?;
    return Ok(());
}

#[test]
fn fractions_p25_5() -> Result<()> {
    let expr = r#"<math><mfrac><mrow><mn>2</mn><mi>a</mi></mrow><mn>7</mn></mfrac></math>"#;
    test_braille("Polish", expr, "⠼⠃⠠⠁⠳⠶")?;
    return Ok(());
}

#[test]
fn fractions_p25_6() -> Result<()> {
    let expr = r#"<math><mfrac><mrow><mi>a</mi><mi>b</mi><mo>+</mo><mi>c</mi><mi>d</mi></mrow><mn>4</mn></mfrac></math>"#;
    // test_braille("Polish", expr, "⠆⠠⠁⠃⠀⠖⠉⠙⠀⠳⠲");
    test_braille("Polish", expr, r"⠆⠠⠁⠃⠀⠖⠉⠙⠀⠳⠲")?;
    return Ok(());
}

#[test]
fn fractions_p26_1() -> Result<()> {
    let expr = r#"<math><mfrac><mn>1</mn><mn>2</mn></mfrac></math>"#;
    test_braille("Polish", expr, "⠼⠁⠆")?;
    return Ok(());
}

#[test]
fn fractions_p26_2() -> Result<()> {
    let expr = r#"<math><mfrac><mn>3</mn><mn>14</mn></mfrac></math>"#;
    test_braille("Polish", expr, "⠼⠉⠂⠲")?;
    return Ok(());
}

#[test]
fn fractions_p26_3() -> Result<()> {
    let expr = r#"<math><mfrac><mn>17</mn><mn>5</mn></mfrac></math>"#;
    test_braille("Polish", expr, "⠼⠁⠛⠢")?;
    return Ok(());
}

#[test]
fn fractions_p26_4() -> Result<()> {
    let expr = r#"<math><mfrac><mn>138</mn><mn>43</mn></mfrac></math>"#;
    test_braille("Polish", expr, "⠼⠁⠉⠓⠲⠒")?;
    return Ok(());
}

#[test]
fn fractions_p26_5() -> Result<()> {
    let expr = r#"<math><mn>2</mn><mfrac><mn>3</mn><mn>4</mn></mfrac></math>"#;
    test_braille("Polish", expr, "⠼⠃⠼⠉⠲")?;
    return Ok(());
}

#[test]
fn fractions_p26_6() -> Result<()> {
    let expr = r#"<math><mn>4</mn><mfrac><mn>7</mn><mn>15</mn></mfrac></math>"#;
    test_braille("Polish", expr, "⠼⠙⠼⠛⠂⠢")?;
    return Ok(());
}

#[test]
fn fractions_p26_7() -> Result<()> {
    let expr = r#"<math><mn>12</mn><mfrac><mn>14</mn><mn>17</mn></mfrac></math>"#;
    test_braille("Polish", expr, "⠼⠁⠃⠼⠁⠙⠂⠶")?;
    return Ok(());
}

#[test]
fn fractions_p26_8a() -> Result<()> {
    let expr = r#"<math><mfrac><mrow><mi>x</mi><mo>+</mo><mi>y</mi></mrow><mrow><mi>x</mi><mo>-</mo><mi>y</mi></mrow></mfrac></math>"#;
    test_braille("Polish", expr, "⠠⠭⠈⠖⠽⠳⠭⠈⠤⠽")?;
    return Ok(());
}

#[test]
fn fractions_p26_8b() -> Result<()> {
    let expr = r#"<math><mfrac><mrow><mi>x</mi><mo>+</mo><mi>y</mi></mrow><mrow><mi>x</mi><mo>-</mo><mi>y</mi></mrow></mfrac></math>"#;
    test_braille_prefs("Polish", vec![("Polish_BrailleLevel", "Beginner")], expr, r"⠆⠠⠭⠀⠖⠽⠀⠳⠀⠭⠀⠤⠽⠰")?;
    return Ok(());
}

#[test]
fn fractions_p26_9() -> Result<()> {
    let expr = r#"<math><mfrac><mrow><mo>-</mo><mi>p</mi><mo>-</mo><mi>q</mi></mrow><mi>n</mi></mfrac></math>"#;
    test_braille("Polish", expr, "⠆⠤⠠⠏⠈⠤⠟⠳⠝")?;
    return Ok(());
}

#[test]
fn fractions_p26_10() -> Result<()> {
    let expr = r#"<math><mfrac><mrow><mi>p</mi><mo>+</mo><mi>q</mi></mrow><mrow><mo>-</mo><mi>n</mi></mrow></mfrac></math>"#;
    test_braille("Polish", expr, "⠠⠏⠈⠖⠟⠳⠈⠤⠝")?;
    return Ok(());
}

#[test]
fn fractions_p26_11() -> Result<()> {
    let expr = r#"<math><mfrac><mrow><mo>(</mo><mi>n</mi><mo>+</mo><mn>1</mn><mo>)</mo><mo>!</mo></mrow><mrow><mn>2</mn><mi>n</mi></mrow></mfrac></math>"#;
    test_braille("Polish", expr, "⠣⠠⠝⠀⠖⠼⠁⠜⠫⠈⠳⠼⠃⠝")?;
    return Ok(());
}

#[test]
fn fractions_p27_1() -> Result<()> {
    let expr = r#"<math><mfrac><mn>3</mn><mn>4</mn></mfrac><mo>+</mo><mfrac><mn>3</mn><mn>4</mn></mfrac><mo>=</mo><mfrac><mn>6</mn><mn>4</mn></mfrac><mo>=</mo><mfrac><mn>3</mn><mn>2</mn></mfrac><mo>=</mo><mn>1</mn><mfrac><mn>1</mn><mn>2</mn></mfrac></math>"#;
    test_braille("Polish", expr, "⠼⠉⠲⠀⠖⠼⠉⠲⠀⠶⠼⠋⠲⠀⠶⠼⠉⠆⠀⠶⠼⠁⠼⠁⠆")?;
    return Ok(());
}

#[test]
fn fractions_p27_2() -> Result<()> {
    let expr = r#"<math><mn>2</mn><mo>,</mo><mn>6</mn><mo>-</mo><mfrac><mn>6</mn><mn>15</mn></mfrac><mo>=</mo><mn>2</mn><mfrac><mn>18</mn><mn>30</mn></mfrac><mo>-</mo><mfrac><mn>12</mn><mn>30</mn></mfrac><mo>=</mo><mn>2</mn><mfrac><mn>6</mn><mn>30</mn></mfrac><mo>=</mo><mn>2</mn><mfrac><mn>1</mn><mn>5</mn></mfrac><mo>=</mo><mn>2</mn><mo>,</mo><mn>2</mn></math>"#;
    test_braille("Polish", expr, "⠼⠃⠂⠋⠀⠤⠼⠋⠂⠢⠀⠶⠼⠃⠼⠁⠓⠒⠴⠀⠤⠼⠁⠃⠒⠴⠀⠶⠼⠃⠼⠋⠒⠴⠀⠶⠼⠃⠼⠁⠢⠀⠶⠼⠃⠂⠃")?;
    return Ok(());
}

#[test]
fn fractions_p27_3a() -> Result<()> {
    let expr = r#"<math><mn>4</mn><mfrac><mn>3</mn><mn>4</mn></mfrac><mo>&#xB7;</mo><mo>(</mo><mn>1</mn><mo>,</mo><mn>2</mn><mo>-</mo><mn>0</mn><mo>,</mo><mn>8</mn><mo>)</mo><mo>=</mo><mn>4</mn><mfrac><mn>3</mn><mn>4</mn></mfrac><mo>&#xB7;</mo><mn>0</mn><mo>,</mo><mn>4</mn><mo>=</mo><mfrac><mn>19</mn><mn>4</mn></mfrac><mo>&#xB7;</mo><mfrac><mn>4</mn><mn>10</mn></mfrac><mo>=</mo><mfrac><mn>19</mn><mn>10</mn></mfrac><mo>=</mo><mn>1</mn><mo>,</mo><mn>9</mn></math>"#;
    test_braille("Polish", expr, "⠼⠙⠼⠉⠲⠄⠣⠼⠁⠂⠃⠀⠤⠼⠚⠂⠓⠜⠀⠶⠼⠙⠼⠉⠲⠄⠼⠚⠂⠙⠀⠶⠼⠁⠊⠲⠄⠼⠙⠂⠴⠀⠶⠼⠁⠊⠂⠴⠀⠶⠼⠁⠂⠊")?;
    return Ok(());
}

#[test]
fn fractions_p27_3b() -> Result<()> {
    let expr = r#"<math><mn>4</mn><mfrac><mn>3</mn><mn>4</mn></mfrac><mo>&#xB7;</mo><mo>(</mo><mn>1</mn><mo>,</mo><mn>2</mn><mo>-</mo><mn>0</mn><mo>,</mo><mn>8</mn><mo>)</mo><mo>=</mo><mn>4</mn><mfrac><mn>3</mn><mn>4</mn></mfrac><mo>&#xB7;</mo><mn>0</mn><mo>,</mo><mn>4</mn><mo>=</mo><mfrac><mn>19</mn><mn>4</mn></mfrac><mo>&#xB7;</mo><mfrac><mn>4</mn><mn>10</mn></mfrac><mo>=</mo><mfrac><mn>19</mn><mn>10</mn></mfrac><mo>=</mo><mn>1</mn><mo>,</mo><mn>9</mn></math>"#;
    test_braille("Polish", expr, "⠼⠙⠼⠉⠲⠀⠄⠣⠼⠁⠂⠃⠀⠤⠼⠚⠂⠓⠜⠀⠶⠼⠙⠼⠉⠲⠀⠄⠼⠚⠂⠙⠀⠶⠼⠁⠊⠲⠀⠄⠼⠙⠂⠴⠀⠶⠼⠁⠊⠂⠴⠀⠶⠼⠁⠂⠊")?;
    return Ok(());
}

#[test]
fn fractions_p27_4() -> Result<()> {
    let expr = r#"<math><mfrac><mrow><mn>3</mn><mi>a</mi><mo>+</mo><mi>b</mi></mrow><mi>c</mi></mfrac></math>"#;
    test_braille("Polish", expr, "⠼⠉⠠⠁⠈⠖⠃⠳⠉")?;
    return Ok(());
}

#[test]
fn fractions_p27_5() -> Result<()> {
    let expr = r#"<math><mfrac><mrow><mn>3</mn><mi>x</mi><mo>+</mo><mi>y</mi></mrow><mi>z</mi></mfrac></math>"#;
    test_braille("Polish", expr, "⠼⠉⠠⠭⠈⠖⠽⠳⠵")?;
    return Ok(());
}

#[test]
fn fractions_p27_6() -> Result<()> {
    let expr = r#"<math><mfrac><mrow><mn>3</mn><mi>a</mi><mo>+</mo><mn>2</mn><mi>b</mi></mrow><mrow><mn>4</mn><mi>c</mi></mrow></mfrac></math>"#;
    test_braille("Polish", expr, "⠆⠼⠉⠠⠁⠈⠖⠼⠃⠠⠃⠳⠼⠙⠠⠉")?;
    return Ok(());
}

#[test]
fn fractions_p27_7() -> Result<()> {
    let expr = r#"<math><mfrac><mrow><mn>0</mn><mo>,</mo><mn>6</mn><mi>a</mi><mo>+</mo><mn>1</mn><mo>,</mo><mn>4</mn><mi>b</mi></mrow><mrow><mn>5</mn><mi>a</mi><mo>-</mo><mn>6</mn><mi>b</mi></mrow></mfrac></math>"#;
    test_braille("Polish", expr, "⠆⠼⠚⠂⠋⠠⠁⠈⠖⠼⠁⠂⠙⠠⠃⠳⠼⠑⠠⠁⠈⠤⠼⠋⠠⠃")?;
    return Ok(());
}

#[test]
fn fractions_p28_1() -> Result<()> {
    let expr = r#"<math><mi>γ</mi><mo>=</mo><mfrac><mrow><mi>&#x3B1;</mi><mo>+</mo><mi>&#x3B2;</mi></mrow><mn>2</mn></mfrac><mo>=</mo><mfrac><mi>&#x3B1;</mi><mn>2</mn></mfrac><mo>+</mo><mfrac><mi>&#x3B2;</mi><mn>2</mn></mfrac></math>"#;
    test_braille("Polish", expr, "⠰⠛⠀⠶⠰⠁⠈⠖⠰⠃⠳⠆⠀⠶⠰⠁⠳⠆⠀⠖⠰⠃⠳⠆")?;
    return Ok(());
}

#[test]
fn fractions_p28_2() -> Result<()> {
    let expr = r#"<math><mn>2</mn><mfrac><mn>2</mn><mn>3</mn></mfrac><mo>+</mo><mfrac><mrow><mn>3</mn><mo>,</mo><mn>2</mn><mi>p</mi><mo>-</mo><mn>1</mn><mo>,</mo><mn>6</mn><mi>q</mi></mrow><mrow><mn>1</mn><mstyle displaystyle="true"><mfrac><mn>2</mn><mn>5</mn></mfrac></mstyle><mo>+</mo><mstyle displaystyle="true"><mfrac><mn>1</mn><mn>2</mn></mfrac></mstyle><mi>r</mi></mrow></mfrac></math>"#;
    test_braille("Polish", expr, "⠼⠃⠼⠃⠒⠀⠖⠆⠼⠉⠂⠃⠠⠏⠈⠤⠼⠁⠂⠋⠟⠳⠼⠁⠼⠃⠢⠈⠖⠼⠁⠆⠗")?;
    return Ok(());
}

#[test]
fn fractions_p28_3() -> Result<()> {
    let expr = r#"<math><mn>2</mn><mfrac><mn>2</mn><mn>3</mn></mfrac><mo>+</mo><mn>3</mn><mi>p</mi><mo>-</mo><mfrac><mrow><mn>1</mn><mo>,</mo><mn>6</mn><mi>q</mi></mrow><mrow><mn>1</mn><mstyle displaystyle="true"><mfrac><mn>2</mn><mn>5</mn></mfrac></mstyle><mo>+</mo><mstyle displaystyle="true"><mfrac><mn>1</mn><mn>2</mn></mfrac></mstyle><mi>r</mi></mrow></mfrac></math>"#;
    test_braille("Polish", expr, "⠼⠃⠼⠃⠒⠀⠖⠼⠉⠠⠏⠀⠤⠼⠁⠂⠋⠟⠳⠼⠁⠼⠃⠢⠈⠖⠼⠁⠆⠗")?;
    return Ok(());
}

#[test]
fn fractions_p28_4() -> Result<()> {
    let expr = r#"<math><mfrac><mrow><mstyle displaystyle="true"><mfrac><mn>2</mn><mn>3</mn></mfrac></mstyle><mo>+</mo><mn>3</mn><mo>,</mo><mn>2</mn><mi>p</mi><mo>-</mo><mn>1</mn><mo>,</mo><mn>6</mn><mi>q</mi></mrow><mrow><mn>1</mn><mstyle displaystyle="true"><mfrac><mn>2</mn><mn>5</mn></mfrac></mstyle></mrow></mfrac><mo>+</mo><mfrac><mn>1</mn><mn>2</mn></mfrac><mi>r</mi></math>"#;
    test_braille("Polish", expr, "⠆⠼⠃⠒⠈⠖⠼⠉⠂⠃⠠⠏⠈⠤⠼⠁⠂⠋⠟⠳⠼⠁⠼⠃⠢⠀⠖⠼⠁⠆⠗")?;
    return Ok(());
}

#[test]
fn fractions_p28_5() -> Result<()> {
    let expr = r#"<math><mfrac><mrow><mstyle displaystyle="true"><mfrac><mn>2</mn><mn>3</mn></mfrac></mstyle><mo>+</mo><mn>3</mn><mo>,</mo><mn>2</mn><mi>p</mi><mo>-</mo><mn>1</mn><mo>,</mo><mn>6</mn><mi>q</mi><mo>+</mo><mstyle displaystyle="true"><mfrac><mn>1</mn><mn>2</mn></mfrac></mstyle><mi>r</mi></mrow><mrow><mn>1</mn><mstyle displaystyle="true"><mfrac><mn>2</mn><mn>5</mn></mfrac></mstyle></mrow></mfrac></math>"#;
    test_braille("Polish", expr, "⠆⠼⠃⠒⠈⠖⠼⠉⠂⠃⠠⠏⠈⠤⠼⠁⠂⠋⠟⠈⠖⠼⠁⠆⠗⠳⠼⠁⠼⠃⠢")?;
    return Ok(());
}

#[test]
fn fractions_p28_6a() -> Result<()> {
    let expr = r#"<math><mfrac><mrow><mstyle displaystyle="true"><mfrac><mn>2</mn><mn>3</mn></mfrac></mstyle><mi>x</mi><mo>-</mo><mn>1</mn><mo>,</mo><mn>5</mn><mi>y</mi></mrow><mrow><mn>2</mn><mi>y</mi><mo>+</mo><mstyle displaystyle="true"><mfrac><mn>3</mn><mn>8</mn></mfrac></mstyle><mi>z</mi></mrow></mfrac></math>"#;
    test_braille("Polish", expr, "⠆⠼⠃⠒⠠⠭⠀⠤⠼⠁⠂⠑⠽⠀⠳⠀⠼⠃⠽⠀⠖⠼⠉⠦⠵⠰")?;
    return Ok(());
}

#[test]
fn fractions_p28_6b() -> Result<()> {
    let expr = r#"<math><mfrac><mrow><mstyle displaystyle="true"><mfrac><mn>2</mn><mn>3</mn></mfrac></mstyle><mi>x</mi><mo>-</mo><mn>1</mn><mo>,</mo><mn>5</mn><mi>y</mi></mrow><mrow><mn>2</mn><mi>y</mi><mo>+</mo><mstyle displaystyle="true"><mfrac><mn>3</mn><mn>8</mn></mfrac></mstyle><mi>z</mi></mrow></mfrac></math>"#;
    test_braille("Polish", expr, "⠆⠼⠃⠒⠠⠭⠈⠤⠼⠁⠂⠑⠽⠳⠼⠃⠽⠈⠖⠼⠉⠦⠵")?;
    return Ok(());
}

#[test]
fn fractions_p28_7() -> Result<()> {
    let expr = r#"<math><mo>-</mo><mfrac><mrow><mi>x</mi><mo>-</mo><mi>y</mi></mrow><mn>2</mn></mfrac><mo>=</mo><mfrac><mrow><mo>-</mo><mo>(</mo><mi>x</mi><mo>-</mo><mi>y</mi><mo>)</mo></mrow><mn>2</mn></mfrac><mo>=</mo><mfrac><mrow><mo>-</mo><mi>x</mi><mo>+</mo><mi>y</mi></mrow><mn>2</mn></mfrac><mo>=</mo><mfrac><mrow><mi>y</mi><mo>-</mo><mi>x</mi></mrow><mn>2</mn></mfrac></math>"#;
    test_braille("Polish", expr, "⠤⠆⠠⠭⠈⠤⠽⠳⠆⠀⠶⠆⠤⠣⠭⠀⠤⠽⠜⠳⠆⠀⠶⠆⠤⠭⠈⠖⠽⠳⠆⠀⠶⠽⠈⠤⠭⠳⠆")?;
    return Ok(());
}

#[test]
fn fractions_p29_1() -> Result<()> {
    let expr = r#"<math><mfrac><mrow><mn>2</mn><mi>x</mi></mrow><mrow><mn>3</mn><mi>y</mi><mo>&#xB7;</mo><mi>z</mi></mrow></mfrac></math>"#;
    test_braille("Polish", expr, "⠼⠃⠠⠭⠳⠼⠉⠽⠄⠵")?;
    return Ok(());
}

#[test]
fn fractions_p29_2() -> Result<()> {
    let expr = r#"<math><mfrac><mrow><mn>2</mn><mi>x</mi></mrow><mrow><mn>3</mn><mi>y</mi></mrow></mfrac><mo>&#xB7;</mo><mi>z</mi></math>"#;
    test_braille("Polish", expr, "⠼⠃⠠⠭⠳⠼⠉⠽⠀⠄⠵")?;
    return Ok(());
}

#[test]
fn fractions_p29_3() -> Result<()> {
    let expr = r#"<math><msup><mi>e</mi><mrow><mi>x</mi><mo>&#xB7;</mo><mi>y</mi></mrow></msup></math>"#;
    test_braille("Polish", expr, "⠠⠑⠬⠭⠄⠽")?;
    return Ok(());
}

#[test]
fn fractions_p29_4() -> Result<()> {
    let expr = r#"<math><msup><mi>e</mi><mi>x</mi></msup><mo>&#xB7;</mo><mi>y</mi></math>"#;
    test_braille("Polish", expr, "⠠⠑⠬⠭⠀⠄⠽")?;
    return Ok(());
}

#[test]
fn fractions_p30_1() -> Result<()> {
    let expr = r#"<math><mfrac><mrow><mi>a</mi><mo>+</mo><mi>b</mi></mrow><mrow><mi>a</mi><mo>-</mo><mi>b</mi></mrow></mfrac><mo>=</mo><mfrac><mrow><mn>1</mn><mo>+</mo><mstyle displaystyle="true"><mfrac><mi>b</mi><mi>a</mi></mfrac></mstyle></mrow><mrow><mn>1</mn><mo>-</mo><mstyle displaystyle="true"><mfrac><mi>b</mi><mi>a</mi></mfrac></mstyle></mrow></mfrac></math>"#;
    test_braille("Polish", expr, r"⠠⠁⠈⠖⠃⠳⠁⠈⠤⠃⠀⠶⠆⠼⠁⠀⠖⠆⠃⠳⠁⠀⠳⠀⠼⠁⠀⠤⠆⠃⠳⠁⠰")?;
    return Ok(());
}

#[test]
fn fractions_p30_2() -> Result<()> {
    let expr = r#"<math>
            <mfrac>
                <mrow>
                    <mstyle displaystyle="true"><mfrac><mi>a</mi><mn>3</mn></mfrac></mstyle><mo>-</mo>
                    <mstyle displaystyle="true"><mfrac><mi>b</mi><mn>4</mn></mfrac></mstyle>
                </mrow>
                    <mstyle displaystyle="true"><mfrac><mi>x</mi><mrow><mi>x</mi><mo>+</mo><mi>y</mi></mrow></mfrac></mstyle>
            </mfrac></math>"#;
    test_braille("Polish", expr, "⠆⠆⠠⠁⠳⠒⠀⠤⠆⠃⠳⠲⠀⠳⠀⠆⠭⠳⠭⠈⠖⠽⠰")?;
    return Ok(());
}

#[test]
fn fractions_p30_3() -> Result<()> {
    let expr = r#"<math><mfrac><mrow><mi>x</mi><mo>+</mo><mstyle displaystyle="true"><mfrac><mrow><mi>x</mi><mo>+</mo><mn>1</mn></mrow><mrow><mi>y</mi><mo>-</mo><mn>1</mn></mrow></mfrac></mstyle></mrow><mrow><mi>x</mi><mo>-</mo><mstyle displaystyle="true"><mfrac><mrow><mi>x</mi><mo>+</mo><mn>1</mn></mrow><mrow><mi>y</mi><mo>+</mo><mn>1</mn></mrow></mfrac></mstyle></mrow></mfrac><mo>=</mo><mfrac><mstyle displaystyle="true"><mfrac><mrow><mi>x</mi><mo>(</mo><mi>y</mi><mo>-</mo><mn>1</mn><mo>)</mo><mo>+</mo><mo>(</mo><mi>x</mi><mo>+</mo><mn>1</mn><mo>)</mo></mrow><mrow><mi>y</mi><mo>-</mo><mn>1</mn></mrow></mfrac></mstyle><mstyle displaystyle="true"><mfrac><mrow><mi>x</mi><mo>(</mo><mi>y</mi><mo>+</mo><mn>1</mn><mo>)</mo><mo>-</mo><mo>(</mo><mi>x</mi><mo>+</mo><mn>1</mn><mo>)</mo></mrow><mrow><mi>y</mi><mo>+</mo><mn>1</mn></mrow></mfrac></mstyle></mfrac><mo>=</mo><mfrac><mstyle displaystyle="true"><mfrac><mrow><mi>x</mi><mi>y</mi><mo>-</mo><mi>x</mi><mo>+</mo><mi>x</mi><mo>-</mo><mn>1</mn></mrow><mrow><mi>y</mi><mo>-</mo><mn>1</mn></mrow></mfrac></mstyle><mstyle displaystyle="true"><mfrac><mrow><mi>x</mi><mi>y</mi><mo>+</mo><mi>x</mi><mo>-</mo><mi>x</mi><mo>-</mo><mn>1</mn></mrow><mrow><mi>y</mi><mo>+</mo><mn>1</mn></mrow></mfrac></mstyle></mfrac><mo>=</mo><mfrac><mrow><mi>y</mi><mo>+</mo><mn>1</mn></mrow><mrow><mi>y</mi><mo>-</mo><mn>1</mn></mrow></mfrac></math>"#;
    test_braille("Polish", expr, "⠆⠠⠭⠀⠖⠆⠭⠈⠤⠼⠁⠳⠽⠈⠤⠼⠁⠀⠳⠀⠭⠀⠤⠆⠭⠈⠖⠼⠁⠳⠽⠈⠖⠼⠁⠰⠀⠶⠆⠆⠭⠣⠽⠀⠤⠼⠁⠜⠈⠖⠣⠭⠀⠤⠼⠁⠜⠳⠽⠈⠤⠼⠁⠀⠳⠀⠆⠭⠣⠽⠀⠖⠼⠁⠜⠈⠤⠣⠭⠀⠖⠼⠁⠜⠳⠽⠈⠖⠼⠁⠰⠠⠶⠆⠆⠭⠽⠈⠤⠭⠈⠖⠭⠈⠤⠼⠁⠳⠽⠈⠤⠼⠁⠀⠳⠀⠆⠭⠽⠈⠖⠭⠈⠤⠭⠈⠤⠼⠁⠳⠽⠈⠖⠼⠁⠰⠀⠶⠽⠈⠖⠼⠁⠳⠽⠈⠤⠼⠁")?;
    return Ok(());
}

#[test]
fn fractions_p30_4() -> Result<()> {
    let expr = r#"<math><mfrac><mrow><mn>2</mn>
            <mstyle displaystyle="true"><mfrac><mn>2</mn><mn>3</mn></mfrac></mstyle><mo>+</mo>
            <mstyle displaystyle="true"><mfrac><mrow><mn>1</mn><mo>,</mo><mn>2</mn><mo>-</mo><mn>0</mn><mo>,</mo><mn>7</mn></mrow><mrow><mn>2</mn><mo>,</mo><mn>6</mn></mrow></mfrac></mstyle>
            </mrow><mrow>
            <mstyle displaystyle="true"><mfrac><mrow><mn>1</mn><mo>,</mo><mn>2</mn><mo>+</mo><mn>0</mn><mo>,</mo><mn>6</mn></mrow><mrow><mn>0</mn><mo>,</mo><mn>8</mn></mrow></mfrac></mstyle>
            <mo>-</mo><mn>3</mn>
            <mstyle displaystyle="true"><mfrac><mn>4</mn><mn>5</mn></mfrac></mstyle>
            </mrow></mfrac></math>"#;
    test_braille("Polish", expr, "⠆⠼⠃⠼⠃⠒⠀⠖⠆⠼⠁⠂⠃⠈⠤⠼⠚⠂⠛⠳⠼⠃⠂⠋⠀⠳⠀⠆⠼⠁⠂⠃⠈⠖⠼⠚⠂⠋⠳⠼⠚⠂⠓⠀⠤⠼⠉⠼⠙⠢⠰")?;
    return Ok(());
}

#[test]
fn fractions_p30_5() -> Result<()> {
    let expr = r#"<math><mfrac><mn>1</mn><mn>2</mn></mfrac><mo>+</mo><mfrac><mn>1</mn><mrow><mn>2</mn><mo>&#xB7;</mo><mn>3</mn></mrow></mfrac><mo>+</mo><mfrac><mn>1</mn><mrow><mn>2</mn><mo>&#xB7;</mo><mn>3</mn><mo>&#xB7;</mo><mn>4</mn></mrow></mfrac></math>"#;
    test_braille("Polish", expr, "⠼⠁⠆⠀⠖⠼⠁⠳⠼⠃⠄⠼⠉⠀⠖⠼⠁⠳⠼⠃⠄⠼⠉⠄⠼⠙")?;
    return Ok(());
}

// exponents and indices

#[test]
fn scripts_p31_1() -> Result<()> {
    let expr = r#"<math><msub><mi>a</mi><mrow><mo>-</mo><mn>3</mn></mrow></msub></math>"#;
    test_braille("Polish", expr, "⠠⠁⠡⠤⠒")?;
    return Ok(());
}

#[test]
fn scripts_p31_2() -> Result<()> {
    let expr = r#"<math><msub><mi>a</mi><mn>0</mn></msub></math>"#;
    test_braille("Polish", expr, "⠠⠁⠡⠴")?;
    return Ok(());
}

#[test]
fn scripts_p31_3() -> Result<()> {
    let expr = r#"<math><msub><mi>a</mi><mn>12</mn></msub></math>"#;
    test_braille("Polish", expr, "⠠⠁⠡⠂⠆")?;
    return Ok(());
}

#[test]
#[ignore = "there is no way to know that these are not exponents"]
fn scripts_p31_4() -> Result<()> {
    let expr = r#"<math><msup><mi>b</mi><mrow><mo>-</mo><mn>1</mn></mrow></msup></math>"#;
    test_braille("Polish", expr, "⠠⠃⠌⠤⠂")?;
    return Ok(());
}

#[test]
#[ignore = "there is no way to know that these are not exponents"]
fn scripts_p31_5() -> Result<()> {
    let expr = r#"<math><msup><mi>b</mi><mn>4</mn></msup></math>"#;
    test_braille("Polish", expr, "⠠⠃⠌⠲")?;
    return Ok(());
}

#[test]
#[ignore = "there is no way to know that these are not exponents"]
fn scripts_p31_6() -> Result<()> {
    let expr = r#"<math><msup><mi>b</mi><mn>31</mn></msup></math>"#;
    test_braille("Polish", expr, "⠠⠃⠌⠒⠂")?;
    return Ok(());
}

#[test]
fn scripts_p31_7() -> Result<()> {
    let expr = r#"<math><msup><mi>x</mi><mrow><mo>-</mo><mn>3</mn></mrow></msup></math>"#;
    test_braille("Polish", expr, "⠠⠭⠬⠤⠒")?;
    return Ok(());
}

#[test]
fn scripts_p31_8() -> Result<()> {
    let expr = r#"<math><msup><mn>5</mn><mn>4</mn></msup></math>"#;
    test_braille("Polish", expr, "⠼⠑⠬⠲")?;
    return Ok(());
}

#[test]
fn scripts_p31_9() -> Result<()> {
    let expr = r#"<math><msup><mi>b</mi><mn>10</mn></msup></math>"#;
    test_braille("Polish", expr, "⠠⠃⠬⠂⠴")?;
    return Ok(());
}

#[test]
fn scripts_p31_10() -> Result<()> {
    let expr = r#"<math><msub><mi>a</mi><mn>11</mn></msub><msub><mi>a</mi><mn>22</mn></msub><mo>-</mo><msub><mi>a</mi><mn>12</mn></msub><msub><mi>a</mi><mn>21</mn></msub></math>"#;
    test_braille("Polish", expr, "⠠⠁⠡⠂⠂⠁⠡⠆⠆⠀⠤⠁⠡⠂⠆⠁⠡⠆⠂")?;
    return Ok(());
}

#[test]
fn scripts_p31_11() -> Result<()> {
    let expr = r#"<math><msup><mi>a</mi><mn>2</mn></msup><msup><mi>b</mi><mn>3</mn></msup><msup><mi>c</mi><mn>2</mn></msup></math>"#;
    test_braille("Polish", expr, "⠠⠁⠬⠆⠃⠬⠒⠉⠬⠆")?;
    return Ok(());
}

#[test]
fn scripts_p31_12() -> Result<()> {
    let expr = r#"<math><mn>2</mn><msup><mi>x</mi><mn>2</mn></msup><msqrt><mi>y</mi></msqrt></math>"#;
    test_braille("Polish", expr, "⠼⠃⠠⠭⠬⠆⠩⠽")?;
    return Ok(());
}

#[test]
fn scripts_p31_13() -> Result<()> {
    let expr = r#"<math><mfrac><msup><mi>x</mi><mn>3</mn></msup><mrow><mn>3</mn><mi>y</mi></mrow></mfrac></math>"#;
    test_braille("Polish", expr, "⠠⠭⠬⠒⠳⠼⠉⠽")?;
    return Ok(());
}

#[test]
fn scripts_p31_14() -> Result<()> {
    let expr = r#"<math><msup><mfenced><mfrac><mn>1</mn><mn>3</mn></mfrac></mfenced><mrow><mo>-</mo><mn>4</mn></mrow></msup></math>"#;
    test_braille("Polish", expr, "⠼⠁⠒⠬⠤⠼⠙")?;
    return Ok(());
}

#[test]
fn scripts_p31_15() -> Result<()> {
    let expr = r#"<math><msup><mi>a</mi><mfrac><mn>1</mn><mn>2</mn></mfrac></msup></math>"#;
    test_braille("Polish", expr, "⠠⠁⠬⠼⠁⠆")?;
    return Ok(());
}

#[test]
fn scripts_p31_16() -> Result<()> {
    let expr = r#"<math><msup><mn>8</mn><mrow><mo>-</mo><mn>3</mn></mrow></msup><mo>=</mo><mfrac><mn>1</mn><msup><mn>8</mn><mn>3</mn></msup></mfrac></math>"#;
    test_braille("Polish", expr, "⠼⠓⠬⠤⠒⠀⠶⠼⠁⠳⠼⠓⠬⠒")?;
    return Ok(());
}

#[test]
fn scripts_p31_17() -> Result<()> {
    let expr = r#"<math><msup><mi>x</mi><mi>n</mi></msup><mo>+</mo><mn>9</mn></math>"#;
    test_braille("Polish", expr, "⠠⠭⠬⠝⠀⠖⠼⠊")?;
    return Ok(());
}

#[test]
fn scripts_p31_18() -> Result<()> {
    let expr = r#"<math><msup><mi>y</mi><mrow><mn>2</mn><mi>n</mi><mo>-</mo><mn>3</mn></mrow></msup><mo>&#xB7;</mo><mi>z</mi></math>"#;
    test_braille("Polish", expr, "⠠⠽⠬⠼⠃⠝⠈⠤⠼⠉⠀⠄⠵")?;
    return Ok(());
}

#[test]
fn scripts_p31_19() -> Result<()> {
    let expr = r#"<math><msub><mi>a</mi><mi>n</mi></msub><mo>+</mo><mn>2</mn></math>"#;
    test_braille("Polish", expr, "⠠⠁⠡⠝⠀⠖⠼⠃")?;
    return Ok(());
}

#[test]
fn scripts_p31_20() -> Result<()> {
    let expr = r#"<math><msub><mi>a</mi><mrow><mi>n</mi><mo>+</mo><mn>1</mn></mrow></msub><mo>-</mo><mn>5</mn></math>"#;
    test_braille("Polish", expr, "⠠⠁⠡⠝⠈⠖⠼⠁⠀⠤⠼⠑")?;
    return Ok(());
}

#[test]
#[ignore = "uses whitespace instead of terminator due to multiplication dot"]
fn scripts_p32_1() -> Result<()> {
    let expr = r#"<math><msub><mi>x</mi><mi>i</mi></msub><mo>&#xB7;</mo><msub><mi>x</mi><mi>j</mi></msub></math>"#;
    test_braille("Polish", expr, "⠠⠭⠡⠊⠱⠄⠭⠡⠚")?;
    return Ok(());
}

#[test]
#[ignore = "uses whitespace instead of terminator due to multiplication dot"]
fn scripts_p32_2() -> Result<()> {
    let expr = r#"<math><msup><mi>y</mi><mrow><mn>2</mn><mi>n</mi><mo>-</mo><mn>3</mn></mrow></msup><mo>&#xB7;</mo><mi>z</mi></math>"#;
    test_braille("Polish", expr, "⠠⠽⠬⠼⠃⠝⠈⠤⠼⠉⠱⠄⠵")?;
    return Ok(());
}

#[test]
fn scripts_p32_3() -> Result<()> {
    let expr = r#"<math><msub><mi>f</mi><mi>n</mi></msub><mo>(</mo><mi>x</mi><mo>)</mo></math>"#;
    test_braille("Polish", expr, "⠠⠋⠡⠝⠱⠣⠭⠜")?;
    return Ok(());
}

#[test]
fn scripts_p32_4() -> Result<()> {
    let expr = r#"<math><msub><mi>g</mi><mrow><mi>i</mi><mi>j</mi></mrow></msub><mo>(</mo><mi>y</mi><mo>)</mo></math>"#;
    test_braille("Polish", expr, "⠠⠛⠡⠊⠚⠱⠣⠽⠜")?;
    return Ok(());
}

#[test]
fn scripts_p32_5() -> Result<()> {
    let expr = r#"<math><msup><mrow><mo>(</mo><msub><mi>a</mi><mi>n</mi></msub><mo>)</mo></mrow><mi>k</mi></msup></math>"#;
    test_braille("Polish", expr, "⠠⠁⠡⠝⠬⠅")?;
    return Ok(());
}

#[test]
fn scripts_p32_6() -> Result<()> {
    let expr = r#"<math><msup><mrow><mo>(</mo><msub><mi>P</mi><mrow><mn>2</mn><mi>n</mi><mo>-</mo><mn>1</mn></mrow></msub><mo>)</mo></mrow><mi>m</mi></msup></math>"#;
    test_braille("Polish", expr, "⠨⠏⠡⠼⠃⠠⠝⠈⠤⠼⠁⠬⠍")?;
    return Ok(());
}

#[test]
fn scripts_p32_7() -> Result<()> {
    let expr = r#"<math><mfrac><msup><mi>x</mi><mi>n</mi></msup><mrow><mi>n</mi><mo>!</mo></mrow></mfrac></math>"#;
    test_braille("Polish", expr, "⠠⠭⠬⠝⠳⠝⠫")?;
    return Ok(());
}

#[test]
fn scripts_p32_8() -> Result<()> {
    let expr = r#"<math><mfrac><mn>1</mn><mrow><mn>4</mn><mo>+</mo><msub><mi>x</mi><mrow><mi>n</mi><mo>+</mo><mn>1</mn></mrow></msub></mrow></mfrac></math>"#;
    // removed fraction terminator as it is not needed
    test_braille("Polish", expr, "⠼⠁⠳⠼⠙⠈⠖⠠⠭⠡⠝⠈⠖⠼⠁")?;
    return Ok(());
}

#[test]
fn scripts_p32_9() -> Result<()> {
    let expr = r#"<math><msup><mrow><mo>(</mo><mn>2</mn><mo>+</mo><msub><mi>x</mi><mrow><mi>n</mi><mo>+</mo><mn>5</mn></mrow></msub><mo>)</mo></mrow><mn>2</mn></msup></math>"#;
    test_braille("Polish", expr, "⠣⠼⠃⠀⠖⠠⠭⠡⠝⠈⠖⠼⠑⠜⠬⠆")?;
    return Ok(());
}

#[test]
fn scripts_p32_10() -> Result<()> {
    let expr = r#"<math><msub><mi>f</mi><msub><mi>n</mi><mi>k</mi></msub></msub><mo>(</mo><mi>x</mi><mo>)</mo></math>"#;
    //  Dot 6 can be omitted 'n' and 'k' and were removed
    test_braille("Polish", expr, "⠠⠋⠐⠡⠝⠡⠅⠐⠱⠣⠭⠜")?;
    return Ok(());
}

#[test]
fn scripts_p33_1() -> Result<()> {
    let expr = r#"<math><msub><mi>P</mi><mn>1</mn></msub><mo>,</mo><mo>&#xA0;</mo><msub><mi>P</mi><mn>4</mn></msub><mo>,</mo><mo>&#xA0;</mo><msub><mi>P</mi><mn>9</mn></msub><mo>,</mo><mo>&#xA0;</mo><mo>.</mo><mo>.</mo><mo>.</mo><mo>,</mo><mo>&#xA0;</mo><msub><mi>P</mi><msup><mi>n</mi><mn>2</mn></msup></msub></math>"#;
    //  Dot 6 removed after ellipsis as it appears to be used for the line break
    test_braille("Polish", expr, r"⠨⠏⠡⠂⠠⠂⠀⠨⠏⠡⠲⠠⠂⠀⠨⠏⠡⠔⠠⠂⠀⠄⠄⠄⠂⠀⠨⠏⠐⠡⠠⠝⠬⠆⠐⠱")?;
    return Ok(());
}

#[test]
fn scripts_p33_2() -> Result<()> {
    let expr = r#"<math><msub><mi>x</mi><msub><mi>n</mi><mn>1</mn></msub></msub><mo>,</mo><mo>&#xA0;</mo><msub><mi>x</mi><msub><mi>n</mi><mn>2</mn></msub></msub><mo>,</mo><mo>&#xA0;</mo><msub><mi>x</mi><msub><mi>n</mi><mn>4</mn></msub></msub><mo>,</mo><mo>&#xA0;</mo><mo>.</mo><mo>.</mo><mo>.</mo><mo>,</mo><mo>&#xA0;</mo><msub><mi>x</mi><msub><mi>n</mi><msup><mn>2</mn><mi>k</mi></msup></msub></msub></math>"#;
    test_braille("Polish", expr, "⠠⠭⠐⠡⠝⠡⠂⠐⠱⠠⠂⠀⠭⠐⠡⠝⠡⠆⠐⠱⠠⠂⠀⠭⠐⠡⠝⠡⠲⠐⠱⠠⠂⠀⠄⠄⠄⠂⠀⠭⠨⠡⠝⠐⠡⠼⠃⠬⠅⠨⠱")?;
    return Ok(());
}

#[test]
fn scripts_p33_3() -> Result<()> {
    let expr = r#"<math><msub><mi>A</mi><mn>1</mn></msub><mo>=</mo><mi>a</mi><mo>,</mo><mo>&#xA0;</mo>
                            <msub><mi>A</mi><mn>2</mn></msub><mo>=</mo><msup><mi>a</mi><mn>7</mn></msup><mo>,</mo><mo>&#xA0;</mo>
                            <msub><mi>A</mi><mn>3</mn></msub><mo>=</mo><msup><mi>a</mi><mn>31</mn></msup><mo>,</mo><mo>&#xA0;</mo>
                            <mo>.</mo><mo>.</mo><mo>.</mo><mo>,</mo><mo>&#xA0;</mo>
                            <msub><mi>A</mi><mi>k</mi></msub><mo>=</mo><msup><mi>a</mi><mrow><msup><mn>2</mn><mrow><mn>2</mn><mi>k</mi><mo>-</mo><mn>1</mn></mrow></msup><mo>-</mo><mn>1</mn></mrow></msup></math>"#;
    test_braille("Polish", expr, "⠨⠁⠡⠂⠀⠶⠠⠁⠂⠀⠨⠁⠡⠆⠀⠶⠠⠁⠬⠶⠠⠂⠀⠨⠁⠡⠒⠀⠶⠠⠁⠬⠒⠂⠠⠂⠀⠄⠄⠄⠂⠀⠨⠁⠡⠠⠅⠀⠶⠠⠁⠐⠬⠼⠃⠬⠼⠃⠅⠈⠤⠼⠁⠱⠈⠤⠼⠁⠐⠱")?;
    return Ok(());
}

#[test]
fn scripts_p33_4() -> Result<()> {
    let expr = r#"<math><mfrac><msup><mi>e</mi><mstyle displaystyle="true"><mfrac><msup><mi>x</mi><mn>2</mn></msup><mn>2</mn></mfrac></mstyle></msup><msqrt><mn>2</mn><mi>&#x3C0;</mi></msqrt></mfrac></math>"#;
    test_braille("Polish", expr, "⠆⠠⠑⠐⠬⠈⠆⠭⠬⠆⠳⠆⠐⠱⠀⠳⠀⠩⠼⠃⠰⠏⠰")?;
    return Ok(());
}

#[test]
fn scripts_p33_5() -> Result<()> {
    let expr = r#"<math><msup><mi>x</mi><msub><mi>n</mi><mn>1</mn></msub></msup><mo>,</mo><mo>&#xA0;</mo><msup><mi>x</mi><msub><mi>n</mi><mn>2</mn></msub></msup><mo>,</mo><mo>&#xA0;</mo><msup><mi>x</mi><msub><mi>n</mi><mn>4</mn></msub></msup><mo>,</mo><mo>&#xA0;</mo><mo>.</mo><mo>.</mo><mo>.</mo><mo>,</mo><mo>&#xA0;</mo><msup><mi>x</mi><msub><mi>n</mi><msup><mn>2</mn><mi>k</mi></msup></msub></msup></math>"#;
    test_braille("Polish", expr, "⠠⠭⠐⠬⠝⠡⠂⠠⠂⠀⠭⠐⠝⠡⠆⠠⠂⠀⠭⠐⠬⠝⠡⠲⠠⠂⠠⠄⠄⠄⠂⠀⠠⠭⠨⠬⠝⠐⠡⠼⠃⠬⠅⠨⠱")?;
    return Ok(());
}

#[test]
fn scripts_p34_1() -> Result<()> {
    let expr = r#"<math><msup><mrow><mo>(</mo><msubsup><mi>x</mi><mi>n</mi><mi>i</mi></msubsup><mo>)</mo></mrow><mi>r</mi></msup></math>"#;
    test_braille("Polish", expr, "⠠⠭⠡⠝⠌⠊⠬⠗")?;
    return Ok(());
}

#[test]
fn scripts_p34_2() -> Result<()> {
    let expr = r#"<math><msup><mrow><mo>(</mo><msub><mi>x</mi><msup><mi>n</mi><mi>i</mi></msup></msub><mo>)</mo></mrow><mi>r</mi></msup></math>"#;
    test_braille("Polish", expr, "⠄⠭⠐⠡⠝⠌⠊⠐⠱⠬⠗")?;
    return Ok(());
}

#[test]
fn scripts_p34_3() -> Result<()> {
    let expr = r#"<math><msub><mi>x</mi><msub><mi>n</mi><msup><mi>j</mi><mi>r</mi></msup></msub></msub></math>"#;
    test_braille("Polish", expr, "⠠⠭⠨⠡⠝⠐⠡⠊⠬⠗⠨⠱")?;
    return Ok(());
}

#[test]
fn scripts_p34_4() -> Result<()> {
    let expr = r#"<math><msup><mrow><mo>(</mo><msubsup><mi>P</mi><msub><mi>a</mi><mi>j</mi></msub><msub><mi>a</mi><mi>k</mi></msub></msubsup><mo>)</mo></mrow><mi>n</mi></msup></math>"#;
    test_braille("Polish", expr, "⠨⠏⠐⠡⠠⠁⠡⠊⠐⠌⠁⠡⠅⠐⠱⠬⠝")?;
    return Ok(());
}

// Tak zwane „znaczki” (page 34)

#[test]
fn scripts_p35_1() -> Result<()> {
    let expr = r#"<math><msup><mi>ℝ</mi><mo>+</mo></msup></math>"#;
    test_braille("Polish", expr, "⠨⠨⠗⠖")?;
    return Ok(());
}

#[test]
fn scripts_p35_2() -> Result<()> {
    let expr = r#"<math><mi>A</mi><mo>''</mo></math>"#;
    test_braille("Polish", expr, "⠨⠁⠔⠔")?;
    return Ok(());
}

#[test]
fn scripts_p35_4() -> Result<()> {
    let expr = r#"<math><mover><mi>C</mi><mo>^</mo></mover></math>"#;
    test_braille("Polish", expr, "⠨⠉⠬")?;
    return Ok(());
}

#[test]
fn scripts_p35_5() -> Result<()> {
    let expr = r#"<math><mover><mi>A</mi><mo>~</mo></mover></math>"#;
    test_braille("Polish", expr, "⠨⠁⠢")?;
    return Ok(());
}

#[test]
fn exponents__p35_6() -> Result<()> {
    let expr = r#"<math><msup><mi>b</mi><mo>&#x2192;</mo></msup></math>"#;
    test_braille("Polish", expr, "⠠⠃⠒⠂")?;
    return Ok(());
}

#[test]
fn scripts_p35_7() -> Result<()> {
    let expr = r#"<math><mi>A</mi><msup><mi>B</mi><mo>&#x2192;</mo></msup></math>"#;
    test_braille("Polish", expr, "⠨⠁⠃⠒⠂")?;
    return Ok(());
}

#[test]
fn scripts_p35_8() -> Result<()> {
    let expr = r#"<math><msub><mover><mi>x</mi><mo>&#x2D9;</mo></mover><mrow><mi>n</mi><mo>+</mo><mn>1</mn></mrow></msub></math>"#;
    test_braille("Polish", expr, "⠠⠭⠆⠡⠝⠈⠖⠼⠁")?;
    return Ok(());
}

#[test]
fn scripts_p35_9() -> Result<()> {
    let expr = r#"<math><msubsup><mi>a</mi><mi>n</mi><mrow><mo>'</mo><mo>'</mo></mrow></msubsup></math>"#;
    test_braille("Polish", expr, "⠠⠁⠔⠔⠡⠝")?;
    return Ok(());
}

#[test]
fn scripts_p35_10() -> Result<()> {
    let expr = r#"<math><msup><mover><mi>v</mi><mo>&#xAF;</mo></mover><mn>2</mn></msup></math>"#;
    test_braille("Polish", expr, "⠠⠧⠒⠬⠆")?;
    return Ok(());
}

#[test]
fn scripts_p35_11() -> Result<()> {
    let expr = r#"<math><msubsup><mi>y</mi><mn>1</mn><mo>'</mo></msubsup></math>"#;
    test_braille("Polish", expr, "⠠⠽⠡⠂⠱⠔")?;
    return Ok(());
}

#[test]
fn scripts_p35_12() -> Result<()> {
    let expr = r#"<math><msubsup><mi>x</mi><mrow><mi>n</mi><mo>+</mo><mn>1</mn></mrow><msup><mrow/><mo>&#x2032;&#x2032;</mo></msup></msubsup></math>"#;
    test_braille("Polish", expr, "⠠⠭⠡⠝⠈⠖⠼⠁⠱⠔⠔")?;
    return Ok(());
}

#[test]
fn scripts_p36_1() -> Result<()> {
    let expr = r#"<math><mover accent="true"><mi>AB</mi><mo>&#x2192;</mo></mover></math>"#;
    test_braille("Polish", expr, "⠨⠒⠂⠨⠁⠃")?;
    return Ok(());
}

#[test]
fn scripts_p36_2() -> Result<()> {
    let expr = r#"<math><mover accent="true"><mi>CD</mi><mo accent="false">&#xAF;</mo></mover></math>"#;
    test_braille("Polish", expr, "⠨⠒⠨⠉⠙")?;
    return Ok(());
}

#[test]
fn scripts_p36_4() -> Result<()> {
    let expr = r#"<math><munder><mrow><msub><mi>A</mi><mn>1</mn></msub><msub><mi>B</mi><mn>1</mn></msub><msub><mi>C</mi><mn>1</mn></msub></mrow><mo>&#x23DD;</mo></munder></math>"#;
    test_braille("Polish", expr, "⠸⠣⠨⠁⠡⠂⠃⠡⠂⠉⠡⠂")?;
    return Ok(());
}

#[test]
fn prescripts_p36_1() -> Result<()> {
    let expr = r#"<math><mroot><mn>8</mn><mn>3</mn></mroot><mo>=</mo><mn>2</mn></math>"#;
    test_braille("Polish", expr, "⠌⠒⠩⠼⠓⠀⠶⠼⠃")?;
    return Ok(());
}

#[test]
fn prescripts_p36_3() -> Result<()> {
    let expr = r#"<math><mroot><mi>x</mi><mi>n</mi></mroot></math>"#;
    test_braille("Polish", expr, "⠌⠠⠝⠩⠭")?;
    return Ok(());
}

#[test]
fn prescripts_p36_4() -> Result<()> {
    let expr = r#"<math><mroot><mi>y</mi><mrow><mi>n</mi><mo>+</mo><mn>1</mn></mrow></mroot></math>"#;
    test_braille("Polish", expr, "⠌⠠⠝⠈⠖⠼⠁⠩⠽")?;
    return Ok(());
}

#[test]
fn prescripts_p36_2() -> Result<()> {
    let expr = r#"<math><mi>l</mi><mi>o</mi><msub><mi>g</mi><mn>2</mn></msub><mn>8</mn><mo>=</mo><mn>3</mn></math>"#;
    test_braille("Polish", expr, "⠌⠆⠫⠇⠼⠓⠀⠶⠼⠉")?;
    return Ok(());
}

// Roots (page 37)

#[test]
fn roots_p37_1() -> Result<()> {
    let expr = r#"<math><msqrt><mn>16</mn></msqrt></math>"#;
    test_braille("Polish", expr, "⠩⠼⠁⠋")?;
    return Ok(());
}

#[test]
fn roots_p37_2() -> Result<()> {
    let expr = r#"<math><msqrt><mn>81</mn></msqrt><mo>=</mo><mn>9</mn></math>"#;
    test_braille("Polish", expr, "⠩⠼⠓⠁⠀⠶⠼⠊")?;
    return Ok(());
}

#[test]
fn roots_p37_3() -> Result<()> {
    let expr = r#"<math><mroot><mn>27</mn><mn>3</mn></mroot><mo>=</mo><mn>3</mn></math>"#;
    test_braille("Polish", expr, "⠌⠒⠩⠼⠃⠛⠀⠶⠼⠉")?;
    return Ok(());
}

#[test]
fn roots_p37_4() -> Result<()> {
    let expr = r#"<math><msqrt><mi>x</mi><mo>+</mo><mfrac><mn>1</mn><mn>2</mn></mfrac></msqrt></math>"#;
    test_braille("Polish", expr, "⠩⠠⠭⠈⠖⠼⠁⠆")?;
    return Ok(());
}

#[test]
fn roots_p37_5() -> Result<()> {
    let expr = r#"<math><mn>3</mn><msqrt><mn>2</mn><mi>x</mi></msqrt></math>"#;
    test_braille("Polish", expr, "⠼⠉⠩⠼⠃⠠⠭")?;
    return Ok(());
}

#[test]
fn roots_p37_6() -> Result<()> {
    let expr = r#"<math><msqrt><mi>x</mi></msqrt><mo>+</mo><mi>y</mi></math>"#;
    test_braille("Polish", expr, "⠩⠠⠭⠀⠖⠽")?;
    return Ok(());
}

#[test]
fn roots_p37_7() -> Result<()> {
    let expr = r#"<math><msqrt><mi>x</mi><mo>+</mo><mi>y</mi></msqrt><mo>+</mo><msqrt><mi>x</mi><mo>-</mo><mi>y</mi></msqrt></math>"#;
    test_braille("Polish", expr, "⠩⠠⠭⠈⠖⠽⠀⠖⠩⠭⠈⠤⠽")?;
    return Ok(());
}

#[test]
#[ignore = "uses whitespace instead of terminator due to multiplication dot"]
fn roots_p37_8() -> Result<()> {
    let expr = r#"<math><msqrt><mi>x</mi><mo>+</mo><mi>y</mi></msqrt><mo>&#xB7;</mo><msup><mi mathvariant="normal">e</mi><mi>z</mi></msup></math>"#;
    test_braille("Polish", expr, "⠩⠠⠭⠈⠖⠽⠱⠄⠑⠬⠵")?;
    return Ok(());
}

#[test]
#[ignore = "uses whitespace instead of terminator due to multiplication dot"]
fn roots_p37_9() -> Result<()> {
    let expr = r#"<math><msqrt><mi>x</mi><mo>+</mo><mi>x</mi></msqrt><mo>&#xB7;</mo><msqrt><mi>x</mi><mo>-</mo><mi>x</mi></msqrt></math>"#;
    test_braille("Polish", expr, "⠩⠠⠭⠈⠖⠽⠱⠄⠩⠭⠈⠤⠽")?;
    return Ok(());
}

#[test]
fn roots_p37_10() -> Result<()> {
    let expr = r#"<math><mn>3</mn><msqrt><mn>2</mn></msqrt><mi>x</mi></math>"#;
    test_braille("Polish", expr, "⠼⠉⠩⠼⠃⠱⠠⠭")?;
    return Ok(());
}

#[test]
fn roots_p37_11() -> Result<()> {
    let expr = r#"<math><mn>0</mn><mo>,</mo><mn>5</mn><msqrt><mi>x</mi></msqrt><mfrac><mrow><mi>x</mi><mo>+</mo><mi>y</mi></mrow><mrow><mi>x</mi><mo>-</mo><mi>y</mi></mrow></mfrac></math>"#;
    test_braille_prefs("Polish", vec![("Polish_BrailleLevel", "Beginner")], expr, r"⠼⠚⠂⠑⠩⠠⠭⠱⠆⠭⠀⠖⠽⠀⠳⠀⠭⠀⠤⠽⠰")?;
    return Ok(());
}

#[test]
fn roots_p37_12() -> Result<()> {
    let expr = r#"<math><msqrt><mi>a</mi><mi>b</mi></msqrt><mo>=</mo><msqrt><mi>a</mi></msqrt><msqrt><mi>b</mi></msqrt></math>"#;
    test_braille("Polish", expr, "⠩⠠⠁⠃⠀⠶⠩⠁⠩⠃")?;
    return Ok(());
}

#[test]
fn roots_p37_13() -> Result<()> {
    let expr = r#"<math><mi>a</mi><msqrt><mn>2</mn><mi>a</mi></msqrt><msqrt><mn>3</mn><mi>b</mi></msqrt></math>"#;
    test_braille("Polish", expr, "⠠⠁⠩⠼⠃⠠⠁⠩⠼⠉⠠⠃")?;
    return Ok(());
}

#[test]
fn roots_p38_1() -> Result<()> {
    let expr = r#"<math><mo>(</mo><mi>a</mi><mo>+</mo><msqrt><mi>ab</mi></msqrt><msup><mo>)</mo><mn>2</mn></msup><mo>-</mo><mi>b</mi></math>"#;
    test_braille("Polish", expr, "⠣⠠⠁⠀⠖⠩⠁⠃⠜⠬⠆⠀⠤⠃")?;
    return Ok(());
}

#[test]
#[ignore = "doesn't use drop numbers for denominator"]
fn roots_p38_2() -> Result<()> {
    let expr = r#"<math><mfrac><msqrt><mn>2</mn></msqrt><mn>2</mn></mfrac><mo>&#x2248;</mo><mn>0</mn><mo>,</mo><mn>7071</mn></math>"#;
    test_braille("Polish", expr, "⠩⠼⠃⠳⠼⠃⠀⠢⠢⠼⠚⠂⠛⠚⠛⠁")?;
    return Ok(());
}

#[test]
fn roots_p38_3() -> Result<()> {
    let expr = r#"<math><msqrt><mn>2</mn><msqrt><mn>2</mn></msqrt></msqrt><mo>=</mo><msqrt><mn>2</mn></msqrt><mo>&#xB7;</mo><mroot><mn>2</mn><mn>4</mn></mroot></math>"#;
    test_braille("Polish", expr, "⠐⠩⠼⠃⠩⠼⠃⠀⠶⠩⠼⠃⠀⠄⠌⠲⠩⠼⠃")?;
    return Ok(());
}

#[test]
fn roots_p38_4() -> Result<()> {
    let expr = r#"<math><msqrt><mfrac><mi>a</mi><mi>b</mi></mfrac></msqrt><mo>=</mo><mfrac><msqrt><mi>a</mi></msqrt><msqrt><mi>b</mi></msqrt></mfrac></math>"#;
    test_braille("Polish", expr, "⠐⠩⠠⠁⠳⠃⠀⠶⠩⠁⠳⠩⠃")?;
    return Ok(());
}

#[test]
fn roots_p38_5() -> Result<()> {
    let expr = r#"<math><msqrt><mi>x</mi><mo>+</mo><mi>y</mi><mo>-</mo><mn>2</mn><msqrt><mi>x</mi><mi>y</mi></msqrt></msqrt></math>"#;
    test_braille("Polish", expr, "⠐⠩⠠⠭⠈⠖⠽⠈⠤⠼⠃⠩⠭⠽")?;
    return Ok(());
}

#[test]
fn roots_p38_6() -> Result<()> {
    let expr = r#"<math><msqrt><mn>2</mn><mi>x</mi><mo>+</mo><msqrt><mi>x</mi><mo>-</mo><mn>2</mn></msqrt><mo>-</mo><mn>3</mn></msqrt></math>"#;
    test_braille("Polish", expr, "⠐⠩⠼⠃⠠⠭⠈⠖⠩⠭⠈⠤⠼⠃⠱⠈⠤⠼⠉")?;
    return Ok(());
}

#[test]
// #[ignore="likely bug in spec with terminator before '+' (space) along with having a terminator at the end of the expr"]
fn roots_p39_1() -> Result<()> {
    let expr = r#"<math><msqrt><mn>2</mn><mi>x</mi><msqrt><mi>x</mi><mo>+</mo><msqrt><mn>2</mn><mo>-</mo><mi>x</mi></msqrt></msqrt><mo>+</mo><mn>5</mn></msqrt></math>"#;
    test_braille("Polish", expr, "⠨⠩⠼⠃⠠⠭⠐⠩⠭⠈⠖⠩⠼⠃⠈⠤⠭⠐⠱⠀⠖⠼⠑⠨⠱")?;
    return Ok(());
}

#[test]
// #[ignore="a terminator at the end of the expr"]
fn roots_p39_2() -> Result<()> {
    let expr = r#"<math><msqrt><mi>x</mi><mo>+</mo><mi>y</mi><mo>-</mo><mn>2</mn><msqrt><mi>x</mi><mo>+</mo><mi>y</mi></msqrt></msqrt></math>"#;
    test_braille("Polish", expr, "⠨⠩⠠⠭⠀⠖⠽⠀⠤⠼⠃⠩⠭⠈⠖⠽⠨⠱")?;
    return Ok(());
}

#[test]
// #[ignore="a terminator at the end of the expr"]
fn roots_p39_3() -> Result<()> {
    let expr = r#"<math><msqrt><mi>x</mi><mo>+</mo><mi>y</mi><mo>-</mo><mn>2</mn><msqrt><mi>x</mi><mo>+</mo><mi>y</mi></msqrt><mo>+</mo><msqrt><mi>x</mi></msqrt></msqrt></math>"#;
    test_braille("Polish", expr, "⠨⠩⠠⠭⠀⠖⠽⠀⠤⠼⠃⠩⠭⠈⠖⠽⠀⠖⠩⠭⠨⠱")?;
    return Ok(());
}

#[test]
// #[ignore="a terminator at the end of the expr"]
fn roots_p39_4() -> Result<()> {
    let expr = r#"<math><msqrt><msqrt><msqrt><mfrac><mn>1</mn><mn>8</mn></mfrac></msqrt><mo>-</mo><mfrac><mn>1</mn><mn>8</mn></mfrac></msqrt><mo>-</mo><mfrac><mn>1</mn><mn>8</mn></mfrac></msqrt></math>"#;
    test_braille("Polish", expr, "⠨⠩⠐⠩⠩⠼⠁⠦⠱⠈⠤⠼⠁⠦⠀⠤⠼⠁⠦⠨⠱")?;
    return Ok(());
}

#[test]
fn roots_p39_5() -> Result<()> {
    let expr = r#"<math><mo>(</mo><mi>a</mi><mo>+</mo><mi>b</mi><msup><mo>)</mo><mn>2</mn></msup><mo>=</mo><msup><mi>a</mi><mn>2</mn></msup><mo>+</mo><mn>2</mn><mi>a</mi><mi>b</mi><mo>+</mo><msup><mi>b</mi><mn>2</mn></msup></math>"#;
    test_braille("Polish", expr, "⠣⠠⠁⠀⠖⠃⠜⠬⠆⠀⠶⠁⠬⠆⠀⠖⠼⠃⠠⠁⠃⠀⠖⠃⠬⠆")?;
    return Ok(());
}

#[test]
#[ignore = "uses superscript not exponent -- no way to know why"]
fn roots_p39_6() -> Result<()> {
    let expr = r#"<math><msup><mi>C</mi><mi>k</mi></msup></math>"#;
    test_braille("Polish", expr, "⠨⠉⠌⠠⠅")?;
    return Ok(());
}

#[test]
fn roots_p39_7() -> Result<()> {
    let expr = r#"<math><msubsup><mi>V</mi><mi>n</mi><mi>k</mi></msubsup></math>"#;
    test_braille("Polish", expr, "⠨⠧⠌⠠⠅⠡⠝")?;
    return Ok(());
}

#[test]
fn roots_p39_8() -> Result<()> {
    let expr = r#"<math><mfrac><msup><mi>a</mi><mi>x</mi></msup><mi>y</mi></mfrac></math>"#;
    test_braille("Polish", expr, "⠠⠁⠬⠭⠳⠽")?;
    return Ok(());
}

#[test]
fn roots_p39_9() -> Result<()> {
    let expr = r#"<math><msup><mi>a</mi><mfrac><mi>x</mi><mi>y</mi></mfrac></msup></math>"#;
    test_braille("Polish", expr, "⠠⠁⠐⠬⠭⠳⠽")?;
    return Ok(());
}

#[test]
fn roots_p39_10() -> Result<()> {
    // example shows "," at end, but braille doesn't, so I have removed it
    let expr = r#"<math><msub><mi>x</mi><mn>2</mn></msub><mo>=</mo><mfrac><mrow><mo>-</mo><mi>b</mi><mo>+</mo><msqrt><msup><mi>b</mi><mn>2</mn></msup><mo>-</mo><mn>4</mn><mi>a</mi><mi>c</mi></msqrt></mrow><mrow><mn>2</mn><mi>a</mi></mrow></mfrac></math>"#;
    test_braille_prefs("Polish", vec![("Polish_BrailleLevel", "Advanced")], expr, r"⠠⠭⠡⠆⠀⠶⠆⠤⠃⠀⠖⠐⠩⠃⠬⠆⠈⠤⠼⠙⠠⠁⠉⠀⠳⠀⠼⠃⠠⠁⠰")?;
    return Ok(());
}

// Functions (page 40)

#[test]
fn functions_p40_1() -> Result<()> {
    let expr = r#"<math><mi>y</mi><mo>=</mo><mi>f</mi><mo>(</mo><mi>x</mi><mo>)</mo></math>"#;
    test_braille("Polish", expr, "⠠⠽⠀⠶⠋⠣⠭⠜")?;
    return Ok(());
}

#[test]
fn functions_p40_2() -> Result<()> {
    let expr = r#"<math><mi>f</mi><mo>(</mo><mi>x</mi><mo>)</mo><mo>=</mo><mn>2</mn><mi>x</mi><mo>-</mo><mn>1</mn></math>"#;
    test_braille("Polish", expr, "⠠⠋⠣⠭⠜⠀⠶⠼⠃⠭⠀⠤⠼⠁")?;
    return Ok(());
}

#[test]
fn functions_p40_3() -> Result<()> {
    let expr = r#"<math><mi>y</mi><mo>=</mo><mn>2</mn><mi>x</mi><mo>&#x2212;</mo><mn>1</mn></math>"#;
    test_braille("Polish", expr, "⠠⠽⠀⠶⠼⠃⠭⠀⠤⠼⠁")?;
    return Ok(());
}

#[test]
fn functions_p40_4() -> Result<()> {
    let expr = r#"<math><mi>x</mi><mo accent="false" stretchy="false">→</mo><mi>y</mi><mo>=</mo><mn>2</mn><mi>x</mi><mo>−</mo><mn>1</mn></math>"#;
    test_braille("Polish", expr, "⠠⠭⠀⠒⠂⠽⠀⠶⠼⠃⠭⠀⠤⠼⠁")?;
    return Ok(());
}

#[test]
fn functions_p40_5() -> Result<()> {
    let expr = r#"<math><mi>x</mi><mo>&#x2192;</mo><mn>2</mn><mi>x</mi><mo>-</mo><mn>1</mn></math>"#;
    test_braille("Polish", expr, "⠠⠭⠀⠒⠂⠼⠃⠭⠀⠤⠼⠁")?;
    return Ok(());
}

#[test]
fn functions_p40_6() -> Result<()> {
    let expr = r#"<math><mi>f</mi><mo>(</mo><mi>x</mi><mo>)</mo><mo>=</mo><mo>-</mo><mn>2</mn><msup><mi>x</mi><mn>2</mn></msup><mo>+</mo><mn>4</mn><mi>x</mi><mo>+</mo><mn>6</mn><mo>=</mo><mo>-</mo><mn>2</mn><mo>(</mo><mi>x</mi><mo>-</mo><mn>1</mn><msup><mo>)</mo><mn>2</mn></msup><mo>+</mo><mn>8</mn></math>"#;
    test_braille("Polish", expr, "⠠⠋⠣⠭⠜⠀⠶⠤⠼⠃⠭⠬⠆⠀⠖⠼⠙⠭⠀⠖⠼⠋⠀⠶⠤⠼⠃⠣⠭⠀⠤⠼⠁⠜⠬⠆⠀⠖⠼⠓")?;
    return Ok(());
}

#[test]
fn functions_p40_7() -> Result<()> {
    let expr = r#"<math><mi>W</mi><mo>=</mo><mo>(</mo><mn>1</mn><mo>,</mo><mn>8</mn><mo>)</mo></math>"#;
    test_braille("Polish", expr, "⠨⠺⠀⠶⠣⠼⠁⠠⠂⠀⠼⠓⠜")?;
    return Ok(());
}

#[test]
fn functions_p40_8() -> Result<()> {
    let expr = r#"<math><msup><mi>f</mi><mrow><mo>-</mo><mn>1</mn></mrow></msup></math>"#;
    test_braille("Polish", expr, "⠠⠋⠌⠤⠂")?;
    return Ok(());
}

#[test]
fn functions_p40_9() -> Result<()> {
    let expr = r#"<math><mi>f</mi><mo>(</mo><mi>x</mi><mo>)</mo><mo>=</mo><mi>sin</mi><mi>x</mi></math>"#;
    test_braille("Polish", expr, "⠠⠋⠣⠭⠜⠀⠶⠫⠎⠭")?;
    return Ok(());
}

#[test]
fn functions_p40_10() -> Result<()> {
    let expr = r#"<math><msup><mi>f</mi><mrow><mo>-</mo><mn>1</mn></mrow></msup><mo>(</mo><mi>x</mi><mo>)</mo><mo>=</mo><mi>arcsin</mi><mi>x</mi></math>"#;
    test_braille("Polish", expr, "⠠⠋⠌⠤⠂⠣⠭⠜⠀⠶⠫⠂⠎⠭")?;
    return Ok(());
}

#[test]
fn functions_p40_11() -> Result<()> {
    let expr = r#"<math><mi>y</mi><mo>=</mo><mfrac><mn>3</mn><mn>4</mn></mfrac><mi>x</mi><mo>-</mo><mn>1</mn></math>"#;
    test_braille("Polish", expr, "⠠⠽⠀⠶⠼⠉⠲⠭⠀⠤⠼⠁")?;
    return Ok(());
}

#[test]
fn functions_p40_12() -> Result<()> {
    let expr = r#"<math><msup><mi>y</mi><mrow><mo>-</mo><mn>1</mn></mrow></msup><mo>=</mo><mfrac><mn>4</mn><mn>3</mn></mfrac><mi>x</mi><mo>+</mo><mfrac><mn>4</mn><mn>3</mn></mfrac></math>"#;
    test_braille("Polish", expr, "⠠⠽⠌⠤⠂⠀⠶⠼⠙⠒⠭⠀⠖⠼⠙⠒")?;
    return Ok(());
}

// Complex functions p. 41

#[test]
fn functions_p41_1() -> Result<()> {
    let expr = r#"<math><mi>f</mi><mo>(</mo><mi>g</mi><mo>(</mo><mi>x</mi><mo>)</mo><mo>)</mo></math>"#;
    test_braille("Polish", expr, "⠠⠋⠣⠛⠣⠭⠜⠜")?;
    return Ok(());
}

#[test]
fn functions_p41_2() -> Result<()> {
    let expr = r#"<math><mi>u</mi><mo>=</mo><mi>g</mi><mo>(</mo><mi>x</mi><mo>)</mo><mo>=</mo><mn>3</mn><mi>x</mi><mo>+</mo><mn>2</mn></math>"#;
    test_braille("Polish", expr, "⠠⠥⠀⠶⠛⠣⠭⠜⠀⠶⠼⠉⠭⠀⠖⠼⠃")?;
    return Ok(());
}

#[test]
fn functions_p41_3() -> Result<()> {
    let expr = r#"<math><mi>f</mi><mo>(</mo><mi>u</mi><mo>)</mo><mo>=</mo><mfrac><msqrt><mi>u</mi></msqrt><mi>u</mi></mfrac></math>"#;
    test_braille("Polish", expr, "⠠⠋⠣⠥⠜⠀⠶⠩⠥⠳⠥")?;
    return Ok(());
}

#[test]
fn functions_p41_4() -> Result<()> {
    let expr = r#"<math><mi>f</mi><mo>(</mo><mi>g</mi><mo>(</mo><mi>x</mi><mo>)</mo><mo>)</mo><mo>=</mo><mfrac><msqrt><mn>3</mn><mi>x</mi><mo>+</mo><mn>2</mn></msqrt><mrow><mn>3</mn><mi>x</mi><mo>+</mo><mn>2</mn></mrow></mfrac></math>"#;
    test_braille("Polish", expr, "⠠⠋⠣⠛⠣⠭⠜⠜⠀⠶⠩⠼⠉⠭⠈⠖⠼⠃⠳⠼⠉⠭⠈⠖⠼⠃")?;
    return Ok(());
}

#[test]
fn functions_p41_5_const() -> Result<()> {
    let expr = r#"<math><mi>f</mi><mo>(</mo><mi>x</mi><mo>)</mo><mo>=</mo><mi>c</mi><mi>o</mi><mi>n</mi><mi>s</mi><mi>t</mi></math>"#;
    test_braille("Polish", expr, "⠠⠋⠣⠭⠜⠀⠶⠼⠅")?;
    return Ok(());
}

#[test]
fn functions_p41_6_sgn() -> Result<()> {
    let expr = r#"<math><mi>sgn</mi><mo>&#xA0;</mo><mn>5</mn><mo>=</mo><mn>1</mn></math>"#;
    test_braille("Polish", expr, "⠼⠎⠼⠑⠀⠶⠼⠁")?;
    return Ok(());
}

#[test]
fn functions_p41_7_sgn() -> Result<()> {
    let expr = r#"<math><mi>sgn</mi><mo>&#xA0;</mo><mo>(</mo><mo>-</mo><mn>27</mn><mo>)</mo><mo>=</mo><mo>-</mo><mn>1</mn></math>"#;
    test_braille("Polish", expr, "⠼⠎⠣⠤⠼⠃⠛⠜⠀⠶⠤⠼⠁")?;
    return Ok(());
}

#[test]
fn functions_p41_8() -> Result<()> {
    let expr = r#"<math><msub><mi>f</mi><mrow><mi>n</mi><mo>+</mo><mn>1</mn></mrow></msub><mo>(</mo><mi>x</mi><mo>)</mo><mo>=</mo><mo>(</mo><mi>n</mi><mo>+</mo><mn>1</mn><mo>)</mo><mi>x</mi><mo>+</mo><mn>1</mn></math>"#;
    test_braille("Polish", expr, "⠠⠋⠡⠝⠈⠖⠼⠁⠱⠣⠭⠜⠀⠶⠣⠝⠀⠖⠼⠁⠜⠭⠀⠖⠼⠁")?;
    return Ok(());
}

#[test]
#[ignore = "MathCAT generates alternative encoding (test functions_p41_9_2)"]
fn functions_p41_9_1() -> Result<()> {
    let expr = r#"<math><msub><mi>F</mi><msub><mi mathvariant="normal">n</mi><mi>k</mi></msub></msub><mo>(</mo><mi>x</mi><mo>)</mo></math>"#;
    test_braille("Polish", expr, "⠨⠋⠐⠡⠠⠝⠡⠅⠀⠣⠭⠜")?;
    return Ok(());
}

#[test]
fn functions_p41_9_2() -> Result<()> {
    let expr = r#"<math><msub><mi>F</mi><msub><mi mathvariant="normal">n</mi><mi>k</mi></msub></msub><mo>(</mo><mi>x</mi><mo>)</mo></math>"#;
    test_braille("Polish", expr, "⠨⠋⠐⠡⠠⠝⠡⠅⠐⠱⠣⠭⠜")?;
    return Ok(());
}

// LARGE CLAMP CONNECTING SEVERAL ROWS p. 43


#[test]
fn functions_p43_3() -> Result<()> {
    let expr = r#"<math><mfenced><mtable intent="binomial-coefficient($upper, $lower)"><mtr><mtd><mi arg="upper">n</mi></mtd></mtr><mtr><mtd><mi arg="lower">k</mi></mtd></mtr></mtable></mfenced></math>"#;
    test_braille("Polish", expr, "⠣⠠⠝⠰⠳⠅⠜")?;
    return Ok(());
}

#[test]
fn functions_p43_4() -> Result<()> {
    let expr = r#"<math><msubsup><mover><mi>C</mi><mo>&#xAF;</mo></mover><mi>n</mi><mi>k</mi></msubsup><mo>=</mo><mfenced><mtable intent="binomial-coefficient($upper, $lower)"><mtr><mtd arg="upper"><mi>n</mi><mo>+</mo><mi>k</mi><mo>-</mo><mn>1</mn></mtd></mtr><mtr><mtd arg="lower"><mi>k</mi></mtd></mtr></mtable></mfenced></math>"#;
    test_braille("Polish", expr, "⠨⠉⠒⠌⠠⠅⠡⠝⠀⠶⠣⠝⠈⠖⠅⠈⠤⠼⠁⠰⠳⠅⠜")?;
    return Ok(());
}

// EQUATIONS AND SYSTEMS OF EQUATIONS p. 44-46
// Matrices and other multi-line expressions

#[test]
fn cases_p43_1() -> Result<()> {
    let expr = r#"<math><mo>|</mo><mi>x</mi><mo>|</mo><mo>=</mo><mfenced open="{" close="" separators="|"><mtable columnspacing="1em" columnalign="left left" intent=":piecewise"><mtr><mtd><mo>&#x2212;</mo><mi>x</mi><mo>&#xA0;</mo><mi>dla</mi><mo>&#xA0;</mo><mi>x</mi><mo>&lt;</mo><mn>0</mn><mo>,</mo></mtd></mtr><mtr><mtd><mspace width="1em"/><mn>0</mn><mo>&#xA0;</mo><mi>dla</mi><mo>&#xA0;</mo><mi>x</mi><mo>=</mo><mn>0</mn><mo>,</mo></mtd></mtr><mtr><mtd><mspace width="1em"/><mi>x</mi><mo>&#xA0;</mo><mi>dla</mi><mo>&#xA0;</mo><mi>x</mi><mo>&gt;</mo><mn>0.</mn><mo>&#x2005;&#x2005;&#x2005;&#x2005;</mo></mtd></mtr></mtable></mfenced></math>"#;
    test_braille("Polish", expr, "⠈⠇⠠⠭⠸⠀⠶⠪⠀⠤⠭⠀⠙⠇⠁⠀⠠⠭⠀⠪⠄⠼⠚⠠⠂⠀⠰⠳⠀⠼⠚⠀⠙⠇⠁⠀⠠⠭⠀⠶⠼⠚⠠⠂⠀⠰⠳⠀⠭⠀⠙⠇⠁⠀⠠⠭⠀⠕⠂⠼⠚⠄")?;
    return Ok(());
}

#[test]
fn cases_p43_2() -> Result<()> {
    let expr = r#"<math><mi>sgn</mi><mi>x</mi><mo>=</mo><mfenced open="{" close="" separators="|"><mtable columnspacing="1em" columnalign="left left" intent=":piecewise"><mtr><mtd><mo>&#x2212;</mo><mn>1</mn><mo>&#xA0;</mo><mi>dla</mi><mo>&#xA0;</mo><mi>x</mi><mo>&lt;</mo><mn>0</mn><mo>,</mo></mtd></mtr><mtr><mtd><mn>0</mn><mo>&#xA0;</mo><mi>dla</mi><mo>&#xA0;</mo><mi>x</mi><mo>=</mo><mn>0</mn><mo>,</mo></mtd></mtr><mtr><mtd><mn>1</mn><mo>&#xA0;</mo><mi>dla</mi><mo>&#xA0;</mo><mi>x</mi><mo>&gt;</mo><mn>0.</mn><mo>&#x2005;&#x2005;&#x2005;&#x2005;</mo></mtd></mtr></mtable></mfenced></math>"#;
    test_braille("Polish", expr, "⠼⠎⠠⠭⠀⠶⠪⠀⠤⠼⠁⠀⠙⠇⠁⠀⠠⠭⠀⠪⠄⠼⠚⠠⠂⠀⠰⠳⠀⠼⠚⠀⠙⠇⠁⠀⠠⠭⠀⠶⠼⠚⠠⠂⠀⠰⠳⠀⠼⠁⠀⠙⠇⠁⠀⠠⠭⠀⠕⠂⠼⠚⠄")?;
    return Ok(());
}

#[test]
fn combination_p43_3() -> Result<()> {
    let expr = r#"<math><mfenced><mtable><mtr><mtd><mi>n</mi></mtd></mtr><mtr><mtd><mi>k</mi></mtd></mtr></mtable></mfenced></math>"#;
    test_braille("Polish", expr, "⠣⠠⠝⠰⠳⠅⠜")?;
    return Ok(());
}

#[test]
fn combination_p43_4() -> Result<()> {
    let expr = r#"<math><msubsup><mover><mi>C</mi><mo>&#xAF;</mo></mover><mi>n</mi><mi>k</mi></msubsup><mo>=</mo><mfenced><mtable><mtr><mtd><mi>n</mi><mo>+</mo><mi>k</mi><mo>-</mo><mn>1</mn></mtd></mtr><mtr><mtd><mi>k</mi></mtd></mtr></mtable></mfenced></math>"#;
    test_braille("Polish", expr, "⠨⠉⠒⠌⠠⠅⠡⠝⠀⠶⠣⠝⠈⠖⠅⠈⠤⠼⠁⠰⠳⠅⠜")?;
    return Ok(());
}


#[test]
fn combination_intent_p43_3() -> Result<()> {
    let expr = r#"<math><mfenced><mtable intent="binomial-coefficient($upper, $lower)"><mtr><mtd><mi arg="upper">n</mi></mtd></mtr><mtr><mtd><mi arg="lower">k</mi></mtd></mtr></mtable></mfenced></math>"#;
    test_braille("Polish", expr, "⠣⠠⠝⠰⠳⠅⠜")?;
    return Ok(());
}

#[test]
fn combination_intent_p43_4() -> Result<()> {
    let expr = r#"<math><msubsup><mover><mi>C</mi><mo>&#xAF;</mo></mover><mi>n</mi><mi>k</mi></msubsup><mo>=</mo><mfenced><mtable intent="binomial-coefficient($upper, $lower)"><mtr><mtd arg="upper"><mi>n</mi><mo>+</mo><mi>k</mi><mo>-</mo><mn>1</mn></mtd></mtr><mtr><mtd arg="lower"><mi>k</mi></mtd></mtr></mtable></mfenced></math>"#;
    test_braille("Polish", expr, "⠨⠉⠒⠌⠠⠅⠡⠝⠀⠶⠣⠝⠈⠖⠅⠈⠤⠼⠁⠰⠳⠅⠜")?;
    return Ok(());
}


#[test]
fn systems_of_equations_p44_1() -> Result<()> {
    let expr = r#"<math><mtable columnspacing="1em" columnalign="left left"><mtr><mtd><mo>&#x2005;&#x2005;&#x2005;&#x2005;</mo><mrow><mn>2</mn><mi>x</mi><mo>+</mo><mn>3</mn><mo>(</mo><mi>x</mi><mo>&#x2212;</mo><mn>2</mn><mo>)</mo><mo>=</mo><mi>x</mi><mo>&#x2212;</mo><mn>10</mn></mrow></mtd></mtr><mtr><mtd><mo>&#x2005;&#x2005;&#x2005;&#x2005;</mo><mn>2</mn><mi>x</mi><mo>+</mo><mn>3</mn><mi>x</mi><mo>&#x2212;</mo><mn>6</mn><mo>=</mo><mi>x</mi><mo>&#x2212;</mo><mn>10</mn></mtd></mtr><mtr><mtd><mo>&#x2005;&#x2005;&#x2005;&#x2005;</mo><mn>5</mn><mi>x</mi><mo>&#x2212;</mo><mn>6</mn><mo>=</mo><mi>x</mi><mo>&#x2212;</mo><mn>10</mn></mtd></mtr><mtr><mtd><mo>&#x2005;&#x2005;&#x2005;&#x2005;</mo><mn>5</mn><mi>x</mi><mo>&#x2212;</mo><mi>x</mi><mo>=</mo><mn>6</mn><mo>&#x2212;</mo><mn>10</mn></mtd></mtr><mtr><mtd><mo>&#x2005;&#x2005;&#x2005;&#x2005;</mo><mn>4</mn><mi>x</mi><mo>=</mo><mo>&#x2212;</mo><mn>4</mn><mo>&#x2223;:</mo><mn>4</mn></mtd></mtr><mtr><mtd><mo>&#x2005;&#x2005;&#x2005;&#x2005;</mo><mi>x</mi><mo>=</mo><mo>&#x2212;</mo><mn>1</mn></mtd></mtr></mtable></math>"#;
    test_braille("Polish", expr, "⠼⠃⠄⠭⠀⠖⠼⠉⠣⠭⠀⠤⠼⠃⠜⠀⠶⠭⠀⠤⠼⠁⠚⠀⠐⠶⠂⠼⠃⠠⠭⠀⠖⠼⠉⠭⠀⠤⠼⠋⠀⠶⠭⠀⠤⠼⠁⠚⠀⠐⠶⠂⠼⠑⠠⠭⠀⠤⠼⠋⠀⠶⠭⠀⠤⠼⠁⠚⠀⠐⠶⠂⠼⠑⠠⠭⠀⠤⠭⠀⠶⠼⠋⠀⠤⠼⠁⠚⠀⠐⠶⠂⠼⠙⠠⠭⠀⠶⠤⠼⠙⠀⠸⠀⠲⠼⠙⠀⠐⠶⠂⠠⠭⠀⠶⠤⠼⠁")?;
    return Ok(());
}

#[test]
fn systems_of_equations_p45_1() -> Result<()> {
    let expr = r#"<math><mfenced open="{" close="" separators="|"><mtable columnspacing="1em" columnalign="left left"><mtr><mtd><mi>x</mi><mo>+</mo><mn>5</mn><mi>y</mi><mo>=</mo><mn>3</mn><mo>+</mo><mn>3</mn><mi>y</mi></mtd></mtr><mtr><mtd><mo>&#x2212;</mo><mn>4</mn><mi>x</mi><mo>+</mo><mi>y</mi><mo>+</mo><mn>2</mn><mo>=</mo><mn>9</mn><mo>&#x2212;</mo><mn>6</mn><mi>x</mi><mo>+</mo><mn>2</mn><mi>y</mi><mo>&#x2005;&#x2005;&#x2005;&#x2005;</mo></mtd></mtr></mtable></mfenced><mspace linebreak="newline"/><mfenced open="{" close="" separators="|"><mtable columnspacing="1em" columnalign="left left"><mtr><mtd><mi>x</mi><mo>+</mo><mn>2</mn><mi>y</mi><mo>=</mo><mn>3</mn></mtd></mtr><mtr><mtd><mn>2</mn><mi>x</mi><mo>&#x2212;</mo><mi>y</mi><mo>=</mo><mn>7</mn><mo>&#x2005;&#x2005;&#x2005;&#x2005;</mo></mtd></mtr></mtable></mfenced></math>"#;
    test_braille("Polish", expr, "⠪⠀⠠⠭⠀⠖⠼⠑⠽⠀⠶⠼⠉⠀⠖⠼⠉⠽⠀⠰⠳⠀⠤⠼⠙⠭⠀⠖⠽⠀⠖⠼⠃⠀⠶⠼⠊⠀⠤⠼⠋⠭⠀⠖⠼⠃⠽⠪⠀⠠⠭⠀⠖⠼⠃⠽⠀⠶⠼⠉⠀⠰⠳⠀⠼⠃⠭⠀⠤⠽⠀⠶⠼⠛")?;
    return Ok(());
}

#[test]
fn matrices_p46_1() -> Result<()> {
    let expr = r#"<math><mi mathvariant="bold">W</mi><mo>=</mo><mfenced open="|" close="|"><mtable><mtr><mtd><mn>1</mn></mtd><mtd><mn>2</mn></mtd></mtr><mtr><mtd><mn>2</mn></mtd><mtd><mo>-</mo><mn>1</mn></mtd></mtr></mtable></mfenced><mo>=</mo><mn>1</mn><mo>&#xB7;</mo><mo>(</mo><mo>-</mo><mn>1</mn><mo>)</mo><mo>-</mo><mn>2</mn><mo>&#xB7;</mo><mn>2</mn><mo>=</mo><mo>-</mo><mn>5</mn></math>"#;
    test_braille("Polish", expr, "⠻⠙⠑⠞⠣⠼⠁⠀⠼⠃⠰⠳⠼⠃⠀⠤⠼⠁⠜⠀⠶⠼⠁⠄⠣⠤⠼⠁⠜⠀⠤⠼⠃⠄⠼⠃⠀⠶⠤⠼⠑")?;
    return Ok(());
}

#[test]
fn matrices_p46_2() -> Result<()> {
    let expr = r#"<math><mfenced open="|" close="|"><mi mathvariant="bold">W</mi></mfenced><mo>=</mo><mo>|</mo><mfenced open="|" close="|"><mtable><mtr><mtd><mn>1</mn></mtd><mtd><mn>2</mn></mtd></mtr><mtr><mtd><mn>2</mn></mtd><mtd><mo>-</mo><mn>1</mn></mtd></mtr></mtable></mfenced><mo>|</mo><mo>=</mo><mo>|</mo><mn>1</mn><mo>&#xB7;</mo><mo>(</mo><mo>-</mo><mn>1</mn><mo>)</mo><mo>-</mo><mn>2</mn><mo>&#xB7;</mo><mn>2</mn><mo>|</mo><mo>=</mo><mo>|</mo><mo>-</mo><mn>5</mn><mo>|</mo><mo>=</mo><mn>5</mn></math>"#;
    // deleted dot 1 in (-1) as it appears to be a line break artifact
    test_braille("Polish", expr, "⠈⠇⠻⠙⠑⠞⠣⠼⠁⠀⠼⠃⠰⠳⠼⠃⠀⠤⠼⠁⠜⠸⠀⠶⠈⠇⠼⠁⠄⠣⠤⠼⠁⠜⠀⠤⠼⠃⠄⠼⠃⠸⠀⠶⠈⠇⠤⠼⠑⠸⠀⠶⠼⠑")?;
    return Ok(());
}

// Logarithms p. 47

#[test]
fn logarithms_p47_1() -> Result<()> {
    let expr = r#"<math><mi>log</mi><mn>1000</mn><mo>=</mo><mn>3</mn></math>"#;
    test_braille("Polish", expr, "⠫⠇⠼⠁⠚⠚⠚⠀⠶⠼⠉")?;
    return Ok(());
}

#[test]
fn logarithms_p47_2() -> Result<()> {
    let expr = r#"<math><msub><mi>log</mi><mn>2</mn></msub><mn>16</mn><mo>=</mo><mn>4</mn></math>"#;
    test_braille("Polish", expr, "⠌⠆⠫⠇⠼⠁⠋⠀⠶⠼⠙")?;
    return Ok(());
}

#[test]
fn logarithms_p47_3() -> Result<()> {
    let expr = r#"<math><mi>ln</mi><mo>&#xA0;</mo><msup><mi>e</mi><mn>2</mn></msup><mo>=</mo><mn>2</mn></math>"#;
    test_braille("Polish", expr, "⠫⠦⠇⠀⠠⠑⠬⠆⠀⠶⠼⠃")?;
    return Ok(());
}

#[test]
fn logarithms_p47_4() -> Result<()> {
    let expr = r#"<math><msub><mi>log</mi><mfrac><mn>1</mn><mn>2</mn></mfrac></msub><mn>4</mn><mo>=</mo><mo>-</mo><mn>2</mn></math>"#;
    test_braille("Polish", expr, "⠌⠼⠁⠆⠫⠇⠼⠙⠀⠶⠤⠼⠃")?;
    return Ok(());
}

#[test]
fn logarithms_p47_5() -> Result<()> {
    let expr = r#"<math><mfrac><mrow><mi>ln</mi><mo>(</mo><mi>x</mi><mo>+</mo><mn>2</mn><mo>)</mo></mrow><mrow><mi>x</mi><mo>-</mo><mn>2</mn></mrow></mfrac></math>"#;
    test_braille("Polish", expr, "⠫⠦⠇⠣⠠⠭⠀⠖⠼⠃⠜⠳⠭⠈⠤⠼⠃")?;
    return Ok(());
}

#[test]
fn logarithms_p47_6() -> Result<()> {
    let expr = r#"<math><mi>ln</mi><mfrac><mrow><mi>x</mi><mo>+</mo><mn>2</mn></mrow><mrow><mi>x</mi><mo>-</mo><mn>2</mn></mrow></mfrac></math>"#;
    test_braille("Polish", expr, "⠫⠦⠇⠀⠠⠭⠈⠖⠼⠃⠳⠭⠈⠤⠼⠃")?;
    return Ok(());
}

#[test]
fn logarithms_p47_7() -> Result<()> {
    let expr = r#"<math><msub><mi>log</mi><mi>a</mi></msub><msup><mi>x</mi><mi>m</mi></msup><mo>=</mo><mi>m</mi><mo>&#xB7;</mo><msub><mi>log</mi><mi>a</mi></msub><mi>x</mi></math>"#;
    test_braille("Polish", expr, "⠌⠠⠁⠫⠇⠭⠬⠍⠀⠶⠍⠄⠌⠠⠁⠫⠇⠭")?;
    return Ok(());
}

#[test]
fn logarithms_p47_8() -> Result<()> {
    let expr = r#"<math><msub><mi>log</mi><mi>x</mi></msub><mi>y</mi><mo>=</mo><mfrac><mn>1</mn><mrow><msub><mi>log</mi><mi>y</mi></msub><mi>x</mi></mrow></mfrac></math>"#;
    test_braille("Polish", expr, "⠌⠠⠭⠫⠇⠽⠀⠶⠼⠁⠳⠌⠽⠫⠇⠭")?;
    return Ok(());
}

// GEOMETRY p. 48

#[test]
fn geometry_p49_1() -> Result<()> {
    let expr = r#"<math><mi>A</mi><mi>B</mi><mo>&#x2225;</mo><mi>C</mi><mi>D</mi></math>"#;
    test_braille("Polish", expr, "⠨⠁⠃⠀⠈⠇⠇⠨⠉⠙")?;
    return Ok(());
}

#[test]
fn geometry_p49_2() -> Result<()> {
    let expr = r#"<math><mo>&#x2222;</mo><mi>B</mi><mo>=</mo><mo>&#x2222;</mo><mi>A</mi><mi>B</mi><mi>C</mi></math>"#;
    test_braille("Polish", expr, "⠻⠪⠨⠃⠀⠶⠻⠪⠨⠁⠃⠉")?;
    return Ok(());
}

#[test]
fn geometry_p49_3() -> Result<()> {
    let expr = r#"<math><mi>a</mi><mo>&#x2226;</mo><mi>b</mi></math>"#;
    // spec wrongly uses dot 3 instead of dot 6 for lower case letter indicator
    // removed lower case letter indicator before b -- not needed
    test_braille("Polish", expr, "⠠⠁⠀⠔⠈⠇⠇⠃")?;
    return Ok(());
}

#[test]
fn geometry_p49_4() -> Result<()> {
    let expr = r#"<math><mo>△</mo><mi>A</mi><mi>B</mi><mi>C</mi><mo>~</mo><mo>△</mo><msub><mi>A</mi><mn>1</mn></msub><msub><mi>B</mi><mn>1</mn></msub><msub><mi>C</mi><mn>1</mn></msub><mo>~</mo><mo>△</mo><msup><mi>A</mi><mo>'</mo></msup><msup><mi>B</mi><mo>'</mo></msup><msup><mi>C</mi><mo>'</mo></msup></math>"#;
    test_braille("Polish", expr, "⠻⠲⠨⠁⠃⠉⠀⠢⠻⠲⠨⠁⠡⠂⠃⠡⠂⠉⠡⠂⠀⠢⠻⠲⠨⠁⠔⠃⠔⠉⠔")?;
    return Ok(());
}

#[test]
fn geometry_p49_5() -> Result<()> {
    let expr = r#"<math><mover><mrow><mi>A</mi><mi>C</mi><mi>B</mi></mrow><mo>&#x23DC;</mo></mover></math>"#;
    test_braille("Polish", expr, "⠨⠣⠨⠁⠉⠃")?;
    return Ok(());
}

#[test]
fn geometry_p49_6() -> Result<()> {
    let expr = r#"<math><mi>C</mi><mo>&#x2208;</mo><mover><mrow><mi>A</mi><mi>B</mi></mrow><mo>&#xAF;</mo></mover></math>"#;
    // alternative version
    test_braille("Polish", expr, "⠨⠉⠀⠈⠑⠨⠒⠨⠁⠃")?;
    return Ok(());
}

#[test]
fn geometry_p49_7() -> Result<()> {
    let expr = r#"<math><mover><mrow><mi>A</mi><mi>C</mi></mrow><mo>&#xAF;</mo></mover><mo>&#x22A5;</mo><mover><mrow><mi>D</mi><mi>B</mi></mrow><mo>&#xAF;</mo></mover></math>"#;
    test_braille("Polish", expr, "⠨⠒⠨⠁⠉⠀⠼⠄⠨⠒⠨⠙⠃")?;
    return Ok(());
}

#[test]
fn geometry_p49_8() -> Result<()> {
    let expr = r#"<math><mover accent="true"><mrow><mi>A</mi><mi>B</mi></mrow><mo>&#x2192;</mo></mover><mo>=</mo><mo>-</mo><mover><mrow><mi>D</mi><mi>C</mi></mrow><mo>&#x2192;</mo></mover></math>"#;
    test_braille("Polish", expr, "⠨⠒⠂⠨⠁⠃⠀⠶⠤⠨⠒⠂⠨⠙⠉")?;
    return Ok(());
}

#[test]
fn geometry_p49_9() -> Result<()> {
    let expr = r#"<math><mover accent="true"><mi>V</mi><mo>&#x2192;</mo></mover><mo>=</mo><mover accent="true"><msub><mi>v</mi><mn>1</mn></msub><mo>&#x2192;</mo></mover><mo>+</mo><mover accent="true"><msub><mi>v</mi><mn>2</mn></msub><mo>&#x2192;</mo></mover></math>"#;
    test_braille("Polish", expr, "⠨⠒⠂⠨⠧⠀⠶⠨⠒⠂⠠⠧⠡⠂⠀⠖⠨⠒⠂⠧⠡⠆")?;
    return Ok(());
}

#[test]
fn geometry_p49_10() -> Result<()> {
    let expr = r#"<math><mtext>prosta</mtext><mi>P</mi><msub><mi>P</mi><mn>0</mn></msub><mo>&#x2225;</mo><mover><mpadded lspace="-1px"><mi>u</mi></mpadded><mo>&#x2192;</mo></mover></math>"#;
    test_braille("Polish", expr, "⠨⠒⠒⠨⠏⠏⠡⠴⠀⠈⠇⠇⠨⠒⠂⠠⠥")?;
    return Ok(());
}

#[test]
fn geometry_p49_11() -> Result<()> {
    let expr = r#"<math><mi>α</mi><mo>=</mo><mo>&#x2222;</mo><mo>(</mo><mover><mi>v</mi><mo>&#x2192;</mo></mover><mo>,</mo><mover><mi>u</mi><mo>&#x2192;</mo></mover><mo>)</mo></math>"#;
    test_braille("Polish", expr, "⠰⠁⠀⠶⠻⠪⠣⠨⠒⠂⠠⠧⠂⠀⠨⠒⠂⠥⠜")?;
    return Ok(());
}

// Analytical geometry p. 50

#[test]
fn geometry_p50_1() -> Result<()> {
    let expr = r#"<math><mi>l</mi><mo>:</mo><mi>A</mi><mi>x</mi><mo>+</mo><mi>B</mi><mi>y</mi><mo>+</mo><mi>C</mi><mo>=</mo><mn>0</mn><mo>,</mo><mi>P</mi><mo>=</mo><mo>(</mo><msub><mi>x</mi><mi>p</mi></msub><mo>,</mo><msub><mi>y</mi><mi>p</mi></msub><mo>)</mo></math>"#;
    test_braille("Polish", expr, "⠠⠇⠀⠲⠨⠁⠠⠭⠀⠖⠨⠃⠠⠽⠀⠖⠨⠉⠀⠶⠼⠚⠠⠂⠀⠨⠏⠀⠶⠣⠠⠭⠡⠏⠂⠀⠽⠡⠏⠜")?;
    return Ok(());
}

#[test]
#[ignore = "alternative form"]
fn geometry_p50_2() -> Result<()> {
    let expr = r#"<math><mi>d</mi><mo>=</mo><mfrac><mfenced open="|" close="|"><mrow><mi>A</mi><msub><mi>x</mi><mi>p</mi></msub><mo>+</mo><mi>B</mi><msub><mi>y</mi><mi>p</mi></msub><mo>+</mo><mi>C</mi></mrow></mfenced><msqrt><msup><mi>A</mi><mn>2</mn></msup><mo>+</mo><msup><mi>B</mi><mn>2</mn></msup></msqrt></mfrac></math>"#;
    test_braille("Polish", expr, "⠠⠙⠀⠶⠆⠈⠇⠨⠁⠠⠭⠡⠏⠀⠖⠨⠃⠠⠽⠡⠏⠀⠖⠨⠉⠸⠀⠳⠀⠐⠩⠁⠬⠆⠈⠖⠃⠬⠆⠰")?;
    return Ok(());
}

// or

#[test]
fn geometry_p50_2a() -> Result<()> {
    let expr = r#"<math><mi>d</mi><mo>=</mo><mfrac><mfenced open="|" close="|"><mrow><mi>A</mi><msub><mi>x</mi><mi>p</mi></msub><mo>+</mo><mi>B</mi><msub><mi>y</mi><mi>p</mi></msub><mo>+</mo><mi>C</mi></mrow></mfenced><msqrt><msup><mi>A</mi><mn>2</mn></msup><mo>+</mo><msup><mi>B</mi><mn>2</mn></msup></msqrt></mfrac></math>"#;
    test_braille("Polish", expr, "⠠⠙⠀⠶⠆⠈⠇⠨⠁⠠⠭⠡⠏⠀⠖⠨⠃⠠⠽⠡⠏⠀⠖⠨⠉⠸⠀⠳⠀⠐⠩⠨⠁⠬⠆⠈⠖⠨⠃⠬⠆⠰")?;
    return Ok(());
}

#[test]
fn geometry_p50_3() -> Result<()> {
    let expr = r#"<math><mi>A</mi><mo>=</mo><mo>(</mo><msub><mi>x</mi><mi>a</mi></msub><mo>,</mo><msub><mi>y</mi><mi>a</mi></msub><mo>)</mo><mo>,</mo></math>"#;
    test_braille("Polish", expr, "⠨⠁⠀⠶⠣⠠⠭⠡⠁⠂⠀⠽⠡⠁⠜⠂")?;
    return Ok(());
}

#[test]
fn geometry_p50_4() -> Result<()> {
    let expr = r#"<math><msub><mi>P</mi><mrow><mi mathvariant="normal">&#x394;</mi><mi>A</mi><mi>B</mi><mi>C</mi></mrow></msub><mo>=</mo><mfrac><mn>1</mn><mn>2</mn></mfrac><mo>|</mo><mi>d</mi><mo>(</mo><mover accent="true"><mi>AB</mi><mo stretchy="false" accent="false">&#x2192;</mo></mover><mo>,</mo><mo>&#xA0;</mo><mover accent="true"><mi>AC</mi><mo stretchy="false" accent="false">&#x2192;</mo></mover><mo>)</mo><mo>|=</mo><mfrac><mn>1</mn><mn>2</mn></mfrac><mo>|</mo><mfenced open="|" close="|" separators="|"><mtable columnspacing="1em"><mtr><mtd><msub><mi>x</mi><mi>b</mi></msub><mo>&#x2212;</mo><msub><mi>x</mi><mi>a</mi></msub><mo>,</mo></mtd><mtd><msub><mi>y</mi><mi>b</mi></msub><mo>&#x2212;</mo><msub><mi>y</mi><mi>a</mi></msub></mtd></mtr><mtr><mtd><msub><mi>x</mi><mi>c</mi></msub><mo>&#x2212;</mo><msub><mi>x</mi><mi>a</mi></msub><mo>,</mo></mtd><mtd><msub><mi>y</mi><mi>c</mi></msub><mo>&#x2212;</mo><msub><mi>y</mi><mi>a</mi></msub></mtd></mtr></mtable></mfenced><mo>|</mo></math>"#;
    test_braille("Polish", expr, "⠨⠏⠡⠻⠲⠨⠁⠃⠉⠀⠶⠼⠁⠆⠈⠇⠠⠙⠣⠨⠒⠂⠨⠁⠃⠂⠠⠨⠒⠂⠨⠁⠉⠜⠸⠀⠶⠼⠁⠆⠈⠇⠇⠠⠭⠡⠃⠀⠤⠭⠡⠁⠂⠀⠽⠡⠃⠀⠤⠽⠡⠁⠸⠸⠇⠇⠀⠭⠡⠉⠀⠤⠭⠡⠁⠂⠀⠽⠡⠉⠀⠤⠽⠡⠁⠸⠸")?;
    return Ok(());
}

// Trigonometry

#[test]
fn trigonometry_p51_1() -> Result<()> {
    let expr = r#"<math><mi>&#x3B1;</mi><mo>=</mo><mn>30</mn><mo>&#xB0;</mo></math>"#;
    test_braille("Polish", expr, "⠰⠁⠀⠶⠼⠉⠚⠴")?;
    return Ok(());
}

#[test]
fn trigonometry_p51_2() -> Result<()> {
    let expr = r#"<math><mi>&#x3B2;</mi><mo>=</mo><msup><mfrac><mn>1</mn><mn>5</mn></mfrac><mo>&#xB0;</mo></msup></math>"#;
    test_braille("Polish", expr, "⠰⠃⠀⠶⠼⠁⠢⠘⠴")?;
    return Ok(());
}

#[test]
fn trigonometry_p51_3() -> Result<()> {
    let expr = r#"<math><mn>19</mn><mo>&#xB0;</mo><mn>23</mn><mo>'</mo><mn>47</mn><mo>&quot;</mo></math>"#;
    test_braille("Polish", expr, "⠼⠁⠊⠴⠼⠃⠉⠘⠔⠼⠙⠛⠘⠔⠔")?;
    return Ok(());
}

#[test]
#[ignore = "spells out radian in answer instead of 'rad'"]
fn trigonometry_p51_4_radian() -> Result<()> {
    let expr = r#"<math><mn>90</mn><mo>&#xB0;</mo><mo>=</mo><mfrac><mi>&#x3C0;</mi><mn>2</mn></mfrac><mi>rad</mi></math>"#;
    test_braille("Polish", expr, "⠼⠊⠚⠴⠀⠶⠰⠏⠳⠆⠻⠗⠁⠙")?;
    return Ok(());
}

#[test]
fn trigonometry_p51_4() -> Result<()> {
    let expr = r#"<math><mn>90</mn><mo>&#xB0;</mo><mo>=</mo><mfrac><mi>&#x3C0;</mi><mn>2</mn></mfrac><mi>rad</mi></math>"#;
    test_braille("Polish", expr, "⠼⠊⠚⠴⠀⠶⠰⠏⠳⠆⠼⠗")?;
    return Ok(());
}

#[test]
fn trigonometry_p51_5() -> Result<()> {
    let expr = r#"<math><mn>1</mn><mi>rad</mi><mo>=</mo><mfrac><mrow><mn>180</mn><mo>&#xB0;</mo></mrow><mi>&#x3C0;</mi></mfrac><mo>&#x2248;</mo><mfrac><mrow><mn>180</mn><mo>&#xB0;</mo></mrow><mrow><mn>3</mn><mo>,</mo><mn>14159</mn></mrow></mfrac><mo>&#x2248;</mo><mn>57</mn><mo>&#xB0;</mo><mn>17</mn><mo>'</mo><mn>45</mn><mo>&quot;</mo></math>"#;
    test_braille("Polish", expr, "⠼⠁⠼⠗⠀⠶⠼⠁⠓⠚⠴⠳⠰⠏⠀⠢⠢⠼⠁⠓⠚⠴⠳⠼⠉⠂⠁⠙⠁⠑⠊⠀⠢⠢⠼⠑⠛⠴⠼⠁⠛⠘⠔⠼⠙⠑⠘⠔⠔")?;
    return Ok(());
}

// Trigonometric functions

#[test]
fn trigonometry_p52_1() -> Result<()> {
    let expr = r#"<math><mi>sin</mi><mn>60</mn><mo>&#xB0;</mo><mo>=</mo><mfrac><msqrt><mn>3</mn></msqrt><mn>2</mn></mfrac></math>"#;
    test_braille("Polish", expr, "⠫⠎⠼⠋⠚⠴⠀⠶⠩⠼⠉⠳⠆")?;
    return Ok(());
}

#[test]
fn trigonometry_p52_2() -> Result<()> {
    let expr = r#"<math><msup><mi>sin</mi><mn>2</mn></msup><mi>&#x3B1;</mi><mo>+</mo><msup><mi>cos</mi><mn>2</mn></msup><mi>&#x3B1;</mi><mo>=</mo><mn>1</mn></math>"#;
    test_braille("Polish", expr, "⠫⠎⠬⠆⠰⠁⠀⠖⠫⠉⠬⠆⠰⠁⠀⠶⠼⠁")?;
    return Ok(());
}

#[test]
fn trigonometry_p52_3() -> Result<()> {
    let expr = r#"<math><mfrac><mrow><mi>cos</mi><mi>&#x3B1;</mi></mrow><mn>2</mn></mfrac></math>"#;
    test_braille("Polish", expr, "⠫⠉⠰⠁⠳⠆")?;
    return Ok(());
}

#[test]
fn trigonometry_p52_4() -> Result<()> {
    let expr = r#"<math><mi>cos</mi><mfenced><mfrac><mi>&#x3B1;</mi><mn>2</mn></mfrac></mfenced></math>"#;
    test_braille("Polish", expr, "⠫⠉⠣⠰⠁⠳⠆⠜")?;
    return Ok(());
}

#[test]
fn trigonometry_p52_5() -> Result<()> {
    let expr = r#"<math><mi>tg</mi><mfenced><mfrac><mrow><mn>90</mn><mo>&#xB0;</mo><mo>-</mo><mi>&#x3B1;</mi></mrow><mn>2</mn></mfrac></mfenced></math>"#;
    test_braille("Polish", expr, "⠫⠞⠣⠼⠊⠚⠴⠈⠤⠰⠁⠳⠆⠜")?;
    return Ok(());
}

#[test]
fn trigonometry_p52_6() -> Result<()> {
    let expr = r#"<math><mi>cos</mi><mfrac><mi>&#x3B1;</mi><mn>2</mn></mfrac></math>"#;
    test_braille("Polish", expr, "⠫⠉⠀⠰⠁⠳⠆")?;
    return Ok(());
}

// or

#[test]
#[ignore = "alternative rendering"]
fn trigonometry_p52_6a() -> Result<()> {
    let expr = r#"<math><mi>cos</mi><mfrac><mi>&#x3B1;</mi><mn>2</mn></mfrac></math>"#;
    test_braille_prefs("Polish", vec![("Polish_BrailleLevel", "Advanced")], expr, r"⠫⠉⠆⠰⠁⠳⠆")?;
    return Ok(());
}

// or

#[test]
#[ignore = "alternative rendering"]
fn trigonometry_p52_6b() -> Result<()> {
    let expr = r#"<math><mi>cos</mi><mfrac><mi>&#x3B1;</mi><mn>2</mn></mfrac></math>"#;
    test_braille("Polish", expr, "⠫⠉⠆⠰⠁⠀⠳⠀⠼⠃⠰")?;
    return Ok(());
}

#[test]
fn trigonometry_p52_7() -> Result<()> {
    let expr = r#"<math><mi>tg</mi><mfrac><mrow><msup><mn>90</mn><mo>°</mo></msup><mo>&#x2212;</mo><mi>&#x3B1;</mi></mrow><mn>2</mn></mfrac></math>"#;
    test_braille("Polish", expr, "⠫⠞⠀⠼⠊⠚⠴⠈⠤⠰⠁⠳⠆")?;
    return Ok(());
}

// or

#[test]
#[ignore = "alternative rendering"]
fn trigonometry_p52_7a() -> Result<()> {
    let expr = r#"<math><mi>tg</mi><mfenced><mfrac><mrow><msup><mn>90</mn><mo>&#x2218;</mo></msup><mo>&#x2212;</mo><mi>&#x3B1;</mi></mrow><mn>2</mn></mfrac></mfenced></math>"#;
    test_braille("Polish", expr, "⠫⠞⠆⠼⠊⠚⠴⠈⠤⠰⠁⠳⠆")?;
    return Ok(());
}

// or

#[test]
#[ignore = "alternative rendering"]
fn trigonometry_p52_7b() -> Result<()> {
    let expr = r#"<math><mi>tg</mi><mfenced><mfrac><mrow><msup><mn>90</mn><mo>&#x2218;</mo></msup><mo>&#x2212;</mo><mi>&#x3B1;</mi></mrow><mn>2</mn></mfrac></mfenced></math>"#;
    test_braille("Polish", expr, "⠫⠞⠆⠼⠊⠚⠴⠀⠤⠰⠁⠀⠳⠀⠼⠃")?;
    return Ok(());
}

#[test]
fn trigonometry_p52_8() -> Result<()> {
    let expr = r#"<math><mi>tg</mi><mfrac><mi>&#x3B1;</mi><mn>2</mn></mfrac><mo>=</mo><mfrac><mrow><mn>1</mn><mo>-</mo><mi>cos</mi><mi>&#x3B1;</mi></mrow><mrow><mi>sin</mi><mi>&#x3B1;</mi></mrow></mfrac></math>"#;
    test_braille("Polish", expr, "⠫⠞⠀⠰⠁⠳⠆⠀⠶⠼⠁⠈⠤⠫⠉⠰⠁⠳⠫⠎⠰⠁")?;
    return Ok(());
}

#[test]
fn trigonometry_p53_1() -> Result<()> {
    let expr = r#"<math><mi>cos</mi><mfenced><mrow><mo>-</mo><mfrac><mi>&#x3C0;</mi><mn>3</mn></mfrac></mrow></mfenced><mo>=</mo><mi>cos</mi><mfenced><mrow><mo>-</mo><mfrac><mfrac><mi>&#x3C0;</mi><mn>3</mn></mfrac><mi>&#x3C0;</mi></mfrac><mo>&#xB7;</mo><mn>180</mn><mo>&#xB0;</mo></mrow></mfenced><mo>=</mo><mi>cos</mi><mo>(</mo><mo>-</mo><mn>60</mn><mo>&#xB0;</mo><mo>)</mo><mo>=</mo><mfrac><mn>1</mn><mn>2</mn></mfrac></math>"#;
    test_braille("Polish", expr, "⠫⠉⠣⠤⠰⠏⠳⠒⠜⠀⠶⠫⠉⠣⠤⠆⠆⠰⠏⠳⠒⠀⠳⠀⠰⠏⠰⠄⠼⠁⠓⠚⠴⠜⠀⠶⠫⠉⠣⠤⠼⠋⠚⠴⠜⠀⠶⠼⠁⠆")?;
    return Ok(());
}

#[test]
fn trigonometry_p53_2() -> Result<()> {
    let expr = r#"<math><mi>cosec</mi><mo>&#xA0;</mo><mn>30</mn><mo>&#xB0;</mo><mo>=</mo><mfrac><mn>1</mn><mrow><mi>sin</mi><mn>30</mn><mo>&#xB0;</mo></mrow></mfrac><mo>=</mo><mfrac><mn>1</mn><mfrac><mn>1</mn><mn>2</mn></mfrac></mfrac><mo>=</mo><mn>2</mn></math>"#;
    test_braille("Polish", expr, "⠫⠣⠼⠉⠚⠴⠀⠶⠼⠁⠳⠫⠎⠼⠉⠚⠴⠀⠶⠼⠁⠳⠼⠁⠆⠀⠶⠼⠃")?;
    return Ok(());
}

// Inverse trigonometric functions

#[test]
fn trigonometry_p53_3() -> Result<()> {
    let expr = r#"<math><mi>f</mi><mo>(</mo><mi>x</mi><mo>)</mo><mo>=</mo><mi>tg</mi><mi>x</mi></math>"#;
    test_braille("Polish", expr, "⠠⠋⠣⠭⠜⠀⠶⠫⠞⠭")?;
    return Ok(());
}

#[test]
fn trigonometry_p53_4() -> Result<()> {
    let expr = r#"<math><msup><mi>f</mi><mrow><mo>-</mo><mn>1</mn></mrow></msup><mo>(</mo><mi>x</mi><mo>)</mo><mo>=</mo><mi>arctg</mi><mo>&#xA0;</mo><mi>x</mi></math>"#;
    test_braille("Polish", expr, "⠠⠋⠌⠤⠂⠱⠣⠭⠜⠀⠶⠫⠂⠞⠭")?;
    return Ok(());
}

#[test]
fn trigonometry_p53_5() -> Result<()> {
    let expr = r#"<math><mi>arcsin</mi><mo>(</mo><msup><mn>30</mn><mo>&#x2218;</mo></msup><mo>+</mo><mi>n</mi><mo>&#x22C5;</mo><msup><mn>360</mn><mo>&#x2218;</mo></msup><mo>)</mo><mo>=</mo><mfrac><mn>1</mn><mn>2</mn></mfrac><mo>&#xA0;</mo><mi>dla</mi><mo>&#xA0;&#xA0;</mo><mi>n</mi><mo>=</mo><mn>0</mn><mo>,</mo><mo>&#xA0;</mo><mo>&#xB1;</mo><mn>1</mn><mo>,</mo><mo>&#xA0;</mo><mo>&#xB1;</mo><mn>2</mn><mo>,</mo><mo>&#xA0;</mo><mo>.</mo><mo>.</mo><mo>.</mo></math>"#;
    test_braille("Polish", expr, "⠫⠂⠎⠣⠼⠉⠚⠴⠀⠖⠠⠝⠄⠼⠉⠋⠚⠴⠜⠀⠶⠼⠁⠆⠙⠇⠁⠀⠠⠝⠀⠶⠼⠚⠠⠂⠀⠖⠤⠼⠁⠠⠂⠀⠖⠤⠼⠃⠠⠂⠀⠄⠄⠄")?;
    return Ok(());
}

// MATHEMATICAL LOGIC

#[test]
fn math_logic_p54_1() -> Result<()> {
    let expr = r#"<math><mi>a</mi><mo>&#x2228;</mo><mi>b</mi></math>"#;
    test_braille("Polish", expr, "⠠⠁⠀⠩⠂⠃")?;
    return Ok(());
}

#[test]
fn math_logic_p54_2() -> Result<()> {
    let expr = r#"<math><mo>(</mo><mo>(</mo><mi>p</mi><mo>&#x2227;</mo><mi>q</mi><mo>)</mo><mo>&#x2228;</mo><mo>(</mo><mi>q</mi><mo>&#x2227;</mo><mi>r</mi><mo>)</mo><mo>)</mo><mo>&#x21D2;</mo><mi>v</mi></math>"#;
    test_braille("Polish", expr, "⠣⠣⠠⠏⠀⠬⠂⠟⠜⠀⠩⠂⠣⠟⠀⠬⠂⠗⠜⠜⠀⠶⠂⠧")?;
    return Ok(());
}

#[test]
fn math_logic_p54_3() -> Result<()> {
    let expr = r#"<math><mo>&#x2200;</mo><mi>x</mi><mo>&#xA0;&#xA0;</mo><msup><mi>x</mi><mn>2</mn></msup><mo>&#x2A7E;</mo><mn>0</mn></math>"#;
    test_braille("Polish", expr, "⠯⠂⠠⠭⠀⠭⠬⠆⠀⠕⠶⠼⠚")?;
    return Ok(());
}

#[test]
fn math_logic_p54_4() -> Result<()> {
    let expr = r#"<math><mo>&#x2203;</mo><mi>x</mi><mo>&#xA0;&#xA0;</mo><mi>x</mi><mo>+</mo><mn>1</mn><mo>&lt;</mo><mn>0</mn></math>"#;
    test_braille("Polish", expr, "⠯⠢⠠⠭⠀⠭⠀⠖⠼⠁⠀⠪⠄⠼⠚")?;
    return Ok(());
}

#[test]
fn math_logic_p54_5() -> Result<()> {
    // needed to add form="prefix" to the second negation to get the right version of ∼ (U+223C); ASCII ~ is converted to this in canonicalization
    let expr = r#"<math><mo>~</mo><mo>&#x2200;</mo><mi>x</mi><mo>&#xA0;&#xA0;</mo><mi>p</mi><mo>(</mo><mi>x</mi><mo>)</mo><mo>&#x21D4;</mo><mo>&#x2203;</mo><mi>x</mi><mo>&#xA0;&#xA0;</mo><mo form="prefix">~</mo><mi>p</mi><mo>(</mo><mi>x</mi><mo>)</mo></math>"#;
    test_braille("Polish", expr, "⠒⠔⠯⠂⠠⠭⠀⠏⠣⠭⠜⠀⠐⠶⠂⠯⠢⠭⠀⠒⠔⠏⠣⠭⠜")?;
    return Ok(());
}

// PROBABILITY AND COMBINATORICS

#[test]
fn prob_comb_p55_1() -> Result<()> {
    let expr = r#"<math><mn>5</mn><mo>!</mo><mo>=</mo><mn>1</mn><mo>&#xB7;</mo><mn>2</mn><mo>&#xB7;</mo><mn>3</mn><mo>&#xB7;</mo><mn>4</mn><mo>&#xB7;</mo><mn>5</mn></math>"#;
    test_braille("Polish", expr, "⠼⠑⠫⠀⠶⠼⠁⠄⠼⠃⠄⠼⠉⠄⠼⠙⠄⠼⠑")?;
    return Ok(());
}

#[test]
fn prob_comb_p55_2() -> Result<()> {
    let expr = r#"<math><mfenced><mtable intent="binomial-coefficient($upper, $lower)"><mtr><mtd><mi>n</mi></mtd></mtr><mtr><mtd><mi>k</mi></mtd></mtr></mtable></mfenced><mo>=</mo><mfrac><mrow><mi>n</mi><mo>!</mo></mrow><mrow><mi>k</mi><mo>!</mo><mo>(</mo><mi>n</mi><mo>-</mo><mi>k</mi><mo>)</mo><mo>!</mo></mrow></mfrac></math>"#;
    test_braille_prefs("Polish", vec![("Polish_BrailleLevel", "Beginner")], expr, r"⠣⠠⠝⠰⠳⠅⠜⠀⠶⠆⠝⠫⠀⠳⠀⠅⠫⠈⠣⠝⠀⠤⠅⠜⠫⠈⠰")?;
    return Ok(());
}

#[test]
fn prob_comb_p55_3() -> Result<()> {
    let expr = r#"<math><msubsup><mi>C</mi><mi>n</mi><mi>k</mi></msubsup><mo>=</mo><mfenced><mtable intent="binomial-coefficient($upper, $lower)"><mtr><mtd><mi>n</mi></mtd></mtr><mtr><mtd><mi>k</mi></mtd></mtr></mtable></mfenced></math>"#;
    test_braille("Polish", expr, "⠨⠉⠌⠠⠅⠡⠝⠀⠶⠣⠝⠰⠳⠅⠜")?;
    return Ok(());
}

#[test]
fn prob_comb_p55_4() -> Result<()> {
    let expr = r#"<math><msubsup><mover><mi>C</mi><mo>&#xAF;</mo></mover><mi>n</mi><mi>k</mi></msubsup><mo>=</mo>
        <mfenced><mtable intent="binomial-coefficient($upper, $lower)">
            <mtr><mtd><mi>n</mi><mo>+</mo><mi>k</mi><mo>-</mo><mn>1</mn></mtd></mtr><mtr><mtd><mi>n</mi><mo>-</mo><mn>1</mn></mtd></mtr>
        </mtable></mfenced><mo>=</mo>
        <mfenced><mtable intent="binomial-coefficient($upper, $lower)">
            <mtr><mtd><mi>n</mi><mo>+</mo><mi>k</mi><mo>-</mo><mn>1</mn></mtd></mtr><mtr><mtd><mi>k</mi></mtd></mtr>
        </mtable></mfenced></math>"#;
    test_braille("Polish", expr, "⠨⠉⠒⠌⠠⠅⠡⠝⠀⠶⠣⠝⠈⠖⠅⠈⠤⠼⠁⠰⠳⠝⠈⠤⠼⠁⠜⠀⠶⠣⠝⠈⠖⠅⠈⠤⠼⠁⠰⠳⠅⠜")?;
    return Ok(());
}

#[test]
fn prob_comb_p55_5() -> Result<()> {
    let expr = r#"<math><msubsup><mi>V</mi><mi>n</mi><mi>k</mi></msubsup><mo>=</mo><mfrac><mrow><mi>n</mi><mo>!</mo></mrow><mrow><mo>(</mo><mi>n</mi><mo>-</mo><mi>k</mi><mo>)</mo><mo>!</mo></mrow></mfrac></math>"#;
    test_braille_prefs("Polish", vec![("Polish_BrailleLevel", "Beginner")], expr, r"⠨⠧⠌⠠⠅⠡⠝⠀⠶⠆⠝⠫⠀⠳⠀⠣⠝⠀⠤⠅⠜⠫⠈⠰")?;
    return Ok(());
}

#[test]
fn prob_comb_p55_6() -> Result<()> {
    let expr = r#"<math><msubsup><mover><mi>V</mi><mo>&#xAF;</mo></mover><mi>n</mi><mi>k</mi></msubsup><mo>=</mo><msup><mi>n</mi><mi>k</mi></msup></math>"#;
    test_braille("Polish", expr, "⠨⠧⠒⠌⠠⠅⠡⠝⠀⠶⠝⠬⠅")?;
    return Ok(());
}

// Borders p56

#[test]
fn limits_p56_1() -> Result<()> {
    let expr = r#"<math><munder><mi>lim</mi><mrow><mi>x</mi><mo>&#x2192;</mo><mo>&#x221E;</mo></mrow></munder><mfrac><mn>1</mn><mi>x</mi></mfrac><mo>=</mo><mn>0</mn></math>"#;
    test_braille("Polish", expr, "⠼⠇⠡⠠⠭⠈⠒⠂⠼⠿⠀⠼⠁⠳⠭⠀⠶⠼⠚")?;
    return Ok(());
}

#[test]
fn limits_p56_2() -> Result<()> {
    let expr = r#"<math><munder><mi>lim</mi><mrow><mi>x</mi><mo>&#x2192;</mo><msup><mn>0</mn><mo>+</mo></msup></mrow></munder><mfrac><mn>1</mn><mi>x</mi></mfrac><mo>=</mo><mo>&#x221E;</mo></math>"#;
    test_braille("Polish", expr, r"⠼⠇⠡⠠⠭⠈⠒⠂⠼⠚⠘⠖⠀⠼⠁⠳⠭⠀⠶⠼⠿")?;
    return Ok(());
}

#[test]
fn limits_p56_3() -> Result<()> {
    let expr = r#"<math><munder><mi>lim</mi><mrow><mi>n</mi><mo>&#x2192;</mo><mo>&#x221E;</mo></mrow></munder><mfrac><mrow><mi>n</mi><mo>+</mo><mn>1</mn></mrow><mi>n</mi></mfrac><mo>=</mo><mn>1</mn></math>"#;
    test_braille("Polish", expr, "⠼⠇⠡⠠⠝⠈⠒⠂⠼⠿⠀⠝⠈⠖⠼⠁⠳⠝⠀⠶⠼⠁X")?;
    return Ok(());
}

#[test]
fn limits_p56_4() -> Result<()> {
    let expr = r#"<math><munder><mi>lim</mi><mrow><mi>x</mi><mo>&#x2192;</mo><mo>&#x221E;</mo></mrow></munder><msup><mn>2</mn><mrow><mo>-</mo><mi>x</mi></mrow></msup><mo>=</mo><mn>0</mn></math>"#;
    test_braille("Polish", expr, "⠼⠇⠡⠠⠭⠈⠒⠂⠼⠿⠀⠼⠃⠬⠤⠭⠀⠶⠼⠚")?;
    return Ok(());
}

#[test]
fn limits_p56_5() -> Result<()> {
    let expr = r#"<math><munder><mi>lim</mi><mrow><mi>&#x394;</mi><mi>x</mi><mo>&#x2192;</mo><mn>0</mn></mrow></munder><mfrac><mrow><mi>f</mi><mo>(</mo><mi>x</mi><mo>+</mo><mi>&#x394;</mi><mi>x</mi><mo>)</mo><mo>-</mo><mi>f</mi><mo>(</mo><mi>x</mi><mo>)</mo></mrow><mrow><mi>&#x394;</mi><mi>x</mi></mrow></mfrac></math>"#;
    // uses long form of fraction
    test_braille_prefs("Polish", vec![("Polish_BrailleLevel", "Beginner")], expr, r"⠼⠇⠡⠸⠙⠠⠭⠈⠒⠂⠼⠚⠀⠆⠋⠣⠭⠀⠖⠸⠙⠠⠭⠜⠀⠤⠋⠣⠭⠜⠀⠳⠀⠸⠙⠠⠭⠰")?;
    return Ok(());
}

#[test]
#[ignore = "uses long form for no particular reason -- MathCAT uses short form"]
fn limits_p56_6() -> Result<()> {
    let expr = r#"<math><munder><mi>lim</mi><mrow><mi>x</mi><mo>&#x2192;</mo><mn>2</mn></mrow></munder><mfrac><mrow><msup><mi>x</mi><mn>2</mn></msup><mo>-</mo><mn>4</mn></mrow><mrow><mi>x</mi><mo>-</mo><mn>2</mn></mrow></mfrac><mo>=</mo><munder><mi>lim</mi><mrow><mi>x</mi><mo>&#x2192;</mo><mn>2</mn></mrow></munder><mo>(</mo><mi>x</mi><mo>+</mo><mn>2</mn><mo>)</mo><mo>=</mo><mn>4</mn></math>"#;
    // uses long form of fraction
    test_braille_prefs("Polish", vec![("Polish_BrailleLevel", "Beginner")], expr, r"⠼⠇⠡⠠⠭⠈⠒⠂⠼⠃⠀⠆⠭⠬⠆⠀⠤⠼⠙⠀⠳⠀⠭⠀⠤⠼⠃⠰⠀⠶⠼⠇⠡⠠⠭⠈⠒⠂⠼⠃⠀⠣⠭⠀⠖⠼⠃⠜⠀⠶⠼⠙")?;
    return Ok(());
}

#[test]
fn limits_p56_7() -> Result<()> {
    let expr = r#"<math><msup><mi>e</mi><mrow><munder><mi>lim</mi><mrow><mi>x</mi><mo>&#x2192;</mo><msub><mi>x</mi><mn>0</mn></msub></mrow></munder><mi>f</mi><mo>(</mo><mi>x</mi><mo>)</mo></mrow></msup></math>"#;
    test_braille("Polish", expr, "⠠⠑⠨⠬⠼⠇⠐⠡⠭⠈⠒⠂⠭⠡⠴⠀⠋⠣⠭⠜⠨⠱")?;
    return Ok(());
}

// derivatives_integrals p57

#[test]
fn derivatives_p57_1() -> Result<()> {
    let expr = r#"<math><msup><mi>f</mi><mo>'</mo></msup><mo>(</mo><mi>x</mi><mo>)</mo><mo>=</mo><mfrac><mrow><mi>d</mi><mi>y</mi></mrow><mrow><mi>d</mi><mi>x</mi></mrow></mfrac><mo>=</mo><msup><mi>y</mi><mo>'</mo></msup></math>"#;
    test_braille("Polish", expr, "⠠⠋⠔⠣⠭⠜⠀⠶⠙⠽⠳⠙⠭⠀⠶⠽⠔")?;
    return Ok(());
}

#[test]
fn derivatives_p57_2() -> Result<()> {
    let expr = r#"<math><mi>g</mi><mo>(</mo><mi>x</mi><mo>,</mo><mi>y</mi><mo>)</mo><mo>=</mo><mfrac><mrow><mo>&#x2202;</mo><mi>f</mi><mo>(</mo><mi>x</mi><mo>,</mo><mi>y</mi><mo>)</mo></mrow><mrow><mo>&#x2202;</mo><mi>x</mi></mrow></mfrac></math>"#;
    test_braille("Polish", expr, "⠠⠛⠣⠭⠂⠀⠽⠜⠀⠶⠹⠋⠣⠭⠂⠀⠽⠜⠳⠹⠭")?;
    return Ok(());
}

#[test]
fn integrals_p57_1() -> Result<()> {
    let expr = r#"<math><mo>&#x222B;</mo><msup><mi>f</mi><mo>'</mo></msup><mo>(</mo><mi>x</mi><mo>)</mo><mi>d</mi><mi>x</mi><mo>=</mo><mi>f</mi><mo>(</mo><mi>x</mi><mo>)</mo><mo>+</mo><mi>C</mi></math>"#;
    test_braille("Polish", expr, "⠮⠠⠋⠔⠣⠭⠜⠀⠙⠭⠀⠶⠋⠣⠭⠜⠀⠖⠨⠉")?;
    return Ok(());
}

#[test]
fn integrals_p57_2() -> Result<()> {
    let expr = r#"<math><msubsup><mo>&#x222B;</mo><msub><mi>x</mi><mn>1</mn></msub><msub><mi>x</mi><mn>2</mn></msub></msubsup><mi>f</mi><mo>(</mo><mi>x</mi><mo>)</mo><mi>d</mi><mi>x</mi><mo>=</mo><mi>F</mi><mo>(</mo><msub><mi>x</mi><mn>1</mn></msub><mo>)</mo><mo>&#x2212;</mo><mi>F</mi><mo>(</mo><msub><mi>x</mi><mn>2</mn></msub><mo>)</mo></math>"#;
    test_braille("Polish", expr, "⠮⠐⠡⠠⠭⠡⠂⠐⠌⠭⠡⠆⠀⠋⠣⠭⠜⠀⠙⠭⠀⠶⠨⠋⠣⠠⠭⠡⠂⠜⠀⠤⠨⠋⠣⠠⠭⠡⠆⠜")?;
    return Ok(());
}

#[test]
fn integrals_p57_3() -> Result<()> {
    let expr = r#"<math><msubsup><mo>&#x222B;</mo><mn>0</mn><mn>2</mn></msubsup><mn>3</mn><msup><mi>x</mi><mn>2</mn></msup><mi>d</mi><mi>x</mi><mo>=</mo><msup><mi>x</mi><mn>3</mn></msup><msubsup><mi mathvariant="normal">|</mi><mn>0</mn><mn>2</mn></msubsup><mo>=</mo><msup><mn>2</mn><mn>3</mn></msup><mo>&#x2212;</mo><msup><mn>0</mn><mn>3</mn></msup><mo>=</mo><mn>8</mn></math>"#;
    test_braille("Polish", expr, "⠮⠡⠴⠌⠆⠀⠼⠉⠠⠭⠬⠆⠀⠙⠭⠀⠶⠭⠬⠒⠀⠸⠡⠴⠌⠆⠀⠶⠼⠃⠬⠒⠀⠤⠼⠚⠬⠒⠀⠶⠼⠓")?;
    return Ok(());
}

// BRAILLE PHYSICAL NOTATION p60

#[test]
fn physics_p60_1() -> Result<()> {
    let expr = r#"<math><mi>v</mi><mo>=</mo><msqrt><msubsup><mi>v</mi><mn>1</mn><mn>2</mn></msubsup><mo>+</mo><msubsup><mi>v</mi><mn>2</mn><mn>2</mn></msubsup></msqrt></math>"#;
    test_braille("Polish", expr, "⠠⠧⠀⠶⠐⠩⠧⠡⠂⠬⠆⠈⠖⠧⠡⠆⠬⠆")?;
    return Ok(());
}

#[test]
fn physics_p60_2() -> Result<()> {
    let expr = r#"<math><mi>&#x3BB;</mi><mo>=</mo><mfrac><mrow><mi>ln</mi><mn>2</mn></mrow><msub><mi>T</mi><mrow><mn>1</mn><mo>/</mo><mn>2</mn></mrow></msub></mfrac></math>"#;
    test_braille("Polish", expr, "⠰⠇⠀⠶⠫⠦⠇⠼⠃⠳⠨⠞⠡⠼⠁⠆")?;
    return Ok(());
}

#[test]
#[ignore = "alternative version that does not use capital letter redundancy"]
fn physics_p60_3() -> Result<()> {
    let expr = r#"<math><msub><mi>C</mi><mn>0</mn></msub><mo>=</mo><mfrac><mi>Q</mi><msub><mi>V</mi><mn>0</mn></msub></mfrac></math>"#;
    test_braille("Polish", expr, "⠨⠉⠡⠴⠀⠶⠟⠳⠧⠡⠴")?;
    return Ok(());
}

// or

#[test]
fn physics_p60_3a() -> Result<()> {
    let expr = r#"<math><msub><mi>C</mi><mn>0</mn></msub><mo>=</mo><mfrac><mi>Q</mi><msub><mi>V</mi><mn>0</mn></msub></mfrac></math>"#;
    test_braille("Polish", expr, r"⠨⠉⠡⠴⠀⠶⠨⠟⠳⠨⠧⠡⠴")?;
    return Ok(());
}

#[test]
fn physics_p60_4() -> Result<()> {
    let expr = r#"<math><msub><mi>U</mi><mrow><mi>s</mi><mi>k</mi></mrow></msub><mo>=</mo><mfrac><msub><mi>U</mi><mn>0</mn></msub><msqrt><mn>2</mn></msqrt></mfrac></math>"#;
    test_braille("Polish", expr, "⠨⠥⠡⠠⠎⠅⠀⠶⠨⠥⠡⠴⠳⠩⠼⠃")?;
    return Ok(());
}

#[test]
fn physics_p60_5() -> Result<()> {
    let expr = r#"<math><msub><mi>U</mi><mrow><mi>s</mi><mi>k</mi></mrow></msub><mo>=</mo><mfrac><msub><mi>U</mi><mn>0</mn></msub><msqrt><mn>2</mn></msqrt></mfrac></math>"#;
    test_braille("Polish", expr, "⠨⠥⠡⠠⠎⠅⠀⠶⠨⠥⠡⠴⠳⠩⠼⠃")?;
    return Ok(());
}

#[test]
fn physics_p60_6() -> Result<()> {
    let expr = r#"<math><mi>x</mi><mo>=</mo><msub><mi>x</mi><mn>0</mn></msub><mo>-</mo><mi>v</mi><mi>t</mi></math>"#;
    test_braille("Polish", expr, "⠠⠭⠀⠶⠭⠡⠴⠀⠤⠧⠞")?;
    return Ok(());
}

#[test]
fn physics_p60_7() -> Result<()> {
    let expr = r#"<math><mfrac><msub><mi>s</mi><mn>1</mn></msub><msub><mi>s</mi><mn>2</mn></msub></mfrac><mo>=</mo><msup><mfenced><mfrac><msub><mi>t</mi><mn>1</mn></msub><msub><mi>t</mi><mn>2</mn></msub></mfrac></mfenced><mn>2</mn></msup></math>"#;
    test_braille("Polish", expr, "⠠⠎⠡⠂⠳⠎⠡⠆⠀⠶⠣⠞⠡⠂⠳⠞⠡⠆⠜⠬⠆")?;
    return Ok(());
}

#[test]
#[ignore = "alternative version"]
fn physics_p61_1() -> Result<()> {
    let expr = r#"<math><msub><mi>I</mi><mn>1</mn></msub><mo>=</mo><mfrac><mrow><msub><mi>I</mi><mn>0</mn></msub><msub><mi>R</mi><mn>1</mn></msub></mrow><mrow><msub><mi>R</mi><mn>1</mn></msub><mo>+</mo><msub><mi>R</mi><mn>2</mn></msub></mrow></mfrac></math>"#;
    test_braille("Polish", expr, "⠨⠊⠡⠂⠀⠶⠊⠡⠴⠗⠡⠂⠳⠗⠡⠂⠈⠖⠗⠡⠆")?;
    return Ok(());
}

// or

#[test]
fn physics_p61_1a() -> Result<()> {
    let expr = r#"<math><msub><mi>I</mi><mn>1</mn></msub><mo>=</mo><mfrac><mrow><msub><mi>I</mi><mn>0</mn></msub><msub><mi>R</mi><mn>1</mn></msub></mrow><mrow><msub><mi>R</mi><mn>1</mn></msub><mo>+</mo><msub><mi>R</mi><mn>2</mn></msub></mrow></mfrac></math>"#;
    test_braille("Polish", expr, "⠨⠊⠡⠂⠀⠶⠨⠊⠡⠴⠨⠗⠡⠂⠳⠨⠗⠡⠂⠈⠖⠨⠗⠡⠆")?;
    return Ok(());
}

// or

#[test]
#[ignore = "alternative version"]
fn physics_p61_1b() -> Result<()> {
    let expr = r#"<math><msub><mi>I</mi><mn>1</mn></msub><mo>=</mo><mfrac><mrow><msub><mi>I</mi><mn>0</mn></msub><msub><mi>R</mi><mn>1</mn></msub></mrow><mrow><msub><mi>R</mi><mn>1</mn></msub><mo>+</mo><msub><mi>R</mi><mn>2</mn></msub></mrow></mfrac></math>"#;
    test_braille("Polish", expr, "⠨⠊⠡⠂⠀⠶⠆⠨⠊⠡⠴⠨⠗⠡⠂⠀⠳⠀⠨⠗⠡⠂⠈⠖⠨⠗⠡⠆⠰")?;
    return Ok(());
}

#[test]
#[ignore = "alternative version"]
fn physics_p61_2() -> Result<()> {
    let expr = r#"<math><mi>&#x3B7;</mi><mo>=</mo><mfrac><mrow><msub><mi>T</mi><mn>1</mn></msub><mo>-</mo><msub><mi>T</mi><mn>2</mn></msub></mrow><msub><mi>T</mi><mn>1</mn></msub></mfrac></math>"#;
    test_braille("Polish", expr, "⠰⠱⠀⠶⠨⠞⠡⠂⠈⠤⠞⠡⠆⠳⠞⠡⠂")?;
    return Ok(());
}

// or

#[test]
fn physics_p61_2a() -> Result<()> {
    let expr = r#"<math><mi>&#x3B7;</mi><mo>=</mo><mfrac><mrow><msub><mi>T</mi><mn>1</mn></msub><mo>-</mo><msub><mi>T</mi><mn>2</mn></msub></mrow><msub><mi>T</mi><mn>1</mn></msub></mfrac></math>"#;
    test_braille("Polish", expr, "⠰⠱⠀⠶⠨⠞⠡⠂⠈⠤⠨⠞⠡⠆⠳⠨⠞⠡⠂")?;
    return Ok(());
}

// or

#[test]
#[ignore = "alternative version"]
fn physics_p61_2b() -> Result<()> {
    let expr = r#"<math><mi>&#x3B7;</mi><mo>=</mo><mfrac><mrow><msub><mi>T</mi><mn>1</mn></msub><mo>-</mo><msub><mi>T</mi><mn>2</mn></msub></mrow><msub><mi>T</mi><mn>1</mn></msub></mfrac></math>"#;
    test_braille("Polish", expr, "⠰⠱⠀⠶⠆⠨⠞⠡⠂⠀⠤⠨⠞⠡⠆⠀⠳⠀⠨⠞⠡⠂⠰")?;
    return Ok(());
}

#[test]
#[ignore = "alternative version that does not use capital letter redundancy"]
fn physics_p61_3() -> Result<()> {
    let expr = r#"<math><msub><mi>W</mi><mrow><mi>A</mi><mi>B</mi></mrow></msub><mo>=</mo><mo>-</mo><mi>G</mi><mi>M</mi><mi>m</mi><mfenced><mrow><mfrac><mn>1</mn><msub><mi>r</mi><mi>A</mi></msub></mfrac><mo>-</mo><mfrac><mn>1</mn><msub><mi>r</mi><mi>B</mi></msub></mfrac></mrow></mfenced></math>"#;
    test_braille("Polish", expr, "⠨⠺⠡⠁⠃⠀⠶⠤⠛⠍⠠⠍⠣⠼⠁⠳⠗⠡⠨⠁⠀⠤⠼⠁⠳⠠⠗⠡⠨⠃⠜")?;
    return Ok(());
}

// or

#[test]
fn physics_p61_3a() -> Result<()> {
    let expr = r#"<math><msub><mi>W</mi><mrow><mi>A</mi><mi>B</mi></mrow></msub><mo>=</mo><mo>-</mo><mi>G</mi><mi>M</mi><mi>m</mi><mfenced><mrow><mfrac><mn>1</mn><msub><mi>r</mi><mi>A</mi></msub></mfrac><mo>-</mo><mfrac><mn>1</mn><msub><mi>r</mi><mi>B</mi></msub></mfrac></mrow></mfenced></math>"#;
    // Removed the unneeded cap indicator before 'M' in -GM
    test_braille("Polish", expr, "⠨⠺⠡⠨⠁⠃⠀⠶⠤⠨⠛⠍⠠⠍⠣⠼⠁⠳⠗⠡⠨⠁⠀⠤⠼⠁⠳⠠⠗⠡⠨⠃⠜")?;
    return Ok(());
}

#[test]
fn physics_p62_1() -> Result<()> {
    let expr = r#"<math><mover><mi>E</mi><mo>&#xAF;</mo></mover><mo>=</mo><mfrac><mn>1</mn><mn>2</mn></mfrac><mi>m</mi><msup><mover><mi>v</mi><mo>&#xAF;</mo></mover><mn>2</mn></msup></math>"#;
    test_braille("Polish", expr, "⠨⠑⠒⠀⠶⠼⠁⠆⠠⠍⠧⠒⠬⠆")?;
    return Ok(());
}

#[test]
fn physics_p62_2() -> Result<()> {
    let expr = r#"<math><mover><mi>ν</mi><mo>~</mo></mover></math>"#;
    test_braille("Polish", expr, "⠰⠝⠢")?;
    return Ok(());
}

#[test]
fn physics_p62_3() -> Result<()> {
    let expr = r#"<math><msup><mi>&#x3C0;</mi><mo>*</mo></msup></math>"#;
    test_braille("Polish", expr, "⠰⠏⠲")?;
    return Ok(());
}

#[test]
fn physics_p62_4() -> Result<()> {
    let expr = r#"<math><mn>1</mn><mo>&#x2192;</mo><mn>2</mn></math>"#;
    test_braille("Polish", expr, "⠼⠁⠀⠒⠂⠼⠃")?;
    return Ok(());
}

#[test]
fn physics_p63_1() -> Result<()> {
    let expr = r#"<math><msub><mi>W</mi><mrow><mn>2</mn><mo>&#x2192;</mo><mn>3</mn></mrow></msub><mo>=</mo><mo>-</mo><msub><mi>W</mi><mrow><mn>4</mn><mo>&#x2192;</mo><mn>1</mn></mrow></msub></math>"#;
    // redundant cap letters
    test_braille("Polish", expr, r"⠨⠺⠡⠼⠃⠈⠒⠂⠼⠉⠀⠶⠤⠨⠺⠡⠼⠙⠈⠒⠂⠼⠁")?;
    return Ok(());
}

#[test]
fn physics_p63_2() -> Result<()> {
    let expr = r#"<math><msub><mi>W</mi><mn>2</mn></msub><mo>&#x2192;</mo><mn>3</mn><mo>=</mo>
                                      <mo>-</mo><msub><mi>W</mi><mn>4</mn></msub><mo>&#x2192;</mo><mn>1</mn></math>"#;
    test_braille("Polish", expr, "⠨⠺⠡⠼⠃⠒⠂⠼⠉⠀⠶⠤⠨⠺⠡⠼⠙⠒⠂⠼⠁")?;
    return Ok(());
}

#[test]
fn physics_p63_3() -> Result<()> {
    let expr = r#"<math><mover accent="true"><mi>F</mi><mo>&#x2192;</mo></mover><mo>=</mo><mi>m</mi><mover accent="true"><mi>a</mi><mo>&#x2192;</mo></mover></math>"#;
    test_braille("Polish", expr, "⠨⠒⠂⠨⠋⠀⠶⠠⠍⠨⠒⠂⠁")?;
    return Ok(());
}

#[test]
fn physics_p63_4() -> Result<()> {
    let expr = r#"<math><mover><mrow><mo>&#x2206;</mo><mi>p</mi></mrow><mo>&#x2192;</mo></mover></math>"#;
    test_braille("Polish", expr, "⠨⠒⠂⠸⠙⠠⠏")?;
    return Ok(());
}

#[test]
fn physics_p63_5() -> Result<()> {
    let expr = r#"<math><mover><msub><mi>F</mi><mrow><mi>A</mi><mi>B</mi></mrow></msub><mo>&#x2192;</mo></mover><mo>=</mo><mo>-</mo><mover><msub><mi>F</mi><mrow><mi>B</mi><mi>A</mi></mrow></msub><mo>&#x2192;</mo></mover></math>"#;
    // redundant cap letters
    test_braille("Polish", expr, r"⠨⠒⠂⠨⠋⠡⠨⠁⠃⠀⠶⠤⠨⠒⠂⠨⠋⠡⠨⠃⠁")?;
    return Ok(());
}

#[test]
fn physics_p64_1() -> Result<()> {
    let expr = r#"<math><mover accent="true"><msub><mi>F</mi><mrow><mn>1</mn><mo>,</mo><mn>2</mn></mrow></msub><mo>&#x2192;</mo></mover><mo>=</mo><mo>-</mo><mover accent="true"><msub><mi>F</mi><mrow><mn>2</mn><mo>,</mo><mn>1</mn></mrow></msub><mo>&#x2192;</mo></mover></math>"#;
    // added second cap indicator for F as per discussion
    test_braille("Polish", expr, r"⠨⠒⠂⠨⠋⠡⠼⠁⠠⠂⠈⠼⠃⠀⠶⠤⠨⠒⠂⠨⠋⠡⠼⠃⠠⠂⠈⠼⠁")?;
    return Ok(());
}

#[test]
fn physics_p64_2() -> Result<()> {
    let expr = r#"<math><mover><mrow><mo>&#x2206;</mo><mi>r</mi></mrow><mo>&#x2192;</mo></mover><mo>=</mo><mover><msub><mi>r</mi><mn>2</mn></msub><mo>&#x2192;</mo></mover><mo>-</mo><mover><msub><mi>r</mi><mn>1</mn></msub><mo>&#x2192;</mo></mover></math>"#;
    test_braille("Polish", expr, "⠨⠒⠂⠸⠙⠠⠗⠀⠶⠨⠒⠂⠗⠡⠆⠀⠤⠨⠒⠂⠗⠡⠂")?;
    return Ok(());
}

#[test]
fn physics_p64_3() -> Result<()> {
    let expr = r#"<math><mo>|</mo><mover accent="true"><msub><mi>r</mi><mn>2</mn></msub><mo>&#x2192;</mo></mover><mo>|</mo><mo>=</mo><msub><mi>x</mi><mn>2</mn></msub></math>"#;
    test_braille("Polish", expr, "⠈⠇⠨⠒⠂⠠⠗⠡⠆⠸⠀⠶⠭⠡⠆")?;
    return Ok(());
}

#[test]
fn physics_p64_4() -> Result<()> {
    let expr = r#"<math><mover accent="true"><mi>a</mi><mo>&#x2192;</mo></mover><mo>=</mo><mfrac><mstyle displaystyle="true"><mover><mrow><mi>&#x394;</mi><mi>v</mi></mrow><mo>&#x2192;</mo></mover></mstyle><mrow><mi>&#x394;</mi><mi>t</mi></mrow></mfrac></math>"#;
    test_braille("Polish", expr, "⠨⠒⠂⠠⠁⠀⠶⠨⠒⠂⠸⠙⠠⠧⠳⠸⠙⠠⠞")?;
    return Ok(());
}

#[test]
fn physics_p64_5() -> Result<()> {
    let expr = r#"<math><mover accent="true"><mrow><mi>&#x394;</mi><msub><mi>p</mi><mn>1</mn></msub></mrow><mo>&#x2192;</mo></mover><mo>=</mo><msub><mi>m</mi><mn>1</mn></msub><mover accent="true"><mrow><mi>&#x394;</mi><mi>v</mi></mrow><mo>&#x2192;</mo></mover></math>"#;
    test_braille("Polish", expr, "⠨⠒⠂⠸⠙⠠⠏⠡⠂⠀⠶⠍⠡⠂⠨⠒⠂⠸⠙⠠⠧")?;
    return Ok(());
}

#[test]
fn physics_p64_6() -> Result<()> {
    let expr = r#"<math><mover accent="true"><mi>E</mi><mo>&#x2192;</mo></mover><mo>=</mo><mfrac><mover accent="true"><mi>F</mi><mo>&#x2192;</mo></mover><msub><mi>q</mi><mn>0</mn></msub></mfrac></math>"#;
    test_braille("Polish", expr, r"⠨⠒⠂⠨⠑⠀⠶⠨⠒⠂⠨⠋⠳⠠⠟⠡⠴")?;
    return Ok(());
}

#[test]
fn physics_p64_7() -> Result<()> {
    let expr = r#"<math><mover accent="true"><mi>M</mi><mo>&#x2192;</mo></mover><mo>=</mo><mover accent="true"><mi>F</mi><mo>&#x2192;</mo></mover><mo>&#xD7;</mo><mover accent="true"><mi>r</mi><mo>&#x2192;</mo></mover></math>"#;
    test_braille("Polish", expr, r"⠨⠒⠂⠨⠍⠀⠶⠨⠒⠂⠨⠋⠀⠦⠨⠒⠂⠠⠗")?;
    return Ok(());
}

#[test]
fn physics_p64_8() -> Result<()> {
    let expr = r#"<math><mi>W</mi><mo>=</mo><mover accent="true"><mi>F</mi><mo>&#x2192;</mo></mover><mo>&#xB7;</mo><mover accent="true"><mi>s</mi><mo>&#x2192;</mo></mover></math>"#;
    test_braille("Polish", expr, r"⠨⠺⠀⠶⠨⠒⠂⠨⠋⠀⠄⠨⠒⠂⠠⠎")?;
    return Ok(());
}

#[test]
fn physics_p64_9() -> Result<()> {
    let expr = r#"<math><msub><mi>N</mi><mn>0</mn></msub><mover><mo>&#x2192;</mo><msub><mi>T</mi><mrow><mn>1</mn><mo>/</mo><mn>2</mn></mrow></msub></mover><mfrac><msub><mi>N</mi><mn>0</mn></msub><mn>2</mn></mfrac><mover><mo>&#x2192;</mo><msub><mi>T</mi><mrow><mn>1</mn><mo>/</mo><mn>2</mn></mrow></msub></mover><mfrac><msub><mi>N</mi><mn>0</mn></msub><mn>4</mn></mfrac><mover><mo>&#x2192;</mo><msub><mi>T</mi><mrow><mn>1</mn><mo>/</mo><mn>2</mn></mrow></msub></mover><mfrac><msub><mi>N</mi><mn>0</mn></msub><mn>8</mn></mfrac></math>"#;
    test_braille("Polish", expr, "⠨⠝⠡⠴⠀⠰⠒⠂⠠⠨⠞⠡⠼⠁⠆⠄⠀⠝⠡⠴⠳⠆⠀⠰⠒⠂⠠⠨⠞⠡⠈⠼⠁⠆⠄⠀⠝⠡⠴⠳⠲⠀⠰⠒⠂⠠⠨⠞⠡⠼⠁⠆⠄⠀⠝⠡⠴⠳⠦")?;
    return Ok(());
}

// or

#[test]
fn physics_p65_1() -> Result<()> {
    let expr = r#"<math><msub><mi>N</mi><mn>0</mn></msub><mover><mo>&#x2192;</mo><msub><mi>T</mi><mrow><mn>1</mn><mo>/</mo><mn>2</mn></mrow></msub></mover><mfrac><msub><mi>N</mi><mn>0</mn></msub><mn>2</mn></mfrac><mover><mo>&#x2192;</mo><msub><mi>T</mi><mrow><mn>1</mn><mo>/</mo><mn>2</mn></mrow></msub></mover><mfrac><msub><mi>N</mi><mn>0</mn></msub><mn>4</mn></mfrac><mover><mo>&#x2192;</mo><msub><mi>T</mi><mrow><mn>1</mn><mo>/</mo><mn>2</mn></mrow></msub></mover><mfrac><msub><mi>N</mi><mn>0</mn></msub><mn>8</mn></mfrac></math>"#;
    test_braille("Polish", expr, "⠨⠝⠡⠴⠀⠰⠒⠂⠠⠨⠞⠡⠼⠁⠆⠄⠀⠨⠝⠡⠴⠳⠆⠀⠰⠒⠂⠠⠨⠞⠡⠼⠁⠆⠄⠀⠨⠝⠡⠴⠳⠲⠀⠰⠒⠂⠠⠨⠞⠡⠼⠁⠆⠄⠀⠨⠝⠡⠴⠳⠦")?;
    return Ok(());
}

#[test]
fn physics_p65_2() -> Result<()> {
    let expr = r#"<math><mmultiscripts><mi>Po</mi><mprescripts/><mn>84</mn><mn>215</mn></mmultiscripts><msubsup><mover><mo>&#x2192;</mo><mi>&#x3B1;</mi></mover><mn>82</mn><mn>211</mn></msubsup><mi>Pb</mi><msubsup><mo>+</mo><mn>2</mn><mn>4</mn></msubsup><mi>He</mi><mo>+</mo><mn>7</mn><mo>,</mo><mn>4</mn><mi>MeV</mi></math>"#;
    test_braille("Polish", expr, "⠌⠆⠂⠢⠡⠦⠲⠨⠏⠕⠀⠰⠒⠂⠰⠁⠀⠌⠆⠂⠂⠡⠦⠆⠨⠏⠃⠀⠖⠌⠲⠡⠆⠨⠓⠑⠀⠖⠼⠛⠂⠙⠻⠨⠍⠑⠨⠧")?;
    return Ok(());
}

#[test]
fn physics_p65_3() -> Result<()> {
    let expr = r#"<math><mmultiscripts><mi>C</mi><mprescripts/><mn>6</mn><mn>14</mn></mmultiscripts><mo>&#x2192;</mo><mmultiscripts><mi>N</mi><mprescripts/><mn>7</mn><mn>14</mn></mmultiscripts><mo>+</mo><mmultiscripts><mi mathvariant="double-struck">&#x3B2;</mi><mprescripts/><mrow><mo>-</mo><mn>1</mn></mrow><mn>0</mn></mmultiscripts><mo>+</mo><mmultiscripts><mover><mi>v</mi><mo>~</mo></mover><mprescripts/><mn>0</mn><mn>0</mn></mmultiscripts></math>"#;
    test_braille("Polish", expr, "⠌⠂⠲⠡⠖⠨⠉⠀⠒⠂⠌⠂⠲⠡⠶⠨⠝⠀⠖⠌⠴⠡⠤⠂⠰⠃⠀⠖⠌⠴⠡⠴⠰⠝⠢")?;
    return Ok(());
}

#[test]
fn physics_p65_4() -> Result<()> {
    let expr = r#"<math><mmultiscripts><mi mathvariant="normal">n</mi><mprescripts/><mn>0</mn><mn>1</mn></mmultiscripts><mover><mo>&#x2192;</mo><msup><mi mathvariant="normal">&#x3B2;</mi><mo>-</mo></msup></mover><mmultiscripts><mrow><msub><mrow/><mrow/></msub><mi mathvariant="normal">p</mi></mrow><mprescripts/><mn>1</mn><mn>1</mn></mmultiscripts><msubsup><mo>+</mo><mrow><mo>-</mo><mn>1</mn></mrow><mn>0</mn></msubsup><mi mathvariant="normal">&#x3B2;</mi><msubsup><mo>+</mo><mn>0</mn><mn>0</mn></msubsup><mover><mi mathvariant="normal">v</mi><mo>~</mo></mover></math>"#;
    test_braille("Polish", expr, "⠌⠂⠡⠴⠠⠝⠀⠰⠒⠂⠰⠃⠤⠀⠌⠂⠡⠂⠠⠏⠀⠖⠌⠴⠡⠤⠂⠈⠰⠃⠀⠖⠌⠴⠡⠴⠰⠝⠢")?;
    return Ok(());
}

#[test]
fn physics_p66_1() -> Result<()> {
    let expr = r#"<math><msub><mi>v</mi><mn>1</mn></msub></math>"#;
    test_braille("Polish", expr, "⠠⠧⠡⠂")?;
    return Ok(());
}

#[test]
fn physics_p66_2() -> Result<()> {
    let expr = r#"<math><msubsup><mi>v</mi><mn>0</mn><mn>2</mn></msubsup></math>"#;
    test_braille("Polish", expr, "⠠⠧⠡⠴⠬⠆")?;
    return Ok(());
}

#[test]
fn physics_p66_3() -> Result<()> {
    let expr = r#"<math><mfrac><mi>R</mi><mn>2</mn></mfrac></math>"#;
    test_braille("Polish", expr, "⠨⠗⠳⠆")?;
    return Ok(());
}

#[test]
fn physics_p66_4() -> Result<()> {
    let expr = r#"<math><mfrac><mrow><mi>m</mi><msup><mover><mi>v</mi><mo>&#xAF;</mo></mover><mn>2</mn></msup></mrow><mn>2</mn></mfrac></math>"#;
    test_braille("Polish", expr, "⠠⠍⠧⠒⠬⠆⠳⠆")?;
    return Ok(());
}

#[test]
fn physics_p66_5() -> Result<()> {
    let expr = r#"<math><mfrac><mrow><msub><mi>T</mi><mn>1</mn></msub><mo>-</mo><msub><mi>T</mi><mn>2</mn></msub></mrow><msub><mi>T</mi><mn>1</mn></msub></mfrac></math>"#;
    test_braille_prefs("Polish", vec![("Polish_BrailleLevel", "Beginner")], expr, r"⠆⠨⠞⠡⠂⠀⠤⠨⠞⠡⠆⠀⠳⠀⠨⠞⠡⠂⠰")?;
    return Ok(());
}

// or

#[test]
fn physics_p66_5a() -> Result<()> {
    let expr = r#"<math><mfrac><mrow><msub><mi>T</mi><mn>1</mn></msub><mo>-</mo><msub><mi>T</mi><mn>2</mn></msub></mrow><msub><mi>T</mi><mn>1</mn></msub></mfrac></math>"#;
    test_braille("Polish", expr, "⠨⠞⠡⠂⠈⠤⠨⠞⠡⠆⠳⠨⠞⠡⠂")?;
    return Ok(());
}

#[test]
fn physics_p66_6() -> Result<()> {
    let expr = r#"<math><msub><mi>v</mi><mi>k</mi></msub><mo>=</mo><msqrt><mn>2</mn><mi>g</mi><mi>h</mi></msqrt></math>"#;
    test_braille("Polish", expr, "⠠⠧⠡⠅⠀⠶⠩⠼⠃⠠⠛⠓")?;
    return Ok(());
}

#[test]
fn physics_p67_1() -> Result<()> {
    let expr = r#"<math><mi>t</mi><mo>=</mo><msqrt><mfrac><mrow><mn>2</mn><mi>h</mi></mrow><mi>g</mi></mfrac></msqrt></math>"#;
    test_braille("Polish", expr, "⠠⠞⠀⠶⠐⠩⠼⠃⠠⠓⠳⠛")?;
    return Ok(());
}

#[test]
fn physics_p67_2() -> Result<()> {
    let expr = r#"<math><mi>t</mi><mo>=</mo><mfrac><mrow><msqrt><msup><msub><mi>v</mi><mn>0</mn></msub><mn>2</mn></msup><mo>+</mo><mn>2</mn><mi>g</mi><mi>h</mi></msqrt><mo>-</mo><msub><mi>v</mi><mn>0</mn></msub></mrow><mi>g</mi></mfrac></math>"#;
    test_braille("Polish", expr, "⠠⠞⠀⠶⠆⠐⠩⠧⠡⠴⠬⠆⠈⠖⠼⠃⠠⠛⠓⠀⠤⠧⠡⠴⠀⠳⠀⠛⠰")?;
    return Ok(());
}

// or

#[test]
fn physics_p67_2a() -> Result<()> {
    let expr = r#"<math><mi>t</mi><mo>=</mo><mfrac><mrow><msqrt><msup><msub><mi>v</mi><mn>0</mn></msub><mn>2</mn></msup><mo>+</mo><mn>2</mn><mi>g</mi><mi>h</mi></msqrt><mo>-</mo><msub><mi>v</mi><mn>0</mn></msub></mrow><mi>g</mi></mfrac></math>"#;
    test_braille("Polish", expr, "⠠⠞⠀⠶⠆⠐⠩⠧⠡⠴⠬⠆⠈⠖⠼⠃⠠⠛⠓⠀⠤⠧⠡⠴⠀⠳⠛")?;
    return Ok(());
}

#[test]
fn physics_p67_3() -> Result<()> {
    let expr = r#"<math><msub><mi>h</mi><mrow><mi>m</mi><mi>a</mi><mi>x</mi></mrow></msub><mo>=</mo><mfrac><msubsup><mi>v</mi><mrow><mn>0</mn><mi>y</mi></mrow><mn>2</mn></msubsup><mrow><mn>2</mn><mi>g</mi></mrow></mfrac><mo>=</mo><mfrac><mrow><mn>2</mn><msub><mi>v</mi><mi>o</mi></msub><mo>-</mo><msup><mi>sin</mi><mn>2</mn></msup><mi>&#x3B1;</mi></mrow><mrow><mn>2</mn><mi>g</mi></mrow></mfrac></math>"#;
    test_braille("Polish", expr, "⠠⠓⠡⠼⠭⠀⠶⠧⠡⠼⠚⠽⠬⠆⠳⠼⠃⠠⠛⠀⠶⠆⠼⠃⠧⠡⠴⠈⠫⠎⠬⠆⠰⠁⠳⠼⠃⠠⠛")?;
    return Ok(());
}

#[test]
fn physics_p68_1() -> Result<()> {
    let expr = r#"<math><mi>F</mi><mo>=</mo><mfrac><mn>1</mn><mrow><mn>4</mn><mi>&#x3C0;</mi><msub><mi>&#x3B5;</mi><mn>0</mn></msub></mrow></mfrac><mo>&#xB7;</mo><mfrac><mrow><mi>q</mi><msub><mi>q</mi><mn>0</mn></msub></mrow><msup><mi>r</mi><mn>2</mn></msup></mfrac></math>"#;
    test_braille_prefs("Polish", vec![("Polish_BrailleLevel", "Beginner")], expr, r"⠨⠋⠀⠶⠆⠼⠁⠀⠳⠀⠼⠙⠰⠏⠰⠑⠡⠴⠰⠄⠆⠠⠟⠟⠡⠴⠀⠳⠀⠗⠬⠆⠰")?;
    return Ok(());
}

// or

#[test]
fn physics_p68_1a() -> Result<()> {
    let expr = r#"<math><mi>F</mi><mo>=</mo><mfrac><mn>1</mn><mrow><mn>4</mn><mi>&#x3C0;</mi><msub><mi>&#x3B5;</mi><mn>0</mn></msub></mrow></mfrac><mo>&#xB7;</mo><mfrac><mrow><mi>q</mi><msub><mi>q</mi><mn>0</mn></msub></mrow><msup><mi>r</mi><mn>2</mn></msup></mfrac></math>"#;
    test_braille("Polish", expr, "⠨⠋⠀⠶⠼⠁⠳⠼⠙⠰⠏⠰⠑⠡⠴⠀⠄⠠⠟⠟⠡⠴⠳⠗⠬⠆")?;
    return Ok(());
}

#[test]
fn physics_p68_2() -> Result<()> {
    let expr = r#"<math><mi>r</mi><mo>=</mo><msqrt><mfrac><mrow><mn>10</mn><mi>N</mi></mrow><mrow><mi>&#x3C0;</mi><msub><mi>N</mi><mn>0</mn></msub></mrow></mfrac></msqrt></math>"#;
    test_braille("Polish", expr, "⠠⠗⠀⠶⠐⠩⠼⠁⠚⠨⠝⠳⠰⠏⠨⠝⠡⠴")?;
    return Ok(());
}

#[test]
fn physics_p68_3() -> Result<()> {
    let expr = r#"<math><msub><mi>T</mi><mn>2</mn></msub><mo>=</mo><mn>2</mn><mi>&#x3C0;</mi><msqrt><mfrac><mi>l</mi><mrow><mi>g</mi><mo>+</mo><mi>a</mi></mrow></mfrac></msqrt></math>"#;
    test_braille("Polish", expr, "⠨⠞⠡⠆⠀⠶⠼⠃⠰⠏⠐⠩⠠⠇⠳⠛⠈⠖⠁")?;
    return Ok(());
}

#[test]
#[ignore="alternative that doesn't repeat cap indicators"]
fn physics_p68_4() -> Result<()> {
    let expr = r#"<math><msub><mi>U</mi><mn>0</mn></msub><mo>=</mo><msub><mi>U</mi><msub><mi>R</mi><mn>1</mn></msub></msub><mo>+</mo><msub><mi>U</mi><msub><mi>R</mi><mn>2</mn></msub></msub></math>"#;
    test_braille("Polish", expr, "⠨⠥⠡⠴⠀⠶⠥⠐⠡⠗⠡⠂⠀⠖⠥⠐⠡⠗⠡⠆")?;
    return Ok(());
}


#[test]
fn physics_p68_4a() -> Result<()> {
    let expr = r#"<math><msub><mi>U</mi><mn>0</mn></msub><mo>=</mo><msub><mi>U</mi><msub><mi>R</mi><mn>1</mn></msub></msub><mo>+</mo><msub><mi>U</mi><msub><mi>R</mi><mn>2</mn></msub></msub></math>"#;
    test_braille("Polish", expr, "⠨⠥⠡⠴⠀⠶⠨⠥⠐⠡⠨⠗⠡⠂⠀⠖⠨⠥⠐⠡⠨⠗⠡⠆")?;
    return Ok(());
}

#[test]
fn physics_p68_5() -> Result<()> {
    let expr = r#"<math><mi>m</mi><mo>=</mo><mfrac><msub><mi>m</mi><mn>0</mn></msub><msqrt><mn>1</mn><mo>-</mo><msup><mfenced><mfrac><mi>v</mi><mi>c</mi></mfrac></mfenced><mn>2</mn></msup></msqrt></mfrac></math>"#;
    test_braille("Polish", expr, "⠠⠍⠀⠶⠆⠍⠡⠴⠀⠳⠀⠐⠩⠼⠁⠈⠤⠣⠧⠳⠉⠜⠬⠆⠰")?;
    return Ok(());
}

#[test]
fn physics_p68_6() -> Result<()> {
    let expr = r#"<math><mi>N</mi><mo>=</mo><msub><mi>N</mi><mn>0</mn></msub><msup><mi mathvariant="normal">e</mi><mrow><mo>-</mo><mfrac><mrow><mi>ln</mi><mn>2</mn></mrow><msub><mi>T</mi><mrow><mn>1</mn><mo>/</mo><mn>2</mn></mrow></msub></mfrac></mrow></msup></math>"#;
    test_braille("Polish", expr, "⠨⠝⠀⠶⠨⠝⠡⠴⠠⠑⠐⠬⠤⠫⠦⠇⠼⠃⠳⠨⠞⠡⠼⠁⠆⠐⠱")?;
    return Ok(());
}

// UNITS AND OPERATIONS ON UNITS p69

#[test]
fn physics_units_p69_1() -> Result<()> {
    let expr = r#"<math><mi mathvariant="normal" intent=":unit">m</mi><mo>/</mo><msup><mi mathvariant="normal">s</mi><mn>2</mn></msup></math>"#;
    test_braille("Polish", expr, "⠻⠍⠳⠎⠬⠆")?;
    return Ok(());
}

#[test]
fn physics_units_p70_1() -> Result<()> {
    let expr = r#"<math><mi intent=":unit">kg</mi><mo>/</mo><msup><mi mathvariant="normal">m</mi><mn>3</mn></msup></math>"#;
    test_braille("Polish", expr, "⠻⠅⠛⠳⠍⠬⠒")?;
    return Ok(());
}

#[test]
fn physics_units_p72_1() -> Result<()> {
    let expr = r#"<math><mn>1</mn><mi>GW</mi><mo>=</mo><msup><mn>10</mn><mn>9</mn></msup><mi mathvariant="normal" intent=":unit">W</mi></math>"#;
    test_braille_prefs("Polish", vec![("Polish_BrailleLevel", "Advanced")], expr, r"⠼⠁⠻⠨⠛⠨⠺⠀⠶⠼⠁⠚⠬⠔⠻⠨⠺")?;
    return Ok(());
}

#[test]
fn physics_units_p72_2() -> Result<()> {
    let expr = r#"<math><mn>1</mn><mi intent=":unit">kWh</mi><mo>=</mo><mn>3600000</mn><mi intent=":unit">Ws</mi></math>"#;
    test_braille("Polish", expr, "⠼⠁⠻⠅⠨⠺⠓⠀⠶⠼⠉⠄⠋⠚⠚⠄⠚⠚⠚⠻⠨⠺⠎")?;
    return Ok(());
}

#[test]
fn physics_units_p72_3() -> Result<()> {
    let expr = r#"<math><mn>1</mn><mi>nm</mi><mo>=</mo><msup><mn>10</mn><mrow><mo>-</mo><mn>9</mn></mrow></msup><mi mathvariant="normal">m</mi></math>"#;
    test_braille("Polish", expr, "⠼⠁⠻⠝⠍⠀⠶⠼⠁⠚⠬⠤⠔⠻⠍")?;
    return Ok(());
}

#[test]
fn physics_units_p72_4() -> Result<()> {
    let expr = r#"<math><mn>1</mn><mi>k&#x3A9;</mi><mo>=</mo><msup><mn>10</mn><mn>3</mn></msup><mi mathvariant="normal">&#x3A9;</mi></math>"#;
    test_braille("Polish", expr, "⠼⠁⠻⠅⠸⠺⠀⠶⠼⠁⠚⠬⠒⠻⠸⠺")?;
    return Ok(());
}

#[test]
fn physics_units_p73_1() -> Result<()> {
    let expr = r#"<math><mn>1</mn><mi mathvariant="normal">l</mi><mo>=</mo><mn>1</mn><msup><mi intent=":unit">dm</mi><mn>3</mn></msup><mo>=</mo><mn>0</mn><mo>,</mo><mn>001</mn><msup><mi mathvariant="normal" intent=":unit">m</mi><mn>3</mn></msup></math>"#;
    test_braille("Polish", expr, "⠼⠁⠻⠇⠀⠶⠼⠁⠻⠙⠍⠬⠒⠀⠶⠼⠚⠂⠚⠚⠁⠻⠍⠬⠒")?;
    return Ok(());
}

#[test]
fn physics_units_p73_2() -> Result<()> {
    let expr = r#"<math><mn>0</mn><mo>&#xB0;</mo><mi mathvariant="normal">C</mi><mo>=</mo><mn>273</mn><mo>,</mo><mn>15</mn><mi mathvariant="normal">K</mi></math>"#;
    test_braille("Polish", expr, "⠼⠚⠘⠴⠨⠉⠀⠶⠼⠃⠛⠉⠂⠁⠑⠻⠨⠅")?;
    return Ok(());
}

#[test]
fn physics_units_p73_3() -> Result<()> {
    let expr = r#"<math><mn>0</mn><mo>&#xB0;</mo><mi mathvariant="normal">C</mi><mo>=</mo><mn>32</mn><mo>&#xB0;</mo><mi mathvariant="normal">F</mi></math>"#;
    test_braille("Polish", expr, "⠼⠚⠘⠴⠨⠉⠀⠶⠼⠉⠃⠴⠨⠋")?;
    return Ok(());
}

#[test]
fn physics_units_p73_4() -> Result<()> {
    let expr = r#"<math><mfrac><mn>3</mn><mn>4</mn></mfrac><mo>&#xB0;</mo><mi mathvariant="normal">C</mi><mo>=</mo><mn>0</mn><mo>,</mo><mn>75</mn><mo>&#xB0;</mo><mi mathvariant="normal">C</mi></math>"#;
    test_braille("Polish", expr, "⠼⠉⠲⠘⠴⠨⠉⠀⠶⠼⠚⠂⠛⠑⠴⠨⠉")?;
    return Ok(());
}

#[test]
fn physics_units_p73_5() -> Result<()> {
    // spec has '1w.=', but the braille dots don't have anything corresponding to ".", so I removed it
    let expr = r#"<math><mn>1</mn><mi mathvariant="normal">w</mi><mo>=</mo><mn>1</mn><mi intent=":unit">NM</mi><mo>/</mo><mi mathvariant="normal" intent=":unit">h</mi><mo>=</mo>
                                <mn>1852</mn><mi mathvariant="normal" intent=":unit">m</mi><mo>/</mo><mi mathvariant="normal" intent=":unit">h</mi></math>"#;
    test_braille_prefs("Polish", vec![("Polish_BrailleLevel", "Advanced")], expr, r"⠼⠁⠻⠺⠀⠶⠼⠁⠻⠨⠝⠨⠍⠳⠓⠀⠶⠼⠁⠓⠑⠃⠻⠍⠳⠓")?;
    return Ok(());
}

#[test]
fn physics_units_p73_6() -> Result<()> {
    let expr = r#"<math><mn>1</mn><mi>ha</mi><mo>=</mo><mn>10</mn><mo>&#xA0;</mo><mn>000</mn><msup><mi mathvariant="normal" intent=":unit">m</mi><mn>2</mn></msup></math>"#;
    test_braille("Polish", expr, "⠼⠁⠻⠓⠁⠀⠶⠼⠁⠚⠄⠚⠚⠚⠻⠍⠬⠆")?;
    return Ok(());
}

#[test]
fn physics_units_p74_1() -> Result<()> {
    let expr = r#"<math><mn>5</mn><mi>kg</mi></math>"#;
    test_braille("Polish", expr, "⠼⠑⠻⠅⠛")?;
    return Ok(());
}

#[test]
fn physics_units_p74_2() -> Result<()> {
    let expr = r#"<math><mn>2</mn><mi intent=":unit">&#x3BC;m</mi></math>"#;
    test_braille("Polish", expr, "⠼⠃⠻⠰⠍⠠⠍")?;
    return Ok(());
}

#[test]
fn physics_units_p74_3() -> Result<()> {
    let expr = r#"<math><mn>3</mn><mi intent=":unit">kWh</mi></math>"#;
    test_braille("Polish", expr, "⠼⠉⠻⠅⠨⠺⠓")?;
    return Ok(());
}

#[test]
fn physics_units_p74_4() -> Result<()> {
    let expr = r#"<math><mn>5</mn><mi intent=":unit">min</mi></math>"#;
    test_braille("Polish", expr, "⠼⠑⠻⠍⠊⠝")?;
    return Ok(());
}

// or

#[test]
fn physics_units_p74_4a() -> Result<()> {
    let expr = r#"<math><mn>5</mn><mi intent=":unit">min</mi></math>"#;
    test_braille("Polish", expr, "⠼⠑⠻⠍⠊⠝")?;
    return Ok(());
}

#[test]
fn physics_units_p74_5() -> Result<()> {
    let expr = r#"<math><mi mathvariant="normal" intent=":unit">J</mi><mo>=</mo><mi mathvariant="normal" intent=":unit">N</mi><mo>&#xB7;</mo><mi mathvariant="normal" intent=":unit">m</mi></math>"#;
    test_braille("Polish", expr, "⠻⠨⠚⠀⠶⠻⠨⠝⠄⠻⠍")?;
    return Ok(());
}

#[test]
#[ignore = "doubled unit sign"]
fn physics_units_p74_5_doubled_sign() -> Result<()> {
    let expr = r#"<math><mi mathvariant="normal" intent=":unit">J</mi><mo>=</mo><mi mathvariant="normal" intent=":unit">N</mi><mo>&#xB7;</mo><mi mathvariant="normal" intent=":unit">m</mi></math>"#;
    test_braille("Polish", expr, "⠻⠨⠚⠀⠶⠻⠻⠨⠝⠄⠍")?;
    return Ok(());
}

#[test]
#[ignore = "doubled unit sign"]
fn physics_units_p74_6() -> Result<()> {
    let expr = r#"<math><mi mathvariant="normal" intent=":unit">J</mi><mo>=</mo><mi intent=":unit">kg</mi><mo>&#xB7;</mo><mfrac><mi mathvariant="normal" intent=":unit">m</mi><msup><mi mathvariant="normal">s</mi><mn>2</mn></msup></mfrac><mo>&#xB7;</mo><mi mathvariant="normal">m</mi><mo>=</mo><mfrac><mrow><mi>kg</mi><mo>&#xB7;</mo><msup><mi mathvariant="normal">m</mi><mn>2</mn></msup></mrow><msup><mi mathvariant="normal">s</mi><mn>2</mn></msup></mfrac></math>"#;
    test_braille("Polish", expr, "⠻⠨⠚⠀⠶⠻⠅⠛⠄⠻⠍⠳⠎⠬⠆⠀⠄⠻⠍⠀⠶⠻⠻⠅⠛⠄⠍⠬⠆⠳⠎⠬⠆")?;
    return Ok(());
}

#[test]
fn units_currency_p75_1() -> Result<()> {
    let expr = r#"<math><mn>5</mn><mi>z&#x142;</mi></math>"#;
    test_braille("Polish", expr, "⠼⠑⠀⠵⠣")?;
    return Ok(());
}

#[test]
fn units_currency_p75_2() -> Result<()> {
    let expr = r#"<math><mn>3</mn><mi>z&#x142;</mi><mo>&#xA0;</mo><mn>50</mn><mi>gr</mi></math>"#;
    test_braille("Polish", expr, "⠼⠉⠀⠵⠣⠀⠼⠑⠚⠀⠛⠗")?;
    return Ok(());
}

#[test]
fn units_p75_3() -> Result<()> {
    let expr = r#"<math><mi intent=":unit">J</mi><mo>&#xA0;</mo><mo>=</mo>
                                <mo>&#xA0;</mo><mi intent=":unit">W</mi><mo>&#x22C5;</mo><mi intent=":unit">s</mi></math>"#;
    test_braille("Polish", expr, "⠻⠨⠚⠀⠶⠻⠨⠺⠄⠻⠎")?;
    return Ok(());
}

#[test]
#[ignore = "uses space before multiplication dot when not needed"]
fn units_p75_4() -> Result<()> {
    let expr = r#"<math><mi mathvariant="normal" intent=":unit">N</mi><mo>=</mo>
                            <mi intent=":unit">kg</mi><mo>&#xB7;</mo>
                            <mfrac><mi mathvariant="normal" intent=":unit">m</mi><msup><mi mathvariant="normal" intent=":unit">s</mi><mn>2</mn></msup></mfrac><mo>=</mo>
                            <mfrac><mrow><mi intent=":unit">kg</mi><mi intent=":unit">m</mi></mrow><msup><mi mathvariant="normal" intent=":unit">s</mi><mn>2</mn></msup></mfrac></math>"#;
    test_braille("Polish", expr, "⠻⠨⠝⠀⠶⠻⠅⠛⠀⠄⠻⠍⠳⠎⠬⠆⠀⠶⠻⠅⠛⠍⠳⠎⠬⠆")?;
    return Ok(());
}

// or

#[test]
#[ignore = "uses a doubled units indicator (⠻⠻) for no apparent reason other than to say it is legal"]
fn units_p75_4b() -> Result<()> {
    let expr = r#"<math><mi mathvariant="normal" intent=":unit">N</mi><mo>=</mo>
                            <mi intent=":unit">kg</mi><mo>&#xB7;</mo>
                            <mfrac><mi mathvariant="normal" intent=":unit">m</mi><msup><mi mathvariant="normal" intent=":unit">s</mi><mn>2</mn></msup></mfrac><mo>=</mo>
                            <mfrac><mrow><mi intent=":unit">kg</mi><mi intent=":unit">m</mi></mrow><msup><mi mathvariant="normal" intent=":unit">s</mi><mn>2</mn></msup></mfrac></math>"#;
    test_braille("Polish", expr, "⠻⠨⠝⠀⠶⠻⠻⠅⠛⠄⠍⠳⠎⠬⠆⠀⠶⠻⠅⠛⠍⠳⠎⠬⠆")?;
    return Ok(());
}

#[test]
fn units_p75_5() -> Result<()> {
    let expr = r#"<math><mn>36</mn><mo>,</mo><mn>6</mn><mo>&#xB0;</mo><mi mathvariant="normal">C</mi></math>"#;
    test_braille("Polish", expr, "⠼⠉⠋⠂⠋⠴⠨⠉")?;
    return Ok(());
}

#[test]
fn units_p75_6() -> Result<()> {
    let expr = r#"<math><mn>2</mn><mfrac><mn>2</mn><mn>5</mn></mfrac><mo>&#xB0;</mo><mi>C</mi></math>"#;
    // the spec lists two forms for degrees Celsius; MathCAT is generating the first one
    test_braille("Polish", expr, "⠼⠃⠼⠃⠢⠘⠴⠨⠉")?;
    return Ok(());
}

#[test]
fn units_p76_1() -> Result<()> {
    let expr = r#"<math><mi>a</mi><mo>=</mo><mn>10</mn><mi>cm</mi><mo>=</mo><mn>0</mn><mo>,</mo><mn>1</mn><mi mathvariant="normal">m</mi></math>"#;
    test_braille("Polish", expr, "⠠⠁⠀⠶⠼⠁⠚⠻⠉⠍⠀⠶⠼⠚⠂⠁⠻⠍")?;
    return Ok(());
}

#[test]
fn units_p76_2() -> Result<()> {
    let expr = r#"<math><mi mathvariant="normal">d</mi><mo>=</mo><mn>0</mn><mo>,</mo><mn>2</mn><mi>mm</mi><mo>=</mo>
                <mn>2</mn><mo>&#xB7;</mo><msup><mn>10</mn><mrow><mo>-</mo><mn>4</mn></mrow></msup><mi mathvariant="normal">m</mi></math>"#;
    test_braille("Polish", expr, "⠠⠙⠀⠶⠼⠚⠂⠃⠻⠍⠍⠀⠶⠼⠃⠄⠼⠁⠚⠬⠤⠲⠻⠍")?;
    return Ok(());
}

#[test]
fn units_p76_3() -> Result<()> {
    let expr = r#"<math><mi mathvariant="normal">C</mi><mo>=</mo>
            <mfrac><mrow><msub><mi mathvariant="normal">&#x3B5;</mi><mn>0</mn></msub><mi mathvariant="normal">S</mi></mrow><mi mathvariant="normal">d</mi></mfrac><mo>=</mo>
            <mfrac><mrow><mn>8</mn><mo>,</mo><mn>85</mn><mo>&#xB7;</mo><msup><mn>10</mn><mrow><mo>-</mo><mn>12</mn></mrow></msup><mfrac><mi mathvariant="normal" intent=":unit">F</mi><mi mathvariant="normal" intent=":unit">m</mi></mfrac><mo>&#xB7;</mo><msup><mn>10</mn><mrow><mo>-</mo><mn>2</mn></mrow></msup><msup><mi mathvariant="normal" intent=":unit">m</mi><mn>2</mn></msup></mrow><mrow><mn>2</mn><mo>&#xB7;</mo><msup><mn>10</mn><mrow><mo>-</mo><mn>4</mn></mrow></msup><mi mathvariant="normal" intent=":unit">m</mi></mrow></mfrac><mo>&#x2248;</mo><mn>0</mn><mo>,</mo><mn>44</mn><mo>&#xB7;</mo><msup><mn>10</mn><mrow><mo>-</mo><mn>9</mn></mrow></msup><mi mathvariant="normal" intent=":unit">F</mi><mo>=</mo>
            <mn>0</mn><mo>,</mo><mn>44</mn><mi>nF</mi></math>"#;
    test_braille("Polish", expr, "⠨⠉⠀⠶⠰⠑⠡⠴⠨⠎⠳⠠⠙⠀⠶⠆⠼⠓⠂⠓⠑⠄⠼⠁⠚⠬⠤⠂⠆⠻⠨⠋⠳⠍⠄⠼⠁⠚⠬⠤⠆⠻⠍⠬⠆⠀⠳⠀⠼⠃⠄⠼⠁⠚⠬⠤⠲⠈⠻⠍⠰⠀⠢⠢⠼⠚⠂⠙⠙⠄⠼⠁⠚⠬⠤⠔⠻⠨⠋⠀⠶⠼⠚⠂⠙⠙⠻⠝⠨⠋")?;
    return Ok(());
}

// CHEMISTRY p77

#[test]
fn chemistry_molecules_p77_1() -> Result<()> {
    let expr = r#"<math><mi>HCl</mi></math>"#;
    test_braille("Polish", expr, "⠨⠓⠨⠉⠇")?;
    return Ok(());
}

#[test]
fn chemistry_molecules_p77_2() -> Result<()> {
    let expr = r#"<math><mi>CaO</mi></math>"#;
    test_braille("Polish", expr, "⠨⠉⠁⠨⠕")?;
    return Ok(());
}

#[test]
fn chemistry_molecules_p77_3a() -> Result<()> {
    let expr = r#"<math><mi>KOH</mi></math>"#;
    test_braille("Polish", expr, "⠸⠅⠕⠓")?;
    return Ok(());
}


#[test]
fn chemistry_molecules_p77_3b() -> Result<()> {
    let expr = r#"<math><mi>KOH</mi></math>"#;
    test_braille_prefs("Polish", vec![("Polish_BrailleLevel", "Beginner")], expr, r"⠨⠅⠨⠕⠨⠓")?;
    return Ok(());
}

#[test]
fn chemistry_molecules_p77_4a() -> Result<()> {
    let expr = r#"<math><mi>HCOOH</mi></math>"#;
    test_braille("Polish", expr, "⠸⠓⠉⠕⠕⠓")?;
    return Ok(());
}

#[test]
fn chemistry_molecules_p77_4b() -> Result<()> {
    let expr = r#"<math><mi>HCOOH</mi></math>"#;
    test_braille_prefs("Polish", vec![("Polish_BrailleLevel", "Beginner")], expr, r"⠨⠓⠨⠉⠨⠕⠨⠕⠨⠓")?;
    return Ok(());
}

#[test]
fn stoichiometric_p78_1() -> Result<()> {
    let expr = r#"<math><msub><mi>CaSO</mi><mn>4</mn></msub></math>"#;
    test_braille_prefs("Polish", vec![("Polish_BrailleLevel", "Beginner")], expr, r"⠨⠉⠁⠨⠎⠨⠕⠲")?;
    return Ok(());
}

#[test]
fn stoichiometric_p78_2() -> Result<()> {
    let expr = r#"<math><msub><mi>Fe</mi><mn>2</mn></msub><msub><mi>O</mi><mn>3</mn></msub></math>"#;
    test_braille("Polish", expr, "⠨⠋⠑⠆⠨⠕⠒")?;
    return Ok(());
}

#[test]
fn stoichiometric_p78_3a() -> Result<()> {
    let expr = r#"<math><msub><mi mathvariant="normal">H</mi><mn>2</mn></msub><mi mathvariant="normal">O</mi></math>"#;
    test_braille("Polish", expr, "⠸⠓⠆⠕")?;
    return Ok(());
}

#[test]
fn stoichiometric_p78_3b() -> Result<()> {
    let expr = r#"<math><msub><mi mathvariant="normal">H</mi><mn>2</mn></msub><mi mathvariant="normal">O</mi></math>"#;
    test_braille_prefs("Polish", vec![("Polish_BrailleLevel", "Beginner")], expr, r"⠨⠓⠆⠨⠕")?;
    return Ok(());
}

#[test]
fn stoichiometric_p78_4a() -> Result<()> {
    let expr = r#"<math><msub><mi mathvariant="normal">H</mi><mn>2</mn></msub><msub><mi>SO</mi><mn>4</mn></msub></math>"#;
    test_braille("Polish", expr, "⠸⠓⠆⠎⠕⠲")?;
    return Ok(());
}

#[test]
fn stoichiometric_p78_4b() -> Result<()> {
    let expr = r#"<math><msub><mi mathvariant="normal">H</mi><mn>2</mn></msub><msub><mi>SO</mi><mn>4</mn></msub></math>"#;
    test_braille_prefs("Polish", vec![("Polish_BrailleLevel", "Beginner")], expr, r"⠨⠓⠆⠨⠎⠨⠕⠲")?;
    return Ok(());
}

#[test]
fn stoichiometric_p78_5() -> Result<()> {
    let expr = r#"<math><mi>Ca</mi><msub><mrow><mo>(</mo><mi>OH</mi><mo>)</mo></mrow><mn>2</mn></msub></math>"#;
    test_braille_prefs("Polish", vec![("Polish_BrailleLevel", "Beginner")], expr, r"⠨⠉⠁⠣⠨⠕⠨⠓⠜⠆")?;
    return Ok(());
}

#[test]
fn stoichiometric_p78_6() -> Result<()> {
    let expr = r#"<math><msub><mi>Al</mi><mn>2</mn></msub><msub><mrow><mo>(</mo><msub><mi>SO</mi><mn>4</mn></msub><mo>)</mo></mrow><mn>3</mn></msub></math>"#;
    test_braille_prefs("Polish", vec![("Polish_BrailleLevel", "Beginner")], expr, r"⠨⠁⠇⠆⠣⠨⠎⠨⠕⠲⠜⠒")?;
    return Ok(());
}

#[test]
fn stoichiometric_p78_7b() -> Result<()> {
    let expr = r#"<math><mn>3</mn><msub><mi mathvariant="normal">H</mi><mn>2</mn></msub><mi mathvariant="normal">O</mi></math>"#;
    test_braille_prefs("Polish", vec![("Polish_BrailleLevel", "Beginner")], expr, r"⠼⠉⠨⠓⠆⠨⠕")?;
    return Ok(());
}

#[test]
fn stoichiometric_p78_7a() -> Result<()> {
    let expr = r#"<math><mn>3</mn><msub><mi mathvariant="normal">H</mi><mn>2</mn></msub><mi mathvariant="normal">O</mi></math>"#;
    test_braille("Polish", expr, "⠼⠉⠸⠓⠆⠕")?;
    return Ok(());
}

#[test]
fn stoichiometric_p78_8() -> Result<()> {
    let expr = r#"<math><mn>2</mn><msub><mi>Al</mi><mn>2</mn></msub><msub><mi mathvariant="normal">O</mi><mn>3</mn></msub></math>"#;
    test_braille("Polish", expr, "⠼⠃⠨⠁⠇⠆⠨⠕⠒")?;
    return Ok(());
}

#[test]
fn stoichiometric_p78_9() -> Result<()> {
    let expr = r#"<math><mn>2</mn><msub><mi mathvariant="normal">P</mi><mn>4</mn></msub><msub><mi mathvariant="normal">O</mi><mn>10</mn></msub></math>"#;
    test_braille("Polish", expr, "⠼⠃⠸⠏⠲⠕⠂⠴")?;
    return Ok(());
}

#[test]
fn reactions_p79_1() -> Result<()> {
    // spec has 'en.' but the braille dots don't have anything corresponding to ".", so I removed it
    let expr = r#"<math><mn>6</mn><mo>&#xA0;</mo><msub><mi mathvariant="normal">H</mi><mn>2</mn></msub><mi mathvariant="normal">O</mi><mo>&#xA0;</mo><mo>+</mo><mo>&#xA0;</mo><mn>6</mn><mo>&#xA0;</mo><msub><mi>CO</mi><mn>2</mn></msub><mover><mo>&#x2192;</mo><mrow><mtext>en</mtext></mrow></mover><msub><mi mathvariant="normal">C</mi><mn>6</mn></msub><msub><mi mathvariant="normal">H</mi><mn>12</mn></msub><msub><mi mathvariant="normal">O</mi><mn>6</mn></msub><mo>&#xA0;</mo><mo>+</mo><mo>&#xA0;</mo><mn>6</mn><mo>&#xA0;</mo><msub><mi mathvariant="normal">O</mi><mn>2</mn></msub><mo>&#xA0;</mo><mo>&#x2191;</mo></math>"#;
    test_braille("Polish", expr, "⠼⠋⠸⠓⠆⠕⠀⠖⠼⠋⠸⠉⠕⠆⠀⠰⠒⠂⠠⠑⠝⠄⠀⠸⠉⠖⠓⠂⠆⠕⠖⠠⠖⠼⠋⠨⠕⠆⠀⠸⠒⠁")?;
    return Ok(());
}

#[test]
fn ions_p80_1() -> Result<()> {
    let expr = r#"<math><msubsup><mi>SO</mi><mn>4</mn><mrow><mo>-</mo><mo>-</mo></mrow></msubsup></math>"#;
    test_braille_prefs("Polish", vec![("Polish_BrailleLevel", "Beginner")], expr, r"⠨⠎⠨⠕⠲⠌⠤⠤")?;
    return Ok(());
}

#[test]
fn ions_p80_2() -> Result<()> {
    let expr = r#"<math><msubsup><mi>SO</mi><mn>4</mn><mrow><mn>2</mn><mo>-</mo></mrow></msubsup></math>"#;
    test_braille("Polish", expr, r"⠨⠎⠨⠕⠲⠌⠼⠃⠤")?;
    return Ok(());
}

#[test]
fn ions_p80_3() -> Result<()> {
    let expr = r#"<math><msub><mi mathvariant="normal">H</mi><mn>3</mn></msub><msup><mi mathvariant="normal">O</mi><mo>+</mo></msup></math>"#;
    test_braille("Polish", expr, "⠸⠓⠒⠕⠌⠖")?;
    return Ok(());
}

#[test]
fn ions_p80_4() -> Result<()> {
    let expr = r#"<math><msup><mi>Ca</mi><mrow><mn>2</mn><mo>+</mo></mrow></msup></math>"#;
    test_braille("Polish", expr, "⠨⠉⠁⠌⠼⠃⠈⠖")?;
    return Ok(());
}

#[test]
fn hydrates_p80_5a() -> Result<()> {
    let expr = r#"<math><msub><mi>Al</mi><mn>2</mn></msub><msub><mi mathvariant="normal">O</mi><mn>3</mn></msub><mo>&#xB7;</mo><msub><mi mathvariant="normal">H</mi><mn>2</mn></msub><mi mathvariant="normal">O</mi></math>"#;
    test_braille("Polish", expr, "⠨⠁⠇⠆⠨⠕⠒⠄⠨⠓⠆⠨⠕")?;
    return Ok(());
}

#[test]
fn hydrates_p80_5b() -> Result<()> {
    let expr = r#"<math><msub><mi>Al</mi><mn>2</mn></msub><msub><mi mathvariant="normal">O</mi><mn>3</mn></msub><mo>&#xB7;</mo><msub><mi mathvariant="normal">H</mi><mn>2</mn></msub><mi mathvariant="normal">O</mi></math>"#;
    test_braille_prefs("Polish", vec![("Polish_BrailleLevel", "Advanced")], expr, r"⠨⠁⠇⠆⠨⠕⠒⠀⠄⠨⠓⠆⠨⠕")?;
    return Ok(());
}

#[test]
fn atoms_p81_1a() -> Result<()> {
    let expr = r#"<math><mmultiscripts><mi>He</mi><mprescripts/><mn>2</mn><mn>4</mn></mmultiscripts></math>"#;
    // avoid drop number
    test_braille_prefs("Polish", vec![("Polish_BrailleLevel", "Beginner")], expr, r"⠌⠼⠙⠡⠼⠃⠱⠨⠓⠑")?;
    return Ok(());
}

#[test]
fn atoms_p81_1b() -> Result<()> {
    let expr = r#"<math><mmultiscripts><mi>He</mi><mprescripts/><mn>2</mn><mn>4</mn></mmultiscripts></math>"#;
    test_braille("Polish", expr, "⠌⠲⠡⠆⠱⠨⠓⠑")?;
    return Ok(());
}

#[test]
fn electronegativity_p81_2() -> Result<()> {
    let expr = r#"<math><mo>&#x2206;</mo><msub><mi mathvariant="normal">E</mi><mi>NaCl</mi></msub><mo>=</mo><mn>3</mn><mo>,</mo><mn>0</mn><mo>-</mo><mn>0</mn><mo>,</mo><mn>9</mn></math>"#;
    test_braille("Polish", expr, "⠸⠙⠨⠑⠡⠨⠝⠁⠨⠉⠇⠀⠶⠼⠉⠂⠚⠀⠤⠼⠚⠂⠊")?;
    return Ok(());
}

#[test]
fn partial_charges_p81_3() -> Result<()> {
    let expr = r#"<math><mi mathvariant="normal">&#x3B4;</mi><mo>+</mo></math>"#;
    test_braille("Polish", expr, "⠰⠙⠈⠲")?;
    return Ok(());
}

#[test]
fn ionic_bonds_p83_1() -> Result<()> {
    let expr = r#"<math><msup><mrow><mo>[</mo><mi>Na</mi><mo>]</mo></mrow><mo>+</mo></msup><msup><mrow><mo>[</mo><mi>Cl</mi><mo>]</mo></mrow><mo>-</mo></msup></math>"#;
    test_braille("Polish", expr, "⠷⠨⠝⠁⠾⠌⠖⠀⠷⠨⠉⠇⠾⠌⠤")?;
    return Ok(());
}

#[test]
fn covalent_bonds_p83_2() -> Result<()> {
    let expr = r#"<math><mi mathvariant="normal">H</mi><mo>-</mo><mi mathvariant="normal">H</mi></math>"#;
    test_braille("Polish", expr, "⠨⠓⠐⠂⠨⠓")?;
    return Ok(());
}

#[test]
fn electron_configuration_p84_1() -> Result<()> {
    let expr = r#"<math><mmultiscripts><mi>Na</mi><mprescripts/><mn>11</mn><none/></mmultiscripts><mo>:</mo><mo>&#xA0;</mo><mn>1</mn><msup><mi mathvariant="normal">s</mi><mn>2</mn></msup><mo>&#xA0;</mo><mn>2</mn><msup><mi mathvariant="normal">s</mi><mn>2</mn></msup><msup><mi mathvariant="normal">p</mi><mn>6</mn></msup><mo>&#xA0;</mo><mn>3</mn><msup><mi mathvariant="normal">s</mi><mn>1</mn></msup></math>"#;
    test_braille("Polish", expr, "⠡⠼⠁⠁⠨⠝⠁⠠⠒⠀⠼⠁⠠⠎⠌⠆⠀⠼⠃⠠⠎⠌⠆⠠⠏⠌⠖⠠⠼⠉⠠⠎⠌⠂")?;
    return Ok(());
}

#[test]
fn electron_configuration_p84_2() -> Result<()> {
    let expr = r#"<math><mmultiscripts><mi mathvariant="normal">K</mi><mprescripts/><mn>19</mn><none/></mmultiscripts><mo>:</mo><mo>&#xA0;</mo><mn>1</mn><msup><mi mathvariant="normal">s</mi><mn>2</mn></msup><mo>&#xA0;</mo><mn>2</mn><msup><mi mathvariant="normal">s</mi><mn>2</mn></msup><msup><mi mathvariant="normal">p</mi><mn>6</mn></msup><mo>&#xA0;</mo><mn>3</mn><msup><mi mathvariant="normal">s</mi><mn>2</mn></msup><msup><mi mathvariant="normal">p</mi><mn>6</mn></msup><mo>&#xA0;</mo><mn>4</mn><msup><mi mathvariant="normal">s</mi><mn>1</mn></msup></math>"#;
    test_braille("Polish", expr, "⠡⠼⠁⠊⠨⠅⠠⠒⠀⠼⠁⠠⠎⠌⠆⠀⠼⠃⠠⠎⠌⠆⠠⠏⠌⠈⠖⠀⠼⠉⠠⠎⠌⠆⠠⠏⠌⠖⠀⠼⠙⠠⠎⠌⠂")?;
    return Ok(());
}

#[test]
fn electron_p84_3a() -> Result<()> {
    let expr = r#"<math><mn>3</mn><mover><mi mathvariant="normal">e</mi><mo>-</mo></mover></math>"#;
    test_braille("Polish", expr, "⠼⠉⠠⠑⠒")?;
    return Ok(());
}

#[test]
fn electron_p84_3b() -> Result<()> {
    let expr = r#"<math><mn>3</mn><msup><mi mathvariant="normal">e</mi><mo>-</mo></msup></math>"#;
    test_braille_prefs("Polish", vec![("Polish_BrailleLevel", "Advanced")], expr, r"⠼⠉⠠⠑⠤")?;
    return Ok(());
}

#[test]
fn valence_p84_4() -> Result<()> {
    let expr = r#"<math><msup><mi>Al</mi><mi>III</mi></msup></math>"#;
    test_braille("Polish", expr, "⠨⠁⠇⠌⠨⠊⠊⠊")?;
    return Ok(());
}

#[test]
fn valence_p84_5() -> Result<()> {
    let expr = r#"<math><msup><mi>Ca</mi><mi>II</mi></msup><msup><mi mathvariant="normal">O</mi><mi>II</mi></msup></math>"#;
    test_braille("Polish", expr, "⠨⠉⠁⠌⠨⠊⠊⠱⠨⠕⠌⠨⠊⠊")?;
    return Ok(());
}

#[test]
fn oxidation_states_p85_1() -> Result<()> {
    let expr = r#"<math><mover><mi mathvariant="normal">S</mi><mi>IV</mi></mover><mover><msub><mi mathvariant="normal">O</mi><mn>2</mn></msub><mrow><mo>-</mo><mi>II</mi></mrow></mover></math>"#;
    test_braille("Polish", expr, "⠨⠎⠣⠨⠊⠧⠜⠨⠕⠆⠣⠤⠨⠊⠊⠜")?;
    return Ok(());
}

#[test]
fn oxidation_states_p85_2a() -> Result<()> {
    let expr = r#"<math intent=":chemical-formula"><mover><msub><mi mathvariant="normal">N</mi><mn>2</mn></msub><mn>0</mn></mover></math>"#;
    test_braille_prefs("Polish", vec![("Polish_BrailleLevel", "Beginner")], expr, r"⠨⠝⠆⠣⠼⠚⠜")?;
    return Ok(());
}

#[test]
fn oxidation_states_p85_2b() -> Result<()> {
    let expr = r#"<math intent=":chemical-formula"><mover><msub><mi mathvariant="normal">N</mi><mn>2</mn></msub><mn>0</mn></mover></math>"#;
    test_braille_prefs("Polish", vec![("Polish_BrailleLevel", "Advanced")], expr, r"⠨⠝⠆⠣⠴⠜")?;
    return Ok(());
}

#[test]
fn reactions_p85_3() -> Result<()> {
    let expr = r#"<math><mi>Fe</mi><mo>+</mo><mn>2</mn><mi>HCl</mi><mo>&#x2192;</mo><msub><mi>FeCl</mi><mn>2</mn></msub><mo>+</mo><msub><mi mathvariant="normal">H</mi><mn>2</mn></msub><mo>&#x2191;</mo></math>"#;
    test_braille("Polish", expr, "⠨⠋⠑⠀⠖⠼⠃⠨⠓⠨⠉⠇⠀⠒⠂⠨⠋⠑⠨⠉⠇⠆⠀⠖⠨⠓⠆⠸⠒⠁")?;
    return Ok(());
}

#[test]
fn reactions_p85_4() -> Result<()> {
    let expr = r#"<math><mi>Fe</mi><mo>+</mo><mn>2</mn><msup><mi mathvariant="normal">H</mi><mo>+</mo></msup><mo>+</mo><mn>2</mn><msup><mi>Cl</mi><mo>-</mo></msup><mo>&#x2192;</mo><msup><mi>Fe</mi><mrow><mn>2</mn><mo>+</mo></mrow></msup><mo>+</mo><mn>2</mn><msup><mi>Cl</mi><mo>-</mo></msup><mo>+</mo><msub><mi mathvariant="normal">H</mi><mn>2</mn></msub><mo>&#x2191;</mo></math>"#;
    test_braille("Polish", expr, "⠨⠋⠑⠀⠖⠼⠃⠨⠓⠌⠖⠀⠖⠼⠃⠨⠉⠇⠌⠤⠀⠒⠂⠨⠋⠑⠌⠼⠃⠈⠖⠀⠖⠼⠃⠨⠉⠇⠌⠤⠀⠖⠨⠓⠆⠸⠒⠁")?;
    return Ok(());
}

#[test]
fn reactions_p85_5() -> Result<()> {
    let expr = r#"<math><mi>Pb</mi><mo>+</mo><msub><mi mathvariant="normal">H</mi><mn>2</mn></msub><msub><mi>SO</mi><mn>4</mn></msub><mo>&#x2192;</mo><mi>PbS</mi><msub><mi>O</mi><mn>4</mn></msub><mo>&#x2193;</mo><mo>+</mo><msub><mi mathvariant="normal">H</mi><mn>2</mn></msub><mo>&#x2191;</mo></math>"#;
    // Corrected: spec is wrong for "Pb"
    test_braille("Polish", expr, "⠨⠏⠃⠀⠖⠸⠓⠆⠎⠕⠲⠀⠒⠂⠨⠏⠃⠨⠎⠨⠕⠲⠸⠒⠄⠖⠨⠓⠆⠸⠒⠁")?;
    return Ok(());
}

#[test]
fn reactions_p86_1() -> Result<()> {
    let expr = r#"<math><mi>Pb</mi><mo>+</mo><mn>2</mn><msup><mi mathvariant="normal">H</mi><mo>+</mo></msup><mo>+</mo>
                                    <msup><msub><mi>SO</mi><mn>4</mn></msub><mrow><mn>2</mn><mo>-</mo></mrow></msup><mo>&#x2192;</mo>
                                    <mi>PbS</mi><msub><mi mathvariant="normal">O</mi><mn>4</mn></msub><mo>&#x2193;</mo><mo>+</mo>
                                    <msub><mi mathvariant="normal">H</mi><mn>2</mn></msub><mo>&#x2191;</mo></math>"#;
    // Corrected: spec is wrong for "Pb"
    test_braille("Polish", expr, "⠨⠏⠃⠀⠖⠼⠃⠨⠓⠌⠖⠀⠖⠸⠎⠕⠲⠬⠼⠃⠈⠤⠀⠒⠂⠨⠏⠃⠨⠎⠨⠕⠲⠸⠒⠄⠖⠨⠓⠆⠸⠒⠁")?;
    return Ok(());
}

#[test]
fn reactions_p86_2() -> Result<()> {
    let expr = r#"<math><mi>KOH</mi><mo>⟷</mo><msup><mi mathvariant="normal">K</mi><mo>+</mo></msup><mo>+</mo><msup><mi>OH</mi><mo>-</mo></msup></math>"#;
    test_braille("Polish", expr, "⠸⠅⠕⠓⠀⠐⠒⠂⠨⠅⠌⠖⠀⠖⠸⠕⠓⠌⠤")?;
    return Ok(());
}

#[test]
fn reactions_p86_3() -> Result<()> {
    let expr = r#"<math><mi>HCOOH</mi>
        <mover><mo>⟷</mo><mrow><msub><mi mathvariant="normal">H</mi><mn>2</mn></msub><mi mathvariant="normal">O</mi></mrow></mover>
        <msup><mi>HCOO</mi><mo>-</mo></msup><mo>+</mo><msup><mi mathvariant="normal">H</mi><mo>+</mo></msup></math>"#;
    test_braille("Polish", expr, "⠸⠓⠉⠕⠕⠓⠀⠰⠐⠒⠂⠠⠸⠓⠆⠕⠄⠀⠸⠓⠉⠕⠕⠌⠤⠀⠖⠨⠓⠌⠖")?;
    return Ok(());
}

#[test]
fn redox_reactions_p86_4() -> Result<()> {
    let expr = r#"<math><mover><mi>Zn</mi><mn>0</mn></mover><mo>+</mo><mn>2</mn><mover><mi mathvariant="normal">H</mi><mi mathvariant="normal">I</mi></mover><mover><mi>Cl</mi><mrow><mo>-</mo><mi mathvariant="normal">I</mi></mrow></mover><mo>&#x2192;</mo>
            <mover><mi>Zn</mi><mi>II</mi></mover><mover><msub><mi>Cl</mi><mn>2</mn></msub><mrow><mo>-</mo><mi mathvariant="normal">I</mi></mrow></mover><mo>+</mo><mover><msub><mi mathvariant="normal">H</mi><mn>2</mn></msub><mn>0</mn></mover></math>"#;
    test_braille("Polish", expr, "⠨⠵⠝⠣⠴⠜⠀⠖⠼⠃⠨⠓⠣⠨⠊⠜⠨⠉⠇⠣⠤⠨⠊⠜⠀⠒⠂⠨⠵⠝⠣⠨⠊⠊⠜⠨⠉⠇⠆⠣⠤⠨⠊⠜⠀⠖⠨⠓⠆⠣⠴⠜")?;
    return Ok(());
}

#[test]
fn redox_reactions_p86_5() -> Result<()> {
    let expr = r#"<math><mover><mi>Al</mi><mn>0</mn></mover><mo>-</mo><mn>3</mn><mover><mi mathvariant="normal">e</mi><mo>-</mo></mover><mo>&#x2192;</mo>
                            <mover><msup><mi>Al</mi><mrow><mn>3</mn><mo>+</mo></mrow></msup><mi>III</mi></mover></math>"#;
    test_braille("Polish", expr, "⠨⠁⠇⠣⠴⠜⠀⠤⠼⠉⠠⠑⠒⠀⠒⠂⠨⠁⠇⠌⠼⠉⠈⠖⠣⠨⠊⠊⠊⠜")?;
    return Ok(());
}

#[test]
fn reactions_p86_6() -> Result<()> {
    // needed to mark some of the chemical elements with intent as MathCAT can't infer them
    let expr = r#"<math><mi mathvariant="normal">A</mi><mo>&#xA0;&#xA0;</mo><mo>(</mo><mo>+</mo><msub><mi intent=":chemical-element">Cl</mi><mn>2</mn></msub><mo>)</mo><mo>&#x2192;</mo><mi mathvariant="normal">B</mi>
                   <mo>&#xA0;&#xA0;</mo><mo>(</mo><mo>-</mo><mi intent=":chemical-element">H</mi><mi intent=":chemical-element">Cl</mi><mo>)</mo><mo>&#x2192;</mo><mi mathvariant="normal">C</mi></math>"#;
    test_braille("Polish", expr, "⠨⠁⠀⠣⠖⠨⠉⠇⠆⠜⠀⠒⠂⠨⠃⠀⠣⠤⠨⠓⠨⠉⠇⠜⠀⠒⠂⠨⠉")?;
    return Ok(());
}

#[test]
fn chemical_units_p87_1() -> Result<()> {
    let expr = r#"<math><mi mathvariant="normal" intent=":unit">g</mi><mo>/</mo><mi>mol</mi></math>"#;
    test_braille("Polish", expr, "⠻⠛⠳⠍⠕⠇")?;
    return Ok(());
}

#[test]
fn chemical_units_p87_2() -> Result<()> {
    let expr = r#"<math><mn>3</mn><mi mathvariant="normal" intent=":unit">g</mi><mo>/</mo><msup><mi>dm</mi><mn>3</mn></msup></math>"#;
    test_braille("Polish", expr, "⠼⠉⠻⠛⠳⠙⠍⠬⠒")?;
    return Ok(());
}

#[test]
fn concentration_p87_3() -> Result<()> {
    let expr = r#"<math><msub><mi mathvariant="normal">c</mi><mi mathvariant="normal">p</mi></msub><mo>=</mo><mfrac><msub><mi mathvariant="normal">m</mi><mi mathvariant="normal">s</mi></msub><msub><mi mathvariant="normal">m</mi><mi mathvariant="normal">r</mi></msub></mfrac><mo>&#xB7;</mo><mn>100</mn><mo>%</mo></math>"#;
    test_braille("Polish", expr, "⠠⠉⠡⠏⠀⠶⠍⠡⠎⠳⠍⠡⠗⠀⠄⠼⠁⠚⠚⠼⠚⠴")?;
    return Ok(());
}

#[test]
fn concentration_p87_4a() -> Result<()> {
    let expr = r#"<math><msub><mi mathvariant="normal">c</mi><msub><mi mathvariant="normal">p</mi><mn>1</mn></msub></msub><mo>=</mo><mfrac><msub><mi mathvariant="normal">m</mi><msub><mi mathvariant="normal">s</mi><mn>1</mn></msub></msub><msub><mi mathvariant="normal">m</mi><msub><mi mathvariant="normal">r</mi><mn>1</mn></msub></msub></mfrac><mo>&#xB7;</mo><mn>100</mn><mo>%</mo></math>"#;
    test_braille("Polish", expr, "⠠⠉⠡⠏⠡⠂⠀⠶⠍⠡⠎⠡⠂⠳⠍⠡⠗⠡⠂⠀⠄⠼⠁⠚⠚⠼⠚⠴")?;
    return Ok(());
}

#[test]
fn concentration_p87_4b() -> Result<()> {
    let expr = r#"<math><msub><mi mathvariant="normal">c</mi><msub><mi mathvariant="normal">p</mi><mn>1</mn></msub></msub><mo>=</mo><mfrac><msub><mi mathvariant="normal">m</mi><msub><mi mathvariant="normal">s</mi><mn>1</mn></msub></msub><msub><mi mathvariant="normal">m</mi><msub><mi mathvariant="normal">r</mi><mn>1</mn></msub></msub></mfrac><mo>&#xB7;</mo><mn>100</mn><mo>%</mo></math>"#;
    test_braille_prefs("Polish", vec![("Polish_BrailleLevel", "Advanced")], expr, r"⠠⠉⠡⠏⠡⠂⠀⠶⠆⠍⠡⠎⠡⠂⠀⠳⠀⠍⠡⠗⠡⠂⠰⠄⠼⠁⠚⠚⠼⠚⠴")?;
    return Ok(());
}

#[test]
fn organic_cmpds_p88_1() -> Result<()> {
    let expr = r#"<math><msub><mi mathvariant="normal">C</mi><mi mathvariant="normal">n</mi></msub><msub><mi mathvariant="normal">H</mi><mrow><mn>2</mn><mi mathvariant="normal">n</mi><mo>+</mo><mn>2</mn></mrow></msub></math>"#;
    test_braille("Polish", expr, "⠨⠉⠡⠠⠝⠱⠨⠓⠡⠼⠃⠠⠝⠈⠖⠼⠃")?;
    return Ok(());
}

#[test]
fn organic_cmpds_p88_2() -> Result<()> {
    let expr = r#"<math><msub><mi mathvariant="normal">C</mi><mi mathvariant="normal">n</mi></msub><msub><mi mathvariant="normal">H</mi><mrow><mn>2</mn><mi mathvariant="normal">n</mi><mo>+</mo><mn>1</mn></mrow></msub><mi>OH</mi></math>"#;
    test_braille("Polish", expr, "⠨⠉⠡⠠⠝⠱⠨⠓⠡⠼⠃⠠⠝⠈⠖⠼⠁⠱⠨⠕⠨⠓")?;
    return Ok(());
}

#[test]
fn structural_formulas_p89_1() -> Result<()> {
    let expr = r#"<math><mi mathvariant="normal">H</mi><mo>-</mo><mi>Cl</mi></math>"#;
    test_braille("Polish", expr, "⠨⠓⠐⠂⠨⠉⠇")?;
    return Ok(());
}

#[test]
fn structural_formulas_p89_2() -> Result<()> {
    let expr = r#"<math><mi mathvariant="normal">C</mi><mo>=</mo><mi mathvariant="normal">O</mi></math>"#;
    test_braille("Polish", expr, "⠨⠉⠨⠅⠨⠕")?;
    return Ok(());
}

#[test]
fn structural_formulas_p89_3() -> Result<()> {
    let expr = r#"<math><mi mathvariant="normal">N</mi><mo>&#x2261;</mo><mi mathvariant="normal">N</mi></math>"#;
    test_braille("Polish", expr, "⠨⠝⠸⠇⠨⠝")?;
    return Ok(());
}

#[test]
fn structural_formulas_p89_4() -> Result<()> {
    let expr = r#"<math><mi>HO</mi><mo>-</mo><mi>Ca</mi><mo>-</mo><mi>OH</mi></math>"#;
    test_braille_prefs("Polish", vec![("Polish_BrailleLevel", "Beginner")], expr, r"⠨⠓⠨⠕⠐⠂⠨⠉⠁⠐⠂⠨⠕⠨⠓")?;
    // test_braille("Polish", expr, "⠨⠓⠨⠕⠐⠂⠨⠉⠁⠐⠂⠨⠕⠨⠓");
    return Ok(());
}

#[test]
fn structural_formulas_p89_5() -> Result<()> {
    let expr = r#"<math><mi mathvariant="normal">O</mi><mo>=</mo><mi>Al</mi><mo>-</mo><mi mathvariant="normal">O</mi><mo>-</mo><mi>Al</mi><mo>=</mo><mi mathvariant="normal">O</mi></math>"#;
    test_braille("Polish", expr, "⠨⠕⠨⠅⠨⠁⠇⠐⠂⠨⠕⠐⠂⠨⠁⠇⠨⠅⠨⠕")?;
    return Ok(());
}

#[test]
fn structural_formulas_p89_6a() -> Result<()> {
    let expr = r#"<math><msub><mi>CH</mi><mn>3</mn></msub><mo>-</mo><msub><mi>CH</mi><mn>2</mn></msub><mo>-</mo><mi>COOH</mi></math>"#;
    test_braille("Polish", expr, "⠸⠉⠓⠒⠐⠂⠸⠉⠓⠆⠐⠂⠸⠉⠕⠕⠓")?;
    return Ok(());
}

#[test]
fn structural_formulas_p89_6b() -> Result<()> {
    let expr = r#"<math><msub><mi>CH</mi><mn>3</mn></msub><mo>-</mo><msub><mi>CH</mi><mn>2</mn></msub><mo>-</mo><mi>COOH</mi></math>"#;
    test_braille_prefs("Polish", vec![("Polish_BrailleLevel", "Beginner")], expr, r"⠨⠉⠨⠓⠒⠐⠂⠨⠉⠨⠓⠆⠐⠂⠨⠉⠨⠕⠨⠕⠨⠓")?;
    return Ok(());
}

#[test]
fn structural_formulas_p89_7() -> Result<()> {
    let expr = r#"<math><msub><mi>CH</mi><mn>3</mn></msub><mo>-</mo><msub><mrow><mo>(</mo><msub><mi>CH</mi><mn>2</mn></msub><mo>)</mo></mrow><mn>8</mn></msub><mo>-</mo><msub><mi>CH</mi><mn>3</mn></msub></math>"#;
    test_braille("Polish", expr, "⠸⠉⠓⠒⠐⠂⠣⠸⠉⠓⠆⠜⠦⠐⠂⠸⠉⠓⠒")?;
    return Ok(());
}

#[test]
fn structural_formulas_p90_1() -> Result<()> {
    let expr = r#"<math><msub><mi>CH</mi><mn>3</mn></msub><mo>-</mo><mi>CH</mi><mo>(</mo><msub><mi>CH</mi><mn>3</mn></msub><mo>)</mo><mo>-</mo><msub><mi>CH</mi><mn>3</mn></msub></math>"#;
    test_braille("Polish", expr, "⠸⠉⠓⠒⠐⠂⠸⠉⠓⠣⠸⠉⠓⠒⠜⠐⠂⠸⠉⠓⠒")?;
    return Ok(());
}

#[test]
fn structural_formulas_p90_2() -> Result<()> {
    let expr = r#"<math><msub><mi>CH</mi><mn>3</mn></msub><mi>CH</mi><mo>(</mo><msub><mi>CH</mi><mn>3</mn></msub><mo>)</mo><msub><mi>CH</mi><mn>3</mn></msub></math>"#;
    test_braille_prefs("Polish", vec![("Polish_BrailleLevel", "Beginner")], expr, r"⠨⠉⠨⠓⠒⠨⠉⠨⠓⠣⠨⠉⠨⠓⠒⠜⠨⠉⠨⠓⠒")?;
    // test_braille("Polish", expr, "⠨⠉⠨⠓⠒⠨⠉⠨⠓⠣⠨⠉⠨⠓⠒⠜⠨⠉⠨⠓⠒");
    return Ok(());
}

#[test]
fn structural_formulas_p91_1() -> Result<()> {
    let expr = r#"<math><msub><mi>CH</mi><mn>3</mn></msub><mi>COH</mi><mo>(</mo><msub><mi>CH</mi><mn>3</mn></msub><mo>)</mo><msub><mi>CH</mi><mn>3</mn></msub></math>"#;
    test_braille("Polish", expr, "⠸⠉⠓⠒⠸⠉⠕⠓⠣⠸⠉⠓⠒⠜⠸⠉⠓⠒")?;
    return Ok(());
}
