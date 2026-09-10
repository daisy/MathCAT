// Polish math braille tests.
//
// The expected strings come from "Brajlowska notacja matematyczna, fizyczna,
// chemiczna", 2nd edition, Krakow-Laski-Lodz 2011 (recommended by the Polish
// Ministry of National Education). The guide prints braille with a 6-dot font,
// so each expectation below was decoded from the guide's own character table
// (page 1) into Unicode braille cells.
//
// This is the first instalment of the code: digits, letters, operators and
// relations, plus roots and short-form fractions. Chapters not yet covered
// (spacing precedence, indices, physics and chemistry) have no tests here.
use crate::common::*;
use anyhow::Result;

// Page 4: numbers are the first-series letters preceded by the number sign.
// The guide writes 6 as "e (number sign + cell for 6).
#[test]
fn liczba_jednocyfrowa() -> Result<()> {
    let expr = r#"<math><mn>6</mn></math>"#;
    test_braille("Polish", expr, "⠼⠋")?;
    return Ok(());
}

// Page 4: 68 is written "eg - the number sign is NOT repeated for the second digit.
#[test]
fn liczba_dwucyfrowa() -> Result<()> {
    let expr = r#"<math><mn>68</mn></math>"#;
    test_braille("Polish", expr, "⠼⠋⠓")?;
    return Ok(());
}

// Page 6: a lower-case Latin letter is preceded by the letter sign (dot 6).
#[test]
fn litera_mala() -> Result<()> {
    let expr = r#"<math><mi>a</mi></math>"#;
    test_braille("Polish", expr, "⠠⠁")?;
    return Ok(());
}

// Page 10: addition. The guide's definition is _*z, i.e. blank + dots 2-3-5.
#[test]
fn dodawanie() -> Result<()> {
    let expr = r#"<math><mn>5</mn><mo>+</mo><mn>2</mn></math>"#;
    test_braille("Polish", expr, "⠼⠑⠀⠖⠼⠃")?;
    return Ok(());
}

// Page 11: equality is defined as _<z, i.e. blank + dots 2-3-5-6.
#[test]
fn rownosc() -> Result<()> {
    let expr = r#"<math><mi>x</mi><mo>=</mo><mn>1</mn></math>"#;
    test_braille("Polish", expr, "⠠⠭⠀⠶⠼⠁")?;
    return Ok(());
}

// Page 37: the simple root sign is ASCII '2' in the guide = dots 1-4-6.
// The guide writes sqrt(16) as 2"`e.
#[test]
fn pierwiastek_kwadratowy() -> Result<()> {
    let expr = r#"<math><msqrt><mn>16</mn></msqrt></math>"#;
    test_braille("Polish", expr, "⠩⠼⠁⠋")?;
    return Ok(());
}

// Page 25: the short fraction form omits the opening/closing signs and any
// blanks around the fraction line. The guide writes 2/3 as "a7"b.
#[test]
fn ulamek_skrocony() -> Result<()> {
    let expr = r#"<math><mfrac><mn>2</mn><mn>3</mn></mfrac></math>"#;
    test_braille("Polish", expr, "⠼⠃⠳⠼⠉")?;
    return Ok(());
}

// Page 31, rule 1: a whole-number exponent is written after the exponent key sign
// with DROPPED digits and no number sign. The guide writes 5^4 as "d/. -
// number sign, cell for 5, exponent sign, dropped 4.
#[test]
fn potega_liczba_calkowita() -> Result<()> {
    let expr = r#"<math><msup><mn>5</mn><mn>4</mn></msup></math>"#;
    test_braille("Polish", expr, "⠼⠑⠬⠲")?;
    return Ok(());
}

// Page 31: the guide's own example b^10 = &a/+( - letter sign, cell for b,
// exponent sign, dropped 1, dropped 0.
#[test]
fn potega_dwucyfrowa() -> Result<()> {
    let expr = r#"<math><msup><mi>b</mi><mn>10</mn></msup></math>"#;
    test_braille("Polish", expr, "⠠⠃⠬⠂⠴")?;
    return Ok(());
}

