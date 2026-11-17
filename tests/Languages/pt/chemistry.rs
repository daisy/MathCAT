/// TESTES DO IDIOMA pt — como reconciliar com as regras
/// Tests for rules shared between various speech styles:
/// *  modified var
use crate::common::*;
use anyhow::Result;

#[test]
fn salt() -> Result<()> {
  let expr = "<math><mi>Na</mi><mi>Cl</mi></math>";
  test_prefs("pt", "SimpleSpeak", vec![("Verbosity", "Terse")], expr, "maiúscula n a; maiúscula c l")?;
  return Ok(());

}

#[test]
fn water() -> Result<()> {
  let expr = "<math><msub><mi>H</mi><mn>2</mn></msub><mi>O</mi></math>";
  // ClearSpeak reativado: Rules/Languages/pt/ClearSpeak_Rules.yaml traduzido.
  // Pausas (vírgula/ponto-e-vírgula) podem precisar de reconciliação via cargo test.
  test_prefs("pt", "ClearSpeak", vec![("Verbosity", "Terse")], expr, "maiúscula h, 2, maiúscula o")?;
  // ClearSpeak reativado: Rules/Languages/pt/ClearSpeak_Rules.yaml traduzido.
  // Pausas (vírgula/ponto-e-vírgula) podem precisar de reconciliação via cargo test.
  test_prefs("pt", "ClearSpeak", vec![("Verbosity", "Medium")], expr, "maiúscula h, sub 2, maiúscula o")?;
  // ClearSpeak reativado: Rules/Languages/pt/ClearSpeak_Rules.yaml traduzido.
  // Pausas (vírgula/ponto-e-vírgula) podem precisar de reconciliação via cargo test.
  test_prefs("pt", "ClearSpeak", vec![("Verbosity", "Verbose")], expr, "maiúscula h, subscrito 2, maiúscula o")?;
  return Ok(());

}

#[test]
fn carbon() -> Result<()> {
  let expr = "<math><mi>C</mi></math>";     // not enough to trigger recognition
  test_prefs("pt", "SimpleSpeak", vec![("Verbosity", "Terse")], expr, "maiúscula c")?;
  return Ok(());

}

#[test]
fn sulfate() -> Result<()> {
  let expr = "<math><mrow><msup>
          <mrow><mo>[</mo><mi>S</mi><msub><mi>O</mi><mn>4</mn></msub><mo>]</mo></mrow>
          <mrow><mn>2</mn><mo>&#x2212;</mo></mrow>
      </msup></mrow></math>";
  // ClearSpeak reativado: Rules/Languages/pt/ClearSpeak_Rules.yaml traduzido.
  // Pausas (vírgula/ponto-e-vírgula) podem precisar de reconciliação via cargo test.
  test_prefs("pt", "ClearSpeak", vec![("Verbosity", "Medium")], expr, "abre colchetes, maiúscula s, maiúscula o, sub 4; fecha colchetes super 2 menos")?;
  return Ok(());

}

#[test]
fn aluminum_sulfate() -> Result<()> {
  let expr = "<math><mrow><msub><mi>Al</mi><mn>2</mn></msub>
          <msub><mrow><mo>(</mo><mi>S</mi><msub><mi>O</mi><mn>4</mn></msub><mo>)</mo></mrow><mn>3</mn></msub></mrow></math>";
  // ClearSpeak reativado: Rules/Languages/pt/ClearSpeak_Rules.yaml traduzido.
  // Pausas (vírgula/ponto-e-vírgula) podem precisar de reconciliação via cargo test.
  test_prefs("pt", "ClearSpeak", vec![("Verbosity", "Terse")], expr, "maiúscula a l, 2; abre parênteses, maiúscula s, maiúscula o, 4; fecha parênteses 3")?;
  // ClearSpeak reativado: Rules/Languages/pt/ClearSpeak_Rules.yaml traduzido.
  // Pausas (vírgula/ponto-e-vírgula) podem precisar de reconciliação via cargo test.
  test_prefs("pt", "ClearSpeak", vec![("Verbosity", "Medium")], expr, "maiúscula a l, sub 2; abre parênteses, maiúscula s, maiúscula o, sub 4; fecha parênteses sub 3")?;
  // ClearSpeak reativado: Rules/Languages/pt/ClearSpeak_Rules.yaml traduzido.
  // Pausas (vírgula/ponto-e-vírgula) podem precisar de reconciliação via cargo test.
  test_prefs("pt", "ClearSpeak", vec![("Verbosity", "Verbose")], expr, "maiúscula a l, subscrito 2; abre parênteses, maiúscula s, maiúscula o, subscrito 4; fecha parênteses subscrito 3")?;
  return Ok(());

}

