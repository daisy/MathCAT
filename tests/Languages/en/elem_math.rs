use crate::common::*;
use anyhow::Result;

#[test]
fn addition_operator_left() -> Result<()> {
    let expr = "<math><mstack><mn>424</mn><msrow><mo>+</mo><none/><mn>33</mn></msrow><msline/></mstack></math>";
    test("en", "ClearSpeak", expr, "addition problem; 424 plus 33; equals; end of problem")?;
    return Ok(());
}

#[test]
fn addition_operator_right() -> Result<()> {
    let expr = "<math><mstack><mn>123</mn><msrow><mn>456</mn><mo>+</mo></msrow><msline/><mn>579</mn></mstack></math>";
    test("en", "ClearSpeak", expr, "addition problem; 123 plus 456, equals; 579; end of problem")?;
    return Ok(());
}

#[test]
fn subtraction_borrow_above() -> Result<()> {
    let expr = r#"<math><mstack>
                        <mscarries crossout='updiagonalstrike'><mn>2</mn><mn>12</mn><mscarry crossout='none'><none/></mscarry></mscarries>
                        <mn>2,327</mn>
                        <msrow><mo>-</mo><mn>1,156</mn></msrow>
                        <msline/>
                        <mn>1,171</mn>
                        </mstack>
                        </math>"#;
    test(
        "en",
        "ClearSpeak",
        expr,
        r#"subtraction problem; crossout and carry 2 in the hundreds column, crossout and carry 12 in the tens column;
                 2327 minus 1156, equals; 1171; end of problem"#,
    )?;
    return Ok(());
}

#[test]
fn subtraction_borrow_aboveleft() -> Result<()> {
    let expr = r#"<math><mstack>
                        <mscarries location='nw'><none/><mscarry crossout='updiagonalstrike' location='n'><mn>2</mn></mscarry><mn>1</mn><none/></mscarries>
                        <mn>2,327</mn>
                        <msrow><mo>-</mo><mn>1,156</mn></msrow>
                        <msline/>
                        <mn>1,171</mn>
                        </mstack></math>"#;
    test(
        "en",
        "ClearSpeak",
        expr,
        r#"subtraction problem; crossout in the thousands column, crossout and carry 2 in the hundreds column, 
         carry 1, in the tens column; 2327 minus 1156, equals; 1171; 
         end of problem"#,
    )?;
    return Ok(());
}

#[test]
fn subtraction_borrow_swedish() -> Result<()> {
    let expr = r#"<math><mstack>
                        <mscarries><mscarry crossout='updiagonalstrike'><none/></mscarry><menclose notation='bottom'><mn>10</mn></menclose></mscarries>
                        <mn>52</mn>
                        <msrow><mo>-</mo><mn>7</mn></msrow>
                        <msline/>
                        <mn>45</mn>
                        </mstack></math>"#;
    test(
        "en",
        "ClearSpeak",
        expr,
        r#"subtraction problem; crossout in the tens column, carry 10 in the ones column, 
                52 minus 7, equals; 45; end of problem"#,
    )?;
    return Ok(());
}

#[test]
fn multiplication_simple() -> Result<()> {
    let expr = r#"<math><mstack>
                        <msgroup>
                          <mn>123</mn>
                          <msrow><mo>&#xD7;</mo><mn>321</mn></msrow>
                        </msgroup><msline/>
                        <msgroup shift='1'>
                            <mn>123</mn>
                            <mn>246</mn>
                            <mn>369</mn>
                        </msgroup>
                        <msline/>
                        </mstack></math>"#;
    test(
        "en",
        "ClearSpeak",
        expr,
        r#"multiplication problem; 123 times 321, equals; 123, shifted to the left 1 digit; 246, shifted to the left 2 digits 
                369; equals; end of problem"#,
    )?;
    return Ok(());
}

