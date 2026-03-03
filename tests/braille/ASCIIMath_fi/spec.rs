// Tests based on the Finnish flavor of ASCIIMath located on the Finnish delegation for braille's "Matematiikan, fysiikan ja kemian merkinnät elektronisissa oppikirjoissa" (https://www.pistekirjoitus.fi/julkaisut/matematiikka-ja-tietotekniikka/).
// Tests will be named according to the page and some identification.
use crate::common::*;
use anyhow::Result;


#[test]
fn p12_equal () -> Result<()> {
    let expr = r#"<math><mn>3</mn><mo>+</mo><mn>4</mn><mo>=</mo><mn>7</mn></math>"#;
    test_braille("ASCIIMath-fi", expr, r"3 +4 =7")?;
    return Ok(());

}

#[test]
fn p12_not_equal () -> Result<()> {
    let expr = r#"<math><mn>5</mn><mo>&#x2212;</mo><mn>2</mn><mo>&#8800;</mo><mn>2</mn></math>"#;
    test_braille("ASCIIMath-fi", expr, r"5 -2 !=2")?;
    return Ok(());

}

#[test]
fn p12_opposite () -> Result<()> {
    let expr = r#"<math><mn>9</mn><mo>&#x2212;</mo><mn>3</mn><mo>&#x2260;</mo><mn>5</mn></math>"#;
    test_braille("ASCIIMath-fi", expr, r"9 -3 != 5")?;
    return Ok(());

}

#[test]
fn p12_multiplication_visible_op () -> Result<()> {
    let expr = r#"<math><mn>27</mn><mo>&#183;</mo><mn>3</mn></math>"#;
    test_braille("ASCIIMath-fi", expr, r"27 *3")?;
    return Ok(());

}

#[test]
fn p12_simple_frac () -> Result<()> {
    let expr = r#"<math><mfrac><mn>1</mn><mn>3</mn></mfrac></math>"#;
    test_braille("ASCIIMath-fi", expr, r"1/3")?;
    return Ok(());

}

#[test]
fn p12_ratio () -> Result<()> {
    let expr = r#"<math><mn>1</mn><mo>:</mo><mn>1000</mn></math>"#;
    test_braille_prefs("ASCIIMath-fi", expr, r"1 :1000")?;
    return Ok(());

}

#[test]
fn p12_fractional () -> Result<()> {
    let expr = r#"<math><mfrac><mrow><mn>6</mn><mo>&#8290;</mo><mi>x</mi><mo>+</mo><mn>3</mn><mo>&#8290;</mo><mi>x</mi></mrow><mrow><mrow><mn>6</mn><mo>&#8290;</mo><mi>x</mi><mo>&#x2212;</mo><mn>4</mn><mo>&#8290;</mo><mi>x</mi></mrow></mfrac></math>"#;
    test_braille("ASCIIMath-fi", expr, r"(6 x +3 x) /(6 x -4 x)")?;
    return Ok(());

}

#[test]
fn p12_absolute_value_eq () -> Result<()> {
    let expr = r#"<math><mo>|</mo><mo>&#x2212;</mo><mo>(</mo><mn>2</mn><mo>+</mo><mn>5</mn><mo>)</mo><mo>|</mo><mo>=</mo><mo>|</mo><mn>&#x2212;7</mn><mo>|</mo><mo>=</mo><mn>7</mn></math>"#;
    test_braille("ASCIIMath-fi", expr, r"|-(2 +5)| =|-7| =7")?;
    return Ok(());

}

#[test]
fn p12_natural_numbers () -> Result<()> {
    let expr = r#"<math><mi>&#x2115;</mi><mo>=</mo><mo>{</mo><mn>1</mn><mo>,</mo><mn>2</mn><mo>,</mo><mn>3</mn><mo>&#8230;</mo><mo>}</mo></math>"#;
    test_braille("ASCIIMath-fi", expr, r"NN ={0, 1, 2, 3, ...}")?;
    return Ok(());

}

#[test]
fn p12_whole_numbers () -> Result<()> {
    let expr = r#"<math><mi>&#8484;</mi><mo>=</mo><mo>{</mo><mo>&#8230;</mo><mo>,</mo><mo>&#x2212;</mo><mn>2</mn><mo>,</mo><mo>&#x2212;</mo><mn>1</mn><mo>,</mo><mn>0</mn><mo>,</mo><mn>1</mn><mo>,</mo><mn>2</mn><mo>,</mo><mo>&#8230;</mo><mo>}</mo></math>"#;
    test_braille("ASCIIMath-fi", expr, r"ZZ ={..., -2, 1, 0, 1, 2, ...}")?;
    return Ok(());

}

#[test]
fn p13_pi () -> Result<()> {
    let expr = r#"<math><mi>&#x3C0;</mi><mo>&#x2248;</mo><mn>3</mn><mo>,</mo><mn>14</mn></math>"#;
    test_braille_prefs("ASCIIMath", vec![("DecimalSeparators", ","), ("BlockSeparators", ". ")], expr, r"~p ~~3,14")?;
    return Ok(());

}

#[test]
fn p13_less_than () -> Result<()> {
    let expr = r#"<math><mi>x</mi><mo>&#60;</mo><mn>18</mn></math>"#;
    test_braille("ASCIIMath-fi", expr, r"x < 18")?;
    return Ok(());

}