#[test]
fn ethanol_bonds() -> Result<()> {
  let expr = "<math>
          <mrow>
              <mi>C</mi>
              <msub>  <mi>H</mi> <mn>3</mn> </msub>
              <mo>&#x2212;</mo>
              <mi>C</mi>
              <msub>  <mi>H</mi> <mn>2</mn> </msub>
              <mo>&#x2212;</mo>
              <mi>O</mi>
              <mi>H</mi>
          </mrow>
      </math>";
  // ClearSpeak reativado: Rules/Languages/pt/ClearSpeak_Rules.yaml traduzido.
  // Pausas (vírgula/ponto-e-vírgula) podem precisar de reconciliação via cargo test.
  test_prefs("pt", "ClearSpeak", vec![("Verbosity", "Terse")], expr, "maiúscula c, maiúscula h, 3, ligação simples, maiúscula c, maiúscula h, 2, ligação simples, maiúscula o, maiúscula h")?;

  return Ok(());

}

#[test]
fn dichlorine_hexoxide() -> Result<()> {
  let expr = "<math><mrow>
      <msup>
        <mrow><mo>[</mo><mi>Cl</mi><msub><mi>O</mi><mn>2</mn></msub><mo>]</mo></mrow>
        <mo>+</mo>
      </msup>
      <msup>
        <mrow><mo>[</mo><mi>Cl</mi><msub><mi>O</mi><mn>4</mn></msub><mo>]</mo></mrow>
        <mo>-</mo>
      </msup>
    </mrow></math>";
  test_prefs("pt", "SimpleSpeak", vec![("Verbosity", "Terse")], 
    expr, "abre colchetes, maiúscula c l; maiúscula o, 2; fecha colchetes mais; \
                          abre colchetes, maiúscula c l; maiúscula o, 4; fecha colchetes menos")?;
  test_prefs("pt", "SimpleSpeak", vec![("Verbosity", "Medium")], 
    expr, "abre colchetes, maiúscula c l; maiúscula o, sub 2; fecha colchetes super mais; \
                          abre colchetes, maiúscula c l; maiúscula o, sub 4; fecha colchetes super menos")?;
  test_prefs("pt", "SimpleSpeak", vec![("Verbosity", "Verbose")], 
    expr, "abre colchetes, maiúscula c l; maiúscula o, subscrito 2; fecha colchetes sobrescrito mais; \
                          abre colchetes, maiúscula c l; maiúscula o, subscrito 4; fecha colchetes sobrescrito menos")?;
                          return Ok(());

}


#[test]
fn ethylene_with_bond() -> Result<()> {
  let expr = "<math><mrow>
          <msub><mi>H</mi><mn>2</mn></msub><mi>C</mi>
          <mo>=</mo>
          <mi>C</mi><msub><mi>H</mi><mn>2</mn></msub>
      </mrow></math>";
  test_prefs("pt", "SimpleSpeak", vec![("Verbosity", "Terse")], expr, "maiúscula h, 2, maiúscula c; ligação dupla, maiúscula c, maiúscula h, 2")?;
  return Ok(());

}

