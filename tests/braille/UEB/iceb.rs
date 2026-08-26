// UEB tests for the basic mathml tags
// BANA guidance (2026; replaces 2019 provisional):
//   https://www.brailleauthority.org/sites/default/files/2026-07/Guidance%20on%20Transcribing%20Math%20and%20Science%20in%20UEB%202026.pdf
// Older `bana_*` names map from 2019 section numbers where still valid; `bana2026_*` are new 2026 examples.
// Grade 1 indicators follow ICEB GTM §1.7 (adopted by BANA 2026).
//
// Many come from (refer to) https://iceb.org/guidelines_for_technical_material_2014.pdf
// For example, "fraction_6_1_1" is a fraction example from section 6.1, and is the first example there.
use crate::common::*;
use anyhow::Result;

#[test]
fn bana_2_1() -> Result<()> {
    // BANA 2026 Ex 2-1 (spacing of signs)
    let expr = "<math><mn>6</mn><mo>=</mo><mn>1</mn><mo>×</mo><mn>2</mn><mo>×</mo><mn>3</mn>
                <mo>=</mo><mn>1</mn><mo>+</mo><mn>2</mn><mo>+</mo><mn>3</mn></math>";
    test_braille("UEB", expr, "⠼⠋⠀⠐⠶⠀⠼⠁⠐⠦⠼⠃⠐⠦⠼⠉⠀⠐⠶⠀⠼⠁⠐⠖⠼⠃⠐⠖⠼⠉")?;
    return Ok(());

}

#[test]
fn bana_5_1() -> Result<()> {
    // BANA 2026 Ex 5-1
    let expr = "<math><mi>x</mi><mo>+</mo><mi>y</mi><mo>=</mo><mn>6</mn></math>";
    test_braille("UEB", expr, "⠭⠐⠖⠽⠀⠐⠶⠀⠼⠋")?;
    return Ok(());

}

#[test]
fn bana_5_2() -> Result<()> {
    // BANA 2026 Ex 5-7
    let expr = "<math><msup><mi>x</mi><mn>2</mn></msup><mo>+</mo><msup><mi>y</mi><mn>2</mn></msup><mo>=</mo><mi>C</mi></math>";
    test_braille("UEB", expr, "⠭⠰⠔⠼⠃⠐⠖⠽⠔⠼⠃⠀⠐⠶⠀⠰⠠⠉")?;
    return Ok(());

}

#[test]
fn bana_5_3() -> Result<()> {
    // BANA 2026 Ex 5-10
    let expr = "<math><mfrac><mi>a</mi><mi>b</mi></mfrac><mo>+</mo><mfrac><mi>c</mi><mi>d</mi></mfrac></math>";
    test_braille("UEB", expr, "⠰⠰⠷⠁⠨⠌⠃⠾⠐⠖⠷⠉⠨⠌⠙⠾")?;
    return Ok(());

}

#[test]
fn bana_5_4() -> Result<()> {
    // BANA 2026 Ex 5-14 (GTM 1.7: word indicator per sequence, not passage)
    let expr = "<math><msup><mi>a</mi><mi>n</mi></msup><mo>×</mo><msup><mi>a</mi><mi>m</mi></msup><mo>=</mo>
                    <msup><mi>a</mi><mrow><mi>n</mi><mo>+</mo><mi>m</mi></mrow></msup></math>";
    test_braille("UEB", expr, "⠰⠰⠁⠔⠝⠐⠦⠁⠔⠍⠀⠐⠶⠀⠰⠰⠁⠔⠣⠝⠐⠖⠍⠜")?;
    return Ok(());

}

#[test]
fn bana_5_5() -> Result<()> {
    // BANA 2026 Ex 5-8 (replaces 2019 log_x y example)
    let expr = "<math><msub><mi>log</mi><mn>2</mn></msub><mi>x</mi><mo>+</mo><mi>log</mi><mi>x</mi></math>";
    test_braille("UEB", expr, "⠇⠕⠛⠰⠢⠼⠃⠭⠐⠖⠇⠕⠛⠀⠰⠭")?;
    return Ok(());

}

#[test]
fn bana_5a_1() -> Result<()> {
    // Former 2019 §5(a) units example (dropped from BANA 2026); kept as unit-spacing practice
    let expr = "<math><msup><mn>100</mn><mo>°</mo></msup><mi>F</mi></math>";
    test_braille("UEB", expr, "⠼⠁⠚⠚⠘⠚⠠⠋")?;
    return Ok(());

}

#[test]
fn bana_5a_1_baseline() -> Result<()> {
    // Former 2019 §5(a) units example (dropped from BANA 2026); kept as unit-spacing practice
    let expr = "<math><mn>100</mn><mo>°</mo><mi class='MathML-unit'>F</mi></math>";
    test_braille("UEB", expr, "⠼⠁⠚⠚⠘⠚⠠⠋")?;
    return Ok(());

}

#[test]
fn bana_5a_1_baseline_unit() -> Result<()> {
    // Same as bana_5a_1_baseline; unit marked via data-intent-property
    let expr = "<math><mn>100</mn><mo>°</mo><mi data-intent-property=':unit:'>F</mi></math>";
    test_braille("UEB", expr, "⠼⠁⠚⠚⠘⠚⠠⠋")?;
    return Ok(());

}

#[test]
fn bana_5a_2() -> Result<()> {
    // Former 2019 §5(a) units example (dropped from BANA 2026); kept as unit-spacing practice
    let expr = "<math><mn>25</mn><mo>&#xA0;</mo><msup><mi class='MathML-unit'>km</mi><mn>2</mn></msup></math>";
    test_braille("UEB", expr, "⠼⠃⠑⠀⠅⠍⠰⠔⠼⠃")?;
    return Ok(());

}

#[test]
fn bana_5a_2_unit() -> Result<()> {
    // Same as bana_5a_2; unit marked via data-intent-property
    let expr = "<math><mn>25</mn><mo>&#xA0;</mo><msup><mi data-intent-property=':unit:'>km</mi><mn>2</mn></msup></math>";
    test_braille("UEB", expr, "⠼⠃⠑⠀⠅⠍⠰⠔⠼⠃")?;
    return Ok(());

}

#[test]
fn bana_5a_2_mtext() -> Result<()> {
    // Former 2019 §5(a) units example (dropped from BANA 2026); kept as unit-spacing practice
    let expr = "<math><mn>25</mn><mo>&#xA0;</mo><msup><mtext class='MathML-unit'>km</mtext><mn>2</mn></msup></math>";
    test_braille("UEB", expr, "⠼⠃⠑⠀⠅⠍⠰⠔⠼⠃")?;
    return Ok(());

}

#[test]
fn bana_5a_2_mtext_unit() -> Result<()> {
    // Same as bana_5a_2_mtext; unit marked via data-intent-property
    let expr = "<math><mn>25</mn><mo>&#xA0;</mo><msup><mtext data-intent-property=':unit:'>km</mtext><mn>2</mn></msup></math>";
    test_braille("UEB", expr, "⠼⠃⠑⠀⠅⠍⠰⠔⠼⠃")?;
    return Ok(());

}

#[test]
fn bana_5a_3() -> Result<()> {
    // Former 2019 §5(a) units example (dropped from BANA 2026); kept as unit-spacing practice
    let expr = "<math><mn>6</mn><mo>&#xA0;</mo><mi class='MathML-unit'>m</mi><mo>&#xA0;</mo>
            <msup><mi class='MathML-unit'>s</mi><mrow><mo>-</mo><mn>1</mn></mrow></msup></math>";
    test_braille("UEB", expr, "⠼⠋⠀⠰⠍⠀⠰⠰⠎⠔⠣⠐⠤⠼⠁⠜")?;
    return Ok(());

}

#[test]
fn bana_5a_3_unit() -> Result<()> {
    // Same as bana_5a_3; unit marked via data-intent-property
    let expr = "<math><mn>6</mn><mo>&#xA0;</mo><mi data-intent-property=':unit:'>m</mi><mo>&#xA0;</mo>
            <msup><mi data-intent-property=':unit:'>s</mi><mrow><mo>-</mo><mn>1</mn></mrow></msup></math>";
    test_braille("UEB", expr, "⠼⠋⠀⠰⠍⠀⠰⠰⠎⠔⠣⠐⠤⠼⠁⠜")?;
    return Ok(());

}

#[test]
fn bana_6_1() -> Result<()> {
    // Punctuation before G1 terminator (BANA 2026 §5.4 / GTM 1.7.3b note)
    let expr = "<math><mi>x</mi><mo>+</mo><mi>y</mi><mo>=</mo><mi>z</mi>
                        <mo>=</mo><msup><mi>t</mi><mn>2</mn></msup><mo>.</mo></math>";
    test_braille("UEB", expr, "⠭⠐⠖⠽⠀⠐⠶⠀⠰⠵⠀⠐⠶⠀⠞⠰⠔⠼⠃⠲")?;
    return Ok(());

}

// --- BANA 2026 examples not covered by older bana_* tests ---

#[test]
fn bana2026_2_2() -> Result<()> {
    // BANA 2026 Ex 2-2: spaced operation signs for beginning readers
    let expr = "<math><mn>6</mn><mo>-</mo><mn>3</mn><mo>=</mo><mo>_</mo></math>";
    test_braille_prefs("UEB", vec![("UseSpacesAroundAllOperators", "true")], expr, "⠼⠋⠀⠐⠤⠀⠼⠉⠀⠐⠶⠀⠨⠤")?;
    return Ok(());

}

#[test]
fn bana2026_5_2() -> Result<()> {
    // BANA 2026 Ex 5-2: words with contractions in a technical expression
    let expr = "<math><mtext>Surface area</mtext><mo>=</mo><mn>2</mn><mi>l</mi><mi>h</mi><mo>+</mo><mn>2</mn><mi>l</mi><mi>w</mi><mo>+</mo><mn>2</mn><mi>w</mi><mi>h</mi></math>";
    test_braille("UEB", expr, "⠠⠎⠥⠗⠋⠁⠉⠑⠀⠜⠑⠁⠀⠐⠶⠀⠼⠃⠇⠓⠐⠖⠼⠃⠇⠺⠐⠖⠼⠃⠺⠓")?;
    return Ok(());

}

#[test]
fn bana2026_5_3() -> Result<()> {
    // BANA 2026 Ex 5-3
    let expr = "<math><mi>NaCl</mi><mo>+</mo><msub><mi>H</mi><mn>2</mn></msub><mi>O</mi></math>";
    test_braille("UEB", expr, "⠠⠝⠁⠠⠉⠇⠐⠖⠠⠓⠰⠢⠼⠃⠠⠕")?;
    return Ok(());

}

#[test]
fn bana2026_5_4() -> Result<()> {
    // BANA 2026 Ex 5-4
    let expr = "<math><mfrac><mrow><mn>5</mn><msup><mi>x</mi><mn>2</mn></msup></mrow><mrow><msup><mi>y</mi><mn>3</mn></msup><mi>z</mi></mrow></mfrac></math>";
    test_braille("UEB", expr, "⠰⠷⠼⠑⠭⠔⠼⠃⠨⠌⠽⠔⠼⠉⠵⠾")?;
    return Ok(());

}

#[test]
fn bana2026_5_5() -> Result<()> {
    // BANA 2026 Ex 5-5: 1×3 matrix and column (linearized like other matrix_* tests)
    let expr = r#"<math>
      <mrow><mo>(</mo><mtable><mtr><mtd><mi>a</mi></mtd><mtd><mi>b</mi></mtd><mtd><mi>c</mi></mtd></mtr></mtable><mo>)</mo></mrow>
      <mrow><mo>(</mo><mtable>
        <mtr><mtd><mo>-</mo><mn>1</mn></mtd></mtr>
        <mtr><mtd><mn>2</mn></mtd></mtr>
        <mtr><mtd><mo>-</mo><mn>3</mn></mtd></mtr>
      </mtable><mo>)</mo></mrow>
    </math>"#;
    test_braille("UEB", expr, "⠠⠐⠣⠁⠀⠰⠃⠀⠉⠠⠐⠜⠠⠐⠣⠐⠤⠼⠁⠠⠐⠜⠸⠀⠠⠐⠣⠼⠃⠠⠐⠜⠸⠀⠠⠐⠣⠐⠤⠼⠉⠠⠐⠜")?;
    return Ok(());

}

#[test]
fn bana2026_5_6() -> Result<()> {
    // BANA 2026 Ex 5-6
    let expr = r#"<math>
      <mrow><mo>(</mo><mtable><mtr><mtd><mi>a</mi></mtd><mtd><mi>b</mi></mtd></mtr></mtable><mo>)</mo></mrow>
      <mrow><mo>(</mo><mtable><mtr><mtd><mo>-</mo><mi>x</mi></mtd></mtr><mtr><mtd><mn>2</mn><mi>y</mi></mtd></mtr></mtable><mo>)</mo></mrow>
    </math>"#;
    test_braille("UEB", expr, "⠠⠐⠣⠁⠀⠃⠠⠐⠜⠠⠐⠣⠐⠤⠭⠠⠐⠜⠸⠀⠠⠐⠣⠼⠃⠽⠠⠐⠜")?;
    return Ok(());

}

#[test]
fn bana2026_5_9_slope() -> Result<()> {
    // BANA 2026 Ex 5-9 (math fragments): slope 3/5 and parallel lines l₁ ∥ l₂
    let expr = "<math><msub><mi>l</mi><mn>1</mn></msub><mo>∥</mo><msub><mi>l</mi><mn>2</mn></msub></math>";
    test_braille("UEB", expr, "⠇⠰⠢⠼⠁⠀⠼⠇⠀⠇⠰⠢⠼⠃")?;
    return Ok(());

}

#[test]
fn bana2026_5_11() -> Result<()> {
    // BANA 2026 Ex 5-11
    let expr = "<math><mfrac><mi>a</mi><mi>b</mi></mfrac></math>";
    test_braille("UEB", expr, "⠰⠰⠷⠁⠨⠌⠃⠾")?;
    return Ok(());

}

#[test]
fn bana2026_5_12() -> Result<()> {
    // BANA 2026 Ex 5-12: vector AB with arrow
    let expr = "<math><mover><mrow><mi>A</mi><mi>B</mi></mrow><mo>→</mo></mover></math>";
    test_braille("UEB", expr, "⠰⠰⠣⠠⠠⠁⠃⠜⠘⠱")?;
    return Ok(());

}

#[test]
fn bana2026_5_13() -> Result<()> {
    // BANA 2026 Ex 5-13
    let expr = "<math><msup><mi>x</mi><mrow><mo>-</mo><mn>1</mn></mrow></msup><mo>=</mo><mfrac><mn>1</mn><mi>x</mi></mfrac></math>";
    test_braille("UEB", expr, "⠰⠰⠭⠔⠣⠐⠤⠼⠁⠜⠀⠐⠶⠀⠰⠷⠼⠁⠨⠌⠭⠾")?;
    return Ok(());

}

#[test]
fn bana2026_5_15() -> Result<()> {
    // BANA 2026 Ex 5-15: all-caps print uses capital passage (⠠⠠⠠ … ⠠⠄) inside G1 passage
    let expr = "<math><mi>P</mi><mo>=</mo><mi>V</mi><mi>I</mi><mo>=</mo><msup><mi>I</mi><mn>2</mn></msup><mi>R</mi><mo>=</mo><mfrac><msup><mi>V</mi><mn>2</mn></msup><mi>R</mi></mfrac></math>";
    test_braille("UEB", expr, "⠰⠰⠰⠠⠠⠠⠏⠀⠐⠶⠀⠧⠊⠀⠐⠶⠀⠊⠔⠼⠃⠗⠀⠐⠶⠀⠷⠧⠔⠼⠃⠨⠌⠗⠾⠠⠄⠰⠄")?;
    return Ok(());

}

#[test]
fn bana2026_5_16() -> Result<()> {
    // BANA 2026 Ex 5-16: CaCO₃ ⇄ CaO + CO₂
    let expr = "<math><mi mathvariant='normal'>Ca</mi><mi mathvariant='normal'>C</mi><msub><mi mathvariant='normal'>O</mi><mn>3</mn></msub>
            <mo>⇄</mo>
            <mi mathvariant='normal'>Ca</mi><mi mathvariant='normal'>O</mi><mo>+</mo>
            <mi mathvariant='normal'>C</mi><msub><mi mathvariant='normal'>O</mi><mn>2</mn></msub></math>";
    test_braille("UEB", expr, "⠰⠰⠰⠠⠉⠁⠠⠉⠠⠕⠢⠼⠉⠀⠳⠕⠻⠳⠪⠀⠠⠉⠁⠠⠕⠐⠖⠠⠉⠠⠕⠢⠼⠃⠰⠄")?;
    return Ok(());

}

#[test]
fn bana2026_7_1() -> Result<()> {
    // BANA 2026 Ex 7-1: V = lim Δx→0 Σ f(xᵢ)Δx (single-line)
    let expr = r#"<math><mi>V</mi><mo>=</mo>
      <munder><mi>lim</mi><mrow><mi>Δ</mi><mi>x</mi><mo>→</mo><mn>0</mn></mrow></munder>
      <munder><mo>∑</mo><mi>i</mi></munder>
      <mi>f</mi><mo>(</mo><msub><mi>x</mi><mi>i</mi></msub><mo>)</mo><mi>Δ</mi><mi>x</mi>
    </math>"#;
    test_braille("UEB", expr, "⠰⠠⠧⠀⠐⠶⠀⠰⠰⠇⠊⠍⠨⠢⠣⠠⠨⠙⠭⠳⠕⠼⠚⠜⠠⠨⠎⠨⠢⠊⠋⠐⠣⠭⠢⠊⠐⠜⠠⠨⠙⠭")?;
    return Ok(());

}

#[test]
fn bana2026_7_2() -> Result<()> {
    // BANA 2026 Ex 7-2: first step of radical simplification chain
    let expr = r#"<math><mi>x</mi><mo>=</mo>
      <mfrac><mrow><msqrt><mrow><mn>36</mn><mo>-</mo><mn>4</mn><mo>(</mo><mn>9</mn><mo>)</mo><mo>+</mo><mn>3</mn><mo>(</mo><mn>12</mn><mo>)</mo></mrow></msqrt></mrow><mn>18</mn></mfrac>
    </math>"#;
    test_braille("UEB", expr, "⠰⠭⠀⠐⠶⠀⠰⠰⠷⠩⠼⠉⠋⠐⠤⠼⠙⠐⠣⠼⠊⠐⠜⠐⠖⠼⠉⠐⠣⠼⠁⠃⠐⠜⠬⠨⠌⠼⠁⠓⠾")?;
    return Ok(());

}

#[test]
fn bana2026_7_3() -> Result<()> {
    // BANA 2026 Ex 7-3
    let expr = r#"<math><mi>x</mi><mo>=</mo>
      <mfrac>
        <mrow><mn>54</mn><mo>+</mo><mn>7</mn><mo>-</mo><mn>111</mn><mo>×</mo><mn>2</mn><mo>+</mo><mn>8000</mn><mo>+</mo><mn>1000</mn></mrow>
        <mrow><mo>-</mo><mn>42</mn></mrow>
      </mfrac>
    </math>"#;
    test_braille("UEB", expr, "⠰⠭⠀⠐⠶⠀⠰⠷⠼⠑⠙⠐⠖⠼⠛⠐⠤⠼⠁⠁⠁⠐⠦⠼⠃⠐⠖⠼⠓⠚⠚⠚⠐⠖⠼⠁⠚⠚⠚⠨⠌⠐⠤⠼⠙⠃⠾")?;
    return Ok(());

}

