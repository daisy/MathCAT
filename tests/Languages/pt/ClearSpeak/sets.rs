/// Testes de conjuntos e pertinência — pt-BR, ClearSpeak e SimpleSpeak.
///
/// POR QUE ESTE ARQUIVO EXISTE
/// O par ∈ / ∊ falou "é membro de" com as preferências padrão durante duas
/// rodadas inteiras sem ninguém notar, porque o pt não tinha NENHUM teste de
/// pertinência — nem ClearSpeak, nem SimpleSpeak. O ∉ foi corrigido sozinho
/// numa rodada e os outros dois ficaram para trás. Ver ACHADOS 7.8.
///
/// A invariante que este arquivo trava: **∈ (0x2208) e ∊ (0x220a) são
/// variantes de forma do mesmo símbolo e devem falar igual em todos os ramos**
/// — nas cinco preferências de ClearSpeak_SetMemberSymbol, dentro e fora de
/// um conjunto, nos dois estilos de fala.
///
/// COMO AS EXPECTATIVAS FORAM ESCRITAS
/// Todas saíram da saída real do motor, nunca de tradução do inglês. O método
/// é o do cabeçalho de tests/Languages/pt/shared.rs: rodar, comparar `left`
/// com `right`, e se o gerado estiver certo em português, copiá-lo; se estiver
/// errado, consertar a REGRA, nunca o teste.
///
/// Cada teste de pertinência fixa ClearSpeak_SetMemberSymbol explicitamente,
/// inclusive no caso 'Auto'. Isso é de propósito: as preferências de
/// ClearSpeak não são reinicializadas entre testes na mesma thread, então um
/// teste que dependesse do padrão implícito poderia passar ou falhar conforme
/// a ordem de execução.

use crate::common::*;
use anyhow::Result;

// x <símbolo> ℝ — pertinência FORA de um conjunto (ramo else_test das regras)
fn sozinho(simbolo: &str) -> String {
    format!("<math><mi>x</mi><mo>{simbolo}</mo><mi>ℝ</mi></math>")
}

