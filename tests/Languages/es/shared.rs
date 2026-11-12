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
    test("es", "SimpleSpeak", expr, 
        "a acento grave, b tilde, c breve, b carón, c acento grave; más r carón más; x punto, y punto, z diéresis, u tres puntos, v cuatro puntos; más x sombrero, más vector t")?;
            return Ok(());

}

#[test]
fn ordinals() -> Result<()> {
    // ordinales como llegan desde LaTeX (luamml): la letra marca el género, el plural y el apócope
    let cases = [
        // 1\text{.\textsuperscript{o}}
        ("<math><mn>1</mn><mtext>.o</mtext></math>", "primero"),
        ("<math><mn>2</mn><mtext>.a</mtext></math>", "segunda"),
        ("<math><mn>3</mn><mtext>.er</mtext></math>", "tercer"),
        ("<math><mn>2</mn><mtext>.os</mtext></math>", "segundos"),
        ("<math><mn>1</mn><mtext>.as</mtext></math>", "primeras"),
        ("<math><mn>13</mn><mtext>.er</mtext></math>", "decimotercer"),
        ("<math><mn>21</mn><mtext>.a</mtext></math>", "vigésima primera"),
        ("<math><mn>30</mn><mtext>.o</mtext></math>", "trigésimo"),
        ("<math><mn>99</mn><mtext>.o</mtext></math>", "nonagésimo noveno"),
        // 1.\text{º}, \text{1.\textsuperscript{o}}
        ("<math><mn>1</mn><mo>.</mo><mtext>º</mtext></math>", "primero"),
        ("<math><mn>2</mn><mo>.</mo><mtext>ª</mtext></math>", "segunda"),
        ("<math><mtext>1.o</mtext></math>", "primero"),
        // 1.^{\text{o}} y 1^{\text{o}}
        ("<math><mn>1</mn><msup><mo>.</mo><mtext>o</mtext></msup></math>", "primero"),
        ("<math><mn>3</mn><msup><mo>.</mo><mtext>er</mtext></msup></math>", "tercer"),
        ("<math><msup><mn>1</mn><mtext>o</mtext></msup></math>", "primero"),
        // dentro de una fórmula
        ("<math><msub><mi>a</mi><mrow><mn>1</mn><mtext>.o</mtext></mrow></msub><mo>+</mo><msub><mi>a</mi><mrow><mn>2</mn><mtext>.o</mtext></mrow></msub></math>",
            "a sub primero fin de sub; más, a sub segundo fin de sub"),
        ("<math><mtext>el </mtext><mn>1</mn><mtext>.er término es </mtext><msub><mi>a</mi><mn>1</mn></msub></math>",
            "el primer término es a sub 1"),
        // no son ordinales
        ("<math><mn>100</mn><mtext>.o</mtext></math>", "100 .o"),
        ("<math><mn>2</mn><mo>.</mo><mi>a</mi></math>", "2 punto a"),
        ("<math><msup><mn>2</mn><mi>a</mi></msup></math>", "2 elevado a a"),
    ];
    for (expr, speech) in cases {
        test("es", "ClearSpeak", expr, speech)?;
        test("es", "SimpleSpeak", expr, speech)?;
    }
    return Ok(());
}

#[test]
fn limit_sup_inf() -> Result<()> {
    // \limsup y \liminf: "límite superior/inferior" (Rules/Intent los llama limit-sup y limit-inf), con "de" como lim
    let sup = "<math><munder><mo>lim sup</mo><mrow><mi>n</mi><mo>→</mo><mi>∞</mi></mrow></munder><msub><mi>a</mi><mi>n</mi></msub></math>";
    let inf = "<math><munder><mo>lim&#x2009;inf</mo><mrow><mi>n</mi><mo>→</mo><mi>∞</mi></mrow></munder><msub><mi>a</mi><mi>n</mi></msub></math>";
    let mi = "<math><munder><mi>limsup</mi><mrow><mi>n</mi><mo>→</mo><mi>∞</mi></mrow></munder><msub><mi>a</mi><mi>n</mi></msub></math>";
    test("es", "ClearSpeak", sup, "el límite superior cuando n tiende a infinito, de a sub n")?;
    test("es", "SimpleSpeak", inf, "el límite inferior cuando n tiende a infinito, de a sub n")?;
    test("es", "ClearSpeak", mi, "el límite superior cuando n tiende a infinito, de a sub n")?;
    test_prefs("es", "SimpleSpeak", vec![("Verbosity", "Terse")], sup, "lím sup cuando n tiende a infinito; a sub n")?;
    return Ok(());
}

