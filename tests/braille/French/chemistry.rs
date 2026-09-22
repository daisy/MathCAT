// French braille tests for chemistry
// From "Notation braille dans le domaine de la chimie" (Première édition juin 2008)
// BrailleDocs/notation_braille_chimie2008.pdf
use crate::common::*;
use anyhow::Result;

// --- I. Atomic and molecular writing ---

#[test]
fn atom_I_1_1() -> Result<()> {
    let expr = r#"<math><mi mathvariant="normal">H</mi></math>"#;
    test_braille("French", expr, "⠨⠓")?;
    return Ok(());
}

#[test]
fn atom_I_1_2() -> Result<()> {
    let expr = r#"<math><mi mathvariant="normal">Al</mi></math>"#;
    test_braille("French", expr, "⠨⠁⠇")?;
    return Ok(());
}

#[test]
fn isotope_I_2_1() -> Result<()> {
    // ¹⁶₈O
    let expr = r#"<math><mmultiscripts>
        <mi mathvariant="normal">O</mi>
        <mprescripts/>
        <mn>8</mn>
        <mn>16</mn>
    </mmultiscripts></math>"#;
    test_braille("French", expr, "⠨⠕⠠⠢⠳⠠⠈⠡⠫")?;
    return Ok(());
}

#[test]
#[ignore = "average-mass left subscript not handled by French atomic-number rule"]
fn isotope_I_2_2() -> Result<()> {
    // ²⁵₂₅⁵⁴.⁹Mn (Z=25, average mass 54.9, A=25)
    let expr = r#"<math intent=':chemical-formula'><mmultiscripts>
        <mi mathvariant="normal">Mn</mi>
        <mprescripts/>
        <mn>25</mn>
        <mn>25</mn>
        <mn>54.9</mn>
        <none/>
    </mmultiscripts></math>"#;
    test_braille("French", expr, "⠨⠍⠝⠠⠢⠣⠱⠠⠢⠢⠱⠹⠂⠪⠠⠈⠱⠱")?;
    return Ok(());
}

#[test]
fn molecule_I_3_1() -> Result<()> {
    let expr = r#"<math><msub><mi mathvariant="normal">H</mi><mn>2</mn></msub><mi mathvariant="normal">O</mi></math>"#;
    test_braille("French", expr, "⠨⠓⠣⠨⠕")?;
    return Ok(());
}

#[test]
fn molecule_I_3_2() -> Result<()> {
    // Cu(NO₃)₂
    let expr = r#"<math><mi mathvariant="normal">Cu</mi><mo>(</mo><msub><mi>NO</mi><mn>3</mn></msub><mo>)</mo><mn>2</mn></math>"#;
    test_braille("French", expr, "⠨⠉⠥⠦⠨⠝⠨⠕⠩⠴⠣")?;
    return Ok(());
}

#[test]
fn bond_I_4_1() -> Result<()> {
    let expr= r#"<math><mi mathvariant="normal">H</mi><mo>-</mo><mi mathvariant="normal">H</mi></math>"#;
    test_braille("French", expr, "⠨⠓⠤⠨⠓")?;
    return Ok(());
}

#[test]
fn bond_I_4_2() -> Result<()> {
    let expr= r#"<math><mi mathvariant="normal">O</mi><mo>=</mo><mi mathvariant="normal">O</mi></math>"#;
    test_braille("French", expr, "⠨⠕⠶⠨⠕")?;
    return Ok(());
}

#[test]
fn bond_I_4_3() -> Result<()> {
    let expr = r#"<math><mi mathvariant="normal">N</mi><mo>≡</mo><mi mathvariant="normal">N</mi></math>"#;
    test_braille("French", expr, "⠨⠝⠿⠨⠝")?;
    return Ok(());
}

#[test]
fn lewis_I_5_1() -> Result<()> {
    // O with 2 Lewis doublets (one bar above, one below) → ⠨⠕⠰⠣⠙⠆
    let expr = r#"<math intent=':chemical-formula'>
        <munderover><mi mathvariant="normal">O</mi><mo>―</mo><mo>―</mo></munderover>
    </math>"#;
    test_braille("French", expr, "⠨⠕⠰⠣⠙⠆")?;
    return Ok(());
}

