/// Tests for rules shared between various speech styles:
/// *  modified var
use crate::common::*;
use anyhow::Result;

#[test]
fn modified_vars() -> Result<()> {
    let expr = "<math> <mrow>
        <mover> <mi>a</mi> <mo>`</mo> </mover>
        <mover> <mi>b</mi> <mo>~</mo> </mover>
        <mover> <mi>c</mi> <mo>&#x0306;</mo> </mover>
        <mover> <mi>b</mi> <mo>&#x030c;</mo> </mover>
        <mover> <mi>c</mi> <mo>`</mo> </mover>  <mo>+</mo>
        <mover> <mi>r</mi> <mo>ˇ</mo> </mover>  <mo>+</mo>
        <mover> <mi>x</mi> <mo>.</mo> </mover>
        <mover> <mi>y</mi> <mo>&#x2D9;</mo> </mover>
        <mover> <mi>z</mi> <mo>&#x00A8;</mo> </mover>
        <mover> <mi>u</mi> <mo>&#x20DB;</mo> </mover>
        <mover> <mi>v</mi> <mo>&#x20DC;</mo> </mover> <mo>+</mo>
        <mover> <mi>x</mi> <mo>^</mo> </mover> <mo>+</mo>
        <mover> <mi>t</mi> <mo>→</mo> </mover>
        </mrow> </math>";
    test("pl", "SimpleSpeak", expr, 
        "a grawis, b tylda, c znak krótkości; b haczek, c grawis; plus r haczek plus; x kropka, y kropka, z diereza, u potrójny kropka; v poczwórny kropka; plus x daszek, plus wektor t")?;
            return Ok(());

}

#[test]
fn limit() -> Result<()> {
    let expr = "<math>
            <munder>
            <mo>lim</mo>
            <mrow>  <mi>x</mi> <mo>&#x2192;</mo>  <mn>0</mn>  </mrow>
            </munder>
            <mrow>
            <mfrac>
                <mrow>  <mi>sin</mi>  <mo>&#x2061;</mo> <mi>x</mi> </mrow>
                <mi>x</mi>
            </mfrac>
            </mrow>
        </math>";
    test("pl", "SimpleSpeak", expr, "granica gdy x dąży do 0, z, ułamek, sinus z x, przez x, koniec ułamka")?;
    test_prefs("pl", "SimpleSpeak", vec![("Impairment", "LearningDisability")], expr,
            "granica gdy x dąży do 0, z, sinus z x, przez x")?;
            return Ok(());

}

#[test]
fn limit_from_below() -> Result<()> {
    let expr = "<math>
            <munder>
            <mo>lim</mo>
            <mrow>  <mi>x</mi> <mo>↗</mo>  <mn>0</mn>  </mrow>
            </munder>
            <mrow>
                <mrow>  <mi>sin</mi>  <mo>&#x2061;</mo> <mi>x</mi> </mrow>
            </mrow>
        </math>";
    test("pl", "SimpleSpeak", expr, "granica gdy x dąży od dołu 0, z sinus z x")?;
    return Ok(());

}


#[test]
fn binomial_mmultiscripts() -> Result<()> {
    let expr = "<math><mmultiscripts><mi>C</mi><mi>m</mi><none/><mprescripts/><mi>n</mi><none/></mmultiscripts></math>";
    test("pl", "SimpleSpeak", expr, "n po m")?;
    return Ok(());

}

#[test]
fn binomial_mmultiscripts_other() -> Result<()> {
    let expr = "<math><mmultiscripts><mi>C</mi><mi>m</mi><none/><mprescripts/><none/><mi>n</mi></mmultiscripts></math>";
    test("pl", "SimpleSpeak", expr, "n po m")?;
    return Ok(());

}

#[test]
fn binomial_subscript() -> Result<()> {  // C_{n,k}
    let expr = "<math><msub><mi>C</mi><mrow><mi>n</mi><mo>,</mo><mi>m</mi></mrow></msub></math>";
    test("pl", "SimpleSpeak", expr, "n po m")?;
    return Ok(());

}