// Page 31: the guide's own example a_12 = &`0+: - letter sign, cell for a,
// lower index sign, dropped 1, dropped 2.
#[test]
fn wskaznik_dolny_dwucyfrowy() -> Result<()> {
    let expr = r#"<math><msub><mi>a</mi><mn>12</mn></msub></math>"#;
    test_braille("Polish", expr, "⠠⠁⠡⠂⠆")?;
    return Ok(());
}

// Page 31: a_0 is written &`0( - the dropped zero, again with no number sign.
#[test]
fn wskaznik_dolny_zero() -> Result<()> {
    let expr = r#"<math><msub><mi>a</mi><mn>0</mn></msub></math>"#;
    test_braille("Polish", expr, "⠠⠁⠡⠴")?;
    return Ok(());
}

// Pages 22-24: a script containing a fraction cannot use the SIMPLE projector,
// because a simple one may hold nothing but a single row (p. 22). The COMPOUND
// sign is the simple sign preceded by dot 5, and it has to be closed by the
// compound terminator. Measured from the guide: on p. 23 the compound signs are
// two cells wide against one cell on p. 22, the extra leading cell being dot 5.
#[test]
fn potega_zlozona_ulamek() -> Result<()> {
    let expr = r#"<math><msup><mi>a</mi><mfrac><mn>1</mn><mn>2</mn></mfrac></msup></math>"#;
    test_braille("Polish", expr, "⠠⠁⠐⠬⠼⠁⠳⠼⠃⠐⠱")?;
    return Ok(());
}

// The same for a lower index holding a fraction.
#[test]
fn wskaznik_zlozony_ulamek() -> Result<()> {
    let expr = r#"<math><msub><mi>a</mi><mfrac><mn>1</mn><mn>2</mn></mfrac></msub></math>"#;
    test_braille("Polish", expr, "⠠⠁⠐⠡⠼⠁⠳⠼⠃⠐⠱")?;
    return Ok(());
}

// Page 7: a Greek letter is its own key sign plus the cell of the same-sounding
// Latin letter. Lower case takes dots 5-6, so alpha is dots 5-6 then the cell
// for "a". Verified against all 24 letters of the guide's table.
#[test]
fn grecka_mala_alfa() -> Result<()> {
    let expr = r#"<math><mi>&#x3B1;</mi></math>"#;
    test_braille("Polish", expr, "⠰⠁")?;
    return Ok(());
}

// Page 7: pi is dots 5-6 then the cell for "p".
#[test]
fn grecka_mala_pi() -> Result<()> {
    let expr = r#"<math><mi>&#x3C0;</mi></math>"#;
    test_braille("Polish", expr, "⠰⠏")?;
    return Ok(());
}

// Page 7: an upper-case Greek letter has ONE sign of its own (dots 4-5-6), not a
// capital sign stacked onto the Greek sign. Omega is dots 4-5-6 then "w".
#[test]
fn grecka_wielka_omega() -> Result<()> {
    let expr = r#"<math><mi>&#x3A9;</mi></math>"#;
    test_braille("Polish", expr, "⠸⠺")?;
    return Ok(());
}

// Page 16: set membership. The guide's definition table gives "nalezy" as two
// cells, dot 4 followed by dots 1-5.
#[test]
fn nalezy_do_zbioru() -> Result<()> {
    let expr = r#"<math><mi>x</mi><mo>&#x2208;</mo><mi>A</mi></math>"#;
    test_braille("Polish", expr, "⠠⠭⠀⠈⠑⠨⠁")?;
    return Ok(());
}

// Page 48: the guide distinguishes three arrows whose ASCII transcription in the
// PDF is IDENTICAL - the difference is a leading cell rendered as a space.
// Left arrow carries that dot-5 prefix, the right arrow does not.
#[test]
fn strzalka_w_prawo() -> Result<()> {
    let expr = r#"<math><mi>a</mi><mo>&#x2192;</mo><mi>b</mi></math>"#;
    test_braille("Polish", expr, "⠠⠁⠒⠂⠠⠃")?;
    return Ok(());
}

