/// Tests for rules shared between various speech styles:
/// *  modified var
use crate::common::*;
use anyhow::Result;

// REVISAR (inglés: breve grave tilde): «a acento grave, b tilde, c breve, b ̌, c acento grave; más r carón más; x punto, y punto, z diéresis, u tres puntos, v cuatro puntos; más x acento circunflejo; más vector t»
#[test]
#[ignore = "lectura en español pendiente de revisión"]
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
    test("es", "SimpleSpeak", expr, 
        "a acento grave, b tilde, c breve, b ̌, c acento grave; más r carón más; x punto, y punto, z diéresis, u tres puntos, v cuatro puntos; más x acento circunflejo; más vector t")?;
            return Ok(());

}

// REVISAR (aproximaciones): «el límite cuando x aproximaciones 0, de, fracción, seno de x, partido por x, fin de fracción»
// REVISAR (aproximaciones): «el límite cuando x aproximaciones 0, de, fracción, seno de x, partido por x, fin de fracción»
#[test]
#[ignore = "lectura en español pendiente de revisión"]
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
    test("es", "SimpleSpeak", expr, "el límite cuando x aproximaciones 0, de, fracción, seno de x, partido por x, fin de fracción")?;
    test_prefs("es", "SimpleSpeak", vec![("Impairment", "LearningDisability")], expr,
            "el límite cuando x aproximaciones 0, de, fracción, seno de x, partido por x, fin de fracción")?;
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
    test("es", "SimpleSpeak", expr, "el límite cuando x enfoque desde abajo 0, de seno de x")?;
    return Ok(());

}


#[test]
fn binomial_mmultiscripts() -> Result<()> {
    let expr = "<math><mmultiscripts><mi>C</mi><mi>m</mi><none/><mprescripts/><mi>n</mi><none/></mmultiscripts></math>";
    test("es", "SimpleSpeak", expr, "n sobre m")?;
    return Ok(());

}

#[test]
fn binomial_mmultiscripts_other() -> Result<()> {
    let expr = "<math><mmultiscripts><mi>C</mi><mi>m</mi><none/><mprescripts/><none/><mi>n</mi></mmultiscripts></math>";
    test("es", "SimpleSpeak", expr, "n sobre m")?;
    return Ok(());

}

#[test]
fn binomial_subscript() -> Result<()> {  // C_{n,k}
    let expr = "<math><msub><mi>C</mi><mrow><mi>n</mi><mo>,</mo><mi>m</mi></mrow></msub></math>";
    test("es", "SimpleSpeak", expr, "n sobre m")?;
    return Ok(());

}

#[test]
fn permutation_mmultiscripts() -> Result<()> {
    let expr = "<math><mmultiscripts><mi>P</mi><mi>k</mi><none/><mprescripts/><mi>n</mi><none/></mmultiscripts></math>";
    test("es", "SimpleSpeak", expr, "k permutaciones de n")?;
    return Ok(());

}

#[test]
fn permutation_mmultiscripts_sup() -> Result<()> {
    let expr = "<math><mmultiscripts><mi>P</mi><mi>k</mi><none/><mprescripts/><none/><mi>n</mi></mmultiscripts></math>";
    test("es", "SimpleSpeak", expr, "k permutaciones de n")?;
    return Ok(());

}

#[test]
fn permutation_msubsup() -> Result<()> {
    let expr = "<math><msubsup><mi>P</mi><mi>k</mi><mi>n</mi></msubsup></math>";
    test("es", "SimpleSpeak", expr, "k permutaciones de n")?;
    return Ok(());

}

#[test]
fn tensor_mmultiscripts() -> Result<()> {
    let expr = "<math><mmultiscripts>
            <mi>R</mi> <mi>i</mi><none/> <none/><mi>j</mi> <mi>k</mi><none/> <mi>l</mi><none/> 
        </mmultiscripts></math>";
    test_prefs("es", "SimpleSpeak", vec![("Verbosity", "Verbose")], expr,
            "mayúscula r con 4 índices, subíndice i superíndice j subíndice k subíndice l")?;
    test_prefs("es", "SimpleSpeak", vec![("Verbosity", "Medium")], expr,
            "mayúscula r con 4 índices, subíndice i superíndice j subíndice k subíndice l")?;
            return Ok(());

}

