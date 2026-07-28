// Nemeth tests for the basic mathml tags
// The numbering refers to the Nemeth green book in most cases.
// The newer NFB lessons include NFB (https://nfb.org/programs-services/braille-certification/mathematics-braille-transcribing)
// These lessons are still being developed, so it is possible the numbering gets changed from that used here.

// New source: https://www.brailleauthority.org/sites/default/files/2024-02/Nemeth_2022.pdf
use crate::common::*;
use anyhow::Result;

#[test]
fn num_indicator_9_a_1() -> Result<()> {
    let expr = "<math><mn>27</mn></math>";
    test_braille("Nemeth", expr, "⠼⠆⠶")?;
    return Ok(());
}

#[test]
fn num_indicator_9_a_4() -> Result<()> {
    let expr = "<math><mrow><mi>y</mi><mo>=</mo><mrow><mn>2</mn><mo>&#x2062;</mo><mrow><mi>sin</mi><mo>&#x2061;</mo><mi>x</mi></mrow></mrow></mrow></math>";
    test_braille("Nemeth", expr, "⠽⠀⠨⠅⠀⠼⠆⠎⠊⠝⠀⠭")?;
    return Ok(());
}

#[test]
fn num_indicator_9_a_5() -> Result<()> {
    let expr = "<math><mrow><mi>sin</mi><mo>&#x2061;</mo><mn>1</mn></mrow></math>";
    test_braille("Nemeth", expr, "⠎⠊⠝⠀⠼⠂")?;
    return Ok(());
}

#[test]
fn num_indicator_9_a_9() -> Result<()> {
    let expr = "<math><mrow><mo>∠</mo><mn>1</mn></mrow></math>";
    test_braille("Nemeth", expr, "⠫⠪⠀⠼⠂")?;
    return Ok(());
}

#[test]
fn num_indicator_9_a_14() -> Result<()> {
    let expr = "<math><mo>-</mo><mn>1</mn></math>";
    test_braille("Nemeth", expr, "⠤⠼⠂")?;
    return Ok(());
}

#[test]
fn num_indicator_9_a_15() -> Result<()> {
    let expr = "<math><mo>-</mo><mn>.3</mn></math>";
    test_braille("Nemeth", expr, "⠤⠼⠨⠒")?;
    return Ok(());
}

#[test]
fn test_9_b_1() -> Result<()> {
    let expr = "<math><mrow><mo>“</mo><mrow><mn>3</mn><mtext>&#xa0;dogs</mtext><mo>”</mo></mrow></mrow></math>";
    test_braille("Nemeth", expr, "⠦⠼⠒⠀⠙⠕⠛⠎⠴")?;
    return Ok(());
}

#[test]
fn test_9_b_1_mtext() -> Result<()> {
    let expr = "<math><mtext displaystyle='true'>&#x201C;3 dogs&#x201D;</mtext></math>";
    test_braille("Nemeth", expr, "⠦⠼⠒⠀⠙⠕⠛⠎⠴")?;
    return Ok(());
}

#[test]
fn test_9_b_4() -> Result<()> {
    let expr = "<math><mrow><mo>“</mo><mrow><mo>-</mo><mn>4</mn></mrow></mrow></math>";
    test_braille("Nemeth", expr, "⠦⠤⠼⠲")?;
    return Ok(());
}
#[test]
fn test_9_c_1_linear() -> Result<()> {
    // see https://github.com/NSoiffer/MathCAT/issues/43 for discussion on linear layout
    let expr = "<math><mo>|</mo><mtable>
            <mtr><mtd><mn>1</mn></mtd><mtd><mn>2</mn></mtd></mtr>
            <mtr><mtd><mo>-</mo><mn>3</mn></mtd><mtd><mo>-</mo><mn>4</mn></mtd></mtr>
        </mtable><mo>|</mo></math>";
    test_braille("Nemeth", expr, "⠠⠳⠼⠂⠀⠼⠆⠀⣍⠤⠒⠀⠤⠼⠲⠠⠳")?;
    return Ok(());
}

#[test]
fn num_indicator_9_d_2() -> Result<()> {
    let expr = "<math><mn>3</mn><mi mathvariant='normal'>#<!-- # --></mi><mn>4</mn></math>";
    test_braille("Nemeth", expr, "⠼⠒⠨⠼⠼⠲")?;
    return Ok(());
}

#[test]
fn num_indicator_9_d_3() -> Result<()> {
    let expr = "<math><mn>3</mn><mo>∗<!-- ∗ --></mo><mn>4</mn></math>";
    test_braille("Nemeth", expr, "⠼⠒⠈⠼⠼⠲")?;
    return Ok(());
}

#[test]
fn num_indicator_9_e_2() -> Result<()> {
    let expr = "<math><mn mathvariant='bold'>0</mn></math>";
    test_braille("Nemeth", expr, "⠸⠼⠴")?;
    return Ok(());
}

#[test]
fn num_indicator_9_e_5() -> Result<()> {
    let expr = "<math><mn>𝟒𝟑56</mn></math>";
    test_braille("Nemeth", expr, "⠸⠼⠲⠒⠼⠢⠖")?;
    return Ok(());
}

#[test]
fn num_indicator_9_e_6() -> Result<()> {
    let expr = "<math><mn>⑤</mn></math>";
    test_braille("Nemeth", expr, "⠫⠉⠸⠫⠼⠢⠻")?;
    return Ok(());
}

#[test]
fn num_indicator_9_f_1() -> Result<()> {
    // shortened to the math part (1-1) -- "correspondence" should probably be text and involves contractions
    let expr = "<math><mtext>1-to-1</mtext></math>";
    test_braille("Nemeth", expr, "⠼⠂⠤⠞⠕⠤⠼⠂")?;
    return Ok(());
}

#[test]
fn non_list_10_4() -> Result<()> {
    let expr = "<math><mo>(</mo><mn>1</mn><mo>,</mo><mn>2</mn><mo>,</mo><mtext>and&#xA0;</mtext><mn>3</mn><mo>)</mo></math>";
    test_braille("Nemeth", expr, "⠷⠂⠠⠀⠼⠆⠠⠀⠁⠝⠙⠀⠼⠒⠾")?;
    return Ok(());
}

#[test]
fn list_10_6_1() -> Result<()> {
    let expr = "<math><mrow><mn>1</mn><mo>,</mo><mi>i</mi><mo>,</mo><mrow><mo>−</mo><mn>1</mn></mrow><mo>,</mo><mrow><mo>−</mo><mi>i</mi></mrow></mrow></math>";
    test_braille("Nemeth", expr, "⠼⠂⠠⠀⠰⠊⠠⠀⠤⠼⠂⠠⠀⠤⠊")?;
    return Ok(());
}

#[test]
fn list_10_6_8() -> Result<()> {
    let expr = "<math>
        <mo>[</mo>
        <mrow><mo>&#x2220;</mo><mn>1</mn><mo>&#x00B0;</mo><mo>,</mo><mi>sin</mi><mn>1</mn><mo>&#x00B0;</mo></mrow>			<mo>]</mo>
    </math>";
    test_braille("Nemeth", expr, "⠈⠷⠫⠪⠀⠼⠂⠘⠨⠡⠠⠀⠎⠊⠝⠀⠼⠂⠘⠨⠡⠐⠈⠾")?;
    return Ok(());
}

#[test]
fn list_10_6_11() -> Result<()> {
    let expr = "<math><mo>(</mo>
          <mrow><mi>x</mi><mo>=</mo><mn>1</mn><mo>,</mo><mn>2</mn><mo>,</mo><mn>…</mn><mo>,</mo><mn>10</mn></mrow>
        <mo>)</mo></math>";
    test_braille("Nemeth", expr, "⠷⠭⠀⠨⠅⠀⠼⠂⠠⠀⠼⠆⠠⠀⠄⠄⠄⠠⠀⠼⠂⠴⠾")?;
    return Ok(());
}


#[test]
fn list_10_6_14() -> Result<()> {
    let expr = "<math><mfenced><mrow><mn>1</mn><mo>,</mo><mn>2</mn><mo>,</mo><mn>3</mn></mrow></mfenced></math>";
    test_braille("Nemeth", expr, "⠷⠂⠠⠀⠆⠠⠀⠒⠾")?;
    return Ok(());
}

#[test]
fn list_num_ind_11_a_1() -> Result<()> {
    let expr = "<math><mo>[</mo><mn>0</mn><mo>,</mo><mn>1</mn><mo>]</mo></math>";
    test_braille("Nemeth", expr, "⠈⠷⠴⠠⠀⠂⠈⠾")?;
    return Ok(());
}

#[test]
fn list_num_ind_11_a_2() -> Result<()> {
    let expr = "<math><mo>(</mo><mo>-</mo><mn>1</mn><mo>,</mo><mo>-</mo><mn>2</mn><mo>,</mo><mo>-</mo><mn>3</mn><mo>)</mo></math>";
    test_braille("Nemeth", expr, "⠷⠤⠂⠠⠀⠤⠆⠠⠀⠤⠒⠾")?;
    return Ok(());
}

#[test]
fn list_num_ind__11_a_3() -> Result<()> {
    let expr = "<math><mo>(</mo><mn>1</mn><mo>+</mo><mi>h</mi><mo>,</mo><mn>2</mn><mo>+</mo><mi>k</mi><mo>,</mo><mn>0</mn><mo>)</mo></math>";
    test_braille("Nemeth", expr, "⠷⠂⠬⠓⠠⠀⠆⠬⠅⠠⠀⠴⠾")?;
    return Ok(());
}


#[test]
fn list_num_ind__11_a_4() -> Result<()> {
    let expr = "<math><mfenced><mrow><mn>0</mn><mo>,</mo><mrow><mo>−</mo><mn>1</mn></mrow><mo>,</mo><mrow><mo>±</mo><mn>2</mn></mrow></mrow></mfenced></math>";
    test_braille("Nemeth", expr, "⠷⠴⠠⠀⠤⠂⠠⠀⠬⠤⠆⠾")?;
    return Ok(());
}

#[test]
fn list_num_ind_11_a_5() -> Result<()> {
    let expr = "<math><mfenced class='paren'>
    <mrow>
      <mrow><mn>2</mn><mo>⁢</mo><mrow><mi>sin</mi><mo>⁡</mo><mrow><mn>30</mn><mo>⁢</mo><mo>°</mo></mrow></mrow></mrow>
      <mo>,</mo>
      <mrow><mn>3</mn><mo>⁢</mo><mrow><mi>cos</mi><mo>⁡</mo><mrow><mn>60</mn><mo>⁢</mo><mo>°</mo></mrow></mrow></mrow>
    </mrow>
  </mfenced></math>";
    test_braille("Nemeth", expr, "⠷⠆⠎⠊⠝⠀⠼⠒⠴⠘⠨⠡⠠⠀⠒⠉⠕⠎⠀⠼⠖⠴⠘⠨⠡⠐⠾")?;
    return Ok(());
}

#[test]
fn lesson_11_35_1() -> Result<()> {
    // this is about using a numeric indicator inside an enclosed list after an angle
    let expr = "<math><mrow><mrow><mo>(</mo><mrow><mo>∠</mo><mn>1</mn><mo>,</mo><mo>∠</mo><mn>2</mn><mo>,</mo><mo>∠</mo><mn>3</mn></mrow><mo>)</mo></mrow></mrow></math>";
    test_braille("Nemeth", expr, "⠷⠫⠪⠀⠼⠂⠠⠀⠫⠪⠀⠼⠆⠠⠀⠫⠪⠀⠼⠒⠾")?;
    return Ok(());
}


#[test]
fn list_num_ind_11_a_7() -> Result<()> {
    let expr = "<math><mo>(</mo><mi>x</mi><mo>,</mo><mn>7</mn><mo>,</mo><mn mathvariant='bold'>8</mn><mo>,</mo><mi>y</mi><mo>)</mo></math>";
    test_braille("Nemeth", expr, "⠷⠭⠠⠀⠶⠠⠀⠸⠼⠦⠠⠀⠽⠾")?;
    return Ok(());
}

#[test]
fn list_num_ind_11_c_1() -> Result<()> {
    let expr = "<math><mi>&#x3C0;</mi><mo>=</mo><mn>3</mn><mo>.</mo><mn>14159</mn><mo>&#xA0;</mo><mn>26535</mn><mo>&#x2026;</mo></math>";
    test_braille("Nemeth", expr, "⠨⠏⠀⠨⠅⠀⠼⠒⠨⠂⠲⠂⠢⠔⠀⠆⠖⠢⠒⠢⠀⠄⠄⠄")?;
    return Ok(());
}

#[test]
fn list_num_ind_11_d_1() -> Result<()> {
    let expr = "<math><mrow><mn>65</mn><mo>-</mo><mn>75</mn></mrow></math>";
    test_braille("Nemeth", expr, "⠼⠖⠢⠤⠶⠢")?;
    return Ok(());
}

#[test]
fn list_num_ind_11_d_2() -> Result<()> {
    let expr = "<math><mn>3</mn><mo>:</mo><mn>30</mn><mo>&#x2212;</mo><mn>4</mn><mo>:</mo><mn>45</mn></math>";
    test_braille("Nemeth", expr, "⠼⠒⠸⠒⠼⠒⠴⠤⠲⠸⠒⠼⠲⠢")?;
    return Ok(());
}

#[test]
fn no_num_ind_11_e_3() -> Result<()> {
    let expr = "<math><mrow><mi>r</mi><mn>5</mn></mrow></math>";
    test_braille("Nemeth", expr, "⠗⠐⠢")?;
    return Ok(());
}

#[test]
fn cap_roman_numeral_18_a_3() -> Result<()> {
    let expr = "<math><mtext>VII</mtext><mo>+</mo><mtext>V</mtext><mo>=</mo><mtext>XII</mtext></math>";
    test_braille("Nemeth", expr, "⠠⠠⠧⠊⠊⠬⠠⠧⠀⠨⠅⠀⠠⠠⠭⠊⠊")?;
    return Ok(());
}