#[test]
fn min_max_with_condition() -> Result<()> {
    // la condición va al final, como se dice: "el mínimo de f de x cuando x pertenece a A"
    let min = "<math><munder><mo movablelimits=\"true\">min</mo><mrow><mi>x</mi><mo>∈</mo><mi>A</mi></mrow></munder><mi>f</mi><mo>(</mo><mi>x</mi><mo>)</mo></math>";
    let sup = "<math><munder><mo movablelimits=\"true\">sup</mo><mrow><mi>x</mi><mo>&gt;</mo><mn>0</mn></mrow></munder><mi>f</mi><mo>(</mo><mi>x</mi><mo>)</mo></math>";
    let plus = "<math><munder><mo movablelimits=\"true\">min</mo><mrow><mi>x</mi><mo>∈</mo><mi>A</mi></mrow></munder><mi>f</mi><mo>(</mo><mi>x</mi><mo>)</mo><mo>+</mo><mn>1</mn></math>";
    test("es", "ClearSpeak", min, "el mínimo de f de x, cuando x pertenece a mayúscula a")?;
    test("es", "SimpleSpeak", sup, "el supremo de f de x, cuando x es mayor que 0")?;
    test("es", "ClearSpeak", plus, "el mínimo de f de x, cuando x pertenece a mayúscula a; más 1")?;
    test_prefs("es", "SimpleSpeak", vec![("Verbosity", "Terse")], min, "mín de f x, cuando x pertenece a mayúscula a")?;
    return Ok(());
}

#[test]
fn accented_operators_babel() -> Result<()> {
    // babel en español escribe lím, lím sup, lím ínf, mín, máx e ínf con tilde
    let lim = |op: &str| format!("<math><munder><mo>{op}</mo><mrow><mi>n</mi><mo>→</mo><mi>∞</mi></mrow></munder><msub><mi>a</mi><mi>n</mi></msub></math>");
    let op = |op: &str| format!("<math><munder><mo movablelimits=\"true\">{op}</mo><mrow><mi>x</mi><mo>∈</mo><mi>A</mi></mrow></munder><mi>f</mi><mo>(</mo><mi>x</mi><mo>)</mo></math>");
    test("es", "ClearSpeak", &lim("lím sup"), "el límite superior cuando n tiende a infinito, de a sub n")?;
    test("es", "ClearSpeak", &lim("lím ínf"), "el límite inferior cuando n tiende a infinito, de a sub n")?;
    test("es", "SimpleSpeak", &lim("lím&#x2009;sup"), "el límite superior cuando n tiende a infinito, de a sub n")?;
    test("es", "ClearSpeak", &op("mín"), "el mínimo de f de x, cuando x pertenece a mayúscula a")?;
    test("es", "SimpleSpeak", &op("máx"), "el máximo de f de x, cuando x pertenece a mayúscula a")?;
    test("es", "ClearSpeak", &op("ínf"), "el ínfimo de f de x, cuando x pertenece a mayúscula a")?;
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
    test("es", "SimpleSpeak", expr, "el límite cuando x tiende a 0, de, fracción, seno de x, partido por x, fin de fracción")?;
    test_prefs("es", "SimpleSpeak", vec![("Impairment", "LearningDisability")], expr,
            "el límite cuando x tiende a 0, de; seno de x, partido por x")?;
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
    test("es", "SimpleSpeak", expr, "el límite cuando x tiende por la izquierda a 0, de seno de x")?;
    return Ok(());

}