#[test]
fn p13_greater_or_equal () -> Result<()> {
    let expr = r#"<math><mn>2</mn><mo>&#8290;</mo><mi>x</mi><mo>&#8805;</mo><mn>6</mn></math>"#;
    test_braille("ASCIIMath-fi", expr, r"2 x >= 6")?;
    return Ok(());

}

#[test]
fn p13_fraction_with_invisible_plus () -> Result<()> {
    let expr = r#"<math><mn>3</mn><mo>&#8292;</mo><mfrac><mn>5</mn><mn>6</mn></mfrac></math>"#;
    test_braille("ASCIIMath-fi", expr, r"3#5/6")?;
    return Ok(());

}

#[test]
fn p13_fraction_without_invisible_plus () -> Result<()> {
    let expr = r#"<math><mn>3</mn><mfrac><mn>5</mn><mn>6</mn></mfrac></math>"#;
    test_braille("ASCIIMath-fi", expr, r"3#5/6")?;
    return Ok(());

}

#[test]
fn p13_fractional_no_paren () -> Result<()> {
    // The numerator doesn't require parentheses to be read correctly.
    let expr = r#"<math><mfrac><mrow><mn>4</mn><mo>&#8290;</mo><mi>x</mi></mrow><mrow><mo >(</mo><mn>1</mn><mo>−</mo><mi>x</mi><mo>)</mo></mrow></mfrac></math>"#;
    test_braille("ASCIIMath-fi", expr, r"4 x /(1 -x)")?;
    return Ok(());

}

#[test]
fn p13_fractional () -> Result<()> {
    let expr = r#"<math><mfrac><mrow><mn>5</mn><mo>+</mo><mi>x</mi></mrow><mrow><mn>5</mn><mo>&#8290;</mo><mi>x</mi></mrow></mfrac></math>"#;
    test_braille("ASCIIMath-fi", expr, r"(5 +x) /(5 x)")?;
    return Ok(());

}

#[test]
fn p13_fractional_simplifying_with_paren () -> Result<()> {
    let expr = r#"<math><mfrac><mrow><mn>5</mn><mo>+</mo><mn>7</mn></mrow><mrow><mn>2</mn><mo>&#183;</mo><mi>3</mi></mrow></mfrac><mo>=</mo><mfrac><mn>12</mn><mn>6</mn></mfrac></math>"#;
    test_braille("ASCIIMath-fi", expr, r"(5 +7) /(2 *3) =12 /6")?;
    return Ok(());

}

#[test]
fn p14_long_fractional () -> Result<()> {
    let expr = r#"<math><mfrac><mfrac><mrow><msup><mi>x</mi><mn>2</mn></msup><mo>−</mo><mn>7</mn><mo>&#8290;</mo><mi>x</mi><mo>+</mo><mn>12</mn></mrow><mrow><mn>4</mn><mo>&#8290;</mo><mi>x</mi><mo>−</mo><mn>20</mn></mrow></mfrac><mfrac><mrow><msup><mi>x</mi><mn>2</mn></msup><mo>−</mo><mn>8</mn><mo>&#8290;</mo><mi>x</mi><mo>+</mo><mn>15</mn></mrow><mrow><mn>4</mn><mo>&#8290;</mo><mi>x</mi><mo>−</mo><mn>16</mn></mrow></mfrac></mfrac></math>"#;
    test_braille("ASCIIMath-fi", expr, r"((x^2 -7 x +12) /(4 x -20)) /((x^2 -8 x +15) /(4 x -16))")?;
    return Ok(());

}

#[test]
fn p15_exponent_plus () -> Result<()> {
    let expr = r#"<math><msup><mn>3</mn><mn>2</mn></msup><mo>+</mo><msup><mn>4</mn><mn>2</mn></msup</math>"#;
    test_braille("ASCIIMath-fi", expr, r"3^2 +4^2")?;
    return Ok(());

}

#[test]
fn p15_exponent_with_negative_base_in_paren () -> Result<()> {
    let expr = r#"<math><msup><mrow><mo>(</mo><mo>−</mo><mn>2</mn><mo>)</mo></mrow><mn>2</mn></msup></math>"#;
    test_braille("ASCIIMath-fi", expr, r"(-2)^2")?;
    return Ok(());

}

#[test]
fn p15_exponent_with_plus_equation () -> Result<()> {
    let expr = r#"<math><msup><mn>2</mn><mrow><mn>3</mn><mo>+</mo><mn>5</mn></mrow></msup></math>"#;
    test_braille("ASCIIMath-fi", expr, r"2^(3 +5)")?;
    return Ok(());

}

#[test]
fn p16_sqrt () -> Result<()> {
    let expr = r#"<math><msqrt><mn>25</mn></msqrt></math>"#;
    test_braille("ASCIIMath-fi", expr, r"sqrt(25)")?;
    return Ok(());

}

#[test]
fn p16_root3 () -> Result<()> {
    let expr = r#"<math><mroot><mn>27</mn><mn>3</mn></mroot></math>"#;
    test_braille("ASCIIMath-fi", expr, r"root3(27)")?;
    return Ok(());

}

#[test]
fn p16_root_equation () -> Result<()> {
    let expr = r#"<math><mroot><mn>32</mn><mn>5</mn></mroot><mo>+</mo><mroot><mn>1</mn><mn>6</mn></mroot></math>"#;
    test_braille("ASCIIMath-fi", expr, r"root5(32) +root6(1)")?;
    return Ok(());

}

