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
    test("el", "SimpleSpeak", expr, 
        "a βαρεία; b κυματοειδής γραμμή; c βραχεία; b καλλωσπιστική ανεστραμμένη γαλλική περισπωμένη; c βαρεία; συν, \
            r ανεστραμμένη γαλλική περισπωμένη; συν; x τελεία, y τελεία, z διπλή κουκκίδα; u καλλωπιστική τριπλή τελεία; v καλλωπιστική τετραπλή τελεία; συν x καπέλο, συν διάνυσμα t")?;
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
    test("el", "SimpleSpeak", expr, "το όριο όταν x προσεγγίζει 0; του; κλάσμα, ημίτονο του x, προς x, τέλος κλάσματος")?;
    test_prefs("el", "SimpleSpeak", vec![("Impairment", "LearningDisability")], expr,
            "το όριο όταν x προσεγγίζει 0; του; ημίτονο του x, προς x")?;
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
    test("el", "SimpleSpeak", expr, "το όριο όταν x προσεγγίζει από κάτω 0; του ημίτονο του x")?;
    return Ok(());

}


#[test]
fn binomial_mmultiscripts() -> Result<()> {
    let expr = "<math><mmultiscripts><mi>C</mi><mi>m</mi><none/><mprescripts/><mi>n</mi><none/></mmultiscripts></math>";
    test("el", "SimpleSpeak", expr, "n ανά m")?;
    return Ok(());

}

#[test]
fn binomial_mmultiscripts_other() -> Result<()> {
    let expr = "<math><mmultiscripts><mi>C</mi><mi>m</mi><none/><mprescripts/><none/><mi>n</mi></mmultiscripts></math>";
    test("el", "SimpleSpeak", expr, "n ανά m")?;
    return Ok(());

}

#[test]
fn binomial_subscript() -> Result<()> {  // C_{n,k}
    let expr = "<math><msub><mi>C</mi><mrow><mi>n</mi><mo>,</mo><mi>m</mi></mrow></msub></math>";
    test("el", "SimpleSpeak", expr, "n ανά m")?;
    return Ok(());

}

#[test]
fn permutation_mmultiscripts() -> Result<()> {
    let expr = "<math><mmultiscripts><mi>P</mi><mi>k</mi><none/><mprescripts/><mi>n</mi><none/></mmultiscripts></math>";
    test("el", "SimpleSpeak", expr, "k μεταθέσεις των n")?;
    return Ok(());

}

#[test]
fn permutation_mmultiscripts_sup() -> Result<()> {
    let expr = "<math><mmultiscripts><mi>P</mi><mi>k</mi><none/><mprescripts/><none/><mi>n</mi></mmultiscripts></math>";
    test("el", "SimpleSpeak", expr, "k μεταθέσεις των n")?;
    return Ok(());
}

#[test]
fn permutation_msubsup() -> Result<()> {
    let expr = "<math><msubsup><mi>P</mi><mi>k</mi><mi>n</mi></msubsup></math>";
    test("el", "SimpleSpeak", expr, "k μεταθέσεις των n")?;
    return Ok(());

}

#[test]
fn tensor_mmultiscripts() -> Result<()> {
    let expr = "<math><mmultiscripts>
            <mi>R</mi> <mi>i</mi><none/> <none/><mi>j</mi> <mi>k</mi><none/> <mi>l</mi><none/> 
        </mmultiscripts></math>";
    test_prefs("el", "SimpleSpeak", vec![("Verbosity", "Verbose")], expr,
            "κεφαλαίο r με 4 μετατεταγμένα μέρη, δείκτης i εκθέτης j δείκτης k δείκτης l")?;
    test_prefs("el", "SimpleSpeak", vec![("Verbosity", "Medium")], expr,
            "κεφαλαίο r με 4 μετατεταγμένα μέρη, δείκτης i εκθέτης j δείκτης k δείκτης l")?;
            return Ok(());

}