#[test]
fn lewis_I_5_2() -> Result<()> {
    // H−O−H with 2 doublets on O
    let expr = r#"<math intent=':chemical-formula'><mi mathvariant="normal">H</mi><mo>-</mo>
        <munderover><mi mathvariant="normal">O</mi><mo>―</mo><mo>―</mo></munderover>
        <mo>-</mo> <mi mathvariant="normal">H</mi></math>"#;
    test_braille("French", expr, "⠨⠓⠤⠨⠕⠰⠣⠙⠆⠤⠨⠓")?;
    return Ok(());
}

#[test]
#[ignore = "Lewis notation with bars on left/right of an atom not implemented"]
fn lewis_I_5_3() -> Result<()> {
    // H−Cl with 3 doublets on Cl
    let expr = r#"<math intent=':chemical-formula'><mi mathvariant="normal">H</mi><mo>-</mo>
        <munderover><mi mathvariant="normal">Cl</mi><mo>―</mo><mo>―</mo></munderover>
        <mo>|</mo></math>"#;
    test_braille("French", expr, "⠨⠓⠤⠨⠉⠇⠰⠩⠙⠆")?;
    return Ok(());
}

// --- II. Ionic writing ---

#[test]
fn ion_II_1_1() -> Result<()> {
    let expr = r#"<math><msup><mi mathvariant="normal">H</mi><mo>+</mo></msup></math>"#;
    test_braille("French", expr, "⠨⠓⠈⠖")?;
    return Ok(());
}

#[test]
fn ion_II_1_2() -> Result<()> {
    let expr = r#"<math><msup><mi mathvariant="normal">O</mi><mrow><mn>2</mn><mo>-</mo></mrow></msup></math>"#;
    test_braille("French", expr, "⠨⠕⠈⠰⠣⠤⠆")?;
    return Ok(());
}

#[test]
fn ion_II_1_3() -> Result<()> {
    let expr= r#"<math><msubsup><mi>SO</mi><mn>4</mn><mrow><mn>2</mn><mo>-</mo></mrow></msubsup></math>"#;
    test_braille("French", expr, "⠨⠎⠨⠕⠹⠈⠰⠣⠤⠆")?;
    return Ok(());
}

#[test]
fn dissociation_II_2_1() -> Result<()> {
    // (Cu²⁺; SO₄²⁻)
    let expr = r#"<math intent=':chemical-formula'><mo>(</mo>
        <msup><mi mathvariant="normal">Cu</mi><mrow><mn>2</mn><mo>+</mo></mrow></msup><mo>;</mo>
        <msubsup><mi>SO</mi><mn>4</mn><mrow><mn>2</mn><mo>-</mo></mrow></msubsup>
        <mo>)</mo></math>"#;
    test_braille("French", expr, "⠦⠨⠉⠥⠈⠰⠣⠖⠆⠠⠆⠨⠎⠨⠕⠹⠈⠰⠣⠤⠆⠴")?;
    return Ok(());
}

#[test]
fn dipole_II_3_1() -> Result<()> {
    // Hᵟ⁺ − Clᵟ⁻
    let expr = r#"<math>
        <msup><mi mathvariant="normal">H</mi><msup><mi>δ</mi><mo>+</mo></msup></msup>
        <mo>-</mo>
        <msup><mi mathvariant="normal">Cl</mi><msup><mi>δ</mi><mo>-</mo></msup></msup>
    </math>"#;
    test_braille("French", expr, "⠨⠓⠈⠰⠘⠙⠈⠖⠆⠤⠨⠉⠇⠈⠰⠘⠙⠈⠤⠆")?;
    return Ok(());
}

// --- III. Chemical reactions and transformations ---

#[test]
fn reaction_III_2_1() -> Result<()> {
    // C(s)+O₂(g)→CO₂(g)
    let expr = r#"<math>
        <mi mathvariant="normal">C</mi><mo>(</mo><mi mathvariant="normal">s</mi><mo>)</mo>
        <mo>+</mo>
        <msub><mi mathvariant="normal">O</mi><mn>2</mn></msub><mo>(</mo><mi mathvariant="normal">g</mi><mo>)</mo>
        <mo>&#x2192;</mo>
        <msub><mi>CO</mi><mn>2</mn></msub><mo>(</mo><mi mathvariant="normal">g</mi><mo>)</mo>
    </math>"#;
    test_braille("French", expr, "⠨⠉⠦⠎⠴⠖⠨⠕⠣⠦⠛⠴⠸⠱⠨⠉⠨⠕⠣⠦⠛⠴")?;
    return Ok(());
}