#[test]
fn p18_tangent_90_degrees_infinity () -> Result<()> {
    let expr = r#"<math><mi>tan</mi><mo>&#8289;</mo><mo>⁡(</mo><mn>90</mn><mi>&#176;</mi><mo>)</mo><mo>=</mo><mi>∞</mi></math>"#;
    test_braille("ASCIIMath-fi", expr, r"tan 90^@ =oo")?;
    return Ok(());

}

#[test]
fn p18_degrees () -> Result<()> {
    let expr = r#"<math><mn>90</mn><mi>&#176;</mi></math>"#;
    test_braille("ASCIIMath-fi", expr, r"90 ^@")?;
    return Ok(());

}

#[test]
fn p18_cosines () -> Result<()> {
    let expr = r#"<math><msup><mi>cos</mi><mn>2</mn></msup><mo>&#8289;⁡</mo><mi>x</mi><mo>−</mo><mn>2</mn><mo>&#8290;</mo><mi>cos</mi><mo>&#8289;⁡</mo><mi>x</mi><mo>+</mo><mn>1</mn><mo>=</mo><mn>0</mn></math>
"#;
    test_braille("ASCIIMath-fi", expr, r"cos^2 x -2 cos x +1 =0")?;
    return Ok(());

}

#[test]
fn p19_vector_with_line () -> Result<()> {
    let expr = r#"<math><mover><mi>OB</mi><mo accent='false'>¯</mo></mover></math>"#;
    test_braille("ASCIIMath-fi", expr, r"vec OB")?;
    return Ok(());

}

#[test]
fn p19_vector_with_arrow () -> Result<()> {
    let expr = r#"<math><mover><mi>OB</mi><mo accent='false'>&#8594;</mo></mover></math>"#;
    test_braille("ASCIIMath-fi", expr, r"vec OB")?;
    return Ok(());

}

#[test]
fn p19_vector_projection () -> Result<()> {
    let expr = r#"<math><msub><mover><mi>a</mi><mo accent='false'>¯</mo></mover><mi>b</mi></msub></math>"#;
    test_braille("ASCIIMath-fi", expr, r"vec a_b")?;
    return Ok(());

}

#[test]
fn p19_unit_vector () -> Result<()> {
    let expr = r#"<math><msup><mover><mi>a</mi><mo accent='false'>¯</mo></mover><mn>0</mn></msup></math>"#;
    test_braille("ASCIIMath-fi", expr, r"vec a^0")?;
    return Ok(());

}

#[test]
fn p19_vector_dot_product () -> Result<()> {
    // Notice that dot product (middle dot) in vectors has space around the dot.
    let expr = r#"<math><mover><mi>a</mi><mo accent='false'>¯</mo></mover><mo>&#183;</mo><mover><mi>b</mi><mo accent='false'>¯</mo></mover></math>"#;
    test_braille("ASCIIMath-fi", expr, r"vec a * vec b")?;
    return Ok(());

}

#[test]
fn p19_vector_cross_product () -> Result<()> {
    let expr = r#"<math><mover><mi>a</mi><mo accent='false'>¯</mo></mover><mo>&#215;</mo><mover><mi>b</mi><mo accent='false'>¯</mo></mover></math>"#;
    test_braille("ASCIIMath-fi", expr, r"vec a xx vec b")?;
    return Ok(());

}

#[test]
fn p20_pair_of_equations () -> Result<()> {
    let expr = r#"<math><mover><mi>a</mi><mo accent='false'>¯</mo></mover><mo>&#215;</mo><mover><mi>b</mi><mo accent='false'>¯</mo></mover></math>"#;
    test_braille("ASCIIMath-fi", expr, r"{2 x +y =0, x -y =5}")?;
    return Ok(());

}

#[test]
fn p22_belongs_to_a_set () -> Result<()> {
    let expr = r#"<math><mi>x</mi><mo>&#8712;</mo><mi>A</mi></math>"#;
    test_braille("ASCIIMath-fi", expr, r"x in A")?;
    return Ok(());

}

#[test]
fn p22_does_not_belong_to_a_set () -> Result<()> {
    let expr = r#"<math><mn>3</mn><mo>&#8713;</mo><mi>B</mi></math>"#;
    test_braille("ASCIIMath-fi", expr, r"3 !in B")?;
    return Ok(());

}

#[test]
fn p22_subset_right () -> Result<()> {
    let expr = r#"<math><mi>A</mi><mo>&#8834;</mo><mi>B</mi></math>"#;
    test_braille("ASCIIMath-fi", expr, r"A sub B")?;
    return Ok(());

}

#[test]
fn p22_subset_left () -> Result<()> {
    let expr = r#"<math><mi>B</mi><mo>&#x2283;</mo><mi>A</mi></math>"#;
    test_braille("ASCIIMath-fi", expr, r"B sup A")?;
    return Ok(());

}

#[test]
fn p22_not_subset () -> Result<()> {
    let expr = r#"<math><mi>A</mi><mo>&#8836;</mo><mi>B</mi></math>"#;
    test_braille("ASCIIMath-fi", expr, r"B !sup A")?;
    return Ok(());

}

#[test]
fn p22_union () -> Result<()> {
    let expr = r#"<math><mi>A</mi><mo>&#8746;</mo><mi>B</mi><mo>=</mo><mo>{</mo><mi>a</mi><mo>,</mo><mi>b</mi><mo>,</mo><mi>c</mi><mo>}</mo></math>"#;
    test_braille("ASCIIMath-fi", expr, r"A uu B ={a, b, c}")?;
    return Ok(());

}



