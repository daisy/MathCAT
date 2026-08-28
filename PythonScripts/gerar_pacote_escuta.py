#!/usr/bin/env python3
# ============================================================================
# gerar_pacote_escuta.py — monta Rules/Languages/pt/SESSAO_ESCUTA.md
#
# É o único risco que teste nenhum cobre: o que o motor produz é português
# correto no papel, mas ninguém ouviu. Este pacote é para uma sessão com um
# ouvinte de leitor de tela: 30 expressões (MathML gerado pelo latex2mathml,
# o mesmo caminho do ACESSÍLIA), a fala nos três níveis de verbosidade e nos
# dois estilos, o roteiro de perguntas e as decisões que dependem dele.
#
# Uso, a partir da raiz do repositório:
#     python3 PythonScripts/gerar_pacote_escuta.py
# (roda `cargo test Languages::pt::escuta -- --ignored --nocapture` por baixo)
# ============================================================================

from __future__ import annotations

import subprocess
import sys
from collections import defaultdict
from pathlib import Path

RAIZ = Path(__file__).resolve().parent.parent
TSV = RAIZ / "tests" / "Languages" / "pt" / "escuta_expressoes.tsv"
SAIDA = RAIZ / "Rules" / "Languages" / "pt" / "SESSAO_ESCUTA.md"

AREAS = {
    "aritmetica": "Aritmética",
    "conjuntos_logica": "Conjuntos e lógica",
    "calculo": "Cálculo",
    "algebra_linear": "Álgebra Linear",
    "quimica": "Química",
    "estatistica": "Estatística",
}

ROTEIRO = """
## Como conduzir

Uma expressão por vez. Leia a fala **em voz alta, com o leitor de tela**, não
com a sua voz — o que está em teste é a pronúncia do sintetizador sobre estas
palavras, tanto quanto as palavras. Comece pelo nível **Medium** (é o padrão
do MathCAT); use Verbose e Terse só nas que gerarem dúvida.

Para cada expressão, na ordem, sem mostrar a fórmula antes:

1. **Reescreva o que ouviu.** (A pergunta mais valiosa. Entregue papel ou
   editor e peça a expressão como o ouvinte a entendeu. Só depois mostre o
   LaTeX. Onde a reescrita divergir, anote *o que* divergiu — expoente,
   limite da fração, sinal, agrupamento.)
2. **Onde começa e onde termina a fração?** (Nas expressões com fração
   aninhada — Bhaskara, fração de fração, variância — o ouvinte consegue dizer
   o que está no numerador e o que está no denominador?)
3. **Qual era o expoente?** (Nas que têm potência: "ao quadrado", "elevado a
   x", "elevado a menos 1". Pergunte também o *que* está elevado: "e elevado a
   x" ou "x e, elevado a x"?)
4. **Soou picotada?** (Muitas pausas — ver ACHADOS 1 e 2: as pausas são
   calculadas em bytes e o português é mais longo que o inglês. Na química é
   onde mais aparece.)
5. **Alguma palavra soou estrangeira ou estranha?** (Anote a palavra exata.
   É aqui que "fraktur", "vazado", "colchete angular", "mho" e "angstrom"
   vão aparecer, se aparecerem.)
6. **Faltou alguma coisa?** (Algo que estava na fórmula e não foi dito, ou
   foi dito e não estava.)

Registre as respostas na tabela ao fim deste arquivo.

## Decisões que dependem desta sessão

Estas estão em `TERMINOLOGIA_PT_BR.md` com status **PENDENTE DE ESCUTA**. A
sessão é o que as fecha.

| decisão | como está hoje | alternativa | onde ouvir |
|---|---|---|---|
| **fraktur × gótico** | "fraktur" (ex.: "maiúscula r fraktur") | "gótico" | não há fraktur nas 30 expressões de propósito — é raro em graduação. Pergunte diretamente: "se eu disser *maiúscula a fraktur*, você entende que letra é?" |
| **vazado × traço duplo** | "vazado" (ℝ é falado "números reais"; 𝔸 solto é "maiúscula a vazado") | "traço duplo", "duplo traço" | Sistema (ℝⁿ), Continência (ℕ), Pertinência (ℤ) — mas note que os conjuntos numéricos nomeados **não** usam a palavra; só letras avulsas usam |
| **maiúscula anteposta** | "maiúscula a" (antes da letra) | "a maiúscula" (depois) | qualquer expressão com A, B, N, P: Matriz, União e interseção, Probabilidade condicional |
| **pausas da química** | vírgula a cada elemento | menos pausas (exige mudança no motor, ver ACHADOS 3) | as cinco de Química |
| **"x barra"** para x̄ | "x barra" | "x com barra", "média de x" | Média amostral, Normal |
| **"colchete angular"** para ⟨u, v⟩ | "abre colchete angular, u vírgula, v, fecha colchete angular" | "produto interno de u e v" (exige inferência de intent, não só palavra) | Produto interno e norma |
| **"varia com"** para ∼ | "varia com" | "tem distribuição", "segue" (em estatística ∼ é distribuição, não proporcionalidade) | Normal |
| **"seta para a direita"** em f: ℝ → ℝ | "números reais seta para a direita números reais" | "em", "para" | não está nas 30; pergunte se surgir |
| **∼ / ≃ / ≈** | "varia com" / "é assintoticamente igual a" / "é aproximadamente igual a" | — | Normal |

## Registro das respostas

| # | expressão | reescreveu certo? | fração ok? | expoente ok? | picotada? | palavra estranha | faltou/sobrou |
|---|---|---|---|---|---|---|---|
"""


