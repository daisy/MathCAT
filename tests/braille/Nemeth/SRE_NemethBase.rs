use crate::common::*;
use anyhow::Result;

#[test]
fn test_00() -> Result<()> {
    let expr = "<math><mo>∗<!-- ∗ --></mo></math>";
    test_braille("Nemeth", expr, "⠈⠼")?;
    return Ok(());

}

#[test]
fn test_01() -> Result<()> {
    let expr = "<math><mo>†<!-- † --></mo></math>";
    test_braille("Nemeth", expr, "⠸⠻")?;
    return Ok(());

}

#[test]
fn test_02() -> Result<()> {
    let expr = "<math><mo>‡<!-- ‡ --></mo></math>";
    test_braille("Nemeth", expr, "⠸⠸⠻")?;
    return Ok(());

}

#[test]
fn test_03() -> Result<()> {
    let expr = "<math><mi mathvariant=\"normal\">§<!-- § --></mi></math>";
    test_braille("Nemeth", expr, "⠈⠠⠎")?;
    return Ok(());

}

#[test]
#[ignore]
fn test_04() -> Result<()> {
    let expr = "<math><mi mathvariant=\"normal\">§<!-- § --></mi><mi mathvariant=\"normal\">§<!-- § --></mi></math>";
    // no example -- on p52, there is no space when the double section mark is shown
    test_braille("Nemeth", expr, "⠈⠠⠎⠀⠈⠠⠎")?;
    return Ok(());

}

#[test]
fn test_05() -> Result<()> {
    let expr = "<math><mo>☆</mo></math>";
    test_braille("Nemeth", expr, "⠫⠎")?;
    return Ok(());

}

#[test]
#[ignore]  // not math (reference symbol) 46_1
fn test_06() -> Result<()> {
    let expr = "<math>
        <msup><mtext>A Cantor</mtext><mo>∗<!-- ∗ --></mo></msup><mtext> set is</mtext><mo>…<!-- … --></mo></math>";
    test_braille("Nemeth", expr, "⠠⠁⠀⠠⠉⠁⠝⠞⠕⠗⠘⠈⠼⠐⠀⠎⠑⠞⠀⠊⠎⠀⠄⠄⠄")?;
    return Ok(());

}

#[test]
fn test_07() -> Result<()> {
    let expr = "<math><mi>f</mi><mo>∗<!-- ∗ --></mo><mi>g</mi></math>";
    test_braille("Nemeth", expr, "⠋⠈⠼⠛")?;
    return Ok(());

}

#[test]
#[ignore]  // not math (reference symbol) 47_1
fn test_08() -> Result<()> {
    let expr = "<math>
        <msup><mtext>Find the index</mtext><mn>1</mn></msup><mtext> of the radical.</mtext></math>";
    test_braille("Nemeth", expr, "⠠⠋⠊⠝⠙⠀⠞⠓⠑⠀⠊⠝⠙⠑⠭⠘⠂⠐⠀⠕⠋⠀⠞⠓⠑⠀⠗⠁⠙⠊⠉⠁⠇⠨")?;
    return Ok(());

}

#[test]
#[ignore]  // not math (reference symbol) 48_a_1
fn test_09() -> Result<()> {
    let expr = "<math>
        <msup><mi></mi><mo>∗<!-- ∗ --></mo></msup><mtext>Irrational numbers</mtext><mo>…<!-- … --></mo></math>";
    test_braille("Nemeth", expr, "⠘⠈⠼⠐⠠⠊⠗⠗⠁⠞⠊⠕⠝⠁⠇⠀⠝⠥⠍⠃⠑⠗⠎⠀⠄⠄⠄")?;
    return Ok(());

}

#[test]
#[ignore]  // not math (reference symbol) 48_a_2
fn test_10() -> Result<()> {
    let expr = "<math>
        <msup><mtext>Irrational</mtext><mo>∗<!-- ∗ --></mo></msup><mtext> numbers</mtext><mo>…<!-- … --></mo></math>";
    test_braille("Nemeth", expr, "⠠⠊⠗⠗⠁⠞⠊⠕⠝⠁⠇⠘⠈⠼⠐⠀⠝⠥⠍⠃⠑⠗⠎⠀⠄⠄⠄")?;
    return Ok(());

}