#[test]
fn multiplication_carries() -> Result<()> {
    let expr = r#"<math><mstack>
                        <mscarries><mn>1</mn><mn>1</mn><none/></mscarries>
                        <mscarries><mn>1</mn><mn>1</mn><none/></mscarries>
                        <mn>1,234</mn>
                        <msrow><mo>&#xD7;</mo><mn>4,321</mn></msrow>
                        <msline/>
                        <mscarries position='2'><mn>1</mn><none/><mn>1</mn><mn>1</mn><mn>1</mn><none/><mn>1</mn></mscarries>
                        <msgroup shift='1'>
                          <mn>1,234</mn>
                          <mn>24,68</mn>
                          <mn>370,2</mn>
                          <msrow position='1'><mn>4,936</mn></msrow>
                        </msgroup>
                        <msline/>
                        <mn>5,332,114</mn>
                        </mstack></math>"#;
    test(
        "en",
        "ClearSpeak",
        expr,
        r#"multiplication problem; carry 1 in the hundreds column, carry 1 in the tens column,
         next row;
         carry 1 in the hundreds column, carry 1 in the tens column;
         1234 times 4321, equals,
         carry 1 in the hundred millions column, carry 1 in the millions column, carry 1 in the hundred thousands column,
         carry 1 in the ten thousands column, 
         carry 1 in the hundreds column;
         1234, shifted to the left 1 digit;
         24 68, shifted to the left 2 digits;
         370 2, shifted to the left 4 digits;
         4 936;
         equals; 5332114; end of 
         problem"#,
    )?;
    return Ok(());
}

#[test]
fn repeating_decimal_line_over_3() -> Result<()> {
    let expr = r#"<math><mstack stackalign='right'><msline length='1'/><mn>0.3333</mn></mstack></math>"#;
    test(
        "en",
        "ClearSpeak",
        expr,
        r#"the repeating decimal, 0 point 3 3 3, followed by repeating digit, 3"#,
    )?;
    return Ok(());
}

#[test]
fn repeating_decimal_line_over_fraction() -> Result<()> {
    let expr = r#"<math><mstack stackalign='right'><msline length='6'/><mn>0.142857</mn></mstack></math>"#;
    test(
        "en",
        "ClearSpeak",
        expr,
        r#"the repeating decimal, 0 point, followed by repeating digits, 1 4 2 8 5 7"#,
    )?;
    return Ok(());
}

#[test]
fn repeating_decimal_line_under_fraction() -> Result<()> {
    let expr = r#"<math><mstack stackalign='right'><mn>0.142857</mn><msline length='6'/></mstack></math>"#;
    test(
        "en",
        "ClearSpeak",
        expr,
        r#"the repeating decimal, 0 point, followed by repeating digits, 1 4 2 8 5 7"#,
    )?;
    return Ok(());
}

#[test]
fn repeating_decimal_dots_over_fraction() -> Result<()> {
    let expr = r#"<math><mstack stackalign='right'><msrow><mo>.</mo><none/><none/><none/><none/><mo>.</mo></msrow><mn>
0.142857</mn></mstack></math>"#;
    test(
        "en",
        "ClearSpeak",
        expr,
        r#"the repeating decimal, 0 point, followed by repeating digits, 1 4 2 8 5 7"#,
    )?;
    return Ok(());
}

