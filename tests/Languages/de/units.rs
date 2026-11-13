/// Tests for rules shared between various speech styles:
/// *  modified var
use crate::common::*;
use anyhow::Result;

// The basic layout of the tests is:
// 1. Sweep through all the SI prefixes
// 2. Sweep through each group of SI units
//    a) with both singular and plural without prefixes
//    b) with both singular and plural with one prefix
// 3. Sweep through each group of units that don't take SI prefixes
// These are broken into chunks so it is easier to see errors, when there are errors

#[test]
fn prefix_sweep() -> Result<()> {
    let expr = r#"<math>
        <mi intent=":unit">Qg</mi><mo>,</mo>
        <mi intent=":unit">Rg</mi><mo>,</mo>
        <mi intent=":unit">Yg</mi><mo>,</mo>
        <mi intent=":unit">Zg</mi><mo>,</mo>
        <mi intent=":unit">Eg</mi><mo>,</mo>
        <mi intent=":unit">Pg</mi><mo>,</mo>
        <mi intent=":unit">Tg</mi><mo>,</mo>
        <mi intent=":unit">Gg</mi><mo>,</mo>
        <mi intent=":unit">Mg</mi><mo>,</mo>
        <mi intent=":unit">kg</mi><mo>,</mo>
        <mi intent=":unit">hg</mi><mo>,</mo>
        <mi intent=":unit">dag</mi><mo>,</mo>
        <mi intent=":unit">dg</mi><mo>,</mo>
        <mi intent=":unit">cg</mi><mo>,</mo>
        <mi intent=":unit">mg</mi><mo>,</mo>
        <mi intent=":unit">µg</mi><mo>,</mo>
        <mi intent=":unit">ng</mi><mo>,</mo>
        <mi intent=":unit">pg</mi><mo>,</mo>
        <mi intent=":unit">fg</mi><mo>,</mo>
        <mi intent=":unit">ag</mi><mo>,</mo>
        <mi intent=":unit">zg</mi><mo>,</mo>
        <mi intent=":unit">yg</mi><mo>,</mo>
        <mi intent=":unit">rg</mi><mo>,</mo>
        <mi intent=":unit">qg</mi>
        </math>"#;
    test("de", "SimpleSpeak", expr,
        "quetta-gramm, komma ronna-gramm, komma yotta-gramm, komma zetta-gramm, komma exa-gramm, komma peta-gramm, komma tera-gramm, komma giga-gramm, komma mega-gramm, komma kilo-gramm, komma hekto-gramm, komma deka-gramm, komma dezi-gramm, komma zenti-gramm, komma milli-gramm, komma mikro-gramm, komma nano-gramm, komma piko-gramm, komma femto-gramm, komma atto-gramm, komma zepto-gramm, komma yocto-gramm, komma ronto-gramm, komma quecto-gramm")?;
        return Ok(());

}

#[test]
fn si_base() -> Result<()> {
    let expr = r#"<math>
        <mn>1</mn><mi intent=":unit">A</mi><mo>,</mo><mn>2</mn><mi intent=":unit">A</mi><mo>,</mo>
        <mn>1</mn><mi intent=":unit">cd</mi><mo>,</mo><mn>2</mn><mi intent=":unit">cd</mi><mo>,</mo>
        <mn>1</mn><mi intent=":unit">K</mi><mo>,</mo><mn>2</mn><mi intent=":unit">K</mi><mo>,</mo>
        <mn>1</mn><mi intent=":unit">K</mi><mo>,</mo><mn>2</mn><mi intent=":unit">K</mi><mo>,</mo>
        <mn>1</mn><mi intent=":unit">g</mi><mo>,</mo><mn>2</mn><mi intent=":unit">g</mi><mo>,</mo>
        <mn>1</mn><mi intent=":unit">m</mi><mo>,</mo><mn>2</mn><mi intent=":unit">m</mi><mo>,</mo>
        <mn>1</mn><mi intent=":unit">mol</mi><mo>,</mo><mn>2</mn><mi intent=":unit">mol</mi><mo>,</mo>
        <mn>1</mn><mi intent=":unit">s</mi><mo>,</mo><mn>2</mn><mi intent=":unit">s</mi><mo>,</mo>
        <mn>1</mn><mi intent=":unit">″</mi><mo>,</mo><mn>2</mn><mi intent=":unit">″</mi><mo>,</mo>
        <mn>1</mn><mi intent=":unit">&quot;</mi><mo>,</mo><mn>2</mn><mi intent=":unit">&quot;</mi><mo>,</mo>
        <mn>1</mn><mi intent=":unit">sec</mi><mo>,</mo><mn>2</mn><mi intent=":unit">sec</mi>
    </math>"#;
    test("de", "SimpleSpeak", expr,
        "1 ampere, komma 2 ampere, komma 1 candela, komma 2 candela, komma 1 kelvin, komma 2 kelvin, komma 1 kelvin, komma 2 kelvin, komma 1 gramm komma 2 gramm, komma 1 meter komma 2 meter, komma 1 mol komma 2 mol, komma 1 sekunde, komma 2 sekunden, komma 1 sekunde, komma 2 sekunden, komma 1 sekunde, komma 2 sekunden, komma 1 sekunde, komma 2 sekunden")?;
        return Ok(());

}

