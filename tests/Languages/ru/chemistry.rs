/// Тесты для правил, общих для различных стилей речи:
/// * модифицированная переменная
use crate::common::*;

use anyhow::Result;

#[test]
fn salt() -> Result<()> {
    let expr = "<math><mi>Na</mi><mi>Cl</mi></math>";
    test_prefs("ru", "SimpleSpeak", vec![("Verbosity", "Terse")], expr, "заглавная эн а, заглавная цэ эль")?;
    return Ok(());
}

#[test]
fn water() -> Result<()> {
    let expr = "<math><msub><mi>H</mi><mn>2</mn></msub><mi>O</mi></math>";
    test_prefs("ru", "ClearSpeak", vec![("Verbosity", "Terse")], expr, "заглавная аш, 2 заглавная о")?;
    test_prefs("ru", "ClearSpeak", vec![("Verbosity", "Medium")], expr, "заглавная аш, нижний индекс 2 заглавная о")?;
    test_prefs("ru", "ClearSpeak", vec![("Verbosity", "Verbose")], expr, "заглавная аш, нижний индекс 2, заглавная о")?;
    return Ok(());
}

#[test]
fn carbon() -> Result<()> {
    let expr = "<math><mi>C</mi></math>";
    test_prefs("ru", "SimpleSpeak", vec![("Verbosity", "Terse")], expr, "заглавная цэ")?;
    return Ok(());
}

#[test]
fn sulfate() -> Result<()> {
    let expr = "<math><mrow><msup>
          <mrow><mo>[</mo><mi>S</mi><msub><mi>O</mi><mn>4</mn></msub><mo>]</mo></mrow>
          <mrow><mn>2</mn><mo>&#x2212;</mo></mrow>
      </msup></mrow></math>";
    test_prefs("ru", "ClearSpeak", vec![("Verbosity", "Medium")], expr, "открывающая квадратная скобка, заглавная эс, заглавная о, нижний индекс 4; закрывающая квадратная скобка, верхний индекс 2 минус")?;
    return Ok(());
}

#[test]
fn aluminum_sulfate() -> Result<()> {
    let expr = "<math><mrow><msub><mi>Al</mi><mn>2</mn></msub>
          <msub><mrow><mo>(</mo><mi>S</mi><msub><mi>O</mi><mn>4</mn></msub><mo>)</mo></mrow><mn>3</mn></msub></mrow></math>";
    test_prefs("ru", "ClearSpeak", vec![("Verbosity", "Terse")], expr, "заглавная а эль, 2, открыть заглавная эс, заглавная о, 4, закрыть 3")?;
    test_prefs("ru", "ClearSpeak", vec![("Verbosity", "Medium")], expr, "заглавная а эль, нижний индекс 2; открывающая круглая скобка, заглавная эс, заглавная о, нижний индекс 4; закрывающая круглая скобка, нижний индекс 3")?;
    test_prefs("ru", "ClearSpeak", vec![("Verbosity", "Verbose")], expr, "заглавная а эль, нижний индекс 2; открывающая круглая скобка, заглавная эс, заглавная о, нижний индекс 4; закрывающая круглая скобка, нижний индекс 3")?;
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
    test_prefs("ru", "ClearSpeak", vec![("Verbosity", "Terse")], expr, "заглавная цэ, заглавная аш, 3 одинарная связь заглавная цэ, заглавная аш, 2 одинарная связь заглавная о, заглавная аш")?;
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
    test_prefs("ru", "SimpleSpeak", vec![("Verbosity", "Terse")], 
        expr, "открывающая квадратная скобка, заглавная цэ эль, заглавная о, 2, закрывающая квадратная скобка плюс; открывающая квадратная скобка, заглавная цэ эль, заглавная о, 4, закрывающая квадратная скобка минус")?;
    test_prefs("ru", "SimpleSpeak", vec![("Verbosity", "Medium")], 
        expr, "открывающая квадратная скобка, заглавная цэ эль, заглавная о, нижний индекс 2; закрывающая квадратная скобка, верхний индекс плюс; открывающая квадратная скобка, заглавная цэ эль, заглавная о, нижний индекс 4; закрывающая квадратная скобка, верхний индекс минус")?;
    test_prefs("ru", "SimpleSpeak", vec![("Verbosity", "Verbose")], 
        expr, "открывающая квадратная скобка, заглавная цэ эль, заглавная о, нижний индекс 2; закрывающая квадратная скобка, верхний индекс плюс; открывающая квадратная скобка, заглавная цэ эль, заглавная о, нижний индекс 4; закрывающая квадратная скобка, верхний индекс минус")?;
        return Ok(());
}

