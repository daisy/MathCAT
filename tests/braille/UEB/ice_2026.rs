// UEB tests for GitHub issue #529: align with the ICEB "Guidelines for Technical Material"
// revision to Section 1.7 "Choice and placement of grade 1 indicators" (approved July 2025).
//
// Source: ICEB-Rules.pdf, Section 1, "1.7 Choice and placement of grade 1 indicators".
//
// Notes on scope:
// - 1.7.1 ("Evaluate the following: 3 - 2 1/2 =") and 1.7.2 ("y = x + 4c") are already covered
//   by grade1_1_7_1 and grade1_1_7_2 in iceb.rs (those examples are unchanged from the older
//   ICEB document), so they are not repeated here.
// - Per guidance, only the *math* part of each example is tested; generic English lead-in
//   text (e.g. "Evaluate the following:", "Solve the following quadratic equations:",
//   "Factorise:") is stripped since it isn't relevant to grade 1 indicator placement.
//   Words that are integral to the expression itself (e.g. "speed =", "Coordinate (x, y)",
//   "Assume g =") are kept, consistent with how existing tests (e.g. fraction_6_4_6,
//   expr_3_1_8) treat such labels.
// - The 1.7.4 example showing a grade 1 passage indicator placed on *separate lines* across
//   a multi-line list of exercises is not testable here: MathCAT's get_braille() returns a
//   single continuous string with no concept of physical print line breaks, which is what
//   that example is specifically about.
// - The combined "Laws of indices" passage (page 11-12) is not included: it is one grade 1
//   passage spanning six separate equations across multiple print lines, and the correct
//   braille for any individual law depends on that surrounding passage context, which can't
//   be reconstructed reliably in isolation.
// - Several expected braille strings below were reconstructed by joining lines that wrapped
//   across PDF pages/print lines. If one of these fails with only a stray extra/missing single
//   space, double check against the PDF source before assuming it's a MathCAT bug.
// - Some of these tests are expected to fail against the current implementation -- that's the
//   point of issue #529. A failure with wildly more/fewer braille cells than expected likely
//   means the MathML (not MathCAT) is wrong and should be double checked.
use crate::common::*;
use anyhow::Result;

// 1.7.3(a) allow one grade 1 symbol indicator per symbols-sequence

#[test]
fn grade1_1_7_3a_squared() -> Result<()> {
    let expr = "<math><msup><mi>x</mi><mn>2</mn></msup></math>";
    test_braille("UEB", expr, "⠭⠰⠔⠼⠃")?;
    return Ok(());
}

#[test]
fn grade1_1_7_3a_arrow_infinity() -> Result<()> {
    let expr = "<math><mi>x</mi><mo>&#x2192;</mo><mi>&#x221E;</mi></math>";
    test_braille("UEB", expr, "⠰⠭⠀⠰⠳⠕⠀⠼⠿")?;
    return Ok(());
}

#[test]
fn grade1_1_7_3a_x_over_y() -> Result<()> {
    let expr = "<math><mfrac><mi>x</mi><mi>y</mi></mfrac></math>";
    // [rather than] ⠰⠷⠭⠨⠌⠽⠰⠾ (word indicator misplaced after the open fraction sign)
    test_braille("UEB", expr, "⠰⠰⠷⠭⠨⠌⠽⠾")?;
    return Ok(());
}

// 1.7.3(b) use a grade 1 passage indicator if 3+ symbols-sequences need grade 1 indicators

#[test]
fn grade1_1_7_3b_subscripts_abc() -> Result<()> {
    let expr = "<math><msub><mi>x</mi><mi>a</mi></msub><mo>=</mo>
                <msub><mi>x</mi><mi>b</mi></msub><mo>=</mo>
                <msub><mi>x</mi><mi>c</mi></msub></math>";
    test_braille("UEB", expr, "⠰⠰⠰⠭⠢⠁⠀⠐⠶⠀⠭⠢⠃⠀⠐⠶⠀⠭⠢⠉⠰⠄")?;
    return Ok(());
}