#[test]
fn reaction_III_2_2() -> Result<()> {
    // CO(g)+H₂O(l)→CO₂(g)+H₂(g) — line-continuation ⠐ omitted (single-line MathML)
    let expr = r#"<math>
        <mi>CO</mi><mo>(</mo><mi mathvariant="normal">g</mi><mo>)</mo>
        <mo>+</mo>
        <msub><mi mathvariant="normal">H</mi><mn>2</mn></msub><mi mathvariant="normal">O</mi><mo>(</mo><mi mathvariant="normal">l</mi><mo>)</mo>
        <mo>&#x2192;</mo>
        <msub><mi>CO</mi><mn>2</mn></msub><mo>(</mo><mi mathvariant="normal">g</mi><mo>)</mo>
        <mo>+</mo>
        <msub><mi mathvariant="normal">H</mi><mn>2</mn></msub><mo>(</mo><mi mathvariant="normal">g</mi><mo>)</mo>
    </math>"#;
    test_braille("French", expr, "⠨⠉⠨⠕⠦⠛⠴⠖⠨⠓⠣⠨⠕⠦⠇⠴⠸⠱⠨⠉⠨⠕⠣⠦⠛⠴⠖⠨⠓⠣⠦⠛⠴")?;
    return Ok(());
}

#[test]
fn reaction_III_2_3() -> Result<()> {
    // HCl(g)+H₂O(l)→(eau) H₃O⁺(aq)+Cl⁻(aq) — line-continuation ⠐ omitted
    let expr = r#"<math>
        <mi>HCl</mi><mo>(</mo><mi mathvariant="normal">g</mi><mo>)</mo>
        <mo>+</mo>
        <msub><mi mathvariant="normal">H</mi><mn>2</mn></msub><mi mathvariant="normal">O</mi><mo>(</mo><mi mathvariant="normal">l</mi><mo>)</mo>
        <mo>&#x2192;</mo><mo>(</mo><mi>eau</mi><mo>)</mo>
        <msub><mi mathvariant="normal">H</mi><mn>3</mn></msub><msup><mi mathvariant="normal">O</mi><mo>+</mo></msup>
        <mo>(</mo><mi>aq</mi><mo>)</mo>
        <mo>+</mo>
        <msup><mi mathvariant="normal">Cl</mi><mo>-</mo></msup>
        <mo>(</mo><mi>aq</mi><mo>)</mo>
    </math>"#;
    test_braille("French", expr, "⠨⠓⠨⠉⠇⠦⠛⠴⠖⠨⠓⠣⠨⠕⠦⠇⠴⠸⠱⠦⠑⠁⠥⠴⠨⠓⠩⠨⠕⠈⠖⠦⠁⠟⠴⠖⠨⠉⠇⠈⠤⠦⠁⠟⠴")?;
    return Ok(());
}

#[test]
fn emerges_III_3_1() -> Result<()> {
    // 2H₃O⁺ + FeS → Fe²⁺ + 2H₂O + H₂S↑
    let expr = r#"<math>
        <mn>2</mn><msub><mi mathvariant="normal">H</mi><mn>3</mn></msub><msup><mi mathvariant="normal">O</mi><mo>+</mo></msup>
        <mo>+</mo>
        <mi>FeS</mi>
        <mo>&#x2192;</mo>
        <msup><mi mathvariant="normal">Fe</mi><mrow><mn>2</mn><mo>+</mo></mrow></msup>
        <mo>+</mo>
        <mn>2</mn><msub><mi mathvariant="normal">H</mi><mn>2</mn></msub><mi mathvariant="normal">O</mi>
        <mo>+</mo>
        <msub><mi mathvariant="normal">H</mi><mn>2</mn></msub><mi mathvariant="normal">S</mi><mo>&#x2191;</mo>
    </math>"#;
    test_braille("French", expr, "⠣⠨⠓⠩⠨⠕⠈⠰⠖⠆⠖⠨⠋⠑⠨⠎⠸⠱⠨⠋⠑⠈⠰⠣⠖⠆⠖⠣⠨⠓⠣⠨⠕⠖⠨⠓⠣⠨⠎⠘⠱")?;
    return Ok(());
}