#[test]
fn huge_num_mmultiscripts() -> Result<()> {
    let expr = "<math><mmultiscripts>
            <mi>R</mi> <mi>i</mi><none/> <none/><mi>j</mi> <mi>k</mi><none/> <mi>l</mi><none/> <mi>m</mi><none/>
            <mprescripts/> <mi>I</mi><none/> <none/><mi>J</mi> <mi>K</mi><none/> <mi>L</mi><none/>
        </mmultiscripts></math>";
    test_prefs("el", "SimpleSpeak", vec![("Verbosity", "Verbose")], expr,
            "κεφαλαίο r με 4 προτεταγμένα μέρη, προτεταγμένος δείκτης κεφαλαίο i, προτεταγμένος εκθέτης κεφαλαίο j και εναλλάξ προτεταγμένα μέρη κεφαλαίο k none κεφαλαίο l none τέλος προτεταγμένων μερών και με 5 μετατεταγμένα μέρη, δείκτης i εκθέτης j δείκτης k δείκτης l και εναλλάξ τεταγμένα μέρη m none τέλος τεταγμένων μερών")?;
            return Ok(());
//theodora. Το none στις άλλες γλώσσες το έχουν αφήσει έτσι, οπότε το άφησα κι εγώ έτσι.
}

#[test]
fn prime() -> Result<()> {
    let expr = "<math> <msup><mi>x</mi><mo >&#x2032;</mo></msup> </math>";
    test("el", "SimpleSpeak", expr, "x τόνος")?;
    return Ok(());

}

#[test]
fn given() -> Result<()> {
    let expr = "<math><mi>P</mi><mo>(</mo><mi>A</mi><mo>|</mo><mi>B</mi><mo>)</mo></math>";
    test("el", "SimpleSpeak", expr, "κεφαλαίο p; ανοίγει παρένθεση; κεφαλαίο A, δεδομένου, κεφαλαίο b; κλείνει παρένθεση")?;
    test("el", "ClearSpeak", expr,  "κεφαλαίο p; ανοίγει παρένθεση; κεφαλαίο A, δεδομένου, κεφαλαίο b; κλείνει παρένθεση")?; // not good, but follows the spec
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
    test("el", "ClearSpeak", expr, "x δείκτης k, στην i οστή δύναμη")?;
    return Ok(());

}

#[test]
fn non_simple_msubsup() -> Result<()> {
  let expr = "<math><msubsup><mi>i</mi><mrow><mi>j</mi><mo>&#x2212;</mo><mn>2</mn></mrow><mi>k</mi></msubsup></math>";
  test("el", "SimpleSpeak", expr, "i δείκτης, j μείον 2 τέλος δείκτη, στην k οστή")?;
  test("el", "ClearSpeak", expr, "i δείκτης, j μείον 2 τέλος δείκτη, στην k οστή δύναμη")?;
  test_prefs("el", "SimpleSpeak", vec![("Impairment", "LearningDisability")], expr,
          "i δείκτης, j μείον 2, στην k οστή")?;
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
    test("el", "ClearSpeak", expr, "x δείκτης k, στην i οστή δύναμη")?;
    return Ok(());

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
    test("el", "SimpleSpeak", expr, "κεφαλαίο p; ανοίγει παρένθεση; κεφαλαίο A and κεφαλαίο b; κλείνει παρένθεση; ισούται με; κεφαλαίο p; ανοίγει παρένθεση; κεφαλαίο A τομή κεφαλαίο b; κλείνει παρένθεση; ισούται με; κεφαλαίο p του κεφαλαίο A; κεφαλαίο p του κεφαλαίο b")?;
    return Ok(());

}

#[test]
fn ignore_mtext_period() -> Result<()> {
    let expr = "<math><mrow><mrow><mo>{</mo><mn>2</mn><mo>}</mo></mrow><mtext>.</mtext></mrow></math>";
    test("el", "SimpleSpeak", expr, "το σύνολο 2")?;
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
    test("el", "SimpleSpeak", expr, "παραλλαγή του φι, του x; ισούται με; c; e που υψώνεται στη μείον h στο τετράγωνο, x στο τετράγωνο τέλος δύναμης")?;
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
    test("el", "ClearSpeak", expr, "cap p, open paren, cap eigh divides cap b, close paren; is equal to; the fraction with numerator; cap p, open paren, cap eigh intersection cap b; close paren; and denominator cap p of cap b")?;
    return Ok(()); 
// theodora. if not ignored, the correct greek translation: 
// κεφαλαίο p; ανοίγει παρένθεση; κεφαλαίο A, δεδομένου κεφαλαίο b; κλείνει παρένθεση; ισούται με; το κλάσμα με αριθμητή; κεφαλαίο p; ανοίγει παρένθεση; κεφαλαίο A τομή κεφαλαίο b; κλείνει παρένθεση; και παρονομαστή κεφαλαίο p του κεφαλαίο b
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
  test("el", "SimpleSpeak",expr, "το κλειστό ανοιχτό διάστημα από 0 ως 2 π")?;
  return Ok(());

}