#[test]
fn lower_roman_numeral_18_b_4() -> Result<()> {
    let expr = "<math><mtext>vi</mtext><mo>+</mo><mtext>iv</mtext><mo>=</mo><mtext>x</mtext></math>";
    test_braille("Nemeth", expr, "⠧⠊⠬⠊⠧⠀⠨⠅⠀⠭")?;
    return Ok(());
}

#[test]
fn cap_22_a_1() -> Result<()> {
    // from WIRIS
    let expr = "<math><mo>&#x25B3;</mo><mo>&#xA0;</mo><mi>A</mi><mi>B</mi><mi>C</mi></math>";
    test_braille("Nemeth", expr, "⠫⠞⠀⠠⠁⠠⠃⠠⠉")?;
    return Ok(());
}

#[test]
fn greek_24_a_1() -> Result<()> {
    let expr = "<math><mi>α</mi></math>";
    test_braille("Nemeth", expr, "⠨⠁")?;
    return Ok(());
}

#[test]
fn german_24_a_7() -> Result<()> {
    let expr = "<math><mi>𝔄</mi></math>";
    test_braille("Nemeth", expr, "⠸⠠⠁")?;
    return Ok(());
}

#[test]
fn hebrew_24_a_8() -> Result<()> {
    let expr = "<math><msub><mi>ℵ</mi><mn>0</mn></msub></math>";
    test_braille("Nemeth", expr, "⠠⠠⠁⠴")?;
    return Ok(());
}

#[test]
fn russian_24_a_10() -> Result<()> {
    let expr = "<math><mi>А</mi></math>";
    test_braille("Nemeth", expr, "⠈⠈⠠⠁")?;
    return Ok(());
}

#[test]
fn greek_24_b_1_together() -> Result<()> {
    let expr = "<math><mi>αβ</mi></math>";
    test_braille("Nemeth", expr, "⠨⠁⠨⠃")?;
    return Ok(());
}

#[test]
fn greek_24_b_1() -> Result<()> {
    let expr = "<math><mi>α</mi><mi>β</mi></math>";
    test_braille("Nemeth", expr, "⠨⠁⠨⠃")?;
    return Ok(());
}

#[test]
fn eli_nemeth_UEB_rule_book_4_11_1() -> Result<()> {
    let expr = "<math>
            <mtext>p</mtext><mtext>&#xA0;</mtext>
            <mtext>D</mtext><mtext>&#xA0;</mtext>
            <mtext>z</mtext><mtext>&#xA0;</mtext>
            <mtext>R</mtext><mtext>&#xA0;</mtext>
            <mtext>x</mtext><mo>,</mo><mtext>&#xA0;</mtext>
            <mtext>“y”</mtext><mtext>&#xA0;</mtext>
            <mtext>“w S”</mtext><mtext>&#xA0;</mtext>
            <mtext>“x”</mtext><mo>+</mo><mtext>“y”</mtext>
        </math>";
    test_braille("Nemeth", expr, "⠰⠏⠀⠰⠠⠙⠀⠰⠵⠀⠰⠠⠗⠀⠰⠭⠠⠀⠦⠰⠽⠸⠴⠀⠦⠰⠺⠀⠰⠠⠎⠸⠴⠀⠦⠰⠭⠸⠴⠬⠸⠦⠰⠽⠸⠴")?;
    return Ok(());
}

#[test]
fn letter_26_b_18() -> Result<()> {
    let expr = "<math><mo>(</mo><mi>p</mi><mo>&#xA0;</mo><mtext>and</mtext><mo>&#xA0;</mo><mi>q</mi><mo>)</mo></math>";
    // Note: NFB lessons now say 'don't use contractions in Nemeth' -- example modified
    test_braille("Nemeth", expr, "⠷⠰⠏⠀⠁⠝⠙⠀⠰⠟⠾")?;
    return Ok(());
}

#[test]
fn letter_26_b_19() -> Result<()> {
    let expr = "<math><mo>(</mo>
            <mi mathvariant='normal'>l</mi><mo>,</mo>
            <mi mathvariant='normal'>m</mi><mo>,</mo>
            <mi mathvariant='normal'>n</mi><mo>,</mo>
            <mtext>are in set&#xa0;</mtext>
            <mi mathvariant='normal'>R</mi>
        <mo>)</mo></math>";
    // Note: NFB lessons now say 'don't use contractions in Nemeth' -- example modified (no contraction for "re" and "in")
    // This likely would use an escape to UEB for the "are in set" under the new rules
    test_braille("Nemeth", expr, "⠷⠰⠇⠠⠀⠰⠍⠠⠀⠰⠝⠠⠀⠁⠗⠑⠀⠊⠝⠀⠎⠑⠞⠀⠰⠠⠗⠾")?;
    return Ok(());
}

#[test]
fn boldface_32_a_7() -> Result<()> {
    let expr = "<math><mn mathvariant='bold-fraktur'>a</mn></math>";
    test_braille("Nemeth", expr, "⠸⠸⠁")?;
    return Ok(());
}

#[test]
fn boldface_32_a_14() -> Result<()> {
    let expr = "<math><mn mathvariant='sans-serif'>H</mn></math>";
    test_braille("Nemeth", expr, "⠠⠨⠰⠠⠓")?;
    return Ok(());
}

#[test]
fn boldface_32_b_2() -> Result<()> {
    let expr = "<math><mn mathvariant='script'>2</mn></math>";
    test_braille("Nemeth", expr, "⠈⠼⠆")?;
    return Ok(());
}

#[test]
fn boldface_32_b_3() -> Result<()> {
    let expr = "<math><mn mathvariant='bold'>345</mn></math>";
    test_braille("Nemeth", expr, "⠸⠼⠒⠲⠢")?;
    return Ok(());
}

#[test]
fn boldface_32_b_6() -> Result<()> {
    let expr = "<math><mn>𝟒35</mn></math>";
    test_braille("Nemeth", expr, "⠸⠼⠲⠼⠒⠢")?;
    return Ok(());
}

#[test]
fn punct_37_1_1() -> Result<()> {
    let expr = "<math>
            <mfrac><mn>1</mn><mn>2</mn></mfrac>
            <mo>,</mo>
            <mfrac><mn>3</mn><mn>4</mn></mfrac>
            <mo>.</mo>
        </math>";
    test_braille("Nemeth", expr, "⠹⠂⠌⠆⠼⠠⠀⠹⠒⠌⠲⠼⠸⠲")?;
    return Ok(());
}

#[test]
fn punct_37_1_2() -> Result<()> {
    let expr = "<math>
            <mover> <mi>velocity</mi> <mo>_</mo> </mover>
            <mtext>.</mtext>
        </math>";
    test_braille("Nemeth", expr, "⠐⠧⠑⠇⠕⠉⠊⠞⠽⠣⠱⠻⠸⠲")?;
    return Ok(());
}

#[test]
fn punct_37_2_2() -> Result<()> {
    let expr = "<math><mtext>“</mtext> <mn>49</mn> <mtext>”</mtext></math>";
    test_braille("Nemeth", expr, "⠦⠼⠲⠔⠸⠴")?;
    return Ok(());
}

#[test]
fn punct_37_3_1() -> Result<()> {
    let expr = "<math><mtext>I</mtext><mo>,</mo><mtext>II</mtext><mo>,</mo><mtext>III</mtext><mo>.</mo></math>";
    test_braille("Nemeth", expr, "⠰⠠⠊⠠⠀⠠⠠⠊⠊⠠⠀⠠⠠⠊⠊⠊⠸⠲")?;
    return Ok(());
}

#[test]
fn punct_37_4_2() -> Result<()> {
    let expr = "<math><mn>1</mn><mo>,</mo><mn>3</mn><mo>,</mo><mo>…</mo><mo>.</mo></math>";
    test_braille("Nemeth", expr, "⠼⠂⠠⠀⠼⠒⠠⠀⠄⠄⠄⠸⠲")?;
    return Ok(());
}

#[test]
fn punct_37_6_1() -> Result<()> {
    let expr = "<math><mn>5</mn><mo>×</mo><mn>3</mn><mo>=</mo><mo>?</mo><mo>.</mo></math>";
    test_braille("Nemeth", expr, "⠼⠢⠈⠡⠒⠀⠨⠅⠀⠿⠸⠲")?;
    return Ok(());
}

#[test]
fn punct_37_7_1() -> Result<()> {
    let expr = "<math><mi>a</mi><mo>,</mo><mi>b</mi><mo>,</mo><mi>c</mi><mo>.</mo></math>";
    test_braille("Nemeth", expr, "⠰⠁⠠⠀⠰⠃⠠⠀⠰⠉⠸⠲")?;
    return Ok(());
}

#[test]
fn punct_37_8_1() -> Result<()> {
    let expr = "<math><mo>&#x25B3;</mo><mi>A</mi><mi>B</mi><mi>C</mi><mo>.</mo></math>";
    test_braille("Nemeth", expr, "⠫⠞⠀⠠⠁⠠⠃⠠⠉⠸⠲")?;
    return Ok(());
}

#[test]
fn punct_37_11_1() -> Result<()> {
    let expr = "<math><mover><mi>x</mi><mo>&#xAF;</mo></mover><mo>.</mo></math>";
    test_braille("Nemeth", expr, "⠭⠱⠸⠲")?;
    return Ok(());
}

#[test]
fn punct_37_16_1() -> Result<()> {
    let expr = "<math><mn>100</mn><mo>%</mo><mo>.</mo></math>";
    test_braille("Nemeth", expr, "⠼⠂⠴⠴⠈⠴⠸⠲")?;
    return Ok(());
}

#[test]
fn punct_37_17_1() -> Result<()> {
    let expr = "<math><mn>0</mn><mo>,</mo><mo>”</mo></math>";
    test_braille("Nemeth", expr, "⠼⠴⠠⠸⠴")?;
    return Ok(());
}

#[test]
fn punct_38_1_2() -> Result<()> {
    let expr = "<math><mo>’</mo><mn>49</mn></math>";
    // Corrected: the green book has the quote mark encoded as if the character was a double quote (”), but this seems like a typo 
    test_braille("Nemeth", expr, "⠴⠠⠼⠲⠔")?;
    return Ok(());
}

#[test]
fn punct_38_4_12() -> Result<()> {
    let expr = "<math><mi>rate</mi><mo>×</mo><mi>time</mi><mo>.</mo></math>";
    test_braille("Nemeth", expr, "⠗⠁⠞⠑⠈⠡⠞⠊⠍⠑⠲")?;
    return Ok(());
}

#[test]
fn punct_38_6_1() -> Result<()> {
    let expr = "<math><mn>0</mn><mo>,</mo><mn>1</mn><mo>,</mo><mn>2</mn></math>";
    test_braille("Nemeth", expr, "⠼⠴⠠⠀⠼⠂⠠⠀⠼⠆")?;
    return Ok(());
}

#[test]
fn punct_38_6_3() -> Result<()> {
    let expr = "<math><mo>(</mo><mn>1</mn><mo>)</mo><mo>,</mo><mo>(</mo><mn>2</mn><mo>)</mo><mo>,</mo><mo>(</mo><mn>3</mn><mo>)</mo><mo>.</mo></math>";
    test_braille("Nemeth", expr, "⠷⠂⠾⠠⠀⠷⠆⠾⠠⠀⠷⠒⠾⠸⠲")?;
    return Ok(());
}

#[test]
fn colon_40_1() -> Result<()> {
    // including 'intent' is a little bit of a cheat, but there is no way to know whether this is time or ratio without it
    let expr = "<math><mn>3</mn><mo intent='time'>:</mo><mn>30</mn></math>";
    test_braille("Nemeth", expr, "⠼⠒⠸⠒⠼⠒⠴")?;
    return Ok(());
}

#[test]
fn colon_40_1_mtext() -> Result<()> {
    let expr = "<math><mtext>3:30</mtext></math>";
    test_braille("Nemeth", expr, "⠼⠒⠸⠒⠼⠒⠴")?;
    return Ok(());
}

#[test]
fn colon_40_2() -> Result<()> {
    let expr = "<math><mi>f</mi><mo>:</mo><mo>(</mo><mi>x</mi><mo>,</mo><mi>y</mi><mo>)</mo></math>";
    test_braille("Nemeth", expr, "⠰⠋⠸⠒⠷⠭⠠⠀⠽⠾")?;
    return Ok(());
}

#[test]
fn dash_42_4() -> Result<()> {
    let expr = "<math><mfrac><mo>&#x2015;</mo><mn>15</mn></mfrac><mo>=</mo><mfrac><mn>2</mn><mn>3</mn></mfrac></math>";
    test_braille("Nemeth", expr, "⠹⠤⠤⠤⠤⠀⠌⠂⠢⠼⠀⠨⠅⠀⠹⠆⠌⠒⠼")?;
    return Ok(());
}

#[test]
fn dash_42_6() -> Result<()> {
    let expr = "<math><mo>$</mo><mn>2</mn><mo>+</mo><mo>$</mo><mn>3</mn><mo>=</mo><mo>$</mo><mo>&#x2015;</mo></math>";
    test_braille("Nemeth", expr, "⠈⠎⠆⠬⠈⠎⠒⠀⠨⠅⠀⠈⠎⠤⠤⠤⠤")?;
    return Ok(());
}

#[test]
fn ellipsis_43_a_1() -> Result<()> {
    let expr = "<math><mn>1</mn><mo>,</mo><mn>3</mn><mo>,</mo><mn>5</mn><mo>,</mo><mo>…</mo><mo>,</mo><mn>15</mn><mo>.</mo></math>";
    test_braille("Nemeth", expr, "⠼⠂⠠⠀⠼⠒⠠⠀⠼⠢⠠⠀⠄⠄⠄⠠⠀⠼⠂⠢⠸⠲")?;
    return Ok(());
}

#[test]
fn ellipsis_43_b_3() -> Result<()> {
    let expr = "<math>
        <msubsup><mi>p</mi><mn>1</mn><msub><mi>&#x3B1;</mi><mn>1</mn></msub></msubsup>
        <mo>&#x2026;</mo>
        <msubsup><mi>p</mi><mi>r</mi><msub><mi>&#x3B1;</mi><mi>r</mi></msub></msubsup>
        </math>";
    test_braille("Nemeth", expr, "⠏⠂⠘⠨⠁⠘⠰⠂⠐⠄⠄⠄⠀⠏⠰⠗⠘⠨⠁⠘⠰⠗")?;
    return Ok(());
}