#[test]
fn precipitate_III_4_1() -> Result<()> {
    // H₂S(g)+Cl₂(g)→2HCl(g)+S(s)↓
    let expr = r#"<math>
        <msub><mi mathvariant="normal">H</mi><mn>2</mn></msub><mi mathvariant="normal">S</mi><mo>(</mo><mi mathvariant="normal">g</mi><mo>)</mo>
        <mo>+</mo>
        <msub><mi mathvariant="normal">Cl</mi><mn>2</mn></msub><mo>(</mo><mi mathvariant="normal">g</mi><mo>)</mo>
        <mo>&#x2192;</mo>
        <mn>2</mn><mi>HCl</mi><mo>(</mo><mi mathvariant="normal">g</mi><mo>)</mo>
        <mo>+</mo>
        <mi mathvariant="normal">S</mi><mo>(</mo><mi mathvariant="normal">s</mi><mo>)</mo><mo>&#x2193;</mo>
    </math>"#;
    test_braille("French", expr, "⠨⠓⠣⠨⠎⠦⠛⠴⠖⠨⠉⠇⠣⠦⠛⠴⠸⠱⠣⠨⠓⠨⠉⠇⠦⠛⠴⠖⠨⠎⠦⠎⠴⠨⠱")?;
    return Ok(());
}

#[test]
fn equilibrium_III_5_1() -> Result<()> {
    // N₂(g)+3H₂(g)⇌2NH₃(g)
    let expr= r#"<math><msub><mi mathvariant="normal">N</mi><mn>2</mn></msub><mo>(</mo><mi mathvariant="normal">g</mi><mo>)</mo><mo>+</mo><mn>3</mn><msub><mi mathvariant="normal">H</mi><mn>2</mn></msub><mo>(</mo><mi mathvariant="normal">g</mi><mo>)</mo><mo>&#x21CC;</mo><mn>2</mn><msub><mi>NH</mi><mn>3</mn></msub><mo>(</mo><mi mathvariant="normal">g</mi><mo>)</mo></math>"#;
    test_braille("French", expr, "⠨⠝⠣⠦⠛⠴⠖⠩⠨⠓⠣⠦⠛⠴⠸⠻⠣⠨⠝⠨⠓⠩⠦⠛⠴")?;
    return Ok(());
}

#[test]
fn equilibrium_III_5_2() -> Result<()> {
    // 2H₂O ⇌(1;2) H₃O⁺ + OH⁻  (line-continuation ⠐ omitted)
    // Note: braille doesn't match the example -- no parens around the '1' and '2'
    let expr= r#"<math><mn>2</mn><msub><mi mathvariant="normal">H</mi><mn>2</mn></msub><mi mathvariant="normal">O</mi>
        <munderover><mo>&#x21CC;</mo><mn>2</mn><mn>1</mn></munderover>
        <msub><mi mathvariant="normal">H</mi><mn>3</mn></msub><msup><mi mathvariant="normal">O</mi><mo>+</mo></msup><mo>+</mo><msup><mi>OH</mi><mo>-</mo></msup></math>"#;
    test_braille("French", expr, "⠣⠨⠓⠣⠨⠕⠸⠻⠦⠡⠆⠣⠴⠨⠓⠩⠨⠕⠈⠰⠖⠆⠖⠨⠕⠨⠓⠈⠤")?;
    return Ok(());
}

#[test]
fn equilibrium_III_5_3() -> Result<()> {
    let expr= r#"<math><mi mathvariant="normal">R</mi><mo>-</mo><mi>CO</mi><mo>-</mo><mi mathvariant="normal">O</mi><mo>-</mo>
                             <mi mathvariant="normal">H</mi><mo>+</mo><mi mathvariant="normal">R</mi><mo>'</mo><mo>-</mo><mi>OH</mi>
                    <munderover><mo>&#x21CC;</mo><mi>hydrolyse</mi><mi>est&#xE9;rification</mi></munderover>
                    <mi mathvariant="normal">R</mi><mo>-</mo><mi>CO</mi><mo>-</mo><mi mathvariant="normal">O</mi><mo>-</mo>
                    <mi mathvariant="normal">R</mi><mo>'</mo><mo>+</mo>
                            <msub><mi mathvariant="normal">H</mi><mn>2</mn></msub><mi mathvariant="normal">O</mi></math>"#;
    test_braille("French", expr, "⠨⠗⠤⠨⠉⠨⠕⠤⠨⠕⠤⠨⠓⠖⠨⠗⠄⠤⠨⠕⠨⠓⠸⠻⠦⠑⠎⠞⠿⠗⠊⠋⠊⠉⠁⠞⠊⠕⠝⠆⠓⠽⠙⠗⠕⠇⠽⠎⠑⠴⠨⠗⠤⠨⠉⠨⠕⠤⠨⠕⠤⠨⠗⠄⠖⠨⠓⠣⠨⠕")?;
    return Ok(());
}