// prueba propia del español: "0⁻" y "0⁺" dentro de un límite
#[test]
fn limit_one_sided_superscript() -> Result<()> {
    let expr = "<math>
            <munder>
            <mo>lim</mo>
            <mrow>  <mi>x</mi> <mo>→</mo>  <msup><mn>0</mn><mo>-</mo></msup>  </mrow>
            </munder>
            <mrow>  <mi>sin</mi>  <mo>&#x2061;</mo> <mi>x</mi> </mrow>
        </math>";
    test("es", "SimpleSpeak", expr, "el límite cuando x tiende a 0 por la izquierda, de seno de x")?;
    let expr = "<math>
            <munder>
            <mo>lim</mo>
            <mrow>  <mi>x</mi> <mo>→</mo>  <msup><mn>0</mn><mo>+</mo></msup>  </mrow>
            </munder>
            <mrow>  <mi>sin</mi>  <mo>&#x2061;</mo> <mi>x</mi> </mrow>
        </math>";
    test("es", "ClearSpeak", expr, "el límite cuando x tiende a 0 por la derecha, de seno x")?;
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
            "mayúscula r con 4 índices, sub i superíndice j sub k sub l")?;
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
    test("es", "SimpleSpeak", expr, "mayúscula p; abrir paréntesis, mayúscula a dado mayúscula b; cerrar paréntesis")?;
    test("es", "ClearSpeak", expr,  "mayúscula p; abrir paréntesis, mayúscula a dado mayúscula b; cerrar paréntesis")?; // not good, but follows the spec
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
    test("es", "ClearSpeak", expr, "x sub k, elevado a i")?;
    return Ok(());

}

#[test]
fn non_simple_msubsup() -> Result<()> {
  let expr = "<math><msubsup><mi>i</mi><mrow><mi>j</mi><mo>&#x2212;</mo><mn>2</mn></mrow><mi>k</mi></msubsup></math>";
  test("es", "SimpleSpeak", expr, "i sub j menos 2 fin de sub, elevado a k")?;
  test("es", "ClearSpeak", expr, "i sub j menos 2 fin de sub, elevado a k")?;
  test_prefs("es", "SimpleSpeak", vec![("Impairment", "LearningDisability")], expr,
          "i sub j menos 2, elevado a k")?;
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
    test("es", "ClearSpeak", expr, "x sub k, elevado a i")?;
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
    test("es", "ClearSpeak", expr, "mayúscula i es igual a, menos b r más z elevado a mayúscula i")?;
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
              <mtext>&nbsp;y&nbsp;</mtext>
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
    test("es", "SimpleSpeak", expr, "mayúscula p; abrir paréntesis, mayúscula a y mayúscula b; cerrar paréntesis; es igual a; mayúscula p; abrir paréntesis, mayúscula a intersección mayúscula b; cerrar paréntesis; es igual a; mayúscula p de mayúscula a; mayúscula p de mayúscula b")?;
    return Ok(());

}

#[test]
fn ignore_mtext_period() -> Result<()> {
    let expr = "<math><mrow><mrow><mo>{</mo><mn>2</mn><mo>}</mo></mrow><mtext>.</mtext></mrow></math>";
    test("es", "SimpleSpeak", expr, "el conjunto 2")?;
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
    test("es", "SimpleSpeak", expr, "fi de x es igual a; c por, e elevado a menos h al cuadrado, x al cuadrado fin de exponente")?;
    return Ok(());

}

#[test]
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
    test("es", "ClearSpeak", expr, "mayúscula p; abrir paréntesis, mayúscula a divide a mayúscula b; cerrar paréntesis; es igual a; la fracción con numerador; mayúscula p; abrir paréntesis, mayúscula a intersección mayúscula b; cerrar paréntesis; y denominador mayúscula p de mayúscula b")?;
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
  test("es", "SimpleSpeak",expr, "el intervalo abierto por la derecha de extremos 0 y 2 pi")?;
  return Ok(());

}

#[test]
fn caret_and_hat() -> Result<()> {
  let expr = "<math><mi>x</mi><mo>^</mo><mn>2</mn><mo>+</mo><mover><mi>y</mi><mo>^</mo></mover></math>";
  test("es", "SimpleSpeak",expr, "x acento circunflejo 2, más y sombrero")?;
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
             expr, "x es igual a, 2 seno de t, menos 1")?;
             return Ok(());

}

