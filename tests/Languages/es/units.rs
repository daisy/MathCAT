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
        "un amperio, coma, 2 amperios, coma, una candela, coma, 2 candelas, coma, un kelvin, coma, 2 kelvins, coma, un kelvin, coma, 2 kelvins, coma, un gramo, coma, 2 gramos, coma, un metro, coma, 2 metros, coma, un mol, coma, 2 moles, coma, un segundo, coma, 2 segundos, coma, un segundo, coma, 2 segundos, coma, un segundo, coma, 2 segundos, coma, un segundo, coma, 2 segundos")?;
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
        "un quettaamperio, coma; 2 ronnaamperios, coma; una yottacandela, coma; 2 zettacandelas, coma; un exakelvin, coma; 2 petakelvins, coma; un terakelvin, coma; 2 gigakelvins, coma; un megagramo, coma; 2 kilogramos, coma, un hectómetro, coma, 2 decámetros, coma, un decimol, coma; 2 centimoles, coma; un milisegundo, coma; 2 microsegundos, coma; un nanosegundo, coma; 2 picosegundos")?;
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
        "un becquerel, coma, 2 becquereles, coma, un culombio, coma, 2 culombios, coma; un grado Celsius, coma; 2 grados Celsius, coma; un grado Celsius, coma; 2 grados Celsius, coma, un faradio, coma, 2 faradios, coma, un gray, coma, 2 grays, coma, un henrio, coma, 2 henrios, coma, un hercio, coma, 2 hercios, coma, un julio, coma, 2 julios, coma, un katal, coma, 2 katales, coma, un lumen, coma, 2 lúmenes, coma, un lux, coma, 2 lux")?;
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
        "un quettabecquerel, coma; 2 ronnabecquereles, coma; un yottaculombio, coma; 2 zettaculombios, coma; un exafaradio, coma; 2 petafaradios, coma, un teragray, coma, 2 gigagrays, coma; un megahenrio, coma; 2 kilohenrios, coma; un decahercio, coma; 2 decihercios, coma; un centijulio, coma; 2 milijulios, coma; un microkatal, coma; 2 nanokatales, coma; un picolumen, coma; 2 femtolúmenes, coma, un attolux, coma, 2 zeptolux, coma; un miligrado Celsius, coma; 2 microgrados Celsius, coma; un picogrado Celsius, coma; 2 nanogrados Celsius")?;
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
        "un newton, coma, 2 newtons, coma, un ohmio, coma, 2 ohmios, coma, un ohmio, coma, 2 ohmios, coma, un pascal, coma, 2 pascales, coma, un siemens, coma, 2 siemens, coma, un sievert, coma, 2 sieverts, coma, un tesla, coma, 2 teslas, coma, un voltio, coma, 2 voltios, coma, un vatio, coma, 2 vatios, coma, un weber, coma, 2 weberes")?;
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
        "un quectonewton, coma; 2 rontonewtons, coma; un yoctoohmio, coma; 2 zeptoohmios, coma; un attoohmio, coma; 2 femtoohmios, coma; un picopascal, coma; 2 nanopascales, coma; un microsiemens, coma; 2 milisiemens, coma; un centisievert, coma; 2 decisieverts, coma; un decatesla, coma; 2 hectoteslas, coma; un kilovoltio, coma; 2 megavoltios, coma; un gigavatio, coma; 2 teravatios, coma; un petaweber, coma; 2 exaweberes")?;
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
        "un litro, coma, 2 litros, coma, un litro, coma, 2 litros, coma, un litro, coma, 2 litros, coma, una tonelada, coma, 2 toneladas, coma, un dalton, coma, 2 daltons, coma, un néper, coma, 2 néperes, coma; una unidad de masa atómica, coma; 2 unidades de masa atómica, coma; un electronvoltio, coma; 2 electronvoltios, coma, un radián, coma, 2 radianes, coma; un estereorradián, coma; 2 estereorradianes, coma, un año, coma, 2 años, coma; un segundo de arco, coma; 2 segundos de arco, coma, un bit, coma, 2 bits, coma, un byte, coma, 2 bytes, coma, un baudio, coma, 2 baudios")?;
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
        "un quettalitro, coma; 2 ronnalitros, coma; un yottalitro, coma; 2 zettalitros, coma, un exalitro, coma; 2 petalitros, coma; una teratonelada, coma; 2 gigatoneladas, coma; un megadalton, coma; 2 kilodaltons, coma; un decinéper, coma; 2 centinéperes, coma; una hectounidad de masa atómica; coma; 2 decaunidades de masa atómica; coma; un milielectronvoltio, coma; 2 microelectronvoltios, coma; un nanoradián, coma; 2 picoradianes, coma; un femtoestereorradián, coma; 2 attoestereorradianes, coma, un gigaaño, coma, 2 megaaños, coma; un zeptosegundo de arco, coma; 2 yoctosegundos de arco, coma, un kilobit, coma, 2 megabits, coma, un gigabyte, coma, 2 terabytes, coma; un terabaudio, coma; 2 exabaudios")?;
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
        "un segundo, coma, 2 segundos, coma, un segundo, coma, 2 segundos, coma, un minuto, coma, 2 minutos, coma, un minuto, coma, 2 minutos, coma, un minuto, coma, 2 minutos, coma, una hora, coma, 2 horas, coma, una hora, coma, 2 horas, coma, una hora, coma, 2 horas, coma, un día, coma, 2 días, coma, un día, coma, 2 días, coma, una semana, coma, 2 semanas, coma, una semana, coma, 2 semanas, coma, un año, coma, 2 años, coma, un año, coma, 2 años")?;
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
        "un grado, coma, 2 grados, coma, un grado, coma, 2 grados, coma; un minuto de arco, coma; 2 minutos de arco, coma; un minuto de arco, coma; 2 minutos de arco, coma; un minuto de arco, coma; 2 minutos de arco, coma; un minuto de arco, coma; 2 minutos de arco, coma; un segundo de arco, coma; 2 segundos de arco, coma; un segundo de arco, coma; 2 segundos de arco")?;
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
        "una unidad astronómica, coma; 2 unidades astronómicas, coma, un año luz, coma, 2 años luz, coma, un parsec, coma, 2 parsecs, coma, un ángstrom, coma, 2 ángstroms, coma, un ángstrom, coma, 2 ángstroms, coma, un fermio, coma, 2 fermios")?;
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
        "una hectárea, coma, 2 hectáreas, coma, un decibelio, coma, 2 decibelios, coma, una atmósfera, coma, 2 atmósferas, coma; una unidad de masa atómica, coma; 2 unidades de masa atómica, coma, un bar, coma, 2 bares, coma, una caloría, coma, 2 calorías, coma, un curio, coma, 2 curios, coma, un gradián, coma, 2 gradianes, coma, un molar, coma, 2 molares, coma, un roentgen, coma, 2 roentgens, coma; una revolución por minuto, coma; 2 revoluciones por minuto, coma; una dracma líquida, coma; 2 dracmas líquidas, coma, un mho, coma, 2 mhos, coma, una dina, coma, 2 dinas, coma, un ergio, coma, 2 ergios")?;
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
        "un kibibit, coma, 2 kibibits, coma, un mebibit, coma, 2 mebibits, coma, un gibibit, coma, 2 gibibits, coma, un tebibit, coma, 2 tebibits, coma, un pebibit, coma, 2 pebibits, coma, un exbibit, coma, 2 exbibits, coma, un zebibit, coma, 2 zebibits, coma, un yobibit, coma, 2 yobibits, coma, un kibibyte, coma, 2 kibibytes, coma, un mebibyte, coma, 2 mebibytes, coma, un gibibyte, coma, 2 gibibytes, coma, un tebibyte, coma, 2 tebibytes, coma, un pebibyte, coma, 2 pebibytes, coma, un exbibyte, coma, 2 exbibytes, coma, un zebibyte, coma, 2 zebibytes, coma, un yobibyte, coma, 2 yobibytes")?;
                return Ok(());

}