#[test]
fn p22_intersection_empty_set () -> Result<()> {
    let expr = r#"<math><mi>A</mi><mo>&#8745;</mo><mi>B</mi><mo>=</mo><mi>&#8709;</mi></math>"#;
    test_braille("ASCIIMath-fi", expr, r"G nn H =O")?;
    return Ok(());

}

#[test]
fn p22_negation () -> Result<()> {
    let expr = r#"<math><mo>&#172;</mo><mi>p</mi></math>"#;
    test_braille("ASCIIMath-fi", expr, r"not p")?;
    return Ok(());

}

#[test]
fn p23_logical_and () -> Result<()> {
    let expr = r#"<math><mi>p</mi><mo>&#8743;</mo><mi>q</mi></math>"#;
    test_braille("ASCIIMath-fi", expr, r"p ^^ q")?;
    return Ok(());

}

#[test]
fn p23_logical_or () -> Result<()> {
    let expr = r#"<math><mi>p</mi><mo>&#8744;</mo><mi>q</mi></math>"#;
    test_braille("ASCIIMath-fi", expr, r"p vv q")?;
    return Ok(());

}

#[test]
fn p23_logical_implication () -> Result<()> {
    let expr = r#"<math><mi>p</mi><mo>&#8594;</mo><mi>q</mi></math>"#;
    test_braille("ASCIIMath-fi", expr, r"p --> q")?;
    return Ok(());

}

#[test]
fn p23_function_definition () -> Result<()> {
    let expr = r#"<math><mi>f</mi><mo>:</mo><mi>x</mi><mo>→</mo><mi>f</mi><mo>(</mo><mi>x</mi><mo>)</mo></math>"#;
    test_braille("ASCIIMath-fi", expr, r"f: x -> f(x)")?;
    return Ok(());

}

#[test]
fn p23_4x4_matrix () -> Result<()> {
    let expr = r#"<math>
      <mrow>
      <mrow><mo>(</mo>
        <mtable>
          <mtr>
          <mtd>
            <mn>1</mn>
          </mtd>
          <mtd>
            <mn>0</mn>
          </mtd>
          <mtd>
            <mn>0</mn>
          </mtd>
          <mtd>
            <mn>1</mn>
          </mtd>
          </mtr>
          <mtr>
          <mtd>
            <mn>0</mn>
          </mtd>
          <mtd>
            <mn>1</mn>
          </mtd>
          <mtd>
            <mn>0</mn>
          </mtd>
          <mtd>
            <mn>0</mn>
          </mtd>
          </mtr>
          <mtr>
          <mtd>
            <mn>0</mn>
          </mtd>
          <mtd>
            <mn>0</mn>
          </mtd>
          <mtd>
            <mn>1</mn>
          </mtd>
          <mtd>
            <mn>0</mn>
          </mtd>
          </mtr>
          <mtr>
          <mtd>
            <mn>1</mn>
          </mtd>
          <mtd>
            <mn>0</mn>
          </mtd>
          <mtd>
            <mn>0</mn>
          </mtd>
          <mtd>
            <mn>1</mn>
          </mtd>
          </mtr>
        </mtable>
      <mo>)</mo></mrow></mrow>
    </math>"#;
    test_braille("ASCIIMath-fi", expr, r"((1 0 0 1), (0 1 0 0), (0 0 1 0), (1 0 0 1))")?;
    return Ok(());

}

#[test]
fn p24_function_definition () -> Result<()> {
    let expr = r#"<math>
      <mrow>
      <mrow><mo>|</mo>
        <mtable>
          <mtr>
          <mtd>
            <mi>a</mi>
            <mo>+</mo>
            <mi>b</mi>
          </mtd>
          <mtd>
            <mi>a</mi>
            <mo>-</mo>
            <mi>b</mi>
          </mtd>
          </mtr>
          <mtr>
          <mtd>
            <mi>a</mi>
            <mo>-</mo>
            <mi>b</mi>
          </mtd>
          <mtd>
            <mi>a</mi>
            <mo>+</mo>
            <mi>b</mi>
          </mtd>
          </mtr>
          
        </mtable>
      <mo>|</mo></mrow></mrow>
</math>"#;
    test_braille("ASCIIMath-fi", expr, r"|(a +b, a -b), (a -b, a +b)|")?;
    return Ok(());

}

#[test]
fn p25_binomial () -> Result<()> {
    // original display code contains forced spaces not in the output -- they are cleaned up here
    let expr = r#"<math>
        <mo minsize="2.047em" maxsize="2.047em">(</mo>
        <mfrac linethickness="0"><mi>n</mi><mi>k</mi></mfrac>
        <mo minsize="2.047em" maxsize="2.047em">)</mo>
    </math>"#;
    test_braille("ASCIIMath-fi", expr, r"((n), (k))")?;
    return Ok(());

}

#[test]
fn p25_factorial () -> Result<()> {
    let expr = r#"<math><mi>5</mi><mo>!</mo></math>"#;
    test_braille("ASCIIMath-fi", expr, r"5!")?;
    return Ok(());

}

#[test]
fn p25_conditional_probability () -> Result<()> {
    let expr = r#"<math><mi>P</mi><mo>&#8289;</mo><mo>(</mo><mi>A</mi><mo>|</mo><mi>B</mi><mo>)</mo></math>"#;
    test_braille("ASCIIMath-fi", expr, r"P(A | B)")?;
    return Ok(());

}