#[test]
fn grade1_1_7_3b_factorise_chain() -> Result<()> {
    // "Factorise:" stripped -- see file header note on lead-in text
    let expr = "<math>
        <mi>y</mi><mo>=</mo><msup><mi>x</mi><mn>2</mn></msup><mo>&#x2212;</mo><mn>4</mn><mo>;</mo>
        <mi>y</mi><mo>=</mo><msup><mi>x</mi><mn>2</mn></msup><mo>&#x2212;</mo><mn>2</mn><mi>x</mi><mo>;</mo>
        <mi>y</mi><mo>=</mo><mi>x</mi><mo>&#x2212;</mo><msup><mi>x</mi><mn>2</mn></msup><mo>.</mo>
    </math>";
    test_braille("UEB", expr,
        "⠰⠰⠰⠽⠀⠐⠶⠀⠭⠔⠼⠃⠐⠤⠼⠙⠆⠀⠽⠀⠐⠶⠀⠭⠔⠼⠃⠐⠤⠼⠃⠭⠆⠀⠽⠀⠐⠶⠀⠭⠐⠤⠭⠔⠼⠃⠲⠰⠄")?;
    return Ok(());
}

// 1.7.5 words which form part of a technical expression

#[test]
fn grade1_1_7_5a_speed_distance_time() -> Result<()> {
    let expr = "<math><mtext>speed</mtext><mo>=</mo>
                <mfrac><mtext>distance</mtext><mtext>time</mtext></mfrac></math>";
    test_braille("UEB", expr, "⠎⠏⠑⠫⠀⠐⠶⠀⠰⠷⠙⠊⠌⠨⠑⠨⠌⠐⠞⠰⠾")?;
    return Ok(());
}

#[test]
fn grade1_1_7_5b_luminosity_sun() -> Result<()> {
    let expr = "<math><msub><mtext>luminosity</mtext><mtext>sun</mtext></msub></math>";
    // [rather than] ⠰⠰⠇⠥⠍⠊⠝⠕⠎⠊⠞⠽⠢⠣⠎⠥⠝⠜ (word indicator placed at the very start
    // rather than just before "sun", so "luminosity" can't appear contracted)
    test_braille("UEB", expr, "⠇⠥⠍⠔⠕⠎⠰⠽⠰⠰⠢⠣⠎⠥⠝⠜")?;
    return Ok(());
}

#[test]
fn grade1_1_7_5c_speed_chain() -> Result<()> {
    let expr = "<math><mtext>speed</mtext><mo>=</mo>
                <mfrac><mtext>distance</mtext><mtext>time</mtext></mfrac><mo>=</mo>
                <mfrac><mrow><mn>30,000</mn><mo>&#xA0;</mo><mi class='MathML-unit'>m</mi></mrow>
                       <mrow><mn>60</mn><mo>&#xA0;</mo><mi class='MathML-unit'>s</mi></mrow></mfrac><mo>=</mo>
                <mn>500</mn><mo>&#xA0;</mo><mi class='MathML-unit'>m</mi><mo>/</mo><mi class='MathML-unit'>s</mi></math>";
    test_braille("UEB", expr,
        "⠎⠏⠑⠫⠀⠐⠶⠀⠰⠰⠰⠷⠙⠊⠎⠞⠁⠝⠉⠑⠨⠌⠞⠊⠍⠑⠾⠀⠐⠶⠀⠷⠼⠉⠚⠂⠚⠚⠚⠀⠍⠨⠌⠼⠋⠚⠀⠎⠾⠀⠐⠶⠀⠼⠑⠚⠚⠀⠍⠸⠌⠎⠰⠄")?;
    return Ok(());
}

// 1.7.9 further examples of preferred grade 1 indicator usage

#[test]
fn grade1_1_7_9_01_y_eq_x() -> Result<()> {
    let expr = "<math><mi>y</mi><mo>=</mo><mi>x</mi></math>";
    test_braille("UEB", expr, "⠰⠽⠀⠐⠶⠀⠰⠭")?;
    return Ok(());
}

#[test]
fn grade1_1_7_9_02_sqrt25() -> Result<()> {
    let expr = "<math><msqrt><mn>25</mn></msqrt><mo>=</mo><mn>5</mn></math>";
    test_braille("UEB", expr, "⠰⠩⠼⠃⠑⠬⠀⠐⠶⠀⠼⠑")?;
    return Ok(());
}

#[test]
fn grade1_1_7_9_03_sqrtx() -> Result<()> {
    let expr = "<math><msqrt><mi>x</mi></msqrt><mo>=</mo><mn>7</mn></math>";
    test_braille("UEB", expr, "⠰⠰⠩⠭⠬⠀⠐⠶⠀⠼⠛")?;
    return Ok(());
}