#[test]
fn mn_with_block_and_decimal_separators() -> Result<()> {
  let expr = "<math><mn>1,234.56</mn></math>";                                       // may want to change this for another language
  test_prefs("es", "SimpleSpeak", vec![("DecimalSeparators", "."), ("BlockSeparators", " ,")], expr, "1234.56")?;
  return Ok(());

}

#[test]
fn divergence() -> Result<()> {
  let expr = "<math><mo>&#x2207;</mo><mo>&#xB7;</mo><mi mathvariant='normal'>F</mi></math>";                                       // may want to change this for another language
  test_prefs("es", "SimpleSpeak", vec![("Verbosity", "Terse")], expr, "div mayúscula f")?;
  test_prefs("es", "SimpleSpeak", vec![("Verbosity", "Verbose")], expr, "divergencia de mayúscula f")?;
  return Ok(());

}

#[test]
fn curl() -> Result<()> {
  let expr = "<math><mo>&#x2207;</mo><mo>&#xD7;</mo><mi mathvariant='normal'>F</mi></math>";          
  // may want to change this for another language
  test_prefs("es", "SimpleSpeak", vec![("Verbosity", "Terse")], expr, "rot mayúscula f")?;
  test_prefs("es", "SimpleSpeak", vec![("Verbosity", "Verbose")], expr, "rotacional de mayúscula f")?;
  return Ok(());

}

#[test]
fn gradient() -> Result<()> {
  let expr = "<math><mo>&#x2207;</mo><mi mathvariant='normal'>F</mi></math>";          
  // may want to change this for another language
  test_prefs("es", "SimpleSpeak", vec![("Verbosity", "Terse")], expr, "grad mayúscula f")?;
  test_prefs("es", "SimpleSpeak", vec![("Verbosity", "Verbose")], expr, "gradiente de mayúscula f")?;
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
    <mo intent='perpendicular-a'>⊥</mo>
    <mover data-latex='\vec{B}'>
      <mi data-latex='B'>B</mi>
      <mo stretchy='false'>→</mo>
    </mover>
  </mrow>
 </math>"#; 
  test("es", "LiteralSpeak", expr, "mayúscula a flecha derecha, perpendicular a, mayúscula b flecha derecha")?;
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
  test("es", "LiteralSpeak", expr, "barra vertical; x cruz, y punto z barra oblicua 2; más a; doble barra vertical, b más x signo de exclamación; barra vertical")?;
  return Ok(());

}

#[test]
fn literal_speak_with_name() -> Result<()> {
  let expr = r#"<math intent='forzado($x)'>
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
  test("es", "LiteralSpeak", expr, "forzado f, abrir paréntesis, x signo de exclamación, cerrar paréntesis")?;
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
  test("es", "LiteralSpeak", expr, "f, abrir paréntesis, x signo de exclamación, cerrar paréntesis")?;
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
    <mo intent='perpendicular-a'>⊥</mo>
    <mover data-latex='\vec{B}'>
      <mi data-latex='B'>B</mi>
      <mo stretchy='false'>→</mo>
    </mover>
  </mrow>
 </math>"#; 
  test("es", "SimpleSpeak", expr, "mayúscula a flecha derecha, perpendicular a, mayúscula b flecha derecha")?;
  return Ok(());

}

#[test]
fn literal_intent_property_with_name() -> Result<()> {
  let expr = r#"<math intent='forzado:literal($x)'>
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
  test("es", "SimpleSpeak", expr, "forzado f, abrir paréntesis, x signo de exclamación, cerrar paréntesis")?;
  return Ok(());

}

#[test]
fn coordinate_point() -> Result<()> {
  let expr = r#"<math><mrow intent="coordinate(1,2)"><mn>1</mn><mn>2</mn></mrow></math>"#;
  test_prefs("es", "ClearSpeak", vec![("Verbosity", "Medium")], expr, "punto, 1 coma 2")?;
  test_prefs("es", "ClearSpeak", vec![("Verbosity", "Verbose")], expr, "el punto de coordenadas, 1 coma 2, fin del punto")?;
  return Ok(());
}