#[test]
fn ellipsis_43_b_4() -> Result<()> {
    let expr = "<math><mo>(</mo><mo>…</mo><mo>,</mo><mo>-</mo><mn>1</mn><mo>,</mo><mn>0</mn><mo>,</mo><mn>1</mn><mo>,</mo><mo>…</mo><mo>)</mo></math>";
    test_braille("Nemeth", expr, "⠷⠄⠄⠄⠠⠀⠤⠂⠠⠀⠴⠠⠀⠂⠠⠀⠄⠄⠄⠾")?;
    return Ok(());
}

#[test]
fn ellipsis_43_b_5() -> Result<()> {
    let expr = "<math><mn>12</mn><mi>¢</mi><mo>+</mo><mn>14</mn><mi>¢</mi><mo>=</mo><mo>…</mo><mi>¢</mi></math>";
    test_braille("Nemeth", expr, "⠼⠂⠆⠈⠉⠬⠂⠲⠈⠉⠀⠨⠅⠀⠄⠄⠄⠈⠉")?;
    return Ok(());
}

#[test]
fn omission_57_1() -> Result<()> {
    let expr = "<math><msup><mrow><mo>(</mo><mo>?</mo><mo>)</mo></mrow><mn>3</mn></msup><mo>=</mo><mn>27</mn></math>";
    test_braille("Nemeth", expr, "⠷⠿⠾⠘⠒⠀⠨⠅⠀⠼⠆⠶")?;
    return Ok(());
}

#[test]
fn omission_57_3() -> Result<()> {
    let expr = " <math><mn>7</mn><mo>&#xD7;</mo><mn>2</mn><mo>?</mo><mn>14</mn></math>";
    test_braille("Nemeth", expr, "⠼⠶⠈⠡⠆⠀⠿⠀⠼⠂⠲")?;
    return Ok(());
}

#[test]
fn omission_57_4() -> Result<()> {
    let expr = "<math><mo>?</mo><mo>+</mo><mo>?</mo><mo>=</mo><mn>10</mn></math>";
    test_braille("Nemeth", expr, "⠿⠬⠿⠀⠨⠅⠀⠼⠂⠴")?;
    return Ok(());
}

#[test]
fn omission_57_5() -> Result<()> {
    let expr = "<math><mn>7</mn><mo>-</mo><menclose notation='bottom'><mo>?</mo></menclose><mo>=</mo><mn>5</mn></math>";
    test_braille("Nemeth", expr, "⠼⠶⠤⠿⠀⠨⠅⠀⠼⠢")?;
    return Ok(());
}

#[test]
fn omission_57_6() -> Result<()> {
    let expr = "<math><mn>9</mn><mo>-</mo><mn>5</mn><mo>=</mo><mtext>-?-</mtext></math>";
    test_braille("Nemeth", expr, "⠼⠔⠤⠢⠀⠨⠅⠀⠿")?;
    return Ok(());
}

#[test]
fn omission_57_7() -> Result<()> {
    // test uses a couple of different forms of blank spaces
    let expr = "<math>
            <mo>(</mo><mn>5</mn><mo>,</mo><mspace width='1.5em'/><mo>)</mo><mo>+</mo>
            <mo>(</mo><mo>&#xA0;</mo><mo>&#xA0;</mo><mo>,</mo><mn>15</mn><mo>)</mo><mo>=</mo>
            <mo>(</mo><mn>7</mn><mo>,</mo><mn>13</mn><mo>)</mo>
        </math>";
    test_braille("Nemeth", expr, "⠷⠢⠠⠀⠿⠾⠬⠷⠿⠠⠀⠂⠢⠾⠀⠨⠅⠀⠷⠶⠠⠀⠂⠒⠾")?;
    return Ok(());
}

#[test]
fn omission_57_8() -> Result<()> {
    let expr = "<math><mn>5</mn><mo>×</mo><mn>25</mn><mo>=</mo><mspace width='1.5em'/></math>";
    test_braille("Nemeth", expr, "⠼⠢⠈⠡⠆⠢⠀⠨⠅⠀⠿")?;
    return Ok(());
}

#[test]
fn simple_frac_62_a_3() -> Result<()> {
    let expr = "<math><mfrac><mrow><mi>a</mi><mo>+</mo><mi>b</mi></mrow><mi>c</mi></mfrac></math>";
    test_braille("Nemeth", expr, "⠹⠁⠬⠃⠌⠉⠼")?;
    return Ok(());
}

#[test]
fn beveled_frac_62_b_1() -> Result<()> {
    let expr = "<math><mfrac bevelled='true'>
        <mrow><mi>a</mi><mo>+</mo><mi>b</mi></mrow>
        <mrow><mi>c</mi><mo>+</mo><mi>d</mi></mrow>
        </mfrac></math>";
    test_braille("Nemeth", expr, "⠹⠁⠬⠃⠸⠌⠉⠬⠙⠼")?;
    return Ok(());
}

#[test]
fn mixed_frac_63_a_1() -> Result<()> {
    let expr = "<math><mn>4</mn><mfrac><mn>3</mn><mn>8</mn></mfrac></math>";
    test_braille("Nemeth", expr, "⠼⠲⠸⠹⠒⠌⠦⠸⠼")?;
    return Ok(());
}

#[test]
fn mixed_frac_64_2() -> Result<()> {
    let expr = "<math><mn>4</mn><mn>3</mn><mo>/</mo><mn>8</mn></math>";
    test_braille("Nemeth", expr, "⠼⠲⠸⠹⠒⠸⠌⠦⠸⠼")?;
    return Ok(());
}

#[test]
fn complex_frac_66_1() -> Result<()> {
    let expr = "<math><mfrac><mfrac><mn>3</mn><mn>8</mn></mfrac><mn>5</mn></mfrac></math>";
    test_braille("Nemeth", expr, "⠠⠹⠹⠒⠌⠦⠼⠠⠌⠢⠠⠼")?;
    return Ok(());
}

#[test]
fn non_hyper_complex_frac_67_1() -> Result<()> {
    let expr = "<math><mfrac><mi>a</mi><msup><mi>b</mi>
            <mfrac>
                <mfrac><mn>3</mn><mn>4</mn></mfrac>
                <mfrac><mn>5</mn><mn>6</mn></mfrac>
            </mfrac>
        </msup></mfrac></math>";
    test_braille("Nemeth", expr, "⠹⠁⠌⠃⠘⠠⠹⠹⠒⠌⠲⠼⠠⠌⠹⠢⠌⠖⠼⠠⠼⠐⠼")?;
    return Ok(());
}

#[test]
fn hyper_complex_frac_68_a_1() -> Result<()> {
    // book uses 2d layout -- linear is used here
    let expr = "<math><mfrac>
            <mfrac>
            <mrow><mn>1</mn><mfrac><mn>1</mn><mn>4</mn></mfrac></mrow>
            <mrow><mn>1</mn><mfrac><mn>3</mn><mn>5</mn></mfrac></mrow>
            </mfrac>
            <mn>5</mn>
        </mfrac></math>";
    test_braille("Nemeth", expr, "⠠⠠⠹⠠⠹⠂⠸⠹⠂⠌⠲⠸⠼⠠⠌⠂⠸⠹⠒⠌⠢⠸⠼⠠⠼⠠⠠⠌⠢⠠⠠⠼")?;
    return Ok(());
}

#[test]
fn nested_sup_74_b_1() -> Result<()> {
    let expr = "<math><msup><mi>n</mi><msup><mi>x</mi><mi>y</mi></msup></msup></math>";
    test_braille("Nemeth", expr, "⠝⠘⠭⠘⠘⠽")?;
    return Ok(());
}

#[test]
fn nested_sup_mmultiscripts_74_b_1() -> Result<()> {
    let expr = "<math><mmultiscripts><mi>n</mi><none/><msup><mi>x</mi><mi>y</mi></msup></mmultiscripts></math>";
    test_braille("Nemeth", expr, "⠝⠘⠭⠘⠘⠽")?;
    return Ok(());
}

#[test]
fn nested_sup_74_b_4() -> Result<()> {
    let expr = "<math><msub><mi>n</mi><msub><mi>x</mi><mi>y</mi></msub></msub></math>";
    test_braille("Nemeth", expr, "⠝⠰⠭⠰⠰⠽")?;
    return Ok(());
}

#[test]
fn nested_sub_sup_74_c_5() -> Result<()> {
    let expr = "<math><msup><mi>n</mi><msub><mi>x</mi><msub><mi>a</mi><mi>j</mi></msub></msub></msup></math>";
    test_braille("Nemeth", expr, "⠝⠘⠭⠘⠰⠁⠘⠰⠰⠚")?;
    return Ok(());
}

#[test]
fn as_multiscript_nested_sub_sup_74_c_5() -> Result<()> {
    let expr = "<math><mmultiscripts><mi>n</mi><none/><msub><mi>x</mi><msub><mi>a</mi><mi>j</mi></msub></msub></mmultiscripts></math>";
    test_braille("Nemeth", expr, "⠝⠘⠭⠘⠰⠁⠘⠰⠰⠚")?;
    return Ok(());
}

#[test]
fn left_sup_75_1() -> Result<()> {
    let expr = "<math><mmultiscripts><mi>n</mi><mprescripts/><none/><mi>x</mi></mmultiscripts></math>";
    test_braille("Nemeth", expr, "⠘⠭⠐⠝")?;
    return Ok(());
}

#[test]
fn left_sup_75_4() -> Result<()> {
    let expr = "<math><mmultiscripts><mi>n</mi><mi>y</mi><none/><mprescripts/><mi>x</mi><none/></mmultiscripts></math>";
    test_braille("Nemeth", expr, "⠰⠭⠐⠝⠰⠽")?;
    return Ok(());
}

#[test]
fn left_sup_75_7() -> Result<()> {
    let expr = "<math><mmultiscripts><mi>x</mi><mprescripts/><none/>
            <mmultiscripts><mi>n</mi><mprescripts/><mi>a</mi><none/></mmultiscripts>
        </mmultiscripts></math>";
    test_braille("Nemeth", expr, "⠘⠰⠁⠘⠝⠐⠭")?;
    return Ok(());
}

#[test]
fn left_sup_75_8() -> Result<()> {
    let expr = "<math><mmultiscripts><mi>x</mi><mprescripts/><msup><mi>n</mi><mi>a</mi></msup><none/></mmultiscripts></math>";
    test_braille("Nemeth", expr, "⠰⠝⠰⠘⠁⠐⠭")?;
    return Ok(());
}

#[test]
fn left_sup_75_12() -> Result<()> {
    // updated to 2022 rev: Example 14-103: Left Superscript Following Right Superscript
    let expr = "<math><msup><mi>p</mi><mi>b</mi></msup><mmultiscripts><mi>q</mi><mprescripts/><none/><mi>c</mi></mmultiscripts></math>";
    test_braille("Nemeth", expr, "⠏⠘⠃⠐⠘⠉⠐⠟")?;
    return Ok(());
}

#[test]
fn left_sub_14_104() -> Result<()> {
    init_logger();
    // Right Subscript Followed by Left Subscript
    let expr = "<math><msub><mi>P</mi><mi>b</mi></msub><mmultiscripts><mi>Q</mi><mprescripts/><mi>c</mi><none/></mmultiscripts></math>";
    test_braille("Nemeth", expr, "⠠⠏⠰⠃⠐⠰⠉⠐⠠⠟")?;
    return Ok(());
}


#[test]
fn german_base_77_4_3() -> Result<()> {
    let expr = "<math><msub> <mi>𝔄</mi> <mn>1</mn> </msub></math>";
    test_braille("Nemeth", expr, "⠸⠠⠁⠂")?;
    return Ok(());
}

#[test]
fn prime_77_4_4() -> Result<()> {
    let expr = "<math><msub> <msup><mi>x</mi><mo>'</mo></msup> <mn>1</mn> </msub></math>";
    test_braille("Nemeth", expr, "⠭⠄⠂")?;
    return Ok(());
}

#[test]
fn prescript_77_4_6() -> Result<()> {
    let expr = "<math><mmultiscripts> <mi>x</mi> <mprescripts/> <mn>3</mn><none/></mmultiscripts></math>";
    test_braille("Nemeth", expr, "⠰⠒⠐⠭")?;
    return Ok(());
}

#[test]
fn prescript_77_4_7() -> Result<()> {
    let expr = "<math><msub><mi>x</mi><msub><mi>i</mi><mn>1</mn></msub></msub></math>";
    test_braille("Nemeth", expr, "⠭⠰⠊⠰⠰⠂")?;
    return Ok(());
}

#[test]
fn log_77_4_8() -> Result<()> {
    let expr = "<math><msub><mi>log</mi><mn>2</mn></msub> <mi>x</mi></math>";
    test_braille("Nemeth", expr, "⠇⠕⠛⠆⠀⠭")?;
    return Ok(());
}

#[test]
fn mmultiscripts_77_4_10() -> Result<()> {
    // not  right to use msub because the subscripts should be aligned -- nested msub's won't align subscripts -- mmultiscripts solves this
    let expr = "<math>
    <mmultiscripts>
        <mrow>
            <mo>(</mo>
            <mi mathvariant='normal'>C</mi>
            <mmultiscripts>  <mi mathvariant='normal'>O</mi> <mn>3</mn> <none/> </mmultiscripts>
            <mo>)</mo>
        </mrow>
        <mn>2</mn>
        <none/>
    </mmultiscripts>
</math>
";
    test_braille("Nemeth", expr, "⠷⠠⠉⠠⠕⠒⠾⠰⠆")?;
    return Ok(());
}

#[test]
fn word_77_4_12() -> Result<()> {
    let expr = "<math><msub><mi>seven</mi><mn>3</mn></msub></math>";
    test_braille("Nemeth", expr, "⠎⠑⠧⠑⠝⠰⠒")?;
    return Ok(());
}

#[test]
fn prescript_77_4_18() -> Result<()> {
    // from MathJaX
    let expr = "<math><msub><mrow/><mn>3</mn></msub><msub><mi>x</mi><mn>1</mn></msub></math>";
    test_braille("Nemeth", expr, "⠰⠒⠐⠭⠂")?;
    return Ok(());
}

#[test]
fn mmultiscripts_77_4_18() -> Result<()> {
    // from WIRIS (changed empty mrow to 'none')
    let expr = "<math><mmultiscripts><mi>x</mi><mn>1</mn><none/><mprescripts/><mn>3</mn><none/></mmultiscripts></math>";
    test_braille("Nemeth", expr, "⠰⠒⠐⠭⠂")?;
    return Ok(());
}

#[test]
fn comma_number_77_4_20() -> Result<()> {
    // mathml from mathjax output for "x_{10,000}"
    let expr = "<math><msub><mi>x</mi><mrow><mn>10</mn><mo>,</mo><mn>000</mn></mrow></msub></math>";
    test_braille("Nemeth", expr, "⠭⠂⠴⠠⠴⠴⠴")?;
    return Ok(());
}

#[test]
fn sum_77_4_23() -> Result<()> {
    let expr = "<math><msubsup><mo>&#x2211;</mo><mn>0</mn><mi>n</mi></msubsup><msub><mi>a</mi><mi>k</mi></msub></math>";
    test_braille("Nemeth", expr, "⠨⠠⠎⠴⠘⠝⠐⠁⠰⠅")?;
    return Ok(());
}

#[test]
fn product_77_4_24() -> Result<()> {
    let expr = "<math><msubsup><mo>&#x220F;</mo><mn>0</mn><mi>n</mi></msubsup><msub><mi>a</mi><mi>k</mi></msub></math>";
    test_braille("Nemeth", expr, "⠨⠠⠏⠴⠘⠝⠐⠁⠰⠅")?;
    return Ok(());
}

#[test]
fn integral_77_4_26() -> Result<()> {
    let expr = "<math>
            <msubsup>
                <mo>&#x222B;</mo>
                <mn>0</mn>
                <msqrt><mn>1</mn><mo>-</mo><msup><mi>x</mi><mn>2</mn></msup></msqrt> 
            </msubsup>
            <mrow><mi>f</mi><mo>(</mo><mi>x</mi><mo>)</mo><mi>d</mi><mi>x</mi></mrow>
        </math>";
    test_braille("Nemeth", expr, "⠮⠰⠴⠘⠜⠂⠤⠭⠘⠘⠆⠘⠻⠐⠋⠷⠭⠾⠙⠭")?;
    return Ok(());
}

#[test]
fn comma_space_78_1() -> Result<()> {
    // WIRIS output when typed with spaces (which I doubt people do)
    let expr = "<math><msub><mi>x</mi>
         <mrow><mi>i</mi><mo>,</mo><mo>&#xA0;</mo><mi>j</mi><mo>,</mo><mo>&#xA0;</mo><mi>k</mi></mrow></msub></math>";
    test_braille("Nemeth", expr, "⠭⠰⠊⠪⠚⠪⠅")?;
    return Ok(());
}

#[test]
fn comma_78_2() -> Result<()> {
    let expr = "<math><msub><mi>x</mi><mrow><mo>(</mo><mi>a</mi><mo>,</mo><mi>b</mi><mo>)</mo></mrow></msub></math>";
    test_braille("Nemeth", expr, "⠭⠰⠷⠁⠪⠃⠾")?;
    return Ok(());
}

#[test]
fn comma_78_2_invisible() -> Result<()> { // test with invisible comma -- should be the same (issue #40)
    let expr = "<math><msub><mi>x</mi><mrow><mo>(</mo><mi>a</mi><mo>&#x2063;</mo><mi>b</mi><mo>)</mo></mrow></msub></math>";
    test_braille("Nemeth", expr, "⠭⠰⠷⠁⠪⠃⠾")?;
    return Ok(());
}

#[test]
fn comma_78_3() -> Result<()> {
    let expr = "<math><msub><mi>x</mi><mrow><mn>1</mn><mo>,</mo><mn>2</mn></mrow></msub></math>";
    test_braille("Nemeth", expr, "⠭⠰⠂⠪⠆")?;
    return Ok(());
}

#[test]
fn comma_78_6() -> Result<()> {
    // WIRIS output when typed with spaces
    let expr = "<math><mo>(</mo><mi>x</mi><mo>,</mo><mo>&#xA0;</mo><mi>y</mi><mo>)</mo></math>";
    test_braille("Nemeth", expr, "⠷⠭⠠⠀⠽⠾")?;
    return Ok(());
}

#[test]
fn nested_super_79_a_2() -> Result<()> {
    let expr = "<math><msub><mi>x</mi><mi>a</mi></msub><mo>+</mo><msup><mi>y</mi><mn>2</mn></msup></math>";
    test_braille("Nemeth", expr, "⠭⠰⠁⠐⠬⠽⠘⠆")?;
    return Ok(());
}

#[test]
fn nested_super_79_a_3() -> Result<()> {
    let expr = "<math><mfrac><mrow><msup><mi>e</mi><mrow><msup><mi>x</mi><mn>2</mn></msup></mrow></msup></mrow><mn>2</mn></mfrac></math>";
    test_braille("Nemeth", expr, "⠹⠑⠘⠭⠘⠘⠆⠐⠌⠆⠼")?;
    return Ok(());
}

#[test]
fn punctuation_after_sup_79_b_2() -> Result<()> {
    let expr = "<math><msup><mi>x</mi><mn>2</mn></msup><mo>,</mo><msup><mi>x</mi><mn>3</mn></msup></math>";
    test_braille("Nemeth", expr, "⠭⠘⠆⠠⠀⠭⠘⠒")?;
    return Ok(());
}

#[test]
fn comma_in_number_in_sup_79_b_3() -> Result<()> {
    // bad mn from Wiris
    let expr = "<math><msup><mi>x</mi><mrow><mn>10</mn><mo>,</mo><mn>000</mn></mrow></msup></math>";
    test_braille("Nemeth", expr, "⠭⠘⠂⠴⠠⠴⠴⠴")?;
    return Ok(());
}

#[test]
fn comma_in_sup_79_b_4() -> Result<()> {
    let expr = "<math><msub><mi>x</mi><mrow><mi>i</mi><mo>,</mo><mi>j</mi></mrow></msub></math>";
    test_braille("Nemeth", expr, "⠭⠰⠊⠪⠚")?;
    return Ok(());
}


#[test]
fn comma_ellipsis_in_sub_79_b_5() -> Result<()> {
    let expr = "<math> <msub><mi>P</mi>
        <mrow><msub><mi>n</mi><mn>1</mn></msub>
          <mo>,</mo>
          <msub><mi>n</mi><mn>2</mn></msub>
          <mo>,</mo><mo>&#x2026;</mo>
          </mrow></msub></math>";
    test_braille("Nemeth", expr, "⠠⠏⠰⠝⠰⠰⠂⠰⠪⠝⠰⠰⠆⠰⠪⠀⠄⠄⠄")?;
    return Ok(());
}
#[test]
fn text_after_sup_79_c_3() -> Result<()> {
    // bad mn from Wiris; also &A0;
    let expr = "<math><mn>6</mn><mo>.</mo><mn>696</mn><mo>×</mo><msup><mn>10</mn><mn>8</mn></msup><mo>&#xA0;</mo><mtext>mph</mtext></math>";
    test_braille("Nemeth", expr, "⠼⠖⠨⠖⠔⠖⠈⠡⠂⠴⠘⠦⠀⠍⠏⠓")?;
    return Ok(());
}

#[test]
fn table_entry_after_sup_79_c_4() -> Result<()> {
    let expr = "<math><mrow><mo>(</mo>
        <mtable><mtr>
          <mtd><msup><mi>x</mi><mn>2</mn></msup></mtd>
          <mtd><msup><mi>y</mi><mn>2</mn></msup></mtd>
        </mtr></mtable>
        <mo>)</mo></mrow></math>";
    test_braille("Nemeth", expr, "⠷⠭⠘⠆⠀⠽⠘⠆⠐⠾")?;
    return Ok(());
}