#[test]
fn longdiv_lefttop() -> Result<()> {
    let expr = r#"<math><mlongdiv longdivstyle='lefttop'>
                        <mn>3</mn>
                        <mn>435.3</mn>
                        <mn>1306</mn>
                        <msgroup position='2' shift='-1'>
                          <msgroup>
                            <mn>12</mn>
                            <msline length='2'/>
                            </msgroup>
                            <msgroup>
                            <mn>10</mn>
                            <mn>9</mn>
                            <msline length='2'/>
                            </msgroup>
                            <msgroup>
                            <mn>16</mn>
                            <mn>15</mn>
                            <msline length='2'/>
                            <mn>1.0</mn>
                            </msgroup>
                            <msgroup position='-1'>
                            <mn>9</mn>
                            <msline length='3'/>
                            <mn>1</mn>
                            </msgroup>
                        </msgroup>
                        </mlongdiv></math>"#;
    test(
        "en",
        "ClearSpeak",
        expr,
        r#"1306, divided by 3, equals, 435.3; here's why; 3 times, 4 in the hundreds column equals 12 in the 
         hundreds column; 1306 in the ones column, minus, 12 in the hundreds column equals 1 in the hundreds column; 
         next step: 3 times, 3 in the tens column equals 9 in the tens column; 10 in the tens column, minus, 9 in the 
         tens column equals 1 in the tens column; next step: 3 times, 5 in the ones column equals 15; 16 in the ones 
         column, minus, 15 in the ones column equals 1 in the ones column; next step: 3 times, 3 in the tenths column 
         equals 9 in the tenths column; 1.0 in the ones column, minus, 9 in the tenths column equals 1 in the tenths 
         column, end long division"#,
    )?;
    return Ok(());
}

#[test]
fn longdiv_stackedrightright() -> Result<()> {
    let expr = r#"<math><mstyle decimalpoint=','><mlongdiv longdivstyle='stackedrightright'>
                        <mn>3</mn>
                        <mn>435.3</mn>
                        <mn>1306</mn>
                        <msgroup position='2' shift='-1'>
                          <msgroup>
                            <mn>12</mn>
                            <msline length='2'/>
                            </msgroup>
                            <msgroup>
                            <mn>10</mn>
                            <mn>9</mn>
                            <msline length='2'/>
                            </msgroup>
                            <msgroup>
                            <mn>16</mn>
                            <mn>15</mn>
                            <msline length='2'/>
                            <mn>1.0</mn>
                            </msgroup>
                            <msgroup position='-1'>
                            <mn>9</mn>
                            <msline length='3'/>
                            <mn>1</mn>
                            </msgroup>
                        </msgroup>
                        </mlongdiv></mstyle></math>"#;
    test_prefs(
        "en",
        "ClearSpeak",
        vec![("DecimalSeparators", "."), ("BlockSeparators", " ,")],
        expr,
        r#"1306, divided by 3, equals, 435.3; here's why; 3 times, 4 in the hundreds column equals 12 in the 
         hundreds column; 1306 in the ones column, minus, 12 in the hundreds column equals 1 in the hundreds column; 
         next step: 3 times, 3 in the tens column equals 9 in the tens column; 10 in the tens column, minus, 9 in the 
         tens column equals 1 in the tens column; next step: 3 times, 5 in the ones column equals 15; 16 in the ones 
         column, minus, 15 in the ones column equals 1 in the ones column; next step: 3 times, 3 in the tenths column 
         equals 9 in the tenths column; 1.0 in the ones column, minus, 9 in the tenths column equals 1 in the tenths 
         column, end long division"#,
    )?;
    return Ok(());
}