// REVISAR (inglés: none): «mayúscula r con 4 preíndices, presubíndice mayúscula i, presuperíndice mayúscula j y preíndices alternados mayúscula k none mayúscula l none fin de preíndices y con 5 índices, subíndice i superíndice j subíndice k subíndice l e índices alternados m none fin de índices»
#[test]
#[ignore = "lectura en español pendiente de revisión"]
fn huge_num_mmultiscripts() -> Result<()> {
    let expr = "<math><mmultiscripts>
            <mi>R</mi> <mi>i</mi><none/> <none/><mi>j</mi> <mi>k</mi><none/> <mi>l</mi><none/> <mi>m</mi><none/>
            <mprescripts/> <mi>I</mi><none/> <none/><mi>J</mi> <mi>K</mi><none/> <mi>L</mi><none/>
        </mmultiscripts></math>";
    test_prefs("es", "SimpleSpeak", vec![("Verbosity", "Verbose")], expr,
            "mayúscula r con 4 preíndices, presubíndice mayúscula i, presuperíndice mayúscula j y preíndices alternados mayúscula k none mayúscula l none fin de preíndices y con 5 índices, subíndice i superíndice j subíndice k subíndice l e índices alternados m none fin de índices")?;
            return Ok(());

}

#[test]
fn prime() -> Result<()> {
    let expr = "<math> <msup><mi>x</mi><mo >&#x2032;</mo></msup> </math>";
    test("es", "SimpleSpeak", expr, "x prima")?;
    return Ok(());

}

#[test]
fn given() -> Result<()> {
    let expr = "<math><mi>P</mi><mo>(</mo><mi>A</mi><mo>|</mo><mi>B</mi><mo>)</mo></math>";
    test("es", "SimpleSpeak", expr, "mayúscula p; se abren paréntesis; mayúscula a barra vertical mayúscula b; se cierran paréntesis")?;
    test("es", "ClearSpeak", expr,  "mayúscula p; se abren paréntesis; mayúscula a barra vertical mayúscula b; se cierran paréntesis")?; // not good, but follows the spec
    return Ok(());

}

// REVISAR (-th, inglés: th): «x subíndice k, a la i-th potencia»
#[test]
#[ignore = "lectura en español pendiente de revisión"]
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
    test("es", "ClearSpeak", expr, "x subíndice k, a la i-th potencia")?;
    return Ok(());

}

// REVISAR (-th, inglés: th): «i subíndice j menos 2 fin de subíndice, a la k-th»
// REVISAR (-th, inglés: th): «i subíndice j menos 2 fin de subíndice, a la k-th potencia»
// REVISAR (-th, inglés: th): «i subíndice j menos 2, a la k-th»
#[test]
#[ignore = "lectura en español pendiente de revisión"]
fn non_simple_msubsup() -> Result<()> {
  let expr = "<math><msubsup><mi>i</mi><mrow><mi>j</mi><mo>&#x2212;</mo><mn>2</mn></mrow><mi>k</mi></msubsup></math>";
  test("es", "SimpleSpeak", expr, "i subíndice j menos 2 fin de subíndice, a la k-th")?;
  test("es", "ClearSpeak", expr, "i subíndice j menos 2 fin de subíndice, a la k-th potencia")?;
  test_prefs("es", "SimpleSpeak", vec![("Impairment", "LearningDisability")], expr,
          "i subíndice j menos 2, a la k-th")?;
          return Ok(());

}

// REVISAR (-th, inglés: th): «x subíndice k, a la i-th potencia»
#[test]
#[ignore = "lectura en español pendiente de revisión"]
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
    test("es", "ClearSpeak", expr, "x subíndice k, a la i-th potencia")?;
    return Ok(());

}

// REVISAR (-th, inglés: th): «mayúscula i es igual a; menos b r más z a la mayúscula i-th potencia»
#[test]
#[ignore = "lectura en español pendiente de revisión"]
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
    test("es", "ClearSpeak", expr, "mayúscula i es igual a; menos b r más z a la mayúscula i-th potencia")?;
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
    test("es", "ClearSpeak", expr, "mayúscula c más mayúscula i más mayúscula x")?;
    Ok(())
}


