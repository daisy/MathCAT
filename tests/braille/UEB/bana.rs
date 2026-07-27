// UEB tests for GitHub issue #529, from the second (BANA) source document:
// "Guidance on Transcribing Mathematics and Science in UEB", Section 5
// "Preference on grade 1 indicators with technical expressions" (RUEB Section 5.9; GTM Section 1.7, 2025 update).
//
// Notes on scope:
// - Examples 5-1 (x + y = 6), 5-7 (x^2 + y^2 = C), and 5-10 (a/b + c/d) are exact duplicates
//   of bana_5_1, bana_5_2, and bana_5_3 already in iceb.rs (same source examples, from the
//   2019 BANA doc), so they aren't repeated here.
// - Example 5-14 (a^n * a^m = a^(n+m)) uses the *same* MathML as the existing bana_5_4 test in
//   iceb.rs, but this document gives it *different* expected braille: two separate grade 1 word
//   indicators instead of one grade 1 passage. This is kept deliberately (as
//   bana2025_5_14_power_multiplication) as a direct old-vs-new contrast for issue #529 --
//   bana_5_4 documents the old (pre-2025) behavior, this documents the new one.
// - Example 5-9 ("24. The slope of l1 is 5/3 and l1 || l2.") is skipped: it's not tagged to the
//   2025 update, and the math is too interleaved with sentence prose to cleanly isolate per the
//   "math parts only" scoping used elsewhere.
// - Examples 5-5 and 5-6 are matrices. The visual layout came through as garbled OCR text, so
//   the MathML here is a best-effort reconstruction from the (reliable, copy-pasted) braille
//   alone. If these fail in a way that doesn't look like an indicator-choice issue, check the
//   PDF directly before trusting the MathML.
use crate::common::*;
use anyhow::Result;

// 5.1 No grade 1 indicators needed

#[test]
fn bana2025_5_2_surface_area() -> Result<()> {
    let expr = "<math><mtext>Surface area</mtext><mo>=</mo>
                <mn>2</mn><mi>l</mi><mi>h</mi><mo>+</mo>
                <mn>2</mn><mi>l</mi><mi>w</mi><mo>+</mo>
                <mn>2</mn><mi>w</mi><mi>h</mi></math>";
    test_braille("UEB", expr, "⠠⠎⠥⠗⠋⠁⠉⠑⠀⠜⠑⠁⠀⠐⠶⠀⠼⠃⠇⠓⠐⠖⠼⠃⠇⠺⠐⠖⠼⠃⠺⠓")?;
    return Ok(());
}

// 5.2 Grade 1 symbol indicators needed

#[test]
fn bana2025_5_3_nacl_h2o() -> Result<()> {
    let expr = "<math><mi mathvariant='normal'>Na</mi><mi mathvariant='normal'>Cl</mi><mo>+</mo>
                <msub><mi mathvariant='normal'>H</mi><mn>2</mn></msub><mi mathvariant='normal'>O</mi></math>";
    test_braille("UEB", expr, "⠠⠝⠁⠠⠉⠇⠐⠖⠠⠓⠰⠢⠼⠃⠠⠕")?;
    return Ok(());
}

#[test]
fn bana2025_5_4_fraction_5x2_over_y3z() -> Result<()> {
    let expr = "<math><mfrac>
                    <mrow><mn>5</mn><msup><mi>x</mi><mn>2</mn></msup></mrow>
                    <mrow><msup><mi>y</mi><mn>3</mn></msup><mi>z</mi></mrow>
                </mfrac></math>";
    test_braille("UEB", expr, "⠰⠷⠼⠑⠭⠔⠼⠃⠨⠌⠽⠔⠼⠉⠵⠾")?;
    return Ok(());
}

#[test]
fn bana2025_5_5_matrix_3x2() -> Result<()> {
    // best-effort reconstruction -- see file header note
    let expr = "<math><mo>(</mo><mtable>
                    <mtr><mtd><mi>a</mi></mtd><mtd><mi>b</mi></mtd><mtd><mi>c</mi></mtd></mtr>
                    <mtr><mtd><mn>-1</mn></mtd><mtd><mn>2</mn></mtd><mtd><mn>-3</mn></mtd></mtr>
                </mtable><mo>)</mo></math>";
    test_braille("UEB", expr, "⠠⠐⠣⠁⠀⠰⠃⠀⠉⠠⠐⠜⠠⠐⠣⠐⠤⠼⠁⠀⠼⠃⠀⠐⠤⠼⠉⠠⠐⠜")?;
    return Ok(());
}

#[test]
fn bana2025_5_6_matrix_2x2() -> Result<()> {
    // best-effort reconstruction -- see file header note
    let expr = "<math><mo>[</mo><mtable>
                    <mtr><mtd><mi>a</mi></mtd><mtd><mi>b</mi></mtd></mtr>
                    <mtr><mtd><mo>-</mo><mi>x</mi></mtd><mtd><mn>2</mn><mi>y</mi></mtd></mtr>
                </mtable><mo>]</mo></math>";
    test_braille("UEB", expr, "⠠⠨⠣⠁⠀⠰⠃⠀⠠⠨⠜⠠⠨⠣⠐⠤⠭⠀⠼⠃⠽⠠⠨⠜")?;
    return Ok(());
}