#[test]
fn permutation_mmultiscripts() -> Result<()> {
    let expr = "<math><mmultiscripts><mi>P</mi><mi>k</mi><none/><mprescripts/><mi>n</mi><none/></mmultiscripts></math>";
    test("pl", "SimpleSpeak", expr, "k permutacji z n")?;
    return Ok(());

}

#[test]
fn permutation_mmultiscripts_sup() -> Result<()> {
    let expr = "<math><mmultiscripts><mi>P</mi><mi>k</mi><none/><mprescripts/><none/><mi>n</mi></mmultiscripts></math>";
    test("pl", "SimpleSpeak", expr, "k permutacji z n")?;
    return Ok(());

}

#[test]
fn permutation_msubsup() -> Result<()> {
    let expr = "<math><msubsup><mi>P</mi><mi>k</mi><mi>n</mi></msubsup></math>";
    test("pl", "SimpleSpeak", expr, "k permutacji z n")?;
    return Ok(());

}

#[test]
fn tensor_mmultiscripts() -> Result<()> {
    let expr = "<math><mmultiscripts>
            <mi>R</mi> <mi>i</mi><none/> <none/><mi>j</mi> <mi>k</mi><none/> <mi>l</mi><none/> 
        </mmultiscripts></math>";
    test_prefs("pl", "SimpleSpeak", vec![("Verbosity", "Verbose")], expr,
            "wielka r z 4 prawe indeksy, indeks dolny i indeks górny j indeks dolny k indeks dolny l")?;
    test_prefs("pl", "SimpleSpeak", vec![("Verbosity", "Medium")], expr,
            "wielka r z 4 prawe indeksy, indeks dolny i indeks górny j indeks dolny k indeks dolny l")?;
            return Ok(());

}

#[test]
fn huge_num_mmultiscripts() -> Result<()> {
    let expr = "<math><mmultiscripts>
            <mi>R</mi> <mi>i</mi><none/> <none/><mi>j</mi> <mi>k</mi><none/> <mi>l</mi><none/> <mi>m</mi><none/>
            <mprescripts/> <mi>I</mi><none/> <none/><mi>J</mi> <mi>K</mi><none/> <mi>L</mi><none/>
        </mmultiscripts></math>";
    test_prefs("pl", "SimpleSpeak", vec![("Verbosity", "Verbose")], expr,
            "wielka r z 4 lewe indeksy, lewy indeks dolny wielka i, lewy indeks górny wielka j i kolejne lewe indeksy wielka k none wielka l none koniec lewych indeksów i z 5 prawe indeksy, indeks dolny i indeks górny j indeks dolny k indeks dolny l i kolejne indeksy m none koniec indeksów")?;
            return Ok(());

}

#[test]
fn prime() -> Result<()> {
    let expr = "<math> <msup><mi>x</mi><mo >&#x2032;</mo></msup> </math>";
    test("pl", "SimpleSpeak", expr, "x prim")?;
    return Ok(());

}

#[test]
fn given() -> Result<()> {
    let expr = "<math><mi>P</mi><mo>(</mo><mi>A</mi><mo>|</mo><mi>B</mi><mo>)</mo></math>";
    test("pl", "SimpleSpeak", expr, "wielka p; nawias otwierający, wielka a pod warunkiem wielka b; nawias zamykający")?;
    test("pl", "ClearSpeak", expr,  "wielka p; nawias otwierający, wielka a pod warunkiem wielka b; nawias zamykający")?; // not good, but follows the spec
    return Ok(());

}

#[test]
fn simple_msubsup() -> Result<()> {
    let expr = "<math>
            <mstyle displaystyle='true' scriptlevel='0'>
            <msubsup>
                <mi>x</mi>
                <mrow>
                <mi>k</mi>
                </mrow>
                <mrow>
                <mi>i</mi>
                </mrow>
            </msubsup>
            </mstyle>
        </math>";
    test("pl", "ClearSpeak", expr, "x indeks dolny k, do potęgi i")?;
    return Ok(());

}

