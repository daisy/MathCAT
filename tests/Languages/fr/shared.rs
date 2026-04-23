/// Tests for rules shared between various speech styles:
/// *  modified var
use crate::common::*;
use anyhow::Result;

// AI generated
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
    test("fr", "SimpleSpeak", expr, "a accent grave, b tilde, c diacritique brève; b diacritique caron; c accent grave; plus r caron plus; x point, y point en chef; z tréma; u V diacritique trois points en chef; v W diacritique quatre points en chef; plus x chapeau, plus vecteur t")?;
            Ok(())
}

// AI generated
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
    test("fr", "SimpleSpeak", expr, "la limite quand x approche 0, de, fraction, sinus de x, sur x, fin de fraction")?;
    test_prefs("fr", "SimpleSpeak", vec![("Impairment", "LearningDisability")], expr, "la limite quand x approche 0, de; sinus de x, sur x")?;
            Ok(())
}

// AI generated
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
    test("fr", "SimpleSpeak", expr, "la limite quand x approche par le bas 0, de sinus de x")?;
    Ok(())
}


// AI generated
#[test]
fn binomial_mmultiscripts() -> Result<()> {
    let expr = "<math><mmultiscripts><mi>C</mi><mi>m</mi><none/><mprescripts/><mi>n</mi><none/></mmultiscripts></math>";
    test("fr", "SimpleSpeak", expr, "n parmi m")?;
    Ok(())
}

// AI generated
#[test]
fn binomial_mmultiscripts_other() -> Result<()> {
    let expr = "<math><mmultiscripts><mi>C</mi><mi>m</mi><none/><mprescripts/><none/><mi>n</mi></mmultiscripts></math>";
    test("fr", "SimpleSpeak", expr, "n parmi m")?;
    Ok(())
}

// AI generated
#[test]
fn binomial_subscript() -> Result<()> {  // C_{n,k}
    let expr = "<math><msub><mi>C</mi><mrow><mi>n</mi><mo>,</mo><mi>m</mi></mrow></msub></math>";
    test("fr", "SimpleSpeak", expr, "n parmi m")?;
    Ok(())
}

// AI generated
#[test]
fn permutation_mmultiscripts() -> Result<()> {
    let expr = "<math><mmultiscripts><mi>P</mi><mi>k</mi><none/><mprescripts/><mi>n</mi><none/></mmultiscripts></math>";
    test("fr", "SimpleSpeak", expr, "n pochhammer k")?;
    Ok(())
}

// AI generated
#[test]
fn permutation_mmultiscripts_sup() -> Result<()> {
    let expr = "<math><mmultiscripts><mi>P</mi><mi>k</mi><none/><mprescripts/><none/><mi>n</mi></mmultiscripts></math>";
    test("fr", "SimpleSpeak", expr, "n pochhammer k")?;
    Ok(())
}

// AI generated
#[test]
fn permutation_msubsup() -> Result<()> {
    let expr = "<math><msubsup><mi>P</mi><mi>k</mi><mi>n</mi></msubsup></math>";
    test("fr", "SimpleSpeak", expr, "n pochhammer k")?;
    Ok(())
}

// AI generated
#[test]
fn tensor_mmultiscripts() -> Result<()> {
    let expr = "<math><mmultiscripts>
            <mi>R</mi> <mi>i</mi><none/> <none/><mi>j</mi> <mi>k</mi><none/> <mi>l</mi><none/> 
        </mmultiscripts></math>";
    test_prefs("fr", "SimpleSpeak", vec![("Verbosity", "Verbose")], expr, "majuscule r avec 4 postscripts, indice i")?;
    test_prefs("fr", "SimpleSpeak", vec![("Verbosity", "Medium")], expr, "majuscule r avec 4 postscripts, indice i")?;
            Ok(())
}