#[test]
fn cap_1_6_1() -> Result<()> {
    let expr = "<math><mi>ABCD</mi></math>";
    test_braille("UEB", expr, "⠠⠠⠁⠃⠉⠙")?;
    return Ok(());

}

#[test]
fn cap_1_6_1_separate() -> Result<()> {
    let expr = "<math>
        <mi mathvariant='normal'>A</mi>
        <mi mathvariant='normal'>B</mi>
        <mi mathvariant='normal'>C</mi>
        <mi mathvariant='normal'>D</mi></math>";
    test_braille("UEB", expr, "⠠⠠⠁⠃⠉⠙")?;
    return Ok(());

}

#[test]
fn cap_1_6_2() -> Result<()> {
    let expr = "<math><mi>V</mi><mo>=</mo><mi>I</mi><mi>R</mi></math>";
    test_braille("UEB", expr, "⠰⠠⠧⠀⠐⠶⠀⠠⠠⠊⠗")?;
    return Ok(());

}

#[test]
fn cap_1_6_4() -> Result<()> {
    let expr = "<math><mi>A</mi><msup><mi>B</mi><mn>2</mn></msup></math>";
    test_braille("UEB", expr, "⠠⠠⠁⠃⠰⠔⠼⠃")?;
    return Ok(());

}

#[test]
fn grade1_1_7_1() -> Result<()> {
    let expr = "<math><mn>3</mn><mo>-</mo><mn>2</mn>
                            <mfrac bevelled='true'><mn>1</mn><mn>2</mn></mfrac ><mo>=</mo></math>";
    // removed the spaces around the '-' from the braille -- they typically wouldn't be used
    test_braille("UEB", expr, "⠼⠉⠐⠤⠼⠃⠼⠁⠌⠃⠀⠐⠶")?;
    return Ok(());

}

#[test]
fn grade1_1_7_2() -> Result<()> {
    let expr = "<math><mi>y</mi><mo>=</mo><mi>x</mi><mo>+</mo><mn>4</mn><mi>c</mi></math>";
    test_braille("UEB", expr, "⠰⠽⠀⠐⠶⠀⠭⠐⠖⠼⠙⠰⠉")?;
    return Ok(());

}

#[test]
fn grade1_1_7_3_1() -> Result<()> {
    let expr = "<math>
        <mn>3</mn><mi>x</mi><mo>−</mo><mn>4</mn><mi>y</mi><mo>+</mo>
        <msup><mi>y</mi><mn>2</mn></msup>
        <mo>=</mo>
        <msup><mi>x</mi><mn>2</mn></msup>
    </math>";
    // GTM says it can be either "⠼⠉⠭⠐⠤⠼⠙⠽⠐⠖⠽⠔⠼⠃⠀⠐⠶⠀⠭⠰⠔⠼⠃" or "⠰⠰⠰⠼⠉⠭⠐⠤⠼⠙⠽⠐⠖⠽⠔⠼⠃⠀⠐⠶⠀⠭⠔⠼⠃⠰⠄"
    test_braille("UEB", expr, "⠼⠉⠭⠐⠤⠼⠙⠽⠐⠖⠽⠔⠼⠃⠀⠐⠶⠀⠭⠰⠔⠼⠃")?;
    return Ok(());

}

#[test]
fn grade1_1_7_3_2() -> Result<()> {
    let expr = "<math> <mfrac>
    <mrow><msup><mi>x</mi><mn>2</mn></msup><mo>+</mo><mn>2</mn><mi>x</mi></mrow>
    <mrow><mn>1</mn><mo>+</mo><msup><mi>x</mi><mn>2</mn></msup></mrow>
    </mfrac><mo>=</mo><mn>1</mn>
    </math>";
    test_braille("UEB", expr, "⠰⠰⠷⠭⠔⠼⠃⠐⠖⠼⠃⠭⠨⠌⠼⠁⠐⠖⠭⠔⠼⠃⠾⠀⠐⠶⠀⠼⠁")?;
    return Ok(());

}

#[test]
fn grade1_1_7_4() -> Result<()> {
    let expr = "<math><msqrt>
            <mo>(</mo><mi>y</mi><mo>−</mo><msup><mi>x</mi><mn>2</mn></msup><mo>)</mo>
        </msqrt></math>";
    test_braille("UEB", expr, "⠰⠰⠩⠐⠣⠽⠐⠤⠭⠔⠼⠃⠐⠜⠬")?;
    return Ok(());

}

// Additional examples from GTM 1.7 (July 2025):
// https://iceb.org/wp-content/uploads/2026/02/GTM-1.7-Grade-1-Indicators-Approved.pdf
// Ordinary surrounding text is omitted; braille is the preferred math from the spec.

#[test]
fn grade1_1_7_3_a_1() -> Result<()> {
    let expr = "<math><msup><mi>x</mi><mn>2</mn></msup></math>";
    test_braille("UEB", expr, "⠭⠰⠔⠼⠃")?;
    return Ok(());

}

#[test]
fn grade1_1_7_3_a_2() -> Result<()> {
    let expr = "<math><mi>x</mi><mo>→</mo><mi>∞</mi></math>";
    test_braille("UEB", expr, "⠰⠭⠀⠰⠳⠕⠀⠼⠿")?;
    return Ok(());

}

#[test]
fn grade1_1_7_3_a_3() -> Result<()> {
    let expr = "<math><mfrac><mi>x</mi><mi>y</mi></mfrac></math>";
    test_braille("UEB", expr, "⠰⠰⠷⠭⠨⠌⠽⠾")?;
    return Ok(());

}

#[test]
fn grade1_1_7_3_b_1() -> Result<()> {
    let expr = "<math><msub><mi>x</mi><mi>a</mi></msub><mo>=</mo><msub><mi>x</mi><mi>b</mi></msub>
                <mo>=</mo><msub><mi>x</mi><mi>c</mi></msub></math>";
    test_braille("UEB", expr, "⠰⠰⠰⠭⠢⠁⠀⠐⠶⠀⠭⠢⠃⠀⠐⠶⠀⠭⠢⠉⠰⠄")?;
    return Ok(());

}

#[test]
fn grade1_1_7_3_b_2() -> Result<()> {
    let expr = "<math><mi>y</mi><mo>=</mo><msup><mi>x</mi><mn>2</mn></msup><mo>-</mo><mn>4</mn><mo>;</mo>
                <mo>&#xA0;</mo><mi>y</mi><mo>=</mo><msup><mi>x</mi><mn>2</mn></msup><mo>-</mo><mn>2</mn><mi>x</mi><mo>;</mo>
                <mo>&#xA0;</mo><mi>y</mi><mo>=</mo><mi>x</mi><mo>-</mo><msup><mi>x</mi><mn>2</mn></msup><mo>.</mo></math>";
    test_braille("UEB", expr, "⠰⠰⠰⠽⠀⠐⠶⠀⠭⠔⠼⠃⠐⠤⠼⠙⠆⠀⠽⠀⠐⠶⠀⠭⠔⠼⠃⠐⠤⠼⠃⠭⠆⠀⠽⠀⠐⠶⠀⠭⠐⠤⠭⠔⠼⠃⠲⠰⠄")?;
    return Ok(());

}

#[test]
fn grade1_1_7_4_1() -> Result<()> {
    let expr = "<math><msup><mi>x</mi><mn>2</mn></msup><mo>-</mo><mi>x</mi><mo>-</mo><mn>2</mn>
                <mo>=</mo><mn>0</mn></math>";
    test_braille("UEB", expr, "⠭⠰⠔⠼⠃⠐⠤⠭⠐⠤⠼⠃⠀⠐⠶⠀⠼⠚")?;
    return Ok(());

}

#[test]
fn grade1_1_7_4_2() -> Result<()> {
    let expr = "<math><msup><mi>x</mi><mn>2</mn></msup><mo>-</mo><mn>4</mn><mi>x</mi><mo>-</mo><mn>3</mn>
                <mo>=</mo><mn>0</mn></math>";
    test_braille("UEB", expr, "⠭⠰⠔⠼⠃⠐⠤⠼⠙⠭⠐⠤⠼⠉⠀⠐⠶⠀⠼⠚")?;
    return Ok(());

}

#[test]
fn grade1_1_7_4_3() -> Result<()> {
    let expr = "<math><msup><mi>x</mi><mn>2</mn></msup><mo>-</mo><mn>1</mn><mo>=</mo><mn>0</mn></math>";
    test_braille("UEB", expr, "⠭⠰⠔⠼⠃⠐⠤⠼⠁⠀⠐⠶⠀⠼⠚")?;
    return Ok(());

}

#[test]
fn grade1_1_7_5_a() -> Result<()> {
    let expr = "<math><mtext>speed</mtext><mo>=</mo>
                <mfrac><mtext>distance</mtext><mtext>time</mtext></mfrac></math>";
    // GTM 1.7.5(a) (2025): two grade 1 symbol indicators so the words stay contracted.
    // After the fraction open, "dis" is not a word start (RUEB 10.6.2), so st+ance: ⠙⠊⠌⠨⠑.
    test_braille("UEB", expr, "⠎⠏⠑⠫⠀⠐⠶⠀⠰⠷⠙⠊⠌⠨⠑⠨⠌⠐⠞⠰⠾")?;
    return Ok(());

}

#[test]
fn grade1_1_7_5_b() -> Result<()> {
    let expr = "<math><msub><mtext>luminosity</mtext><mtext>sun</mtext></msub></math>";
    test_braille("UEB", expr, "⠰⠰⠇⠥⠍⠊⠝⠕⠎⠊⠞⠽⠢⠣⠎⠥⠝⠜")?;
    return Ok(());

}

#[test]
fn grade1_1_7_5_c() -> Result<()> {
    let expr = "<math><mtext>speed</mtext><mo>=</mo>
                <mfrac><mtext>distance</mtext><mtext>time</mtext></mfrac><mo>=</mo>
                <mfrac><mrow><mn>30,000</mn><mo>&#xA0;</mo><mi class='MathML-unit'>m</mi></mrow>
                       <mrow><mn>60</mn><mo>&#xA0;</mo><mi class='MathML-unit'>s</mi></mrow></mfrac><mo>=</mo>
                <mn>500</mn><mo>&#xA0;</mo><mi class='MathML-unit'>m</mi><mo>/</mo><mi class='MathML-unit'>s</mi></math>";
    test_braille("UEB", expr, "⠎⠏⠑⠫⠀⠐⠶⠀⠰⠰⠰⠷⠙⠊⠎⠞⠁⠝⠉⠑⠨⠌⠞⠊⠍⠑⠾⠀⠐⠶⠀⠷⠼⠉⠚⠂⠚⠚⠚⠀⠍⠨⠌⠼⠋⠚⠀⠎⠾⠀⠐⠶⠀⠼⠑⠚⠚⠀⠍⠸⠌⠎⠰⠄")?;
    return Ok(());

}

#[test]
fn grade1_1_7_5_c_unit() -> Result<()> {
    // Same as grade1_1_7_5_c; unit marked via data-intent-property
    let expr = "<math><mtext>speed</mtext><mo>=</mo>
                <mfrac><mtext>distance</mtext><mtext>time</mtext></mfrac><mo>=</mo>
                <mfrac><mrow><mn>30,000</mn><mo>&#xA0;</mo><mi data-intent-property=':unit:'>m</mi></mrow>
                       <mrow><mn>60</mn><mo>&#xA0;</mo><mi data-intent-property=':unit:'>s</mi></mrow></mfrac><mo>=</mo>
                <mn>500</mn><mo>&#xA0;</mo><mi data-intent-property=':unit:'>m</mi><mo>/</mo><mi data-intent-property=':unit:'>s</mi></math>";
    test_braille("UEB", expr, "⠎⠏⠑⠫⠀⠐⠶⠀⠰⠰⠰⠷⠙⠊⠎⠞⠁⠝⠉⠑⠨⠌⠞⠊⠍⠑⠾⠀⠐⠶⠀⠷⠼⠉⠚⠂⠚⠚⠚⠀⠍⠨⠌⠼⠋⠚⠀⠎⠾⠀⠐⠶⠀⠼⠑⠚⠚⠀⠍⠸⠌⠎⠰⠄")?;
    return Ok(());

}

#[test]
fn grade1_1_7_9_1() -> Result<()> {
    let expr = "<math><mi>y</mi><mo>=</mo><mi>x</mi></math>";
    test_braille("UEB", expr, "⠰⠽⠀⠐⠶⠀⠰⠭")?;
    return Ok(());

}

#[test]
fn grade1_1_7_9_2() -> Result<()> {
    let expr = "<math><msqrt><mn>25</mn></msqrt><mo>=</mo><mn>5</mn></math>";
    test_braille("UEB", expr, "⠰⠩⠼⠃⠑⠬⠀⠐⠶⠀⠼⠑")?;
    return Ok(());

}

#[test]
fn grade1_1_7_9_3() -> Result<()> {
    let expr = "<math><msqrt><mi>x</mi></msqrt><mo>=</mo><mn>7</mn></math>";
    test_braille("UEB", expr, "⠰⠰⠩⠭⠬⠀⠐⠶⠀⠼⠛")?;
    return Ok(());

}

#[test]
fn grade1_1_7_9_4() -> Result<()> {
    let expr = "<math><mi class='MathML-unit'>m</mi><msup><mi class='MathML-unit'>s</mi>
                <mrow><mo>-</mo><mn>1</mn></mrow></msup></math>";
    test_braille("UEB", expr, "⠰⠰⠍⠎⠔⠣⠐⠤⠼⠁⠜")?;
    return Ok(());

}

#[test]
fn grade1_1_7_9_4_unit() -> Result<()> {
    // Same as grade1_1_7_9_4; unit marked via data-intent-property
    let expr = "<math><mi data-intent-property=':unit:'>m</mi><msup><mi data-intent-property=':unit:'>s</mi>
                <mrow><mo>-</mo><mn>1</mn></mrow></msup></math>";
    test_braille("UEB", expr, "⠰⠰⠍⠎⠔⠣⠐⠤⠼⠁⠜")?;
    return Ok(());

}

#[test]
fn grade1_1_7_9_5() -> Result<()> {
    let expr = "<math><mi>y</mi><mo>=</mo><mfrac><mi>x</mi><mn>2</mn></mfrac></math>";
    // GTM 1.7.9 (July 2025): one symbol indicator; numeric mode covers the closer.
    test_braille("UEB", expr, "⠰⠽⠀⠐⠶⠀⠰⠷⠭⠨⠌⠼⠃⠾")?;
    return Ok(());

}

#[test]
fn grade1_1_7_9_6() -> Result<()> {
    let expr = "<math><mi>y</mi><mo>=</mo><mfrac><msup><mi>x</mi><mn>2</mn></msup><mn>2</mn></mfrac></math>";
    test_braille("UEB", expr, "⠰⠽⠀⠐⠶⠀⠰⠰⠷⠭⠔⠼⠃⠨⠌⠼⠃⠾")?;
    return Ok(());

}

#[test]
fn grade1_1_7_9_7() -> Result<()> {
    let expr = "<math><mfrac><mi>x</mi><mi>y</mi></mfrac><mo>=</mo><mfrac><mi>c</mi><mi>d</mi></mfrac></math>";
    test_braille("UEB", expr, "⠰⠰⠷⠭⠨⠌⠽⠾⠀⠐⠶⠀⠰⠰⠷⠉⠨⠌⠙⠾")?;
    return Ok(());

}

#[test]
fn grade1_1_7_9_8() -> Result<()> {
    let expr = "<math><mo>(</mo><mi>x</mi><mo>,</mo><mi>y</mi><mo>)</mo></math>";
    test_braille("UEB", expr, "⠐⠣⠰⠭⠂⠀⠰⠽⠐⠜")?;
    return Ok(());

}

#[test]
fn grade1_1_7_9_9() -> Result<()> {
    let expr = "<math><mo>(</mo><msub><mi>x</mi><mi>i</mi></msub><mo>,</mo><msub><mi>y</mi><mi>i</mi></msub><mo>)</mo></math>";
    test_braille("UEB", expr, "⠐⠣⠭⠰⠢⠊⠂⠀⠽⠰⠢⠊⠐⠜")?;
    return Ok(());

}

#[test]
fn grade1_1_7_9_10() -> Result<()> {
    let expr = "<math><mo>(</mo><msubsup><mi>x</mi><mi>i</mi><mn>2</mn></msubsup><mo>,</mo>
                <msubsup><mi>y</mi><mi>i</mi><mn>2</mn></msubsup><mo>)</mo></math>";
    test_braille("UEB", expr, "⠰⠰⠐⠣⠭⠢⠊⠔⠼⠃⠂⠀⠰⠰⠽⠢⠊⠔⠼⠃⠐⠜")?;
    return Ok(());

}

#[test]
fn grade1_1_7_9_11() -> Result<()> {
    let expr = "<math><mover><mi mathvariant='normal'>B</mi><mo>¯</mo></mover></math>";
    test_braille("UEB", expr, "⠠⠃⠰⠱")?;
    return Ok(());

}

#[test]
fn grade1_1_7_9_12() -> Result<()> {
    let expr = "<math><mover><mi mathvariant='normal'>M</mi><mo>↔</mo></mover></math>";
    test_braille("UEB", expr, "⠠⠍⠨⠔⠰⠳⠺⠗⠕")?;
    return Ok(());

}

#[test]
fn grade1_1_7_9_13() -> Result<()> {
    let expr = "<math><mover><mrow><mi mathvariant='normal'>A</mi><mi mathvariant='normal'>Z</mi></mrow>
                <mo>↔</mo></mover></math>";
    test_braille("UEB", expr, "⠰⠰⠣⠠⠠⠁⠵⠜⠨⠔⠳⠺⠗⠕")?;
    return Ok(());

}

#[test]
fn grade1_1_7_9_14() -> Result<()> {
    let expr = "<math><msubsup><mrow/><mn>13</mn><mn>27</mn></msubsup><mi>Al</mi></math>";
    test_braille("UEB", expr, "⠰⠢⠼⠁⠉⠔⠼⠃⠛⠠⠁⠇")?;
    return Ok(());

}

#[test]
fn grade1_1_7_9_15() -> Result<()> {
    let expr = "<math><msup><mi mathvariant='normal'>O</mi><mrow><mn>2</mn><mo>-</mo></mrow></msup></math>";
    test_braille("UEB", expr, "⠰⠰⠠⠕⠔⠣⠼⠃⠐⠤⠜")?;
    return Ok(());

}