// pruebas propias del español: abreviaturas con tilde o en español
#[test]
fn limit_with_accent() -> Result<()> {
  let expr = "<math><munder><mo>lím</mo><mrow><mi>x</mi><mo>→</mo><mn>0</mn></mrow></munder>
                <mi>f</mi><mo>&#x2061;</mo><mo>(</mo><mi>x</mi><mo>)</mo></math>";
  test("es", "ClearSpeak", expr, "el límite cuando x tiende a 0, de f de x")?;
  let expr = "<math><munder><mi>lím</mi><mrow><mi>x</mi><mo>→</mo><msup><mn>0</mn><mo>+</mo></msup></mrow></munder>
                <mfrac><mn>1</mn><mi>x</mi></mfrac></math>";
  test("es", "ClearSpeak", expr, "el límite cuando x tiende a 0 por la derecha, de 1 partido por x")?;
  return Ok(());
}

#[test]
fn spanish_function_names() -> Result<()> {
  let expr = "<math><mi>sen</mi><mo>&#x2061;</mo><mi>x</mi></math>";
  test("es", "ClearSpeak", expr, "seno x")?;
  let expr = "<math><msup><mi>sen</mi><mrow><mo>-</mo><mn>1</mn></mrow></msup><mo>&#x2061;</mo><mi>x</mi></math>";
  test("es", "ClearSpeak", expr, "seno inverso de x")?;
  let expr = "<math><mi>senh</mi><mo>&#x2061;</mo><mi>x</mi></math>";
  test("es", "ClearSpeak", expr, "seno hiperbólico de x")?;
  return Ok(());
}

// pruebas propias del español: símbolos que cambiaban el significado
#[test]
fn spanish_symbols_p1() -> Result<()> {
  test("es", "SimpleSpeak", "<math><mn>2</mn><mo>·</mo><mn>5</mn></math>", "2 por 5")?;
  test("es", "SimpleSpeak", "<math><mn>2</mn><mo>⋅</mo><mn>5</mn></math>", "2 por 5")?;
  test("es", "ClearSpeak", "<math><mn>2</mn><mo>⋅</mo><mn>5</mn></math>", "2 por 5")?;
  test("es", "SimpleSpeak", "<math><mi>a</mi><mo>∤</mo><mi>b</mi></math>", "a no es divisor de b")?;
  test("es", "SimpleSpeak", "<math><mn>90</mn><mo>°</mo></math>", "90 grados")?;
  test("es", "SimpleSpeak", "<math><mn>5</mn><mo>%</mo></math>", "5 por ciento")?;
  test("es", "SimpleSpeak", "<math><mi>μ</mi><mo>+</mo><mi>ν</mi></math>", "mu más nu")?;
  test("es", "SimpleSpeak", "<math><mfrac><mn>1</mn><mn>2</mn></mfrac></math>", "un medio")?;
  return Ok(());
}

// pruebas propias del español: las primas se leen por el símbolo (sirve en cálculo, geometría y ángulos)
#[test]
fn spanish_primes() -> Result<()> {
  test("es", "ClearSpeak", "<math><msup><mi>f</mi><mo>′</mo></msup></math>", "f prima")?;
  test("es", "ClearSpeak", "<math><msup><mi>f</mi><mo>″</mo></msup></math>", "f segunda")?;
  test("es", "ClearSpeak", "<math><msup><mi>f</mi><mo>‴</mo></msup></math>", "f tercera")?;
  test("es", "ClearSpeak", "<math><msup><mi>f</mi><mo>⁗</mo></msup></math>", "f cuarta")?;
  test("es", "SimpleSpeak", "<math><msup><mi>A</mi><mo>″</mo></msup></math>", "mayúscula a segunda")?;
  return Ok(());
}