#[test]
fn grade1_1_7_9_04_ms_inverse() -> Result<()> {
    let expr = "<math><mi class='MathML-unit'>m</mi><msup><mi class='MathML-unit'>s</mi>
                <mrow><mo>-</mo><mn>1</mn></mrow></msup></math>";
    // [rather than] ⠍⠎⠰⠔⠰⠣⠐⠤⠼⠁⠜ [or] ⠍⠎⠰⠰⠔⠣⠐⠤⠼⠁⠜
    test_braille("UEB", expr, "⠰⠰⠍⠎⠔⠣⠐⠤⠼⠁⠜")?;
    return Ok(());
}

#[test]
fn grade1_1_7_9_05_y_eq_x_over_2() -> Result<()> {
    let expr = "<math><mi>y</mi><mo>=</mo><mfrac><mi>x</mi><mn>2</mn></mfrac></math>";
    test_braille("UEB", expr, "⠰⠽⠀⠐⠶⠀⠰⠷⠭⠨⠌⠼⠃⠾")?;
    return Ok(());
}

#[test]
fn grade1_1_7_9_06_y_eq_xsq_over_2() -> Result<()> {
    let expr = "<math><mi>y</mi><mo>=</mo>
                <mfrac><msup><mi>x</mi><mn>2</mn></msup><mn>2</mn></mfrac></math>";
    test_braille("UEB", expr, "⠰⠽⠀⠐⠶⠀⠰⠰⠷⠭⠔⠼⠃⠨⠌⠼⠃⠾")?;
    return Ok(());
}

#[test]
fn grade1_1_7_9_07_x_over_y_eq_c_over_d() -> Result<()> {
    let expr = "<math><mfrac><mi>x</mi><mi>y</mi></mfrac><mo>=</mo>
                <mfrac><mi>c</mi><mi>d</mi></mfrac></math>";
    test_braille("UEB", expr, "⠰⠰⠷⠭⠨⠌⠽⠾⠀⠐⠶⠀⠰⠰⠷⠉⠨⠌⠙⠾")?;
    return Ok(());
}

#[test]
fn grade1_1_7_9_08_coordinate_xy() -> Result<()> {
    let expr = "<math><mtext>Coordinate</mtext><mo>&#xA0;</mo><mo>(</mo><mi>x</mi><mo>,</mo><mi>y</mi><mo>)</mo></math>";
    test_braille("UEB", expr, "⠠⠉⠕⠕⠗⠙⠔⠁⠞⠑⠀⠐⠣⠰⠭⠂⠀⠰⠽⠐⠜")?;
    return Ok(());
}

#[test]
fn grade1_1_7_9_09_coordinate_xi_yi() -> Result<()> {
    let expr = "<math><mtext>Coordinate</mtext><mo>&#xA0;</mo><mo>(</mo>
                <msub><mi>x</mi><mi>i</mi></msub><mo>,</mo>
                <msub><mi>y</mi><mi>i</mi></msub><mo>)</mo></math>";
    test_braille("UEB", expr, "⠠⠉⠕⠕⠗⠙⠔⠁⠞⠑⠀⠐⠣⠭⠰⠢⠊⠂⠀⠽⠰⠢⠊⠐⠜")?;
    return Ok(());
}

#[test]
fn grade1_1_7_9_10_coordinate_xi2_yi2() -> Result<()> {
    let expr = "<math><mtext>Coordinate</mtext><mo>&#xA0;</mo><mo>(</mo>
                <msubsup><mi>x</mi><mi>i</mi><mn>2</mn></msubsup><mo>,</mo>
                <msubsup><mi>y</mi><mi>i</mi><mn>2</mn></msubsup><mo>)</mo></math>";
    test_braille("UEB", expr, "⠠⠉⠕⠕⠗⠙⠔⠁⠞⠑⠀⠰⠰⠐⠣⠭⠢⠊⠔⠼⠃⠂⠀⠰⠰⠽⠢⠊⠔⠼⠃⠐⠜")?;
    return Ok(());
}

#[test]
fn grade1_1_7_9_11_b_bar() -> Result<()> {
    let expr = "<math><mover><mi mathvariant='normal'>B</mi><mo>&#xAF;</mo></mover></math>";
    test_braille("UEB", expr, "⠠⠃⠰⠱")?;
    return Ok(());
}