#[test]
fn strzalka_w_lewo() -> Result<()> {
    let expr = r#"<math><mi>a</mi><mo>&#x2190;</mo><mi>b</mi></math>"#;
    test_braille("Polish", expr, "⠠⠁⠐⠒⠠⠃")?;
    return Ok(());
}

// Page 52: trigonometric function names get their own contraction - a dots-1-2-4-6
// prefix plus one cell for the function, so "sin" is two cells, not three letters.
#[test]
fn funkcja_sinus() -> Result<()> {
    let expr = r#"<math><mi>sin</mi><mi>x</mi></math>"#;
    test_braille("Polish", expr, "⠫⠎⠠⠭")?;
    return Ok(());
}

// Page 53: the inverse functions insert dot 2 after that prefix and keep the cell
// of the base function - arcsin is the sin cell with the arc marker in front.
#[test]
fn funkcja_arcus_sinus() -> Result<()> {
    let expr = r#"<math><mi>arcsin</mi><mi>x</mi></math>"#;
    test_braille("Polish", expr, "⠫⠂⠎⠠⠭")?;
    return Ok(());
}

// The whole point of the work: an NVDA user has to be able to PICK this code.
// get_supported_braille_codes() lists the subdirectories of Rules/Braille, so a
// new directory is enough - but that is a claim until measured, and a code that
// compiles yet never shows up in the preference list is useless to a reader.
#[test]
fn kod_polski_jest_na_liscie_wyboru() -> Result<()> {
    use libmathcat::interface::{get_supported_braille_codes, set_rules_dir};
    set_rules_dir(abs_rules_dir_path()).unwrap();
    let codes = get_supported_braille_codes()?;
    assert!(
        codes.contains(&"Polish".to_string()),
        "kod 'Polish' nie jest widoczny na liscie kodow brajlowskich: {codes:?}"
    );
    return Ok(());
}

// Page 59 defines a PRECEDENCE between the spacing groups (C and C' absolute,
// then B and B', then A and A'). Only the group-B half is implemented so far:
// relations and binary operators take their blank on the left. The conflict
// resolution itself is NOT implemented - see the comment in definitions.yaml.
// This test therefore only pins the group-B behaviour that does work.
// Page 58-59: group B signs take the blank cell on the LEFT only ("znaki pisane z
// odstepem z lewej strony"). What follows them is decided by the NEXT sign's own
// group, and rule 1 says two signs are written with no gap when the second one is
// from group C. The number sign and the letter/capital sign behave that way here,
// which the guide's own examples confirm: "x = 1" reads letter-sign, x, blank,
// equals, number-sign, 1 - with no blank after the relation.
#[test]
fn grupa_b_daje_odstepy() -> Result<()> {
    let expr = r#"<math><mi>a</mi><mo>=</mo><mi>b</mi></math>"#;
    test_braille("Polish", expr, "⠠⠁⠀⠶⠠⠃")?;
    return Ok(());
}

// The measured result for a prime after "=" is "a = prime" with NO blanks at all,
// but that is NOT the work of a precedence rule: removing the group-C condition
// entirely leaves this output unchanged, and forcing the condition to always-true
// changes nothing either (measured both ways). The blanks are absent because a
// trailing "mo" has no following sibling to be spaced from, so this test records
// current behaviour rather than proving the precedence works.
#[test]
fn prim_po_relacji_bez_odstepu() -> Result<()> {
    let expr = r#"<math><mi>a</mi><mo>=</mo><mo>&#x2032;</mo></math>"#;
    test_braille("Polish", expr, "⠠⠁⠶⠔")?;
    return Ok(());
}

// Page 12: absolute value has its OWN pair of signs - opening dots 4 + 1-2-3 and
// closing dots 4-5-6 - not a repeated bar. Before this rule the vertical bar had
// no entry at all and leaked into the output as raw ASCII "|".
#[test]
fn wartosc_bezwzgledna() -> Result<()> {
    let expr = r#"<math><mrow><mo>|</mo><mi>a</mi><mo>|</mo></mrow></math>"#;
    test_braille("Polish", expr, "⠈⠇⠠⠁⠸")?;
    return Ok(());
}