#[test]
fn grade1_1_7_9_16() -> Result<()> {
    let expr = "<math><mn>2</mn><msub><mi mathvariant='normal'>H</mi><mn>2</mn></msub>
                <mo>(</mo><mi>g</mi><mo>)</mo><mo>+</mo>
                <msub><mi mathvariant='normal'>O</mi><mn>2</mn></msub>
                <mo>(</mo><mi>g</mi><mo>)</mo><mo>→</mo>
                <mn>2</mn><msub><mi mathvariant='normal'>H</mi><mn>2</mn></msub>
                <mi mathvariant='normal'>O</mi><mo>(</mo><mi>l</mi><mo>)</mo></math>";
    test_braille("UEB", expr, "⠼⠃⠠⠓⠢⠼⠃⠐⠣⠛⠐⠜⠐⠖⠠⠕⠢⠼⠃⠐⠣⠛⠐⠜⠀⠰⠳⠕⠀⠼⠃⠠⠓⠢⠼⠃⠠⠕⠐⠣⠇⠐⠜")?;
    return Ok(());

}

#[test]
fn grade1_1_7_9_17() -> Result<()> {
    let expr = "<math><mi>y</mi><mo>=</mo><mi>m</mi><mi>x</mi><mo>+</mo><mi>c</mi></math>";
    test_braille("UEB", expr, "⠰⠽⠀⠐⠶⠀⠍⠭⠐⠖⠉")?;
    return Ok(());

}

#[test]
fn grade1_1_7_9_18() -> Result<()> {
    let expr = "<math><mi>A</mi><mo>=</mo><mi>π</mi><msup><mi>r</mi><mn>2</mn></msup></math>";
    test_braille("UEB", expr, "⠠⠁⠀⠐⠶⠀⠨⠏⠗⠰⠔⠼⠃")?;
    return Ok(());

}

#[test]
fn grade1_1_7_9_19() -> Result<()> {
    let expr = "<math><mi>E</mi><mo>=</mo><mi>m</mi><msup><mi>c</mi><mn>2</mn></msup></math>";
    test_braille("UEB", expr, "⠰⠠⠑⠀⠐⠶⠀⠍⠉⠰⠔⠼⠃")?;
    return Ok(());

}

#[test]
fn grade1_1_7_9_20() -> Result<()> {
    let expr = "<math><msup><mi>x</mi><mn>2</mn></msup><mo>-</mo><mn>2</mn><mi>x</mi><mo>+</mo><mn>1</mn>
                <mo>=</mo><msup><mrow><mo>(</mo><mi>x</mi><mo>-</mo><mn>1</mn><mo>)</mo></mrow><mn>2</mn></msup></math>";
    test_braille("UEB", expr, "⠭⠰⠔⠼⠃⠐⠤⠼⠃⠭⠐⠖⠼⠁⠀⠐⠶⠀⠐⠣⠭⠐⠤⠼⠁⠐⠜⠔⠼⠃")?;
    return Ok(());

}

#[test]
fn grade1_1_7_9_21() -> Result<()> {
    let expr = "<math><msub><mi>x</mi><mi>n</mi></msub><mo>=</mo><mn>1</mn><mo>+</mo>
                <mfrac><mrow><mo>-</mo><msup><mn>1</mn><mi>n</mi></msup></mrow><mi>n</mi></mfrac></math>";
    test_braille("UEB", expr, "⠭⠰⠢⠝⠀⠐⠶⠀⠼⠁⠐⠖⠷⠐⠤⠼⠁⠔⠝⠨⠌⠝⠾")?;
    return Ok(());

}

#[test]
fn grade1_1_7_9_22() -> Result<()> {
    let expr = "<math><msub><mi>x</mi><mi>n</mi></msub><mo>=</mo>
                <mfrac><msup><mn>2</mn><mi>n</mi></msup><msup><mi>n</mi><mn>2</mn></msup></mfrac></math>";
    test_braille("UEB", expr, "⠭⠰⠢⠝⠀⠐⠶⠀⠰⠷⠼⠃⠔⠝⠨⠌⠝⠔⠼⠃⠾")?;
    return Ok(());

}

#[test]
fn grade1_1_7_9_23() -> Result<()> {
    let expr = "<math><mi>x</mi><mo>=</mo>
        <mfrac>
            <mrow><mo>−</mo><mi>b</mi><mo>±</mo>
                <msqrt><msup><mi>b</mi><mn>2</mn></msup><mo>−</mo><mn>4</mn><mi>a</mi><mi>c</mi></msqrt>
            </mrow>
            <mrow><mn>2</mn><mi>a</mi></mrow>
        </mfrac>
    </math>";
    test_braille("UEB", expr, "⠰⠭⠀⠐⠶⠀⠰⠰⠷⠐⠤⠃⠸⠖⠩⠃⠔⠼⠃⠐⠤⠼⠙⠰⠁⠉⠬⠨⠌⠼⠃⠰⠁⠾")?;
    return Ok(());

}

#[test]
fn grade1_1_7_9_24() -> Result<()> {
    let expr = "<math><mi>x</mi><msqrt><mi>sin</mi><mi>θ</mi></msqrt></math>";
    test_braille("UEB", expr, "⠰⠰⠭⠩⠎⠊⠝⠨⠹⠬")?;
    return Ok(());

}

#[test]
fn grade1_1_7_9_25() -> Result<()> {
    let expr = "<math><msup><mtext>Version</mtext><mrow><mn>1</mn><mi>b</mi></mrow></msup></math>";
    test_braille("UEB", expr, "⠰⠰⠠⠧⠑⠗⠎⠊⠕⠝⠔⠣⠼⠁⠰⠃⠜")?;
    return Ok(());

}

#[test]
fn grade1_1_7_9_26() -> Result<()> {
    let expr = "<math><mi>W</mi><mo>=</mo><mi>F</mi><mi>s</mi></math>";
    test_braille("UEB", expr, "⠰⠠⠺⠀⠐⠶⠀⠠⠋⠎")?;
    return Ok(());

}

#[test]
fn grade1_1_7_9_27() -> Result<()> {
    let expr = "<math><mtext>work</mtext><mo>=</mo><mtext>force</mtext><mo>×</mo><mtext>distance</mtext></math>";
    test_braille("UEB", expr, "⠐⠺⠀⠐⠶⠀⠿⠉⠑⠐⠦⠙⠊⠌⠨⠑")?;
    return Ok(());

}

#[test]
fn grade1_1_7_9_28() -> Result<()> {
    let expr = "<math><mo>∴</mo><mtext>force</mtext><mo>=</mo>
                <mfrac><mtext>work</mtext><mtext>distance</mtext></mfrac></math>";
    test_braille("UEB", expr, "⠰⠠⠡⠀⠿⠉⠑⠀⠐⠶⠀⠰⠷⠐⠺⠨⠌⠙⠊⠌⠨⠑⠰⠾")?;
    return Ok(());

}

#[test]
fn grade1_1_7_9_29() -> Result<()> {
    let expr = "<math><mtext>acceleration</mtext><mo>=</mo>
                <mfrac><mrow><mi mathvariant='normal'>Δ</mi><mtext>speed</mtext></mrow>
                       <mrow><mi mathvariant='normal'>Δ</mi><mtext>time</mtext></mrow></mfrac></math>";
    test_braille("UEB", expr, "⠁⠒⠑⠇⠻⠁⠰⠝⠀⠐⠶⠀⠰⠷⠠⠨⠙⠎⠏⠑⠫⠨⠌⠠⠨⠙⠐⠞⠰⠾")?;
    return Ok(());

}

#[test]
fn grade1_1_7_9_30() -> Result<()> {
    let expr = "<math><mtext>frequency</mtext><mo>=</mo><mfrac><mn>1</mn><mtext>time</mtext></mfrac></math>";
    test_braille("UEB", expr, "⠋⠗⠑⠟⠥⠢⠉⠽⠀⠐⠶⠀⠰⠷⠼⠁⠨⠌⠞⠊⠍⠑⠾")?;
    return Ok(());

}

#[test]
fn grade1_1_7_9_31() -> Result<()> {
    let expr = "<math><mtext>frequency</mtext><mo>=</mo><mn>1</mn><mo>/</mo><mtext>time</mtext></math>";
    test_braille("UEB", expr, "⠋⠗⠑⠟⠥⠢⠉⠽⠀⠐⠶⠀⠼⠁⠸⠌⠞⠊⠍⠑")?;
    return Ok(());

}

#[test]
fn grade1_1_7_9_32() -> Result<()> {
    let expr = "<math><msub><mi>V</mi><mtext>cone</mtext></msub><mo>=</mo>
                <mfrac><mn>1</mn><mn>3</mn></mfrac><mi>π</mi>
                <msup><mi>r</mi><mn>2</mn></msup><mi>h</mi></math>";
    test_braille("UEB", expr, "⠰⠰⠠⠧⠢⠣⠉⠕⠝⠑⠜⠀⠐⠶⠀⠼⠁⠌⠉⠨⠏⠗⠔⠼⠃⠰⠓")?;
    return Ok(());

}

#[test]
fn grade1_1_7_9_33() -> Result<()> {
    let expr = "<math><mi>V</mi><mo>=</mo><mo>∫</mo><mi>π</mi>
                <msup><mi>y</mi><mn>2</mn></msup><mi>d</mi><mi>x</mi></math>";
    test_braille("UEB", expr, "⠰⠠⠧⠀⠐⠶⠀⠰⠰⠮⠨⠏⠽⠔⠼⠃⠰⠙⠭")?;
    return Ok(());

}

#[test]
fn grade1_1_7_9_34() -> Result<()> {
    let expr = "<math><mtext>period</mtext><mo>=</mo><mn>2</mn><mi>π</mi>
                <msqrt><mfrac>
                    <mrow><mi>l</mi><mi>cos</mi><mi>α</mi></mrow>
                    <mi>g</mi>
                </mfrac></msqrt></math>";
    test_braille("UEB", expr, "⠏⠻⠊⠕⠙⠀⠐⠶⠀⠼⠃⠨⠏⠩⠷⠇⠀⠰⠰⠉⠕⠎⠨⠁⠨⠌⠛⠾⠬")?;
    return Ok(());

}

#[test]
fn grade1_1_7_9_35() -> Result<()> {
    let expr = "<math><mtext>Assume</mtext><mo>&#xA0;</mo><mi>g</mi><mo>=</mo><mn>9.81</mn><mo>&#xA0;</mo>
                <mi class='MathML-unit'>m</mi><msup><mi class='MathML-unit'>s</mi>
                <mrow><mo>-</mo><mn>2</mn></mrow></msup></math>";
    test_braille("UEB", expr, "⠠⠁⠎⠎⠥⠍⠑⠀⠰⠛⠀⠐⠶⠀⠼⠊⠲⠓⠁⠀⠰⠰⠍⠎⠔⠣⠐⠤⠼⠃⠜")?;
    return Ok(());

}

#[test]
fn grade1_1_7_9_35_unit() -> Result<()> {
    // Same as grade1_1_7_9_35; unit marked via data-intent-property
    let expr = "<math><mtext>Assume</mtext><mo>&#xA0;</mo><mi>g</mi><mo>=</mo><mn>9.81</mn><mo>&#xA0;</mo>
                <mi data-intent-property=':unit:'>m</mi><msup><mi data-intent-property=':unit:'>s</mi>
                <mrow><mo>-</mo><mn>2</mn></mrow></msup></math>";
    test_braille("UEB", expr, "⠠⠁⠎⠎⠥⠍⠑⠀⠰⠛⠀⠐⠶⠀⠼⠊⠲⠓⠁⠀⠰⠰⠍⠎⠔⠣⠐⠤⠼⠃⠜")?;
    return Ok(());

}

#[test]
fn grade1_1_7_9_36() -> Result<()> {
    let expr = "<math><msub><mi>v</mi><mn>1</mn></msub><mo>=</mo><mi>x</mi><mo>&#xA0;</mo>
                <mi class='MathML-unit'>m</mi><msup><mi class='MathML-unit'>s</mi>
                <mrow><mo>-</mo><mn>1</mn></mrow></msup></math>";
    test_braille("UEB", expr, "⠰⠰⠰⠧⠢⠼⠁⠀⠐⠶⠀⠭⠀⠍⠎⠔⠣⠐⠤⠼⠁⠜⠰⠄")?;
    return Ok(());

}

#[test]
fn grade1_1_7_9_36_unit() -> Result<()> {
    // Same as grade1_1_7_9_36; unit marked via data-intent-property
    let expr = "<math><msub><mi>v</mi><mn>1</mn></msub><mo>=</mo><mi>x</mi><mo>&#xA0;</mo>
                <mi data-intent-property=':unit:'>m</mi><msup><mi data-intent-property=':unit:'>s</mi>
                <mrow><mo>-</mo><mn>1</mn></mrow></msup></math>";
    test_braille("UEB", expr, "⠰⠰⠰⠧⠢⠼⠁⠀⠐⠶⠀⠭⠀⠍⠎⠔⠣⠐⠤⠼⠁⠜⠰⠄")?;
    return Ok(());

}

#[test]
fn grade1_1_7_9_37() -> Result<()> {
    let expr = "<math><mi>P</mi><mo>=</mo>
                <mfenced open='{' close='}'><mrow><mi>t</mi><mo>,</mo><mi>u</mi><mo>,</mo><mi>v</mi></mrow></mfenced></math>";
    test_braille("UEB", expr, "⠰⠰⠰⠠⠏⠀⠐⠶⠀⠸⠣⠞⠂⠀⠥⠂⠀⠧⠸⠜⠰⠄")?;
    return Ok(());

}

#[test]
fn grade1_1_7_9_38() -> Result<()> {
    let expr = "<math><msub><mi>x</mi><mi>a</mi></msub><mo>=</mo><msub><mi>x</mi><mi>b</mi></msub>
                <mo>=</mo><msub><mi>x</mi><mi>c</mi></msub><mo>=</mo><msub><mi>x</mi><mi>d</mi></msub></math>";
    test_braille("UEB", expr, "⠰⠰⠰⠭⠢⠁⠀⠐⠶⠀⠭⠢⠃⠀⠐⠶⠀⠭⠢⠉⠀⠐⠶⠀⠭⠢⠙⠰⠄")?;
    return Ok(());

}

#[test]
fn grade1_1_7_9_39() -> Result<()> {
    let expr = "<math><msup><mi>x</mi><mn>2</mn></msup><mo>×</mo><msup><mi>x</mi><mn>3</mn></msup>
                <mo>=</mo><msup><mi>x</mi><mrow><mn>2</mn><mo>+</mo><mn>3</mn></mrow></msup>
                <mo>=</mo><msup><mi>x</mi><mn>5</mn></msup></math>";
    test_braille("UEB", expr, "⠰⠰⠰⠭⠔⠼⠃⠐⠦⠭⠔⠼⠉⠀⠐⠶⠀⠭⠔⠣⠼⠃⠐⠖⠼⠉⠜⠀⠐⠶⠀⠭⠔⠼⠑⠰⠄")?;
    return Ok(());

}

#[test]
fn grade1_1_7_9_40() -> Result<()> {
    let expr = "<math><msup><mi>a</mi><mi>m</mi></msup><mo>×</mo><msup><mi>a</mi><mi>n</mi></msup>
                <mo>=</mo><msup><mi>a</mi><mrow><mi>m</mi><mo>+</mo><mi>n</mi></mrow></msup></math>";
    test_braille("UEB", expr, "⠰⠰⠁⠔⠍⠐⠦⠁⠔⠝⠀⠐⠶⠀⠰⠰⠁⠔⠣⠍⠐⠖⠝⠜")?;
    return Ok(());

}

#[test]
fn grade1_1_7_9_41() -> Result<()> {
    let expr = "<math><msup><mrow><mo>(</mo><msup><mi>a</mi><mi>m</mi></msup><mo>)</mo></mrow><mi>n</mi></msup>
                <mo>=</mo><msup><mi>a</mi><mrow><mi>m</mi><mi>n</mi></mrow></msup></math>";
    test_braille("UEB", expr, "⠰⠰⠐⠣⠁⠔⠍⠐⠜⠔⠝⠀⠐⠶⠀⠰⠰⠁⠔⠣⠍⠝⠜")?;
    return Ok(());

}

#[test]
fn grade1_1_7_9_42() -> Result<()> {
    let expr = "<math><mfrac><msup><mi>a</mi><mi>m</mi></msup><msup><mi>a</mi><mi>n</mi></msup></mfrac>
                <mo>=</mo><msup><mi>a</mi><mrow><mi>m</mi><mo>-</mo><mi>n</mi></mrow></msup></math>";
    test_braille("UEB", expr, "⠰⠰⠷⠁⠔⠍⠨⠌⠁⠔⠝⠾⠀⠐⠶⠀⠰⠰⠁⠔⠣⠍⠐⠤⠝⠜")?;
    return Ok(());

}

#[test]
fn grade1_1_7_9_43() -> Result<()> {
    let expr = "<math><msup><mi>a</mi><mrow><mo>-</mo><mi>m</mi></mrow></msup>
                <mo>=</mo><mfrac><mn>1</mn><msup><mi>a</mi><mi>m</mi></msup></mfrac></math>";
    test_braille("UEB", expr, "⠰⠰⠁⠔⠣⠐⠤⠍⠜⠀⠐⠶⠀⠰⠷⠼⠁⠨⠌⠁⠔⠍⠾")?;
    return Ok(());

}

#[test]
fn grade1_1_7_9_44() -> Result<()> {
    let expr = "<math><msup><mi>a</mi><mn>0</mn></msup><mo>=</mo><mn>1</mn></math>";
    test_braille("UEB", expr, "⠁⠰⠔⠼⠚⠀⠐⠶⠀⠼⠁")?;
    return Ok(());

}

#[test]
fn grade1_1_7_9_45() -> Result<()> {
    let expr = "<math><msup><mi>a</mi><mfrac><mn>1</mn><mi>n</mi></mfrac></msup>
                <mo>=</mo><mroot><mi>a</mi><mi>n</mi></mroot></math>";
    test_braille("UEB", expr, "⠰⠰⠁⠔⠷⠼⠁⠨⠌⠝⠾⠀⠐⠶⠀⠰⠰⠩⠔⠝⠁⠬")?;
    return Ok(());

}

#[test]
fn grade1_1_7_9_46() -> Result<()> {
    let expr = "<math><mi>B</mi><mo>→</mo><mi>C</mi></math>";
    test_braille("UEB", expr, "⠰⠰⠰⠠⠃⠀⠳⠕⠀⠠⠉⠰⠄")?;
    return Ok(());

}

#[test]
fn grade1_1_7_9_47() -> Result<()> {
    let expr = "<math><mi>x</mi><mo>+</mo><mi>y</mi><mo>=</mo><mn>5</mn></math>";
    test_braille("UEB", expr, "⠭⠐⠖⠽⠀⠐⠶⠀⠼⠑")?;
    return Ok(());

}

#[test]
fn grade1_1_7_9_48() -> Result<()> {
    let expr = "<math><mi mathvariant='normal'>C</mi><msub><mi mathvariant='normal'>H</mi><mn>4</mn></msub>
                <mo>+</mo><mn>4</mn><msub><mi>Cl</mi><mn>2</mn></msub>
                <mo>→</mo>
                <mi mathvariant='normal'>C</mi><msub><mi>Cl</mi><mn>4</mn></msub>
                <mo>+</mo><mn>4</mn><mi mathvariant='normal'>H</mi><mi>Cl</mi></math>";
    test_braille("UEB", expr, "⠰⠰⠰⠠⠉⠠⠓⠢⠼⠙⠐⠖⠼⠙⠠⠉⠇⠢⠼⠃⠀⠳⠕⠀⠠⠉⠠⠉⠇⠢⠼⠙⠐⠖⠼⠙⠠⠓⠠⠉⠇⠰⠄")?;
    return Ok(());

}