#[test]
fn ethylene_with_bond() -> Result<()> {
    let expr = "<math><mrow>
          <msub><mi>H</mi><mn>2</mn></msub><mi>C</mi>
          <mo>=</mo>
          <mi>C</mi><msub><mi>H</mi><mn>2</mn></msub>
      </mrow></math>";
    test_prefs("ru", "SimpleSpeak", vec![("Verbosity", "Terse")], expr, "заглавная аш, 2 заглавная цэ, двойная связь заглавная цэ, заглавная аш, 2")?;
    return Ok(());
}

#[test]
fn ferric_chloride_aq() -> Result<()> {
    let expr = "<math><mrow>
        <mi>Fe</mi>
        <msub><mi>Cl</mi><mn>3</mn></msub>
        <mrow><mo>(</mo><mrow><mi>aq</mi></mrow><mo>)</mo></mrow>
    </mrow></math>";
    test_prefs("ru", "SimpleSpeak", vec![("Verbosity", "Terse")], expr, "заглавная эф е, заглавная цэ эль, 3 водный")?;
    return Ok(());
}

#[test]
fn ethylene_with_colon_bond() -> Result<()> {
    let expr = "<math><mrow>
          <msub><mi>H</mi><mn>2</mn></msub><mi>C</mi>
          <mo>::</mo>
          <mi>C</mi><msub><mi>H</mi><mn>2</mn></msub>
      </mrow></math>";
    test_prefs("ru", "SimpleSpeak", vec![("Verbosity", "Terse")], expr, "заглавная аш, 2 заглавная цэ, двойная связь заглавная цэ, заглавная аш, 2")?;
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
    test_prefs("ru", "ClearSpeak", vec![("Verbosity", "Terse")], expr,
        "14, 6, заглавная цэ; образует, 14, 7, заглавная эн; плюс 0, минус 1, е")?;
    test_prefs("ru", "ClearSpeak", vec![("Verbosity", "Medium")], expr,
        "верхний индекс 14, нижний индекс 6, заглавная цэ; реагирует с образованием; верхний индекс 14, нижний индекс 7, заглавная эн; плюс верхний индекс 0, нижний индекс минус 1, е")?;
    test_prefs("ru", "ClearSpeak", vec![("Verbosity", "Verbose")], expr,
        "верхний индекс 14, нижний индекс 6, заглавная цэ; реагирует с образованием; верхний индекс 14, нижний индекс 7, заглавная эн; плюс, верхний индекс 0, нижний индекс минус 1, е")?;
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
              <mpadded height='0' depth='0'>
                <mphantom>
                  <mn>6</mn>
                </mphantom>
              </mpadded>
            </mrow>
          </mrow>
          <mrow>
            <mrow>
              <mpadded height='0' depth='0'>
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
                  <mpadded height='0'>
                    <mn>6</mn>
                  </mpadded>
                </mrow>
              </mpadded>
            </mrow>
          </mrow>
          <mrow>
            <mrow>
              <mpadded height='0'>
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
              <mpadded height='0' depth='0'>
                <mphantom>
                  <mn>7</mn>
                </mphantom>
              </mpadded>
            </mrow>
          </mrow>
          <mrow>
            <mrow>
              <mpadded height='0' depth='0'>
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
                  <mpadded height='0'>
                    <mn>7</mn>
                  </mpadded>
                </mrow>
              </mpadded>
            </mrow>
          </mrow>
          <mrow>
            <mrow>
              <mpadded height='0'>
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
              <mpadded height='0' depth='0'>
                <mphantom>
                  <mo>&#x2212;</mo>
                  <mn>1</mn>
                </mphantom>
              </mpadded>
            </mrow>
          </mrow>
          <mrow>
            <mrow>
              <mpadded height='0' depth='0'>
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
                  <mpadded height='0'>
                    <mo>&#x2212;</mo>
                    <mn>1</mn>
                  </mpadded>
                </mrow>
              </mpadded>
            </mrow>
          </mrow>
          <mrow>
            <mrow>
              <mpadded height='0'>
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
    test_prefs("ru", "ClearSpeak", vec![("Verbosity", "Terse")], expr,
        "14, 6, заглавная цэ; образует, 14, 7, заглавная эн; плюс 0, минус 1, е")?;
    test_prefs("ru", "ClearSpeak", vec![("Verbosity", "Medium")], expr,
        "верхний индекс 14, нижний индекс 6, заглавная цэ; реагирует с образованием; верхний индекс 14, нижний индекс 7, заглавная эн; плюс верхний индекс 0, нижний индекс минус 1, е")?;
    test_prefs("ru", "ClearSpeak", vec![("Verbosity", "Verbose")], expr,
        "верхний индекс 14, нижний индекс 6, заглавная цэ; реагирует с образованием; верхний индекс 14, нижний индекс 7, заглавная эн; плюс, верхний индекс 0, нижний индекс минус 1, е")?;
        return Ok(());
}