#[test]
#[ignore]  // not math (reference symbol) 48_a_3
fn test_11() -> Result<()> {
    let expr = "<math>
        <msup><mi></mi><mo>∗<!-- ∗ --></mo></msup><mtext> Irrational numbers</mtext><mo>…<!-- … --></mo></math>";
    test_braille("Nemeth", expr, "⠘⠈⠼⠐⠀⠠⠊⠗⠗⠁⠞⠊⠕⠝⠁⠇⠀⠝⠥⠍⠃⠑⠗⠎⠀⠄⠄⠄")?;
    return Ok(());

}

#[test]
#[ignore]  // not math (reference symbol) 48_a_4
fn test_12() -> Result<()> {
    let expr = "<math><mo>…<!-- … --></mo>
        <msup><mtext>sets.</mtext><mo>∗<!-- ∗ --></mo></msup></math>";
    test_braille("Nemeth", expr, "⠀⠄⠄⠄⠎⠑⠞⠎⠨⠈⠼")?;
    return Ok(());

}

#[test]
#[ignore]  // not math (reference symbol) 48_a_5
fn test_13() -> Result<()> {
    let expr = "<math><mo>…<!-- … --></mo>
        <msup><mtext>sets</mtext><mo>∗<!-- ∗ --></mo></msup><mo>.</mo></math>";
    test_braille("Nemeth", expr, "⠀⠄⠄⠄⠀⠎⠑⠞⠎⠈⠼⠨⠐")?;
    return Ok(());

}

#[test]
#[ignore]  // not math (reference symbol) 48_b_2
fn test_14() -> Result<()> {
    let expr = "<math>
        <msup><mtext>A Cantor</mtext><mn>1</mn></msup><mtext> set is</mtext><mo>…<!-- … --></mo></math>";
    test_braille("Nemeth", expr, "⠠⠁⠀⠠⠉⠁⠝⠞⠕⠗⠘⠂⠐⠀⠎⠑⠞⠀⠊⠎⠀⠄⠄⠄")?;
    return Ok(());

}

#[test]
#[ignore]  // not math (reference symbol) 48_b_3
fn test_15() -> Result<()> {
    let expr = "<math><msup><mi></mi><mo>∗<!-- ∗ --></mo></msup><mn>10.</mn></math>";
    test_braille("Nemeth", expr, "⠘⠈⠼⠐⠂⠴⠨")?;
    return Ok(());

}

#[test]
#[ignore]  // not math (reference symbol) 48_b_4
fn test_16() -> Result<()> {
    let expr = "<math><msup><mn>1</mn><mo>∗<!-- ∗ --></mo></msup><mo>.</mo></math>";
    test_braille("Nemeth", expr, "⠼⠂⠘⠈⠼⠨⠐")?;
    return Ok(());

}

#[test]
#[ignore]  // not math (reference symbol) 48_b_5
fn test_17() -> Result<()> {
    let expr = "<math><msup><mn>1.</mn><mo>∗<!-- ∗ --></mo></msup></math>";
    test_braille("Nemeth", expr, "⠼⠂⠨⠘⠈⠼")?;
    return Ok(());

}

#[test]
#[ignore]  // not math (reference symbol) 48_b_6
fn test_18() -> Result<()> {
    let expr = "<math>
        <msup><mi></mi><mo>∗<!-- ∗ --></mo></msup><mtext> For extra credit.</mtext></math>";
    test_braille("Nemeth", expr, "⠘⠈⠼⠐⠀⠠⠋⠕⠗⠀⠑⠭⠞⠗⠁⠀⠉⠗⠑⠙⠊⠞⠨")?;
    return Ok(());

}

#[test]
fn test_19() -> Result<()> {
    let expr = "<math><mn>0</mn></math>";
    test_braille("Nemeth", expr, "⠼⠴")?;
    return Ok(());

}

#[test]
fn test_20() -> Result<()> {
    let expr = "<math><mn>1</mn></math>";
    test_braille("Nemeth", expr, "⠼⠂")?;
    return Ok(());

}

#[test]
fn test_21() -> Result<()> {
    let expr = "<math><mn>2</mn></math>";
    test_braille("Nemeth", expr, "⠼⠆")?;
    return Ok(());

}

#[test]
fn test_22() -> Result<()> {
    let expr = "<math><mn>3</mn></math>";
    test_braille("Nemeth", expr, "⠼⠒")?;
    return Ok(());

}

