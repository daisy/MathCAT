/// TESTES DO IDIOMA pt — como reconciliar com as regras
///
/// As strings esperadas abaixo foram escritas para refletir a intenção
/// das regras em Rules/Languages/pt. Vírgulas e ponto-e-vírgula marcam
/// PAUSAS inseridas pelo motor e podem divergir levemente na primeira
/// execução. Método do guia oficial de tradutores:
///   1. rode `cargo test Languages::pt`
///   2. para cada falha, compare `left` (esperado) com `right` (gerado)
///   3. se o GERADO estiver correto em português, copie-o para o teste;
///      se estiver errado, conserte a REGRA, nunca o teste.
///
/// ATENÇÃO ao histórico: o arquivo espanhol equivalente continha testes
/// que não refletiam as regras (ex.: esperava "logaritmo natural" com a
/// regra dizendo "natural log") e dano de busca-e-troca (ex.: "eigh"→"8").
/// Não herde strings esperadas de es/ sem conferir.

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
    test("pt", "SimpleSpeak", expr, 
        "quettagramas vírgula, ronnagramas vírgula, yottagramas vírgula, zettagramas vírgula, exagramas vírgula, petagramas vírgula, teragramas vírgula, gigagramas vírgula, megagramas vírgula, quilogramas vírgula, hectogramas vírgula, decagramas vírgula, decigramas vírgula, centigramas vírgula, miligramas vírgula, microgramas vírgula, nanogramas vírgula, picogramas vírgula, femtogramas vírgula, attogramas vírgula, zeptogramas vírgula, yoctogramas vírgula, rontogramas vírgula, quectogramas")?;
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
    test("pt", "SimpleSpeak", expr, 
        "1 ampère, vírgula; 2 ampères, vírgula; 1 candela, vírgula; 2 candelas, vírgula; 1 kelvin, vírgula; 2 kelvins, vírgula; 1 kelvin, vírgula; 2 kelvins, vírgula, 1 grama, vírgula; 2 gramas, vírgula, 1 metro, vírgula; 2 metros, vírgula, 1 mol, vírgula, 2 mols, vírgula; 1 segundo, vírgula; 2 segundos, vírgula; 1 segundo, vírgula; 2 segundos, vírgula; 1 segundo, vírgula; 2 segundos, vírgula; 1 segundo, vírgula; 2 segundos")?;
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
    test("pt", "SimpleSpeak", expr, 
        "1 quettaampère, vírgula; 2 ronnaampères, vírgula; 1 yottacandela, vírgula; 2 zettacandelas, vírgula; 1 exakelvin, vírgula; 2 petakelvins, vírgula; 1 terakelvin, vírgula; 2 gigakelvins, vírgula; 1 megagrama, vírgula; 2 quilogramas, vírgula; 1 hectômetro, vírgula; 2 decâmetros, vírgula; 1 decimol, vírgula; 2 centimols, vírgula; 1 milissegundo, vírgula; 2 microssegundos, vírgula; 1 nanossegundo, vírgula; 2 picossegundos")?;
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
    test("pt", "SimpleSpeak", expr, 
        "1 becquerel, vírgula; 2 becquerels, vírgula; 1 coulomb, vírgula; 2 coulombs, vírgula; 1 grau Celsius, vírgula; 2 graus Celsius, vírgula; 1 grau Celsius, vírgula; 2 graus Celsius, vírgula, 1 farad, vírgula; 2 farads, vírgula, 1 gray, vírgula, 2 grays, vírgula, 1 henry, vírgula; 2 henrys, vírgula, 1 hertz, vírgula, 2 hertz, vírgula, 1 joule, vírgula; 2 joules, vírgula, 1 katal, vírgula; 2 katals, vírgula; 1 lúmen, vírgula; 2 lúmens, vírgula, 1 lux, vírgula, 2 lux")?;
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
    test("pt", "SimpleSpeak", expr, 
        "1 quettabecquerel, vírgula; 2 ronnabecquerels, vírgula; 1 yottacoulomb, vírgula; 2 zettacoulombs, vírgula; 1 exafarad, vírgula; 2 petafarads, vírgula; 1 teragray, vírgula; 2 gigagrays, vírgula; 1 megahenry, vírgula; 2 quilohenrys, vírgula; 1 decahertz, vírgula; 2 decihertz, vírgula; 1 centijoule, vírgula; 2 milijoules, vírgula; 1 microkatal, vírgula; 2 nanokatals, vírgula; 1 picolúmen, vírgula; 2 femtolúmens, vírgula; 1 attolux, vírgula; 2 zeptolux, vírgula; 1 miligrau Celsius, vírgula; 2 micrograus Celsius, vírgula; 1 picograu Celsius, vírgula; 2 nanograus Celsius")?;
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
    test("pt", "SimpleSpeak", expr, 
        "1 newton, vírgula; 2 newtons, vírgula, 1 ohm, vírgula, 2 ohms, vírgula, 1 ohm, vírgula, 2 ohms, vírgula; 1 pascal, vírgula; 2 pascals, vírgula; 1 siemens, vírgula; 2 siemens, vírgula; 1 sievert, vírgula; 2 sieverts, vírgula, 1 tesla, vírgula; 2 teslas, vírgula, 1 volt, vírgula, 2 volts, vírgula, 1 watt, vírgula, 2 watts, vírgula, 1 weber, vírgula; 2 webers")?;
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
    test("pt", "SimpleSpeak", expr, 
        "1 quectonewton, vírgula; 2 rontonewtons, vírgula; 1 yoctoohm, vírgula; 2 zeptoohms, vírgula; 1 attoohm, vírgula; 2 femtoohms, vírgula; 1 picopascal, vírgula; 2 nanopascals, vírgula; 1 microssiemens, vírgula; 2 milissiemens, vírgula; 1 centissievert, vírgula; 2 decissieverts, vírgula; 1 decatesla, vírgula; 2 hectoteslas, vírgula; 1 quilovolt, vírgula; 2 megavolts, vírgula; 1 gigawatt, vírgula; 2 terawatts, vírgula; 1 petaweber, vírgula; 2 exawebers")?;
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
    test("pt", "SimpleSpeak", expr, 
        "1 litro, vírgula; 2 litros, vírgula, 1 litro, vírgula; 2 litros, vírgula, 1 litro, vírgula; 2 litros, vírgula; 1 tonelada, vírgula; 2 toneladas, vírgula; 1 dalton, vírgula; 2 daltons, vírgula, 1 neper, vírgula; 2 neperes, vírgula; 1 unidade de massa atômica, vírgula; 2 unidades de massa atômica; vírgula; 1 elétron-volt, vírgula; 2 elétron-volts, vírgula; 1 radiano, vírgula; 2 radianos, vírgula; 1 esferorradiano, vírgula; 2 esferorradianos, vírgula, 1 ano, vírgula, 2 anos, vírgula; 1 segundo de arco, vírgula; 2 segundos de arco, vírgula, 1 bit, vírgula, 2 bits, vírgula, 1 byte, vírgula, 2 bytes, vírgula, 1 baud, vírgula, 2 bauds")?;
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
    test("pt", "SimpleSpeak", expr, 
        "1 quettalitro, vírgula; 2 ronnalitros, vírgula; 1 yottalitro, vírgula; 2 zettalitros, vírgula; 1 exalitro, vírgula; 2 petalitros, vírgula; 1 teratonelada, vírgula; 2 gigatoneladas, vírgula; 1 megadalton, vírgula; 2 quilodaltons, vírgula; 1 decineper, vírgula; 2 centineperes, vírgula; 1 hectounidade de massa atômica; vírgula; 2 decaunidades de massa atômica; vírgula; 1 milielétron-volt, vírgula; 2 microelétron-volts, vírgula; 1 nanorradiano, vírgula; 2 picorradianos, vírgula; 1 femtoesferorradiano, vírgula; 2 attoesferorradianos, vírgula; 1 gigaano, vírgula; 2 megaanos, vírgula; 1 zeptossegundo de arco, vírgula; 2 yoctossegundos de arco, vírgula; 1 quilobit, vírgula; 2 megabits, vírgula; 1 gigabyte, vírgula; 2 terabytes, vírgula; 1 terabaud, vírgula; 2 exabauds")?;
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
    test("pt", "SimpleSpeak", expr, 
        "1 segundo, vírgula; 2 segundos, vírgula; 1 segundo, vírgula; 2 segundos, vírgula; 1 minuto, vírgula; 2 minutos, vírgula; 1 minuto, vírgula; 2 minutos, vírgula; 1 minuto, vírgula; 2 minutos, vírgula, 1 hora, vírgula, 2 horas, vírgula, 1 hora, vírgula, 2 horas, vírgula, 1 hora, vírgula, 2 horas, vírgula, 1 dia, vírgula, 2 dias, vírgula, 1 dia, vírgula, 2 dias, vírgula; 1 semana, vírgula; 2 semanas, vírgula; 1 semana, vírgula; 2 semanas, vírgula, 1 ano, vírgula, 2 anos, vírgula, 1 ano, vírgula, 2 anos")?;
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
    test("pt", "SimpleSpeak", expr, 
        "1 grau, vírgula, 2 graus, vírgula, \
                1 grau, vírgula, 2 graus, vírgula; \
                1 minuto de arco, vírgula; 2 minutos de arco, vírgula; \
                1 minuto de arco, vírgula; 2 minutos de arco, vírgula; \
                1 minuto de arco, vírgula; 2 minutos de arco, vírgula; \
                1 minuto de arco, vírgula; 2 minutos de arco, vírgula; \
                1 segundo de arco, vírgula; 2 segundos de arco, vírgula; \
                1 segundo de arco, vírgula; 2 segundos de arco")?;
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
    test("pt", "SimpleSpeak", expr, 
        "1 unidade astronômica, vírgula; 2 unidades astronômicas, vírgula; 1 ano-luz, vírgula; 2 anos-luz, vírgula; 1 parsec, vírgula; 2 parsecs, vírgula; 1 angström, vírgula; 2 angströms, vírgula; 1 angström, vírgula; 2 angströms, vírgula, 1 fermi, vírgula; 2 fermis")?;
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
        <mn>1</mn><mi intent=":unit">℧</mi><mo>,</mo><mn>2</mn><mi intent=":unit">℧</mi><mo>,</mo>
        <mn>1</mn><mi intent=":unit">dyn</mi><mo>,</mo><mn>2</mn><mi intent=":unit">dyn</mi><mo>,</mo>
        <mn>1</mn><mi intent=":unit">erg</mi><mo>,</mo><mn>2</mn><mi intent=":unit">erg</mi>
    </math>"#;
    test("pt", "SimpleSpeak", expr, 
        "1 hectare, vírgula; 2 hectares, vírgula; 1 decibel, vírgula; 2 decibéis, vírgula; 1 atmosfera, vírgula; 2 atmosferas, vírgula; 1 unidade de massa atômica, vírgula; 2 unidades de massa atômica; vírgula, 1 bar, vírgula, 2 bares, vírgula; 1 caloria, vírgula; 2 calorias, vírgula, 1 curie, vírgula; 2 curies, vírgula, 1 grado, vírgula; 2 grados, vírgula, 1 molar, vírgula; 2 molares, vírgula; 1 roentgen, vírgula; 2 roentgens, vírgula; 1 rotação por minuto, vírgula; 2 rotações por minuto, vírgula, 1 mho, vírgula, 2 mhos, vírgula, 1 dina, vírgula, 2 dinas, vírgula, 1 erg, vírgula, 2 ergs")?;
                return Ok(());

}

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
    test("pt", "SimpleSpeak", expr, 
        "1 metro, vírgula; 2 metros, vírgula; 1 gigâmetro, vírgula; 2 gigâmetros, vírgula; 1 megâmetro, vírgula; 2 megâmetros, vírgula; 1 quilômetro, vírgula; 2 quilômetros, vírgula; 1 hectômetro, vírgula; 2 hectômetros, vírgula; 1 decâmetro, vírgula; 2 decâmetros, vírgula; 1 milímetro, vírgula; 2 milímetros")?;
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
    test("pt", "SimpleSpeak", expr, 
        "1 kibibit, vírgula; 2 kibibits, vírgula; 1 mebibit, vírgula; 2 mebibits, vírgula; 1 gibibit, vírgula; 2 gibibits, vírgula; 1 tebibit, vírgula; 2 tebibits, vírgula; 1 pebibit, vírgula; 2 pebibits, vírgula; 1 exbibit, vírgula; 2 exbibits, vírgula; 1 zebibit, vírgula; 2 zebibits, vírgula; 1 yobibit, vírgula; 2 yobibits, vírgula; 1 kibibyte, vírgula; 2 kibibytes, vírgula; 1 mebibyte, vírgula; 2 mebibytes, vírgula; 1 gibibyte, vírgula; 2 gibibytes, vírgula; 1 tebibyte, vírgula; 2 tebibytes, vírgula; 1 pebibyte, vírgula; 2 pebibytes, vírgula; 1 exbibyte, vírgula; 2 exbibytes, vírgula; 1 zebibyte, vírgula; 2 zebibytes, vírgula; 1 yobibyte, vírgula; 2 yobibytes")?;
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
    test("pt", "SimpleSpeak", expr, 
        "10 litro, vírgula; 20 metros, vírgula; x milissegundos, vírgula; y microssegundos, vírgula, decagramas vírgula; 1235 decanewtons, vírgula; 25 microssegundos, vírgula; 3234 mols")?;
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
    test("pt", "SimpleSpeak", expr, 
        "abre colchetes; 1 tonelada, vírgula; 2 petaampères, vírgula; 3 pascals, vírgula; 45 militeslas; fecha colchetes")?;
        return Ok(());

}

