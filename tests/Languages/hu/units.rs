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

// AI generated
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
    test("hu", "SimpleSpeak", expr, 
        "quetta-gram, vessző, ronna-gram, vessző, yotta-gram, vessző, zetta-gram, vessző, exa-gram, vessző, peta-gram, vessző, tera-gram, vessző, giga-gram, vessző, mega-gram, vessző, kiló-gram, vessző, hektó-gram, vessző, deka-gram, vessző, deci-gram, vessző, centi-gram, vessző, milli-gram, vessző, mikro-gram, vessző, nano-gram, vessző, pikó-gram, vessző, femto-gram, vessző, atto-gram, vessző, zepto-gram, vessző, yokto-gram, vessző, ronto-gram, vessző, quecto-gram")?;
                return Ok(());

}

// AI generated
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
    test("hu", "SimpleSpeak", expr, 
        "1 amper, vessző; 2 amper, vessző, 1 kandela, vessző; 2 kandela, vessző, 1 kelvin, vessző; 2 kelvin, vessző, 1 kelvin, vessző; 2 kelvin, vessző, 1 gram, vessző; 2 gram, vessző, 1 méter, vessző; 2 méter, vessző, 1 mol, vessző, 2 mol, vessző; 1 másodperc, vessző; 2 másodperc, vessző; 1 másodperc, vessző; 2 másodperc, vessző; 1 másodperc, vessző; 2 másodperc, vessző; 1 másodperc, vessző; 2 másodperc")?;
                return Ok(());

}

// AI generated
#[test]
fn si_base_with_prefixes() -> Result<()> {
    let expr = r#"<math>
        <mn>1</mn><mi intent=":unit">QA</mi><mo>,</mo><mn>2</mn><mi intent=":unit">RA</mi><mo>,</mo>
        <mn>1</mn><mi intent=":unit">Ycd</mi><mo>,</mo><mn>2</mn><mi intent=":unit">Zcd</mi><mo>,</mo>
        <mn>1</mn><mi intent=":unit">EK</mi><mo>,</mo><mn>2</mn><mi intent=":unit">PK</mi><mo>,</mo>
        <mn>1</mn><mi intent=":unit">TK</mi><mo>,</mo><mn>2</mn><mi intent=":unit">GK</mi><mo>,</mo>
        <mn>1</mn><mi intent=":unit">Mg</mi><mo>,</mo><mn>2</mn><mi intent=":unit">kg</mi><mo>,</mo>
        <mn>1</mn><mi intent=":unit">hm</mi><mo>,</mo><mn>2</mn><mi intent=":unit">dam</mi><mo>,</mo>
        <mn>1</mn><mi intent=":unit">dmol</mi><mo>,</mo><mn>2</mn><mi intent=":unit">cmol</mi><mo>,</mo>
        <mn>1</mn><mi intent=":unit">ms</mi><mo>,</mo><mn>2</mn><mi intent=":unit">µs</mi><mo>,</mo>
        <mn>1</mn><mi intent=":unit">nsec</mi><mo>,</mo><mn>2</mn><mi intent=":unit">psec</mi>
    </math>"#;
    test("hu", "SimpleSpeak", expr, 
        "1 quetta-amper, vessző; 2 ronna-amper, vessző; 1 yotta-kandela, vessző; 2 zetta-kandela, vessző; 1 exa-kelvin, vessző; 2 peta-kelvin, vessző; 1 tera-kelvin, vessző; 2 giga-kelvin, vessző; 1 mega-gram, vessző; 2 kiló-gram, vessző; 1 hektó-méter, vessző; 2 deka-méter, vessző; 1 deci-mol, vessző; 2 centi-mol, vessző; 1 milli-másodperc, vessző; 2 mikro-másodperc; vessző; 1 nano-másodperc, vessző; 2 pikó-másodperc")?;
                return Ok(());

}


