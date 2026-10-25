/// TESTES DO IDIOMA pt — decisões de terminologia registradas em
/// Rules/Languages/pt/ACHADOS.md, seção 8.
///
/// Este arquivo cobre duas decisões que antes não tinham teste nenhum:
///   * 8.2 — numerais romanos Unicode (0x2160-0x217F) falam o VALOR;
///   * 8.3 — os expoentes ⁰ e ⁱ usam a forma cardinal.
/// e fixa o comportamento ASCII que foi decidido PRESERVAR, para que uma
/// mudança futura no caminho de inferência de contexto não passe em silêncio.
///
/// Todas as expectativas abaixo foram capturadas da saída real do motor, sob
/// as preferências do próprio harness — não foram escritas a partir do que se
/// esperava que saísse.
use crate::common::*;
use anyhow::Result;

// ---------------------------------------------------------------------------
// 8.2 — Numerais romanos Unicode: falar o valor, não a grafia.
// Antes desta decisão a saída era a soletração ("I V", "X I I", "M").
// ---------------------------------------------------------------------------

#[test]
fn numerais_romanos_unicode_falam_o_valor() -> Result<()> {
    // Representativos da faixa 0x2160-0x216F (maiúsculas).
    let expr = "<math><mi>Ⅰ</mi></math>"; // 0x2160
    test("pt", "SimpleSpeak", expr, "um")?;
    let expr = "<math><mi>Ⅳ</mi></math>"; // 0x2163 — o caso que motivou a decisão
    test("pt", "SimpleSpeak", expr, "quatro")?;
    let expr = "<math><mi>Ⅻ</mi></math>"; // 0x216b
    test("pt", "SimpleSpeak", expr, "doze")?;
    let expr = "<math><mi>Ⅼ</mi></math>"; // 0x216c
    test("pt", "SimpleSpeak", expr, "cinquenta")?;
    let expr = "<math><mi>Ⅽ</mi></math>"; // 0x216d
    test("pt", "SimpleSpeak", expr, "cem")?;
    let expr = "<math><mi>Ⅾ</mi></math>"; // 0x216e
    test("pt", "SimpleSpeak", expr, "quinhentos")?;
    let expr = "<math><mi>Ⅿ</mi></math>"; // 0x216f
    test("pt", "SimpleSpeak", expr, "mil")?;
    return Ok(());
}

#[test]
fn numerais_romanos_unicode_minusculos() -> Result<()> {
    // A metade minúscula da faixa (0x2170-0x217F) tem os mesmos valores.
    let expr = "<math><mi>ⅳ</mi></math>"; // 0x2173
    test("pt", "SimpleSpeak", expr, "quatro")?;
    let expr = "<math><mi>ⅿ</mi></math>"; // 0x217f
    test("pt", "SimpleSpeak", expr, "mil")?;
    return Ok(());
}

#[test]
fn numerais_romanos_unicode_em_expressao() -> Result<()> {
    // O valor também é falado quando o numeral aparece dentro de uma conta.
    let expr = "<math><mi>Ⅳ</mi><mo>+</mo><mi>Ⅻ</mi></math>";
    test("pt", "SimpleSpeak", expr, "quatro mais doze")?;
    // A decisão não depende do estilo de fala.
    test("pt", "ClearSpeak", expr, "quatro mais doze")?;
    return Ok(());
}

#[test]
fn numerais_romanos_unicode_em_mn() -> Result<()> {
    // O caractere dedicado vale como numeral tanto em <mi> quanto em <mn>.
    let expr = "<math><mn>Ⅳ</mn></math>";
    test("pt", "SimpleSpeak", expr, "quatro")?;
    return Ok(());
}

// ---------------------------------------------------------------------------
// 8.3 — Expoentes ⁰ e ⁱ na forma cardinal.
// Antes: "elevado à potência zero" / "elevado à potência i".
// ---------------------------------------------------------------------------

#[test]
fn expoente_zero_e_i_forma_cardinal() -> Result<()> {
    let expr = "<math><mi>x</mi><mi>⁰</mi></math>"; // 0x2070
    test("pt", "SimpleSpeak", expr, "x elevado a zero")?;
    let expr = "<math><mi>x</mi><mi>ⁱ</mi></math>"; // 0x2071
    test("pt", "SimpleSpeak", expr, "x elevado a i")?;
    // A decisão não depende do estilo de fala.
    let expr = "<math><mi>x</mi><mi>⁰</mi></math>";
    test("pt", "ClearSpeak", expr, "x elevado a zero")?;
    return Ok(());
}