#[test]
fn si_other_numbers() -> Result<()> {
    let expr = r#"<math><mn>1,0</mn><mi intent=":unit">l</mi><mo>,</mo>
                            <mn>2,0</mn><mo>&#xA0;</mo><mi intent=":unit">m</mi><mo>,</mo>
                            <mi>x</mi><mo>&#xA0;</mo><mi intent=":unit">ms</mi><mo>,</mo>
                            <mi>y</mi><mi intent=":unit">µs</mi><mo>,</mo>
                            <mi intent=":unit">dag</mi><mo>,</mo>
                            <mn>1235</mn><mi intent=":unit">daN</mi><mo>,</mo>
                            <mn>2,5</mn><mi intent=":unit">&#xB5;sec</mi><mo>,</mo>
                            <mn>32,34</mn><mi intent=":unit">mol</mi></math>"#;
    test_prefs("es", "SimpleSpeak", vec![("Verbosity", "Terse")], expr,
            "1,0 l coma, 2,0 m coma; x milisegundos, coma; y microsegundos, coma, decagramos coma; 1235 decanewtons, coma; 2,5 microsegundos, coma, 32,34 moles")?;
    test_prefs("es", "ClearSpeak", vec![("Verbosity", "Medium")], expr,
            "1,0 litro, coma, 2,0 metros, coma; x milisegundos, coma; y microsegundos, coma, decagramos coma; 1235 decanewtons, coma; 2,5 microsegundos, coma, 32,34 moles")?;
    test_prefs("es", "SimpleSpeak", vec![("Verbosity", "Verbose")], expr,
            "1,0 litro, coma, 2,0 metros, coma; x milisegundos, coma; y microsegundos, coma, decagramos coma; 1235 decanewtons, coma; 2,5 microsegundos, coma, 32,34 moles")?;
                    return Ok(());

}