// { x <símbolo> ℤ : x > 5 } — pertinência DENTRO de um conjunto (ramo then_test)
fn no_conjunto(simbolo: &str) -> String {
    format!("<math>
                <mo>{{</mo>
                <mi>x</mi> <mo>{simbolo}</mo> <mi>ℤ</mi>
                <mo>:</mo> <mi>x</mi> <mo>&#x003E;</mo> <mn>5</mn>
                <mo>}}</mo>
            </math>")
}

const PERTENCE: &str = "\u{2208}";      // ∈
const NAO_PERTENCE: &str = "\u{2209}";  // ∉
const PERTENCE_PEQ: &str = "\u{220A}";  // ∊ — variante de forma do 0x2208

/// ∈ e ∊ têm de falar igual. É o caso que sobreviveu duas rodadas.
fn ambas_variantes(pref: &str, expr: fn(&str) -> String, esperado: &str) -> Result<()> {
    test_ClearSpeak("pt", "ClearSpeak_SetMemberSymbol", pref, &expr(PERTENCE), esperado)?;
    test_ClearSpeak("pt", "ClearSpeak_SetMemberSymbol", pref, &expr(PERTENCE_PEQ), esperado)?;
    return Ok(());
}

// ---------------------------------------------------------------------------
// Conjuntos numéricos
// ---------------------------------------------------------------------------

#[test]
fn complexos() -> Result<()> {
    test("pt", "ClearSpeak", "<math><mi>ℂ</mi></math>", "números complexos")?;
    return Ok(());
}

#[test]
fn naturais() -> Result<()> {
    test("pt", "ClearSpeak", "<math><mi>ℕ</mi></math>", "números naturais")?;
    return Ok(());
}

#[test]
fn racionais() -> Result<()> {
    test("pt", "ClearSpeak", "<math><mi>ℚ</mi></math>", "números racionais")?;
    return Ok(());
}

#[test]
fn reais() -> Result<()> {
    test("pt", "ClearSpeak", "<math><mi>ℝ</mi></math>", "números reais")?;
    return Ok(());
}

// Sem "números", diferente dos quatro acima. O inglês tem a mesma assimetria
// ("the integers" contra "the complex numbers"), então isto acompanha o en.
#[test]
fn inteiros() -> Result<()> {
    test("pt", "ClearSpeak", "<math><mi>ℤ</mi></math>", "inteiros")?;
    return Ok(());
}

#[test]
fn conjuntos_numericos_iguais_em_simplespeak() -> Result<()> {
    test("pt", "SimpleSpeak", "<math><mi>ℂ</mi></math>", "números complexos")?;
    test("pt", "SimpleSpeak", "<math><mi>ℕ</mi></math>", "números naturais")?;
    test("pt", "SimpleSpeak", "<math><mi>ℚ</mi></math>", "números racionais")?;
    test("pt", "SimpleSpeak", "<math><mi>ℝ</mi></math>", "números reais")?;
    test("pt", "SimpleSpeak", "<math><mi>ℤ</mi></math>", "inteiros")?;
    return Ok(());
}

// ---------------------------------------------------------------------------
// Conjunto vazio, por extensão, por compreensão
// ---------------------------------------------------------------------------

#[test]
fn conjunto_vazio_por_chaves() -> Result<()> {
    test("pt", "ClearSpeak", "<math><mo>{</mo> <mo>}</mo></math>", "o conjunto vazio")?;
    test("pt", "SimpleSpeak", "<math><mo>{</mo> <mo>}</mo></math>", "o conjunto vazio")?;
    return Ok(());
}

// Sem artigo, ao contrário das chaves vazias acima. O en faz a mesma distinção
// ('empty set' para ∅, 'the empty set' para {}), então isto acompanha o en.
#[test]
fn conjunto_vazio_por_simbolo() -> Result<()> {
    test("pt", "ClearSpeak", "<math><mi>∅</mi></math>", "conjunto vazio")?;
    test("pt", "SimpleSpeak", "<math><mi>∅</mi></math>", "conjunto vazio")?;
    return Ok(());
}

#[test]
fn extensao_um_elemento() -> Result<()> {
    let expr = "<math><mo>{</mo> <mn>12</mn> <mo>}</mo></math>";
    test("pt", "ClearSpeak", expr, "o conjunto 12")?;
    test("pt", "SimpleSpeak", expr, "o conjunto 12")?;
    return Ok(());
}

#[test]
fn extensao_varios_elementos() -> Result<()> {
    let expr = "<math>
                <mo>{</mo> <mn>5</mn> <mo>,</mo> <mn>10</mn> <mo>,</mo> <mn>15</mn> <mo>}</mo>
            </math>";
    test("pt", "ClearSpeak", expr, "o conjunto 5 vírgula, 10 vírgula, 15")?;
    test("pt", "SimpleSpeak", expr, "o conjunto 5 vírgula, 10 vírgula, 15")?;
    return Ok(());
}

#[test]
fn compreensao_com_dois_pontos() -> Result<()> {
    let expr = "<math>
                <mo>{</mo> <mrow><mi>x</mi><mo>:</mo><mi>x</mi><mo>&#x003E;</mo><mn>2</mn></mrow> <mo>}</mo>
            </math>";
    test("pt", "ClearSpeak", expr, "o conjunto de todos os x tal que x é maior que 2")?;
    return Ok(());
}

// A barra vertical tem de falar igual aos dois pontos.
#[test]
fn compreensao_com_barra() -> Result<()> {
    let expr = "<math>
                <mo>{</mo> <mrow><mi>x</mi><mo>|</mo><mi>x</mi><mo>&#x003E;</mo><mn>2</mn></mrow> <mo>}</mo>
            </math>";
    test("pt", "ClearSpeak", expr, "o conjunto de todos os x tal que x é maior que 2")?;
    test("pt", "SimpleSpeak", expr, "o conjunto de todos os x tal que x é maior que 2")?;
    return Ok(());
}

// ---------------------------------------------------------------------------
// União e interseção
// ---------------------------------------------------------------------------

#[test]
fn uniao() -> Result<()> {
    let expr = "<math><mi>A</mi><mo>∪</mo><mi>B</mi></math>";
    test("pt", "ClearSpeak", expr, "maiúscula a união maiúscula b")?;
    test("pt", "SimpleSpeak", expr, "maiúscula a união maiúscula b")?;
    return Ok(());
}

#[test]
fn intersecao() -> Result<()> {
    let expr = "<math><mi>A</mi><mo>∩</mo><mi>B</mi></math>";
    test("pt", "ClearSpeak", expr, "maiúscula a interseção maiúscula b")?;
    test("pt", "SimpleSpeak", expr, "maiúscula a interseção maiúscula b")?;
    return Ok(());
}

// ---------------------------------------------------------------------------
// Pertinência FORA de um conjunto — as cinco preferências
// ---------------------------------------------------------------------------

#[test]
fn pertinencia_auto() -> Result<()> {
    ambas_variantes("Auto", sozinho, "x pertence a números reais")
}

#[test]
fn pertinencia_member() -> Result<()> {
    ambas_variantes("Member", sozinho, "x é membro de números reais")
}

#[test]
fn pertinencia_element() -> Result<()> {
    ambas_variantes("Element", sozinho, "x é um elemento de, números reais")
}

#[test]
fn pertinencia_belongs() -> Result<()> {
    ambas_variantes("Belongs", sozinho, "x pertence a números reais")
}

#[test]
fn pertinencia_in() -> Result<()> {
    ambas_variantes("In", sozinho, "x está em números reais")
}

// ---------------------------------------------------------------------------
// Pertinência DENTRO de um conjunto — as cinco preferências
// ---------------------------------------------------------------------------

#[test]
fn pertinencia_no_conjunto_auto() -> Result<()> {
    ambas_variantes("Auto", no_conjunto,
        "o conjunto de todos os x em inteiros tal que x é maior que 5")
}

#[test]
fn pertinencia_no_conjunto_member() -> Result<()> {
    ambas_variantes("Member", no_conjunto,
        "o conjunto de todos os x membros de inteiros tal que x é maior que 5")
}

#[test]
fn pertinencia_no_conjunto_element() -> Result<()> {
    ambas_variantes("Element", no_conjunto,
        "o conjunto de todos os x elementos de inteiros tal que x é maior que 5")
}

// Auto e In coincidem dentro de um conjunto, como no inglês ('Auto or In').
#[test]
fn pertinencia_no_conjunto_in() -> Result<()> {
    ambas_variantes("In", no_conjunto,
        "o conjunto de todos os x em inteiros tal que x é maior que 5")
}

// ---------------------------------------------------------------------------
// Não-pertinência
// ---------------------------------------------------------------------------

#[test]
fn nao_pertinencia_auto() -> Result<()> {
    test_ClearSpeak("pt", "ClearSpeak_SetMemberSymbol", "Auto",
        &sozinho(NAO_PERTENCE), "x não pertence a números reais")?;
    return Ok(());
}

#[test]
fn nao_pertinencia_member() -> Result<()> {
    test_ClearSpeak("pt", "ClearSpeak_SetMemberSymbol", "Member",
        &sozinho(NAO_PERTENCE), "x não é membro de, números reais")?;
    return Ok(());
}

#[test]
fn nao_pertinencia_element() -> Result<()> {
    test_ClearSpeak("pt", "ClearSpeak_SetMemberSymbol", "Element",
        &sozinho(NAO_PERTENCE), "x não é um elemento de, números reais")?;
    return Ok(());
}

#[test]
fn nao_pertinencia_belongs() -> Result<()> {
    test_ClearSpeak("pt", "ClearSpeak_SetMemberSymbol", "Belongs",
        &sozinho(NAO_PERTENCE), "x não pertence a números reais")?;
    return Ok(());
}

// Auto e In coincidem na negação também, como no positivo. E aqui coincidem
// também com Belongs: "não em" é truncado e "fora de" troca o conceito
// (localização, não pertinência), então a negação nominal é uma só.
// Ver TERMINOLOGIA_PT_BR.md.
#[test]
fn nao_pertinencia_no_conjunto_auto() -> Result<()> {
    test_ClearSpeak("pt", "ClearSpeak_SetMemberSymbol", "Auto", &no_conjunto(NAO_PERTENCE),
        "o conjunto de todos os x não pertencentes a inteiros tal que x é maior que 5")?;
    return Ok(());
}

#[test]
fn nao_pertinencia_no_conjunto_member() -> Result<()> {
    test_ClearSpeak("pt", "ClearSpeak_SetMemberSymbol", "Member", &no_conjunto(NAO_PERTENCE),
        "o conjunto de todos os x não membros de inteiros tal que x é maior que 5")?;
    return Ok(());
}

#[test]
fn nao_pertinencia_no_conjunto_element() -> Result<()> {
    test_ClearSpeak("pt", "ClearSpeak_SetMemberSymbol", "Element", &no_conjunto(NAO_PERTENCE),
        "o conjunto de todos os x não elementos de inteiros tal que x é maior que 5")?;
    return Ok(());
}

#[test]
fn nao_pertinencia_no_conjunto_in() -> Result<()> {
    test_ClearSpeak("pt", "ClearSpeak_SetMemberSymbol", "In", &no_conjunto(NAO_PERTENCE),
        "o conjunto de todos os x não pertencentes a inteiros tal que x é maior que 5")?;
    return Ok(());
}

// ---------------------------------------------------------------------------
// SimpleSpeak — a preferência de ClearSpeak não vale aqui
// ---------------------------------------------------------------------------

#[test]
fn pertinencia_simplespeak() -> Result<()> {
    test("pt", "SimpleSpeak", &sozinho(PERTENCE), "x pertence a números reais")?;
    test("pt", "SimpleSpeak", &sozinho(PERTENCE_PEQ), "x pertence a números reais")?;
    return Ok(());
}

#[test]
fn nao_pertinencia_simplespeak() -> Result<()> {
    test("pt", "SimpleSpeak", &sozinho(NAO_PERTENCE), "x não pertence a números reais")?;
    return Ok(());
}

// ---------------------------------------------------------------------------
// Pertinência sob um operador grande (não é "dentro de um conjunto")
// ---------------------------------------------------------------------------

#[test]
fn pertinencia_sob_somatorio() -> Result<()> {
    let expr = "<math>
            <munder>
                <mo>∑</mo>
                <mrow> <mi>i</mi> <mo>∈</mo> <mi>ℤ</mi> </mrow>
            </munder>
            <mfrac>
                <mn>1</mn>
                <mrow> <msup> <mi>i</mi> <mn>2</mn> </msup> </mrow>
            </mfrac>
        </math>";
    test_ClearSpeak("pt", "ClearSpeak_SetMemberSymbol", "Auto", expr,
        "o somatório sobre i pertence a inteiros de; a fração com numerador 1; e denominador i ao quadrado")?;
    return Ok(());
}

// ---------------------------------------------------------------------------
// Ramos que exigem forma não-finita (antes falavam com verbo finito)
//
// Dentro de um conjunto a expressão modifica "todos os x". Verbo finito ali
// gruda em "conjunto" ("o conjunto de todos os x PERTENCE A inteiros") e a
// concordância cai no lugar errado. O inglês usa 'belonging to' justamente
// para evitar isso. Ver ACHADOS 7.9.
// ---------------------------------------------------------------------------

#[test]
fn pertinencia_no_conjunto_belongs() -> Result<()> {
    ambas_variantes("Belongs", no_conjunto,
        "o conjunto de todos os x pertencentes a inteiros tal que x é maior que 5")
}

#[test]
fn nao_pertinencia_no_conjunto_belongs() -> Result<()> {
    test_ClearSpeak("pt", "ClearSpeak_SetMemberSymbol", "Belongs", &no_conjunto(NAO_PERTENCE),
        "o conjunto de todos os x não pertencentes a inteiros tal que x é maior que 5")?;
    return Ok(());
}

// O SimpleSpeak usava UMA string só para dentro e fora do conjunto, então não
// conseguia distinguir os dois contextos. Agora bifurca como o ClearSpeak.
#[test]
fn pertinencia_no_conjunto_simplespeak() -> Result<()> {
    test("pt", "SimpleSpeak", &no_conjunto(PERTENCE),
        "o conjunto de todos os x pertencentes a inteiros tal que x é maior que 5")?;
    test("pt", "SimpleSpeak", &no_conjunto(PERTENCE_PEQ),
        "o conjunto de todos os x pertencentes a inteiros tal que x é maior que 5")?;
    return Ok(());
}

#[test]
fn nao_pertinencia_no_conjunto_simplespeak() -> Result<()> {
    test("pt", "SimpleSpeak", &no_conjunto(NAO_PERTENCE),
        "o conjunto de todos os x não pertencentes a inteiros tal que x é maior que 5")?;
    return Ok(());
}

/// Fora de um conjunto o verbo finito está certo (o sujeito é um termo só), mas
/// o ∉ com 'In' falava "não está contido em" — exatamente a fala do ⊄
/// (unicode.yaml, 0x2284). Duas relações distintas soavam idênticas. Agora
/// espelha o "está em" do ∈, que é o que o próprio ∉ já dizia num conjunto.
#[test]
fn nao_pertinencia_in_nao_colide_com_subconjunto() -> Result<()> {
    test_ClearSpeak("pt", "ClearSpeak_SetMemberSymbol", "In",
        &sozinho(NAO_PERTENCE), "x não está em números reais")?;
    return Ok(());
}

/// Guarda da distinção: ∉ e ⊄ não podem falar igual.
#[test]
fn nao_pertinencia_e_nao_subconjunto_falam_diferente() -> Result<()> {
    test_ClearSpeak("pt", "ClearSpeak_SetMemberSymbol", "In",
        "<math><mi>x</mi><mo>\u{2209}</mo><mi>ℝ</mi></math>", "x não está em números reais")?;
    test("pt", "ClearSpeak",
        "<math><mi>x</mi><mo>\u{2284}</mo><mi>ℝ</mi></math>", "x não está contido em, números reais")?;
    return Ok(());
}