#[test]
fn p25_x_average () -> Result<()> {
    // This might prove to be wrong mark up, but this way there won't be mix up with vectors.
    let expr = r#"<math><mover><mi>x</mi><mo>¯</mo></mover></math>"#;
    test_braille("ASCIIMath-fi", expr, r"bar x")?;
    return Ok(());

}

#[test]
fn p26_expected_value () -> Result<()> {
    // This might prove to be wrong mark up, but this way there won't be mix up with vectors.
    let expr = r#"<math><mi>E</mi><mo>&#8289;</mo><mo>(</mo><mi>X</mi><mo>)</mo><mo>=</mo><mi>&#956;</mi><mo>=</mo><msub><mo>∑</mo><mi>i</mi></msub><mo>(</mo><msub><mi>p</mi><mi>i</mi></msub><mo>&#8290;</mo><msub><mi>x</mi><mi>i</mi></msub><mo>)</mo></math>"#;
    test_braille("ASCIIMath-fi", expr, r"E(X) =~m =sum_i (p_i x_i)")?;
    return Ok(());

}

#[test]
fn p26_msubsup () -> Result<()> {
    let expr = r#"<math><msubsup><mi>C</mi><mi>k</mi><mi>n</mi></msubsup></math>"#;
    test_braille("ASCIIMath-fi", expr, r"(C_k)^n")?;
    return Ok(());

}

#[test]
fn p26_derivation_fraction () -> Result<()> {
    // This might prove difficult, because of contradictory mark up in asciimath. If special case can't be coded, then this should regular rules for fractions and multiplication with variables.
    let expr = r#"<math><mfrac><mrow><mi>d</mi><mi>f</mi><mo>(</mo><mi>x</mi><mo>)</mo></mrow><mrow><mi>d</mi><mi>x</mi></mrow></mfrac></math>"#;
    test_braille("ASCIIMath-fi", expr, r"df(x)/dx")?;
    return Ok(());

}

#[test]
fn p26_derivation_prime_regular () -> Result<()> {
    // The ' doesn't have to be escaped, right? The r(aw string) does it already.
    let expr = r#"<math><msup><mi>f</mi><mo>'</mo></msup><mo>&#8289;</mo><mo>(</mo><mi>x</mi><mo>)</mo></math>"#;
    test_braille("ASCIIMath-fi", expr, r"f'(x)")?;
    return Ok(());

}

#[test]
fn p26_derivation_prime_alternative () -> Result<()> {
    // The ' doesn't have to be escaped, right? The r(aw string) does it already.
    let expr = r#"<math><msup><mi>f</mi><mi>&#x2032;</mi></msup><mo>&#8289;</mo><mo>(</mo><mi>x</mi><mo>)</mo></math>"#;
    test_braille("ASCIIMath-fi", expr, r"f'(x)")?;
    return Ok(());

}

#[test]
fn p26_derivation_prime_2_alternative () -> Result<()> {
    // The ' doesn't have to be escaped, right? The r(aw string) does it already.
    let expr = r#"<math><msup><mi>f</mi><mrow><mi>&#x2032;</mi><mi>&#x2032;</mi></mrow></msup><mo>&#8289;</mo><mo>(</mo><mi>x</mi><mo>)</mo></math>"#;
    test_braille("ASCIIMath-fi", expr, r"f''(x)")?;
    return Ok(());

}

#[test]
fn p26_derivation_cap_d () -> Result<()> {
    // Should there be an operator between D and f? Which one? Another question is that is D an operator or not? Here it is marked up as such.
    let expr = r#"<math><mo>D</mo><mi>f</mi><mo>(</mo><mi>x</mi><mo>)</mo></math>"#;
    test_braille("ASCIIMath-fi", expr, r"Df(x)")?;
    return Ok(());

}

#[test]
fn p26_derivation_cap_d_to_two () -> Result<()> {
    // Notice whitespace after D^2, compared to p26_derication_cap_d
    let expr = r#"<math><msup><mo>D</mo><mn>2</mn></msup><mi>f</mi><mo>(</mo><mi>x</mi><mo>)</mo></math>"#;
    test_braille("ASCIIMath-fi", expr, r"D^2 f(x)")?;
    return Ok(());

}

#[test]
fn p26_partial_derivatives () -> Result<()> {
    let expr = r#"<math><mfrac><mrow><mi>𝜕</mi><mi>y</mi></mrow><mrow><mi>𝜕</mi><mi>x</mi></mrow></mfrac></math>"#;
    test_braille("ASCIIMath-fi", expr, r"del y /(del x)")?;
    return Ok(());

}

#[test]
fn p26_gradient () -> Result<()> {
    let expr = r#"<math><mi>&#8711;</mi><mi>f</mi></math>"#;
    test_braille("ASCIIMath-fi", expr, r"grad f")?;
    return Ok(());

}

#[test]
fn p26_gradients_with_space () -> Result<()> {
    let expr = r#"<math><mi>&#8711;</mi><mi>f</mi><mi>&#8711;</mi><mi>g</mi></math>"#;
    test_braille("ASCIIMath-fi", expr, r"grad f grad g")?;
    return Ok(());

}