#[test]
fn nested_super_space_79_d_3() -> Result<()> {
    let expr = "<math><msup><mi>cos</mi><mn>2</mn></msup><mi>x</mi></math>";
    test_braille("Nemeth", expr, "⠉⠕⠎⠘⠆⠀⠭")?;
    return Ok(());
}

#[test]
fn nested_super_space_79_d_7() -> Result<()> {
    let expr = "<math><mrow><msup><mi>e</mi><mrow><msup><mi>cos</mi><mn>2</mn></msup><mi>x</mi></mrow></msup></mrow></math>";
    test_braille("Nemeth", expr, "⠑⠘⠉⠕⠎⠘⠘⠆⠀⠭")?;
    return Ok(());
}

#[test]
fn nested_sup_sup_space_79_d_9() -> Result<()> {
    let expr = "<math><msup><mi>q</mi><mrow><msub><mi>log</mi><mi>q</mi></msub><mi>a</mi></mrow></msup></math>";
    test_braille("Nemeth", expr, "⠟⠘⠇⠕⠛⠘⠰⠟⠀⠁")?;
    return Ok(());
}

#[test]
fn whitespace_in_sup_79_e_1() -> Result<()> {
    let expr = "<math><msup><mi>e</mi><mn>3.14159 26535</mn></msup></math>";
    test_braille("Nemeth", expr, "⠑⠘⠒⠨⠂⠲⠂⠢⠔⠀⠆⠖⠢⠒⠢")?;
    return Ok(());
}

#[test]
fn ellipsis_level_79_f_1() -> Result<()> {
    let expr = "<math><msup><mi>x</mi>
        <mrow><mn>1</mn><mo>+</mo><mn>1</mn><mo>/</mo><mn>2</mn><mo>+</mo><mn>1</mn><mo>/</mo><mn>3</mn><mo>+</mo>
        <mo>…</mo><mo>+</mo><mn>1</mn><mo>/</mo><mi>n</mi></mrow></msup></math>";
    test_braille("Nemeth", expr, "⠭⠘⠂⠬⠂⠸⠌⠆⠬⠂⠸⠌⠒⠬⠀⠄⠄⠄⠀⠬⠂⠸⠌⠝")?;
    return Ok(());
}

#[test]
fn comparison_79_g_2() -> Result<()> {
    let expr = "<math><msup><mn>2</mn><mi>x</mi></msup><mo>&lt;</mo><msup><mn>3</mn><mi>x</mi></msup></math>";
    test_braille("Nemeth", expr, "⠼⠆⠘⠭⠀⠐⠅⠀⠼⠒⠘⠭")?;
    return Ok(());
}

#[test]
fn sub_ind_79_g_4() -> Result<()> {
    let expr = "<math><msub><mo>∫</mo><mrow><mi>u</mi><mo>=</mo><mi>a</mi></mrow></msub></math>";
    test_braille("Nemeth", expr, "⠮⠰⠥⠀⠰⠨⠅⠀⠁")?;
    return Ok(());
}

#[test]
fn baseline_80_a_1() -> Result<()> {
    let expr = "<math><msqrt><msup><mi>x</mi><mn>2</mn></msup><mo>+</mo><msup><mi>y</mi><mn>2</mn></msup></msqrt></math>";
    test_braille("Nemeth", expr, "⠜⠭⠘⠆⠐⠬⠽⠘⠆⠐⠻")?;
    return Ok(());
}

#[test]
fn superscript_80_a_2() -> Result<()> {
    let expr = "<math><msup><mi>e</mi><msqrt><msup><mi>x</mi><mn>2</mn></msup><mo>+</mo><msup><mi>y</mi><mn>2</mn></msup></msqrt></msup></math>";
    test_braille("Nemeth", expr, "⠑⠘⠜⠭⠘⠘⠆⠘⠬⠽⠘⠘⠆⠘⠻")?;
    return Ok(());
}

#[test]
fn sub_ind_80_b_3() -> Result<()> {
    // also Example 14-105: Right Numeric Subscript and Left Subscript
    let expr = "<math><msub><mi>P</mi><mn>1</mn></msub><mmultiscripts><mi>Q</mi><mprescripts/><mn>2</mn><none/></mmultiscripts></math>";
    test_braille("Nemeth", expr, "⠠⠏⠂⠐⠰⠆⠐⠠⠟")?;
    return Ok(());
}

#[test]
fn left_sub_14_105() -> Result<()> {
    // Right Numeric Subscript and Left Subscript
    let expr = "<math><msub><mi>P</mi><mn>1</mn></msub><mmultiscripts><mi>Q</mi><mprescripts/><mn>2</mn><none/></mmultiscripts></math>";
    test_braille("Nemeth", expr, "⠠⠏⠂⠐⠰⠆⠐⠠⠟")?;
    return Ok(());
}

#[test]
fn sub_ind_mmultiscripts_80_b_3() -> Result<()> {
    let expr = "<math><mmultiscripts><mi>P</mi><mn>1</mn><none/></mmultiscripts>
                           <mmultiscripts><mi>Q</mi><mprescripts/><mn>2</mn><none/></mmultiscripts></math>";
    test_braille("Nemeth", expr, "⠠⠏⠂⠰⠆⠐⠠⠟")?;
    return Ok(());
}

#[test]
fn sub_ind_80_b_4() -> Result<()> {
    // also Example 14-106: Subscript with Subscript
    let expr = "<math><msub><mi>A</mi><mrow><mover><mi>x</mi><mo>~</mo></mover><mo>+</mo><mover><mi>y</mi><mo>~</mo></mover></mrow></msub></math>";
    test_braille("Nemeth", expr, "⠠⠁⠰⠐⠭⠣⠈⠱⠻⠬⠰⠐⠽⠣⠈⠱⠻")?;
    return Ok(());
}