#[test]
fn cmu_2024_readings() -> Result<()> {
    // lecturas alineadas con el Código Matemático Unificado para Iberoamérica (2.ª ed., 2024)
    let cases = [
        ("<math><mo>±</mo><mn>2</mn></math>", "más o menos 2"),
        ("<math><mi>a</mi><mo>≠</mo><mi>b</mi></math>", "a es distinto de b"),
        ("<math><mi>a</mi><mo>≤</mo><mi>b</mi></math>", "a es menor o igual que b"),
        ("<math><mi>A</mi><mo>⊂</mo><mi>B</mi></math>", "mayúscula a está incluido en mayúscula b"),
        ("<math><mi>p</mi><mo>⇒</mo><mi>q</mi></math>", "p implica q"),
        ("<math><mi>l</mi><mo>∥</mo><mi>m</mi></math>", "l es paralela a m"),
        ("<math><mi>n</mi><mo>!</mo></math>", "factorial de n"),
        ("<math><mn>1</mn><mfrac><mn>1</mn><mn>2</mn></mfrac></math>", "1 entero y un medio"),
        ("<math><mi>mcd</mi><mo>&#x2061;</mo><mo>(</mo><mn>4</mn><mo>,</mo><mn>6</mn><mo>)</mo></math>",
            "el máximo común divisor de, abrir paréntesis, 4 coma, 6, cerrar paréntesis"),
        ("<math><mi>log</mi><mo>&#x2061;</mo><mi>x</mi></math>", "el logaritmo de x"),
        ("<math><mroot><mi>x</mi><mi>n</mi></mroot></math>", "la raíz enésima de x"),
    ];
    for (expr, speech) in cases {
        test("es", "SimpleSpeak", expr, speech)?;
    }
    return Ok(());
}