// AI generated
#[test]
fn si_derived_1() -> Result<()> {
    let expr = r#"<math>
        <mn>1</mn><mi intent=":unit">Bq</mi><mo>,</mo><mn>2</mn><mi intent=":unit">Bq</mi><mo>,</mo>
        <mn>1</mn><mi intent=":unit">C</mi><mo>,</mo><mn>2</mn><mi intent=":unit">C</mi><mo>,</mo>
        <mn>1</mn><mi intent=":unit">°C</mi><mo>,</mo><mn>2</mn><mi intent=":unit">°C</mi><mo>,</mo>
        <mn>1</mn><mi intent=":unit">℃</mi><mo>,</mo><mn>2</mn><mi intent=":unit">℃</mi><mo>,</mo>
        <mn>1</mn><mi intent=":unit">F</mi><mo>,</mo><mn>2</mn><mi intent=":unit">F</mi><mo>,</mo>
        <mn>1</mn><mi intent=":unit">Gy</mi><mo>,</mo><mn>2</mn><mi intent=":unit">Gy</mi><mo>,</mo>
        <mn>1</mn><mi intent=":unit">H</mi><mo>,</mo><mn>2</mn><mi intent=":unit">H</mi><mo>,</mo>
        <mn>1</mn><mi intent=":unit">Hz</mi><mo>,</mo><mn>2</mn><mi intent=":unit">Hz</mi><mo>,</mo>
        <mn>1</mn><mi intent=":unit">J</mi><mo>,</mo><mn>2</mn><mi intent=":unit">J</mi><mo>,</mo>
        <mn>1</mn><mi intent=":unit">kat</mi><mo>,</mo><mn>2</mn><mi intent=":unit">kat</mi><mo>,</mo>
        <mn>1</mn><mi intent=":unit">lm</mi><mo>,</mo><mn>2</mn><mi intent=":unit">lm</mi><mo>,</mo>
        <mn>1</mn><mi intent=":unit">lx</mi><mo>,</mo><mn>2</mn><mi intent=":unit">lx</mi>
    </math>"#;
    test("hu", "SimpleSpeak", expr, 
        "1 becquerel, vessző; 2 becquerel, vessző, 1 Coulomb, vessző; 2 Coulomb, vessző; 1 Celsius fok, vessző; 2 Celsius fok, vessző; 1 Celsius fok, vessző; 2 Celsius fok, vessző, 1 farád, vessző; 2 farád, vessző, 1 gray, vessző; 2 gray, vessző, 1 henry, vessző, 2 henri, vessző, 1 hertz, vessző, 2 hertz, vessző, 1 joule, vessző; 2 joule, vessző, 1 katal, vessző; 2 katal, vessző, 1 lumen, vessző; 2 lumen, vessző, 1 lux, vessző, 2 lux")?;
                return Ok(());

}

// AI generated
#[test]
fn si_derived_1_with_prefixes() -> Result<()> {
    let expr = r#"<math>
        <mn>1</mn><mi intent=":unit">QBq</mi><mo>,</mo><mn>2</mn><mi intent=":unit">RBq</mi><mo>,</mo>
        <mn>1</mn><mi intent=":unit">YC</mi><mo>,</mo><mn>2</mn><mi intent=":unit">ZC</mi><mo>,</mo>
        <mn>1</mn><mi intent=":unit">EF</mi><mo>,</mo><mn>2</mn><mi intent=":unit">PF</mi><mo>,</mo>
        <mn>1</mn><mi intent=":unit">TGy</mi><mo>,</mo><mn>2</mn><mi intent=":unit">GGy</mi><mo>,</mo>
        <mn>1</mn><mi intent=":unit">MH</mi><mo>,</mo><mn>2</mn><mi intent=":unit">kH</mi><mo>,</mo>
        <mn>1</mn><mi intent=":unit">daHz</mi><mo>,</mo><mn>2</mn><mi intent=":unit">dHz</mi><mo>,</mo>
        <mn>1</mn><mi intent=":unit">cJ</mi><mo>,</mo><mn>2</mn><mi intent=":unit">mJ</mi><mo>,</mo>
        <mn>1</mn><mi intent=":unit">µkat</mi><mo>,</mo><mn>2</mn><mi intent=":unit">nkat</mi><mo>,</mo>
        <mn>1</mn><mi intent=":unit">plm</mi><mo>,</mo><mn>2</mn><mi intent=":unit">flm</mi><mo>,</mo>
        <mn>1</mn><mi intent=":unit">alx</mi><mo>,</mo><mn>2</mn><mi intent=":unit">zlx</mi><mo>,</mo>
        <mn>1</mn><mi intent=":unit">m°C</mi><mo>,</mo><mn>2</mn><mi intent=":unit">µ°C</mi><mo>,</mo>
        <mn>1</mn><mi intent=":unit">p℃</mi><mo>,</mo><mn>2</mn><mi intent=":unit">n℃</mi>
    </math>"#;
    test("hu", "SimpleSpeak", expr, 
        "1 quetta-becquerel, vessző; 2 ronna-becquerel; vessző; 1 yotta-Coulomb, vessző; 2 zetta-Coulomb, vessző; 1 exa-farád, vessző; 2 peta-farád, vessző; 1 tera-gray, vessző; 2 giga-gray, vessző; 1 mega-henry, vessző; 2 kiló-henri, vessző; 1 deka-hertz, vessző; 2 deci-hertz, vessző; 1 centi-joule, vessző; 2 milli-joule, vessző; 1 mikro-katal, vessző; 2 nano-katal, vessző; 1 pikó-lumen, vessző; 2 femto-lumen, vessző; 1 atto-lux, vessző; 2 zepto-lux, vessző; 1 milli-Celsius fok, vessző; 2 mikro-Celsius fok; vessző; 1 pikó-Celsius fok, vessző; 2 nano-Celsius fok")?;
                return Ok(());

}