#[test]
fn test_23() -> Result<()> {
    let expr = "<math><mn>4</mn></math>";
    test_braille("Nemeth", expr, "⠼⠲")?;
    return Ok(());

}

#[test]
fn test_24() -> Result<()> {
    let expr = "<math><mn>5</mn></math>";
    test_braille("Nemeth", expr, "⠼⠢")?;
    return Ok(());

}

#[test]
fn test_25() -> Result<()> {
    let expr = "<math><mn>6</mn></math>";
    test_braille("Nemeth", expr, "⠼⠖")?;
    return Ok(());

}

#[test]
fn test_26() -> Result<()> {
    let expr = "<math><mn>7</mn></math>";
    test_braille("Nemeth", expr, "⠼⠶")?;
    return Ok(());

}

#[test]
fn test_27() -> Result<()> {
    let expr = "<math><mn>8</mn></math>";
    test_braille("Nemeth", expr, "⠼⠦")?;
    return Ok(());

}

#[test]
fn test_28() -> Result<()> {
    let expr = "<math><mn>9</mn></math>";
    test_braille("Nemeth", expr, "⠼⠔")?;
    return Ok(());

}

#[test]
fn test_29() -> Result<()> {
    let expr = "<math><mo>,</mo></math>";
    test_braille("Nemeth", expr, "⠠")?;
    return Ok(());

}

#[test]
#[ignore]
fn test_30() -> Result<()> {
    // needs some context to know what is correct
    let expr = "<math><mo>.</mo></math>";
    test_braille("Nemeth", expr, "⠨⠐")?;
    return Ok(());

}

#[test]
#[ignore]  // duplicate
fn test_31() -> Result<()> {
    let expr = "<math><mo>.</mo></math>";
    test_braille("Nemeth", expr, "⠨⠐")?;
    return Ok(());

}

#[test]
fn test_32() -> Result<()> {
    let expr = "<math><mo>,</mo></math>";
    test_braille("Nemeth", expr, "⠠")?;
    return Ok(());

}

#[test]
fn test_33() -> Result<()> {
    // 8_a_1
    let expr = "<math><mn>1</mn><mo>,</mo><mn>378</mn></math>";
    // corrected
    test_braille("Nemeth", expr, "⠼⠂⠠⠒⠶⠦")?;
    return Ok(());

}

#[test]
fn test_34() -> Result<()> {
    // 8_a_2
    let expr = "<math><mn>1.378</mn></math>";
    test_braille("Nemeth", expr, "⠼⠂⠨⠒⠶⠦")?;
    return Ok(());

}

#[test]
fn test_35() -> Result<()> {
    // 8_a_3
    let expr = "<math><mn>3.76</mn></math>";
    test_braille("Nemeth", expr, "⠼⠒⠨⠶⠖")?;
    return Ok(());

}

#[test]
fn test_36() -> Result<()> {
    // 8_a_4
    let expr = "<math><mn>3</mn><mo>,</mo><mn>76</mn></math>";
    test_braille("Nemeth", expr, "⠼⠒⠠⠀⠼⠶⠖")?;
    return Ok(());

}

#[test]
fn test_37() -> Result<()> {
    // 8_b_1
    let expr = "<math><mn>1</mn><mo>,</mo><mn>478</mn></math>";
    // corrected
    test_braille("Nemeth", expr, "⠼⠂⠠⠲⠶⠦")?;
    return Ok(());

}

#[test]
fn test_38() -> Result<()> {
    // improper markup changed to include spaces -- MathJax markup using \; for the extra space
    // let expr = "<math><mn>100</mn><mo>,</mo><mn>200</mn><mo>,</mo><mn>300</mn></math>";
    let expr = "<math><mn>100</mn><mo>,</mo><mspace width=\"0.278em\"/><mn>200</mn><mo>,</mo><mspace width=\"0.278em\"/><mn>300</mn></math>";
    test_braille("Nemeth", expr, "⠼⠂⠴⠴⠠⠀⠼⠆⠴⠴⠠⠀⠼⠒⠴⠴")?;
    return Ok(());

}
#[test]
fn test_38_wiris() -> Result<()> {
    // improper markup changed to include spaces -- WIRIS output with space
    let expr = "<math><mn>100</mn><mo>,</mo><mo>&#xA0;</mo><mn>200</mn><mo>,</mo><mo>&#xA0;</mo><mn>300</mn></math>";
    test_braille("Nemeth", expr, "⠼⠂⠴⠴⠠⠀⠼⠆⠴⠴⠠⠀⠼⠒⠴⠴")?;
    return Ok(());

}