#[test]
fn si_unit_plural_regression() -> Result<()> {
    let expr = r#"<math>
        <mn>2</mn><mi intent=":unit">kg</mi><mo>,</mo>
        <mn>2</mn><mi intent=":unit">°C</mi><mo>,</mo>
        <mn>2</mn><mi intent=":unit">s</mi>
    </math>"#;
    test("de", "ClearSpeak", expr,
        "2 kilo-gramm, komma 2 grad celsius, komma 2 sekunden")?;
    Ok(())
}

#[test]
fn english_rute_plural_regression() -> Result<()> {
    let expr = r#"<math>
        <mn>1</mn><mi intent=":unit">rd</mi><mo>,</mo>
        <mn>2</mn><mi intent=":unit">rd</mi>
    </math>"#;
    test("de", "SimpleSpeak", expr, "1 rute komma 2 ruten")?;
    Ok(())
}

#[test]
fn non_si_unit_plural_regression() -> Result<()> {
    let expr = r#"<math>
        <mn>1</mn><mi intent=":unit">h</mi><mo>,</mo><mn>2</mn><mi intent=":unit">h</mi><mo>,</mo>
        <mn>1</mn><mi intent=":unit">d</mi><mo>,</mo><mn>2</mn><mi intent=":unit">d</mi><mo>,</mo>
        <mn>1</mn><mi intent=":unit">w</mi><mo>,</mo><mn>2</mn><mi intent=":unit">w</mi><mo>,</mo>
        <mn>1</mn><mi intent=":unit">pc</mi><mo>,</mo><mn>2</mn><mi intent=":unit">pc</mi><mo>,</mo>
        <mn>1</mn><mi intent=":unit">ha</mi><mo>,</mo><mn>2</mn><mi intent=":unit">ha</mi><mo>,</mo>
        <mn>1</mn><mi intent=":unit">Kib</mi><mo>,</mo><mn>2</mn><mi intent=":unit">Kib</mi>
    </math>"#;
    test("de", "SimpleSpeak", expr,
        "1 stunde, komma 2 stunden, komma 1 tag komma 2 tage, komma 1 woche komma 2 wochen, komma 1 parsec, komma 2 parsec, komma 1 hektar, komma 2 hektar, komma 1 kibi-bit, komma 2 kibi-bit")?;
    Ok(())
}

#[test]
fn currency_symbol_precedes_amount() -> Result<()> {
    // German speaks a prefixed currency symbol after its amount.
    let expr = r#"<math><mi>$</mi><mn>3</mn></math>"#;
    test("de", "ClearSpeak", expr, "3 dollar")?;
    test("de", "SimpleSpeak", expr, "3 dollar")?;

    let expr = r#"<math><mi>€</mi><mn>1</mn></math>"#;
    test("de", "ClearSpeak", expr, "1 euro")?;
    test("de", "SimpleSpeak", expr, "1 euro")?;

    let expr = r#"<math><mi>£</mi><mn>4,7</mn></math>"#;
    test("de", "ClearSpeak", expr, "4,7 pfund")?;
    test("de", "SimpleSpeak", expr, "4,7 pfund")?;
    Ok(())
}

#[test]
fn rpm_uses_singular_and_plural_forms() -> Result<()> {
    // The unit name is singular only for a coefficient of exactly one.
    test(
        "de",
        "SimpleSpeak",
        r#"<math><mn>1</mn><mi intent=":unit">rpm</mi></math>"#,
        "1 Umdrehung pro Minute",
    )?;
    test(
        "de",
        "SimpleSpeak",
        r#"<math><mn>2</mn><mi intent=":unit">rpm</mi></math>"#,
        "2 Umdrehungen pro Minute",
    )?;
    Ok(())
}