#[test]
fn caret_and_hat() -> Result<()> {
  let expr = "<math><mi>x</mi><mo>^</mo><mn>2</mn><mo>+</mo><mover><mi>y</mi><mo>^</mo></mover></math>";
  test("el", "SimpleSpeak",expr, "x καπέλο 2, συν y καπέλο")?;
  return Ok(());

}

#[test]
fn mn_with_space() -> Result<()> {
  let expr = "<math><mn>1 234 567</mn></math>";
  test_prefs("el", "SimpleSpeak", vec![("DecimalSeparators", "."), ("BlockSeparators", " ,")], expr, "1234567")?;
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
  test_prefs("el", "SimpleSpeak", vec![("IgnoreBold", "false")],
             expr, "έντονο x ισούται με; 2, ημίτονο του έντονο t; μείον 1")?;
  test_prefs("el", "SimpleSpeak", vec![("IgnoreBold", "true")],
             expr, "έντονο x ισούται με; 2, ημίτονο του έντονο t; μείον 1")?;
             return Ok(());

}

#[test]
fn mn_with_block_and_decimal_separators() -> Result<()> {
  let expr = "<math><mn>1,234.56</mn></math>";                                       // may want to change this for another language
  test_prefs("el", "SimpleSpeak", vec![("DecimalSeparators", "."), ("BlockSeparators", " ,")], expr, "1234.56")?;
  return Ok(());

}

#[test]
fn divergence() -> Result<()> {
  let expr = "<math><mo>&#x2207;</mo><mo>&#xB7;</mo><mi mathvariant='normal'>F</mi></math>";                                       // may want to change this for another language
  test_prefs("el", "SimpleSpeak", vec![("Verbosity", "Terse")], expr, "απόκλιση κεφαλαίο f")?;
  test_prefs("el", "SimpleSpeak", vec![("Verbosity", "Verbose")], expr, "απόκλιση του κεφαλαίο f")?;
  return Ok(());

}

#[test]
fn curl() -> Result<()> {
  let expr = "<math><mo>&#x2207;</mo><mo>&#xD7;</mo><mi mathvariant='normal'>F</mi></math>";          
  // may want to change this for another language
  test_prefs("el", "SimpleSpeak", vec![("Verbosity", "Terse")], expr, "στροβιλισμός κεφαλαίο f")?;
  test_prefs("el", "SimpleSpeak", vec![("Verbosity", "Verbose")], expr, "στροβιλισμός του κεφαλαίο f")?;
  return Ok(());

}

#[test]
fn gradient() -> Result<()> {
  let expr = "<math><mo>&#x2207;</mo><mi mathvariant='normal'>F</mi></math>";          
  // may want to change this for another language
  test_prefs("el", "SimpleSpeak", vec![("Verbosity", "Terse")], expr, "ανάδελτα κεφαλαίο f")?;
  test_prefs("el", "SimpleSpeak", vec![("Verbosity", "Verbose")], expr, "ανάδελτα του κεφαλαίο f")?;
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
  test("el", "LiteralSpeak", expr, "κεφαλαίο A βέλος προς τα δεξιά; perpendicular to, κεφαλαίο b βέλος προς τα δεξιά")?;
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
  test("el", "LiteralSpeak", expr, "κάθετη γραμμη; x σύμβολο πολλαπλασιασμού; y τελεία, z κάθετος 2; συν a; διπλή κάθετη γραμμή; b συν x θαυμαστικό; κάθετη γραμμη")?;
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
  test("el", "LiteralSpeak", expr, "forced f; αριστερή παρένθεση; x θαυμαστικό, δεξιά παρένθεση")?;
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
  test("el", "LiteralSpeak", expr, "f; αριστερή παρένθεση; x θαυμαστικό, δεξιά παρένθεση")?;
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
  test("el", "SimpleSpeak", expr, "κεφαλαίο A βέλος προς τα δεξιά; perpendicular to, κεφαλαίο b βέλος προς τα δεξιά")?;
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
  test("el", "SimpleSpeak", expr, "forced f; ανοίγει παρένθεση, x θαυμαστικό, κλείνει παρένθεση")?;
  return Ok(());
  }