#[test]
fn test_39() -> Result<()> {
    // 8_c_1
    let expr = "<math><mn>.35</mn></math>";
    test_braille("Nemeth", expr, "⠼⠨⠒⠢")?;
    return Ok(());

}

#[test]
fn test_40() -> Result<()> {
    // 8_c_2
    let expr = "<math><mn>3.14</mn></math>";
    test_braille("Nemeth", expr, "⠼⠒⠨⠂⠲")?;
    return Ok(());

}

#[test]
fn test_41() -> Result<()> {
    // 8_c_3
    let expr = "<math><mn>.2</mn><msub><mi>a</mi><mn>1</mn></msub><msub><mi>a</mi><mn>2</mn></msub><msub><mi>a</mi><mn>3</mn></msub></math>";
    test_braille("Nemeth", expr, "⠼⠨⠆⠁⠂⠁⠆⠁⠒")?;
    return Ok(());

}

#[test]
fn test_42() -> Result<()> {
    // 8_c_4
    let expr = "<math><mo>.</mo><msub><mi>a</mi><mn>1</mn></msub><msub><mi>a</mi><mn>2</mn></msub><msub><mi>a</mi><mn>3</mn></msub></math>";
    test_braille("Nemeth", expr, "⠨⠐⠁⠂⠁⠆⠁⠒")?;
    return Ok(());

}

#[test]
fn test_43() -> Result<()> {
    // 8_c_5
    let expr = "<math><mn>.1</mn><mo>+</mo><mn>.2</mn><mo>=</mo><mo>.</mo><mstyle displaystyle=\"false\" scriptlevel=\"0\"><mtext>---</mtext></mstyle></math>";
    // corrected
    test_braille("Nemeth", expr, "⠼⠨⠂⠬⠨⠆⠀⠨⠅⠀⠨⠐⠀⠤⠤⠤⠤")?;
    return Ok(());

}

#[test]
fn test_44() -> Result<()> {
    // 9_a_1
    let expr = "<math><mn>27</mn></math>";
    test_braille("Nemeth", expr, "⠼⠆⠶")?;
    return Ok(());

}

#[test]
#[ignore] // text (with contractions)
fn test_45() -> Result<()> {
    // 9_a_2
    let expr = "<math><mtext>There were </mtext><mn>7</mn><mtext> balls</mtext><mo>.</mo></math>";
    test_braille("Nemeth", expr, "⠠⠞⠓⠑⠗⠑⠀⠺⠑⠗⠑⠀⠼⠶⠀⠃⠁⠇⠇⠎⠨⠐")?;
    return Ok(());

}

#[test]
fn test_46() -> Result<()> {
    // 9_a_3
    let expr = "<math><mn>1</mn><mo>+</mo><mi>x</mi><mo>+</mo><mi>y</mi><mo>=</mo><mn>0</mn></math>";
    test_braille("Nemeth", expr, "⠼⠂⠬⠭⠬⠽⠀⠨⠅⠀⠼⠴")?;
    return Ok(());

}

#[test]
fn test_47() -> Result<()> {
    let expr = "<math><mi>y</mi><mo>=</mo><mn>2</mn><mi>sin</mi><mo>⁡<!-- ⁡ --></mo><mi>x</mi></math>";
    test_braille("Nemeth", expr, "⠽⠀⠨⠅⠀⠼⠆⠎⠊⠝⠀⠭")?;
    return Ok(());

}

#[test]
#[ignore]  // missing input
fn test_48() -> Result<()> {
    let expr = "<math></math>";
    test_braille("Nemeth", expr, "")?;
    return Ok(());

}

#[test]
fn test_49() -> Result<()> {
    // 9_a_5
    let expr = "<math><mi>sin</mi><mo>⁡<!-- ⁡ --></mo><mn>1</mn></math>";
    test_braille("Nemeth", expr, "⠎⠊⠝⠀⠼⠂")?;
    return Ok(());

}

