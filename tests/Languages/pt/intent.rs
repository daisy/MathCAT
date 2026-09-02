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

/// Tests for rules shared between various speech styles:
/// *  this has tests focused on the various alphabets
use crate::common::*;
use anyhow::Result;


#[test]
fn silent_intent_mi() -> Result<()> {
    let expr = "<math> <mn>2</mn> <mi intent=':silent'>x</mi></math>";
    test("pt", "SimpleSpeak", expr, "2")?;
    // ClearSpeak reativado: Rules/Languages/pt/ClearSpeak_Rules.yaml traduzido.
    // Pausas (vírgula/ponto-e-vírgula) podem precisar de reconciliação via cargo test.
    test("pt", "ClearSpeak", expr, "2")?;
    return Ok(());

}

#[test]
fn silent_intent_msup() -> Result<()> {
    let expr = "<math>
        <msup intent='index:silent($H,$n)'>
            <mi arg='H' mathvariant='normal'>H</mi>
            <mn arg='n'>2</mn>
        </msup></math>";
    test("pt", "SimpleSpeak", expr, "maiúscula h 2")?;
    // ClearSpeak reativado: Rules/Languages/pt/ClearSpeak_Rules.yaml traduzido.
    // Pausas (vírgula/ponto-e-vírgula) podem precisar de reconciliação via cargo test.
    test("pt", "ClearSpeak", expr, "maiúscula h 2")?;
    return Ok(());

}

#[test]
fn silent_intent_underscore() -> Result<()> {
    let expr = "<math>
        <msup intent='_($H,$n)'>
            <mi arg='H' mathvariant='normal'>H</mi>
            <mn arg='n'>2</mn>
        </msup></math>";
    test("pt", "SimpleSpeak", expr, "maiúscula h 2")?;
    // ClearSpeak reativado: Rules/Languages/pt/ClearSpeak_Rules.yaml traduzido.
    // Pausas (vírgula/ponto-e-vírgula) podem precisar de reconciliação via cargo test.
    test("pt", "ClearSpeak", expr, "maiúscula h 2")?;
    return Ok(());

}

#[test]
fn intent_prob_x() -> Result<()> {
    let expr = "<math>
    <msup intent='$op($arg)'>
        <mi arg='arg'>x</mi>
        <mi arg='op' intent='probability' mathvariant='normal'>P</mi>
    </msup></math>";
    // ClearSpeak reativado: Rules/Languages/pt/ClearSpeak_Rules.yaml traduzido.
    // Pausas (vírgula/ponto-e-vírgula) podem precisar de reconciliação via cargo test.
    test("pt", "ClearSpeak", expr, "probabilidade de x")?;
    return Ok(());

}