#[test]
fn p26_inverse_function () -> Result<()> {
    let expr = r#"<math>
  <mrow>
    <msup>
      <mi>f</mi>
      <mrow>
        <mo>−</mo>
        <mn>1</mn>
      </mrow>
    </msup>
    <mo>=</mo>
  </mrow>
  <mrow>
    <mo>{</mo>
    <mo>(</mo>
    <mi>y</mi>
    <mo>,</mo>
    <mi>x</mi>
    <mo>)</mo>
    <mo>∈</mo>
  </mrow>
  <mrow>
    <mi>B</mi>
    <mo>×</mo>
  </mrow>
  <mrow>
    <mi>A</mi>
    <mi>|</mi>
    <mi>y</mi>
    <mo>=</mo>
  </mrow>
  <mrow>
    <mi>f</mi>
    <mo>(</mo>
    <mi>x</mi>
    <mo>)</mo>
    <mo>}</mo>
  </mrow>
</math>"#;
    test_braille("ASCIIMath-fi", expr, r"f^-1 ={(y, x) in (B xx A) | y =f(x)}")?;
    return Ok(());

}

#[test]
fn p26_lg() -> Result<()> {
    let expr = r#"<math>
    <mi>lg</mi>
    <mo>&#8289;</mo>
    <mo>(</mo>
    <mn>5</mn>
    <mo>&#8290;</mo>
    <mi>a</mi>
    <mo>)</mo>
    <mo>=</mo>
    <mi>lg</mi>
    <mo>&#8289;</mo>
    <mi>a</mi>
    <mo>+</mo>
    <mi>lg</mi>
    <mo>&#8289;</mo>
    <mn>5</mn>
</math>"#;
    test_braille("ASCIIMath-fi", expr, r"lg (5 a) =lg a +lg 5")?;
    return Ok(());

}

#[test]
fn p26_log_additional() -> Result<()> {
    let expr = r#"<math>
    <mi>log</mi>
    <mo>&#8289;</mo>
    <mo>(</mo>
    <mn>5</mn>
    <mo>&#8290;</mo>
    <mi>a</mi>
    <mo>)</mo>
    <mo>=</mo>
    <mi>log</mi>
    <mo>&#8289;</mo>
    <mi>a</mi>
    <mo>+</mo>
    <mi>log</mi>
    <mo>&#8289;</mo>
    <mn>5</mn>
</math>"#;
    test_braille("ASCIIMath-fi", expr, r"log (5 a) =log a +log 5")?;
    return Ok(());

}

#[test]
fn p26_limit_from_positive_side() -> Result<()> {
    let expr = r#"<math>
  <mrow>
    <msub>
      <mi>lim</mi>
      <mrow>
        <mi>x</mi>
        <mo>→</mo>
        <msup>
          <mn>0</mn>
          <mo>+</mo>
        </msup>
      </mrow>
    </msub>
  </mrow>
  <mrow>
    <mo>=</mo>
  </mrow>
  <mrow>
    <mi>f</mi>
    <mo>(</mo>
    <mi>x</mi>
    <mo>)</mo>
  </mrow>
</math>"#;
    test_braille("ASCIIMath-fi", expr, r"lim_(x -> 0 +) f(x)")?;
    return Ok(());

}

#[test]
fn p26_limit_of_fractional() -> Result<()> {
    let expr = r#"<math>
  <mrow>
    <msub>
      <mi>lim</mi>
      <mrow>
        <mi>x</mi>
        <mo>→</mo>
        <mn>1</mn>
      </mrow>
    </msub>
  </mrow>
  <mrow>
    <mfrac>
      <mrow>
        <msup>
          <mi>x</mi>
          <mn>4</mn>
        </msup>
        <mo>−</mo>
        <mi>x</mi>
      </mrow>
      <mrow>
        <msup>
          <mi>x</mi>
          <mn>4</mn>
        </msup>
        <mo>−</mo>
        <mn>1</mn>
      </mrow>
    </mfrac>
  </mrow>
</math>"#;
    test_braille("ASCIIMath-fi", expr, r"lim_(x -> 1) [(x^4 -x) /(x^4 -1)]")?;
    return Ok(());

}

#[test]
fn p26_simple_integral() -> Result<()> {
    // Should the integrals 'dx' be in one <mi> or two?
    let expr = r#"<math>
  <mrow>
    <mo>∫</mo>
    <msup>
      <mi>x</mi>
      <mn>2</mn>
    </msup>
    <mi>dx</mi>
  </mrow>
</math>"#;
    test_braille("ASCIIMath-fi", expr, r"int x^2 dx")?;
    return Ok(());

}

#[test]
fn p26_integral_with_bounds() -> Result<()> {
    let expr = r#"<math>
  <mrow>
    <msubsup>
      <mo>∫</mo>
      <mi>π</mi>
      <mrow>
        <mn>2</mn>
        <mo>&#8290;</mo>
        <mi>π</mi>
      </mrow>
    </msubsup>
    <msup>
      <mi>tan</mi>
      <mn>2</mn>
    </msup>
  </mrow>
  <mo>&#8289;</mo>
    <mi>x</mi>
    <mi>dx</mi>
</math>"#;
    test_braille("ASCIIMath-fi", expr, r"int x^2 dx")?;
    return Ok(());

}

#[test]
fn p26_sum() -> Result<()> {
    let expr = r#"<math>
  <mrow>
    <msubsup>
      <mo>∑</mo>
      <mrow>
        <mi>i</mi>
        <mo>=</mo>
        <mn>0</mn>
      </mrow>
      <mi>n</mi>
    </msubsup>
    <mo>(</mo>
    <msub>
      <mi>f</mi>
      <mi>i</mi>
    </msub>
    <mo>&#8290;</mo>
    <msub>
      <mi>x</mi>
      <mi>i</mi>
    </msub>
    <mo>)</mo>
  </mrow>
</math>"#;
    test_braille("ASCIIMath-fi", expr, r"int x^2 dx")?;
    return Ok(());

}