// AI generated
#[test]
fn huge_num_mmultiscripts() -> Result<()> {
    let expr = "<math><mmultiscripts>
            <mi>R</mi> <mi>i</mi><none/> <none/><mi>j</mi> <mi>k</mi><none/> <mi>l</mi><none/> <mi>m</mi><none/>
            <mprescripts/> <mi>I</mi><none/> <none/><mi>J</mi> <mi>K</mi><none/> <mi>L</mi><none/>
        </mmultiscripts></math>";
    test_prefs("fr", "SimpleSpeak", vec![("Verbosity", "Verbose")], expr, "majuscule r avec 4 préscripts, pré-indice majuscule i et avec 5 postscripts, indice i")?;
            Ok(())
}

// AI generated
#[test]
fn prime() -> Result<()> {
    let expr = "<math> <msup><mi>x</mi><mo >&#x2032;</mo></msup> </math>";
    test("fr", "SimpleSpeak", expr, "x prime")?;
    Ok(())
}

// AI generated
#[test]
fn given() -> Result<()> {
    let expr = "<math><mi>P</mi><mo>(</mo><mi>A</mi><mo>|</mo><mi>B</mi><mo>)</mo></math>";
    test("fr", "SimpleSpeak", expr, "majuscule p; parenthèse gauche, majuscule a donné majuscule b; parenthèse droite")?;
    test("fr", "ClearSpeak", expr, "majuscule p; parenthèse gauche, majuscule a donné majuscule b; parenthèse droite")?; // not good, but follows the spec
    Ok(())
}

// AI generated
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
    test("fr", "ClearSpeak", expr, "x indice k, à la i-ième")?;
    Ok(())
}

// AI generated
#[test]
fn non_simple_msubsup() -> Result<()> {
  let expr = "<math><msubsup><mi>i</mi><mrow><mi>j</mi><mo>&#x2212;</mo><mn>2</mn></mrow><mi>k</mi></msubsup></math>";
  test("fr", "SimpleSpeak", expr, "i indice j moins 2 fin d'indice, à la k-ième")?;
  test("fr", "ClearSpeak", expr, "i indice j moins 2 fin d'indice, à la k-ième")?;
  test_prefs("fr", "SimpleSpeak", vec![("Impairment", "LearningDisability")], expr, "i indice j moins 2, à la k-ième")?;
          Ok(())
}

// AI generated
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
    test("fr", "ClearSpeak", expr, "x indice k, à la i-ième")?;
    Ok(())
}

// AI generated
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
    test("fr", "SimpleSpeak", expr, "majuscule p; parenthèse gauche, majuscule a and majuscule b; parenthèse droite; est égal à; majuscule p; parenthèse gauche, majuscule a intersection majuscule b; parenthèse droite; est égal à; majuscule p de majuscule a; majuscule p de majuscule b")?;
    Ok(())
}

// AI generated
#[test]
fn ignore_mtext_period() -> Result<()> {
    let expr = "<math><mrow><mrow><mo>{</mo><mn>2</mn><mo>}</mo></mrow><mtext>.</mtext></mrow></math>";
    test("fr", "SimpleSpeak", expr, "ensemble 2")?;
    Ok(())
}

// AI generated
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
    test("fr", "SimpleSpeak", expr, "phi droit de x, est égal à; c fois, e élevé à la négatif h au carré, x au carré puissance")?;
    Ok(())
}

// AI generated
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
    test("fr", "ClearSpeak", expr, "majuscule p; parenthèse gauche, majuscule a est un diviseur de, majuscule b; parenthèse droite; est égal à; fraction, majuscule p; parenthèse gauche, majuscule a intersection majuscule b; parenthèse droite, sur, majuscule p de majuscule b, fin de fraction")?;
    Ok(())
}


// AI generated
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
  test("fr", "SimpleSpeak", expr, "closed open interval de 0 comma, 2 pi")?;
  Ok(())
}