#[test]
fn cmu_2024_additions() -> Result<()> {
    // lecturas añadidas desde el Código Matemático Unificado para Iberoamérica (2.ª ed., 2024)
    let cases = [
        ("<math><mo>∃</mo><mo>!</mo><mi>x</mi></math>", "existe un único x"),
        ("<math><mo>⌊</mo><mi>x</mi><mo>⌋</mo></math>", "abrir suelo x cerrar suelo"),
        ("<math><mo>⌈</mo><mi>x</mi><mo>⌉</mo></math>", "abrir techo x cerrar techo"),
        ("<math><mn>3</mn><mi>€</mi></math>", "3 euros"),
        ("<math><mn>1</mn><mi>€</mi></math>", "un euro"),
        ("<math><mi>a</mi><mo>∾</mo><mi>b</mi></math>", "a es semejante a b"),
        ("<math><mi>a</mi><mo>≙</mo><mi>b</mi></math>", "a corresponde a b"),
        ("<math><mi>x</mi><mo>∈</mo><mi>ℍ</mi></math>", "x pertenece a los cuaterniones"),
        ("<math><mi>p</mi><mo>∈</mo><mi>ℙ</mi></math>", "p pertenece a los números primos"),
        ("<math><munder><mi>lím</mi><mrow><mi>x</mi><mo>↓</mo><mn>0</mn></mrow></munder><mi>f</mi><mo>(</mo><mi>x</mi><mo>)</mo></math>", "el límite cuando x tiende decreciendo a 0, de f de x"),
        ("<math><munder><mi>lím</mi><mrow><mi>x</mi><mo>↑</mo><mn>0</mn></mrow></munder><mi>f</mi><mo>(</mo><mi>x</mi><mo>)</mo></math>", "el límite cuando x tiende creciendo a 0, de f de x"),
        ("<math><mfrac><mrow><mi>d</mi><mi>y</mi></mrow><mrow><mi>d</mi><mi>x</mi></mrow></mfrac></math>", "la derivada de y respecto de x"),
        ("<math><mfrac><mrow><msup><mi>d</mi><mn>2</mn></msup><mi>y</mi></mrow><mrow><mi>d</mi><msup><mi>x</mi><mn>2</mn></msup></mrow></mfrac></math>", "la derivada segunda de y respecto de x"),
        ("<math><mfrac><mrow><mo>∂</mo><mi>f</mi></mrow><mrow><mo>∂</mo><mi>x</mi></mrow></mfrac></math>", "la derivada parcial de f respecto de x"),
        ("<math><mover><mrow><mi>A</mi><mi>B</mi></mrow><mo>¯</mo></mover></math>", "el segmento mayúscula a mayúscula b"),
        ("<math><mo>#</mo><mi>A</mi></math>", "el cardinal de mayúscula a"),
        ("<math><mi mathvariant=\"normal\">#</mi><mi>A</mi></math>", "el cardinal de mayúscula a"),
        ("<math><mi mathvariant=\"normal\">#</mi><mn>3</mn></math>", "almohadilla 3"),
        ("<math><mi>sgn</mi><mo>&#x2061;</mo><mi>x</mi></math>", "el signo de x"),
        ("<math><mi>Dom</mi><mo>&#x2061;</mo><mi>f</mi></math>", "el dominio de f"),
        ("<math><mi>Rec</mi><mo>&#x2061;</mo><mi>f</mi></math>", "el recorrido de f"),
        ("<math><mi>rg</mi><mo>&#x2061;</mo><mi>A</mi></math>", "el rango de mayúscula a"),
        ("<math><mi>Adj</mi><mo>&#x2061;</mo><mi>A</mi></math>", "la adjunta de mayúscula a"),
        ("<math><mi>antilog</mi><mo>&#x2061;</mo><mi>x</mi></math>", "el antilogaritmo de x"),
        ("<math><mi>colog</mi><mo>&#x2061;</mo><mi>x</mi></math>", "el cologaritmo de x"),
        ("<math><mi mathvariant=\"normal\">Var</mi><mo>(</mo><mi>X</mi><mo>)</mo></math>", "la varianza de mayúscula x"),
        ("<math><mi>A</mi><mo>∆</mo><mi>B</mi></math>", "mayúscula a diferencia simétrica, mayúscula b"),
        ("<math><mo>∆</mo><mi>x</mi></math>", "el incremento x"),
        ("<math><mi>a</mi><mo>≟</mo><mi>b</mi></math>", "a cuestionado que sea igual a b"),
        ("<math><mi mathvariant=\"normal\">Cov</mi><mo>&#x2061;</mo><mi>X</mi></math>", "la covarianza de mayúscula x"),
        // como llegan desde un PDF etiquetado (luamml): letras itálicas, nombres sin U+2061
        ("<math><mroot><mi>𝑥</mi><mi>𝑛</mi></mroot></math>", "la raíz enésima de x"),
        ("<math><mfrac><mrow><msup><mi>𝑑</mi><mi>𝑛</mi></msup><mi>𝑦</mi></mrow><mrow><mi>𝑑</mi><msup><mi>𝑥</mi><mi>𝑛</mi></msup></mrow></mfrac></math>", "la derivada enésima de y respecto de x"),
        ("<math><mfrac><mrow><mi>𝑑</mi><mi>𝑦</mi></mrow><mrow><mi>𝑑</mi><mi>𝑥</mi></mrow></mfrac></math>", "la derivada de y respecto de x"),
        ("<math><mfrac><mrow><msup><mi>𝑑</mi><mn>2</mn></msup><mi>𝑦</mi></mrow><mrow><mi>𝑑</mi><msup><mi>𝑥</mi><mn>2</mn></msup></mrow></mfrac></math>", "la derivada segunda de y respecto de x"),
        ("<math><mfrac><mrow><mi>𝜕</mi><mi>𝑓</mi></mrow><mrow><mi>𝜕</mi><mi>𝑥</mi></mrow></mfrac></math>", "la derivada parcial de f respecto de x"),
        ("<math><mfrac><mrow><msup><mi>𝜕</mi><mn>2</mn></msup><mi>𝑓</mi></mrow><mrow><mi>𝜕</mi><msup><mi>𝑥</mi><mn>2</mn></msup></mrow></mfrac></math>", "la derivada parcial segunda de f respecto de x"),
        ("<math><mi mathvariant=\"normal\">Dom</mi><mspace width=\"0.167em\"></mspace><mi>𝑓</mi></math>", "el dominio de f"),
        ("<math><mi mathvariant=\"normal\">Rec</mi><mspace width=\"0.167em\"></mspace><mi>𝑓</mi></math>", "el recorrido de f"),
        ("<math><mi mathvariant=\"normal\">Adj</mi><mspace width=\"0.167em\"></mspace><mi>𝐴</mi></math>", "la adjunta de mayúscula a"),
    ];
    for (expr, speech) in cases {
        test("es", "SimpleSpeak", expr, speech)?;
    }
    return Ok(());
}