#[test]
fn equilibrium_III_5_4() -> Result<()> {
    // A ⇌ B with hν / empty half-zone ⠐⠂
    let expr = r#"<math>
        <mi mathvariant="normal">A</mi>
        <munderover><mo>&#x21CC;</mo><mrow/><mrow><mi>h</mi><mi>ν</mi></mrow></munderover>
        <mi mathvariant="normal">B</mi>
    </math>"#;
    test_braille("French", expr, "⠨⠁⠸⠻⠦⠓⠘⠝⠆⠐⠂⠴⠨⠃")?;
    return Ok(());
}

// --- IV. Some notes (redox pairs / mesomerism) ---

#[test]
fn redox_IV_1_1() -> Result<()> {
    let expr = r#"<math>
        <msup><mi mathvariant="normal">Cu</mi><mrow><mn>2</mn><mo>+</mo></mrow></msup>
        <mo>/</mo>
        <mi mathvariant="normal">Cu</mi>
    </math>"#;
    test_braille("French", expr, "⠨⠉⠥⠈⠰⠣⠖⠆⠌⠨⠉⠥")?;
    return Ok(());
}

#[test]
fn redox_IV_1_2() -> Result<()> {
    let expr = r#"<math>
        <msup><mi mathvariant="normal">Fe</mi><mrow><mn>3</mn><mo>+</mo></mrow></msup>
        <mo>/</mo>
        <msup><mi mathvariant="normal">Fe</mi><mrow><mn>2</mn><mo>+</mo></mrow></msup>
    </math>"#;
    test_braille("French", expr, "⠨⠋⠑⠈⠰⠩⠖⠆⠌⠨⠋⠑⠈⠰⠣⠖⠆")?;
    return Ok(());
}

#[test]
fn redox_IV_1_3() -> Result<()> {
    let expr = r#"<math>
        <msub><mi mathvariant="normal">H</mi><mn>3</mn></msub><msup><mi mathvariant="normal">O</mi><mo>+</mo></msup>
        <mo>/</mo>
        <msub><mi mathvariant="normal">H</mi><mn>2</mn></msub><mi mathvariant="normal">O</mi>
    </math>"#;
    test_braille("French", expr, "⠨⠓⠩⠨⠕⠈⠖⠌⠨⠓⠣⠨⠕")?;
    return Ok(());
}

#[test]
fn mesomerism_IV_2_1() -> Result<()> {
    // (CH₃)₂C=O ↔ (CH₃)₂C⁺−O⁻
    let expr = r#"<math>
        <msub><mrow><mo>(</mo><msub><mi>CH</mi><mn>3</mn></msub><mo>)</mo></mrow><mn>2</mn></msub>
        <mi mathvariant="normal">C</mi><mo>=</mo><mi mathvariant="normal">O</mi>
        <mo>&#x2194;</mo>
        <msub><mrow><mo>(</mo><msub><mi>CH</mi><mn>3</mn></msub><mo>)</mo></mrow><mn>2</mn></msub>
        <msup><mi mathvariant="normal">C</mi><mo>+</mo></msup><mo>-</mo>
        <msup><mi mathvariant="normal">O</mi><mo>-</mo></msup>
    </math>"#;
    test_braille("French", expr, "⠦⠨⠉⠨⠓⠩⠴⠣⠨⠉⠶⠨⠕⠐⠻⠦⠨⠉⠨⠓⠩⠴⠣⠨⠉⠈⠖⠤⠨⠕⠈⠤")?;
    return Ok(());
}