def falas() -> dict[tuple[str, str], dict[tuple[str, str], str]]:
    r = subprocess.run(
        ["cargo", "test", "Languages::pt::escuta", "--", "--ignored", "--nocapture"],
        cwd=RAIZ, capture_output=True, text=True,
    )
    out: dict[tuple[str, str], dict[tuple[str, str], str]] = defaultdict(dict)
    for l in r.stdout.splitlines():
        if not l.startswith("FALA\t"):
            continue
        _, area, titulo, estilo, verb, fala = l.split("\t", 5)
        out[(area, titulo)][(estilo, verb)] = fala
    if not out:
        print(r.stdout[-2000:], r.stderr[-2000:], file=sys.stderr)
        raise SystemExit("o gerador não imprimiu nada — veja acima")
    return out


def main() -> int:
    exprs = []
    for l in TSV.read_text(encoding="utf-8").splitlines():
        if l.startswith("#") or not l.strip():
            continue
        area, titulo, latex, mathml = l.split("\t", 3)
        exprs.append((area, titulo, latex, mathml))
    f = falas()

    md = ["# Sessão de escuta — MathCAT pt-BR", "",
          "Pacote gerado por `PythonScripts/gerar_pacote_escuta.py`. Não edite à mão:",
          "regenere depois de qualquer mudança nas regras, para a fala aqui ser a fala real.", "",
          f"30 expressões, 6 áreas, MathML produzido pelo `latex2mathml` (o caminho do ACESSÍLIA).",
          "Para cada uma: LaTeX, MathML, e a fala do motor em ClearSpeak e SimpleSpeak,",
          "nos três níveis de verbosidade (Terse / Medium / Verbose).", "",
          ROTEIRO.split("## Registro das respostas")[0]]
    n = 0
    for area, nome in AREAS.items():
        md.append(f"\n## {nome}\n")
        for a, titulo, latex, mathml in exprs:
            if a != area:
                continue
            n += 1
            md.append(f"### {n}. {titulo}\n")
            md.append(f"`{latex}`\n")
            md.append("<details><summary>MathML</summary>\n\n```xml\n" + mathml + "\n```\n</details>\n")
            md.append("| estilo | verbosidade | fala |\n|---|---|---|")
            for estilo in ("ClearSpeak", "SimpleSpeak"):
                for verb in ("Terse", "Medium", "Verbose"):
                    fala = f[(a, titulo)].get((estilo, verb), "(sem saída)")
                    md.append(f"| {estilo} | {verb} | {fala} |")
            md.append("")
    md.append("## Registro das respostas\n")
    md.append("| # | expressão | reescreveu certo? | fração ok? | expoente ok? | picotada? | palavra estranha | faltou/sobrou |")
    md.append("|---|---|---|---|---|---|---|---|")
    n = 0
    for area in AREAS:
        for a, titulo, _, _ in exprs:
            if a == area:
                n += 1
                md.append(f"| {n} | {titulo} |  |  |  |  |  |  |")
    SAIDA.write_text("\n".join(md) + "\n", encoding="utf-8")
    print(f"escrito: {SAIDA} ({n} expressões)")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