// REVISAR (inglés: and): «mayúscula p; se abren paréntesis; mayúscula a and mayúscula b; se cierran paréntesis; es igual a; mayúscula p; se abren paréntesis, mayúscula a intersección mayúscula b; se cierran paréntesis; es igual a; mayúscula p de mayúscula a; mayúscula p de mayúscula b»
#[test]
#[ignore = "lectura en español pendiente de revisión"]
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
    test("es", "SimpleSpeak", expr, "mayúscula p; se abren paréntesis; mayúscula a and mayúscula b; se cierran paréntesis; es igual a; mayúscula p; se abren paréntesis, mayúscula a intersección mayúscula b; se cierran paréntesis; es igual a; mayúscula p de mayúscula a; mayúscula p de mayúscula b")?;
    return Ok(());

}

#[test]
fn ignore_mtext_period() -> Result<()> {
    let expr = "<math><mrow><mrow><mo>{</mo><mn>2</mn><mo>}</mo></mrow><mtext>.</mtext></mrow></math>";
    test("es", "SimpleSpeak", expr, "el conjunto 2")?;
    return Ok(());

}

// REVISAR (phi recta, inglés: phi): «phi recta de x, es igual a; c por, e elevado a la menos h al cuadrado, x al cuadrado potencia»
#[test]
#[ignore = "lectura en español pendiente de revisión"]
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
    test("es", "SimpleSpeak", expr, "phi recta de x, es igual a; c por, e elevado a la menos h al cuadrado, x al cuadrado potencia")?;
    return Ok(());

}

// REVISAR (divisible): «mayúscula p; se abren paréntesis, mayúscula a divisible mayúscula b; se cierran paréntesis; es igual a; la fracción con numerador; mayúscula p; se abren paréntesis, mayúscula a intersección mayúscula b; se cierran paréntesis; y denominador mayúscula p de mayúscula b»
#[test]
#[ignore = "lectura en español pendiente de revisión"]
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
    test("es", "ClearSpeak", expr, "mayúscula p; se abren paréntesis, mayúscula a divisible mayúscula b; se cierran paréntesis; es igual a; la fracción con numerador; mayúscula p; se abren paréntesis, mayúscula a intersección mayúscula b; se cierran paréntesis; y denominador mayúscula p de mayúscula b")?;
    return Ok(());

}


// REVISAR (inglés: closed interval open): «el closed open interval de 0 a 2 pi»
#[test]
#[ignore = "lectura en español pendiente de revisión"]
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
  test("es", "SimpleSpeak",expr, "el closed open interval de 0 a 2 pi")?;
  return Ok(());

}

#[test]
fn caret_and_hat() -> Result<()> {
  let expr = "<math><mi>x</mi><mo>^</mo><mn>2</mn><mo>+</mo><mover><mi>y</mi><mo>^</mo></mover></math>";
  test("es", "SimpleSpeak",expr, "x acento circunflejo 2, más y acento circunflejo")?;
  return Ok(());

}

#[test]
fn dots() -> Result<()> {
  let expr = "<math>
         <mover><mi>x</mi><mo>.</mo></mover><mo>+</mo>
         <mover><mi>x</mi><mo>..</mo></mover><mo>+</mo>
         <mover><mi>x</mi><mo>...</mo></mover>
    </math>";
  test("es", "SimpleSpeak",expr, "x punto, más x .., más x ...")?;
  return Ok(());
}

#[test]
fn mn_with_space() -> Result<()> {
  let expr = "<math><mn>1 234 567</mn></math>";
  test_prefs("es", "SimpleSpeak", vec![("DecimalSeparators", "."), ("BlockSeparators", " ,")], expr, "1234567")?;
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
  test_prefs("es", "SimpleSpeak", vec![("IgnoreBold", "false")],
             expr, "negrita x es igual a, 2 seno de negrita t, menos 1")?;
  test_prefs("es", "SimpleSpeak", vec![("IgnoreBold", "true")],
             expr, "negrita x es igual a, 2 seno de negrita t, menos 1")?;
             return Ok(());

}