// AI generated
#[test]
fn si_derived_2() -> Result<()> {
    let expr = r#"<math>
        <mn>1</mn><mi intent=":unit">N</mi><mo>,</mo><mn>2</mn><mi intent=":unit">N</mi><mo>,</mo>
        <mn>1</mn><mi intent=":unit">Ω</mi><mo>,</mo><mn>2</mn><mi intent=":unit">Ω</mi><mo>,</mo>
        <mn>1</mn><mi intent=":unit">Ω</mi><mo>,</mo><mn>2</mn><mi intent=":unit">Ω</mi><mo>,</mo>
        <mn>1</mn><mi intent=":unit">Pa</mi><mo>,</mo><mn>2</mn><mi intent=":unit">Pa</mi><mo>,</mo>
        <mn>1</mn><mi intent=":unit">S</mi><mo>,</mo><mn>2</mn><mi intent=":unit">S</mi><mo>,</mo>
        <mn>1</mn><mi intent=":unit">Sv</mi><mo>,</mo><mn>2</mn><mi intent=":unit">Sv</mi><mo>,</mo>
        <mn>1</mn><mi intent=":unit">T</mi><mo>,</mo><mn>2</mn><mi intent=":unit">T</mi><mo>,</mo>
        <mn>1</mn><mi intent=":unit">V</mi><mo>,</mo><mn>2</mn><mi intent=":unit">V</mi><mo>,</mo>
        <mn>1</mn><mi intent=":unit">W</mi><mo>,</mo><mn>2</mn><mi intent=":unit">W</mi><mo>,</mo>
        <mn>1</mn><mi intent=":unit">Wb</mi><mo>,</mo><mn>2</mn><mi intent=":unit">Wb</mi>
    </math>"#;
    test("hu", "SimpleSpeak", expr, 
        "1 newton, vessző; 2 newton, vessző, 1 ohm, vessző, 2 ohm, vessző, 1 ohm, vessző, 2 ohm, vessző, 1 pascal, vessző; 2 pascal, vessző, 1 siemens, vessző, 2 siemens, vessző, 1 sievert, vessző; 2 sievert, vessző, 1 tesla, vessző; 2 tesla, vessző, 1 volt, vessző; 2 volt, vessző, 1 watt, vessző; 2 watt, vessző, 1 weber, vessző; 2 weber")?;
                return Ok(());

}

// AI generated
#[test]
fn si_derived_2_with_prefixes() -> Result<()> {
    let expr = r#"<math>
        <mn>1</mn><mi intent=":unit">qN</mi><mo>,</mo><mn>2</mn><mi intent=":unit">rN</mi><mo>,</mo>
        <mn>1</mn><mi intent=":unit">yΩ</mi><mo>,</mo><mn>2</mn><mi intent=":unit">zΩ</mi><mo>,</mo>
        <mn>1</mn><mi intent=":unit">aΩ</mi><mo>,</mo><mn>2</mn><mi intent=":unit">fΩ</mi><mo>,</mo>
        <mn>1</mn><mi intent=":unit">pPa</mi><mo>,</mo><mn>2</mn><mi intent=":unit">nPa</mi><mo>,</mo>
        <mn>1</mn><mi intent=":unit">µS</mi><mo>,</mo><mn>2</mn><mi intent=":unit">mS</mi><mo>,</mo>
        <mn>1</mn><mi intent=":unit">cSv</mi><mo>,</mo><mn>2</mn><mi intent=":unit">dSv</mi><mo>,</mo>
        <mn>1</mn><mi intent=":unit">daT</mi><mo>,</mo><mn>2</mn><mi intent=":unit">hT</mi><mo>,</mo>
        <mn>1</mn><mi intent=":unit">kV</mi><mo>,</mo><mn>2</mn><mi intent=":unit">MV</mi><mo>,</mo>
        <mn>1</mn><mi intent=":unit">GW</mi><mo>,</mo><mn>2</mn><mi intent=":unit">TW</mi><mo>,</mo>
        <mn>1</mn><mi intent=":unit">PWb</mi><mo>,</mo><mn>2</mn><mi intent=":unit">EWb</mi>
    </math>"#;
    test("hu", "SimpleSpeak", expr, 
        "1 quecto-newton, vessző; 2 ronto-newton, vessző; 1 yokto-ohm, vessző; 2 zepto-ohm, vessző; 1 atto-ohm, vessző; 2 femto-ohm, vessző; 1 pikó-pascal, vessző; 2 nano-pascal, vessző; 1 mikro-siemens, vessző; 2 milli-siemens, vessző; 1 centi-sievert, vessző; 2 deci-sievert, vessző; 1 deka-tesla, vessző; 2 hektó-tesla, vessző; 1 kiló-volt, vessző; 2 mega-volt, vessző; 1 giga-watt, vessző; 2 tera-watt, vessző; 1 peta-weber, vessző; 2 exa-weber")?;
                return Ok(());

}