#[test]
fn numeric_sub_81_a_1() -> Result<()> {
    let expr = "<math><mo>(</mo><msub><mi>x</mi><mn>1</mn></msub><mo>+</mo><mn>1</mn><mo>)</mo></math>";
    test_braille("Nemeth", expr, "⠷⠭⠂⠬⠂⠾")?;
    return Ok(());
}

#[test]
fn msubsup_82_a_1() -> Result<()> {
    let expr = "<math><msubsup><mi>x</mi><mi>a</mi><mi>n</mi></msubsup></math>";
    test_braille("Nemeth", expr, "⠭⠰⠁⠘⠝")?;
    return Ok(());
}

#[test]
fn msubsup_82_a_3() -> Result<()> {
    let expr = "<math><msubsup><mi>x</mi><mn>1</mn><mn>2</mn></msubsup></math>";
    test_braille("Nemeth", expr, "⠭⠂⠘⠆")?;
    return Ok(());
}

#[test]
fn mmultiscripts_82_a_1() -> Result<()> {
    let expr = "<math><mmultiscripts><mi>x</mi><mi>a</mi><mi>n</mi></mmultiscripts></math>";
    test_braille("Nemeth", expr, "⠭⠰⠁⠘⠝")?;
    return Ok(());
}

#[test]
fn mmultiscripts_82_a_2() -> Result<()> {
    let expr = "<math><mmultiscripts><mi>x</mi><mprescripts/><mi>a</mi><mi>n</mi></mmultiscripts></math>";
    test_braille("Nemeth", expr, "⠰⠁⠘⠝⠐⠭")?;
    return Ok(());
}

#[test]
fn mmultiscripts_82_a_3() -> Result<()> {
    let expr = "<math><mmultiscripts><mi>x</mi><mn>1</mn><mn>2</mn></mmultiscripts></math>";
    test_braille("Nemeth", expr, "⠭⠂⠘⠆")?;
    return Ok(());
}

#[test]
fn sub_sup_82_b_1() -> Result<()> {
    let expr = "<math><msub><msup><mi>a</mi><mi>n</mi></msup><mi>m</mi></msub></math>";
    test_braille("Nemeth", expr, "⠁⠘⠝⠐⠰⠍")?;
    return Ok(());
}

#[test]
fn mmultiscripts_82_b_1() -> Result<()> {
    let expr = "<math><mmultiscripts><mi>a</mi><none/><mi>n</mi><mi>m</mi><none/></mmultiscripts></math>";
    test_braille("Nemeth", expr, "⠁⠘⠝⠐⠰⠍")?;
    return Ok(());
}

#[test]
fn sub_sup_82_b_2() -> Result<()> {
    let expr = "<math><msup><msub><mi>a</mi><mi>m</mi></msub><mi>n</mi></msup></math>";
    test_braille("Nemeth", expr, "⠁⠰⠍⠐⠘⠝")?;
    return Ok(());
}

#[test]
fn mmultiscripts_82_b_2() -> Result<()> {
    let expr = "<math><mmultiscripts><mi>a</mi><mi>m</mi><none/><none/><mi>n</mi></mmultiscripts></math>";
    test_braille("Nemeth", expr, "⠁⠰⠍⠐⠘⠝")?;
    return Ok(());
}

#[test]
fn mmultiscripts_82_b_3() -> Result<()> {
    let expr = "<math><mmultiscripts><mi>x</mi><mprescripts/><none/><mi>a</mi><mi>b</mi><none/></mmultiscripts></math>";
    test_braille("Nemeth", expr, "⠘⠁⠐⠰⠃⠐⠭")?;
    return Ok(());
}

#[test]
fn mmultiscripts_82_b_4() -> Result<()> {
    let expr = "<math><mmultiscripts><mi>x</mi><mprescripts/><mi>b</mi><none/><none/><mi>a</mi></mmultiscripts></math>";
    test_braille("Nemeth", expr, "⠰⠃⠐⠘⠁⠐⠭")?;
    return Ok(());
}

#[test]
fn mmultiscripts_82_b_5() -> Result<()> {
    let expr = "<math><mmultiscripts><mi>x</mi><mn>1</mn><none/><none/><mn>2</mn></mmultiscripts></math>";
    test_braille("Nemeth", expr, "⠭⠂⠐⠘⠆")?;
    return Ok(());
}

#[test]
fn mmultiscripts_82_b_6() -> Result<()> {
    let expr = "<math><mmultiscripts><mi>x</mi><mi>a</mi><mo>'</mo><none/><mi>b</mi></mmultiscripts></math>";
    test_braille("Nemeth", expr, "⠭⠄⠰⠁⠐⠘⠃")?;
    return Ok(());
}

#[test]
fn prime_83_b_1() -> Result<()> {
    let expr = "<math><msubsup><mi>x</mi><mi>a</mi><mo>'</mo></msubsup></math>";
    test_braille("Nemeth", expr, "⠭⠄⠰⠁")?;
    return Ok(());
}

#[test]
fn prime_mmultiscripts_83_b_2() -> Result<()> {
    let expr = "<math><mmultiscripts> <mi>x</mi> <none/><mo>'</mo> <none/><mn>2</mn></mmultiscripts></math>";
    test_braille("Nemeth", expr, "⠭⠄⠘⠆")?;
    return Ok(());
}

#[test]
fn prime_mathjax_83_b_2() -> Result<()> {
    // from MathJax with input x^{\prime 2}
    let expr = "<math><msup> <mrow><mi>x</mi><mo>'</mo></mrow> <mn>2</mn></msup></math>";
    test_braille("Nemeth", expr, "⠭⠄⠘⠆")?;
    return Ok(());
}

#[test]
fn prime_wiris_83_b_2() -> Result<()> {
    let expr = "<math><msup><mi>x</mi> <mrow><mi>&#x2032;</mi><mn>2</mn></mrow> </msup></math>";
    test_braille("Nemeth", expr, "⠭⠄⠘⠆")?;
    return Ok(());
}

#[test]
fn prime_83_b_3() -> Result<()> {
    let expr = "<math><mmultiscripts> <mi>x</mi> <none/><mo>'</mo> <mi>a</mi><mi>b</mi></mmultiscripts></math>";
    test_braille("Nemeth", expr, "⠭⠄⠰⠁⠘⠃")?;
    return Ok(());
}

#[test]
fn prime_83_b_4() -> Result<()> {
    let expr = "<math><msubsup> <msup><mi>x</mi><mo>''</mo></msup> <mn>1</mn> <mn>3</mn></msubsup></math>";
    test_braille("Nemeth", expr, "⠭⠄⠄⠂⠘⠒")?;
    return Ok(());
}

#[test]
fn prime_mmultiscripts_83_b_4() -> Result<()> {
    let expr = "<math><mmultiscripts> <mi>x</mi> <none/><mo>''</mo> <mn>1</mn><mn>3</mn></mmultiscripts></math>";
    test_braille("Nemeth", expr, "⠭⠄⠄⠂⠘⠒")?;
    return Ok(());
}

#[test]
fn prime_83_b_5() -> Result<()> {
    let expr = "<math><mmultiscripts> <mi>x</mi> <none/><mo>'</mo> <none/><mo>*</mo></mmultiscripts></math>";
    test_braille("Nemeth", expr, "⠭⠄⠘⠈⠼")?;
    return Ok(());
}

#[test]
fn prime_83_b_6() -> Result<()> {
    let expr = "<math><msup> <mi>x</mi> <mrow><mo>*</mo> <mo>'</mo></mrow> </msup></math>";
    test_braille("Nemeth", expr, "⠭⠘⠈⠼⠄")?;
    return Ok(());
}

#[test]
fn prime_83_b_7() -> Result<()> {
    let expr = "<math>
        <msubsup>
            <mi>A</mi>
            <mrow><mi>u</mi><mi>e</mi></mrow>
            <mrow><mo>&#x2217;</mo><mo>&#x2032;</mo></mrow>
        </msubsup>
    </math>";
    test_braille("Nemeth", expr, "⠠⠁⠰⠥⠑⠘⠈⠼⠄")?;
    return Ok(());
}

#[test]
fn prime_83_b_8() -> Result<()> {
    let expr = "<math>
        <msubsup>
            <mi>A</mi>
            <mrow><mi>u</mi><mi>e</mi></mrow>
            <mrow><mo>&#x2032;</mo><mo>&#x2217;</mo></mrow>
        </msubsup>
    </math>";
    test_braille("Nemeth", expr, "⠠⠁⠄⠰⠥⠑⠘⠈⠼")?;
    return Ok(());
}

#[test]
fn underbar_86_a_1() -> Result<()> {
    // Note: NFB lessons added a contracted form (lesson 12.5.1.b)
    let expr = "<math><munder><mi>x</mi><mo>&#xAF;</mo></munder></math>";
    test_braille("Nemeth", expr, "⠭⠩⠱")?;
    return Ok(());
}

#[test]
fn menclose_86_a_1() -> Result<()> {
    // Note: NFB lessons added a contracted form (lesson 12.5.1.b)
    let expr = "<math><menclose notation='bottom'><mi>x</mi></menclose></math>";
    test_braille("Nemeth", expr, "⠭⠩⠱")?;
    return Ok(());
}

#[test]
fn lim_86_a_3() -> Result<()> {
    let expr = "<math><munder><mi>lim</mi><mrow><mi>x</mi><mo>&#x2192;</mo><mn>0</mn></mrow></munder><mi>f</mi><mo>(</mo><mi>x</mi><mo>)</mo></math>";
    test_braille("Nemeth", expr, "⠐⠇⠊⠍⠩⠭⠀⠫⠕⠀⠼⠴⠻⠀⠋⠷⠭⠾")?;
    return Ok(());
}

#[test]
fn overbar_86_a_4() -> Result<()> {
    let expr = "<math><mover><msup><mi>x</mi><mn>2</mn></msup><mo>&#xAF;</mo></mover></math>";
    test_braille("Nemeth", expr, "⠐⠭⠘⠆⠐⠣⠱⠻")?;
    return Ok(());
}

#[test]
fn menclose_86_a_4() -> Result<()> {
    let expr = "<math><menclose notation='top'><msup><mi>x</mi><mn>2</mn></msup></menclose></math>";
    test_braille("Nemeth", expr, "⠐⠭⠘⠆⠐⠣⠱⠻")?;
    return Ok(());
}

#[test]
fn overbar_86_a_5() -> Result<()> {
    let expr = "<math><mover><msup><mi>x</mi><mn>2</mn></msup><mo>&#xAF;</mo></mover></math>";
    test_braille("Nemeth", expr, "⠐⠭⠘⠆⠐⠣⠱⠻")?;
    return Ok(());
}

#[test]
fn mathml_spec_example_86_a() -> Result<()> {
    let expr = "<math>
        <munder><mo>(</mo><mo>&#x5F;<!--LOW LINE--></mo></munder>
        <mfrac><mi>a</mi><mi>b</mi></mfrac>
        <mover><mo>)</mo><mo>&#x203E;<!--OVERLINE--></mo></mover>
    </math>";
    test_braille("Nemeth", expr, "⠐⠷⠩⠱⠻⠹⠁⠌⠃⠼⠐⠾⠣⠱⠻")?;
    return Ok(());
}

#[test]
fn mathml_spec_example_alt_char_86_a() -> Result<()> {
    let expr = "<math>
        <munder><mo>(</mo><mo>&#x2015;</mo></munder>
        <mfrac><mi>a</mi><mi>b</mi></mfrac>
        <mover><mo>)</mo><mo>&#x2015;</mo></mover>
    </math>";
    test_braille("Nemeth", expr, "⠐⠷⠩⠱⠻⠹⠁⠌⠃⠼⠐⠾⠣⠱⠻")?;
    return Ok(());
}

#[test]
fn menclose_lesson_12_5_5_5() -> Result<()> {
    // this is what WIRIS exports
    let expr = "<math><mi>A</mi><mo>(</mo><menclose notation='bottom'><mi>s</mi></menclose><mi>n</mi><mo>)</mo></math>";
    test_braille("Nemeth", expr, "⠠⠁⠷⠎⠩⠱⠝⠾")?;
    return Ok(());
}

#[test]
fn munder_lesson_12_5_5_5() -> Result<()> {
    let expr = "<math><mi>A</mi><mo>(</mo><munder><mi>s</mi><mo>&#xAF;</mo></munder><mi>n</mi><mo>)</mo></math>";
    test_braille("Nemeth", expr, "⠠⠁⠷⠎⠩⠱⠝⠾")?;
    return Ok(());
}

#[test]
fn overbar_86_b_1() -> Result<()> {
    let expr = "<math><mover><mi>x</mi><mo>&#xAF;</mo></mover></math>";
    test_braille("Nemeth", expr, "⠭⠱")?;
    return Ok(());
}

#[test]
fn menclose_86_b_1() -> Result<()> {
    let expr = "<math><menclose notation='top'><mi>x</mi></menclose></math>";
    test_braille("Nemeth", expr, "⠭⠱")?;
    return Ok(());
}

#[test]
fn overbar_86_b_2() -> Result<()> {
    let expr = "<math><mover>
            <mrow><mi>x</mi><mo>+</mo><mi>y</mi></mrow>
            <mo>&#xAF;</mo>
        </mover></math>";
    test_braille("Nemeth", expr, "⠐⠭⠬⠽⠣⠱⠻")?;
    return Ok(());
}

#[test]
fn primed_86_b_6() -> Result<()> {
    let expr = "<math><msup><mrow><mover><mi>x</mi><mo>&#xAF;</mo></mover></mrow><mo>&#x2032;</mo></msup></math>";
    test_braille("Nemeth", expr, "⠭⠱⠄")?;
    return Ok(());
}

#[test]
fn menclose_primed_86_b_6() -> Result<()> {
    let expr = "<math><msup><menclose notation='top'><mi>x</mi></menclose><mo>&#x2032;</mo></msup></math>";
    test_braille("Nemeth", expr, "⠭⠱⠄")?;
    return Ok(());
}

#[test]
fn overbar_86_b_10() -> Result<()> {
    let expr = "<math><mn>3</mn><mo>.</mo><mn>5</mn><mover><mn>4</mn><mo>&#xAF;</mo></mover></math>";
    test_braille("Nemeth", expr, "⠼⠒⠨⠢⠲⠱")?;
    return Ok(());
}

