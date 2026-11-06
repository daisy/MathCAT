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
    test("es", "SimpleSpeak", expr, 
        "quettagramos coma, ronnagramos coma, yottagramos coma, zettagramos coma, exagramos coma, petagramos coma, teragramos coma, gigagramos coma, megagramos coma, kilogramos coma, hectogramos coma, decagramos coma, decigramos coma, centigramos coma, miligramos coma, microgramos coma, nanogramos coma, picogramos coma, femtogramos coma, attogramos coma, zeptogramos coma, yoctogramos coma, rontogramos coma, quectogramos")?;
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
    test("es", "SimpleSpeak", expr, 
        "1 amperio, coma, 2 amperios, coma, 1 candela, coma, 2 candelas, coma, 1 kelvin, coma, 2 kelvin, coma, 1 kelvin, coma, 2 kelvin, coma, 1 gramo, coma, 2 gramos, coma, 1 metro, coma, 2 metros, coma, 1 mol coma, 2 moles, coma, 1 segundo, coma, 2 segundos, coma, 1 segundo, coma, 2 segundos, coma, 1 segundo, coma, 2 segundos, coma, 1 segundo, coma, 2 segundos")?;
                return Ok(());

}

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
    test("es", "SimpleSpeak", expr, 
        "1 quettaamperio, coma; 2 ronnaamperios, coma; 1 yottacandela, coma; 2 zettacandelas, coma, 1 exakelvin, coma; 2 petakelvin, coma; 1 terakelvin, coma; 2 gigakelvin, coma, 1 megagramo, coma; 2 kilogramos, coma, 1 hectómetro, coma, 2 decámetros, coma, 1 decimol, coma; 2 centimoles, coma; 1 milisegundo, coma; 2 microsegundos, coma; 1 nanosegundo, coma; 2 picosegundos")?;
                return Ok(());

}


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
    test("es", "SimpleSpeak", expr, 
        "1 becquerel, coma, 2 becquereles, coma, 1 culombio, coma, 2 culombios, coma, 1 grado Celsius, coma; 2 grados Celsius, coma, 1 grado Celsius, coma; 2 grados Celsius, coma, 1 faradio, coma, 2 faradios, coma, 1 gray, coma, 2 grays, coma, 1 henrio, coma, 2 henrios, coma, 1 hercio, coma, 2 hercios, coma, 1 julio, coma, 2 julios, coma, 1 katal, coma, 2 katales, coma, 1 lumen, coma, 2 lúmenes, coma, 1 lux coma, 2 lux")?;
                return Ok(());

}

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
    test("es", "SimpleSpeak", expr, 
        "1 quettabecquerel, coma; 2 ronnabecquereles, coma; 1 yottaculombio, coma; 2 zettaculombios, coma; 1 exafaradio, coma; 2 petafaradios, coma, 1 teragray, coma, 2 gigagrays, coma; 1 megahenrio, coma; 2 kilohenrios, coma; 1 decahercio, coma; 2 decihercios, coma; 1 centijulio, coma; 2 milijulios, coma; 1 microkatal, coma; 2 nanokatales, coma, 1 picolumen, coma; 2 femtolúmenes, coma, 1 attolux, coma, 2 zeptolux, coma; 1 miligrado Celsius, coma; 2 microgrados Celsius, coma; 1 picogrado Celsius, coma; 2 nanogrados Celsius")?;
                return Ok(());

}

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
    test("es", "SimpleSpeak", expr, 
        "1 newton, coma, 2 newtons, coma, 1 ohmio, coma, 2 ohmios, coma, 1 ohmio, coma, 2 ohmios, coma, 1 pascal, coma, 2 pascales, coma, 1 siemens, coma, 2 siemens, coma, 1 sievert, coma, 2 sieverts, coma, 1 tesla, coma, 2 teslas, coma, 1 voltio, coma, 2 voltios, coma, 1 vatio, coma, 2 vatios, coma, 1 weber, coma, 2 weberes")?;
                return Ok(());

}

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
    test("es", "SimpleSpeak", expr, 
        "1 quectonewton, coma; 2 rontonewtons, coma; 1 yoctoohmio, coma; 2 zeptoohmios, coma, 1 attoohmio, coma; 2 femtoohmios, coma; 1 picopascal, coma; 2 nanopascales, coma; 1 microsiemens, coma; 2 milisiemens, coma; 1 centisievert, coma; 2 decisieverts, coma, 1 decatesla, coma; 2 hectoteslas, coma; 1 kilovoltio, coma; 2 megavoltios, coma, 1 gigavatio, coma; 2 teravatios, coma, 1 petaweber, coma; 2 exaweberes")?;
                return Ok(());

}


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
    test("es", "SimpleSpeak", expr, 
        "1 litro, coma, 2 litros, coma, 1 litro, coma, 2 litros, coma, 1 litro, coma, 2 litros, coma, 1 tonelada, coma, 2 toneladas, coma, 1 dalton, coma, 2 daltons, coma, 1 néper, coma, 2 néperes, coma; 1 unidad de masa atómica, coma; 2 unidades de masa atómica, coma; 1 electronvoltio, coma; 2 electronvoltios, coma, 1 radián, coma, 2 radianes, coma; 1 estereorradián, coma; 2 estereorradianes, coma, 1 año, coma, 2 años, coma; 1 segundo de arco, coma; 2 segundos de arco, coma, 1 bit coma, 2 bits, coma, 1 byte, coma, 2 bytes, coma, 1 baudio, coma, 2 baudios")?;
                return Ok(());

}

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
    test("es", "SimpleSpeak", expr, 
        "1 quettalitro, coma; 2 ronnalitros, coma; 1 yottalitro, coma; 2 zettalitros, coma, 1 exalitro, coma; 2 petalitros, coma; 1 teratonelada, coma; 2 gigatoneladas, coma; 1 megadalton, coma; 2 kilodaltons, coma; 1 decinéper, coma; 2 centinéperes, coma; 1 hectounidad de masa atómica; coma; 2 decaunidades de masa atómica; coma; 1 milielectronvoltio, coma; 2 microelectronvoltios, coma; 1 nanoradián, coma; 2 picoradianes, coma; 1 femtoestereorradián, coma; 2 attoestereorradianes, coma, 1 gigaaño, coma, 2 megaaños, coma; 1 zeptosegundo de arco, coma; 2 yoctosegundos de arco, coma, 1 kilobit, coma, 2 megabits, coma, 1 gigabyte, coma, 2 terabytes, coma; 1 terabaudio, coma; 2 exabaudios")?;
                return Ok(());

}

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
    test("es", "SimpleSpeak", expr, 
        "1 segundo, coma, 2 segundos, coma, 1 segundo, coma, 2 segundos, coma, 1 minuto, coma, 2 minutos, coma, 1 minuto, coma, 2 minutos, coma, 1 minuto, coma, 2 minutos, coma, 1 hora, coma, 2 horas, coma, 1 hora, coma, 2 horas, coma, 1 hora, coma, 2 horas, coma, 1 día, coma, 2 días, coma, 1 día, coma, 2 días, coma, 1 semana, coma, 2 semanas, coma, 1 semana, coma, 2 semanas, coma, 1 año, coma, 2 años, coma, 1 año, coma, 2 años")?;
                return Ok(());

}

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
    test("es", "SimpleSpeak", expr, 
        "1 grado, coma, 2 grados, coma, 1 grado, coma, 2 grados, coma; 1 minuto de arco, coma; 2 minutos de arco, coma; 1 minuto de arco, coma; 2 minutos de arco, coma; 1 minuto de arco, coma; 2 minutos de arco, coma; 1 minuto de arco, coma; 2 minutos de arco, coma; 1 segundo de arco, coma; 2 segundos de arco, coma; 1 segundo de arco, coma; 2 segundos de arco")?;
                return Ok(());

}

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
    test("es", "SimpleSpeak", expr, 
        "1 unidad astronómica, coma; 2 unidades astronómicas, coma, 1 año luz, coma, 2 años luz, coma, 1 parsec, coma, 2 parsecs, coma, 1 ángstrom, coma, 2 ángstroms, coma, 1 ángstrom, coma, 2 ángstroms, coma, 1 fermio, coma, 2 fermios")?;
                return Ok(());

}

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
    test("es", "SimpleSpeak", expr, 
        "1 hectárea, coma, 2 hectáreas, coma, 1 decibelio, coma, 2 decibelios, coma, 1 atmósfera, coma, 2 atmósferas, coma; 1 unidad de masa atómica, coma; 2 unidades de masa atómica, coma, 1 bar coma, 2 bares, coma, 1 caloría, coma, 2 calorías, coma, 1 curio, coma, 2 curios, coma, 1 gradián, coma, 2 gradianes, coma, 1 molar, coma, 2 molares, coma, 1 roentgen, coma, 2 roentgens, coma; 1 revolución por minuto, coma; 2 revoluciones por minuto, coma; 1 dracma líquida, coma; 2 dracmas líquidas, coma, 1 mho coma, 2 mhos, coma, 1 dina, coma, 2 dinas, coma, 1 ergio, coma, 2 ergios")?;
                return Ok(());

}

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
    test("es", "SimpleSpeak", expr, 
        "1 kibibit, coma, 2 kibibits, coma, 1 mebibit, coma, 2 mebibits, coma, 1 gibibit, coma, 2 gibibits, coma, 1 tebibit, coma, 2 tebibits, coma, 1 pebibit, coma, 2 pebibits, coma, 1 exbibit, coma, 2 exbibits, coma, 1 zebibit, coma, 2 zebibits, coma, 1 yobibit, coma, 2 yobibits, coma, 1 kibibyte, coma, 2 kibibytes, coma, 1 mebibyte, coma, 2 mebibytes, coma, 1 gibibyte, coma, 2 gibibytes, coma, 1 tebibyte, coma, 2 tebibytes, coma, 1 pebibyte, coma, 2 pebibytes, coma, 1 exbibyte, coma, 2 exbibytes, coma, 1 zebibyte, coma, 2 zebibytes, coma, 1 yobibyte, coma, 2 yobibytes")?;
                return Ok(());

}


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
    test_prefs("es", "SimpleSpeak", vec![("Verbosity", "Terse")], expr,
            "10 l coma, 20 m coma; x milisegundos, coma; y microsegundos, coma, decagramos coma; 1235 decanewtons, coma; 25 microsegundos, coma, 3234 moles")?;
    test_prefs("es", "ClearSpeak", vec![("Verbosity", "Medium")], expr,
            "10 litro, coma, 20 metros, coma; x milisegundos, coma; y microsegundos, coma, decagramos coma; 1235 decanewtons, coma; 25 microsegundos, coma, 3234 moles")?;
    test_prefs("es", "SimpleSpeak", vec![("Verbosity", "Verbose")], expr,
            "10 litro, coma, 20 metros, coma; x milisegundos, coma; y microsegundos, coma, decagramos coma; 1235 decanewtons, coma; 25 microsegundos, coma, 3234 moles")?;
                    return Ok(());

}