// AI generated
#[test]
fn si_accepted() -> Result<()> {
    let expr = r#"<math>
        <mn>1</mn><mi intent=":unit">l</mi><mo>,</mo><mn>2</mn><mi intent=":unit">l</mi><mo>,</mo>
        <mn>1</mn><mi intent=":unit">L</mi><mo>,</mo><mn>2</mn><mi intent=":unit">L</mi><mo>,</mo>
        <mn>1</mn><mi intent=":unit">ℓ</mi><mo>,</mo><mn>2</mn><mi intent=":unit">ℓ</mi><mo>,</mo>
        <mn>1</mn><mi intent=":unit">t</mi><mo>,</mo><mn>2</mn><mi intent=":unit">t</mi><mo>,</mo>
        <mn>1</mn><mi intent=":unit">Da</mi><mo>,</mo><mn>2</mn><mi intent=":unit">Da</mi><mo>,</mo>
        <mn>1</mn><mi intent=":unit">Np</mi><mo>,</mo><mn>2</mn><mi intent=":unit">Np</mi><mo>,</mo>
        <mn>1</mn><mi intent=":unit">u</mi><mo>,</mo><mn>2</mn><mi intent=":unit">u</mi><mo>,</mo> 
        <mn>1</mn><mi intent=":unit">eV</mi><mo>,</mo><mn>2</mn><mi intent=":unit">eV</mi><mo>,</mo>
        <mn>1</mn><mi intent=":unit">rad</mi><mo>,</mo><mn>2</mn><mi intent=":unit">rad</mi><mo>,</mo>
        <mn>1</mn><mi intent=":unit">sr</mi><mo>,</mo><mn>2</mn><mi intent=":unit">sr</mi><mo>,</mo>
        <mn>1</mn><mi intent=":unit">a</mi><mo>,</mo><mn>2</mn><mi intent=":unit">a</mi><mo>,</mo>
        <mn>1</mn><mi intent=":unit">as</mi><mo>,</mo><mn>2</mn><mi intent=":unit">as</mi><mo>,</mo>
        <mn>1</mn><mi intent=":unit">b</mi><mo>,</mo><mn>2</mn><mi intent=":unit">b</mi><mo>,</mo>
        <mn>1</mn><mi intent=":unit">B</mi><mo>,</mo><mn>2</mn><mi intent=":unit">B</mi><mo>,</mo>
        <mn>1</mn><mi intent=":unit">Bd</mi><mo>,</mo><mn>2</mn><mi intent=":unit">Bd</mi>
    </math>"#;
    test("hu", "SimpleSpeak", expr, 
        "1 liter, vessző; 2 liter, vessző, 1 liter, vessző; 2 liter, vessző, 1 liter, vessző; 2 liter, vessző; 1 metrikus tonna, vessző; 2 metrikus tonna, vessző, 1 dalton, vessző; 2 dalton, vessző, 1 neper, vessző; 2 neper, vessző; 1 atomtömegegység, vessző; 2 atomtömegegység, vessző; 1 elektronvolt, vessző; 2 elektronvolt, vessző, 1 radián, vessző; 2 radián, vessző; 1 szteradián, vessző; 2 szteradián, vessző, 1 év, vessző, 2 év, vessző; 1 ívmásodperc, vessző; 2 ívmásodperc, vessző, 1 bit, vessző, 2 bit, vessző, 1 byte, vessző; 2 byte, vessző, 1 baud, vessző; 2 baud")?;
                return Ok(());

}