#[test]
fn non_simple_msubsup() -> Result<()> {
  let expr = "<math><msubsup><mi>i</mi><mrow><mi>j</mi><mo>&#x2212;</mo><mn>2</mn></mrow><mi>k</mi></msubsup></math>";
  test("pl", "SimpleSpeak", expr, "i indeks dolny j minus 2 koniec indeksu dolnego, do potęgi k")?;
  test("pl", "ClearSpeak", expr, "i indeks dolny j minus 2 koniec indeksu dolnego, do potęgi k")?;
  test_prefs("pl", "SimpleSpeak", vec![("Impairment", "LearningDisability")], expr,
          "i indeks dolny j minus 2, do potęgi k")?;
          return Ok(());

}

#[test]
fn presentation_mathml_in_semantics() -> Result<()> {
    let expr = "<math>
        <semantics>
            <annotation encoding='application/x-tex'>{\\displaystyle x_k^i}</annotation>
            <annotation-xml encoding='MathML-Presentation'>
                <msubsup>
                    <mi>x</mi>
                    <mrow>
                    <mi>k</mi>
                    </mrow>
                    <mrow>
                    <mi>i</mi>
                    </mrow>
                </msubsup>
            </annotation-xml>
        </semantics>
    </math>";
    test("pl", "ClearSpeak", expr, "x indeks dolny k, do potęgi i")?;
    return Ok(());

}

#[test]
fn roman_like_superscript_identifier_is_not_chemistry() -> Result<()> {
    // Regression test for https://github.com/daisy/MathCAT/issues/528
    let expr = "<math>
        <mi>I</mi>
        <mo>=</mo>
        <mo>−</mo>
        <mi>b</mi>
        <mi>r</mi>
        <mo>+</mo>
        <msup>
            <mi>z</mi>
            <mi>I</mi>
        </msup>
    </math>";
    test("pl", "ClearSpeak", expr, "wielka i równa się, minus b r plus z do potęgi wielka i")?;
    Ok(())
}

#[test]
fn roman_like_identifier_sequence_is_not_number() -> Result<()> {
    // Regression test for https://github.com/daisy/MathCAT/issues/528
    let expr = "<math>
        <mi>C</mi>
        <mo>+</mo>
        <mi>I</mi>
        <mo>+</mo>
        <mi>X</mi>
    </math>";
    test("pl", "ClearSpeak", expr, "wielka c plus wielka i plus wielka x")?;
    Ok(())
}


#[test]
fn ignore_period() -> Result<()> {
    // from https://en.wikipedia.org/wiki/Probability
    let expr = "<math>
    <semantics>
    <annotation encoding='application/x-tex'>{\\displaystyle x_k^i}</annotation>
    <annotation-xml encoding='MathML-Presentation'>
      <mrow>
        <mstyle displaystyle='true' scriptlevel='0'>
          <mi>P</mi>
          <mo stretchy='false'>(</mo>
          <mi>A</mi>
          <mrow>
            <mstyle displaystyle='false' scriptlevel='0'>
              <mtext>&nbsp;and&nbsp;</mtext>
            </mstyle>
          </mrow>
          <mi>B</mi>
          <mo stretchy='false'>)</mo>
          <mo>=</mo>
          <mi>P</mi>
          <mo stretchy='false'>(</mo>
          <mi>A</mi>
          <mo>∩<!-- ∩ --></mo>
          <mi>B</mi>
          <mo stretchy='false'>)</mo>
          <mo>=</mo>
          <mi>P</mi>
          <mo stretchy='false'>(</mo>
          <mi>A</mi>
          <mo stretchy='false'>)</mo>
          <mi>P</mi>
          <mo stretchy='false'>(</mo>
          <mi>B</mi>
          <mo stretchy='false'>)</mo>
          <mo>.</mo>
        </mstyle>
      </mrow>
      </annotation-xml>
    </semantics>  
  </math>";
    test("pl", "SimpleSpeak", expr, "wielka p; nawias otwierający, wielka a and wielka b; nawias zamykający; równa się; wielka p; nawias otwierający, wielka a przecięcie wielka b; nawias zamykający; równa się; wielka p z wielka a, wielka p z wielka b")?;
    return Ok(());

}