#[test]
fn grade1_1_7_9_12_m_leftright_arrow() -> Result<()> {
    let expr = "<math><mover><mi mathvariant='normal'>M</mi><mo>&#x2194;</mo></mover></math>";
    test_braille("UEB", expr, "⠠⠍⠨⠔⠰⠳⠺⠗⠕")?;
    return Ok(());
}

#[test]
fn grade1_1_7_9_13_az_leftright_arrow() -> Result<()> {
    let expr = "<math><mover><mrow><mi mathvariant='normal'>A</mi><mi mathvariant='normal'>Z</mi></mrow>
                <mo>&#x2194;</mo></mover></math>";
    // [rather than] ⠰⠣⠠⠠⠁⠵⠰⠜⠨⠔⠰⠳⠺⠗⠕
    test_braille("UEB", expr, "⠰⠰⠣⠠⠠⠁⠵⠜⠨⠔⠳⠺⠗⠕")?;
    return Ok(());
}

#[test]
fn grade1_1_7_9_14_al_isotope() -> Result<()> {
    let expr = "<math><mmultiscripts><mi mathvariant='normal'>Al</mi>
                <mprescripts/><mn>13</mn><mn>27</mn></mmultiscripts></math>";
    test_braille("UEB", expr, "⠰⠢⠼⠁⠉⠔⠼⠃⠛⠠⠁⠇")?;
    return Ok(());
}

#[test]
fn grade1_1_7_9_15_o_2minus() -> Result<()> {
    let expr = "<math><msup><mi mathvariant='normal'>O</mi><mrow><mn>2</mn><mo>-</mo></mrow></msup></math>";
    test_braille("UEB", expr, "⠰⠰⠠⠕⠔⠣⠼⠃⠐⠤⠜")?;
    return Ok(());
}

#[test]
fn grade1_1_7_9_16_chem_h2_o2_reaction() -> Result<()> {
    let expr = "<math>
        <mn>2</mn><msub><mi mathvariant='normal'>H</mi><mn>2</mn></msub><mo>(</mo><mi>g</mi><mo>)</mo>
        <mo>+</mo><msub><mi mathvariant='normal'>O</mi><mn>2</mn></msub><mo>(</mo><mi>g</mi><mo>)</mo>
        <mo>&#x2192;</mo>
        <mn>2</mn><msub><mi mathvariant='normal'>H</mi><mn>2</mn></msub><mi mathvariant='normal'>O</mi><mo>(</mo><mi>l</mi><mo>)</mo>
    </math>";
    test_braille("UEB", expr,
        "⠼⠃⠠⠓⠢⠼⠃⠐⠣⠛⠐⠜⠐⠖⠠⠕⠢⠼⠃⠐⠣⠛⠐⠜⠀⠰⠳⠕⠀⠼⠃⠠⠓⠢⠼⠃⠠⠕⠐⠣⠇⠐⠜")?;
    return Ok(());
}

#[test]
fn grade1_1_7_9_17_y_eq_mx_c() -> Result<()> {
    let expr = "<math><mi>y</mi><mo>=</mo><mi>m</mi><mi>x</mi><mo>+</mo><mi>c</mi></math>";
    test_braille("UEB", expr, "⠰⠽⠀⠐⠶⠀⠍⠭⠐⠖⠉")?;
    return Ok(());
}

#[test]
fn grade1_1_7_9_18_a_eq_pi_r_sq() -> Result<()> {
    let expr = "<math><mi mathvariant='normal'>A</mi><mo>=</mo>
                <mi mathvariant='normal'>&#x3C0;</mi><msup><mi>r</mi><mn>2</mn></msup></math>";
    test_braille("UEB", expr, "⠠⠁⠀⠐⠶⠀⠨⠏⠗⠰⠔⠼⠃")?;
    return Ok(());
}

#[test]
fn grade1_1_7_9_19_e_eq_mc2() -> Result<()> {
    let expr = "<math><mi mathvariant='normal'>E</mi><mo>=</mo>
                <mi>m</mi><msup><mi>c</mi><mn>2</mn></msup></math>";
    test_braille("UEB", expr, "⠰⠠⠑⠀⠐⠶⠀⠍⠉⠰⠔⠼⠃")?;
    return Ok(());
}