// AI generated
#[test]
fn si_accepted_with_prefixes() -> Result<()> {
    let expr = r#"<math>
        <mn>1</mn><mi intent=":unit">Ql</mi><mo>,</mo><mn>2</mn><mi intent=":unit">Rl</mi><mo>,</mo>
        <mn>1</mn><mi intent=":unit">YL</mi><mo>,</mo><mn>2</mn><mi intent=":unit">ZL</mi><mo>,</mo>
        <mn>1</mn><mi intent=":unit">Eℓ</mi><mo>,</mo><mn>2</mn><mi intent=":unit">Pℓ</mi><mo>,</mo>
        <mn>1</mn><mi intent=":unit">Tt</mi><mo>,</mo><mn>2</mn><mi intent=":unit">Gt</mi><mo>,</mo>
        <mn>1</mn><mi intent=":unit">MDa</mi><mo>,</mo><mn>2</mn><mi intent=":unit">kDa</mi><mo>,</mo>
        <mn>1</mn><mi intent=":unit">dNp</mi><mo>,</mo><mn>2</mn><mi intent=":unit">cNp</mi><mo>,</mo>
        <mn>1</mn><mi intent=":unit">hu</mi><mo>,</mo><mn>2</mn><mi intent=":unit">dau</mi><mo>,</mo> 
        <mn>1</mn><mi intent=":unit">meV</mi><mo>,</mo><mn>2</mn><mi intent=":unit">µeV</mi><mo>,</mo>
        <mn>1</mn><mi intent=":unit">nrad</mi><mo>,</mo><mn>2</mn><mi intent=":unit">prad</mi><mo>,</mo>
        <mn>1</mn><mi intent=":unit">fsr</mi><mo>,</mo><mn>2</mn><mi intent=":unit">asr</mi><mo>,</mo>
        <mn>1</mn><mi intent=":unit">Ga</mi><mo>,</mo><mn>2</mn><mi intent=":unit">Ma</mi><mo>,</mo>
        <mn>1</mn><mi intent=":unit">zas</mi><mo>,</mo><mn>2</mn><mi intent=":unit">yas</mi><mo>,</mo>
        <mn>1</mn><mi intent=":unit">kb</mi><mo>,</mo><mn>2</mn><mi intent=":unit">Mb</mi><mo>,</mo>
        <mn>1</mn><mi intent=":unit">GB</mi><mo>,</mo><mn>2</mn><mi intent=":unit">TB</mi><mo>,</mo>
        <mn>1</mn><mi intent=":unit">TBd</mi><mo>,</mo><mn>2</mn><mi intent=":unit">EBd</mi>
    </math>"#;
    test("hu", "SimpleSpeak", expr, 
        "1 quetta-liter, vessző; 2 ronna-liter, vessző; 1 yotta-liter, vessző; 2 zetta-liter, vessző; 1 exa-liter, vessző; 2 peta-liter, vessző; 1 tera-metrikus tonna; vessző; 2 giga-metrikus tonna; vessző; 1 mega-dalton, vessző; 2 kiló-dalton, vessző; 1 deci-neper, vessző; 2 centi-neper, vessző; 1 hektó-atomtömegegység; vessző; 2 deka-atomtömegegység; vessző; 1 milli-elektronvolt; vessző; 2 mikro-elektronvolt; vessző; 1 nano-radián, vessző; 2 pikó-radián, vessző; 1 femto-szteradián, vessző; 2 atto-szteradián; vessző; 1 giga-év, vessző; 2 mega-év, vessző; 1 zepto-ívmásodperc; vessző; 2 yokto-ívmásodperc; vessző; 1 kiló-bit, vessző; 2 mega-bit, vessző; 1 giga-byte, vessző; 2 tera-byte, vessző; 1 tera-baud, vessző; 2 exa-baud")?;
                return Ok(());

}

// AI generated
#[test]
fn without_prefix_time() -> Result<()> {
    let expr = r#"<math>
        <mn>1</mn><mi intent=":unit">″</mi><mo>,</mo><mn>2</mn><mi intent=":unit">″</mi><mo>,</mo>
        <mn>1</mn><mi intent=":unit">&quot;</mi><mo>,</mo><mn>2</mn><mi intent=":unit">&quot;</mi><mo>,</mo>
        <mn>1</mn><mi intent=":unit">′</mi><mo>,</mo><mn>2</mn><mi intent=":unit">′</mi><mo>,</mo>
        <mn>1</mn><mi intent=":unit">'</mi><mo>,</mo><mn>2</mn><mi intent=":unit">'</mi><mo>,</mo>
        <mn>1</mn><mi intent=":unit">min</mi><mo>,</mo><mn>2</mn><mi intent=":unit">min</mi><mo>,</mo>
        <mn>1</mn><mi intent=":unit">h</mi><mo>,</mo><mn>2</mn><mi intent=":unit">h</mi><mo>,</mo>
        <mn>1</mn><mi intent=":unit">hr</mi><mo>,</mo><mn>2</mn><mi intent=":unit">hr</mi><mo>,</mo> 
        <mn>1</mn><mi intent=":unit">Hr</mi><mo>,</mo><mn>2</mn><mi intent=":unit">Hr</mi><mo>,</mo>
        <mn>1</mn><mi intent=":unit">d</mi><mo>,</mo><mn>2</mn><mi intent=":unit">d</mi><mo>,</mo>
        <mn>1</mn><mi intent=":unit">dy</mi><mo>,</mo><mn>2</mn><mi intent=":unit">dy</mi><mo>,</mo>
        <mn>1</mn><mi intent=":unit">w</mi><mo>,</mo><mn>2</mn><mi intent=":unit">w</mi><mo>,</mo>
        <mn>1</mn><mi intent=":unit">wk</mi><mo>,</mo><mn>2</mn><mi intent=":unit">wk</mi><mo>,</mo>
        <mn>1</mn><mi intent=":unit">y</mi><mo>,</mo><mn>2</mn><mi intent=":unit">y</mi><mo>,</mo>
        <mn>1</mn><mi intent=":unit">yr</mi><mo>,</mo><mn>2</mn><mi intent=":unit">yr</mi>
    </math>"#;
    test("hu", "SimpleSpeak", expr, 
        "1 másodperc, vessző; 2 másodperc, vessző; 1 másodperc, vessző; 2 másodperc, vessző, 1 perc, vessző; 2 perc, vessző, 1 perc, vessző; 2 perc, vessző, 1 perc, vessző; 2 perc, vessző, 1 óra, vessző; 2 óra, vessző, 1 óra, vessző; 2 óra, vessző, 1 óra, vessző; 2 óra, vessző, 1 nap, vessző, 2 nap, vessző, 1 nap, vessző, 2 nap, vessző, 1 hét, vessző; 2 hét, vessző, 1 hét, vessző; 2 hét, vessző, 1 év, vessző, 2 év, vessző, 1 év, vessző, 2 év")?;
                return Ok(());

}