#[test]
fn test_50() -> Result<()> {
    // 9_a_6
    let expr = "<math><msup><mi>sin</mi><mn>2</mn></msup><mo>⁡<!-- ⁡ --></mo><mn>2</mn><mi>x</mi></math>";
    test_braille("Nemeth", expr, "⠎⠊⠝⠘⠆⠀⠼⠆⠭")?;
    return Ok(());

}

#[test]
fn test_51() -> Result<()> {
    // 9_a_7
    let expr = "<math><mn>0.333</mn><mo>…<!-- … --></mo><mn>3</mn><mo>…<!-- … --></mo></math>";
    test_braille("Nemeth", expr, "⠼⠴⠨⠒⠒⠒⠀⠄⠄⠄⠀⠼⠒⠀⠄⠄⠄")?;
    return Ok(());

}

#[test]
fn test_52() -> Result<()> {
    // 9_a_9
    let expr = "<math><mi mathvariant=\"normal\">∠<!-- ∠ --></mi><mn>1</mn></math>";
    test_braille("Nemeth", expr, "⠫⠪⠀⠼⠂")?;
    return Ok(());

}

#[test]
fn test_53() -> Result<()> {
    // 9_a_10
    let expr = "<math><mo stretchy=\"false\">(</mo><mi>x</mi><mo>=</mo><mn>0</mn><mo stretchy=\"false\">)</mo></math>";
    test_braille("Nemeth", expr, "⠷⠭⠀⠨⠅⠀⠼⠴⠾")?;
    return Ok(());

}

#[test]
fn test_54() -> Result<()> {
    // 9_a_11
    let expr = "<math><mfrac><mn>11</mn><mn>5</mn></mfrac></math>";
    // this is the linear form, but the book does 2D
    test_braille("Nemeth", expr, "⠹⠂⠂⠌⠢⠼")?;
    return Ok(());

}

#[test]
fn test_55() -> Result<()> {
    // 9_a_12
    let expr = "<math>
        <mfrac>
        <mfrac>
        <mrow><mn>1</mn><mo>+</mo><mn>3</mn></mrow>
        <mrow><mn>4</mn><mo>+</mo><mn>5</mn></mrow></mfrac>
        <mfrac>
        <mrow><mn>3</mn><mo>+</mo><mn>4</mn></mrow>
        <mrow><mn>5</mn><mo>+</mo><mn>6</mn></mrow></mfrac></mfrac></math>";
    // this is the linear form, but the book does 2D
    test_braille("Nemeth", expr, "⠠⠹⠹⠂⠬⠒⠌⠲⠬⠢⠼⠠⠌⠹⠒⠬⠲⠌⠢⠬⠖⠼⠠⠼")?;
    return Ok(());

}

#[test]
fn test_56() -> Result<()> {
    // 9_a_13
    let expr = "<math>
        <mfrac>
        <mfrac>
        <mrow><mo stretchy=\"false\">(</mo><mn>1</mn><mo>−<!-- − --></mo><mi>x</mi><mo stretchy=\"false\">)</mo>
        <mfrac><mi>d</mi>
        <mrow><mi>d</mi><mi>x</mi></mrow></mfrac><mo stretchy=\"false\">(</mo><mn>2</mn><mi>x</mi><mo stretchy=\"false\">)</mo><mo>−<!-- − --></mo><mn>2</mn><mi>x</mi>
        <mfrac><mi>d</mi>
        <mrow><mi>d</mi><mi>x</mi></mrow></mfrac><mo stretchy=\"false\">(</mo><mn>1</mn><mo>−<!-- − --></mo><mi>x</mi><mo stretchy=\"false\">)</mo></mrow>
        <mrow><mo stretchy=\"false\">(</mo><mn>1</mn><mo>−<!-- − --></mo><mi>x</mi>
        <msup><mo stretchy=\"false\">)</mo><mn>2</mn></msup></mrow></mfrac>
        <mrow><mn>1</mn><mo>+</mo><mo stretchy=\"false\">(</mo>
        <mfrac>
        <mrow><mn>2</mn><mi>x</mi></mrow>
        <mrow><mn>1</mn><mo>−<!-- − --></mo><mi>x</mi></mrow></mfrac>
        <msup><mo stretchy=\"false\">)</mo><mn>2</mn></msup></mrow></mfrac></math>";
    // this is the linear form, but the book does 2D
    test_braille("Nemeth", expr, "⠠⠠⠹⠠⠹⠷⠂⠤⠭⠾⠹⠙⠌⠙⠭⠼⠷⠆⠭⠾⠤⠆⠭⠹⠙⠌⠙⠭⠼⠷⠂⠤⠭⠾⠠⠌⠷⠂⠤⠭⠾⠘⠆⠐⠠⠼⠠⠠⠌⠂⠬⠷⠹⠆⠭⠌⠂⠤⠭⠼⠾⠘⠆⠐⠠⠠⠼")?;
    return Ok(());

}