#[test]
fn addition_mn_rows() -> Result<()> {
    let expr = r#"<math><mstack><mn>123</mn><mn>98</mn><msrow><mo>+</mo><mn>456</mn></msrow><msline></msline><mn>677</mn></mstack></math>"#;
    test("en", "ClearSpeak", expr,
        r#"addition problem; 12398 plus 456, equals; 677; end of problem"#)?;
    return Ok(());
}

#[test]
fn decimal_subtraction_stack() -> Result<()> {
    let expr = r#"<math><mstack><mn>1.23</mn><msrow><mo>-</mo><mn>.98</mn></msrow><msline></msline><mn>0.25</mn></mstack></math>"#;
    test("en", "ClearSpeak", expr,
        r#"subtraction problem; 1.23 minus .98, equals; 0.25; end of problem"#)?;
    return Ok(());
}

#[test]
fn formatted_subtraction_stack() -> Result<()> {
    let expr =
        r#"<math><mstack><mn>1,234,567.23</mn><msrow><mo>-</mo><mn>6 543.98</mn></msrow><msline></msline></mstack></math>"#;
    test(
        "en",
        "ClearSpeak",
        expr,
        r#"subtraction problem; 1234567.23 minus 6543.98, equals; end of problem"#,
    )?;
    return Ok(());
}

#[test]
fn carry_formatted_subtraction() -> Result<()> {
    let expr = r#"<math><mstack>
                        <mscarries crossout='updiagonalstrike'><mn>2</mn><mn>12</mn><none/></mscarries>
                        <mn>2,327</mn>
                        <msrow><mo>-</mo><mn>1,156</mn></msrow>
                        <msline/>
                        <mn>1,171</mn>
                        </mstack></math>"#;
    test(
        "en",
        "ClearSpeak",
        expr,
        r#"subtraction problem; crossout and carry 2 in the hundreds column, crossout and carry 12 in the tens 
         column; 2327 minus 1156, equals; 1171; end of problem"#,
    )?;
    return Ok(());
}

#[test]
fn longdiv_msgroup() -> Result<()> {
    let expr =
        r#"<math><mlongdiv longdivstyle='lefttop'><mn>3</mn><mn>435.3</mn><mn>1306</mn><msgroup position='2' 
         shift='-1'><msgroup><mn>12</mn><msline length='2'/></msgroup><msgroup><mn>10</mn><mn>9</mn><msline 
         length='2'/></msgroup></msgroup></mlongdiv></math>"#;
    test(
        "en",
        "ClearSpeak",
        expr,
        r#"long division problem, 1306, divided by 3, equals, 435.3; here's why; 3 times, 4 in the hundreds 
         column equals 12 in the hundreds column; 1306 in the ones column, minus, 12 in the hundreds column 
         equals 1 in the hundreds column; next step: 3 times, 3 in the tens column equals 9 in the tens 
         column; 10 in the tens column, minus, 9 in the tens column equals, end long division"#,
    )?;
    return Ok(());
}

#[test]
fn addition_stack() -> Result<()> {
    let expr = r#"<math><mstack>
                        <msrow><mo>+</mo><mn>2</mn><mn>7</mn></msrow>
                        <msrow><mn>1</mn><mn>4</mn></msrow>
                        <msline/>
                        <msrow><mn>4</mn><mn>1</mn></msrow>
                        </mstack></math>"#;
    test("en", "ClearSpeak", expr,
        r#"addition problem; 27 plus 14, equals; 41; end of problem"#)?;
    return Ok(());
}

#[test]
fn subtraction_stack() -> Result<()> {
    let expr = r#"<math><mstack><msrow><mo>−</mo><mn>5</mn><mn>0</mn><mn>5</mn></msrow><msrow><mn>2</mn><mn>2</mn>
<mn>5</mn></msrow><msline/><msrow><mn>2</mn><mn>8</mn><mn>0</mn></msrow></mstack></math>"#;
    test(
        "en",
        "ClearSpeak",
        expr,
        "subtraction problem; 505 minus 225, equals; 280; end of problem",
    )?;
    return Ok(());
}

#[test]
fn simple_longdiv() -> Result<()> {
    let expr = r#"<math><mlongdiv><mn>3</mn><mn>435</mn><mn>1306</mn></mlongdiv></math>"#;
    test(
        "en",
        "ClearSpeak",
        expr,
        "long division problem 1306 divided by 3 equals 435, end long division",
    )?;
    return Ok(());
}

#[test]
fn carry_row() -> Result<()> {
    let expr = r#"<math display="block"><mstack stackalign="right">
      <mscarries><mn>1</mn><none/><none/></mscarries>
      <msrow><mn>8</mn><mn>3</mn></msrow>
      <msrow><mo>+</mo><mrow/><mn>4</mn><mn>5</mn></msrow>
      <msline/>
      <msrow><mn>1</mn><mn>2</mn><mn>8</mn></msrow>
      </mstack></math>"#;
    test(
        "en",
        "ClearSpeak",
        expr,
        "addition problem; carry 1 in the hundreds column; 83 plus 45, equals; 128; end of problem",
    )?;
    return Ok(());
}