// AI generated
#[test]
fn without_prefix_angles() -> Result<()> {
    let expr = r#"<math>
        <mn>1</mn><mi intent=":unit">°</mi><mo>,</mo><mn>2</mn><mi intent=":unit">°</mi><mo>,</mo>
        <mn>1</mn><mi intent=":unit">deg</mi><mo>,</mo><mn>2</mn><mi intent=":unit">deg</mi><mo>,</mo>
        <mn>1</mn><mi intent=":unit">arcmin</mi><mo>,</mo><mn>2</mn><mi intent=":unit">arcmin</mi><mo>,</mo>
        <mn>1</mn><mi intent=":unit">amin</mi><mo>,</mo><mn>2</mn><mi intent=":unit">amin</mi><mo>,</mo>
        <mn>1</mn><mi intent=":unit">am</mi><mo>,</mo><mn>2</mn><mi intent=":unit">am</mi><mo>,</mo>
        <mn>1</mn><mi intent=":unit">MOA</mi><mo>,</mo><mn>2</mn><mi intent=":unit">MOA</mi><mo>,</mo>
        <mn>1</mn><mi intent=":unit">arcsec</mi><mo>,</mo><mn>2</mn><mi intent=":unit">arcsec</mi><mo>,</mo> 
        <mn>1</mn><mi intent=":unit">asec</mi><mo>,</mo><mn>2</mn><mi intent=":unit">asec</mi>
    </math>"#;
    test("hu", "SimpleSpeak", expr, 
        "1 fok, vessző, 2 fok, vessző, 1 fok, vessző, 2 fok, vessző, 1 ívperc, vessző; 2 ívperc, vessző, 1 ívperc, vessző; 2 ívperc, vessző, 1 ívperc, vessző; 2 ívperc, vessző, 1 ívperc, vessző; 2 ívperc, vessző; 1 ívmásodperc, vessző; 2 ívmásodperc, vessző; 1 ívmásodperc, vessző; 2 ívmásodperc")?;
                return Ok(());

}

// AI generated
#[test]
fn without_prefix_distance() -> Result<()> {
    let expr = r#"<math>
        <mn>1</mn><mi intent=":unit">au</mi><mo>,</mo><mn>2</mn><mi intent=":unit">au</mi><mo>,</mo>
        <mn>1</mn><mi intent=":unit">ltyr</mi><mo>,</mo><mn>2</mn><mi intent=":unit">ltyr</mi><mo>,</mo>
        <mn>1</mn><mi intent=":unit">pc</mi><mo>,</mo><mn>2</mn><mi intent=":unit">pc</mi><mo>,</mo>
        <mn>1</mn><mi intent=":unit">Å</mi><mo>,</mo><mn>2</mn><mi intent=":unit">Å</mi><mo>,</mo>
        <mn>1</mn><mi intent=":unit">Å</mi><mo>,</mo><mn>2</mn><mi intent=":unit">Å</mi><mo>,</mo>
        <mn>1</mn><mi intent=":unit">fm</mi><mo>,</mo><mn>2</mn><mi intent=":unit">fm</mi>
    </math>"#;
    test("hu", "SimpleSpeak", expr, 
        "1 csillagászati egység, vessző; 2 csillagászati egység; vessző; 1 fényév, vessző; 2 fényév, vessző, 1 parszek, vessző; 2 parszek, vessző; 1 angström, vessző; 2 angström, vessző; 1 angström, vessző; 2 angström, vessző, 1 fermi, vessző; 2 fermi")?;
                return Ok(());

}