#[test]
fn grade1_1_7_9_20_perfect_square() -> Result<()> {
    let expr = "<math><msup><mi>x</mi><mn>2</mn></msup><mo>&#x2212;</mo><mn>2</mn><mi>x</mi><mo>+</mo><mn>1</mn>
                <mo>=</mo><msup><mrow><mo>(</mo><mi>x</mi><mo>&#x2212;</mo><mn>1</mn><mo>)</mo></mrow><mn>2</mn></msup></math>";
    test_braille("UEB", expr, "⠭⠰⠔⠼⠃⠐⠤⠼⠃⠭⠐⠖⠼⠁⠀⠐⠶⠀⠐⠣⠭⠐⠤⠼⠁⠐⠜⠔⠼⠃")?;
    return Ok(());
}

#[test]
fn grade1_1_7_9_21_xn_alternating() -> Result<()> {
    let expr = "<math><msub><mi>x</mi><mi>n</mi></msub><mo>=</mo><mn>1</mn><mo>+</mo>
                <mfrac><msup><mrow><mo>(</mo><mo>&#x2212;</mo><mn>1</mn><mo>)</mo></mrow><mi>n</mi></msup><mi>n</mi></mfrac></math>";
    test_braille("UEB", expr, "⠭⠰⠢⠝⠀⠐⠶⠀⠼⠁⠐⠖⠷⠐⠤⠼⠁⠔⠝⠨⠌⠝⠾")?;
    return Ok(());
}

#[test]
fn grade1_1_7_9_22_xn_exponential() -> Result<()> {
    let expr = "<math><msub><mi>x</mi><mi>n</mi></msub><mo>=</mo>
                <mfrac><msup><mn>2</mn><mi>n</mi></msup><msup><mi>n</mi><mn>2</mn></msup></mfrac></math>";
    test_braille("UEB", expr, "⠭⠰⠢⠝⠀⠐⠶⠀⠰⠷⠼⠃⠔⠝⠨⠌⠝⠔⠼⠃⠾")?;
    return Ok(());
}

#[test]
fn grade1_1_7_9_23_quadratic_formula() -> Result<()> {
    let expr = "<math><mi>x</mi><mo>=</mo>
        <mfrac>
            <mrow><mo>&#x2212;</mo><mi>b</mi><mo>&#xB1;</mo>
                <msqrt><msup><mi>b</mi><mn>2</mn></msup><mo>&#x2212;</mo><mn>4</mn><mi>a</mi><mi>c</mi></msqrt>
            </mrow>
            <mrow><mn>2</mn><mi>a</mi></mrow>
        </mfrac>
    </math>";
    test_braille("UEB", expr, "⠰⠭⠀⠐⠶⠀⠰⠰⠷⠐⠤⠃⠸⠖⠩⠃⠔⠼⠃⠐⠤⠼⠙⠰⠁⠉⠬⠨⠌⠼⠃⠰⠁⠾")?;
    return Ok(());
}

#[test]
fn grade1_1_7_9_24_x_sqrt_sintheta() -> Result<()> {
    let expr = "<math><mi>x</mi><msqrt><mi>sin</mi><mo>&#x2061;</mo><mi>&#x3B8;</mi></msqrt></math>";
    // [rather than] ⠭⠰⠩⠎⠔⠨⠹⠰⠬
    test_braille("UEB", expr, "⠰⠰⠭⠩⠎⠊⠝⠨⠹⠬")?;
    return Ok(());
}

#[test]
fn grade1_1_7_9_25_version_1b() -> Result<()> {
    let expr = "<math><msup><mtext>Version</mtext><mrow><mn>1</mn><mi>b</mi></mrow></msup></math>";
    // [rather than] ⠰⠰⠠⠧⠑⠗⠎⠊⠕⠝⠔⠣⠼⠁⠰⠃⠜
    test_braille("UEB", expr, "⠠⠧⠻⠨⠝⠰⠰⠔⠣⠼⠁⠰⠃⠜")?;
    return Ok(());
}

#[test]
fn grade1_1_7_9_26_w_eq_fs() -> Result<()> {
    let expr = "<math><mi mathvariant='normal'>W</mi><mo>=</mo>
                <mi mathvariant='normal'>F</mi><mi>s</mi></math>";
    test_braille("UEB", expr, "⠰⠠⠺⠀⠐⠶⠀⠠⠋⠎")?;
    return Ok(());
}

#[test]
fn grade1_1_7_9_27_work_eq_force_times_distance() -> Result<()> {
    let expr = "<math><mtext>work</mtext><mo>=</mo><mtext>force</mtext><mo>&#xD7;</mo><mtext>distance</mtext></math>";
    test_braille("UEB", expr, "⠐⠺⠀⠐⠶⠀⠿⠉⠑⠐⠦⠙⠊⠌⠨⠑")?;
    return Ok(());
}

#[test]
fn grade1_1_7_9_28_therefore_force() -> Result<()> {
    let expr = "<math><mo>&#x2234;</mo><mtext>force</mtext><mo>=</mo>
                <mfrac><mtext>work</mtext><mtext>distance</mtext></mfrac></math>";
    test_braille("UEB", expr, "⠰⠠⠡⠀⠿⠉⠑⠀⠐⠶⠀⠰⠷⠐⠺⠨⠌⠙⠊⠌⠨⠑⠰⠾")?;
    return Ok(());
}

#[test]
fn grade1_1_7_9_29_acceleration() -> Result<()> {
    let expr = "<math><mtext>acceleration</mtext><mo>=</mo>
                <mfrac><mrow><mi mathvariant='normal'>&#x394;</mi><mtext>speed</mtext></mrow>
                       <mrow><mi mathvariant='normal'>&#x394;</mi><mtext>time</mtext></mrow></mfrac></math>";
    test_braille("UEB", expr, "⠁⠒⠑⠇⠻⠁⠰⠝⠀⠐⠶⠀⠰⠷⠠⠨⠙⠎⠏⠑⠫⠨⠌⠠⠨⠙⠐⠞⠰⠾")?;
    return Ok(());
}

#[test]
fn grade1_1_7_9_30_frequency_fraction() -> Result<()> {
    let expr = "<math><mtext>frequency</mtext><mo>=</mo><mfrac><mn>1</mn><mtext>time</mtext></mfrac></math>";
    test_braille("UEB", expr, "⠋⠗⠑⠟⠥⠢⠉⠽⠀⠐⠶⠀⠰⠷⠼⠁⠨⠌⠞⠊⠍⠑⠾")?;
    return Ok(());
}

#[test]
fn grade1_1_7_9_31_frequency_slash() -> Result<()> {
    let expr = "<math><mtext>frequency</mtext><mo>=</mo><mn>1</mn><mo>/</mo><mtext>time</mtext></math>";
    test_braille("UEB", expr, "⠋⠗⠑⠟⠥⠢⠉⠽⠀⠐⠶⠀⠼⠁⠸⠌⠞⠊⠍⠑")?;
    return Ok(());
}

#[test]
fn grade1_1_7_9_32_vcone() -> Result<()> {
    let expr = "<math><msub><mi mathvariant='normal'>V</mi><mtext>cone</mtext></msub><mo>=</mo>
                <mfrac><mn>1</mn><mn>3</mn></mfrac>
                <mi mathvariant='normal'>&#x3C0;</mi><msup><mi>r</mi><mn>2</mn></msup><mi>h</mi></math>";
    // [rather than] ⠠⠧⠰⠢⠰⠣⠉⠐⠕⠰⠜⠀⠐⠶⠀⠼⠁⠌⠉⠨⠏⠗⠔⠼⠃⠰⠓
    test_braille("UEB", expr, "⠰⠰⠠⠧⠢⠣⠉⠕⠝⠑⠜⠀⠐⠶⠀⠼⠁⠌⠉⠨⠏⠗⠔⠼⠃⠰⠓")?;
    return Ok(());
}

#[test]
fn grade1_1_7_9_33_v_integral() -> Result<()> {
    let expr = "<math><mi mathvariant='normal'>V</mi><mo>=</mo>
                <mo>&#x222B;</mo><mi mathvariant='normal'>&#x3C0;</mi>
                <msup><mi>y</mi><mn>2</mn></msup><mi>d</mi><mi>x</mi></math>";
    // [rather than] ⠰⠰⠰⠠⠧⠀⠐⠶⠀⠮⠨⠏⠽⠔⠼⠃⠰⠙⠭⠰⠄
    test_braille("UEB", expr, "⠰⠠⠧⠀⠐⠶⠀⠰⠰⠮⠨⠏⠽⠔⠼⠃⠰⠙⠭")?;
    return Ok(());
}

#[test]
fn grade1_1_7_9_34_period() -> Result<()> {
    let expr = "<math><mtext>period</mtext><mo>=</mo><mn>2</mn><mi mathvariant='normal'>&#x3C0;</mi>
                <msqrt><mfrac><mrow><mi>l</mi><mi>cos</mi><mo>&#x2061;</mo><mi>&#x3B1;</mi></mrow><mi>g</mi></mfrac></msqrt></math>";
    test_braille("UEB", expr, "⠏⠻⠊⠕⠙⠀⠐⠶⠀⠼⠃⠨⠏⠩⠷⠇⠀⠰⠰⠉⠕⠎⠨⠁⠨⠌⠛⠾⠬")?;
    return Ok(());
}

#[test]
fn grade1_1_7_9_35_assume_g() -> Result<()> {
    let expr = "<math><mtext>Assume</mtext><mo>&#xA0;</mo><mi>g</mi><mo>=</mo><mn>9.81</mn><mo>&#xA0;</mo>
                <mi class='MathML-unit'>m</mi><msup><mi class='MathML-unit'>s</mi><mrow><mo>-</mo><mn>2</mn></mrow></msup></math>";
    test_braille("UEB", expr, "⠠⠁⠎⠎⠥⠍⠑⠀⠰⠛⠀⠐⠶⠀⠼⠊⠲⠓⠁⠀⠰⠰⠍⠎⠔⠣⠐⠤⠼⠃⠜")?;
    return Ok(());
}

#[test]
fn grade1_1_7_9_36_v1_x_ms() -> Result<()> {
    let expr = "<math><msub><mi>v</mi><mn>1</mn></msub><mo>=</mo><mi>x</mi><mo>&#xA0;</mo>
                <mi class='MathML-unit'>m</mi><msup><mi class='MathML-unit'>s</mi><mrow><mo>-</mo><mn>1</mn></mrow></msup></math>";
    test_braille("UEB", expr, "⠰⠰⠰⠧⠢⠼⠁⠀⠐⠶⠀⠭⠀⠍⠎⠔⠣⠐⠤⠼⠁⠜⠰⠄")?;
    return Ok(());
}

#[test]
fn grade1_1_7_9_37_p_set() -> Result<()> {
    let expr = "<math><mi mathvariant='normal'>P</mi><mo>=</mo>
                <mo>{</mo><mi>t</mi><mo>,</mo><mi>u</mi><mo>,</mo><mi>v</mi><mo>}</mo></math>";
    // [rather than] ⠰⠠⠏⠀⠐⠶⠀⠸⠣⠰⠞⠂⠀⠰⠥⠂⠀⠰⠧⠸⠜
    test_braille("UEB", expr, "⠰⠰⠰⠠⠏⠀⠐⠶⠀⠸⠣⠞⠂⠀⠥⠂⠀⠧⠸⠜⠰⠄")?;
    return Ok(());
}

#[test]
fn grade1_1_7_9_38_four_subscripts() -> Result<()> {
    let expr = "<math><msub><mi>x</mi><mi>a</mi></msub><mo>=</mo>
                <msub><mi>x</mi><mi>b</mi></msub><mo>=</mo>
                <msub><mi>x</mi><mi>c</mi></msub><mo>=</mo>
                <msub><mi>x</mi><mi>d</mi></msub></math>";
    // [rather than] ⠭⠰⠢⠁⠀⠐⠶⠀⠭⠰⠢⠃⠀⠐⠶⠀⠭⠰⠢⠉⠀⠐⠶⠀⠭⠰⠢⠙
    test_braille("UEB", expr, "⠰⠰⠰⠭⠢⠁⠀⠐⠶⠀⠭⠢⠃⠀⠐⠶⠀⠭⠢⠉⠀⠐⠶⠀⠭⠢⠙⠰⠄")?;
    return Ok(());
}

#[test]
fn grade1_1_7_9_39_power_multiplication() -> Result<()> {
    let expr = "<math><msup><mi>x</mi><mn>2</mn></msup><mo>&#xD7;</mo><msup><mi>x</mi><mn>3</mn></msup><mo>=</mo>
                <msup><mi>x</mi><mrow><mn>2</mn><mo>+</mo><mn>3</mn></mrow></msup><mo>=</mo>
                <msup><mi>x</mi><mn>5</mn></msup></math>";
    test_braille("UEB", expr, "⠰⠰⠰⠭⠔⠼⠃⠐⠦⠭⠔⠼⠉⠀⠐⠶⠀⠭⠔⠣⠼⠃⠐⠖⠼⠉⠜⠀⠐⠶⠀⠭⠔⠼⠑⠰⠄")?;
    return Ok(());
}

#[test]
fn grade1_1_7_9_40_b_arrow_c() -> Result<()> {
    let expr = "<math><mi mathvariant='normal'>B</mi><mo>&#x2192;</mo><mi mathvariant='normal'>C</mi></math>";
    // [rather than] ⠰⠠⠃⠀⠰⠳⠕⠀⠰⠠⠉
    test_braille("UEB", expr, "⠰⠰⠰⠠⠃⠀⠳⠕⠀⠠⠉⠰⠄")?;
    return Ok(());
}

#[test]
fn grade1_1_7_9_41_x_plus_y_eq_5() -> Result<()> {
    let expr = "<math><mi>x</mi><mo>+</mo><mi>y</mi><mo>=</mo><mn>5</mn></math>";
    // [rather than] ⠰⠰⠰⠭⠐⠖⠽⠀⠐⠶⠀⠼⠑⠰⠄
    test_braille("UEB", expr, "⠭⠐⠖⠽⠀⠐⠶⠀⠼⠑")?;
    return Ok(());
}

#[test]
fn grade1_1_7_9_42_methane_chlorine() -> Result<()> {
    let expr = "<math>
        <mi mathvariant='normal'>C</mi><msub><mi mathvariant='normal'>H</mi><mn>4</mn></msub>
        <mo>+</mo><mn>4</mn><msub><mi mathvariant='normal'>Cl</mi><mn>2</mn></msub>
        <mo>&#x2192;</mo>
        <mi mathvariant='normal'>C</mi><msub><mi mathvariant='normal'>Cl</mi><mn>4</mn></msub>
        <mo>+</mo><mn>4</mn><mi mathvariant='normal'>H</mi><mi mathvariant='normal'>Cl</mi>
    </math>";
    test_braille("UEB", expr,
        "⠰⠰⠰⠠⠉⠠⠓⠢⠼⠙⠐⠖⠼⠙⠠⠉⠇⠢⠼⠃⠀⠳⠕⠀⠠⠉⠠⠉⠇⠢⠼⠙⠐⠖⠼⠙⠠⠓⠠⠉⠇⠰⠄")?;
    return Ok(());
}

#[test]
fn grade1_1_7_9_43_carbon_decay() -> Result<()> {
    let expr = "<math>
        <mmultiscripts><mi mathvariant='normal'>C</mi><mprescripts/><mn>6</mn><mn>14</mn></mmultiscripts>
        <mo>&#x2192;</mo>
        <mmultiscripts><mi mathvariant='normal'>N</mi><mprescripts/><mn>7</mn><mn>14</mn></mmultiscripts>
        <mo>+</mo>
        <mmultiscripts><mi>&#x3B2;</mi><mprescripts/><mrow><mo>-</mo><mn>1</mn></mrow><mn>0</mn></mmultiscripts>
    </math>";
    test_braille("UEB", expr,
        "⠰⠰⠰⠢⠼⠋⠔⠼⠁⠙⠠⠉⠀⠳⠕⠀⠢⠼⠛⠔⠼⠁⠙⠠⠝⠐⠖⠢⠣⠐⠤⠼⠁⠜⠔⠼⠚⠨⠃⠰⠄")?;
    return Ok(());
}

#[test]
fn grade1_1_7_9_44_reliability() -> Result<()> {
    let expr = "<math><mtext>reliability</mtext><mo>=</mo>
                <mfrac><mtext>number of faults</mtext><mtext>total number of items</mtext></mfrac>
                <mo>=</mo><mi>p</mi></math>";
    test_braille("UEB", expr,
        "⠗⠑⠇⠊⠁⠃⠊⠇⠰⠽⠀⠐⠶⠀⠰⠰⠰⠷⠝⠥⠍⠃⠑⠗⠀⠕⠋⠀⠋⠁⠥⠇⠞⠎⠨⠌⠞⠕⠞⠁⠇⠀⠝⠥⠍⠃⠑⠗⠀⠕⠋⠀⠊⠞⠑⠍⠎⠾⠀⠐⠶⠀⠏⠰⠄")?;
    return Ok(());
}
