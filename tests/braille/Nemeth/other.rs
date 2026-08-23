// These are additional Nemeth tests from other sources, mainly from bugs/issues
use crate::common::*;
use anyhow::Result;


#[test]
fn not_number_space_blocks() -> Result<()> {
    // https://github.com/NSoiffer/MathCAT/issues/144
    let expr = "<math><mn>123</mn><mtext>&nbsp;&#x2063;</mtext><mn>456</mn></math>";
    test_braille("Nemeth", expr, "⠼⠂⠆⠒⠀⠼⠲⠢⠖")?;
    return Ok(());

}

#[test]
fn space_between_digits() -> Result<()> {
    // https://github.com/NSoiffer/MathCAT/issues/144
    let expr = "<math><mn>1</mn><mtext>&#x00a0;</mtext><mn>3</mn><mtext>&#x00a0;</mtext><mn>5</mn></math>";
    test_braille("Nemeth", expr, "⠼⠂⠀⠼⠒⠀⠼⠢")?;
    return Ok(());

}

#[test]
fn space_hack_between_digits() -> Result<()> {
    // https://github.com/NSoiffer/MathCAT/issues/144
    let expr = "<math><mn>1</mn><mtext>&#x00a0;&#x2063;</mtext><mn>3</mn><mtext>&#x00a0;&#x2063;</mtext><mn>5</mn></math>";
    test_braille("Nemeth", expr, "⠼⠂⠀⠼⠒⠀⠼⠢")?;
    return Ok(());

}

#[test]
fn tilde_prefix_bug_244() -> Result<()> {
    // https://github.com/NSoiffer/MathCAT/issues/244
    let expr = "<math> <mo>~</mo> <mi>p</mi> </math>";
    test_braille("Nemeth", expr, "⠈⠱⠏")?;
    return Ok(());

}

#[test]
fn double_struck_bug_334() -> Result<()> {
    // https://github.com/NSoiffer/MathCAT/issues/334 -- double struck was problem (⠼ was missing); test all of the scripted numbers here
    let expr = "<math><mn>𝟙</mn><mo>,</mo><mn>𝟐</mn><mo>,</mo><mn>𝟯</mn><mo>,</mo><mn>𝟺</mn></math>";
    test_braille("Nemeth", expr, "⠠⠸⠼⠂⠠⠀⠸⠼⠆⠠⠀⠠⠨⠸⠼⠒⠠⠀⠼⠲")?;
    return Ok(());

}

#[test]
fn extra_indicators_bug_343() -> Result<()> {
    // https://github.com/NSoiffer/MathCAT/issues/343 -- extra indicators before baseline indicator due to -x^2, not there for x^2
    let expr = "<math xmlns='http://www.w3.org/1998/Math/MathML'>
                        <mrow>
                        <msup>
                            <mi>e</mi>
                            <mrow>
                            <mo>-</mo>
                            <msup>
                                <mi>x</mi>
                                <mn>2</mn>
                            </msup>
                            </mrow>
                        </msup>
                        <mo>+</mo>
                        <msub>
                            <mi>C</mi>
                            <mn>1</mn>
                        </msub>
                        </mrow>
                    </math>";
    test_braille("Nemeth", expr, "⠑⠘⠤⠭⠘⠘⠆⠐⠬⠠⠉⠂")?;
    return Ok(());

}


#[test]
fn find_baseline_indicator_bug_364() -> Result<()> {
    // https://github.com/NSoiffer/MathCAT/issues/364
    let expr = r#" <math id='id-0'>
        <mrow data-changed='added' id='id-1'>
            <mi id='id-2'>π</mi>
            <mo id='id-3'>&#x2062;</mo>
            <msup id='id-4'>
                <mi id='id-5'>r</mi>
                <mn id='id-6'>2</mn>
            </msup>
        </mrow>
    </math>"#;
    test_braille_nav_position("Nemeth", expr, 4, "id-6", 0)?;
    return Ok(());
}

#[test]
fn no_omission_for_spaces_at_start_or_end_single() -> Result<()> {
    // http://github.com/TalkingCatSW/MathCAT/issues/468
    let expr = r#" <math><mtext> </mtext><mtext> </mtext><mi>x</mi><mtext> </mtext><mtext> </mtext></math>"#;
    test_braille("Nemeth", expr, "⠰⠭")?;
    return Ok(());

}

#[test]
fn no_omission_for_spaces_at_start() -> Result<()> {
    // http://github.com/TalkingCatSW/MathCAT/issues/468
    let expr = r#"<math><mtext> </mtext><mtext> </mtext><mn>2</mn><mo>+</mo><mi>x</mi></math>"#;
    test_braille("Nemeth", expr, "⠼⠆⠬⠭")?;
    return Ok(());

}

#[test]
fn no_omission_for_spaces_at_end() -> Result<()> {
    // http://github.com/TalkingCatSW/MathCAT/issues/468
    let expr = r#"<math><msup><mi>x</mi><mn>2</mn></msup><mo>+</mo><mn>9</mn>
                        <mtext> </mtext><mtext> </mtext><mtext> </mtext><mtext> </mtext><mtext> </mtext></math>"#;
    test_braille("Nemeth", expr, "⠭⠘⠆⠐⠬⠔")?;
    return Ok(());

}

#[test]
fn no_omission_for_spaces_in_middle() -> Result<()> {
    // http://github.com/TalkingCatSW/MathCAT/issues/468
    let expr = r#"<math xmlns="http://www.w3.org/1998/Math/MathML" display="block">
        <mstyle displaystyle="true" scriptlevel="0">
            <mfrac>
            <mn>1</mn>
            <mn>2</mn>
            </mfrac>
        </mstyle>
        <mo stretchy="false">(</mo>
        <mi>p</mi>
        <mo>+</mo>
        <mi>q</mi>
        <mo stretchy="false">)</mo>
        <mtext>&#xA0;</mtext>
        <mtext>&#xA0;</mtext>
        <mtext>&#xA0;</mtext>
        <mtext>&#xA0;</mtext>
        <mtext>or</mtext>
        <mtext>&#xA0;</mtext>
        <mtext>&#xA0;</mtext>
        <mtext>&#xA0;</mtext>
        <mtext>&#xA0;</mtext>
        <mstyle displaystyle="true" scriptlevel="0">
            <mfrac>
            <mrow>
                <mi>p</mi>
                <mo>+</mo>
                <mi>q</mi>
            </mrow>
            <mn>2</mn>
            </mfrac>
        </mstyle>
    </math>"#;
    test_braille("Nemeth", expr, "⠹⠂⠌⠆⠼⠷⠏⠬⠟⠾⠀⠕⠗⠀⠹⠏⠬⠟⠌⠆⠼")?;
    return Ok(());

}

#[test]
fn dots() -> Result<()> {
  let expr = "<math>
         <mover><mi>x</mi><mo>.</mo></mover><mo>+</mo>
         <mover><mi>x</mi><mo>..</mo></mover><mo>+</mo>
         <mover><mi>x</mi><mo>...</mo></mover>
    </math>";
    test_braille("Nemeth",expr, "⠐⠭⠣⠡⠻⠬⠐⠭⠣⠡⠡⠻⠬⠐⠭⠣⠡⠡⠡⠻")?;
  return Ok(());
}