#[test]
fn p26_sequence() -> Result<()> {
    let expr = r#"<math>
  <mrow>
    <mo>(</mo>
    <msub>
      <mi>x</mi>
      <mi>n</mi>
    </msub>
    <msubsup>
      <mo>)</mo>
      <mrow>
        <mi>n</mi>
        <mo>=</mo>
        <mn>1</mn>
      </mrow>
      <mi>∞</mi>
    </msubsup>
  </mrow>
</math>"#;
    test_braille("ASCIIMath-fi", expr, r"(x_n)_(n =1)^oo")?;
    return Ok(());

}

#[test]
fn p27_follows_normal_distribution() -> Result<()> {
    let expr = r#"<math>
  <mrow>
    <mi>p</mi>
    <mo>~</mo>
    <mi>N</mi>
    <mo>(</mo>
    <mn>58</mn>
    <mo>,</mo>
    <mn>2</mn>
    <mo>)</mo>
  </mrow>
</math>"#;
    test_braille("ASCIIMath-fi", expr, r"p ~ N(58, 2)")?;
    return Ok(());

}

#[test]
fn p27_quadratic_formula() -> Result<()> {
    let expr = r#"<math>
        <mi>x</mi>
        <mo>=</mo>
        <mfrac>
        <mrow>
            <mo>-</mo>
            <mi>b</mi>
            <mo>&#xB1;</mo>
            <msqrt><mrow><msup><mi>b</mi><mn>2</mn></msup><mo>-</mo><mn>4</mn><mi>a</mi><mi>c</mi></mrow></msqrt>
        </mrow>
        <mrow><mn>2</mn><mi>a</mi></mrow>
        </mfrac>
    </math>"#;
    test_braille("ASCIIMath-fi", expr, r"x =(-b +-sqrt(b^2 -4 a c)) /(2 a)")?;
    return Ok(());

}

#[test]
fn p35_atomic_numbers() -> Result<()> {
  let expr = r#" <math> <mrow>
      <msubsup>
          <mrow></mrow>
          <mrow><mn>92</mn></mrow>
          <mrow><mn>232</mn></mrow>
      </msubsup>
      <mrow><mi mathvariant="normal">U</mi></mrow>
      </mrow></math>"#;
  test_braille("ASCIIMath-fi", expr, r"_92^232U")?;
  return Ok(());

}

#[test]
fn p34_chem_single_bond_colon() -> Result<()> {
    let expr = r#"<math><mi>C</<mi><mo>:</mo><mi>C</mi></math>"#;
    test_braille("ASCIIMath-fi", expr, r"C;C")?;
    return Ok(());

}

#[test]
fn p34_chem_single_bond_dash() -> Result<()> {
    let expr = r#"<math><mi>C</<mi><mo>-</mo><mi>C</mi></math>"#;
    test_braille("ASCIIMath-fi", expr, r"C;C")?;
    return Ok(());

}

#[test]
fn p34_chem_double_bond_equal_sign() -> Result<()> {
    let expr = r#"<math><mi>C</<mi><mo>=</mo><mi>C</mi></math>"#;
    test_braille("ASCIIMath-fi", expr, r"C=C")?;
    return Ok(());

}

#[test]
fn p34_chem_double_bond_double_colon() -> Result<()> {
    let expr = r#"<math><mi>C</<mi><mo>::</mo><mi>C</mi></math>"#;
    test_braille("ASCIIMath-fi", expr, r"C=C")?;
    return Ok(());

}

#[test]
fn p34_chem_triple_bond() -> Result<()> {
    let expr = r#"<math><mi>C</<mi><mo>≡</mo><mi>C</mi></math>"#;
    test_braille("ASCIIMath-fi", expr, r"C;=C")?;
    return Ok(());

}

#[test]
fn p34_H2O() -> Result<()> {
    let expr = r#"<math><msub><mi>H</mi><mn>2</mn></msub><mi>O</mi></math>"#;
    test_braille("ASCIIMath-fi", expr, r"H_2O")?;
    return Ok(());

}

#[test]
fn p34_2NH_3() -> Result<()> {
    let expr = r#"<math><mn>2</mn><mo>&#8290;</mo><mi>N</mi><msub><mi>H</mi><mn>3</mn></msub></math>"#;
    test_braille("ASCIIMath-fi", expr, r"2 NH_3")?;
    return Ok(());

}

#[test]
fn p34_K_2Cr_2O_7() -> Result<()> {
    let expr = r#"<math><msub><mi>K</mi><mn>2</mn></msub><msub><mi>Cr</mi><mn>2</mn></msub><msub><mi>O</mi><mn>7</mn></msub></math>"#;
    test_braille("ASCIIMath-fi", expr, r"K_2Cr_2O_7")?;
    return Ok(());

}

#[test]
fn p34_Na_2CO_3_times_10H_2O() -> Result<()> {
    let expr = r#"<math><msub><mi>Na</mi><mn>2</mn></msub><msub><mi>Co</mi><mn>3</mn></msub><mo>&#183;</mo><mn>10</mn><mo>&#8290;</mo><msub><mi>H</mi><mn>2</mn></msub><mi>O</mi></math>"#;
    test_braille("ASCIIMath-fi", expr, r"Na_2CO_3 *10 H_2O")?;
    return Ok(());

}