#[test]
fn ferric_chloride_aq() -> Result<()> {
  let expr = "<math><mrow>
        <mi>Fe</mi>
        <msub><mi>Cl</mi><mn>3</mn></msub>
        <mrow><mo>(</mo><mrow><mi>aq</mi></mrow><mo>)</mo></mrow>
    </mrow></math>";
  test_prefs("pt", "SimpleSpeak", vec![("Verbosity", "Terse")], expr, "maiúscula f e; maiúscula c l, 3, aquoso")?;
  return Ok(());

  }

#[test]
fn ethylene_with_colon_bond() -> Result<()> {
  let expr = "<math><mrow>
          <msub><mi>H</mi><mn>2</mn></msub><mi>C</mi>
          <mo>::</mo>
          <mi>C</mi><msub><mi>H</mi><mn>2</mn></msub>
      </mrow></math>";
  test_prefs("pt", "SimpleSpeak", vec![("Verbosity", "Terse")], expr, "maiúscula h, 2, maiúscula c; ligação dupla, maiúscula c, maiúscula h, 2")?;
  return Ok(());

}

#[test]
fn beta_decay() -> Result<()> {
  let expr = "<math>
      <mmultiscripts>
        <mtext>C</mtext>
        <mprescripts />
        <mn>6</mn>
        <mn>14</mn>
      </mmultiscripts>
      <mo>&#x2192;</mo>
      <mmultiscripts>
        <mtext>N</mtext>
        <mprescripts />
        <mn>7</mn>
        <mn>14</mn>
      </mmultiscripts>
      <mo>+</mo>
      <mmultiscripts>
        <mtext>e</mtext>
        <mprescripts />
        <mrow>
          <mo>&#x2212;</mo>
          <mn>1</mn>
        </mrow>
        <mn>0</mn>
      </mmultiscripts>
    </math>";
    // ClearSpeak reativado: Rules/Languages/pt/ClearSpeak_Rules.yaml traduzido.
    // Pausas (vírgula/ponto-e-vírgula) podem precisar de reconciliação via cargo test.
    test_prefs("pt", "ClearSpeak", vec![("Verbosity", "Terse")], expr,
      "14, 6, maiúscula c; forma, 14, 7, maiúscula n; mais 0, menos 1, e")?;
    // ClearSpeak reativado: Rules/Languages/pt/ClearSpeak_Rules.yaml traduzido.
    // Pausas (vírgula/ponto-e-vírgula) podem precisar de reconciliação via cargo test.
    test_prefs("pt", "ClearSpeak", vec![("Verbosity", "Medium")], expr,
      "super 14, sub 6, maiúscula c; reage formando; super 14, sub 7, maiúscula n; mais super 0, sub menos 1, e")?;
    // ClearSpeak reativado: Rules/Languages/pt/ClearSpeak_Rules.yaml traduzido.
    // Pausas (vírgula/ponto-e-vírgula) podem precisar de reconciliação via cargo test.
    test_prefs("pt", "ClearSpeak", vec![("Verbosity", "Verbose")], expr,
      "sobrescrito 14, subscrito 6, maiúscula c; reage formando; sobrescrito 14, subscrito 7, maiúscula n; mais, sobrescrito 0, subscrito menos 1, e")?;
      return Ok(());

}