#[test]
fn hcl_na_yields() -> Result<()> {
    let expr = "<math> <mrow>
      <mn>2</mn><mi>H</mi><mi>Cl</mi><mo>+</mo><mn>2</mn><mtext>Na</mtext>
      <mo>&#x2192;</mo>
      <mn>2</mn><mtext>Na</mtext><mi>Cl</mi><mo>+</mo>
      <msub> <mi>H</mi> <mn>2</mn> </msub>
      </mrow>
    </math>";
    test_prefs("ru", "SimpleSpeak", vec![("Verbosity", "Verbose")], expr,
        "2, заглавная аш, заглавная цэ эль; плюс 2 заглавная эн а; реагирует с образованием; 2, заглавная эн а, заглавная цэ эль; плюс заглавная аш, нижний индекс 2")?;
        return Ok(());
}

#[test]
fn mhchem_so4_2plus() -> Result<()> {
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
            <mpadded height='0'>
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
    test_prefs("ru", "SimpleSpeak", vec![("Verbosity", "Terse")], expr, "заглавная эс; заглавная о, 4, 2 плюс")?;
    test_prefs("ru", "SimpleSpeak", vec![("Verbosity", "Medium")], expr, "заглавная эс; заглавная о, нижний индекс 4, верхний индекс 2 плюс")?;
    test_prefs("ru", "SimpleSpeak", vec![("Verbosity", "Verbose")], expr, "заглавная эс; заглавная о, нижний индекс 4, верхний индекс 2 плюс")?;
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
  </math>";
    test_prefs("ru", "SimpleSpeak", vec![("Verbosity", "Terse")],
        expr, "2, заглавная аш, заглавная цэ эль, водный; плюс, 2, заглавная эн а, твердый; образует; 2, заглавная эн а, заглавная цэ эль, водный; плюс, заглавная аш, 2, газ")?;
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
          <mrow data-mjx-texclass='ORD' depth='0' height='0' data-changed='added'>
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
    test_prefs("ru", "SimpleSpeak", vec![("Verbosity", "Terse")],
        expr, "заглавная аш, 2, газ; плюс; заглавная и, 2, газ; находится в равновесии с, 2, заглавная аш, заглавная и, газ")?;
        return Ok(());
}