#[test]
fn overbar_86_b_11() -> Result<()> {
    let expr = "<math><mover><mfenced>
            <mrow><mover><mi>a</mi><mo>&#xAF;</mo></mover><mi mathvariant='bold'>A</mi><mo>+</mo>
                <mover><mi>b</mi><mo>&#xAF;</mo></mover><mi mathvariant='bold'>B</mi></mrow>
        </mfenced><mo>&#xAF;</mo></mover></math>";
    test_braille("Nemeth", expr, "⠐⠷⠁⠱⠸⠰⠠⠁⠬⠃⠱⠸⠰⠠⠃⠾⠣⠱⠻")?;
    return Ok(());
}

#[test]
fn menclose_86_b_11() -> Result<()> {
    let expr = "<math><menclose notation='top'><mfenced>
            <mrow><menclose notation='top'><mi>a</mi></menclose><mi mathvariant='bold'>A</mi><mo>+</mo>
            <menclose notation='top'><mi>b</mi></menclose><mi mathvariant='bold'>B</mi></mrow>
        </mfenced></menclose></math>";
    test_braille("Nemeth", expr, "⠐⠷⠁⠱⠸⠰⠠⠁⠬⠃⠱⠸⠰⠠⠃⠾⠣⠱⠻")?;
    return Ok(());
}

#[test]
fn order2_overbar_87_a_1() -> Result<()> {
    let expr = "<math><mover>
            <mrow><mi>x</mi><mo>+</mo><mi>y</mi></mrow>
            <mover><mo>&#xAF;</mo><mrow><mi>a</mi><mo>=</mo><mn>3</mn></mrow></mover>
        </mover></math>";
    // this is a possible other interpretation of 87a(1), but I think the above is the right one
    // let expr = "<math><mover>
    //         <mover><mrow><mi>x</mi><mo>+</mo><mi>y</mi></mrow><mo>&#xAF;</mo></mover>
    //         <mrow><mi>a</mi><mo>=</mo><mn>3</mn></mrow>
    //      </mover></math>";
    test_braille("Nemeth", expr, "⠐⠭⠬⠽⠣⠱⠣⠣⠁⠀⠨⠅⠀⠼⠒⠻")?;
    return Ok(());
}

#[test]
fn bar_above_and_below_88_1() -> Result<()> {
    let expr = "<math><munderover>
            <mrow><mi>x</mi><mo>+</mo><mi>y</mi></mrow>
            <mo>&#xAF;</mo>
            <mo>&#xAF;</mo>
        </munderover></math>";
    test_braille("Nemeth", expr, "⠐⠭⠬⠽⠩⠱⠣⠱⠻")?;
    return Ok(());
}

#[test]
fn above_and_below_88_2() -> Result<()> {
    let expr = "<math><munderover><mo>&#x2211;</mo><mrow><mi>n</mi><mo>=</mo><mn>1</mn></mrow><mo>&#x221E;</mo></munderover>
                            <mfrac><mn>1</mn><msup><mn>2</mn><mi>n</mi></msup></mfrac><mo>=</mo><mn>1</mn></math>";
    test_braille("Nemeth", expr, "⠐⠨⠠⠎⠩⠝⠀⠨⠅⠀⠼⠂⠣⠠⠿⠻⠹⠂⠌⠆⠘⠝⠐⠼⠀⠨⠅⠀⠼⠂")?;
    return Ok(());
}

#[test]
fn menclose_top_bottom_88_1() -> Result<()> {
    let expr = "<math><menclose notation='top bottom'><mi>x</mi><mo>+</mo><mi>y</mi></menclose></math>";
    test_braille("Nemeth", expr, "⠐⠭⠬⠽⠩⠱⠣⠱⠻")?;
    return Ok(());
}

#[test]
fn binomial_90_1() -> Result<()> {
    let expr = "<math><mo>(</mo><mfrac linethickness='0'><mi>n</mi><mi>k</mi></mfrac><mo>)</mo></math>";
    test_braille("Nemeth", expr, "⠷⠝⠩⠅⠾")?;
    return Ok(());
}

#[test]
fn binomial_90_1_mtable() -> Result<()> {
    // intent name not finalized -- may change
    let expr = "<math><mrow intent='binomial($n,$k)'>
            <mo>(</mo>
                <mtable>
                <mtr><mtd><mi arg='n'>n</mi></mtd></mtr>
                <mtr><mtd><mi arg='k'>k</mi></mtd></mtr>
                </mtable>
            <mo>)</mo>
        </mrow></math>";
    test_braille("Nemeth", expr, "⠷⠝⠩⠅⠾")?;
    return Ok(());
}

#[test]
fn modifier_in_script_91_1() -> Result<()> {
    let expr = "<math><msub><mi>A</mi><mover><mi>x</mi><mo>~</mo></mover></msub></math>";
    test_braille("Nemeth", expr, "⠠⠁⠰⠐⠭⠣⠈⠱⠻")?;
    return Ok(());
}

#[test]
fn arrow_96_1() -> Result<()> {
    let expr = "<math>
        <mover>
        <mrow><mi mathvariant='normal'>A</mi> <mi mathvariant='normal'>B</mi></mrow>
        <mo>→</mo>
        </mover>
    </math>";
    test_braille("Nemeth", expr, "⠐⠠⠁⠠⠃⠣⠫⠕⠻")?;
    return Ok(());
}

#[test]
fn arrow_96_10() -> Result<()> {
    let expr = "<math>
        <mi>X</mi>
        <mover>
        <mo>→</mo>
        <mrow><mi>f</mi> <mo>∘</mo><mi>g</mi></mrow>
        </mover>
        <mi>Y</mi>
    </math>";
    test_braille("Nemeth", expr, "⠠⠭⠀⠐⠫⠒⠒⠕⠣⠋⠨⠡⠛⠻⠀⠠⠽")?;
    return Ok(());
}

#[test]
fn bar_97_b_1() -> Result<()> {
    let expr = "<math><mo>.</mo><mover><mn>3</mn><mo>&#xAF;</mo></mover></math>";
    test_braille("Nemeth", expr, "⠼⠨⠒⠱")?;
    return Ok(());
}

#[test]
fn menclose_bar_97_b_1() -> Result<()> {
    let expr = "<math><mo>.</mo><menclose notation='top'><mn>3</mn></menclose></math>";
    test_braille("Nemeth", expr, "⠼⠨⠒⠱")?;
    return Ok(());
}

#[test]
fn menclose_bar_97_b_3() -> Result<()> {
    let expr = "<math><mn>3.57</mn><mover><mn>29</mn><mo stretchy='true'>&#xAF;</mo></mover></math>";
    test_braille("Nemeth", expr, "⠼⠒⠨⠢⠶⠐⠆⠔⠣⠱⠻")?;
    return Ok(());
}

#[test]
fn carrot_98_1() -> Result<()> {
    let expr = "<math><mover><mi>x</mi><mo>^</mo></mover></math>";
    test_braille("Nemeth", expr, "⠐⠭⠣⠸⠣⠻")?;
    return Ok(());
}

#[test]
fn dots_99_a_1() -> Result<()> {
    let expr = "<math><mo>.</mo><mover><mn>3</mn><mo>&#x2D9;</mo></mover></math>";
    test_braille("Nemeth", expr, "⠼⠨⠐⠒⠣⠡⠻")?;
    return Ok(());
}

#[test]
fn dots_99_a_2() -> Result<()> {
    let expr = "<math><mo>.</mo><mover><mn>1</mn><mo>&#x2D9;</mo></mover><mover><mn>3</mn><mo>&#x2D9;</mo></mover><mover><mn>5</mn><mo>&#x2D9;</mo></mover></math>";
    test_braille("Nemeth", expr, "⠼⠨⠐⠂⠒⠢⠣⠡⠻")?;
    return Ok(());
}

#[test]
fn dots_99_a_3() -> Result<()> {
    let expr = "<math><mn>.13</mn><mover><mn>5</mn><mo>&#x2D9;</mo></mover></math>";
    test_braille("Nemeth", expr, "⠼⠨⠂⠒⠐⠢⠣⠡⠻")?;
    return Ok(());
}

#[test]
fn ring_dot_100_1() -> Result<()> {
    let expr = "<math><mo>≗</mo></math>";
    test_braille("Nemeth", expr, "⠐⠨⠅⠣⠨⠡⠻")?;
    return Ok(());
}

#[test]
fn question_mark_over_equals_101_1() -> Result<()> {
    let expr = "<math><mover><mo>=</mo><mo>?</mo></mover></math>";
    test_braille("Nemeth", expr, "⠐⠨⠅⠣⠸⠦⠻")?;
    return Ok(());
}

#[test]
fn question_mark_under_equals_101_2() -> Result<()> {
    let expr = "<math><munder><mo>=</mo><mo>?</mo></munder></math>";
    test_braille("Nemeth", expr, "⠐⠨⠅⠩⠸⠦⠻")?;
    return Ok(());
}

#[test]
fn sqrt_103_a_2() -> Result<()> {
    let expr = "<math><msqrt><mrow><mi>x</mi><mo>+</mo><mi>y</mi></mrow></msqrt></math>";
    test_braille("Nemeth", expr, "⠜⠭⠬⠽⠻")?;
    return Ok(());
}

#[test]
fn sqrt_103_a_4() -> Result<()> {
    let expr = "<math><msqrt>
            <msup><mi>x</mi><mn>2</mn></msup>
            <mo>+</mo>
            <msup><mi>y</mi><mn>2</mn></msup>
        </msqrt></math>";
    test_braille("Nemeth", expr, "⠜⠭⠘⠆⠐⠬⠽⠘⠆⠐⠻")?;
    return Ok(());
}

#[test]
fn sqrt_103_b_2() -> Result<()> {
    let expr = "<math><mo>√</mo><mo>(</mo><mi>x</mi><mo>+</mo><mi>y</mi><mo>)</mo></math>";
    test_braille("Nemeth", expr, "⠜⠷⠭⠬⠽⠾")?;
    return Ok(());
}

#[test]
fn root_104_iii_1() -> Result<()> {
    let expr = "<math><mroot><mn>2</mn><mn>3</mn></mroot></math>";
    test_braille("Nemeth", expr, "⠣⠒⠜⠆⠻")?;
    return Ok(());
}

#[test]
fn root_104_iii_4() -> Result<()> {
    let expr = "<math><mroot>
            <mrow><mi>p</mi><mo>+</mo><mi>q</mi></mrow>
            <mrow><mi>m</mi><mo>+</mo><mi>n</mi></mrow>
        </mroot></math>";
    test_braille("Nemeth", expr, "⠣⠍⠬⠝⠜⠏⠬⠟⠻")?;
    return Ok(());
}

#[test]
fn nested_sqrt_105_1() -> Result<()> {
    let expr = "<math><msqrt><mi>x</mi><mo>+</mo>
            <msqrt><mi>x</mi><mo>+</mo><mi>y</mi></msqrt>
            <mo>+</mo><mi>z</mi></msqrt></math>";
    test_braille("Nemeth", expr, "⠜⠭⠬⠨⠜⠭⠬⠽⠨⠻⠬⠵⠻")?;
    return Ok(());
}

#[test]
fn nested_root_105_2() -> Result<()> {
    let expr = "<math><mroot>
    <mrow>
        <msup> <mi>x</mi><mn>2</mn> </msup>
        <mo>+</mo>
        <mroot>
            <mrow>
                <msup> <mi>x</mi> <mn>2</mn> </msup>
                <mo>+</mo>
                <msup> <mi>y</mi> <mn>2</mn>  </msup>
            </mrow>
            <mn>3</mn>
        </mroot>
        <mo>+</mo>
        <msup> <mi>y</mi> <mn>2</mn> </msup>
    </mrow>
    <mn>3</mn>
</mroot></math>";
    test_braille("Nemeth", expr, "⠣⠒⠜⠭⠘⠆⠐⠬⠨⠣⠒⠜⠭⠘⠆⠐⠬⠽⠘⠆⠐⠨⠻⠬⠽⠘⠆⠐⠻")?;
    return Ok(());
}


#[test]
fn nested_sqrt_105_3() -> Result<()> {
    let expr = "<math>
        <msqrt> <mroot> <mi>x</mi><mn>3</mn> </mroot> </msqrt>
        <mo>=</mo>
        <mroot> <msqrt><mi>x</mi></msqrt> <mn>3</mn></mroot>
    </math>";
    test_braille("Nemeth", expr, "⠜⠨⠣⠒⠜⠭⠨⠻⠻⠀⠨⠅⠀⠣⠒⠜⠨⠜⠭⠨⠻⠻")?;
    return Ok(());
}

#[test]
fn nested_sqrt_105_4() -> Result<()> {
    let expr = "<math>
            <msqrt><mi>x</mi><mo>+</mo><msqrt><mi>y</mi><mo>+</mo><msqrt><mi>z</mi></msqrt></msqrt></msqrt>
        </math>";
    test_braille("Nemeth", expr, "⠜⠭⠬⠨⠜⠽⠬⠨⠨⠜⠵⠨⠨⠻⠨⠻⠻")?;
    return Ok(());
}

#[test]
fn menclose_111_a_4() -> Result<()> {
    let expr = "<math><menclose notation='phasorangle'><mrow><mn>30</mn><mo>&#xB0;</mo></mrow></menclose></math>";
    test_braille("Nemeth", expr, "⠫⠪⠸⠫⠼⠒⠴⠘⠨⠡⠐⠻")?;
    return Ok(());
}

#[test]
fn menclose_111_a_1() -> Result<()> {
    let expr = "<math><menclose notation='circle'><mi>A</mi></menclose></math>";
    test_braille("Nemeth", expr, "⠫⠉⠸⠫⠠⠁⠻")?;
    return Ok(());
}

#[test]
fn shape_115_a_1() -> Result<()> {
    let expr = "<math><mo>∠</mo><mn>1</mn></math>";
    test_braille("Nemeth", expr, "⠫⠪⠀⠼⠂")?;
    return Ok(());
}