#[test]
fn mhchem_beta_decay() -> Result<()> {
  let expr = "<math>
      <mrow>
        <msubsup>
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
              <mpadded hat='0' depth='0'>
                <mphantom>
                  <mn>6</mn>
                </mphantom>
              </mpadded>
            </mrow>
          </mrow>
          <mrow>
            <mrow>
              <mpadded hat='0' depth='0'>
                <mphantom>
                  <mn>14</mn>
                </mphantom>
              </mpadded>
            </mrow>
          </mrow>
        </msubsup>
        <mspace width='-0.083em'></mspace>
        <msubsup>
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
              <mpadded width='0'>
                <mphantom>
                  <mn>2</mn>
                </mphantom>
              </mpadded>
            </mrow>
            <mrow>
              <mpadded width='0' lspace='-1width'>
                <mrow>
                  <mpadded hat='0'>
                    <mn>6</mn>
                  </mpadded>
                </mrow>
              </mpadded>
            </mrow>
          </mrow>
          <mrow>
            <mrow>
              <mpadded hat='0'>
                <mrow>
                  <mpadded width='0'>
                    <mphantom>
                      <mn>2</mn>
                    </mphantom>
                  </mpadded>
                </mrow>
              </mpadded>
            </mrow>
            <mrow>
              <mpadded width='0' lspace='-1width'>
                <mn>14</mn>
              </mpadded>
            </mrow>
          </mrow>
        </msubsup>
        <mrow>
          <mi mathvariant='normal'>C</mi>
        </mrow>
        <mrow></mrow>
        <mrow>
          <mo stretchy='false'>&#x27F6;</mo>
        </mrow>
        <mrow></mrow>
        <msubsup>
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
              <mpadded hat='0' depth='0'>
                <mphantom>
                  <mn>7</mn>
                </mphantom>
              </mpadded>
            </mrow>
          </mrow>
          <mrow>
            <mrow>
              <mpadded hat='0' depth='0'>
                <mphantom>
                  <mn>14</mn>
                </mphantom>
              </mpadded>
            </mrow>
          </mrow>
        </msubsup>
        <mspace width='-0.083em'></mspace>
        <msubsup>
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
              <mpadded width='0'>
                <mphantom>
                  <mn>2</mn>
                </mphantom>
              </mpadded>
            </mrow>
            <mrow>
              <mpadded width='0' lspace='-1width'>
                <mrow>
                  <mpadded hat='0'>
                    <mn>7</mn>
                  </mpadded>
                </mrow>
              </mpadded>
            </mrow>
          </mrow>
          <mrow>
            <mrow>
              <mpadded hat='0'>
                <mrow>
                  <mpadded width='0'>
                    <mphantom>
                      <mn>2</mn>
                    </mphantom>
                  </mpadded>
                </mrow>
              </mpadded>
            </mrow>
            <mrow>
              <mpadded width='0' lspace='-1width'>
                <mn>14</mn>
              </mpadded>
            </mrow>
          </mrow>
        </msubsup>
        <mrow>
          <mi mathvariant='normal'>N</mi>
        </mrow>
        <mrow></mrow>
        <mo>+</mo>
        <mrow></mrow>
        <msubsup>
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
              <mpadded hat='0' depth='0'>
                <mphantom>
                  <mo>&#x2212;</mo>
                  <mn>1</mn>
                </mphantom>
              </mpadded>
            </mrow>
          </mrow>
          <mrow>
            <mrow>
              <mpadded hat='0' depth='0'>
                <mphantom>
                  <mn>0</mn>
                </mphantom>
              </mpadded>
            </mrow>
          </mrow>
        </msubsup>
        <mspace width='-0.083em'></mspace>
        <msubsup>
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
              <mpadded width='0'>
                <mphantom>
                  <mn>2</mn>
                </mphantom>
              </mpadded>
            </mrow>
            <mrow>
              <mpadded width='0' lspace='-1width'>
                <mrow>
                  <mpadded hat='0'>
                    <mo>&#x2212;</mo>
                    <mn>1</mn>
                  </mpadded>
                </mrow>
              </mpadded>
            </mrow>
          </mrow>
          <mrow>
            <mrow>
              <mpadded hat='0'>
                <mrow>
                  <mpadded width='0'>
                    <mphantom>
                      <mn>2</mn>
                    </mphantom>
                  </mpadded>
                </mrow>
              </mpadded>
            </mrow>
            <mrow>
              <mpadded width='0' lspace='-1width'>
                <mn>0</mn>
              </mpadded>
            </mrow>
          </mrow>
        </msubsup>
        <mrow>
          <mi mathvariant='normal'>e</mi>
        </mrow>
      </mrow>
    </math>";
    // ClearSpeak reativado: Rules/Languages/pt/ClearSpeak_Rules.yaml traduzido.
    // Pausas (vírgula/ponto-e-vírgula) podem precisar de reconciliação via cargo test.
    test_prefs("pt", "ClearSpeak", vec![("Verbosity", "Terse")], expr,
      "14, 6, maiúscula c; forma, 14, 7, maiúscula n; mais 0, menos 1, e")?;
    // ClearSpeak reativado: Rules/Languages/pt/ClearSpeak_Rules.yaml traduzido.
    // Pausas (vírgula/ponto-e-vírgula) podem precisar de reconciliação via cargo test.
    test_prefs("pt", "ClearSpeak", vec![("Verbosity", "Medium")], expr,
      "super 14, sub 6, maiúscula c; reage formando; super 14, sub 7, maiúscula n; mais super 0, sub menos 1, e")?;
    // ClearSpeak reativado: Rules/Languages/pt/ClearSpeak_Rules.yaml traduzido.
    // Pausas (vírgula/ponto-e-vírgula) podem precisar de reconciliação via cargo test.
    test_prefs("pt", "ClearSpeak", vec![("Verbosity", "Verbose")], expr,
      "sobrescrito 14, subscrito 6, maiúscula c; reage formando; sobrescrito 14, subscrito 7, maiúscula n; mais, sobrescrito 0, subscrito menos 1, e")?;
      return Ok(());

}