#[test]
fn test_57() -> Result<()> {
    // 9_a_14
    let expr = "<math><mo>−<!-- − --></mo><mn>1</mn></math>";
    test_braille("Nemeth", expr, "⠤⠼⠂")?;
    return Ok(());

}

#[test]
fn test_58() -> Result<()> {
    // 9_a_15
    let expr = "<math><mo>−<!-- − --></mo><mn>.3</mn></math>";
    test_braille("Nemeth", expr, "⠤⠼⠨⠒")?;
    return Ok(());

}

#[test]
fn test_59() -> Result<()> {
    // 9_b_1
    // correct input to use open/close quotes as in book
    let expr = "<math><mo>“</mo><mn>3</mn><mstyle displaystyle=\"false\" scriptlevel=\"0\"><mtext> dogs</mtext></mstyle><mo>”</mo></math>";
    // corrected
    test_braille("Nemeth", expr, "⠦⠼⠒⠀⠙⠕⠛⠎⠴")?;
    return Ok(());

}

#[test]
#[ignore] // text (has contraction)
fn test_60() -> Result<()> {
    // 9_b_2
    // probably input is not a good match
    let expr = "<math><mtext>Probability ---</mtext><mn>0</mn></math>";
    test_braille("Nemeth", expr, "⠠⠏⠗⠕⠃⠁⠃⠊⠇⠊⠞⠽⠀⠤⠤⠤⠼⠴")?;
    return Ok(());

}

#[test]
fn test_61() -> Result<()> {
    // 9_b_3
    // corrected input to use open quote
    let expr = "<math><mo>“</mo><mn>.5</mn></math>";
    // corrected
    test_braille("Nemeth", expr, "⠦⠼⠨⠢")?;
    return Ok(());

}

#[test]
fn test_62() -> Result<()> {
    // 9_b_4
    // corrected input to use open quote
    let expr = "<math><mo>“</mo><mo>−<!-- − --></mo><mn>4</mn></math>";
    // corrected
    test_braille("Nemeth", expr, "⠦⠤⠼⠲")?;
    return Ok(());

}

#[test]
#[ignore] // not clear what linear output should be
fn test_63() -> Result<()> {
    // 9_c_1
    let expr = "<math>
        <mrow><mo>|</mo><mtable columnspacing=\"1em\" rowspacing=\"4pt\"><mtr><mtd><mn>1</mn></mtd><mtd><mn>2</mn></mtd></mtr><mtr><mtd><mo>−<!-- − --></mo><mn>3</mn></mtd><mtd><mo>−<!-- − --></mo><mn>4</mn></mtd></mtr></mtable><mo>|</mo></mrow></math>";
    // this is the linear form, but the book does 2D
    test_braille("Nemeth", expr, "⠠⠳⠼⠂⠀⠼⠆⠠⠳⠀⠠⠳⠤⠼⠒⠀⠤⠼⠲⠠⠳")?;
    return Ok(());

}

#[test]
#[ignore] // not clear what linear output should be
fn test_64() -> Result<()> {
    // 9_c_2
    let expr = "<math>
        <mrow><mo>|</mo><mtable columnspacing=\"1em\" rowspacing=\"4pt\"><mtr><mtd><mn>1</mn></mtd><mtd>
        <mfrac><mn>1</mn><mn>2</mn></mfrac></mtd></mtr><mtr><mtd>
        <mfrac><mn>1</mn><mn>2</mn></mfrac></mtd><mtd>
        <mfrac><mn>1</mn>
        <mrow><mn>4</mn><mi>r</mi></mrow></mfrac></mtd></mtr></mtable><mo>|</mo></mrow></math>";
    // this is the linear form, but the book does 2D
    test_braille("Nemeth", expr, "⠠⠳⠼⠂⠀⠹⠂⠌⠆⠼⠠⠳⠀⠠⠳⠹⠂⠌⠆⠼⠀⠹⠂⠌⠲⠗⠼⠠⠳")?;
    return Ok(());

}