#[test]
fn mn_with_block_and_decimal_separators() -> Result<()> {
  let expr = "<math><mn>1,234.56</mn></math>";                                       // may want to change this for another language
  test_prefs("es", "SimpleSpeak", vec![("DecimalSeparators", "."), ("BlockSeparators", " ,")], expr, "1234.56")?;
  return Ok(());

}

// REVISAR (inglés: div of): «div of mayúscula f»
// REVISAR (inglés: divergence of): «divergence of mayúscula f»
#[test]
#[ignore = "lectura en español pendiente de revisión"]
fn divergence() -> Result<()> {
  let expr = "<math><mo>&#x2207;</mo><mo>&#xB7;</mo><mi mathvariant='normal'>F</mi></math>";                                       // may want to change this for another language
  test_prefs("es", "SimpleSpeak", vec![("Verbosity", "Terse")], expr, "div of mayúscula f")?;
  test_prefs("es", "SimpleSpeak", vec![("Verbosity", "Verbose")], expr, "divergence of mayúscula f")?;
  return Ok(());

}

// REVISAR (inglés: curl of): «curl of mayúscula f»
// REVISAR (inglés: curl of): «curl of mayúscula f»
#[test]
#[ignore = "lectura en español pendiente de revisión"]
fn curl() -> Result<()> {
  let expr = "<math><mo>&#x2207;</mo><mo>&#xD7;</mo><mi mathvariant='normal'>F</mi></math>";          
  // may want to change this for another language
  test_prefs("es", "SimpleSpeak", vec![("Verbosity", "Terse")], expr, "curl of mayúscula f")?;
  test_prefs("es", "SimpleSpeak", vec![("Verbosity", "Verbose")], expr, "curl of mayúscula f")?;
  return Ok(());

}

// REVISAR (inglés: gradient of): «gradient of mayúscula f»
#[test]
#[ignore = "lectura en español pendiente de revisión"]
fn gradient() -> Result<()> {
  let expr = "<math><mo>&#x2207;</mo><mi mathvariant='normal'>F</mi></math>";          
  // may want to change this for another language
  test_prefs("es", "SimpleSpeak", vec![("Verbosity", "Terse")], expr, "del mayúscula f")?;
  test_prefs("es", "SimpleSpeak", vec![("Verbosity", "Verbose")], expr, "gradient of mayúscula f")?;
  return Ok(());

}

// REVISAR (inglés: to): «mayúscula a flecha derecha, perpendicular to, mayúscula b flecha derecha»
#[test]
#[ignore = "lectura en español pendiente de revisión"]
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
  test("es", "LiteralSpeak", expr, "mayúscula a flecha derecha, perpendicular to, mayúscula b flecha derecha")?;
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
  test("es", "LiteralSpeak", expr, "barra vertical; x por, y por z barra oblicua 2; más a; es paralelo a, b más x factorial; barra vertical")?;
  return Ok(());

}

// REVISAR (inglés: forced): «forced f, se abren paréntesis, x factorial, se cierran paréntesis»
#[test]
#[ignore = "lectura en español pendiente de revisión"]
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
  test("es", "LiteralSpeak", expr, "forced f, se abren paréntesis, x factorial, se cierran paréntesis")?;
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
  test("es", "LiteralSpeak", expr, "f, se abren paréntesis, x factorial, se cierran paréntesis")?;
  return Ok(());

}

// REVISAR (inglés: to): «mayúscula a flecha derecha, perpendicular to, mayúscula b flecha derecha»
#[test]
#[ignore = "lectura en español pendiente de revisión"]
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
  test("es", "SimpleSpeak", expr, "mayúscula a flecha derecha, perpendicular to, mayúscula b flecha derecha")?;
  return Ok(());

}

// REVISAR (inglés: forced): «forced f, se abren paréntesis, x factorial, se cierran paréntesis»
#[test]
#[ignore = "lectura en español pendiente de revisión"]
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
  test("es", "SimpleSpeak", expr, "forced f, se abren paréntesis, x factorial, se cierran paréntesis")?;
  return Ok(());

}