#[test]
fn hcl_na_produz() -> Result<()> {
    let expr = "<math> <mrow>
      <mn>2</mn><mi>H</mi><mi>Cl</mi><mo>+</mo><mn>2</mn><mtext>Na</mtext>
      <mo>&#x2192;</mo>
      <mn>2</mn><mtext>Na</mtext><mi>Cl</mi><mo>+</mo>
      <msub> <mi>H</mi> <mn>2</mn> </msub>
      </mrow>
    </math>";
    test_prefs("pt", "SimpleSpeak", vec![("Verbosity", "Verbose")], expr,
        "2, maiúscula h, maiúscula c l; mais 2 maiúscula n a; reage formando; 2, maiúscula n a; maiúscula c l; mais maiúscula h, subscrito 2")?;
        return Ok(());

}

#[test]
fn mhchem_so4_2mais() -> Result<()> {
  let expr = "<math>
    <mrow>
      <mrow>
        <mi>SO</mi>
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
            <mpadded hat='0'>
              <mn>4</mn>
            </mpadded>
          </mrow>
        </mrow>
      </msub>
      <msup>
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
          <mn>2</mn>
          <mo>+</mo>
        </mrow>
      </msup>
    </mrow>
  </math>";
  test_prefs("pt", "SimpleSpeak", vec![("Verbosity", "Terse")], expr, "maiúscula s; maiúscula o, 4, 2 mais")?;
  test_prefs("pt", "SimpleSpeak", vec![("Verbosity", "Medium")], expr, "maiúscula s; maiúscula o, sub 4, super 2 mais")?;
  test_prefs("pt", "SimpleSpeak", vec![("Verbosity", "Verbose")], expr, "maiúscula s; maiúscula o, subscrito 4, sobrescrito 2 mais")?;
  return Ok(());

}


#[test]
fn mhchem_hcl_aq_etc() -> Result<()> {
  let expr = "<math>
    <mrow>
      <mn>2</mn>
      <mstyle scriptlevel='0'>
        <mspace width='0.167em'></mspace>
      </mstyle>
      <mrow>
        <mi>HCl</mi>
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
        <mi>NaCl</mi>
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
            <mpadded hat='0'>
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
  </math>";
  test_prefs("pt", "SimpleSpeak", vec![("Verbosity", "Terse")],
      expr, "2, maiúscula h, maiúscula c l; aquoso; mais, 2, maiúscula n a; sólido; forma; 2, maiúscula n a; maiúscula c l; aquoso; mais, maiúscula h, 2; gasoso")?;

      return Ok(());

}