#[test]
fn test_65() -> Result<()> {
    // 9_d_1
    let expr = "<math><mn>3</mn><mi mathvariant=\"normal\">§<!-- § --></mi><mn>4</mn></math>";
    // corrected
    test_braille("Nemeth", expr, "⠼⠒⠈⠠⠎⠼⠲")?;
    return Ok(());

}

#[test]
fn test_66() -> Result<()> {
    // 9_d_2
    let expr = "<math><mn>3</mn><mi mathvariant=\"normal\">#<!-- # --></mi><mn>4</mn></math>";
    // corrected
    test_braille("Nemeth", expr, "⠼⠒⠨⠼⠼⠲")?;
    return Ok(());

}

#[test]
fn test_67() -> Result<()> {
    // 9_d_3
    let expr = "<math><mn>3</mn><mo>∗<!-- ∗ --></mo><mn>4</mn></math>";
    // corrected
    test_braille("Nemeth", expr, "⠼⠒⠈⠼⠼⠲")?;
    return Ok(());

}

#[test]
#[ignore]  // text reference
fn test_68() -> Result<()> {
    // 9_d_4
    let expr = "<math><mtext>See page </mtext>
    <msup><mn>15</mn><mn>1</mn></msup><mo>.</mo></math>";
    test_braille("Nemeth", expr, "⠠⠎⠑⠑⠀⠏⠁⠛⠑⠀⠼⠂⠢⠘⠂⠨⠐")?;
    return Ok(());

}

#[test]
#[ignore]  // text reference
fn test_69() -> Result<()> {
    // 9_d_5
    let expr = "<math><mo>†<!-- † --></mo><mn>3</mn></math>";
    test_braille("Nemeth", expr, "⠸⠻⠼⠒")?;
    return Ok(());

}

#[test]
fn test_70() -> Result<()> {
    // 9_e_1
    let expr = "<math><mn mathvariant=\"italic\">3</mn></math>";
    test_braille("Nemeth", expr, "⠨⠼⠒")?;
    return Ok(());

}

#[test]
fn test_71() -> Result<()> {
    // 9_e_2
    let expr = "<math><mn mathvariant=\"bold\">0</mn></math>";
    test_braille("Nemeth", expr, "⠸⠼⠴")?;
    return Ok(());

}

#[test]
fn test_72() -> Result<()> {
    // 9_e_3
    let expr = "<math><mn mathvariant=\"italic\">.3</mn></math>";
    // corrected -- added '.' after numeric indicator
    test_braille("Nemeth", expr, "⠨⠼⠨⠒")?;
    return Ok(());

}

#[test]
fn test_73() -> Result<()> {
    // 9_e_4
    let expr = "<math><mn mathvariant=\"script\">2</mn></math>";
    test_braille("Nemeth", expr, "⠈⠼⠆")?;
    return Ok(());

}

#[test]
fn test_74() -> Result<()> {
    // 9_e_5
    let expr = "<math><mn mathvariant=\"bold\">43</mn><mn mathvariant=\"bold\">56</mn></math>";
    test_braille("Nemeth", expr, "⠸⠼⠲⠒⠸⠼⠢⠖")?;
    return Ok(());

}

#[test]
#[ignore]  // missing input
fn test_75() -> Result<()> {
    let expr = "<math></math>";
    test_braille("Nemeth", expr, "")?;
    return Ok(());

}

#[test]
#[ignore]  // missing input
fn test_76() -> Result<()> {
    let expr = "<math></math>";
    test_braille("Nemeth", expr, "")?;
    return Ok(());

}

#[test]
fn test_77() -> Result<()> {
    // 9_f_1
    let expr = "<math>
        <mrow>
        <mrow><mn>1</mn><mo>−<!-- − --></mo></mrow><mtext>to</mtext>
        <mrow><mo>−<!-- − --></mo><mn>1</mn></mrow><mtext> correspondence</mtext></mrow></math>";
    test_braille("Nemeth", expr, "⠼⠂⠤⠞⠕⠤⠼⠂⠀⠉⠕⠗⠗⠑⠎⠏⠕⠝⠙⠑⠝⠉⠑")?;
    return Ok(());

}