#[test]
fn test_mtext_inference() -> Result<()> {
    let expr = r#"<math><mo>[</mo>
                <mn>1</mn><mtext>t</mtext><mo>,</mo>
                <mn>2</mn><mtext>PA</mtext><mo>,</mo>
                <mn>3</mn><mtext>Pa</mtext><mo>,</mo>
                <mn>4.5</mn><mtext>mT</mtext>
            <mo>]</mo></math>"#;
    test("es", "SimpleSpeak", expr, 
        "abrir corchetes; 1 tonelada, coma; 2 petaamperios, coma, 3 pascales, coma; 45 militeslas; cerrar corchetes")?;
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
        test("es", "SimpleSpeak", expr, 
            "3 metros, coma, 1 kilómetro, coma, 3 metros, coma; 3 décimos faradios, coma; m sub min fin de sub")?;
            return Ok(());

    }

// Tests propios del español (no existen en inglés)
#[test]
fn metro() -> Result<()> {
    // this is a special case in Spanish
    let expr = r#"<math>
                <mn>1</mn><mi intent=":unit">m</mi><mo>,</mo>
                <mn>2</mn><mi intent=":unit">m</mi><mo>,</mo>
                <mn>1</mn><mi intent=":unit">Gm</mi><mo>,</mo>
                <mn>2</mn><mi intent=":unit">Gm</mi><mo>,</mo>
                <mn>1</mn><mi intent=":unit">Mm</mi><mo>,</mo>
                <mn>2</mn><mi intent=":unit">Mm</mi><mo>,</mo>
                <mn>1</mn><mi intent=":unit">km</mi><mo>,</mo>
                <mn>2</mn><mi intent=":unit">km</mi><mo>,</mo>
                <mn>1</mn><mi intent=":unit">hm</mi><mo>,</mo>
                <mn>2</mn><mi intent=":unit">hm</mi><mo>,</mo>
                <mn>1</mn><mi intent=":unit">dam</mi><mo>,</mo>
                <mn>2</mn><mi intent=":unit">dam</mi><mo>,</mo>
                <mn>1</mn><mi intent=":unit">mm</mi><mo>,</mo>
                <mn>2</mn><mi intent=":unit">mm</mi>
            </math>"#;
    test("es", "SimpleSpeak", expr, 
        "1 metro, coma, 2 metros, coma, 1 gigámetro, coma, 2 gigámetros, coma, 1 megámetro, coma, 2 megámetros, coma, 1 kilómetro, coma, 2 kilómetros, coma, 1 hectómetro, coma, 2 hectómetros, coma, 1 decámetro, coma, 2 decámetros, coma, 1 milímetro, coma, 2 milímetros")?;
                return Ok(());

}