// AI generated
#[test]
fn without_prefix_other() -> Result<()> {
    let expr = r#"<math>
        <mn>1</mn><mi intent=":unit">ha</mi><mo>,</mo><mn>2</mn><mi intent=":unit">ha</mi><mo>,</mo>
        <mn>1</mn><mi intent=":unit">dB</mi><mo>,</mo><mn>2</mn><mi intent=":unit">dB</mi><mo>,</mo>
        <mn>1</mn><mi intent=":unit">atm</mi><mo>,</mo><mn>2</mn><mi intent=":unit">atm</mi><mo>,</mo>
        <mn>1</mn><mi intent=":unit">amu</mi><mo>,</mo><mn>2</mn><mi intent=":unit">amu</mi><mo>,</mo>
        <mn>1</mn><mi intent=":unit">bar</mi><mo>,</mo><mn>2</mn><mi intent=":unit">bar</mi><mo>,</mo>
        <mn>1</mn><mi intent=":unit">cal</mi><mo>,</mo><mn>2</mn><mi intent=":unit">cal</mi><mo>,</mo>
        <mn>1</mn><mi intent=":unit">Ci</mi><mo>,</mo><mn>2</mn><mi intent=":unit">Ci</mi><mo>,</mo> 
        <mn>1</mn><mi intent=":unit">grad</mi><mo>,</mo><mn>2</mn><mi intent=":unit">grad</mi><mo>,</mo>
        <mn>1</mn><mi intent=":unit">M</mi><mo>,</mo><mn>2</mn><mi intent=":unit">M</mi><mo>,</mo>
        <mn>1</mn><mi intent=":unit">R</mi><mo>,</mo><mn>2</mn><mi intent=":unit">R</mi><mo>,</mo>
        <mn>1</mn><mi intent=":unit">rpm</mi><mo>,</mo><mn>2</mn><mi intent=":unit">rpm</mi><mo>,</mo>
        <mn>1</mn><mi intent=":unit">fl dr</mi><mo>,</mo><mn>2</mn><mi intent=":unit">fl dr</mi><mo>,</mo>
        <mn>1</mn><mi intent=":unit">℧</mi><mo>,</mo><mn>2</mn><mi intent=":unit">℧</mi><mo>,</mo>
        <mn>1</mn><mi intent=":unit">dyn</mi><mo>,</mo><mn>2</mn><mi intent=":unit">dyn</mi><mo>,</mo>
        <mn>1</mn><mi intent=":unit">erg</mi><mo>,</mo><mn>2</mn><mi intent=":unit">erg</mi>
    </math>"#;
    test("hu", "SimpleSpeak", expr, 
        "1 hektár, vessző; 2 hektár, vessző, 1 decibel, vessző; 2 decibel, vessző; 1 atmoszféra, vessző; 2 atmoszféra, vessző; 1 atomikus tömeg, vessző; 2 atomikus tömeg, vessző, 1 bár, vessző; 2 bár, vessző; 1 kalória, vessző; 2 kalória, vessző, 1 curie, vessző; 2 curie, vessző; 1 gradiens, vessző; 2 gradiens, vessző, 1 mól, vessző; 2 mól, vessző; 1 röntgen, vessző; 2 röntgen, vessző; 1 fordulat per perc, vessző; 2 fordulat per perc, vessző; 1 folyadékdróm, vessző; 2 folyadékdróm, vessző, 1 m-h-o, vessző; 2 m-h-o, vessző, 1 dyn, vessző, 2 dyn, vessző, 1 erg, vessző, 2 erg")?;
                return Ok(());

}

// AI generated
#[test]
fn without_prefix_powers_of_2() -> Result<()> {
    let expr = r#"<math>
        <mn>1</mn><mi intent=":unit">Kib</mi><mo>,</mo><mn>2</mn><mi intent=":unit">Kib</mi><mo>,</mo>
        <mn>1</mn><mi intent=":unit">Mib</mi><mo>,</mo><mn>2</mn><mi intent=":unit">Mib</mi><mo>,</mo>
        <mn>1</mn><mi intent=":unit">Gib</mi><mo>,</mo><mn>2</mn><mi intent=":unit">Gib</mi><mo>,</mo>
        <mn>1</mn><mi intent=":unit">Tib</mi><mo>,</mo><mn>2</mn><mi intent=":unit">Tib</mi><mo>,</mo>
        <mn>1</mn><mi intent=":unit">Pib</mi><mo>,</mo><mn>2</mn><mi intent=":unit">Pib</mi><mo>,</mo>
        <mn>1</mn><mi intent=":unit">Eib</mi><mo>,</mo><mn>2</mn><mi intent=":unit">Eib</mi><mo>,</mo>
        <mn>1</mn><mi intent=":unit">Zib</mi><mo>,</mo><mn>2</mn><mi intent=":unit">Zib</mi><mo>,</mo> 
        <mn>1</mn><mi intent=":unit">Yib</mi><mo>,</mo><mn>2</mn><mi intent=":unit">Yib</mi><mo>,</mo>
        <mn>1</mn><mi intent=":unit">KiB</mi><mo>,</mo><mn>2</mn><mi intent=":unit">KiB</mi><mo>,</mo>
        <mn>1</mn><mi intent=":unit">MiB</mi><mo>,</mo><mn>2</mn><mi intent=":unit">MiB</mi><mo>,</mo>
        <mn>1</mn><mi intent=":unit">GiB</mi><mo>,</mo><mn>2</mn><mi intent=":unit">GiB</mi><mo>,</mo>
        <mn>1</mn><mi intent=":unit">TiB</mi><mo>,</mo><mn>2</mn><mi intent=":unit">TiB</mi><mo>,</mo>
        <mn>1</mn><mi intent=":unit">PiB</mi><mo>,</mo><mn>2</mn><mi intent=":unit">PiB</mi><mo>,</mo>
        <mn>1</mn><mi intent=":unit">EiB</mi><mo>,</mo><mn>2</mn><mi intent=":unit">EiB</mi><mo>,</mo>
        <mn>1</mn><mi intent=":unit">ZiB</mi><mo>,</mo><mn>2</mn><mi intent=":unit">ZiB</mi><mo>,</mo> 
        <mn>1</mn><mi intent=":unit">YiB</mi><mo>,</mo><mn>2</mn><mi intent=":unit">YiB</mi>
    </math>"#;
    test("hu", "SimpleSpeak", expr, 
        "1 kibi-bit, vessző; 2 kibi-bit, vessző; 1 mebi-bit, vessző; 2 mebi-bit, vessző; 1 gibi-bit, vessző; 2 gibi-bit, vessző; 1 tebi-bit, vessző; 2 tebi-bit, vessző; 1 pebi-bit, vessző; 2 pebi-bit, vessző; 1 exbi-bit, vessző; 2 exbi-bit, vessző; 1 zebi-bit, vessző; 2 zebi-bit, vessző; 1 yobi-bit, vessző; 2 yobi-bit, vessző; 1 kibi-byte, vessző; 2 kibi-byte, vessző; 1 mebi-byte, vessző; 2 mebi-byte, vessző; 1 gibi-byte, vessző; 2 gibi-byte, vessző; 1 tebi-byte, vessző; 2 tebi-byte, vessző; 1 pebi-byte, vessző; 2 pebi-byte, vessző; 1 exbi-byte, vessző; 2 exbi-byte, vessző; 1 zebi-byte, vessző; 2 zebi-byte, vessző; 1 yobi-byte, vessző; 2 yobi-byte")?;
                return Ok(());

}