#[test]
fn ignore_mtext_period() -> Result<()> {
    let expr = "<math><mrow><mrow><mo>{</mo><mn>2</mn><mo>}</mo></mrow><mtext>.</mtext></mrow></math>";
    test("pl", "SimpleSpeak", expr, "zbiór 2")?;
    return Ok(());

}

#[test]
fn ignore_comma() -> Result<()> {
    // from https://en.wikipedia.org/wiki/Probability
    let expr = "<math>
    <mrow>
      <mstyle displaystyle='true' scriptlevel='0'>
        <mi>ϕ<!-- ϕ --></mi>
        <mo stretchy='false'>(</mo>
        <mi>x</mi>
        <mo stretchy='false'>)</mo>
        <mo>=</mo>
        <mi>c</mi>
        <msup>
          <mi>e</mi>
          <mrow>
            <mo>−<!-- − --></mo>
            <msup>
              <mi>h</mi>
              <mrow>
                <mn>2</mn>
              </mrow>
            </msup>
            <msup>
              <mi>x</mi>
              <mrow>
                <mn>2</mn>
              </mrow>
            </msup>
          </mrow>
        </msup>
        <mo>,</mo>
      </mstyle>
    </mrow>
</math>";
    test("pl", "SimpleSpeak", expr, "fi z x równa się; c razy, e do potęgi minus h do kwadratu, x do kwadratu")?;
    return Ok(());

}

#[test]
#[ignore] // issue #14
fn ignore_period_and_space() -> Result<()> {
    // from https://en.wikipedia.org/wiki/Probability
    let expr = "<math>
      <mrow>
        <mstyle displaystyle='true' scriptlevel='0'>
          <mi>P</mi>
          <mo stretchy='false'>(</mo>
          <mi>A</mi>
          <mo>∣<!-- ∣ --></mo>
          <mi>B</mi>
          <mo stretchy='false'>)</mo>
          <mo>=</mo>
          <mrow>
            <mfrac>
              <mrow>
                <mi>P</mi>
                <mo stretchy='false'>(</mo>
                <mi>A</mi>
                <mo>∩<!-- ∩ --></mo>
                <mi>B</mi>
                <mo stretchy='false'>)</mo>
              </mrow>
              <mrow>
                <mi>P</mi>
                <mo stretchy='false'>(</mo>
                <mi>B</mi>
                <mo stretchy='false'>)</mo>
              </mrow>
            </mfrac>
          </mrow>
          <mo>.</mo>
          <mspace width='thinmathspace'></mspace>
        </mstyle>
      </mrow>
</math>";
    test("pl", "ClearSpeak", expr, "cap p, open paren, cap eigh divides cap b, close paren; is equal to; the fraction with numerator; cap p, open paren, cap eigh intersection cap b; close paren; and denominator cap p of cap b")?;
    return Ok(());

}


#[test]
fn bug_199_2pi() -> Result<()> {
  let expr = "<math>
      <mrow>
        <mo stretchy=\"false\" form=\"prefix\">[</mo>
        <mspace width=\"0.333em\"></mspace>
        <mn>0</mn>
        <mspace width=\"0.333em\"></mspace>
        <mo>,</mo>
        <mspace width=\"0.333em\"></mspace>
        <mn>2</mn>
        <mi>π</mi>
        <mspace width=\"0.333em\"></mspace>
        <mo stretchy=\"false\" form=\"postfix\">)</mo>
      </mrow>
    </math>";
  test("pl", "SimpleSpeak",expr, "przedział lewostronnie domknięty prawostronnie otwarty od 0 do 2 pi")?;
  return Ok(());

}