#[test]
fn grade1_1_7_9_49() -> Result<()> {
    let expr = "<math><msubsup><mrow/><mn>6</mn><mn>14</mn></msubsup><mi mathvariant='normal'>C</mi>
                <mo>→</mo>
                <msubsup><mrow/><mn>7</mn><mn>14</mn></msubsup><mi mathvariant='normal'>N</mi>
                <mo>+</mo>
                <msubsup><mrow/><mrow><mo>-</mo><mn>1</mn></mrow><mn>0</mn></msubsup><mi>β</mi></math>";
    test_braille("UEB", expr, "⠰⠰⠰⠢⠼⠋⠔⠼⠁⠙⠠⠉⠀⠳⠕⠀⠢⠼⠛⠔⠼⠁⠙⠠⠝⠐⠖⠢⠣⠐⠤⠼⠁⠜⠔⠼⠚⠨⠃⠰⠄")?;
    return Ok(());

}

#[test]
fn grade1_1_7_9_50() -> Result<()> {
    let expr = "<math><mtext>reliability</mtext><mo>=</mo>
                <mfrac><mtext>number of faults</mtext><mtext>total number of items</mtext></mfrac>
                <mo>=</mo><mi>p</mi></math>";
    test_braille("UEB", expr, "⠰⠰⠰⠗⠑⠇⠊⠁⠃⠊⠇⠊⠞⠽⠀⠐⠶⠀⠷⠝⠥⠍⠃⠑⠗⠀⠕⠋⠀⠋⠁⠥⠇⠞⠎⠨⠌⠞⠕⠞⠁⠇⠀⠝⠥⠍⠃⠑⠗⠀⠕⠋⠀⠊⠞⠑⠍⠎⠾⠀⠐⠶⠀⠏⠰⠄")?;
    return Ok(());

}

#[test]
fn number_2_1_2() -> Result<()> {
    let expr = "<math><mn>3,000</mn></math>";
    test_braille("UEB", expr, "⠼⠉⠂⠚⠚⠚")?;
    return Ok(());

}

#[test]
fn number_2_1_3() -> Result<()> {
    let expr = "<math><mn>5 000 000</mn></math>";
    test_braille("UEB", expr, "⠼⠑⠐⠚⠚⠚⠐⠚⠚⠚")?;
    return Ok(());

}

#[test]
fn number_2_2_1() -> Result<()> {
    let expr = "<math><mn>8.93</mn></math>";
    test_braille("UEB", expr, "⠼⠓⠲⠊⠉")?;
    return Ok(());

}

#[test]
fn number_2_2_2() -> Result<()> {
    let expr = "<math><mn>0.7</mn></math>";
    test_braille("UEB", expr, "⠼⠚⠲⠛")?;
    return Ok(());

}

#[test]
fn number_2_2_3() -> Result<()> {
    let expr = "<math><mn>.7</mn></math>";
    test_braille("UEB", expr, "⠼⠲⠛")?;
    return Ok(());

}

#[test]
fn time_2_4_1() -> Result<()> {
    let expr = "<math><mn>5</mn><mo>:</mo><mn>30</mn><mo>&#xA0;</mo><mtext>pm</mtext></math>";
    test_braille("UEB", expr, "⠼⠑⠒⠼⠉⠚⠀⠏⠍")?;
    return Ok(());

}

#[test]
fn time_2_4_1_mtext() -> Result<()> {
    // this was a bug when only mtext occurred
    let expr = "<math><mtext>5:30</mtext></math>";
    test_braille("UEB", expr, "⠼⠑⠒⠼⠉⠚")?;
    return Ok(());

}

#[test]
fn roman_numeral_2_6_1() -> Result<()> {
    let expr = " <math><mi mathvariant='normal'>I</mi><mo>,</mo>
        <mo>&#xA0;</mo><mi>II</mi>
        <mo>&#xA0;</mo><mtext>and</mtext><mo>&#xA0;</mo><mi mathvariant='normal'>V</mi></math>";
    test_braille("UEB", expr, "⠠⠊⠂⠀⠠⠠⠊⠊⠀⠯⠀⠰⠠⠧")?;
    return Ok(());

}

#[test]
fn roman_numeral_2_6_2() -> Result<()> {
    let expr = " <math><mi mathvariant='normal'>i</mi><mo>,</mo>
        <mo>&#xA0;</mo><mi>vi</mi>
        <mo>&#xA0;</mo><mtext>and</mtext><mo>&#xA0;</mo><mi mathvariant='normal'>x</mi></math>";
    test_braille("UEB", expr, "⠊⠂⠀⠧⠊⠀⠯⠀⠰⠭")?;
    return Ok(());

}

#[test]
fn roman_numeral_2_6_3() -> Result<()> {
    let expr = "<math><mn>CD</mn></math>";
    test_braille("UEB", expr, "⠰⠠⠠⠉⠙")?;
    return Ok(());

}

#[test]
fn bold_2_7_1() -> Result<()> {
    let expr = "<math><mn>67𝟖45</mn></math>";
    test_braille("UEB", expr, "⠼⠋⠛⠘⠆⠼⠓⠙⠑")?;
    return Ok(());

}

#[test]
fn bold_2_7_2() -> Result<()> {
    let expr = "<math><mn>67</mn><mn mathvariant='bold'>845</mn></math>";
    test_braille("UEB", expr, "⠼⠋⠛⠘⠂⠼⠓⠙⠑")?;
    return Ok(());

}

#[test]
fn bold_2_7_3() -> Result<()> {
    let expr = "<math><mn>67</mn><mn mathvariant='bold'>84</mn><mn>5</mn></math>";
    test_braille("UEB", expr, "⠼⠋⠛⠘⠂⠼⠓⠙⠘⠄⠼⠑")?;
    return Ok(());

}

#[test]
fn signs_2_10_2() -> Result<()> {
    let expr = "<math><mo>$</mo><mn>0.30</mn><mo>,</mo><mo>&#xA0;</mo>
                <mn>30</mn><mi mathvariant='normal'>c</mi><mo>&#xA0;</mo>
                <mtext>or</mtext><mo>&#xA0;</mo><mn>30</mn><mo>¢</mo></math>";
    test_braille("UEB", expr, "⠈⠎⠼⠚⠲⠉⠚⠂⠀⠼⠉⠚⠰⠉⠀⠕⠗⠀⠼⠉⠚⠈⠉")?;
    return Ok(());

}

#[test]
fn signs_2_10_5() -> Result<()> {
    let expr = "<math><mn>1</mn><mo>&#xA0;</mo><mi>ft</mi><mo>&#xA0;</mo><mn>6</mn><mo>&#xA0;</mo><mi>in</mi>
        <mo>&#xA0;</mo><mtext>or</mtext><mo>&#xA0;</mo>
        <mn>1</mn><mo>′</mo><mo>&#xA0;</mo><mn>6</mn><mo>′</mo><mo>′</mo></math>";
    test_braille("UEB", expr, "⠼⠁⠀⠋⠞⠀⠼⠋⠀⠔⠀⠕⠗⠀⠼⠁⠶⠀⠼⠋⠶⠶")?;
    return Ok(());

}

#[test]
fn signs_2_10_8() -> Result<()> {
    let expr = "<math><mn>0</mn><mo>°</mo><mi mathvariant='normal'>C</mi><mo>&#xA0;</mo><mtext>or</mtext>
        <mo>&#xA0;</mo><mn>32</mn><mo>°</mo><mi mathvariant='normal'>F</mi></math>";
    test_braille("UEB", expr, "⠼⠚⠘⠚⠠⠉⠀⠕⠗⠀⠼⠉⠃⠘⠚⠠⠋")?;
    return Ok(());

}

#[test]
fn signs_2_10_16() -> Result<()> {
    let expr = "<math><mn>1</mn><mo>&#xA0;</mo><mi mathvariant='normal'>Å</mi><mo>=</mo>
        <mfrac><mn>1</mn><mrow><mn>10</mn><mo>,</mo><mn>000</mn></mrow></mfrac><mo>&#xA0;</mo>
        <mi mathvariant='normal'>μ</mi></math>";
    test_braille("UEB", expr, "⠼⠁⠀⠠⠘⠫⠁⠀⠐⠶⠀⠼⠁⠌⠁⠚⠂⠚⠚⠚⠀⠨⠍")?;
    return Ok(());

}

#[test]
fn expr_3_1_1_spaces() -> Result<()> {
    let expr = "<math><mn>3</mn><mo>+</mo><mn>5</mn><mo>=</mo><mn>8</mn></math>";
    test_braille_prefs("UEB", vec![("UseSpacesAroundAllOperators", "true")], expr, "⠼⠉⠀⠐⠖⠀⠼⠑⠀⠐⠶⠀⠼⠓")?;
    return Ok(());

}

#[test]
fn expr_3_1_2_spaces() -> Result<()> {
    let expr = "<math><mn>8</mn><mo>-</mo><mn>5</mn><mo>=</mo><mn>3</mn></math>";
    test_braille_prefs("UEB", vec![("UseSpacesAroundAllOperators", "true")], expr, "⠼⠓⠀⠐⠤⠀⠼⠑⠀⠐⠶⠀⠼⠉")?;
    return Ok(());

}

#[test]
fn expr_3_1_1() -> Result<()> {
    let expr = "<math><mn>3</mn><mo>+</mo><mn>5</mn><mo>=</mo><mn>8</mn></math>";
    // correct not to use extra spacing
    test_braille("UEB", expr, "⠼⠉⠐⠖⠼⠑⠀⠐⠶⠀⠼⠓")?;
    return Ok(());

}

#[test]
fn expr_3_1_2() -> Result<()> {
    let expr = "<math><mn>8</mn><mo>-</mo><mn>5</mn><mo>=</mo><mn>3</mn></math>";
    // correct not to use extra spacing
    test_braille("UEB", expr, "⠼⠓⠐⠤⠼⠑⠀⠐⠶⠀⠼⠉")?;
    return Ok(());

}

#[test]
fn expr_3_1_3() -> Result<()> {
    let expr = "<math><mn>3</mn><mo>×</mo><mn>5</mn><mo>=</mo><mn>5</mn><mo>×</mo><mn>3</mn><mo>=</mo><mn>15</mn></math>";
    test_braille("UEB", expr, "⠼⠉⠐⠦⠼⠑⠀⠐⠶⠀⠼⠑⠐⠦⠼⠉⠀⠐⠶⠀⠼⠁⠑")?;
    return Ok(());

}

#[test]
fn expr_3_1_6() -> Result<()> {
    // example includes spaces, so does the MathML (from WIRIS)
    let expr = "<math><mn>5</mn><mo>.</mo><mn>72</mn><mo>&#xA0;</mo><mtext>m</mtext><mo>÷</mo><mn>10</mn><mo>=</mo>
                    <mn>57</mn><mo>.</mo><mn>2</mn><mo>&#xA0;</mo><mi>cm</mi></math>";
    test_braille("UEB", expr, "⠼⠑⠲⠛⠃⠀⠍⠐⠌⠼⠁⠚⠀⠐⠶⠀⠼⠑⠛⠲⠃⠀⠉⠍")?;
    return Ok(());

}

#[test]
fn expr_3_1_7() -> Result<()> {
    let expr = "<math><mn>15</mn><mo>±</mo><mn>0</mn><mo>.</mo><mn>5</mn></math>";
    test_braille("UEB", expr, "⠼⠁⠑⠸⠖⠼⠚⠲⠑")?;
    return Ok(());

}

#[test]
fn expr_3_1_8() -> Result<()> {
    let expr = "<math><mi>Area</mi><mo>=</mo><mi>b</mi><mi>h</mi><mo>=</mo>
            <mn>5</mn><mo>·</mo><mn>3</mn><mo>=</mo><mn>15</mn></math>";
    test_braille("UEB", expr, "⠠⠜⠑⠁⠀⠐⠶⠀⠃⠓⠀⠐⠶⠀⠼⠑⠐⠲⠼⠉⠀⠐⠶⠀⠼⠁⠑")?;
    return Ok(());

}

#[test]
fn expr_3_1_9_wiris() -> Result<()> {
    let expr = "<math><mn>3</mn><mo>.</mo><mn>9</mn><mo>×</mo><mn>4</mn><mo>.</mo><mn>1</mn><mo>≃</mo><mn>16</mn></math>";
    test_braille("UEB", expr, "⠼⠉⠲⠊⠐⠦⠼⠙⠲⠁⠀⠸⠔⠀⠼⠁⠋")?;
    return Ok(());

}

#[test]
fn expr_3_1_9() -> Result<()> {
    let expr = "<math><mn>3.9</mn><mo>×</mo><mn>4.1</mn><mo>≃</mo><mn>16</mn></math>";
    test_braille("UEB", expr, "⠼⠉⠲⠊⠐⠦⠼⠙⠲⠁⠀⠸⠔⠀⠼⠁⠋")?;
    return Ok(());

}

#[test]
fn expr_3_1_10() -> Result<()> {
    let expr = "<math><mn>5</mn><mo>-</mo><mn>3</mn><mo>≠</mo><mn>3</mn><mo>-</mo><mn>5</mn></math>";
    test_braille("UEB", expr, "⠼⠑⠐⠤⠼⠉⠀⠐⠶⠈⠱⠀⠼⠉⠐⠤⠼⠑")?;
    return Ok(());

}

#[test]
fn ratio_3_1_11() -> Result<()> {
    let expr = "<math><mn>1</mn><mo>:</mo><mn>200</mn></math>";
    test_braille("UEB", expr, "⠼⠁⠒⠼⠃⠚⠚")?;
    return Ok(());

}

#[test]
fn ratio_3_1_12() -> Result<()> {
    let expr = "<math><mn>2</mn><mo>:</mo><mn>4</mn><mo>=</mo><mn>6</mn><mo>:</mo><mn>12</mn></math>";
    test_braille("UEB", expr, "⠼⠃⠒⠼⠙⠀⠐⠶⠀⠼⠋⠒⠼⠁⠃")?;
    return Ok(());

}

#[test]
fn alg_3_2_1_1() -> Result<()> {
    let expr = "<math><mi>y</mi><mo>∝</mo><mi>x</mi></math>";
    test_braille("UEB", expr, "⠰⠽⠀⠸⠐⠶⠀⠰⠭")?;
    return Ok(());

}

#[test]
fn alg_3_2_1_2() -> Result<()> {
    let expr = "<math><mi>y</mi><mo>=</mo><mi>k</mi><mi>x</mi></math>";
    test_braille("UEB", expr, "⠰⠽⠀⠐⠶⠀⠅⠭")?;
    return Ok(());

}

#[test]
fn alg_3_2_2() -> Result<()> {
    let expr = "<math><mn>0</mn><mo>≤</mo><mi>θ</mi>
            <mo>≤</mo><mn>2</mn><mi mathvariant='normal'>π</mi></math>";
    test_braille("UEB", expr, "⠼⠚⠀⠸⠈⠣⠀⠨⠹⠀⠸⠈⠣⠀⠼⠃⠨⠏")?;
    return Ok(());

}

#[test]
fn alg_3_2_3() -> Result<()> {
    let expr = "<math><mi>y</mi><mo>=</mo><mi>x</mi><mo>+</mo><mn>4</mn></math>";
    test_braille("UEB", expr, "⠰⠽⠀⠐⠶⠀⠭⠐⠖⠼⠙")?;
    return Ok(());

}


#[test]
fn alg_3_2_4() -> Result<()> {
    let expr = "<math><mn>2</mn><mi>y</mi><mo>=</mo><mn>2</mn><mi>c</mi><mo>-</mo><mn>4</mn></math>";
    test_braille("UEB", expr, "⠼⠃⠽⠀⠐⠶⠀⠼⠃⠰⠉⠐⠤⠼⠙")?;
    return Ok(());

}

#[test]
fn alg_3_2_5() -> Result<()> {
    let expr = "<math><mi>d</mi><mo>+</mo><mi>a</mi><mi>b</mi><mo>=</mo><mi>a</mi><mi>c</mi></math>";
    // GTM 1.7 / BANA 2026: one G1 symbol for standing-alone 'a' after '='
    test_braille("UEB", expr, "⠙⠐⠖⠁⠃⠀⠐⠶⠀⠰⠁⠉")?;
    return Ok(());

}

#[test]
fn ratio_3_2_6() -> Result<()> {
    // the difference from ratio_3_1_12 is this involves letters
    let expr = "<math><mi>x</mi><mo>:</mo><mi>y</mi></math>";
    test_braille("UEB", expr, "⠭⠰⠒⠽")?;
    return Ok(());

}

#[test]
fn standing_alone_1() -> Result<()> {
    // Tests bug: github.com/NSoiffer/MathCAT/issues/142
    let expr = "<math><mo>(</mo><mi>n</mi><mo>=</mo><mn>7</mn><mo>)</mo></math>";
    test_braille("UEB", expr, "⠐⠣⠰⠝⠀⠐⠶⠀⠼⠛⠐⠜")?;
    return Ok(());

}

#[test]
fn example_3_4_1() -> Result<()> {
    let expr = "<math><mo>-</mo><mn>4</mn><mtext>&#xA0;to&#xA0;</mtext><mo>+</mo><mn>5</mn></math>";
    test_braille("UEB", expr, "⠐⠤⠼⠙⠀⠞⠕⠀⠐⠖⠼⠑")?;
    return Ok(());

}

#[test]
fn example_3_4_2() -> Result<()> {
    // removed some cruft from TeX output of {}^{-}2+{}^{+}3, but the basics are preserved
    let expr = "<math>
        <msup> <mrow/> <mo>−</mo></msup>
        <mn>2</mn>
        <mo>+</mo>
        <msup> <mrow/> <mo>−</mo></msup>
        <mn>3</mn>
    </math>";
    test_braille("UEB", expr, "⠰⠔⠐⠤⠼⠃⠐⠖⠔⠐⠤⠼⠉")?;
    return Ok(());

}

#[test]
fn omission_3_6_1() -> Result<()> {
    let expr = "<math><mn>3</mn><mo>+</mo><mn>7</mn><mo>=</mo><mo>―</mo></math>";
    test_braille("UEB", expr, "⠼⠉⠐⠖⠼⠛⠀⠐⠶⠀⠐⠠⠤")?;
    return Ok(());

}

#[test]
fn omission_3_6_2() -> Result<()> {
    let expr = "<math><mn>3</mn><mo>+</mo><mn>7</mn><mo>=</mo><mi>_</mi></math>";
    test_braille("UEB", expr, "⠼⠉⠐⠖⠼⠛⠀⠐⠶⠀⠨⠤")?;
    return Ok(());

}

#[test]
fn omission_3_6_3() -> Result<()> {
    let expr = "<math><mn>3</mn><mo>+</mo><mn>7</mn><mo>=</mo><mo>?</mo></math>";
    test_braille("UEB", expr, "⠼⠉⠐⠖⠼⠛⠀⠐⠶⠀⠰⠦")?;
    return Ok(());

}