// Page 49: the vector arrow is a BRACKETING key sign (dots 4-5, 2-5, 2) placed in
// front of the whole expression it covers. Taken from the guide's own example.
#[test]
fn wektor_nad_litera() -> Result<()> {
    let expr = r#"<math><mover><mi>u</mi><mo>&#x2192;</mo></mover></math>"#;
    test_braille("Polish", expr, "⠨⠒⠂⠠⠥")?;
    return Ok(());
}

// Page 56: a limit is written as a lower index of the operator, closed by the
// simple-projector terminator - the guide writes "lim x->oo" as number sign, l,
// index sign, x, arrow, infinity, blank.
#[test]
fn granica_pod_symbolem() -> Result<()> {
    let expr = r#"<math><munder><mi>lim</mi><mi>n</mi></munder></math>"#;
    test_braille("Polish", expr, "⠼⠇⠡⠠⠝⠀")?;
    return Ok(());
}

// Page 46: a matrix is bracketed with dots 1-2-6 ... 3-4-6, entries and rows are
// separated by a blank cell.
#[test]
fn macierz_dwa_na_dwa() -> Result<()> {
    let expr = r#"<math><mtable><mtr><mtd><mn>1</mn></mtd><mtd><mn>2</mn></mtd></mtr><mtr><mtd><mn>3</mn></mtd><mtd><mn>4</mn></mtd></mtr></mtable></math>"#;
    test_braille("Polish", expr, "⠣⠼⠁⠀⠼⠃⠀⠼⠉⠀⠼⠙⠜")?;
    return Ok(());
}

// A big operator with both limits. The guide gives sum no sign of its own (only
// the Greek capitals on p. 7), so it comes out as capital sigma, then the lower
// index and the upper index, each closed by the terminator.
#[test]
fn suma_z_granicami() -> Result<()> {
    let expr = r#"<math><munderover><mo>&#x2211;</mo><mi>i</mi><mi>n</mi></munderover></math>"#;
    test_braille("Polish", expr, "⠸⠎⠡⠠⠊⠀⠬⠠⠝⠀")?;
    return Ok(());
}

// Page 5: in braille the integer part is separated from the fraction by a COMMA
// (dot 2) and never by a point - the guide says braille uses only the comma, so
// print notation's point/comma ambiguity does not arise. Its example: 7,29.
#[test]
fn liczba_dziesietna() -> Result<()> {
    let expr = r#"<math><mn>7,29</mn></math>"#;
    test_braille("Polish", expr, "⠼⠛⠂⠃⠊")?;
    return Ok(());
}

// Page 5: per cent follows the number.
#[test]
fn procent() -> Result<()> {
    let expr = r#"<math><mn>25</mn><mo>%</mo></math>"#;
    test_braille("Polish", expr, "⠼⠃⠑⠼⠚⠴")?;
    return Ok(());
}

// Page 25: the FULL fraction form brackets the fraction and puts a blank cell on
// both sides of the fraction line; the guide writes 2/3 that way as well.
#[test]
fn ulamek_pelny_z_odstepami() -> Result<()> {
    let expr = r#"<math><mfrac><mi>x</mi><mi>y</mi></mfrac></math>"#;
    test_braille("Polish", expr, "⠠⠭⠳⠽")?;
    return Ok(());
}

// Page 51: the guide gives both a full and a SHORT form for angle units and tells
// the transcriber to prefer the short one; its own example writes 30 degrees with
// the bare cell, no prefix.
#[test]
fn stopien_katowy() -> Result<()> {
    let expr = r#"<math><mn>30</mn><mo>&#xB0;</mo></math>"#;
    test_braille("Polish", expr, "⠼⠉⠚⠴")?;
    return Ok(());
}

// Page 48 gives a whole family of plane-figure signs behind one prefix (dots
// 1-2-4-5-6). Verified across all seven figures in the guide's table.
#[test]
fn figura_trojkat() -> Result<()> {
    let expr = r#"<math><mo>&#x25B3;</mo><mi>A</mi><mi>B</mi><mi>C</mi></math>"#;
    test_braille("Polish", expr, "⠻⠲⠨⠁⠨⠃⠨⠉")?;
    return Ok(());
}