#[test]
fn caret_and_hat() -> Result<()> {
  let expr = "<math><mi>x</mi><mo>^</mo><mn>2</mn><mo>+</mo><mover><mi>y</mi><mo>^</mo></mover></math>";
  test("pl", "SimpleSpeak",expr, "x daszek 2 plus y daszek")?;
  return Ok(());

}

#[test]
fn dots() -> Result<()> {
  let expr = "<math>
         <mover><mi>x</mi><mo>.</mo></mover><mo>+</mo>
         <mover><mi>x</mi><mo>..</mo></mover><mo>+</mo>
         <mover><mi>x</mi><mo>...</mo></mover>
    </math>";
  test("pl", "SimpleSpeak",expr, "x kropka, plus x .., plus x ...")?;
  return Ok(());
}

#[test]
fn mn_with_space() -> Result<()> {
  let expr = "<math><mn>1 234 567</mn></math>";
  test_prefs("pl", "SimpleSpeak", vec![("DecimalSeparators", "."), ("BlockSeparators", " ,")], expr, "1234567")?;
  return Ok(());

}

#[test]
fn ignore_bold() -> Result<()> {
  let expr = r#"<math>
				<mi mathvariant="bold-italic">x</mi>
				<mo>=</mo>
				<mn>2</mn>
				<mrow>
				<mi>𝒔𝒊𝒏</mi>
				<mo>&#x2061;</mo>
				<mrow><mi mathvariant="bold-italic">t</mi></mrow>
				</mrow>
				<mo>-</mo>
				<mn>1</mn>
			</math>"#; 
  test_prefs("pl", "SimpleSpeak", vec![("IgnoreBold", "false")],
             expr, "pogrubione x równa się, 2 sinus z pogrubione t; minus 1")?;
  test_prefs("pl", "SimpleSpeak", vec![("IgnoreBold", "true")],
             expr, "pogrubione x równa się, 2 sinus z pogrubione t; minus 1")?;
             return Ok(());

}

#[test]
fn mn_with_block_and_decimal_separators() -> Result<()> {
  let expr = "<math><mn>1,234.56</mn></math>";                                       // may want to change this for another language
  test_prefs("pl", "SimpleSpeak", vec![("DecimalSeparators", "."), ("BlockSeparators", " ,")], expr, "1234.56")?;
  return Ok(());

}

#[test]
fn divergence() -> Result<()> {
  let expr = "<math><mo>&#x2207;</mo><mo>&#xB7;</mo><mi mathvariant='normal'>F</mi></math>";                                       // may want to change this for another language
  test_prefs("pl", "SimpleSpeak", vec![("Verbosity", "Terse")], expr, "dywergencja wielka f")?;
  test_prefs("pl", "SimpleSpeak", vec![("Verbosity", "Verbose")], expr, "dywergencja z wielka f")?;
  return Ok(());

}

#[test]
fn curl() -> Result<()> {
  let expr = "<math><mo>&#x2207;</mo><mo>&#xD7;</mo><mi mathvariant='normal'>F</mi></math>";          
  // may want to change this for another language
  test_prefs("pl", "SimpleSpeak", vec![("Verbosity", "Terse")], expr, "rotacja wielka f")?;
  test_prefs("pl", "SimpleSpeak", vec![("Verbosity", "Verbose")], expr, "rotacja z wielka f")?;
  return Ok(());

}

#[test]
fn gradient() -> Result<()> {
  let expr = "<math><mo>&#x2207;</mo><mi mathvariant='normal'>F</mi></math>";          
  // may want to change this for another language
  test_prefs("pl", "SimpleSpeak", vec![("Verbosity", "Terse")], expr, "gradient wielka f")?;
  test_prefs("pl", "SimpleSpeak", vec![("Verbosity", "Verbose")], expr, "gradient z wielka f")?;
  return Ok(());

}