#[test]
fn omission_3_6_4() -> Result<()> {
    let expr = "<math><mn>3</mn><mo>□</mo><mn>7</mn><mo>=</mo><mn>10</mn></math>";
    test_braille("UEB", expr, "⠼⠉⠫⠼⠙⠱⠼⠛⠀⠐⠶⠀⠼⠁⠚")?;
    return Ok(());

}

#[test]
fn omission_3_6_5() -> Result<()> {
    let expr = "<math><mn>3</mn><mo>&#xA0;</mo><mo>&#xA0;</mo><mn>7</mn><mo>=</mo><mn>10</mn></math>";
    test_braille("UEB", expr, "⠼⠉⠬⠼⠛⠀⠐⠶⠀⠼⠁⠚")?;
    return Ok(());

}

#[test]
fn omission_3_6_6() -> Result<()> {
    // comes from WIRIS
    let expr = "<math><mfrac><mn>9</mn><mn>12</mn></mfrac><mo>=</mo><mfrac><mn>3</mn><mrow/></mfrac></math>";
    test_braille("UEB", expr, "⠼⠊⠌⠁⠃⠀⠐⠶⠀⠰⠷⠼⠉⠨⠌⠬⠾")?;
    return Ok(());

}

#[test]
fn omission_3_6_7() -> Result<()> {
    // comes from MathType
    let expr = "<math><mrow><mn>5</mn><mo>=</mo><msqrt><mrow/></msqrt></mrow></math>";
    test_braille("UEB", expr, "⠼⠑⠀⠐⠶⠀⠰⠰⠩⠬⠬")?;
    return Ok(());

}

#[test]
fn fraction_6_1_1() -> Result<()> {
    let expr = "<math><mfrac><mn>5</mn><mn>8</mn></mfrac></math>";
    test_braille("UEB", expr, "⠼⠑⠌⠓")?;
    return Ok(());

}

#[test]
fn fraction_6_1_2() -> Result<()> {
    let expr = "<math><mfrac><mrow><mn>5</mn><mo>.</mo><mn>7</mn></mrow><mrow><mn>2</mn><mo>,</mo><mn>000</mn></mrow></mfrac></math>";
    test_braille("UEB", expr, "⠼⠑⠲⠛⠌⠃⠂⠚⠚⠚")?;
    return Ok(());

}

#[test]
fn fraction_6_2_1() -> Result<()> {
    let expr = "<math><mn>2</mn><mfrac bevelled='true'><mn>1</mn><mn>2</mn></mfrac></math>";
    test_braille("UEB", expr, "⠼⠃⠼⠁⠌⠃")?;
    return Ok(());

}

#[test]
fn fraction_6_2_2() -> Result<()> {
    let expr = "<math><mn>1750</mn>
                <mo>&#xA0;</mo><mi mathvariant='normal' class='MathML-unit'>cm</mi><mo>=</mo>
                <mn>1</mn><mfrac bevelled='true'><mn>3</mn><mn>4</mn></mfrac>
                <mo>&#xA0;</mo><mi mathvariant='normal' class='MathML-unit'>m</mi></math>";
    test_braille("UEB", expr, "⠼⠁⠛⠑⠚⠀⠉⠍⠀⠐⠶⠀⠼⠁⠼⠉⠌⠙⠀⠰⠍")?;
    return Ok(());

}

#[test]
fn fraction_6_2_2_unit() -> Result<()> {
    // Same as fraction_6_2_2; unit marked via data-intent-property
    let expr = "<math><mn>1750</mn>
                <mo>&#xA0;</mo><mi mathvariant='normal' data-intent-property=':unit:'>cm</mi><mo>=</mo>
                <mn>1</mn><mfrac bevelled='true'><mn>3</mn><mn>4</mn></mfrac>
                <mo>&#xA0;</mo><mi mathvariant='normal' data-intent-property=':unit:'>m</mi></math>";
    test_braille("UEB", expr, "⠼⠁⠛⠑⠚⠀⠉⠍⠀⠐⠶⠀⠼⠁⠼⠉⠌⠙⠀⠰⠍")?;
    return Ok(());

}

#[test]
fn fraction_6_2_2_unicode_frac() -> Result<()> {
    let expr = "<math><mn>1750</mn>
                <mo>&#xA0;</mo><mi mathvariant='normal' class='MathML-unit'>cm</mi><mo>=</mo>
                <mn>1</mn><mn>¾</mn>
                <mo>&#xA0;</mo><mi mathvariant='normal' class='MathML-unit'>m</mi></math>";
    test_braille("UEB", expr, "⠼⠁⠛⠑⠚⠀⠉⠍⠀⠐⠶⠀⠼⠁⠼⠉⠌⠙⠀⠰⠍")?;
    return Ok(());

}

#[test]
fn fraction_6_2_2_unicode_frac_unit() -> Result<()> {
    // Same as fraction_6_2_2_unicode_frac; unit marked via data-intent-property
    let expr = "<math><mn>1750</mn>
                <mo>&#xA0;</mo><mi mathvariant='normal' data-intent-property=':unit:'>cm</mi><mo>=</mo>
                <mn>1</mn><mn>¾</mn>
                <mo>&#xA0;</mo><mi mathvariant='normal' data-intent-property=':unit:'>m</mi></math>";
    test_braille("UEB", expr, "⠼⠁⠛⠑⠚⠀⠉⠍⠀⠐⠶⠀⠼⠁⠼⠉⠌⠙⠀⠰⠍")?;
    return Ok(());

}

#[test]
fn fraction_6_3_1() -> Result<()> {
    let expr = "<math><mn>3</mn><mo>/</mo><mn>8</mn></math>";
    test_braille("UEB", expr, "⠼⠉⠸⠌⠼⠓")?;
    return Ok(());

}

#[test]
fn fraction_6_4_2() -> Result<()> {
    let expr = "<math><mfrac>
        <mrow><mn>2</mn><mfrac><mn>1</mn><mn>2</mn></mfrac></mrow>
        <mrow><mi>x</mi><mo>+</mo><mi>y</mi></mrow>
        </mfrac></math>";
    test_braille("UEB", expr, "⠰⠷⠼⠃⠼⠁⠌⠃⠨⠌⠭⠐⠖⠽⠾")?;
    return Ok(());

}

#[test]
fn fraction_6_4_3() -> Result<()> {
    let expr = "<math><mfrac><mrow><mn>2</mn><mo>/</mo><mn>3</mn></mrow><mn>5</mn></mfrac></math>";
    test_braille("UEB", expr, "⠰⠷⠼⠃⠸⠌⠼⠉⠨⠌⠼⠑⠾")?;
    return Ok(());

}

#[test]
fn fraction_6_4_4() -> Result<()> {
    let expr = "<math><mfrac>
    <mrow><mfrac><mi>x</mi><mn>2</mn></mfrac><mo>+</mo><mfrac><mi>y</mi><mn>3</mn></mfrac></mrow>
    <mrow><mi>x</mi><mo>+</mo><mi>y</mi></mrow>
        </mfrac></math>";
    test_braille("UEB", expr, "⠰⠰⠷⠷⠭⠨⠌⠼⠃⠾⠐⠖⠷⠽⠨⠌⠼⠉⠾⠨⠌⠭⠐⠖⠽⠾")?;
    return Ok(());

}

#[test]
fn fraction_6_4_5() -> Result<()> {
    let expr = "<math><mfrac>
        <mrow><mfrac><mi>x</mi><mn>2</mn></mfrac><mo>+</mo><mfrac><mi>y</mi><mn>3</mn></mfrac></mrow>
        <mrow><mi>x</mi><mo>+</mo><mi>y</mi></mrow>
        </mfrac></math>";
    test_braille("UEB", expr, "⠰⠰⠷⠷⠭⠨⠌⠼⠃⠾⠐⠖⠷⠽⠨⠌⠼⠉⠾⠨⠌⠭⠐⠖⠽⠾")?;
    return Ok(());

}

#[test]
fn fraction_6_4_6() -> Result<()> {
    let expr = "<math><mtext>speed</mtext><mo>=</mo><mfrac><mtext>distance</mtext><mtext>time</mtext></mfrac></math>";
    // GTM 2014 §6.4 listed word-indicator and passage alternatives (uncontracted words).
    // GTM 1.7.5(a) (2025) prefers two grade 1 symbol indicators with contracted words.
    test_braille("UEB", expr, "⠎⠏⠑⠫⠀⠐⠶⠀⠰⠷⠙⠊⠌⠨⠑⠨⠌⠐⠞⠰⠾")?;
    return Ok(());

}


#[test]
fn msup_7_3_2() -> Result<()> {
    let expr = "<math><msup><mi>x</mi><mn>2</mn></msup><mi>y</mi></math>";
    test_braille("UEB", expr, "⠭⠰⠔⠼⠃⠽")?;
    return Ok(());

}

#[test]
fn msup_7_3_3() -> Result<()> {
    let expr = "<math><msup><mi>x</mi><mrow><mn>2</mn><mi>y</mi></mrow></msup></math>";
    test_braille("UEB", expr, "⠰⠰⠭⠔⠣⠼⠃⠽⠜")?;
    return Ok(());

}

#[test]
fn msup_7_3_4() -> Result<()> {
    let expr = "<math><msup><mi>x</mi><mi>y</mi></msup><mo>+</mo><mn>1</mn></math>";
    test_braille("UEB", expr, "⠭⠰⠔⠽⠐⠖⠼⠁")?;
    return Ok(());

}

#[test]
fn msup_7_3_6() -> Result<()> {
    let expr = "<math><msup><mi>x</mi><mrow><mi>y</mi><mo>+</mo><mn>1</mn></mrow></msup><mo>+</mo><mn>3</mn></math>";
    test_braille("UEB", expr, "⠰⠰⠭⠔⠣⠽⠐⠖⠼⠁⠜⠐⠖⠼⠉")?;
    return Ok(());

}

#[test]
fn msup_7_3_7() -> Result<()> {
    let expr = "<math><msup><mi>x</mi><mn>⅔</mn></msup></math>";
    test_braille("UEB", expr, "⠭⠰⠔⠼⠃⠌⠉")?;
    return Ok(());

}

#[test]
fn msup_7_3_11() -> Result<()> {
    let expr = "<math><msup><mi>x</mi><mfrac><mi>a</mi><mi>b</mi></mfrac></msup><mi>y</mi><mo>=</mo><mi>x</mi></math>";
    test_braille("UEB", expr, "⠰⠰⠭⠔⠷⠁⠨⠌⠃⠾⠽⠀⠐⠶⠀⠰⠭")?;
    return Ok(());

}

#[test]
fn msup_7_4_1() -> Result<()> {
    let expr = "<math><msup><mi>e</mi><msup><mi>x</mi><mn>2</mn></msup></msup></math>";
    test_braille("UEB", expr, "⠰⠰⠑⠔⠣⠭⠔⠼⠃⠜")?;
    return Ok(());

}

#[test]
fn msup_7_4_2() -> Result<()> {
    let expr = "<math><msup><mi>e</mi><mrow><mo>(</mo><msup><mi>x</mi><mn>2</mn></msup><mo>)</mo></mrow></msup></math>";
    test_braille("UEB", expr, "⠰⠰⠑⠔⠐⠣⠭⠔⠼⠃⠐⠜")?;
    return Ok(());

}

#[test]
fn msub_7_4_3() -> Result<()> {
    let expr = "<math><msub><mi>P</mi><msub><mi>x</mi><mi>i</mi></msub></msub></math>";
    test_braille("UEB", expr, "⠰⠰⠠⠏⠢⠣⠭⠢⠊⠜")?;
    return Ok(());

}

#[test]
fn msup_7_5_1() -> Result<()> {
    let expr = "<math><mn>0</mn><mo>.</mo><mn>0045</mn><mo>=</mo>
        <mn>4</mn><mo>.</mo><mn>5</mn><mo>×</mo><msup><mn>10</mn><mrow><mo>-</mo><mn>3</mn></mrow></msup>
        </math>";
    test_braille("UEB", expr, "⠼⠚⠲⠚⠚⠙⠑⠀⠐⠶⠀⠼⠙⠲⠑⠐⠦⠼⠁⠚⠔⠣⠐⠤⠼⠉⠜")?;
    return Ok(());

}

#[test]
fn msup_7_5_3() -> Result<()> {
    let expr = "<math><msup><mi>a</mi><mrow><mo>-</mo><mn>2</mn><mi>b</mi></mrow></msup></math>";
    test_braille("UEB", expr, "⠰⠰⠁⠔⠣⠐⠤⠼⠃⠰⠃⠜")?;
    return Ok(());

}

#[test]
fn msup_7_6_2() -> Result<()> {
    let expr = "<math><msup><mi mathvariant='normal'>H</mi><mo>+</mo></msup></math>";
    test_braille("UEB", expr, "⠠⠓⠰⠔⠐⠖")?;
    return Ok(());

}

#[test]
fn msubsup_7_7_1() -> Result<()> {
    let expr = "<math><msubsup><mi>x</mi><mn>1</mn><mn>2</mn></msubsup><mo>=</mo><msubsup><mi>y</mi><mn>2</mn><mn>3</mn></msubsup></math>";
    test_braille("UEB", expr, "⠭⠰⠢⠼⠁⠔⠼⠃⠀⠐⠶⠀⠽⠰⠢⠼⠃⠔⠼⠉")?;
    return Ok(());

}

#[test]
fn msubsup_7_7_2() -> Result<()> {
    let expr = "<math><msub><msup><mi>x</mi><mn>2</mn></msup><mi>k</mi></msub></math>";
    test_braille("UEB", expr, "⠭⠰⠔⠼⠃⠢⠅")?;
    return Ok(());

}

#[test]
fn pre_msubsup_7_8_1() -> Result<()> {
    // Note: modified because word indicator is not needed
    let expr = "<math><mmultiscripts><mi>U</mi><mprescripts/><mn>92</mn><mn>238</mn></mmultiscripts></math>";
    test_braille("UEB", expr, "⠰⠢⠼⠊⠃⠔⠼⠃⠉⠓⠠⠥")?;
    return Ok(());

}

#[test]
fn pre_sup_7_8_2() -> Result<()> {
    let expr = "<math><mmultiscripts><mn>2</mn><mprescripts/><none/><mo>-</mo></mmultiscripts>
            <mo>+</mo><mmultiscripts><mn>3</mn><mprescripts/><none/><mo>-</mo></mmultiscripts>
            <mo>=</mo><mmultiscripts><mn>5</mn><mprescripts/><none/><mo>-</mo></mmultiscripts>
        </math>";
    test_braille("UEB", expr, "⠰⠔⠐⠤⠼⠃⠐⠖⠔⠐⠤⠼⠉⠀⠐⠶⠀⠰⠔⠐⠤⠼⠑")?;
    return Ok(());

}


#[test]
fn sum_7_9_1() -> Result<()> {
    let expr = "<math><munderover><mo>∑</mo><mrow><mi>x</mi><mo>=</mo><mn>1</mn></mrow><mi>n</mi></munderover>
            <msubsup><mi>x</mi><mi>i</mi><mn>2</mn></msubsup></math>";
    test_braille("UEB", expr, "⠠⠨⠎⠨⠢⠰⠣⠭⠐⠶⠼⠁⠜⠨⠔⠝⠭⠢⠊⠔⠼⠃")?;
    return Ok(());

}

#[test]
fn lim_7_9_2() -> Result<()> {
    // Note: modified because passage indicator is not needed (same expr when word indicator is used)
    let expr = "<math><munder><mi>lim</mi><mrow><mi>x</mi><mo>→</mo><mi>a</mi></mrow></munder>
            <mi>f</mi><mo>(</mo><mi>x</mi><mo>)</mo><mo>=</mo><mn>1</mn></math>";
    test_braille("UEB", expr, "⠰⠰⠇⠊⠍⠨⠢⠣⠭⠳⠕⠁⠜⠋⠐⠣⠭⠐⠜⠀⠐⠶⠀⠼⠁")?;
    return Ok(());

}

#[test]
fn sqrt_8_1_1() -> Result<()> {
    let expr = "<math><msqrt><mn>9</mn></msqrt><mo>=</mo><mn>3</mn></math>";
    test_braille("UEB", expr, "⠰⠩⠼⠊⠬⠀⠐⠶⠀⠼⠉")?;
    return Ok(());

}

#[test]
fn sqrt_8_1_2() -> Result<()> {
    let expr = "<math><mi>r</mi><mo>=</mo>
        <msqrt><msup><mi>x</mi><mn>2</mn></msup><mo>+</mo><msup><mi>y</mi><mn>2</mn></msup></msqrt></math>";
    test_braille("UEB", expr, "⠰⠗⠀⠐⠶⠀⠰⠰⠩⠭⠔⠼⠃⠐⠖⠽⠔⠼⠃⠬")?;
    return Ok(());

}

#[test]
fn sqrt_8_1_3() -> Result<()> {
    let expr = "<math>
    <msqrt>
      <mfrac>
        <mrow><mn>783.2</mn><mo>×</mo><mn>6.547</mn></mrow>
        <mn>0.4628</mn>
      </mfrac>
    </msqrt>
  </math>";
    test_braille("UEB", expr, "⠰⠰⠩⠷⠼⠛⠓⠉⠲⠃⠐⠦⠼⠋⠲⠑⠙⠛⠨⠌⠼⠚⠲⠙⠋⠃⠓⠾⠬")?;
    return Ok(());

}

#[test]
fn sqrt_8_1_4() -> Result<()> {
    let expr = "<math>
      <mi>x</mi> <mo>=</mo>
      <mfrac>
       <mrow>
        <mo>−</mo><mi>b</mi><mo>±</mo>
        <msqrt>
          <msup><mi>b</mi> <mn>2</mn></msup>
          <mo>−</mo><mn>4</mn><mi>a</mi><mi>c</mi>
        </msqrt>
        </mrow>
       <mrow><mn>2</mn><mi>a</mi></mrow>
      </mfrac>
      </math>
    ";
    test_braille("UEB", expr, "⠰⠭⠀⠐⠶⠀⠰⠰⠷⠐⠤⠃⠸⠖⠩⠃⠔⠼⠃⠐⠤⠼⠙⠰⠁⠉⠬⠨⠌⠼⠃⠰⠁⠾")?;
    return Ok(());

}

#[test]
fn root_8_2_1() -> Result<()> {
    let expr = "<math><mroot><mn>8</mn><mn>3</mn></mroot><mo>=</mo><mn>2</mn></math>";
    test_braille("UEB", expr, "⠰⠰⠩⠔⠼⠉⠼⠓⠬⠀⠐⠶⠀⠼⠃")?;
    return Ok(());

}

#[test]
fn root_8_2_2() -> Result<()> {
    let expr = "<math><mi>q</mi><mo>=</mo>
        <mroot><mrow>
        <msup><mi>x</mi><mn>3</mn></msup><mo>+</mo>
        <msup><mi>y</mi><mn>3</mn></msup><mo>+</mo>
        <msup><mi>z</mi><mn>3</mn></msup>
        </mrow><mn>3</mn></mroot></math>";
    test_braille("UEB", expr, "⠰⠟⠀⠐⠶⠀⠰⠰⠩⠔⠼⠉⠭⠔⠼⠉⠐⠖⠽⠔⠼⠉⠐⠖⠵⠔⠼⠉⠬")?;
    return Ok(());

}