// AI generated
#[test]
fn si_other_numbers() -> Result<()> {
    let expr = r#"<math><mn>1.0</mn><mi intent=":unit">l</mi><mo>,</mo>
                            <mn>2.0</mn><mo>&#xA0;</mo><mi intent=":unit">m</mi><mo>,</mo>
                            <mi>x</mi><mo>&#xA0;</mo><mi intent=":unit">ms</mi><mo>,</mo>
                            <mi>y</mi><mi intent=":unit">µs</mi><mo>,</mo>
                            <mi intent=":unit">dag</mi><mo>,</mo>
                            <mn>1235</mn><mi intent=":unit">daN</mi><mo>,</mo>
                            <mn>2.5</mn><mi intent=":unit">&#xB5;sec</mi><mo>,</mo>
                            <mn>32.34</mn><mi intent=":unit">mol</mi></math>"#;
    test_prefs("hu", "SimpleSpeak", vec![("Verbosity", "Terse")], expr,
            "10 l vessző, 20 m vessző; x milli-másodperc; vessző; y mikro-másodperc; vessző, deka-gram, vessző; 1235 deka-newton; vessző; 25 mikro-másodperc; vessző; 3234 mol")?;
    test_prefs("hu", "ClearSpeak", vec![("Verbosity", "Medium")], expr,
            "10 liter, vessző; 20 méter, vessző; x milli-másodperc; vessző; y mikro-másodperc; vessző, deka-gram, vessző; 1235 deka-newton; vessző; 25 mikro-másodperc; vessző; 3234 mol")?;
    test_prefs("hu", "SimpleSpeak", vec![("Verbosity", "Verbose")], expr,
            "10 liter, vessző; 20 méter, vessző; x milli-másodperc; vessző; y mikro-másodperc; vessző, deka-gram, vessző; 1235 deka-newton; vessző; 25 mikro-másodperc; vessző; 3234 mol")?;
                    return Ok(());

}


// AI generated
#[test]
fn test_mtext_inference() -> Result<()> {
    let expr = r#"<math><mo>[</mo>
                <mn>1</mn><mtext>t</mtext><mo>,</mo>
                <mn>2</mn><mtext>PA</mtext><mo>,</mo>
                <mn>3</mn><mtext>Pa</mtext><mo>,</mo>
                <mn>4.5</mn><mtext>mT</mtext>
            <mo>]</mo></math>"#;
    test("hu", "SimpleSpeak", expr, 
        "nyitott zárójel; 1 metrikus tonna, vessző; 2 peta-amper, vessző; 3 pascal, vessző; 45 milli-tesla; zárójel")?;
                return Ok(());

}

    #[test]
    fn infer_unit() -> Result<()> {
        let expr = r#"<math>
            <mn>3</mn><mi mathvariant="normal">m</mi><mo>,</mo>
            <mn>1</mn><mi>km</mi><mo>,</mo>
            <mn>3</mn><mtext>m</mtext><mo>,</mo>
            <mfrac><mn>3</mn><mn>10</mn></mfrac><mi mathvariant="normal">F</mi><mo>,</mo>
            <msub><mi>m</mi><mi>min</mi></msub>
            </math>"#;
        test("hu", "SimpleSpeak", expr, 
            "3 méter, vessző; 1 kiló-méter, vessző; 3 méter, vessző; 3 tized farád, vessző; m alsó index min alsó index vége")?;
            return Ok(());

    }