// pruebas propias del español: plurales de monedas y unidades
#[test]
fn currency_plural() -> Result<()> {
    test("es", "SimpleSpeak", "<math><mo>$</mo><mn>1</mn></math>", "1 dólar")?;
    test("es", "SimpleSpeak", "<math><mo>$</mo><mn>3</mn></math>", "3 dólares")?;
    test("es", "SimpleSpeak", "<math><mo>€</mo><mn>20</mn></math>", "20 euros")?;
    test("es", "SimpleSpeak", "<math><mo>₡</mo><mn>5</mn></math>", "5 colones")?;
    return Ok(());
}

#[test]
fn radian_singular_plural() -> Result<()> {
    test("es", "SimpleSpeak", r#"<math><mn>1</mn><mi intent=":unit">rad</mi></math>"#, "1 radián")?;
    test("es", "SimpleSpeak", r#"<math><mn>3</mn><mi intent=":unit">rad</mi></math>"#, "3 radianes")?;
    return Ok(());
}

#[test]
fn degrees_celsius_and_angles_cmu() -> Result<()> {
    // CMU 2024, apéndice 1 y 11.5: {}^{\circ}C, y minutos y segundos tras los grados
    let cases = [
        ("<math><mn>1</mn><mspace width=\"0.167em\"></mspace><msup><mrow></mrow><mo>∘</mo></msup><mi mathvariant=\"normal\">C</mi></math>", "1 grado Celsius"),
        ("<math><mn>25</mn><mspace width=\"0.167em\"></mspace><msup><mrow></mrow><mo>∘</mo></msup><mi mathvariant=\"normal\">C</mi></math>", "25 grados Celsius"),
        ("<math><mn>25</mn><mspace width=\"0.167em\"></mspace><mrow intent=\":unit\"><msup><mrow></mrow><mo>∘</mo></msup><mi mathvariant=\"normal\">C</mi></mrow></math>", "25 grados Celsius"),
        ("<math><mn>98</mn><mspace width=\"0.167em\"></mspace><msup><mrow></mrow><mo>∘</mo></msup><mi mathvariant=\"normal\">F</mi></math>", "98 grados Fahrenheit"),
        ("<math><msup><mn>37</mn><mo>∘</mo></msup><mspace width=\"0.167em\"></mspace><msup><mn>22</mn><mo>′</mo></msup><mspace width=\"0.167em\"></mspace><msup><mn>49</mn><mrow><mo>′</mo><mo>′</mo></mrow></msup></math>", "37 grados, 22 minutos, 49 segundos"),
        ("<math><msup><mn>1</mn><mo>°</mo></msup><msup><mn>1</mn><mo>′</mo></msup><msup><mn>1</mn><mo>″</mo></msup></math>", "1 grado, 1 minuto, 1 segundo"),
        ("<math><msup><mi>f</mi><mo>′</mo></msup><mo>(</mo><mi>x</mi><mo>)</mo></math>", "f prima, de x"),
        ("<math><msup><mi>x</mi><mo>″</mo></msup></math>", "x segunda"),
        ("<math><msup><mn>5</mn><mo>′</mo></msup></math>", "5 prima"),
    ];
    for (expr, speech) in cases {
        test("es", "ClearSpeak", expr, speech)?;
    }
    return Ok(());
}

#[test]
fn units_and_currencies_common_in_spanish() -> Result<()> {
    // unidades y monedas de uso corriente en español que no están en inglés
    let cases = [
        ("<math><mn>1</mn><mspace width=\"0.167em\"></mspace><mi mathvariant=\"normal\">kWh</mi></math>", "1 kilovatio hora"),
        ("<math><mn>10</mn><mspace width=\"0.167em\"></mspace><mi mathvariant=\"normal\">kWh</mi></math>", "10 kilovatios hora"),
        ("<math><mn>10</mn><mspace width=\"0.167em\"></mspace><mi mathvariant=\"normal\">kcal</mi></math>", "10 kilocalorías"),
        ("<math><mn>10</mn><mspace width=\"0.167em\"></mspace><mi mathvariant=\"normal\">mAh</mi></math>", "10 miliamperios hora"),
        ("<math><mn>760</mn><mspace width=\"0.167em\"></mspace><mi mathvariant=\"normal\">mmHg</mi></math>", "760 milímetros de mercurio"),
        ("<math><mn>32</mn><mspace width=\"0.167em\"></mspace><mi mathvariant=\"normal\">psi</mi></math>", "32 libras por pulgada cuadrada"),
        ("<math><mn>1013</mn><mspace width=\"0.167em\"></mspace><mi mathvariant=\"normal\">mbar</mi></math>", "1013 milibares"),
        ("<math><mn>5</mn><mspace width=\"0.167em\"></mspace><mi mathvariant=\"normal\">cc</mi></math>", "5 centímetros cúbicos"),
        ("<math><mn>20</mn><mspace width=\"0.167em\"></mspace><mi mathvariant=\"normal\">kn</mi></math>", "20 nudos"),
        ("<math><mn>90</mn><mspace width=\"0.167em\"></mspace><mi mathvariant=\"normal\">CV</mi></math>", "90 caballos de vapor"),
        ("<math><mn>400</mn><mspace width=\"0.167em\"></mspace><mi mathvariant=\"normal\">ppm</mi></math>", "400 partes por millón"),
        ("<math><mn>10</mn><mspace width=\"0.167em\"></mspace><mi mathvariant=\"normal\">kgf</mi></math>", "10 kilogramos fuerza"),
        ("<math><mn>100</mn><mspace width=\"0.167em\"></mspace><mi mathvariant=\"normal\">Mbps</mi></math>", "100 megabits por segundo"),
        ("<math><mn>3</mn><mspace width=\"0.167em\"></mspace><mi mathvariant=\"normal\">yd</mi></math>", "3 yardas"),
        ("<math><mn>10</mn><mspace width=\"0.167em\"></mspace><mi mathvariant=\"normal\">KB</mi></math>", "10 kilobytes"),
        ("<math><mn>10</mn><mspace width=\"0.167em\"></mspace><mi mathvariant=\"normal\">bps</mi></math>", "10 bits por segundo"),
        ("<math><mn>10</mn><mspace width=\"0.167em\"></mspace><mi mathvariant=\"normal\">MBps</mi></math>", "10 megabytes por segundo"),
        ("<math><mn>300</mn><mspace width=\"0.167em\"></mspace><mi mathvariant=\"normal\">dpi</mi></math>", "300 puntos por pulgada"),
        ("<math><mn>300</mn><mspace width=\"0.167em\"></mspace><mi mathvariant=\"normal\">ppp</mi></math>", "300 puntos por pulgada"),
        ("<math><mn>1</mn><mspace width=\"0.167em\"></mspace><mi mathvariant=\"normal\">px</mi></math>", "1 píxel"),
        ("<math><mn>10</mn><mspace width=\"0.167em\"></mspace><mi mathvariant=\"normal\">px</mi></math>", "10 píxeles"),
        ("<math><mn>60</mn><mspace width=\"0.167em\"></mspace><mi mathvariant=\"normal\">fps</mi></math>", "60 fotogramas por segundo"),
        ("<math><mn>10</mn><mspace width=\"0.167em\"></mspace><mi mathvariant=\"normal\">GFLOPS</mi></math>", "10 gigaflops"),
        ("<math><mn>10</mn><mspace width=\"0.167em\"></mspace><mi mathvariant=\"normal\">dBm</mi></math>", "10 decibelios milivatio"),
        ("<math><mn>1</mn><mspace width=\"0.167em\"></mspace><mi mathvariant=\"normal\">ua</mi></math>", "1 unidad astronómica"),
        ("<math><mi mathvariant=\"normal\">R$</mi><mn>150</mn></math>", "150 reales"),
        ("<math><mi mathvariant=\"normal\">¥</mi><mn>3</mn></math>", "3 yenes"),
        ("<math><mi mathvariant=\"normal\">₲</mi><mn>3</mn></math>", "3 guaraníes"),
        ("<math><mi mathvariant=\"normal\">₿</mi><mn>3</mn></math>", "3 bitcoins"),
    ];
    for (expr, speech) in cases {
        test("es", "ClearSpeak", expr, speech)?;
    }
    return Ok(());
}