#[test]
fn literal_speak_perpendicular() -> Result<()> {
  let expr = r#"<math data-latex='\vec{A} \perp \vec{B}' display='block'>
  <mrow data-changed='added'>
    <mover data-latex='\vec{A}'>
      <mi data-latex='A'>A</mi>
      <mo stretchy='false'>→</mo>
    </mover>
    <mo intent='perpendicular-to'>⊥</mo>
    <mover data-latex='\vec{B}'>
      <mi data-latex='B'>B</mi>
      <mo stretchy='false'>→</mo>
    </mover>
  </mrow>
 </math>"#; 
  test("pl", "LiteralSpeak", expr, "wielka a strzałka w prawo, perpendicular to, wielka b strzałka w prawo")?;
  return Ok(());

}

#[test]
fn literal_speak_chars() -> Result<()> {
  let expr = r#"<math>
        <mfenced open="|" close="|">
            <mrow>
                <mi>x</mi><mo>&#xD7;</mo><mi>y</mi>
                <mo>&#xB7;</mo>
                <mi>z</mi><mo>/</mo><mn>2</mn>
                <mo>+</mo>
                <mi>a</mi><mo>&#x2225;</mo><mi>b</mi>
                <mo>+</mo>
                <mi>x</mi><mo>!</mo>
            </mrow>
        </mfenced>
    </math>"#; 
  test("pl", "LiteralSpeak", expr, "kreska pionowa; x krzyż, y kropka z ukośnik 2; plus a; podwójna pionowa kreska, b plus x wykrzyknik; kreska pionowa")?;
  return Ok(());

}

#[test]
fn literal_speak_with_name() -> Result<()> {
  let expr = r#"<math intent='forced($x)'>
      <mrow arg="x">
        <mi>f</mi>
        <mo data-changed='added'>&#x2061;</mo>
        <mrow data-changed='added'>
          <mo>(</mo>
          <mrow data-changed='added'>
            <mi>x</mi>
            <mo>!</mo>
          </mrow>
          <mo>)</mo>
        </mrow>
      </mrow>
    </math>"#;
  test("pl", "LiteralSpeak", expr, "forced f, lewy nawias x wykrzyknik, prawy nawias")?;
  return Ok(());

}

#[test]
fn literal_speak_with_property() -> Result<()> {
  let expr = r#"<math intent=':prefix'>
      <mrow arg="x">
        <mi>f</mi>
        <mo data-changed='added'>&#x2061;</mo>
        <mrow data-changed='added'>
          <mo>(</mo>
          <mrow data-changed='added'>
            <mi>x</mi>
            <mo>!</mo>
          </mrow>
          <mo>)</mo>
        </mrow>
      </mrow>
    </math>"#; 
  test("pl", "LiteralSpeak", expr, "f, lewy nawias x wykrzyknik, prawy nawias")?;
  return Ok(());

}

#[test]
fn literal_intent_property() -> Result<()> {
  let expr = r#"<math data-latex='\vec{A} \perp \vec{B}' display='block'>
  <mrow intent=":literal">
    <mover data-latex='\vec{A}'>
      <mi data-latex='A'>A</mi>
      <mo stretchy='false'>→</mo>
    </mover>
    <mo intent='perpendicular-to'>⊥</mo>
    <mover data-latex='\vec{B}'>
      <mi data-latex='B'>B</mi>
      <mo stretchy='false'>→</mo>
    </mover>
  </mrow>
 </math>"#; 
  test("pl", "SimpleSpeak", expr, "wielka a strzałka w prawo, perpendicular to, wielka b strzałka w prawo")?;
  return Ok(());

}

#[test]
fn literal_intent_property_with_name() -> Result<()> {
  let expr = r#"<math intent='forced:literal($x)'>
      <mrow arg="x">
        <mi>f</mi>
        <mo data-changed='added'>&#x2061;</mo>
        <mrow data-changed='added'>
          <mo>(</mo>
          <mrow data-changed='added'>
            <mi>x</mi>
            <mo>!</mo>
          </mrow>
          <mo>)</mo>
        </mrow>
      </mrow>
    </math>"#; 
  test("pl", "SimpleSpeak", expr, "forced f, nawias otwierający, x wykrzyknik, nawias zamykający")?;
  return Ok(());

}