#[test]
fn root_8_2_3() -> Result<()> {
    let expr = "<math><mroot><mrow><mi>x</mi><mi>y</mi></mrow><mrow><mi>m</mi><mi>n</mi></mrow></mroot></math>";
    test_braille("UEB", expr, "⠰⠰⠩⠔⠣⠍⠝⠜⠭⠽⠬")?;
    return Ok(());

}

#[test]
fn root_8_2_4() -> Result<()> {
    let expr = "<math>
        <msup><mn>81</mn><mfrac><mn>3</mn><mn>4</mn></mfrac></msup> <mo>=</mo>
        <msup><mrow><mo>(</mo><mroot><mn>81</mn><mn>4</mn></mroot><mo>)</mo></mrow><mn>3</mn></msup><mo>=</mo>
        <msup><mrow><mo>(</mo><msqrt><msqrt><mn>81</mn></msqrt></msqrt><mo>)</mo></mrow><mn>3</mn></msup><mo>=</mo>
        <msup><mrow><mo>(</mo><msqrt><mn>9</mn></msqrt><mo>)</mo></mrow><mn>3</mn></msup>  <mo>=</mo>
        <msup><mn>3</mn><mn>3</mn></msup>
        <mo>=</mo> <mn>27</mn> </math>";
    test_braille("UEB", expr, "⠰⠰⠰⠼⠓⠁⠔⠼⠉⠌⠙⠀⠐⠶⠀⠐⠣⠩⠔⠼⠙⠼⠓⠁⠬⠐⠜⠔⠼⠉⠀⠐⠶⠀⠐⠣⠩⠩⠼⠓⠁⠬⠬⠐⠜⠔⠼⠉⠀⠐⠶⠀⠐⠣⠩⠼⠊⠬⠐⠜⠔⠼⠉⠀⠐⠶⠀⠼⠉⠔⠼⠉⠀⠐⠶⠀⠼⠃⠛⠰⠄")?;
    return Ok(());

}

#[test]
fn root_letter_base() -> Result<()> {
    // none of the guides cover this case, but it seems that an a-j base needs a grade 1 indicator
    let expr = "<math><mroot><mi>b</mi><mn>3</mn></mroot><mroot><mi>x</mi><mn>3</mn></mroot></math>";
    test_braille("UEB", expr, "⠰⠰⠩⠔⠼⠉⠰⠃⠬⠩⠔⠼⠉⠭⠬")?;
    return Ok(());

}

#[test]
fn spacing_9_3_1_1() -> Result<()> {
    let expr = "<math> <mi>Sin</mi><mo>&#x2061;</mo> <mn>30</mn> </math>";
    test_braille("UEB", expr, "⠠⠎⠔⠼⠉⠚")?;
    return Ok(());

}

#[test]
fn spacing_9_3_1_2() -> Result<()> {
    let expr = "<math><mn>3</mn><mi>tan</mi><mn>45</mn><mo>°</mo></math>";
    test_braille("UEB", expr, "⠼⠉⠞⠁⠝⠼⠙⠑⠘⠚")?;
    return Ok(());

}

#[test]
fn spacing_9_3_1_3() -> Result<()> {
    let expr = "<math><mn>4</mn><mi>cos</mi><mn>5</mn><mi>x</mi></math>";
    test_braille("UEB", expr, "⠼⠙⠰⠉⠕⠎⠼⠑⠭")?;
    return Ok(());

}

#[test]
fn spacing_9_3_2_1() -> Result<()> {
    let expr = "<math><mi>log</mi><mi>y</mi></math>";
    test_braille("UEB", expr, "⠇⠕⠛⠀⠰⠽")?;
    return Ok(());

}

#[test]
fn spacing_9_3_2_2() -> Result<()> {
    let expr = "<math><mi>sin</mi><mi>θ</mi></math>";
    test_braille("UEB", expr, "⠎⠔⠨⠹")?;
    return Ok(());

}

#[test]
fn spacing_9_3_2_3() -> Result<()> {
    let expr = "<math><mi>Sec</mi><mi>A</mi></math>";
    test_braille("UEB", expr, "⠠⠎⠑⠉⠠⠁")?;
    return Ok(());

}

#[test]
fn spacing_9_3_2_4() -> Result<()> {
    let expr = "<math><mi>log</mi><mo>(</mo><mi>x</mi><mo>+</mo><mi>y</mi><mo>)</mo></math>";
    test_braille("UEB", expr, "⠇⠕⠛⠐⠣⠭⠐⠖⠽⠐⠜")?;
    return Ok(());

}

#[test]
fn spacing_9_3_2_5() -> Result<()> {
    let expr = "<math><mi>Lim</mi><mfrac><mi>x</mi><mn>2</mn></mfrac></math>";
    // GTM 2014 9.3.2 used a word indicator (;;,lim(x./#b)). GTM 1.7.3(a): one
    // symbol indicator on the fraction open; numeric mode covers the closer.
    test_braille("UEB", expr, "⠠⠇⠊⠍⠰⠷⠭⠨⠌⠼⠃⠾")?;
    return Ok(());

}

#[test]
fn spacing_9_3_3_1() -> Result<()> {
    let expr = "<math><mi>x</mi><mi>sin</mi><mn>60</mn></math>";
    test_braille("UEB", expr, "⠰⠭⠀⠎⠔⠼⠋⠚")?;
    return Ok(());

}

#[test]
fn spacing_9_3_3_2() -> Result<()> {
    let expr = "<math><mi>x</mi><mrow><mi>Sin</mi><mo>&#x2061;</mo><mn>60</mn></mrow></math>";
    test_braille("UEB", expr, "⠭⠠⠎⠔⠼⠋⠚")?;
    return Ok(());

}

#[test]
fn spacing_9_3_3_3() -> Result<()> {
    let expr = "<math><mi>X</mi><mi>log</mi><mi>y</mi></math>";
    test_braille("UEB", expr, "⠰⠠⠭⠀⠇⠕⠛⠀⠰⠽")?;
    return Ok(());

}

#[test]
fn spacing_9_3_3_4() -> Result<()> {
    let expr = "<math><mi>x</mi><mi>Log</mi><mi>y</mi></math>";
    test_braille("UEB", expr, "⠭⠠⠇⠕⠛⠀⠰⠽")?;
    return Ok(());

}

#[test]
fn spacing_9_3_3_5() -> Result<()> {
    let expr = "<math>
        <mi>sin</mi><mo>&#x2061;</mo><mrow><mo>(</mo><mi>A</mi><mo>+</mo><mi>B</mi><mo>)</mo> </mrow>
        <mo>=</mo><mi>sin</mi><mo>&#x2061;</mo><mi>A</mi><mi>cos</mi><mo>&#x2061;</mo>  <mi>B</mi>
        <mo>+</mo><mi>cos</mi><mo>&#x2061;</mo><mi>A</mi><mi>sin</mi><mo>&#x2061;</mo><mi>B</mi></math>";
    test_braille("UEB", expr, "⠎⠔⠐⠣⠠⠁⠐⠖⠠⠃⠐⠜⠀⠐⠶⠀⠎⠔⠠⠁⠀⠉⠕⠎⠠⠃⠐⠖⠉⠕⠎⠠⠁⠀⠎⠔⠠⠃")?;
    return Ok(());

}

#[test]
fn spacing_9_3_3_6() -> Result<()> {
    let expr = "<math><mi>sin</mi><mn>2</mn><mi>β</mi><mo>=</mo>
                <mn>2</mn><mi>sin</mi><mi>β</mi><mi>cos</mi><mi>β</mi></math>";
    test_braille("UEB", expr, "⠎⠔⠼⠃⠨⠃⠀⠐⠶⠀⠼⠃⠎⠊⠝⠨⠃⠉⠕⠎⠨⠃")?;
    return Ok(());

}

#[test]
fn text_9_7_1_mtext() -> Result<()> {
    let expr = "<math><mtext>Pr</mtext><mo>(</mo><mi>A</mi><mo>&#xA0;</mo><mtext>and</mtext><mo>&#xA0;</mo><mi>B</mi><mo>)</mo><mo>=</mo>
         <mtext>Pr</mtext><mi>A</mi><mo>+</mo><mtext>Pr</mtext><mi>B</mi></math>";
    test_braille("UEB", expr, "⠠⠏⠗⠐⠣⠠⠁⠀⠯⠀⠰⠠⠃⠐⠜⠀⠐⠶⠀⠠⠏⠗⠠⠁⠐⠖⠠⠏⠗⠠⠃")?;
    return Ok(());

}

#[test]
fn text_9_7_1() -> Result<()> {
    // ugly as the MathML is with non-breaking space in mo's, this is a WIRIS editor output
    let expr = "<math><mi>Pr</mi><mo>(</mo><mi>A</mi><mo>&#xA0;</mo><mi>and</mi><mo>&#xA0;</mo><mi>B</mi><mo>)</mo><mo>=</mo>
         <mi>Pr</mi><mi>A</mi><mo>+</mo><mi>Pr</mi><mi>B</mi></math>";
    test_braille("UEB", expr, "⠠⠏⠗⠐⠣⠠⠁⠀⠯⠀⠰⠠⠃⠐⠜⠀⠐⠶⠀⠠⠏⠗⠠⠁⠐⠖⠠⠏⠗⠠⠃")?;
    return Ok(());

}

#[test]
fn stat_9_7_2() -> Result<()> {
    let expr = "<math><mi>Exp</mi><mo>(</mo><mi>R</mi><mo>)</mo><mo>=</mo>
                            <mfrac><mi>n</mi><mn>2</mn></mfrac><mo>+</mo><mn>1</mn></math>";
    // GTM 2014 9.7 used a passage around the whole equation. GTM 1.7.3(a): one
    // symbol indicator on n/2 (same as grade1_1_7_9_5).
    test_braille("UEB", expr, "⠠⠑⠭⠏⠐⠣⠠⠗⠐⠜⠀⠐⠶⠀⠰⠷⠝⠨⠌⠼⠃⠾⠐⠖⠼⠁")?;
    return Ok(());

}

#[test]
fn set_10_1() -> Result<()> {
    let expr = "<math><mi>A</mi><mo>=</mo>
        <mfenced open='{' close='}'> 
        <mrow><mn>1</mn><mo>,</mo><mn>2</mn><mo>,</mo><mn>3</mn><mo>,</mo><mn>4</mn></mrow>
    </mfenced></math>";
    test_braille("UEB", expr, "⠠⠁⠀⠐⠶⠀⠸⠣⠼⠁⠂⠀⠼⠃⠂⠀⠼⠉⠂⠀⠼⠙⠸⠜")?;
    return Ok(());

}

#[test]
fn set_10_3() -> Result<()> {
    let expr = "<math><mn>3</mn><mo>∈</mo><mi>A</mi><mo>∩</mo><mi>B</mi></math>";
    test_braille("UEB", expr, "⠼⠉⠀⠘⠑⠀⠠⠁⠨⠦⠠⠃")?;
    return Ok(());

}

#[test]
fn set_10_4() -> Result<()> {
    let expr = "<math><mi>A</mi><mo>∩</mo><mi>B</mi><mo>⊂</mo><mi>A</mi><mo>∪</mo><mi>B</mi></math>";
    test_braille("UEB", expr, "⠠⠁⠨⠦⠠⠃⠀⠘⠣⠀⠠⠁⠨⠖⠠⠃")?;
    return Ok(());

}
#[test]
fn set_10_5() -> Result<()> {
    let expr = "<math><msup><mi>A</mi><mo>'</mo></msup><mo>∪</mo><msup><mi>B</mi><mo>'</mo></msup><mo>=</mo>
                        <msup><mrow><mo>(</mo><mi>A</mi><mo>∩</mo><mi>B</mi><mo>)</mo></mrow><mo>'</mo></msup></math>";
    test_braille("UEB", expr, "⠰⠰⠠⠁⠶⠨⠖⠠⠃⠶⠀⠐⠶⠀⠐⠣⠠⠁⠨⠦⠠⠃⠐⠜⠰⠶")?;
    return Ok(());

}

#[test]
fn set_10_6() -> Result<()> {
    // Note: example uses the wrong char "├" in the display -- should be "⊢"
    let expr = "<math><mo>[</mo><mo>(</mo><mi>p</mi><mo>∨</mo><mi>q</mi><mo>)</mo><mo>∧</mo><mo>¬</mo><mi>p</mi><mo>]</mo>
                <mo>⊢</mo><mi>q</mi></math>";
    // GTM / BANA 2026: grade 1 passage
    test_braille("UEB", expr, "⠨⠣⠐⠣⠏⠈⠖⠟⠐⠜⠈⠦⠈⠹⠏⠨⠜⠀⠸⠒⠀⠰⠟")?;
    return Ok(());

}

#[test]
fn example_11_5_1_2() -> Result<()> {
    let expr = "<math><mfrac><mrow><mi>d</mi><mi>y</mi></mrow><mrow><mi>d</mi><mi>x</mi></mrow></mfrac></math>";
    test_braille("UEB", expr, "⠰⠰⠷⠙⠽⠨⠌⠙⠭⠾")?;
    return Ok(());

}

#[test]
fn example_11_5_1_3() -> Result<()> {
    let expr = "<math><mi>f</mi><mo>'</mo><mo>(</mo><mi>x</mi><mo>)</mo></math>";
    // GTM / BANA 2026: grade 1 word indicator
    test_braille("UEB", expr, "⠋⠰⠶⠐⠣⠭⠐⠜")?;
    return Ok(());

}

#[test]
fn example_11_5_1_4() -> Result<()> {
    let expr = "<math><mfrac><mrow><mo>∂</mo><mi>y</mi></mrow><mrow><mo>∂</mo><mi>x</mi></mrow></mfrac></math>";
    test_braille("UEB", expr, "⠰⠰⠷⠈⠙⠽⠨⠌⠈⠙⠭⠾")?;
    return Ok(());

}

#[test]
fn example_11_5_2() -> Result<()> {
    let expr = "<math><msubsup><mo>∫</mo><mn>2</mn><mn>3</mn></msubsup><mo>(</mo><mn>2</mn><mi>x</mi><mo>+</mo><mn>1</mn><mo>)</mo><mo>d</mo><mi>x</mi>
        <mo>=</mo><msubsup><mfenced open='[' close=']'><mrow><msup><mi>x</mi><mn>2</mn></msup><mo>+</mo><mi>x</mi></mrow></mfenced><mn>2</mn><mn>3</mn></msubsup>
        <mo>=</mo><mo>(</mo><msup><mn>3</mn><mn>2</mn></msup><mo>+</mo><mn>3</mn><mo>)</mo><mo>-</mo><mo>(</mo><msup><mn>2</mn><mn>2</mn></msup><mo>+</mo><mn>2</mn><mo>)</mo>
        <mo>=</mo><mn>12</mn><mo>-</mo><mn>6</mn><mo>=</mo><mn>6</mn></math>";
    test_braille("UEB", expr, "⠰⠰⠮⠢⠼⠃⠔⠼⠉⠐⠣⠼⠃⠭⠐⠖⠼⠁⠐⠜⠙⠭⠀⠐⠶⠀⠨⠣⠭⠰⠔⠼⠃⠐⠖⠭⠨⠜⠢⠼⠃⠔⠼⠉⠀⠐⠶⠀⠐⠣⠼⠉⠔⠼⠃⠐⠖⠼⠉⠐⠜⠐⠤⠐⠣⠼⠃⠔⠼⠃⠐⠖⠼⠃⠐⠜⠀⠐⠶⠀⠼⠁⠃⠐⠤⠼⠋⠀⠐⠶⠀⠼⠋")?;
    return Ok(());

}

#[test]
fn example_11_5_3() -> Result<()> {
    // from MathJaX
    let expr = "<math>
        <msup><mrow/><mi>n</mi></msup>
        <msub><mi>C</mi><mi>r</mi></msub>
        <mo>=</mo>
        <mrow>
            <mo minsize='2.047em' maxsize='2.047em'>(</mo>
            <mfrac linethickness='0'><mi>n</mi><mi>r</mi></mfrac>
            <mo minsize='2.047em' maxsize='2.047em'>)</mo>
        </mrow>
        <mo>=</mo>
        <mfrac>
            <mrow><mi>n</mi><mo>!</mo></mrow>
            <mrow><mi>r</mi><mo>!</mo><mo stretchy='false'>(</mo><mi>n</mi><mo>−</mo><mi>r</mi><mo stretchy='false'>)</mo><mo>!</mo></mrow>
        </mfrac>
    </math>";
    // modified to use "shape" as recommended in a comment on this example
    test_braille("UEB", expr, "⠰⠰⠔⠝⠠⠉⠢⠗⠀⠐⠶⠀⠐⠣⠝⠰⠻⠗⠐⠜⠀⠐⠶⠀⠰⠰⠷⠝⠖⠨⠌⠗⠖⠐⠣⠝⠐⠤⠗⠐⠜⠖⠾")?;
    return Ok(());

}

#[test]
fn example_11_5_4() -> Result<()> {
    let expr = "<math><mi>a</mi><mo>∗</mo><mo>(</mo><mi>b</mi><mo>◦</mo><mi>c</mi><mo>)</mo>
        <mo>=</mo><mo>(</mo><mi>a</mi><mo>∗</mo><mi>b</mi><mo>)</mo><mo>◦</mo><mo>(</mo><mi>a</mi><mo>∗</mo><mi>c</mi><mo>)</mo></math>";
    test_braille("UEB", expr, "⠁⠐⠔⠐⠣⠃⠐⠴⠉⠐⠜⠀⠐⠶⠀⠐⠣⠁⠐⠔⠃⠐⠜⠐⠴⠐⠣⠁⠐⠔⠉⠐⠜")?;
    return Ok(());

}

#[test]
fn example_11_5_5_2() -> Result<()> {
    let expr = "<math>
    <msup>
      <mi>f</mi>
      <mrow> <mo>−</mo> <mn>1</mn> </mrow>
    </msup>
    <mo>:</mo>
    <mi>Y</mi>
    <mo>→</mo>
    <mi>X</mi>
  </math>";
    test_braille("UEB", expr, "⠰⠰⠰⠋⠔⠣⠐⠤⠼⠁⠜⠒⠀⠠⠽⠀⠳⠕⠀⠠⠭⠰⠄")?;
    return Ok(());

}

#[test]
fn example_11_5_5_3() -> Result<()> {
    // this comes from MathJax
    let expr = "<math>
        <mi mathvariant='normal'>∀</mi>
        <mi>y</mi>
        <mo>∈</mo>
        <mi>Y</mi>
        <mstyle scriptlevel='0'>  <mspace width='0.278em'></mspace> </mstyle>
        <mi mathvariant='normal'>∃</mi>
        <mi>x</mi>
        <mo>∈</mo>
        <mi>X</mi>
  </math>";
    test_braille("UEB", expr, "⠘⠁⠽⠀⠘⠑⠀⠰⠠⠽⠀⠘⠢⠭⠀⠘⠑⠀⠰⠠⠭")?;
    return Ok(());

}