#[test]
fn test_mtext_inference() -> Result<()> {
    let expr = r#"<math><mo>[</mo>
                <mn>1</mn><mtext>t</mtext><mo>,</mo>
                <mn>2</mn><mtext>PA</mtext><mo>,</mo>
                <mn>3</mn><mtext>Pa</mtext><mo>,</mo>
                <mn>4,5</mn><mtext>mT</mtext>
            <mo>]</mo></math>"#;
    test("es", "SimpleSpeak", expr, 
        "abrir corchetes; una tonelada, coma; 2 petaamperios, coma, 3 pascales, coma; 4,5 militeslas; cerrar corchetes")?;
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
            "3 metros, coma, un kilómetro, coma, 3 metros, coma; 3 décimos faradios, coma; m sub min fin de sub")?;
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
        "un metro, coma, 2 metros, coma, un gigámetro, coma, 2 gigámetros, coma, un megámetro, coma, 2 megámetros, coma, un kilómetro, coma, 2 kilómetros, coma, un hectómetro, coma, 2 hectómetros, coma, un decámetro, coma, 2 decámetros, coma, un milímetro, coma, 2 milímetros")?;
                return Ok(());

}

// pruebas propias del español: plurales de monedas y unidades
#[test]
fn currency_plural() -> Result<()> {
    test("es", "SimpleSpeak", "<math><mo>$</mo><mn>1</mn></math>", "un dólar")?;
    test("es", "SimpleSpeak", "<math><mo>$</mo><mn>3</mn></math>", "3 dólares")?;
    test("es", "SimpleSpeak", "<math><mo>€</mo><mn>20</mn></math>", "20 euros")?;
    test("es", "SimpleSpeak", "<math><mo>₡</mo><mn>5</mn></math>", "5 colones")?;
    return Ok(());
}