// AI generated
#[test]
fn caret_and_hat() -> Result<()> {
  let expr = "<math><mi>x</mi><mo>^</mo><mn>2</mn><mo>+</mo><mover><mi>y</mi><mo>^</mo></mover></math>";
  test("fr", "SimpleSpeak", expr, "x circonflexe 2, plus y chapeau")?;
  Ok(())
}

// AI generated
#[test]
fn mn_with_space() -> Result<()> {
  let expr = "<math><mn>1 234 567</mn></math>";
  test_prefs("fr", "SimpleSpeak", vec![("DecimalSeparators", "."), ("BlockSeparators", " ,")], expr, "1234567")?;
  Ok(())
}

// AI generated
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
  test_prefs("fr", "SimpleSpeak", vec![("IgnoreBold", "false")], expr, "gras x est égal à, 2 sinus de gras t, moins 1")?;
  test_prefs("fr", "SimpleSpeak", vec![("IgnoreBold", "true")], expr, "x est égal à, 2 sinus de t, moins 1")?;
             Ok(())
}

// AI generated
#[test]
fn mn_with_block_and_decimal_separators() -> Result<()> {
  let expr = "<math><mn>1,234.56</mn></math>";                                       // may want to change this for another language
  test_prefs("fr", "SimpleSpeak", vec![("DecimalSeparators", "."), ("BlockSeparators", " ,")], expr, "1234.56")?;
  Ok(())
}

// AI generated
#[test]
fn divergence() -> Result<()> {
  let expr = "<math><mo>&#x2207;</mo><mo>&#xB7;</mo><mi mathvariant='normal'>F</mi></math>";                                       // may want to change this for another language
  test_prefs("fr", "SimpleSpeak", vec![("Verbosity", "Terse")], expr, "div majuscule f")?;
  test_prefs("fr", "SimpleSpeak", vec![("Verbosity", "Verbose")], expr, "divergence de majuscule f")?;
  Ok(())
}

// AI generated
#[test]
fn curl() -> Result<()> {
  let expr = "<math><mo>&#x2207;</mo><mo>&#xD7;</mo><mi mathvariant='normal'>F</mi></math>";          
  // may want to change this for another language
  test_prefs("fr", "SimpleSpeak", vec![("Verbosity", "Terse")], expr, "rotationnel majuscule f")?;
  test_prefs("fr", "SimpleSpeak", vec![("Verbosity", "Verbose")], expr, "rotationnel de majuscule f")?;
  Ok(())
}

// AI generated
#[test]
fn gradient() -> Result<()> {
  let expr = "<math><mo>&#x2207;</mo><mi mathvariant='normal'>F</mi></math>";          
  // may want to change this for another language
  test_prefs("fr", "SimpleSpeak", vec![("Verbosity", "Terse")], expr, "del majuscule f")?;
  test_prefs("fr", "SimpleSpeak", vec![("Verbosity", "Verbose")], expr, "gradient de majuscule f")?;
  Ok(())
}

// AI generated
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
  test("fr", "LiteralSpeak", expr, "majuscule a flèche droite, perpendicular to, majuscule b flèche droite")?;
  Ok(())
}

// AI generated
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
  test("fr", "LiteralSpeak", expr, "ligne verticale; x croix, y point z barre oblique 2; plus a; double ligne verticale, b plus x factorielle; ligne verticale")?;
  Ok(())
}

// AI generated
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
  test("fr", "LiteralSpeak", expr, "forced f, parenthèse gauche, x factorielle, parenthèse droite")?;
  Ok(())
}

// AI generated
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
  test("fr", "LiteralSpeak", expr, "f, parenthèse gauche, x factorielle, parenthèse droite")?;
  Ok(())
}

// AI generated
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
  test("fr", "SimpleSpeak", expr, "majuscule a flèche droite, perpendicular to, majuscule b flèche droite")?;
  Ok(())
}

// AI generated
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
  test("fr", "SimpleSpeak", expr, "forced f, parenthèse gauche, x factorielle, parenthèse droite")?;
  Ok(())
}