#[test]
fn p34_Na_plus() -> Result<()> {
    let expr = r#"<math><msup><mi>Na</mi><mo>+</mo></msup></math>"#;
    test_braille("ASCIIMath-fi", expr, r"Na^+")?;
    return Ok(());

}

#[test]
fn p34_Cu_to_2_plus() -> Result<()> {
    let expr = r#"<math><msup><mi>Cu</mi><mrow><mn>2<mo>+</mo></mrow></msup></math>"#;
    test_braille("ASCIIMath-fi", expr, r"Cu^(2 +)")?;
    return Ok(());

}

#[test]
fn p35_Mg_S_chemical_equation() -> Result<()> {
    let expr = r#"<math>
      <mi>Mg</mi><mo>+</mo><mi>S</mi>
      <mo>&#x2192;</mo>
      <msup>
        <mi>Mg</mi>
        <mrow><mn>2</mn><mo>+</mo></mrow>
      </msup>
      <mo>+</mo>
      <msup>
        <mi>S</mi>
        <mrow><mn>2</mn><mo>+</mo></mrow>
      </msup>
      
    </math>"#;
    test_braille("ASCIIMath-fi", expr, r"Mg +S -> Mg^(2 +) +S^(2 -)")?;
    return Ok(());

}

#[test]
fn p34_Cu_to_2_plus() -> Result<()> {
    let expr = r#"<math><msup><mi>Ag</mi><mrow><mo>+</mo> </mrow></msup>
<msup><mi>S</mi><mrow><mo>+</mo> </mrow></msup><mo>&#x2192;</mo><mi>Ag</mi><mi>Cl</mi></math>"#;
    test_braille("ASCIIMath-fi", expr, r"Ag^+ +Cl^- -> AgCl")?;
    return Ok(());

}

#[test]
fn chem_equations_with_states() -> Result<()> {
    let expr = r#"<math>
    <mrow>
      <mn>2</mn>
      <mstyle scriptlevel='0'>
        <mspace width='0.167em'></mspace>
      </mstyle>
      <mrow>
        <mi>H</mi>
        <mi>Cl</mi>
      </mrow>
      <mspace width='0.111em'></mspace>
      <mo stretchy='false'>(</mo>
      <mrow>
        <mi>aq</mi>
      </mrow>
      <mo stretchy='false'>)</mo>
      <mrow></mrow>
      <mo>+</mo>
      <mrow></mrow>
      <mn>2</mn>
      <mstyle scriptlevel='0'>
        <mspace width='0.167em'></mspace>
      </mstyle>
      <mrow>
        <mi>Na</mi>
      </mrow>
      <mspace width='0.111em'></mspace>
      <mo stretchy='false'>(</mo>
      <mrow>
        <mi mathvariant='normal'>s</mi>
      </mrow>
      <mo stretchy='false'>)</mo>
      <mrow></mrow>
      <mrow>
        <mo stretchy='false'>&#x27F6;</mo>
      </mrow>
      <mrow></mrow>
      <mn>2</mn>
      <mstyle scriptlevel='0'>
        <mspace width='0.167em'></mspace>
      </mstyle>
      <mrow>
        <mi>Na</mi>
        <mi>Cl</mi>
      </mrow>
      <mspace width='0.111em'></mspace>
      <mo stretchy='false'>(</mo>
      <mrow>
        <mi>aq</mi>
      </mrow>
      <mo stretchy='false'>)</mo>
      <mrow></mrow>
      <mo>+</mo>
      <mrow></mrow>
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
      <mspace width='0.111em'></mspace>
      <mo stretchy='false'>(</mo>
      <mrow>
        <mi mathvariant='normal'>g</mi>
      </mrow>
      <mo stretchy='false'>)</mo>
    </mrow>
  </math>"#;
    test_braille("ASCIIMath-fi", expr, r"2 HCl (aq) +2 Na (s) -> 2 NaCl (aq) +H_2 (g)")?;
    return Ok(());

}

#[test]
fn p34_chem_text_over_arrow() -> Result<()> {
    let expr = r#"<math>
<mi>Ca</mi><msub><mi>Co</mi><mn>3</mn></msub><mo>(</mo><mi>s</mi><mo>)</mo>
    <mover><mo>&#x27F6;</mo><mtext>kuumennus</mtext></mover>
    <mi>Ca</mi><mi>O</mi><mo>(</mo><mi>s</mi><mo>)</mo><mo>+</mo><msub><mi>C</mi><mi>O</mi><mn>2</mn></msub><mo>(</mo><mi>g</mi><mo>)</mo>
</math>"#;
    test_braille("ASCIIMath-fi", expr, r"CaCO_3 (s) -> kuumennus -> CaO (s) +CO_2 (g)")?;
    return Ok(());

}

#[test]
fn some_greek_letters () -> Result<()> {
    let expr = r#"<math><mi>&#x3B1;</mi><mo>,</mo><mi>&#x3B2;</mi><mo>,</mo><mi>&#x3B3;</mi><mo>,</mo><mi>&#x3B4;</mi><mo>,</mo><mi>&#x3B5;</mi></math>"#;
    test_braille("ASCIIMath-fi", expr, r"~a, ~b, ~g, ~d, ~e")?;
    return Ok(());

}