#[test]
fn radian_singular_plural() -> Result<()> {
    test("es", "SimpleSpeak", r#"<math><mn>1</mn><mi intent=":unit">rad</mi></math>"#, "un radián")?;
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
        ("<math><mn>1</mn><mspace width=\"0.167em\"></mspace><mi mathvariant=\"normal\">kWh</mi></math>", "un kilovatio hora"),
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
        ("<math><mn>1</mn><mspace width=\"0.167em\"></mspace><mi mathvariant=\"normal\">px</mi></math>", "un píxel"),
        ("<math><mn>10</mn><mspace width=\"0.167em\"></mspace><mi mathvariant=\"normal\">px</mi></math>", "10 píxeles"),
        ("<math><mn>60</mn><mspace width=\"0.167em\"></mspace><mi mathvariant=\"normal\">fps</mi></math>", "60 fotogramas por segundo"),
        ("<math><mn>10</mn><mspace width=\"0.167em\"></mspace><mi mathvariant=\"normal\">GFLOPS</mi></math>", "10 gigaflops"),
        ("<math><mn>10</mn><mspace width=\"0.167em\"></mspace><mi mathvariant=\"normal\">dBm</mi></math>", "10 decibelios milivatio"),
        ("<math><mn>10</mn><mspace width=\"0.167em\"></mspace><mi mathvariant=\"normal\">°K</mi></math>", "10 grados kelvin"),
        ("<math><mn>1</mn><mspace width=\"0.167em\"></mspace><mi mathvariant=\"normal\">ua</mi></math>", "una unidad astronómica"),
        ("<math><mi mathvariant=\"normal\">R$</mi><mn>150</mn></math>", "150 reales"),
        ("<math><mi mathvariant=\"normal\">¥</mi><mn>3</mn></math>", "3 yenes"),
        ("<math><mi mathvariant=\"normal\">₲</mi><mn>3</mn></math>", "3 guaraníes"),
        ("<math><mi mathvariant=\"normal\">₿</mi><mn>3</mn></math>", "3 bitcoines"),
    ];
    for (expr, speech) in cases {
        test("es", "ClearSpeak", expr, speech)?;
    }
    return Ok(());
}

#[test]
fn unit_gender_exceptions() -> Result<()> {
    // "1" con el género de la unidad: excepciones a la regla de la última letra
    let cases = [
        ("<math><mn>1</mn><mspace width=\"0.167em\"></mspace><mi mathvariant=\"normal\">T</mi></math>", "un tesla"),
        ("<math><mn>1</mn><mspace width=\"0.167em\"></mspace><mi mathvariant=\"normal\">Wh</mi></math>", "un vatio hora"),
        ("<math><mn>1</mn><mspace width=\"0.167em\"></mspace><mi mathvariant=\"normal\">Ah</mi></math>", "un amperio hora"),
        ("<math><mn>1</mn><mspace width=\"0.167em\"></mspace><mi mathvariant=\"normal\">kgf</mi></math>", "un kilogramo fuerza"),
        ("<math><mn>1</mn><mspace width=\"0.167em\"></mspace><mi mathvariant=\"normal\">hp</mi></math>", "un caballo de fuerza"),
        ("<math><mn>1</mn><mspace width=\"0.167em\"></mspace><mi mathvariant=\"normal\">ppp</mi></math>", "un punto por pulgada"),
        ("<math><mn>1</mn><mspace width=\"0.167em\"></mspace><mi mathvariant=\"normal\">oz t</mi></math>", "una onza troy"),
        ("<math><mn>1</mn><mspace width=\"0.167em\"></mspace><mi mathvariant=\"normal\">lb t</mi></math>", "una libra troy"),
        ("<math><mn>1</mn><mspace width=\"0.167em\"></mspace><mi mathvariant=\"normal\">mpg</mi></math>", "una milla por galón"),
    ];
    for (expr, speech) in cases {
        test("es", "ClearSpeak", expr, speech)?;
    }
    return Ok(());
}