#[test]
fn shape_115_a_3() -> Result<()> {
    let expr = "<math><mo>&#x25CB;</mo><mi>R</mi></math>";
    test_braille("Nemeth", expr, "⠫⠉⠀⠠⠗")?;
    return Ok(());
}

#[test]
fn shape_115_a_6() -> Result<()> {
    let expr = "<math><mo>∟</mo><mi>A</mi></math>";
    test_braille("Nemeth", expr, "⠫⠪⠨⠗⠻⠀⠠⠁")?;
    return Ok(());
}

#[test]
fn shape_115_a_11() -> Result<()> {
    let expr = "<math><mi>m</mi><mo>&#x2220;</mo><mi>A</mi><mi>B</mi><mi>C</mi></math>";
    test_braille("Nemeth", expr, "⠍⠫⠪⠀⠠⠁⠠⠃⠠⠉")?;
    return Ok(());
}

#[test]
fn function_space_119_c_3() -> Result<()> {
    // this depends upon a canonicalization to get the degree sign into a superscript position
    let expr = "<math><mi>sin</mi><mn>30</mn><mo>&#xB0;</mo><mi>cos</mi><mn>45</mn><mo>&#xB0;</mo>
           <mo>+</mo><mi>cos</mi><mn>30</mn><mo>&#xB0;</mo><mi>sin</mi><mn>45</mn><mo>&#xB0;</mo></math>";
    test_braille("Nemeth", expr, "⠎⠊⠝⠀⠼⠒⠴⠘⠨⠡⠐⠉⠕⠎⠀⠼⠲⠢⠘⠨⠡⠐⠬⠉⠕⠎⠀⠼⠒⠴⠘⠨⠡⠐⠎⠊⠝⠀⠼⠲⠢⠘⠨⠡")?;
    return Ok(());
}

#[test]
fn brace_above_121_1() -> Result<()> {
    let expr = "<math><mover><mrow><mi>x</mi><mo>+</mo><mi>y</mi></mrow><mo>&#x23DE;</mo></mover></math>";
    test_braille("Nemeth", expr, "⠐⠭⠬⠽⠣⠨⠷⠻")?;
    return Ok(());
}

#[test]
fn brace_below_121_2() -> Result<()> {
    let expr = "<math><munder><mrow><mi>x</mi><mo>+</mo><mi>y</mi></mrow><mo>&#x23DF;</mo></munder></math>";
    test_braille("Nemeth", expr, "⠐⠭⠬⠽⠩⠨⠾⠻")?;
    return Ok(());
}

#[test]
fn bracket_above_121_3() -> Result<()> {
    let expr = "<math><mover><mrow><mi>x</mi><mo>+</mo><mi>y</mi></mrow><mo>&#x23B4;</mo></mover></math>";
    test_braille("Nemeth", expr, "⠐⠭⠬⠽⠣⠈⠷⠻")?;
    return Ok(());
}

#[test]
fn racket_below_121_4() -> Result<()> {
    let expr = "<math><munder><mrow><mi>x</mi><mo>+</mo><mi>y</mi></mrow><mo>&#x23B5;</mo></munder></math>";
    test_braille("Nemeth", expr, "⠐⠭⠬⠽⠩⠈⠾⠻")?;
    return Ok(());
}

#[test]
fn identity_matrix_126_linearize() -> Result<()> {
    // see https://github.com/NSoiffer/MathCAT/issues/43 for discussion on linear layout
    let expr = "<math> <mrow><mo>(</mo> <mtable>
        <mtr><mtd><mn>1</mn></mtd><mtd><mn>0</mn></mtd><mtd><mn>0</mn></mtd></mtr>
        <mtr><mtd><mn>0</mn></mtd><mtd><mn>1</mn></mtd><mtd><mn>0</mn></mtd></mtr>
        <mtr><mtd><mn>0</mn></mtd><mtd><mn>0</mn></mtd><mtd><mn>1</mn></mtd></mtr>
      </mtable><mo>)</mo></mrow></math>";
    // Note: braille pattern is just a guess as to how to linearize a matrix
    test_braille("Nemeth", expr, "⠠⠷⠼⠂⠀⠼⠴⠀⠼⠴⠀⣍⠴⠀⠼⠂⠀⠼⠴⠀⣍⠴⠀⠼⠴⠀⠼⠂⠠⠾")?;
    return Ok(());
}

#[test]
fn multipurpose_134_1() -> Result<()> {
    let expr = "<math><mo>+</mo><mn>2</mn><mo>-</mo><mo>+</mo><mn>3</mn></math>";
    test_braille("Nemeth", expr, "⠬⠆⠤⠐⠬⠒")?;
    return Ok(());
}

#[test]
fn plus_minus_134_4() -> Result<()> {
    let expr = "<math><mi>x</mi><mo>&#xB1;</mo><mi>y</mi></math>";
    test_braille("Nemeth", expr, "⠭⠬⠤⠽")?;
    return Ok(());
}

#[test]
fn multipurpose_lesson_5_2_3() -> Result<()> {
    let expr = "<math><mn>10</mn><mo>+</mo><mo>-</mo><mn>5</mn></math>";
    test_braille("Nemeth", expr, "⠼⠂⠴⠬⠐⠤⠢")?;
    return Ok(());
}

#[test]
fn multipurpose_lesson_5_2_4() -> Result<()> {
    let expr = "<math><mn>10</mn><mo>-</mo><mo>+</mo><mn>5</mn></math>";
    test_braille("Nemeth", expr, "⠼⠂⠴⠤⠐⠬⠢")?;
    return Ok(());
}

#[test]
fn multipurpose_lesson_5_2_5() -> Result<()> {
    let expr = "<math><mn>10</mn><mo>-</mo><mo>-</mo><mn>5</mn></math>";
    test_braille("Nemeth", expr, "⠼⠂⠴⠤⠐⠤⠢")?;
    return Ok(());
}

#[test]
fn tilde_137_2() -> Result<()> {
    let expr = "<math><mo>∼</mo><mi>p</mi><mo>∨</mo><mo>∼</mo><mi>q</mi><mo>∨</mo><mo>∼</mo><mi>r</mi></math>";
    test_braille("Nemeth", expr, "⠈⠱⠏⠈⠬⠈⠱⠟⠈⠬⠈⠱⠗")?;
    return Ok(());
}

#[test]
fn tilde_137_3() -> Result<()> {
    let expr = "<math><mo>~</mo><mo>~</mo><mi>T</mi><mo>&#x2228;</mo><mi>R</mi></math>";
    test_braille("Nemeth", expr, "⠈⠱⠐⠈⠱⠠⠞⠈⠬⠠⠗")?;
    return Ok(());
}

#[test]
fn tilde_137_3_mathjax() -> Result<()> {
    let expr = "<math><mo>∼∼</mo><mi>T</mi><mo>∨</mo><mi>R</mi></math>";
    test_braille("Nemeth", expr, "⠈⠱⠐⠈⠱⠠⠞⠈⠬⠠⠗")?;
    return Ok(());
}

#[test]
fn tilde_144_1() -> Result<()> {
    let expr = "<math><mi>x</mi><mo>∼</mo><mi>y</mi></math>";
    test_braille("Nemeth", expr, "⠭⠀⠈⠱⠀⠽")?;
    return Ok(());
}

#[test]
fn set_vertical_bar_145_1() -> Result<()> {
    let expr = "<math><mo>{</mo><mi>x</mi><mo>|</mo><mo>|</mo><mi>x</mi><mo>|</mo><mo>&lt;</mo><mn>10</mn><mo>}</mo></math>";
    test_braille("Nemeth", expr, "⠨⠷⠭⠀⠳⠀⠳⠭⠳⠀⠐⠅⠀⠼⠂⠴⠨⠾")?;
    return Ok(());
}

#[test]
fn vertical_bar_145_4() -> Result<()> {
    // this test was added in an addendum
    let expr = "<math><mi>P</mi><mo>(</mo><mi>A</mi><mo>|</mo><mi>B</mi><mo>)</mo></math>";
    test_braille("Nemeth", expr, "⠠⠏⠷⠠⠁⠀⠳⠀⠠⠃⠾")?;
    return Ok(());
}

#[test]
fn ratio_151_10() -> Result<()> {
    let expr = "<math><mn>1</mn><mo>:</mo><mn>2</mn><mo>∷</mo><mn>3</mn><mo>:</mo><mn>6</mn></math>";
    test_braille("Nemeth", expr, "⠼⠂⠀⠐⠂⠀⠼⠆⠀⠰⠆⠀⠼⠒⠀⠐⠂⠀⠼⠖")?;
    return Ok(());
}

#[test]
fn ratio_151_11() -> Result<()> {
    let expr = "<math><mi>a</mi><mo>+</mo><mi>b</mi><mo>:</mo><mi>b</mi><mo>∷</mo><mi>c</mi><mo>+</mo><mi>d</mi><mo>:</mo><mi>d</mi></math>";
    test_braille("Nemeth", expr, "⠁⠬⠃⠀⠐⠂⠀⠃⠀⠰⠆⠀⠉⠬⠙⠀⠐⠂⠀⠙")?;
    return Ok(());
}

#[test]
fn space_after_punct_bug_152() -> Result<()> {
    // this was a bug involving a bad cleanup rule for whitespace after punctuation
    let expr = "<math><mn>7</mn><mover><mo>=</mo><mo>?</mo></mover><mn>8</mn></math>";
    test_braille("Nemeth", expr, "⠼⠶⠀⠐⠨⠅⠣⠸⠦⠻⠀⠼⠦")?;
    return Ok(());
}

#[test]
fn arrow_lesson_9_5_1() -> Result<()> {
    // Nemeth rule 152
    let expr = "<math><mi>A</mi><mo>→</mo><mi>B</mi></math>";
    test_braille("Nemeth", expr, "⠠⠁⠀⠫⠕⠀⠠⠃")?;
    return Ok(());
}

#[test]
fn not_ratio_nfb_5_7_b_2() -> Result<()> {
    let expr = "<math><mo>{</mo><mi>x</mi><mo>:</mo><mi>x</mi><mo>></mo><mn>0</mn><mo>}</mo></math>";
    test_braille("Nemeth", expr, "⠨⠷⠰⠭⠸⠒⠀⠭⠀⠨⠂⠀⠼⠴⠨⠾")?;
    return Ok(());
}

#[test]
fn not_ratio_nfb_5_7_b_4() -> Result<()> {
    let expr = "<math><mi>p</mi><mo>:</mo><mi>r</mi><mo>=</mo><mi>q</mi><mo>:</mo><mi>s</mi></math>";
    test_braille("Nemeth", expr, "⠰⠏⠸⠒⠗⠀⠨⠅⠀⠟⠸⠒⠰⠎")?;
    return Ok(());
}

#[test]
fn trilinear_not_ratio() -> Result<()> {
    let expr = "<math><mi>a</mi><mo>:</mo><mi>b</mi><mo>:</mo><mi>c</mi></math>";
    // decided because spacing is symmetric, no space added after ':'
    test_braille("Nemeth", expr, "⠰⠁⠸⠒⠰⠃⠸⠒⠰⠉")?;
    return Ok(());
}

#[test]
fn extension_field_not_ratio() -> Result<()> {
    let expr = "<math><mo>[</mo><mi>K</mi><mo>:</mo><mi>F</mi><mo>]</mo></math>";
    // decided because spacing is symmetric, no space added after ':'
    test_braille("Nemeth", expr, "⠈⠷⠰⠠⠅⠸⠒⠰⠠⠋⠈⠾")?;
    return Ok(());
}

#[test]
fn proportional_151_12() -> Result<()> {
    let expr = "<math><mi>x</mi><mo>∝</mo><mi>y</mi></math>";
    test_braille("Nemeth", expr, "⠭⠀⠸⠿⠀⠽")?;
    return Ok(());
}

#[test]
fn comparison_ops_151_14() -> Result<()> {
    let expr = "<math><mo>{</mo><mi>x</mi><mo>|</mo><mn>0</mn><mo>≤</mo><mi>x</mi><mo>≤</mo><mn>1</mn><mo>}</mo></math>";
    test_braille("Nemeth", expr, "⠨⠷⠭⠀⠳⠀⠼⠴⠀⠐⠅⠱⠀⠭⠀⠐⠅⠱⠀⠼⠂⠨⠾")?;
    return Ok(());
}

#[test]
fn no_space_comparison_151_16() -> Result<()> {
    let expr = "<math><mo>(</mo><mo>&lt;</mo><mo>,</mo><mo>=</mo><mo>,</mo><mo>&gt;</mo><mo>)</mo></math>";
    test_braille("Nemeth", expr, "⠷⠐⠅⠠⠀⠨⠅⠠⠀⠨⠂⠾")?;
    return Ok(());
}

#[test]
fn in_scripts_comparison_151_17() -> Result<()> {
    let expr = "<math>
            <msubsup>
            <mo>∫</mo>
            <mrow><mi>x</mi><mo>=</mo><mi>a</mi></mrow>
            <mrow><mi>x</mi><mo>=</mo><mi>b</mi></mrow>
            </msubsup>
            <mrow>
            <mrow>
                <mi>f</mi>
                <mrow><mo>(</mo><mi>x</mi><mo>)</mo></mrow>
            </mrow>
            <mi>d</mi>
            <mi>x</mi>
            </mrow>
        </math>";
    test_braille("Nemeth", expr, "⠮⠰⠭⠀⠰⠨⠅⠀⠁⠘⠭⠀⠘⠨⠅⠀⠃⠐⠋⠷⠭⠾⠙⠭")?;
    return Ok(());
}

#[test]
fn degrees_165_1() -> Result<()> {
    let expr = "<math><mn>90</mn><mo>&#xB0;</mo><mo>+</mo><mn>90</mn><mo>&#xB0;</mo><mo>=</mo><mn>180</mn><mo>&#xB0;</mo></math>";
    test_braille("Nemeth", expr, "⠼⠔⠴⠘⠨⠡⠐⠬⠔⠴⠘⠨⠡⠀⠨⠅⠀⠼⠂⠦⠴⠘⠨⠡")?;
    return Ok(());
}