#[test]
fn expoente_zero_e_i_isolados() -> Result<()> {
    let expr = "<math><mi>⁰</mi></math>";
    test("pt", "SimpleSpeak", expr, "elevado a zero")?;
    let expr = "<math><mi>ⁱ</mi></math>";
    test("pt", "SimpleSpeak", expr, "elevado a i")?;
    return Ok(());
}

#[test]
fn expoentes_ordinais_vizinhos_intactos() -> Result<()> {
    // As entradas vizinhas continuam na forma ORDINAL: a decisão 8.3 valeu só
    // para ⁰ e ⁱ, que não têm ordinal. Este teste existe para impedir que
    // alguém "uniformize" o bloco inteiro por engano (ver ACHADOS 5.2).
    let expr = "<math><mi>x</mi><mi>⁴</mi></math>"; // 0x2074
    test("pt", "SimpleSpeak", expr, "x elevado à quarta potência")?;
    let expr = "<math><mi>x</mi><mi>ⁿ</mi></math>"; // 0x207f
    test("pt", "SimpleSpeak", expr, "x elevado à enésima potência")?;
    return Ok(());
}

// ---------------------------------------------------------------------------
// GUARDA DE REGRESSÃO — o caminho ASCII foi decidido intocado (ACHADOS 8.2).
//
// Numerais romanos escritos com letras ASCII NÃO passam pelo unicode-full.yaml:
// são marcados por src/canonicalize.rs com `data-roman-numeral` e falados pelas
// regras `default` de <mn>/<mi> em SharedRules/default.yaml, que soletram.
// Esse é o mesmo ramo de regra que trata I e V como identificador ou variável.
//
// Se algum dia o valor passar a ser falado também no caminho ASCII, ESTES
// testes vão falhar — e é essa a intenção: a mudança tem que ser deliberada.
// ---------------------------------------------------------------------------

#[test]
fn romanos_ascii_continuam_soletrados_nao_viram_valor() -> Result<()> {
    // XIV NÃO deve virar "quatorze".
    let expr = "<math><mi>XIV</mi></math>";
    test("pt", "SimpleSpeak", expr, "maiúscula x maiúscula i maiúscula v")?;
    let expr = "<math><mn>XIV</mn></math>";
    test("pt", "SimpleSpeak", expr, "maiúscula x maiúscula i maiúscula v")?;
    // Também não depende do estilo de fala.
    let expr = "<math><mi>XIV</mi></math>";
    test("pt", "ClearSpeak", expr, "maiúscula x maiúscula i maiúscula v")?;
    // MIX (1009 em romano) é o caso clássico de colisão com palavra.
    let expr = "<math><mi>MIX</mi></math>";
    test("pt", "SimpleSpeak", expr, "maiúscula m maiúscula i maiúscula x")?;
    // Minúsculas idem: "xiv" não vira "quatorze".
    let expr = "<math><mn>xiv</mn></math>";
    test("pt", "SimpleSpeak", expr, "x i v")?;
    return Ok(());
}

#[test]
fn letras_ascii_isoladas_continuam_identificadores() -> Result<()> {
    // I e V sozinhos são identificadores/variáveis, não os romanos 1 e 5.
    // Este é o caminho que a decisão 8.2 mandou explicitamente NÃO tocar.
    let expr = "<math><mi>I</mi></math>";
    test("pt", "SimpleSpeak", expr, "maiúscula i")?;
    let expr = "<math><mi>V</mi></math>";
    test("pt", "SimpleSpeak", expr, "maiúscula v")?;
    // Separados, continuam três identificadores — não o numeral 14.
    let expr = "<math><mi>X</mi><mi>I</mi><mi>V</mi></math>";
    test("pt", "SimpleSpeak", expr, "maiúscula x maiúscula i maiúscula v")?;
    return Ok(());
}

#[test]
fn romano_ascii_em_estado_de_oxidacao_intacto() -> Result<()> {
    // O caminho da química (estado de oxidação) usa romanos ASCII e tem o seu
    // próprio tratamento em src/chemistry.rs. A decisão 8.2 não o alcança.
    let expr = "<math><mn>IV</mn></math>";
    test("pt", "SimpleSpeak", expr, "maiúscula i maiúscula v")?;
    return Ok(());
}