// Page 48: a negated relation is the affirmative one with dot-3-5 in front.
// Measured on eight pairs in the guide, all consistent.
#[test]
fn nierownolegle() -> Result<()> {
    let expr = r#"<math><mi>a</mi><mo>&#x2226;</mo><mi>b</mi></math>"#;
    test_braille("Polish", expr, "⠠⠁⠀⠔⠈⠇⠇⠠⠃")?;
    return Ok(());
}

// Pages 69-72, 76: a unit is preceded by the "znak miana" (dots 1-2-4-5-6) and
// then spelled with plain letter cells, with no letter sign. Verified across all
// 35 unit symbols in the guide's SI tables, so it is a rule, not a table.
// Units must be MARKED UP as such: the braille path never runs intent inference
// (see the rule comment), so class='MathML-unit' is what identifies them - the
// same signal the UEB rules use.
#[test]
fn jednostka_metr() -> Result<()> {
    let expr = r#"<math><mn>5</mn><mo>&#x2062;</mo><mi class="MathML-unit">m</mi></math>"#;
    test_braille("Polish", expr, "⠼⠑⠻⠍")?;
    return Ok(());
}

// A capital unit symbol keeps its capital sign after the unit prefix: N is the
// unit sign, then the capital sign, then "n".
#[test]
fn jednostka_niuton() -> Result<()> {
    let expr = r#"<math><mn>10</mn><mo>&#x2062;</mo><mi class="MathML-unit">N</mi></math>"#;
    test_braille("Polish", expr, "⠼⠁⠚⠻⠨⠝")?;
    return Ok(());
}

// Pages 77-79: a chemical element symbol is preceded by the element key sign and
// the two-letter symbols form ONE unit - the guide states explicitly that no
// letter sign goes before the second letter. So Cl is the key sign, c, l.
// The expression must first be RECOGNISED as chemistry: the engine scores each
// candidate (likely_chem_formula in src/chemistry.rs) and short fragments such as
// a lone "Cl" or "Cl2" stay below the threshold on purpose, since they could be
// ordinary variables. Measured: in "HCl" the element attribute IS set and the
// rule fires; in "Cl2" alone it is not. Hence a full molecule here, as in the
// Nemeth chemistry tests.
#[test]
fn pierwiastek_dwuliterowy() -> Result<()> {
    let expr = r#"<math><mi>Na</mi><mo>&#x2062;</mo><mi>Cl</mi></math>"#;
    test_braille("Polish", expr, "⠨⠝⠁⠨⠉⠇")?;
    return Ok(());
}

// A molecule repeats the key sign for every element (p. 77): HCl is the key sign
// with h, then the key sign again with c and l.
#[test]
fn czasteczka_hcl() -> Result<()> {
    let expr = r#"<math><mi>H</mi><mo>&#x2062;</mo><mi>Cl</mi></math>"#;
    test_braille("Polish", expr, "⠨⠓⠨⠉⠇")?;
    return Ok(());
}

// Page 59, conflict rule 1: two signs are written with NO blank between them when
// the second one is from group C. The prime is group C ("znaki pisane bez odstepu
// z lewej strony", p. 58), so a' has no gap before the prime.
#[test]
fn prim_bez_odstepu() -> Result<()> {
    let expr = r#"<math><msup><mi>a</mi><mo>&#x2032;</mo></msup></math>"#;
    test_braille("Polish", expr, "⠠⠁⠔")?;
    return Ok(());
}

// Page 10: the ASCII hyphen-minus U+002D shares the cell with the proper minus
// U+2212. Without an entry it used to reach the output as a raw ASCII byte.
#[test]
fn minus_ascii_i_unicode() -> Result<()> {
    let expr = r#"<math><mi>a</mi><mo>=</mo><mo>-</mo><mi>b</mi></math>"#;
    test_braille("Polish", expr, "⠠⠁⠀⠶⠤⠠⠃")?;
    return Ok(());
}