#[test]
fn prime_172_5() -> Result<()> {
    let expr = "<math><msubsup><mi>x</mi><mi>i</mi><mo>'</mo></msubsup></math>";
    test_braille("Nemeth", expr, "⠭⠄⠰⠊")?;
    return Ok(());
}

#[test]
fn prime_172_6() -> Result<()> {
    let expr = "<math><msubsup><mi>x</mi><mn>1</mn><mo>'</mo></msubsup></math>";
    test_braille("Nemeth", expr, "⠭⠄⠂")?;
    return Ok(());
}

#[test]
fn prime_172_8() -> Result<()> {
    let expr = "<math><msup><mover><mi>x</mi><mo>&#xAF;</mo></mover><mo>'</mo></msup></math>";
    test_braille("Nemeth", expr, "⠭⠱⠄")?;
    return Ok(());
}

#[test]
fn prime_172_9() -> Result<()> {
    let expr = "<math><msup><mn>5</mn><mo>'</mo></msup><msup><mn>8</mn><mrow><mo>'</mo><mo>'</mo></mrow></msup></math>";
    test_braille("Nemeth", expr, "⠼⠢⠄⠦⠄⠄")?;
    return Ok(());
}

#[test]
fn multipurpose_177_2_1() -> Result<()> {
    let expr = "<math> <mi>x5</mi> </math>";
    test_braille("Nemeth", expr, "⠭⠐⠢")?;
    return Ok(());
}

#[test]
fn multipurpose_177_2_2() -> Result<()> {
    let expr = "<math> <mi>x</mi> <mn>.6</mn> </math>";
    test_braille("Nemeth", expr, "⠭⠐⠨⠖")?;
    return Ok(());
}

#[test]
fn multipurpose_177_3_1() -> Result<()> {
    let expr = "<math>
            <msub><mi>c</mi><mn>0</mn></msub>
            <msup><mn>10</mn><mn>2</mn></msup>
            <mo>+</mo>
            <msub><mi>c</mi><mn>1</mn></msub>
            <mn>10</mn><mo>+</mo>
            <msub><mi>c</mi><mn>2</mn></msub>
        </math>";
    test_braille("Nemeth", expr, "⠉⠴⠐⠂⠴⠘⠆⠐⠬⠉⠂⠐⠂⠴⠬⠉⠆")?;
    return Ok(());
}

#[test]
fn multipurpose_177_5_1() -> Result<()> {
    let expr = "<math><mn>0.</mn><msub><mi>a</mi><mn>1</mn></msub><msub><mi>a</mi><mn>2</mn></msub><mo>…</mo></math>";
    test_braille("Nemeth", expr, "⠼⠴⠨⠐⠁⠂⠁⠆⠀⠄⠄⠄")?;
    return Ok(());
}

#[test]
fn multipurpose_177_5_4() -> Result<()> {
    let expr = "<math><mn>3.</mn><mo>+</mo><mn>.4</mn><mo>=</mo><mn>3.4</mn></math>";
    test_braille("Nemeth", expr, "⠼⠒⠨⠐⠬⠨⠲⠀⠨⠅⠀⠼⠒⠨⠲")?;
    return Ok(());
}

#[test]
fn multipurpose_177_5_5() -> Result<()> {
    let expr = "<math><mo>(</mo><mn>3.</mn><mo>)</mo></math>";
    test_braille("Nemeth", expr, "⠷⠒⠨⠐⠾")?;
    return Ok(());
}

#[test]
fn multipurpose_177_5_6() -> Result<()> {
    let expr = "<math><mfrac><mn>1.</mn><mn>2.</mn></mfrac></math>";
    test_braille("Nemeth", expr, "⠹⠂⠨⠐⠌⠆⠨⠐⠼")?;
    return Ok(());
}

#[test]
fn multipurpose_177_7_1() -> Result<()> {
    let expr = "<math><mrow><mo>|</mo><mi>x</mi><mo>|</mo></mrow><mrow><mo>|</mo><mi>y</mi><mo>|</mo></mrow></math>";
    test_braille("Nemeth", expr, "⠳⠭⠳⠐⠳⠽⠳")?;
    return Ok(());
}

#[test]
fn multipurpose_177_7_2() -> Result<()> {
    let expr = "<math><mrow><mo>‖</mo><mi>x</mi><mo>‖</mo></mrow><mrow><mo>‖</mo><mi>y</mi><mo>‖</mo></mrow></math>";
    test_braille("Nemeth", expr, "⠳⠳⠭⠳⠳⠐⠳⠳⠽⠳⠳")?;
    return Ok(());
}

#[test]
fn multipurpose_177_7_9() -> Result<()> {
    let expr = "<math><mo>∼</mo><mo>∼</mo><mi>T</mi></math>";
    test_braille("Nemeth", expr, "⠈⠱⠐⠈⠱⠠⠞")?;
    return Ok(());
}

#[test]
fn no_multipurpose_lesson_5_2_6() -> Result<()> {
    let expr = "<math><mo>-</mo><mn>10</mn><mo>+</mo><mo>+</mo><mn>5</mn></math>";
    test_braille("Nemeth", expr, "⠤⠼⠂⠴⠬⠬⠢")?;
    return Ok(());
}

#[test]
fn multipurpose_lesson_5_9_1_1() -> Result<()> {
    let expr = "<math><mi>n</mi><mo>&gt;</mo><mo>&lt;</mo><mn>1</mn></math>";
    test_braille("Nemeth", expr, "⠝⠀⠨⠂⠐⠐⠅⠀⠼⠂")?;
    return Ok(());
}

#[test]
fn multipurpose_lesson_5_9_2_1() -> Result<()> {
    let expr = "<math><mi>n</mi><mo>&lt;</mo><mo>&gt;</mo><mn>1</mn></math>";
    test_braille("Nemeth", expr, "⠝⠀⠐⠅⠐⠨⠂⠀⠼⠂")?;
    return Ok(());
}

#[test]
fn multipurpose_lesson_5_9_2_2() -> Result<()> {
    let expr = "<math><mi>n</mi><mo>&lt;</mo><mo>=</mo><mo>&gt;</mo><mn>1</mn></math>";
    test_braille("Nemeth", expr, "⠝⠀⠐⠅⠐⠨⠅⠐⠨⠂⠀⠼⠂")?;
    return Ok(());
}

#[test]
fn lesson_11_24_1() -> Result<()> {
    let expr = "<math><menclose notation='roundedbox'><msup><mi>x</mi><mn>2</mn></msup></menclose></math>";
    test_braille("Nemeth", expr, "⠫⠅⠭⠘⠆⠐⠻")?;
    return Ok(());
}

#[test]
fn ms_38_4_8() -> Result<()> {
    let expr = "<math><mo>(</mo><ms lquote='“' rquote='”'>three</ms><mo>)</mo></math>";
    test_braille("Nemeth", expr, "⠷⠸⠦⠞⠓⠗⠑⠑⠴⠾")?;
    return Ok(());
}

#[test]
fn ms() -> Result<()> {
    let expr = "<math><ms>a string</ms><mo>,</mo><ms lquote='‘' rquote='’'>another string</ms></math>";
    // Not 100% sure this is the right output -- I am a little skeptical of "⠄⠄" being the braille for '"'
    // Note: no punct indicator after word (see 38_4_8)
    test_braille("Nemeth", expr, "⠄⠄⠁⠀⠎⠞⠗⠊⠝⠛⠄⠄⠠⠀⠸⠠⠦⠁⠝⠕⠞⠓⠑⠗⠀⠎⠞⠗⠊⠝⠛⠴⠠")?;
    return Ok(());
}

#[test]
fn full_binomial() -> Result<()> {
    let expr = "<math>
    <mo stretchy='false'>(</mo>
    <mi>x</mi>
    <mo>+</mo>
    <mi>a</mi>
    <msup>
        <mo stretchy='false'>)</mo>
        <mrow>
            <mi>n</mi>
        </mrow>
    </msup>
    <mo>=</mo>
    <munderover>
        <mo>∑</mo>
        <mrow>
            <mi>k</mi>
            <mo>=</mo>
            <mn>0</mn>
        </mrow>
        <mrow>
            <mi>n</mi>
        </mrow>
    </munderover>
    <mrow>
        <mo>(</mo>
        <mfrac linethickness='0'>
            <mi>n</mi>
            <mi>k</mi>
        </mfrac>
        <mo>)</mo>
    </mrow>
    <msup>
        <mi>x</mi>
        <mrow>
            <mi>k</mi>
        </mrow>
    </msup>
    <msup>
        <mi>a</mi>
        <mrow>
            <mi>n</mi>
            <mo>−</mo>
            <mi>k</mi>
        </mrow>
    </msup>
</math>
";
    test_braille("Nemeth", expr, "⠷⠭⠬⠁⠾⠘⠝⠀⠨⠅⠀⠐⠨⠠⠎⠩⠅⠀⠨⠅⠀⠼⠴⠣⠝⠻⠷⠝⠩⠅⠾⠭⠘⠅⠐⠁⠘⠝⠤⠅")?;
    return Ok(());
}

// Extra tests targeted at special cases in MathCAT
#[test]
fn number_space_before() -> Result<()> {
    let expr = "<math><mtext>&#xA0;</mtext><mn>2</mn></math>";
    test_braille("Nemeth", expr, "⠼⠆")?;
    return Ok(());
}

#[test]
fn number_space_after() -> Result<()> {
    let expr = "<math><mn>2</mn><mtext>&#xA0;</mtext></math>";
    test_braille("Nemeth", expr, "⠼⠆")?;
    return Ok(());
}

#[test]
fn number_space_before_and_after() -> Result<()> {
    let expr = "<math><mtext>&#xA0;</mtext><mn>2</mn><mtext>&#xA0;</mtext></math>";
    test_braille("Nemeth", expr, "⠼⠆")?;
    return Ok(());
}

#[test]
fn tensor_from_mathml_spec() -> Result<()> {
    let expr = "<math>
    <mmultiscripts>
      <mi>R</mi>
      <mi>i</mi> <none></none>
      <none></none> <mi>j</mi>
      <mi>k</mi> <none></none>
      <mi>l</mi> <none></none>
    </mmultiscripts>
    </math>";
    // Note: the braille answer was verified to be correct (see https://github.com/NSoiffer/MathCAT/issues/55) 
    test_braille("Nemeth", expr, "⠠⠗⠰⠊⠐⠘⠚⠐⠰⠅⠐⠰⠇")?;
    return Ok(());
}

// The following are from the new BANA Nemeth Code (2020): https://www.brailleauthority.org/nemeth/2020-nemeth-code.html

#[test]
fn perpendicular_17_57() -> Result<()> {
    let expr = "<math><mi>A</mi><mi>B</mi><mo>&#x22A5;</mo><mi>C</mi><mi>D</mi></math>";
    test_braille("Nemeth", expr, "⠠⠁⠠⠃⠀⠫⠏⠀⠠⠉⠠⠙")?;
    return Ok(());
}




// The following are chemistry tests from Braille Code of Chemical Notation 1997 (http://www.brl.org/chemistry/ which seems bug, update in late 2023?)
#[test]
fn chem_HOH_1_1_1_mchem() -> Result<()> {
    let expr = "<math>
      <mi mathvariant='normal'>H</mi>
      <mo>-</mo>
      <mi mathvariant='normal'>O</mi>
      <mo>-</mo>
      <mi mathvariant='normal'>H</mi>
   </math>";
    test_braille("Nemeth", expr, "⠠⠓⠸⠒⠻⠠⠕⠸⠒⠻⠠⠓")?;
    return Ok(());
}

// The following are chemistry tests from Braille Code of Chemical Notation 1997 (http://www.brl.org/chemistry/ which seems bug, update in late 2023?)
#[test]
fn chem_2_5_1_mchem() -> Result<()> {
    let expr = "<math>
        <mrow>
        <mrow><mi>CaC</mi></mrow>
        <msub>
            <mrow><mrow><mpadded width='0'><mphantom><mi>A</mi></mphantom></mpadded></mrow></mrow>
            <mrow><mrow><mpadded height='0'><mn>2</mn></mpadded></mrow></mrow>
        </msub>
        <mrow></mrow>
        <mo>+</mo>
        <mrow></mrow>
        <mn>2</mn>
        <mstyle scriptlevel='0'><mspace width='0.167em'/></mstyle>
        <mrow><mi>HOH</mi></mrow>
        <mrow></mrow>
        <mrow><mo stretchy='false'>&#x27F6;</mo></mrow>
        <mrow></mrow>
        <mrow><mi mathvariant='normal'>H</mi></mrow>
        <mrow><mo>&#x2212;</mo></mrow>
        <mrow><mi mathvariant='normal'>C</mi></mrow>
        <mrow><mo>&#x2261;</mo></mrow>
        <mrow><mi mathvariant='normal'>C</mi></mrow>
        <mrow><mo>&#x2212;</mo></mrow>
        <mrow><mi mathvariant='normal'>H</mi></mrow>
        <mrow></mrow>
        <mo>+</mo>
        <mrow></mrow>
        <mrow><mi>Ca</mi></mrow>
        <mo stretchy='false'>(</mo>
        <mrow><mi>OH</mi></mrow>
        <mo stretchy='false'>)</mo>
        <msub>
            <mrow><mrow><mpadded width='0'><mphantom><mi>A</mi></mphantom></mpadded></mrow></mrow>
            <mrow><mrow><mpadded height='0'><mn>2</mn></mpadded></mrow></mrow>
        </msub>
        </mrow>
    </math>";
    // The example uses a short right arrow but chemistry normally uses a long one -- this test has a long right arrow so that char differs from the reference
    test_braille("Nemeth", expr, "⠠⠉⠁⠠⠉⠆⠬⠆⠠⠓⠠⠕⠠⠓⠀⠫⠒⠒⠒⠕⠀⠠⠓⠸⠒⠻⠠⠉⠸⠿⠻⠠⠉⠸⠒⠻⠠⠓⠬⠠⠉⠁⠷⠠⠕⠠⠓⠾⠰⠆")?;
    return Ok(());
}
