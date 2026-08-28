/// Gerador do pacote da sessão de escuta (ACHADOS 9.3).
///
/// Não é um teste de regressão: é #[ignore] e só roda quando chamado de
/// propósito por PythonScripts/gerar_pacote_escuta.py, que monta o
/// Rules/Languages/pt/SESSAO_ESCUTA.md a partir do que este arquivo imprime.
///
/// Lê tests/Languages/pt/escuta_expressoes.tsv (30 expressões, MathML gerado
/// pelo latex2mathml — o mesmo caminho do ACESSÍLIA) e imprime a fala do motor
/// nos três níveis de verbosidade, nos dois estilos. Formato de cada linha:
///     FALA<TAB>área<TAB>título<TAB>estilo<TAB>verbosidade<TAB>fala
///
/// Rodar à mão:
///     cargo test Languages::pt::escuta -- --ignored --nocapture

use crate::common::*;

fn falar(estilo: &str, verbosidade: &str, mathml: &str) -> String {
    set_rules_dir(abs_rules_dir_path()).unwrap();
    set_preference("Language", "pt".to_string()).unwrap();
    set_preference("SpeechStyle", estilo.to_string()).unwrap();
    set_preference("Verbosity", verbosidade.to_string()).unwrap();
    set_preference("SpeechOverrides_CapitalLetters", "".to_string()).unwrap();
    set_preference("ClearSpeak_SetMemberSymbol", "Auto".to_string()).unwrap();
    match set_mathml(mathml.to_string()) {
        Ok(_) => match get_spoken_text() {
            Ok(fala) => regex::Regex::new(r"  +").unwrap().replace_all(&fala, " ").to_string(),
            Err(e) => format!("[ERRO ao falar: {}]", errors_to_string(&e)),
        },
        Err(e) => format!("[ERRO no MathML: {}]", errors_to_string(&e)),
    }
}

#[test]
#[ignore = "gerador do pacote de escuta; rode com --ignored --nocapture"]
fn gerar_pacote_escuta() {
    let tsv = include_str!("escuta_expressoes.tsv");
    for linha in tsv.lines().filter(|l| !l.starts_with('#') && !l.trim().is_empty()) {
        let campos: Vec<&str> = linha.split('\t').collect();
        assert_eq!(campos.len(), 4, "linha malformada no tsv: {linha}");
        let (area, titulo, mathml) = (campos[0], campos[1], campos[3]);
        for estilo in ["ClearSpeak", "SimpleSpeak"] {
            for verb in ["Terse", "Medium", "Verbose"] {
                println!("FALA\t{area}\t{titulo}\t{estilo}\t{verb}\t{}", falar(estilo, verb, mathml));
            }
        }
    }
}