#[test]
fn mhchem_roman_in_superscript() -> Result<()> {
    let expr = "<math>
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
          <mi>III</mi>
        </mmultiscripts>
        <mo>&#x2063;</mo>
        <mmultiscripts>
          <mi mathvariant='normal' >O</mi>
          <mn>4</mn>
          <none></none>
        </mmultiscripts>
      </mrow>
    </math>";
    test_prefs("ru", "SimpleSpeak", vec![("Verbosity", "Terse")],
        expr, "заглавная эф е, 2; заглавная эф е, 3; заглавная о, 4")?;
        return Ok(());
}

#[test]
fn dropped_msubsup_bug_358() -> Result<()> {
    let expr = r#"<math>
          <mrow class="MJX-TeXAtom-ORD">
              <mrow class="MJX-TeXAtom-ORD">
                <mn>2</mn>
                <mspace width="thinmathspace"></mspace>
                <msubsup>
                  <mtext>SO</mtext>
                  <mrow class="MJX-TeXAtom-ORD">
                    <mn>2</mn>
                  </mrow>
                  <mrow class="MJX-TeXAtom-ORD">
                    <mspace width="0pt" height="0pt" depth=".2em"></mspace>
                  </mrow>
                </msubsup>
                <mo>+</mo>
                <msubsup>
                  <mtext>O</mtext>
                  <mrow class="MJX-TeXAtom-ORD">
                    <mn>2</mn>
                  </mrow>
                  <mrow class="MJX-TeXAtom-ORD">
                    <mspace width="0pt" height="0pt" depth=".2em"></mspace>
                  </mrow>
                </msubsup>
                <mrow class="MJX-TeXAtom-REL">
                  <mover>
                    <mrow class="MJX-TeXAtom-OP MJX-fixedlimits">
                      <mrow class="MJX-TeXAtom-ORD">
                        <mpadded height="0" depth="0">
                          <mrow class="MJX-TeXAtom-ORD">
                            <mo stretchy="false">↽<!-- ↽ --></mo>
                          </mrow>
                          <mspace width="negativethinmathspace"></mspace>
                          <mspace width="negativethinmathspace"></mspace>
                          <mrow class="MJX-TeXAtom-ORD">
                            <mo>−<!-- − --></mo>
                          </mrow>
                        </mpadded>
                      </mrow>
                    </mrow>
                    <mrow class="MJX-TeXAtom-ORD">
                        <mrow class="MJX-TeXAtom-ORD">
                          <mrow class="MJX-TeXAtom-ORD">
                            <mo>−<!-- − --></mo>
                          </mrow>
                          <mspace width="negativethinmathspace"></mspace>
                          <mspace width="negativethinmathspace"></mspace>
                          <mrow class="MJX-TeXAtom-ORD">
                            <mo stretchy="false">⇀<!-- ⇀ --></mo>
                          </mrow>
                        </mrow>
                    </mrow>
                  </mover>
                </mrow>
                <mn>2</mn>
                <mspace width="thinmathspace"></mspace>
                <msubsup>
                  <mtext>SO</mtext>
                  <mrow class="MJX-TeXAtom-ORD">
                    <mn>3</mn>
                  </mrow>
                  <mrow class="MJX-TeXAtom-ORD">
                    <mspace width="0pt" height="0pt" depth=".2em"></mspace>
                  </mrow>
                </msubsup>
              </mrow>
          </mrow>
      </math>"#;
    test_prefs("ru", "SimpleSpeak", vec![("Verbosity", "Terse")],
        expr, "2, заглавная эс, заглавная о, 2; плюс; заглавная о, 2 находится в равновесии с, 2, заглавная эс, заглавная о, 3")?;
        return Ok(());
}