#[test]
fn mhchem_barbed_equilibrium() -> Result<()> {
  let expr = "<math>
    <mrow data-mjx-texclass='ORD' data-chem-equation='14'>
      <mrow data-changed='added' data-chem-equation='3'>
        <mmultiscripts data-chem-formula='1'>
          <mi data-mjx-texclass='ORD' mathvariant='normal' data-chem-element='1'>H</mi>
          <mn data-mjx-texclass='ORD'>2</mn>
          <none></none>
        </mmultiscripts>
        <mo data-changed='added' data-function-guess='true'>&#x2063;</mo>
        <mrow data-changed='added' data-chem-equation='1'>
          <mo stretchy='false'>(</mo>
          <mi data-mjx-texclass='ORD' mathvariant='normal'>g</mi>
          <mo stretchy='false'>)</mo>
        </mrow>
      </mrow>
      <mo data-chem-equation-op='1'>+</mo>
      <mrow data-changed='added' data-chem-equation='10'>
        <mrow data-changed='added' data-chem-equation='3'>
          <mmultiscripts data-chem-formula='1'>
            <mi data-mjx-texclass='ORD' mathvariant='normal' data-chem-element='1'>I</mi>
            <mn data-mjx-texclass='ORD'>2</mn>
            <none></none>
          </mmultiscripts>
          <mo data-changed='added' data-function-guess='true'>&#x2063;</mo>
          <mrow data-changed='added' data-chem-equation='1'>
            <mo stretchy='false'>(</mo>
            <mi data-mjx-texclass='ORD' mathvariant='normal'>g</mi>
            <mo stretchy='false'>)</mo>
          </mrow>
        </mrow>
        <mo data-changed='added'>&#x2062;</mo>
        <mover data-mjx-texclass='REL'>
          <mrow data-mjx-texclass='ORD' depth='0' hat='0' data-changed='added'>
            <mo data-mjx-texclass='ORD' stretchy='false'>↽</mo>
            <mo data-mjx-texclass='ORD'>-</mo>
          </mrow>
          <mrow data-mjx-texclass='ORD' displaystyle='false' scriptlevel='0' data-changed='added'>
            <mo data-mjx-texclass='ORD'>-</mo>
            <mo data-mjx-texclass='ORD' stretchy='false'>⇀</mo>
          </mrow>
        </mover>
        <mo data-changed='added'>&#x2062;</mo>
        <mn>2</mn>
        <mo data-changed='added'>&#x2062;</mo>
        <mrow data-changed='added' data-chem-equation='5'>
          <mi mathvariant='normal' data-chem-element='1'>H</mi>
          <mo data-changed='added'>&#x2063;</mo>
          <mi mathvariant='normal' data-chem-element='1'>I</mi>
          <mo data-changed='added' data-function-guess='true'>&#x2063;</mo>
          <mrow data-changed='added' data-chem-equation='1'>
            <mo stretchy='false'>(</mo>
            <mi data-mjx-texclass='ORD' mathvariant='normal'>g</mi>
            <mo stretchy='false'>)</mo>
          </mrow>
        </mrow>
      </mrow>
    </mrow>
  </math>";
  test_prefs("pt", "SimpleSpeak", vec![("Verbosity", "Terse")],
      expr, "maiúscula h, 2; gasoso; mais; maiúscula i, 2; gasoso; está em equilíbrio com, 2, maiúscula h, maiúscula i, gasoso")?;
      return Ok(());

}



#[test]
fn mhchem_roman_in_sobrescrito() -> Result<()> {
      let expr = " <math>
      <mrow>
        <mmultiscripts>
          <mi>Fe</mi>
          <none></none>
          <mi>II</mi>
        </mmultiscripts>
        <mo>&#x2063;</mo>
        <mmultiscripts>
          <mi>Fe</mi>
          <none></none>
          <mi data-number='3'>III</mi>
        </mmultiscripts>
        <mo>&#x2063;</mo>
        <mmultiscripts>
          <mi mathvariant='normal' >O</mi>
          <mn>4</mn>
          <none></none>
        </mmultiscripts>
      </mrow>
    </math>";
  test_prefs("pt", "SimpleSpeak", vec![("Verbosity", "Terse")],
      expr, "maiúscula f e, 2; maiúscula f e, 3; maiúscula o, 4")?;
      return Ok(());

}