#[test]
fn bana2025_5_8_log_base2_plus_log() -> Result<()> {
    let expr = "<math><msub><mi>log</mi><mn>2</mn></msub><mi>x</mi><mo>+</mo><mi>log</mi><mi>x</mi></math>";
    test_braille("UEB", expr, "⠇⠕⠛⠰⠢⠼⠃⠭⠐⠖⠇⠕⠛⠀⠰⠭")?;
    return Ok(());
}

// 5.3 Grade 1 word indicators

#[test]
fn bana2025_5_11_a_over_b() -> Result<()> {
    let expr = "<math><mfrac><mi>a</mi><mi>b</mi></mfrac></math>";
    test_braille("UEB", expr, "⠰⠰⠷⠁⠨⠌⠃⠾")?;
    return Ok(());
}

#[test]
fn bana2025_5_12_vector_ab() -> Result<()> {
    let expr = "<math><mover><mrow><mi mathvariant='normal'>A</mi><mi mathvariant='normal'>B</mi></mrow>
                <mo>→</mo></mover></math>";
    test_braille("UEB", expr, "⠰⠰⠣⠠⠠⠁⠃⠜⠘⠱")?;
    return Ok(());
}

#[test]
fn bana2025_5_13_x_inverse_eq_one_over_x() -> Result<()> {
    let expr = "<math><msup><mi>x</mi><mrow><mo>-</mo><mn>1</mn></mrow></msup><mo>=</mo>
                <mfrac><mn>1</mn><mi>x</mi></mfrac></math>";
    test_braille("UEB", expr, "⠰⠰⠭⠔⠣⠐⠤⠼⠁⠜⠀⠐⠶⠀⠰⠷⠼⠁⠨⠌⠭⠾")?;
    return Ok(());
}

#[test]
fn bana2025_5_14_power_multiplication() -> Result<()> {
    // Same MathML as bana_5_4 in iceb.rs, but the 2025 update prefers two separate grade 1
    // word indicators here instead of the single grade 1 passage bana_5_4 expects -- this is
    // meant to directly contrast the old vs. new behavior for issue #529.
    let expr = "<math><msup><mi>a</mi><mi>n</mi></msup><mo>×</mo><msup><mi>a</mi><mi>m</mi></msup><mo>=</mo>
                    <msup><mi>a</mi><mrow><mi>n</mi><mo>+</mo><mi>m</mi></mrow></msup></math>";
    test_braille("UEB", expr, "⠰⠰⠁⠔⠝⠐⠦⠁⠔⠍⠀⠐⠶⠀⠰⠰⠁⠔⠣⠝⠐⠖⠍⠜")?;
    return Ok(());
}

// 5.4 Grade 1 passages

#[test]
fn bana2025_5_15_power_formula_passage() -> Result<()> {
    let expr = "<math><mi mathvariant='normal'>P</mi><mo>=</mo>
                <mi mathvariant='normal'>V</mi><mi mathvariant='normal'>I</mi><mo>=</mo>
                <msup><mi mathvariant='normal'>I</mi><mn>2</mn></msup><mi mathvariant='normal'>R</mi><mo>=</mo>
                <mfrac><msup><mi mathvariant='normal'>V</mi><mn>2</mn></msup><mi mathvariant='normal'>R</mi></mfrac></math>";
    test_braille("UEB", expr, "⠰⠰⠰⠠⠠⠠⠏⠀⠐⠶⠀⠧⠊⠀⠐⠶⠀⠊⠔⠼⠃⠗⠀⠐⠶⠀⠷⠧⠔⠼⠃⠨⠌⠗⠾⠠⠄⠰⠄")?;
    return Ok(());
}

#[test]
fn bana2025_5_16_equilibrium_reaction() -> Result<()> {
    let expr = "<math><mi mathvariant='normal'>Ca</mi><mi mathvariant='normal'>C</mi><msub><mi mathvariant='normal'>O</mi><mn>3</mn></msub>
                <mo>⇄</mo>
                <mi mathvariant='normal'>Ca</mi><mi mathvariant='normal'>O</mi><mo>+</mo>
                <mi mathvariant='normal'>C</mi><msub><mi mathvariant='normal'>O</mi><mn>2</mn></msub></math>";
    test_braille("UEB", expr, "⠰⠰⠰⠠⠉⠁⠠⠉⠠⠕⠢⠼⠉⠀⠳⠕⠻⠳⠪⠀⠠⠉⠁⠠⠕⠐⠖⠠⠉⠠⠕⠢⠼⠃⠰⠄")?;
    return Ok(());
}