#[test]
fn example_11_5_6() -> Result<()> {
    let expr = "<math> <mo>{</mo>
            <mo>(</mo> <mi>x</mi> <mo>,</mo> <mi>y</mi> <mo>)</mo>
            <mo>|</mo>
            <mi>x</mi> <mo>+</mo> <mi>y</mi> <mo>=</mo> <mn>6</mn>
            return Ok(());
        <mo>}</mo> </math>";
    test_braille("UEB", expr, "⠸⠣⠐⠣⠰⠭⠂⠀⠰⠽⠐⠜⠀⠸⠳⠀⠭⠐⠖⠽⠀⠐⠶⠀⠼⠋⠸⠜")?;
    return Ok(());
}

#[test]
fn example_11_6_math_variant() -> Result<()> {
    let expr = "<math><mi mathvariant='fraktur'>R</mi></math>";
    test_braille("UEB", expr, "⠈⠆⠰⠠⠗")?;
    return Ok(());

}

#[test]
fn example_11_6() -> Result<()> {
    let expr = "<math><mi>ℜ</mi></math>";
    test_braille("UEB", expr, "⠈⠆⠰⠠⠗")?;
    return Ok(());

}

#[test]
fn bar_over_12_1_1() -> Result<()> {
    let expr = "<math><mover><mi>x</mi><mo>_</mo></mover><mo>=</mo>
        <mfrac><mrow><mn>10</mn><mo>+</mo><mn>11</mn><mo>+</mo><mn>12</mn></mrow><mn>3</mn></mfrac></math>";
    test_braille("UEB", expr, "⠭⠰⠱⠀⠐⠶⠀⠰⠷⠼⠁⠚⠐⠖⠼⠁⠁⠐⠖⠼⠁⠃⠨⠌⠼⠉⠾")?;
    return Ok(());

}

#[test]
fn bar_under_12_1_2() -> Result<()> {
    let expr = "<math><munder><mrow><mi>x</mi><mo>+</mo><mi>y</mi></mrow><mo>_</mo></munder></math>";
    test_braille("UEB", expr, "⠰⠰⠣⠭⠐⠖⠽⠜⠠⠱")?;
    return Ok(());

}

#[test]
fn bar_menclose_12_1_2() -> Result<()> {
    let expr = "<math><menclose notation='bottom'><mi>x</mi><mo>+</mo><mi>y</mi></menclose></math>";
    test_braille("UEB", expr, "⠰⠰⠣⠭⠐⠖⠽⠜⠠⠱")?;
    return Ok(());

}

#[test]
fn dot_12_1_4() -> Result<()> {
    let expr = "<math><mn>0</mn><mo>.</mo><mover><mn>3</mn><mo>.</mo></mover></math>";
    test_braille("UEB", expr, "⠼⠚⠲⠣⠼⠉⠜⠘⠲")?;
    return Ok(());

}

#[test]
fn dot_12_1_5() -> Result<()> {
    let expr = "<math><mn>0</mn><mo>.</mo><mn>56</mn><mover><mn>1</mn><mo>˙</mo></mover>
            <mn>2</mn><mover><mn>3</mn><mo>˙</mo></mover></math>";
    test_braille("UEB", expr, "⠼⠚⠲⠑⠋⠣⠼⠁⠜⠘⠲⠼⠃⠣⠼⠉⠜⠘⠲")?;
    return Ok(());

}

#[test]
fn dot_12_1_6_single() -> Result<()> {
    let expr = "<math><mover><mi>x</mi><mo>˙</mo></mover></math>";
    test_braille("UEB", expr, "⠭⠘⠲")?;
    return Ok(());

}

#[test]
fn dot_12_1_6_double() -> Result<()> {
    let expr = "<math><mover><mi>x</mi><mo>¨</mo></mover></math>";
    test_braille("UEB", expr, "⠰⠰⠭⠨⠔⠣⠲⠲⠜")?;
    return Ok(());

}

#[test]
fn hat_12_1_7() -> Result<()> {
    let expr = "<math><mi>A</mi><mover><mi>B</mi><mo>^</mo></mover><mi>C</mi></math>";
    // GTM / BANA 2026: grade 1 word indicator
    test_braille("UEB", expr, "⠠⠁⠠⠃⠰⠐⠱⠠⠉")?;
    return Ok(());

}

#[test]
fn arrow_over_12() -> Result<()> {
    // This comes from https://uebonline.org/wp-content/uploads/2021/05/Unified-English-Braille-Extension-Maths-Training-Manual-First-Edition-Rev-4.pdf
    let expr = "<math><mover><mi>x</mi><mo>→</mo></mover></math>";
    test_braille("UEB", expr, "⠭⠰⠘⠱")?;
    return Ok(());

}

#[test]
fn arrow_under_12() -> Result<()> {
    // This comes from https://uebonline.org/wp-content/uploads/2021/05/Unified-English-Braille-Extension-Maths-Training-Manual-First-Edition-Rev-4.pdf
    let expr = "<math><munder><mi>x</mi><mo>→</mo></munder></math>";
    test_braille("UEB", expr, "⠭⠰⠠⠘⠱")?;
    return Ok(());

}

#[test]
fn bar_12_2_1() -> Result<()> {
    let expr = "<math><msup><mi>x</mi><mover><mi>y</mi><mo>¯</mo></mover></msup></math>";
    test_braille("UEB", expr, "⠰⠰⠭⠔⠣⠽⠱⠜")?;
    return Ok(());

}

#[test]
fn bar_12_2_2() -> Result<()> {
    let expr = "<math><mover><msup><mi>x</mi><mi>y</mi></msup><mo>¯</mo></mover></math>";
    test_braille("UEB", expr, "⠰⠰⠣⠭⠔⠽⠜⠱")?;
    return Ok(());

}

#[test]
fn shape_14_1_1_1() -> Result<()> {
    let expr = "<math><mo>△</mo><mo>&#xA0;</mo><mtext>ABC</mtext></math>";
    test_braille("UEB", expr, "⠰⠫⠼⠉⠀⠠⠠⠁⠃⠉")?;
    return Ok(());

}

#[test]
fn shape_14_1_2_1() -> Result<()> {
    let expr = "<math><mo>△</mo><mtext>ABC</mtext></math>";
    test_braille("UEB", expr, "⠰⠫⠼⠉⠱⠠⠠⠁⠃⠉")?;
    return Ok(());

}

#[test]
fn shape_14_1_2_2() -> Result<()> {
    // the <mo> for the shapes are wrong -- but it isn't clear what they should be (from WIRIS editor)
    let expr = "<math><mo>{</mo><mo>□</mo><mo>,</mo>
                            <mo>&#xA0;</mo><mo>◍</mo><mo>,</mo>
                            <mo>&#xA0;</mo><mo>▲</mo><mo>,</mo>
                            return Ok(());
                            <mo>&#xA0;</mo><mo>▧</mo><mo>&#xA0;</mo><mo>…</mo><mo>}</mo></math>";
    test_braille("UEB", expr, "⠸⠣⠰⠫⠼⠙⠱⠂⠀⠨⠫⠿⠱⠂⠀⠸⠫⠼⠉⠱⠂⠀⠨⠫⠼⠙⠀⠲⠲⠲⠸⠜")?;
    return Ok(());
}

#[test]
fn binomial_14_3_3_2() -> Result<()> {
    let expr = "<math><mfenced><mfrac linethickness='0'><mi>n</mi><mi>r</mi></mfrac></mfenced></math>";
    test_braille("UEB", expr, "⠐⠣⠝⠰⠻⠗⠐⠜")?;
    return Ok(());

}

#[test]
fn binomial_14_3_3_2_mtable() -> Result<()> {
    let expr = "<math><mrow intent='binomial($n,$r)'>
            <mo>(</mo>
                <mtable>
                <mtr><mtd><mi arg='n'>n</mi></mtd></mtr>
                <mtr><mtd><mi arg='r'>r</mi></mtd></mtr>
                </mtable>
            <mo>)</mo>
        </mrow></math>";
    test_braille("UEB", expr, "⠐⠣⠝⠰⠻⠗⠐⠜")?;
    return Ok(());

}

#[test]
fn matrix_15_2_1() -> Result<()> {
    // GTM 15.2 first example: I = [[1,0],[0,1]] with enlarged parentheses (one-line encoding).
    let expr = r#"<math><mi>I</mi><mo>=</mo>
       <mrow><mo>(</mo><mtable>
          <mtr><mtd><mn>1</mn></mtd><mtd><mn>0</mn></mtd></mtr>
          <mtr><mtd><mn>0</mn></mtd><mtd><mn>1</mn></mtd></mtr>
        </mtable><mo>)</mo>
      </mrow></math>"#;
    test_braille("UEB", expr, "⠠⠊⠀⠐⠶⠀⠠⠐⠣⠼⠁⠀⠼⠚⠠⠐⠜⠸⠀⠠⠐⠣⠼⠚⠀⠼⠁⠠⠐⠜")?;
    return Ok(());
}

#[test]
fn matrix_15_2_2() -> Result<()> {
    // GTM 15.2 second example: matrix multiplication (linearized with `⠸⠀` between rows).
    // [1 2 3; 4 5 6] [1 2; -3 4; 5 -6]
    let expr = r#"<math>
      <mrow><mo>[</mo><mtable>
        <mtr><mtd><mn>1</mn></mtd><mtd><mn>2</mn></mtd><mtd><mn>3</mn></mtd></mtr>
        <mtr><mtd><mn>4</mn></mtd><mtd><mn>5</mn></mtd><mtd><mn>6</mn></mtd></mtr>
      </mtable><mo>]</mo></mrow>
      <mrow><mo>[</mo><mtable>
        <mtr><mtd><mn>1</mn></mtd><mtd><mn>2</mn></mtd></mtr>
        <mtr><mtd><mo>-</mo><mn>3</mn></mtd><mtd><mn>4</mn></mtd></mtr>
        <mtr><mtd><mn>5</mn></mtd><mtd><mo>-</mo><mn>6</mn></mtd></mtr>
      </mtable><mo>]</mo></mrow>
    </math>"#;
    // example has G1 passage mode start/end, but they are not needed and not included.
    test_braille(
        "UEB",
        expr,
        "⠠⠨⠣⠼⠁⠀⠼⠃⠀⠼⠉⠠⠨⠜⠸⠀⠠⠨⠣⠼⠙⠀⠼⠑⠀⠼⠋⠠⠨⠜⠠⠨⠣⠼⠁⠀⠼⠃⠠⠨⠜⠸⠀⠠⠨⠣⠐⠤⠼⠉⠀⠼⠙⠠⠨⠜⠸⠀⠠⠨⠣⠼⠑⠀⠐⠤⠼⠋⠠⠨⠜",
    )?;
    return Ok(());
}

#[test]
fn matrix_15_2_3() -> Result<()> {
    // GTM 15.2 third example: (a −b ; −c d), linearized with `⠸⠀` between rows.
    // Spec spatial braille (ASCII):  ,"< a "-b,"> / ,"<"-c d,">
    // Spec spatial (Unicode):       ⠠⠐⠣⠀⠁⠀⠐⠤⠃⠠⠐⠜ / ⠠⠐⠣⠐⠤⠉⠀⠙⠠⠐⠜
    // (extra space after the first-row open paren aligns columns so the minus stands out)
    let expr = r#"<math><mrow><mo>(</mo><mtable>
        <mtr><mtd><mi>a</mi></mtd><mtd><mrow><mo>-</mo><mi>b</mi></mrow></mtd></mtr>
        <mtr><mtd><mrow><mo>-</mo><mi>c</mi></mrow></mtd><mtd><mi>d</mi></mtd></mtr>
      </mtable><mo>)</mo></mrow></math>"#;
    test_braille("UEB", expr, "⠠⠐⠣⠁⠀⠐⠤⠃⠠⠐⠜⠸⠀⠠⠐⠣⠐⠤⠉⠀⠙⠠⠐⠜")?;
    return Ok(());
}

#[test]
fn determinant_15_3_1() -> Result<()> {
    // GTM 15.3: |P| = |a b; c d| = ad − bc (determinant linearized with `⠸⠀` between rows).
    let expr = r#"<math>
      <mrow><mo>|</mo><mi>P</mi><mo>|</mo></mrow>
      <mo>=</mo>
      <mrow><mo>|</mo><mtable>
        <mtr><mtd><mi>a</mi></mtd><mtd><mi>b</mi></mtd></mtr>
        <mtr><mtd><mi>c</mi></mtd><mtd><mi>d</mi></mtd></mtr>
      </mtable><mo>|</mo></mrow>
      <mo>=</mo>
      <mrow><mi>a</mi><mi>d</mi><mo>-</mo><mi>b</mi><mi>c</mi></mrow>
    </math>"#;
    test_braille(
        "UEB",
        expr,
        "⠸⠳⠠⠏⠸⠳⠀⠐⠶⠀⠠⠸⠳⠁⠀⠃⠠⠸⠳⠸⠀⠠⠸⠳⠉⠀⠙⠠⠸⠳⠀⠐⠶⠀⠁⠙⠐⠤⠃⠉",
    )?;
    return Ok(());
}

#[test]
fn omission_15_4_1() -> Result<()> {
    // GTM 15.4: determinant with omission dots (linearized with `⠸⠀` between rows).
    // Spec spatial (ASCII):
    //   ;;;,_|a5#aa a5#ab 444 a5<#an>,_|
    //    ,_|a5#ba a5#bb 444 a5<#bn>,_|
    //    ,_| 4 4 444 4 ,_|
    //    ,_|a5<m#a> a5<m#b> 444 a5<mn> ,_|;'
    // Spec uses . (⠄) / ⋯ (⠄⠄⠄) for omission dots; U+2024 / U+22EF in the MathML.
    let expr = r#"<math><mrow><mo>|</mo><mtable>
      <mtr>
        <mtd><msub><mi>a</mi><mn>11</mn></msub></mtd>
        <mtd><msub><mi>a</mi><mn>12</mn></msub></mtd>
        <mtd><mo>⋯</mo></mtd>
   
        <mtd><msub><mi>a</mi><mrow><mn>1</mn><mi>n</mi></mrow></msub></mtd>
      </mtr>
      <mtr>
        <mtd><msub><mi>a</mi><mn>21</mn></msub></mtd>
        <mtd><msub><mi>a</mi><mn>22</mn></msub></mtd>
        <mtd><mo>⋯</mo></mtd>
        <mtd><msub><mi>a</mi><mrow><mn>2</mn><mi>n</mi></mrow></msub></mtd>
      </mtr>
      <mtr>
        <mtd><mo>.</mo></mtd>
        <mtd><mo>.</mo></mtd>
        <mtd><mo>⋯</mo></mtd>
        <mtd><mo>.</mo></mtd>
      </mtr>
      <mtr>
        <mtd><msub><mi>a</mi><mrow><mi>m</mi><mn>1</mn></mrow></msub></mtd>
        <mtd><msub><mi>a</mi><mrow><mi>m</mi><mn>2</mn></mrow></msub></mtd>
        <mtd><mo>⋯</mo></mtd>
        <mtd><msub><mi>a</mi><mrow><mi>m</mi><mi>n</mi></mrow></msub></mtd>
      </mtr>
    </mtable><mo>|</mo></mrow></math>"#;
    test_braille(
        "UEB",
        expr,
        "⠰⠰⠰⠠⠸⠳⠁⠢⠼⠁⠁⠀⠁⠢⠼⠁⠃⠀⠄⠄⠄⠀⠁⠢⠣⠼⠁⠝⠜⠠⠸⠳⠸⠀⠠⠸⠳⠁⠢⠼⠃⠁⠀⠁⠢⠼⠃⠃⠀⠄⠄⠄⠀⠁⠢⠣⠼⠃⠝⠜⠠⠸⠳⠸⠀⠠⠸⠳⠲⠀⠲⠀⠄⠄⠄⠀⠲⠠⠸⠳⠸⠀⠠⠸⠳⠁⠢⠣⠍⠼⠁⠜⠀⠁⠢⠣⠍⠼⠃⠜⠀⠄⠄⠄⠀⠁⠢⠣⠍⠝⠜⠠⠸⠳⠰⠄",
    )?;
    return Ok(());
}

#[test]
fn chem_16_2_8() -> Result<()> {
    let expr = "<math><mi>Ca</mi><msub><mrow><mo>(</mo><mi>OH</mi><mo>)</mo></mrow><mn>2</mn></msub></math>";
    // GTM / BANA 2026: single G1 symbol for subscript (no word indicator)
    test_braille("UEB", expr, "⠠⠉⠁⠐⠣⠠⠕⠠⠓⠐⠜⠰⠢⠼⠃")?;
    return Ok(());

}

#[test]
fn chem_16_2_9() -> Result<()> {
    // from mhchem -- \ce{CuSO4·5H2O}
    let expr = "<math>
        <mrow>
        <mrow>
            <mi data-mjx-auto-op='false'>CuSO</mi>
        </mrow>
        <msub>
            <mrow>
            <mrow>
                <mpadded width='0'>
                <mphantom>
                    <mi>A</mi>
                </mphantom>
                </mpadded>
            </mrow>
            </mrow>
            <mrow>
            <mrow>
                <mpadded height='0'>
                <mn>4</mn>
                </mpadded>
            </mrow>
            </mrow>
        </msub>
        <mstyle scriptlevel='0'>
            <mspace width='0.167em'></mspace>
        </mstyle>
        <mrow>
            <mo>⋅</mo>
        </mrow>
        <mstyle scriptlevel='0'>
            <mspace width='0.167em'></mspace>
        </mstyle>
        <mn>5</mn>
        <mstyle scriptlevel='0'>
            <mspace width='0.167em'></mspace>
        </mstyle>
        <mrow>
            <mi mathvariant='normal'>H</mi>
        </mrow>
        <msub>
            <mrow>
            <mrow>
                <mpadded width='0'>
                <mphantom>
                    <mi>A</mi>
                </mphantom>
                </mpadded>
            </mrow>
            </mrow>
            <mrow>
            <mrow>
                <mpadded height='0'>
                <mn>2</mn>
                </mpadded>
            </mrow>
            </mrow>
        </msub>
        <mrow>
            <mi mathvariant='normal'>O</mi>
        </mrow>
        </mrow>
    </math>";
    // GTM / BANA 2026: single G1 symbol for first subscript (no word indicator)
    test_braille("UEB", expr, "⠠⠉⠥⠠⠎⠠⠕⠰⠢⠼⠙⠐⠲⠼⠑⠠⠓⠢⠼⠃⠠⠕")?;
    return Ok(());

}

#[test]
fn chem_16_2_10() -> Result<()> {
    let expr = "<math><mmultiscripts><mi mathvariant='normal'>H</mi><none/><mo>+</mo></mmultiscripts></math>";
    test_braille("UEB", expr, "⠠⠓⠰⠔⠐⠖")?;
    return Ok(());

}

#[test]
fn chem_16_2_11() -> Result<()> {
    let expr = "<math>
        <mi mathvariant='normal'>S</mi>
        <mmultiscripts> <mi mathvariant='normal'>O</mi> <mn>4</mn> <mrow><mo>-</mo><mo>-</mo></mrow>  </mmultiscripts>
    </math>";
    test_braille("UEB", expr, "⠠⠎⠠⠕⠰⠢⠼⠙⠔⠣⠐⠤⠐⠤⠜")?;
    return Ok(());

}