#[test]
fn per_and_silent_dot_between_units() -> Result<()> {
    // "/" y "∕" entre unidades se leen "por"; "·" y "⋅" entre unidades no se leen
    let cases = [
        ("<math><mn>10</mn><mspace width=\"0.167em\"></mspace><mrow><mi mathvariant=\"normal\">m</mi><mo>/</mo><mi mathvariant=\"normal\">s</mi></mrow></math>", "10 metros por segundo"),
        ("<math><mn>90</mn><mspace width=\"0.167em\"></mspace><mrow><mi mathvariant=\"normal\">km</mi><mo>/</mo><mi mathvariant=\"normal\">h</mi></mrow></math>", "90 kilómetros por hora"),
        ("<math><mn>9</mn><mspace width=\"0.167em\"></mspace><mi mathvariant=\"normal\">m</mi><mo>∕</mo><mi mathvariant=\"normal\">s</mi></math>", "9 metros por segundo"),
        // CompoundUnitPlural first: pluralize the first factor only (“newtons metro”).
        ("<math><mn>5</mn><mspace width=\"0.167em\"></mspace><mi mathvariant=\"normal\">N</mi><mo>·</mo><mi mathvariant=\"normal\">m</mi></math>", "5 newtons metro"),
        ("<math><mn>5</mn><mspace width=\"0.167em\"></mspace><mi mathvariant=\"normal\">N</mi><mo>⋅</mo><mi mathvariant=\"normal\">m</mi></math>", "5 newtons metro"),
        ("<math><mn>2</mn><mo>·</mo><mn>5</mn></math>", "2 por 5"),
        ("<math><mi>a</mi><mo>/</mo><mi>b</mi></math>", "a dividido por b"),
    ];
    for (expr, speech) in cases {
        test("es", "ClearSpeak", expr, speech)?;
    }
    return Ok(());
}

#[test]
fn currency_after_amount() -> Result<()> {
    // símbolo después de la cantidad (España): "1 €" con el mismo "un"/"una" que "€ 1"; rublo y grivna;
    // millones exactos con "de" (NVDA dice "un millón de dólares")
    let cases = [
        ("<math><mn>1</mn><mo>€</mo></math>", "un euro"),
        ("<math><mn>1</mn><mo>£</mo></math>", "una libra"),
        ("<math><mn>2</mn><mo>£</mo></math>", "2 libras"),
        ("<math><mn>1,5</mn><mo>€</mo></math>", "1,5 euros"),
        ("<math><mn>1</mn><mi>₲</mi></math>", "un guaraní"),
        ("<math><mn>3</mn><mi>₲</mi></math>", "3 guaraníes"),
        ("<math><mn>1</mn><mi>R$</mi></math>", "un real"),
        ("<math><mi>₽</mi><mn>1</mn></math>", "un rublo"),
        ("<math><mn>2</mn><mi>₽</mi></math>", "2 rublos"),
        ("<math><mn>1</mn><mi>₴</mi></math>", "una grivna"),
        ("<math><mi>₴</mi><mn>2</mn></math>", "2 grivnas"),
        ("<math><mo>$</mo><mn>1000000</mn></math>", "1000000 de dólares"),
        ("<math><mo>$</mo><mn>3 000 000</mn></math>", "3000000 de dólares"),
        ("<math><mn>1 000 000</mn><mo>€</mo></math>", "1000000 de euros"),
        ("<math><mo>$</mo><mn>1 500 000</mn></math>", "1500000 dólares"),
        ("<math><mo>$</mo><mn>1 000 000,50</mn></math>", "1000000,50 dólares"),
    ];
    for (expr, speech) in cases {
        test("es", "ClearSpeak", expr, speech)?;
    }
    return Ok(());
}

#[test]
fn units_with_exact_millions() -> Result<()> {
    // millones exactos llevan "de": NVDA dice "un millón de metros"
    let cases = [
        ("<math><mn>1000000</mn><mi intent=\":unit\">m</mi></math>", "1000000 de metros"),
        ("<math><mn>2 000 000</mn><mspace width=\"0.167em\"></mspace><mi mathvariant=\"normal\">km</mi></math>", "2000000 de kilómetros"),
        ("<math><mn>1000000</mn><msup><mi intent=\":unit\">m</mi><mn>2</mn></msup></math>", "1000000 de metros al cuadrado"),
        ("<math><mn>1000000</mn><mrow><mi mathvariant=\"normal\">m</mi><mo>/</mo><mi mathvariant=\"normal\">s</mi></mrow></math>", "1000000 de metros por segundo"),
        ("<math><mn>1500000</mn><mi intent=\":unit\">m</mi></math>", "1500000 metros"),
        ("<math><mn>1000000,5</mn><mi intent=\":unit\">m</mi></math>", "1000000,5 metros"),
    ];
    for (expr, speech) in cases {
        test("es", "ClearSpeak", expr, speech)?;
    }
    return Ok(());
}