#[test]
fn chem_16_2_12() -> Result<()> {
    // from MathJax/mhchem V3
    let expr = "<math>
            <mrow data-mjx-texclass='ORD'>
            <mi mathvariant='normal'>R</mi>
            <mstyle scriptlevel='0'><mspace width='0.167em'/></mstyle>
            <mo>⋅</mo>
            <mstyle scriptlevel='0'><mspace width='0.167em'/></mstyle>
            <mi data-mjx-auto-op='false'>CH</mi>
            <mo stretchy='false'>(</mo>
            <mi data-mjx-auto-op='false'>OH</mi>
            <mo stretchy='false'>)</mo>
            <mstyle scriptlevel='0'><mspace width='0.167em'/></mstyle>
            <mo>⋅</mo>
            <mstyle scriptlevel='0'><mspace width='0.167em'/></mstyle>
            <mi data-mjx-auto-op='false'>CH</mi>
            <msub>
                <mpadded width='0'><mphantom><mi>A</mi></mphantom></mpadded>
                <mpadded height='0'><mn>2</mn></mpadded>
            </msub>
            <mstyle scriptlevel='0'><mspace width='0.167em'/></mstyle>
            <mo>⋅</mo>
            <mstyle scriptlevel='0'><mspace width='0.167em'/></mstyle>
            <mi data-mjx-auto-op='false'>CH</mi>
            <msub>
                <mpadded width='0'><mphantom><mi>A</mi></mphantom></mpadded>
                <mpadded height='0'><mn>2</mn></mpadded>
            </msub>
            <mstyle scriptlevel='0'><mspace width='0.167em'/></mstyle>
            <mo>⋅</mo>
            <mstyle scriptlevel='0'><mspace width='0.167em'/></mstyle>
            <mi data-mjx-auto-op='false'>CO</mi>
            <msub>
                <mpadded width='0'><mphantom><mi>A</mi></mphantom></mpadded>
                <mpadded height='0'><mn>2</mn></mpadded>
            </msub>
            <mi mathvariant='normal'>H</mi>
            </mrow>
        </math>";
    // GTM / BANA 2026: G2 with inline G1 symbols (not forced word mode)
    test_braille("UEB", expr, "⠠⠠⠠⠗⠐⠲⠡⠐⠣⠕⠓⠐⠜⠐⠲⠡⠰⠢⠼⠃⠐⠲⠉⠓⠢⠼⠃⠐⠲⠉⠕⠢⠼⠃⠰⠓⠠⠄")?;
    return Ok(());

}

#[test]
fn chem_16_2_13() -> Result<()> {
    let expr = "<math>
        <mmultiscripts><mi>Fe</mi><none/><mi>III</mi></mmultiscripts>
        <mmultiscripts><mi>Cl</mi><mn>3</mn><none/></mmultiscripts>
    </math>";
    test_braille("UEB", expr, "⠰⠰⠠⠋⠑⠔⠣⠠⠠⠊⠊⠊⠜⠠⠉⠇⠢⠼⠉")?;
    return Ok(());

}

#[test]
fn chem_16_3_1() -> Result<()> {
    // see also pre_msubsup_7_8_1 which uses mmultiscripts
    // from MathJax
    let expr = "<math><msubsup><mrow/><mn>92</mn><mn>238</mn></msubsup><mi>U</mi></math>";
    test_braille("UEB", expr, "⠰⠢⠼⠊⠃⠔⠼⠃⠉⠓⠠⠥")?;
    return Ok(());

}

#[test]
fn chem_16_4_2() -> Result<()> {
    // from MathJax
    let expr = r#"<math><msup><mrow/><mn>1</mn></msup><msub><mi mathvariant="normal">S</mi><mn>0</mn></msub></math>"#;
    test_braille("UEB", expr, "⠰⠔⠼⠁⠠⠎⠢⠼⠚")?;
    return Ok(());

}

#[test]
fn chem_16_4_3() -> Result<()> {
    // from MathJax using \mathrm{}
    let expr = r#"<math xmlns="http://www.w3.org/1998/Math/MathML" display="block">
        <mn>4</mn><msup><mi mathvariant="normal">d</mi><mn>9</mn></msup>
        <mn>5</mn><msup><mi mathvariant="normal">s</mi><mn>2</mn></msup>
        <mstyle scriptlevel="0"><mspace width="0.278em"/></mstyle>
        <msup><mrow/><mn>2</mn></msup>
        <msub><mi mathvariant="normal">D</mi><mrow><mn>5</mn><mo>/</mo><mn>2</mn></mrow></msub>
    </math>"#;
    test_braille("UEB", expr, "⠼⠙⠰⠙⠔⠼⠊⠼⠑⠎⠔⠼⠃⠀⠰⠔⠼⠃⠠⠙⠢⠣⠼⠑⠸⠌⠼⠃⠜")?;
    return Ok(());

}

#[test]
fn chem_16_5_1() -> Result<()> {
    // from MathJax v3 mhchem, but substituted short arrow because that is what example uses (although it shouldn't)
    let expr = r#"
        <math>
        <mrow>
            <mn>2</mn>
            <mstyle scriptlevel="0">
            <mspace width="0.167em"></mspace>
            </mstyle>
            <mi>NaOH</mi>
            <mrow></mrow>
            <mo>+</mo>
            <mrow></mrow>
            <mi mathvariant="normal">H</mi>
            <msub>
            <mpadded width="0">
                <mphantom>
                <mi>A</mi>
                </mphantom>
            </mpadded>
            <mpadded height="0">
                <mn>2</mn>
            </mpadded>
            </msub>
            <mi>SO</mi>
            <msub>
            <mpadded width="0">
                <mphantom>
                <mi>A</mi>
                </mphantom>
            </mpadded>
            <mpadded height="0">
                <mn>4</mn>
            </mpadded>
            </msub>
            <mrow></mrow>
            <mo stretchy="false">→</mo>
            <mrow></mrow>
            <mi>Na</mi>
            <msub>
            <mpadded width="0">
                <mphantom>
                <mi>A</mi>
                </mphantom>
            </mpadded>
            <mpadded height="0">
                <mn>2</mn>
            </mpadded>
            </msub>
            <mi>SO</mi>
            <msub>
            <mpadded width="0">
                <mphantom>
                <mi>A</mi>
                </mphantom>
            </mpadded>
            <mpadded height="0">
                <mn>4</mn>
            </mpadded>
            </msub>
            <mrow></mrow>
            <mo>+</mo>
            <mrow></mrow>
            <mn>2</mn>
            <mstyle scriptlevel="0">
            <mspace width="0.167em"></mspace>
            </mstyle>
            <mi mathvariant="normal">H</mi>
            <msub>
            <mpadded width="0">
                <mphantom>
                <mi>A</mi>
                </mphantom>
            </mpadded>
            <mpadded height="0">
                <mn>2</mn>
            </mpadded>
            </msub>
            <mi mathvariant="normal">O</mi>
        </mrow>
        </math>
        "#;
    test_braille("UEB", expr, "⠼⠃⠠⠝⠁⠠⠕⠠⠓⠐⠖⠠⠓⠢⠼⠃⠠⠎⠠⠕⠢⠼⠙⠀⠰⠳⠕⠀⠠⠝⠁⠰⠢⠼⠃⠠⠎⠠⠕⠢⠼⠙⠐⠖⠼⠃⠠⠓⠢⠼⠃⠠⠕")?;
    return Ok(());

}

#[test]
fn chem_16_5_2() -> Result<()> {
    // from WIRIS
    let expr = r#"
    <math>
        <msub><mi mathvariant="normal">N</mi><mn>2</mn></msub>
        <munderover><mo>→</mo><mtext>Haber&#xA0;process</mtext><msub><mi mathvariant="normal">H</mi><mn>2</mn></msub></munderover>
        <mi mathvariant="normal">N</mi>
        <msub><mi mathvariant="normal">H</mi><mn>3</mn></msub>
    </math>
        "#;
    // GTM has the order be over followed by under. This is opposite of what is shown in 7.9 (which are  large op examples).
    // I have spoken with several people about this, including ICEB committee members who all feel this example is a mistake.
    // I have adjusted the expected output
    test_braille("UEB", expr, "⠰⠰⠰⠠⠝⠢⠼⠃⠀⠳⠕⠨⠢⠣⠠⠓⠁⠃⠑⠗⠀⠏⠗⠕⠉⠑⠎⠎⠜⠨⠔⠣⠠⠓⠢⠼⠃⠜⠀⠠⠝⠠⠓⠢⠼⠉⠰⠄")?;
    return Ok(());

}

#[test]
fn chem_16_5_3() -> Result<()> {
    // from MathJax v3 mhchem, uses "\;" before parens because example adds a space there (although it shouldn't)
    let expr = r#"
        <math>
            <mrow>
            <mi mathvariant="normal">H</mi>
            <msub>
                <mpadded width="0"><mphantom><mi>A</mi></mphantom></mpadded>      <mpadded height="0">
                <mn>2</mn>
                </mpadded>
            </msub>
            <mstyle scriptlevel="0"><mspace width="0.167em"/></mstyle>
            <mspace width="0.111em"></mspace>
            <mo stretchy="false">(</mo>
            <mi mathvariant="normal">g</mi>
            <mo stretchy="false">)</mo>
            <mrow/>
            <mo>+</mo>
            <mrow/>
            <mi mathvariant="normal">I</mi>
            <msub>
                <mpadded width="0"><mphantom><mi>A</mi></mphantom></mpadded>      <mpadded height="0">
                <mn>2</mn>
                </mpadded>
            </msub>
            <mstyle scriptlevel="0"><mspace width="0.167em"/></mstyle>
            <mspace width="0.111em"></mspace>
            <mo stretchy="false">(</mo>
            <mi mathvariant="normal">s</mi>
            <mo stretchy="false">)</mo>
            <mrow/>
            <mo>=</mo>
            <mrow/>
            <mn>2</mn>
            <mstyle scriptlevel="0"><mspace width="0.167em"/></mstyle>
            <mi>HI</mi>
            <mstyle scriptlevel="0"><mspace width="0.167em"/></mstyle>
            <mspace width="0.111em"></mspace>
            <mo stretchy="false">(</mo>
            <mi mathvariant="normal">g</mi>
            <mo stretchy="false">)</mo>
            </mrow>
        </math>
          "#;
    test_braille("UEB", expr, "⠰⠰⠰⠠⠓⠢⠼⠃⠀⠐⠣⠛⠐⠜⠐⠖⠠⠊⠢⠼⠃⠀⠐⠣⠎⠐⠜⠀⠐⠶⠀⠼⠃⠠⠓⠠⠊⠀⠐⠣⠛⠐⠜⠰⠄")?;
    return Ok(());

}

#[test]
fn chem_16_5_4() -> Result<()> {
    // from MathJax v4 mhchem, but substituted short arrow because that is what example uses (although it shouldn't) 
    let expr = r#"
    <math xmlns="http://www.w3.org/1998/Math/MathML" data-latex="\ce{HNCO + ROH -&gt; NH2.CO.OR -&gt; NH2CO.NH.CO2R}" display="block">
    <mrow data-latex="{\mathrm{HNCO} {}+{} \mathrm{ROH} {}\mathrel{\longrightarrow}{} \mathrm{NH}{\vphantom{A}}_{\smash[t]{2}}\,{\cdot}\,\mathrm{CO}\,{\cdot}\,\mathrm{OR} {}\mathrel{\longrightarrow}{} \mathrm{NH}{\vphantom{A}}_{\smash[t]{2}}\mathrm{CO}\,{\cdot}\,\mathrm{NH}\,{\cdot}\,\mathrm{CO}{\vphantom{A}}_{\smash[t]{2}}\mathrm{R}}">
      <mrow>
        <mrow data-latex="\mathrm{HNCO}">
          <mi data-latex="HNCO">HNCO</mi>
        </mrow>
        <mo data-latex="+">+</mo>
        <mrow data-latex="{}"></mrow>
        <mrow data-latex="\mathrm{ROH}">
          <mi data-latex="ROH">ROH</mi>
        </mrow>
      </mrow>
      <mrow data-mjx-texclass="REL" data-latex="\mathrel{\longrightarrow}">
        <mo stretchy="false" data-latex="\longrightarrow">→</mo>
      </mrow>
      <mrow>
        <mrow>
          <mrow data-latex="\mathrm{NH}">
            <mi data-latex="NH">NH</mi>
          </mrow>
          <mo>&#x2062;</mo>
          <msub data-latex="{\vphantom{A}}_{\smash[t]{2}}">
            <mrow data-latex="{\vphantom{A}}">
              <mrow data-latex="\vphantom{A}">
                <mpadded width="0">
                  <mphantom>
                    <mi data-latex="A">A</mi>
                  </mphantom>
                </mpadded>
              </mrow>
            </mrow>
            <mrow data-latex="{\smash[t]{2}}">
              <mrow data-latex="\smash[t]{2}">
                <mpadded height="0">
                  <mn data-latex="2">2</mn>
                </mpadded>
              </mrow>
            </mrow>
          </msub>
        </mrow>
        <mstyle scriptlevel="0" data-latex="\,">
          <mspace width="0.167em"></mspace>
        </mstyle>
        <mrow data-latex="{\cdot}">
          <mo data-latex="\cdot">⋅</mo>
        </mrow>
        <mstyle scriptlevel="0" data-latex="\,">
          <mspace width="0.167em"></mspace>
        </mstyle>
        <mrow data-latex="\mathrm{CO}">
          <mi data-latex="CO">CO</mi>
        </mrow>
        <mstyle scriptlevel="0" data-latex="\,">
          <mspace width="0.167em"></mspace>
        </mstyle>
        <mrow data-latex="{\cdot}">
          <mo data-latex="\cdot">⋅</mo>
        </mrow>
        <mstyle scriptlevel="0" data-latex="\,">
          <mspace width="0.167em"></mspace>
        </mstyle>
        <mrow data-latex="\mathrm{OR}">
          <mi data-latex="OR">OR</mi>
        </mrow>
      </mrow>
      <mrow data-mjx-texclass="REL" data-latex="\mathrel{\longrightarrow}">
        <mo stretchy="false" data-latex="\longrightarrow">→</mo>
      </mrow>
      <mrow>
        <mrow>
          <mrow data-latex="\mathrm{NH}">
            <mi data-latex="NH">NH</mi>
          </mrow>
          <mo>&#x2062;</mo>
          <msub data-latex="{\vphantom{A}}_{\smash[t]{2}}">
            <mrow data-latex="{\vphantom{A}}">
              <mrow data-latex="\vphantom{A}">
                <mpadded width="0">
                  <mphantom>
                    <mi data-latex="A">A</mi>
                  </mphantom>
                </mpadded>
              </mrow>
            </mrow>
            <mrow data-latex="{\smash[t]{2}}">
              <mrow data-latex="\smash[t]{2}">
                <mpadded height="0">
                  <mn data-latex="2">2</mn>
                </mpadded>
              </mrow>
            </mrow>
          </msub>
          <mo>&#x2062;</mo>
          <mrow data-latex="\mathrm{CO}">
            <mi data-latex="CO">CO</mi>
          </mrow>
        </mrow>
        <mstyle scriptlevel="0" data-latex="\,">
          <mspace width="0.167em"></mspace>
        </mstyle>
        <mrow data-latex="{\cdot}">
          <mo data-latex="\cdot">⋅</mo>
        </mrow>
        <mstyle scriptlevel="0" data-latex="\,">
          <mspace width="0.167em"></mspace>
        </mstyle>
        <mrow data-latex="\mathrm{NH}">
          <mi data-latex="NH">NH</mi>
        </mrow>
        <mstyle scriptlevel="0" data-latex="\,">
          <mspace width="0.167em"></mspace>
        </mstyle>
        <mrow data-latex="{\cdot}">
          <mo data-latex="\cdot">⋅</mo>
        </mrow>
        <mstyle scriptlevel="0" data-latex="\,">
          <mspace width="0.167em"></mspace>
        </mstyle>
        <mrow>
          <mrow data-latex="\mathrm{CO}">
            <mi data-latex="CO">CO</mi>
          </mrow>
          <mo>&#x2062;</mo>
          <msub data-latex="{\vphantom{A}}_{\smash[t]{2}}">
            <mrow data-latex="{\vphantom{A}}">
              <mrow data-latex="\vphantom{A}">
                <mpadded width="0">
                  <mphantom>
                    <mi data-latex="A">A</mi>
                  </mphantom>
                </mpadded>
              </mrow>
            </mrow>
            <mrow data-latex="{\smash[t]{2}}">
              <mrow data-latex="\smash[t]{2}">
                <mpadded height="0">
                  <mn data-latex="2">2</mn>
                </mpadded>
              </mrow>
            </mrow>
          </msub>
          <mo>&#x2062;</mo>
          <mrow data-latex="\mathrm{R}">
            <mi mathvariant="normal" data-latex="R">R</mi>
          </mrow>
        </mrow>
      </mrow>
    </mrow>
  </math>
            "#;
    test_braille("UEB", expr, "⠰⠰⠰⠠⠠⠠⠓⠝⠉⠕⠐⠖⠗⠕⠓⠀⠳⠕⠀⠝⠓⠢⠼⠃⠐⠲⠉⠕⠐⠲⠕⠗⠀⠳⠕⠀⠝⠓⠢⠼⠃⠰⠉⠕⠐⠲⠝⠓⠐⠲⠉⠕⠢⠼⠃⠗⠠⠄⠰⠄")?;
    return Ok(());

}

#[test]
fn chem_16_5_5() -> Result<()> {
    // from MathJax v3 mhchem
    let expr = r#"
        <math>
            <mrow>
                <mi>Pb</mi>
                <msup><mpadded width="0"><mphantom><mi>A</mi></mphantom></mpadded><mrow><mo>+</mo><mo>+</mo></mrow></msup>
                <mrow>    </mrow>
                <mo>+</mo>
                <mrow>    </mrow>
                <mn>2</mn>
                <mstyle scriptlevel="0"><mspace width="0.167em"/></mstyle>
                <mi mathvariant="normal">e</mi>
                <mrow>    </mrow>
                <mover>
                    <mpadded height="0" depth="0">
                        <mo stretchy="false">↽</mo>
                        <mstyle scriptlevel="0"><mspace width="-0.167em"/></mstyle>
                        <mstyle scriptlevel="0"><mspace width="-0.167em"/></mstyle>
                        <mo>−</mo>
                    </mpadded>
                    <mstyle displaystyle="false" scriptlevel="0">
                        <mo>−</mo>
                        <mstyle scriptlevel="0"><mspace width="-0.167em"/></mstyle>
                        <mstyle scriptlevel="0"><mspace width="-0.167em"/></mstyle>
                        <mo stretchy="false">⇀</mo>
                    </mstyle>
                </mover>
                <mrow>    </mrow>
                <mi>Pb</mi>
            </mrow>
        </math>
        "#;
    test_braille("UEB", expr, "⠰⠰⠠⠏⠃⠔⠣⠐⠖⠐⠖⠜⠐⠖⠼⠃⠰⠑⠀⠰⠘⠸⠶⠀⠠⠏⠃")?;
    return Ok(());

}
