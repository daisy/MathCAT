#!/usr/bin/env python3
# ============================================================================
# gerar_recuo_en.py — recuo controlado para caracteres sem regra em português.
#
# O problema: um caractere sem regra sai CRU para o sintetizador (ver
# src/speech.rs, replace_single_char: se não está em nenhuma tabela, devolve o
# próprio caractere). O que o sintetizador faz com "⟪" ou "𝚨" é imprevisível.
# Trocar isso no motor seria mudança em src/. Este script faz o recuo só em
# regra: copia para o fim do unicode-full.yaml do pt as entradas do inglês
# cujos codepoints o pt ainda não cobre — em inglês mesmo, marcadas `t:`
# minúsculo, para a auditoria continuar contando o que falta traduzir.
#
# Duas coisas o script NÃO copia às cegas:
#   1. palavras de estilo já DEFINIDAS em TERMINOLOGIA_PT_BR.md (bold →
#      negrito, script → caligráfico, double-struck → vazado ...) — senão a
#      conferência de vocabulário acusaria o mesmo conceito com duas palavras;
#   2. textos que o pt já traduziu em OUTRO codepoint (ex.: as cópias MathType
#      na área de uso privado) — reaproveita a tradução existente.
#
# O bloco fica entre marcadores e é regenerado inteiro a cada execução.
#
# Uso, a partir da raiz do repositório:
#     python3 PythonScripts/gerar_recuo_en.py            # aplica
#     python3 PythonScripts/gerar_recuo_en.py --contar   # só relata
# ============================================================================

from __future__ import annotations

import argparse
import re
import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))
from conferir_vocabulario import Regra, expandir, indexar, ler_regras  # noqa: E402

INICIO = "# ==== RECUO CONTROLADO (gerado por PythonScripts/gerar_recuo_en.py) ===="
FIM = "# ==== FIM DO RECUO CONTROLADO ===="

# Palavras de estilo com decisão registrada em TERMINOLOGIA_PT_BR.md.
ESTILO = {
    "bold": "negrito",
    "italic": "itálico",
    "bold italic": "negrito itálico",
    "script": "caligráfico",
    "fraktur": "fraktur",
    "double-struck": "vazado",
    "double struck": "vazado",
    "sans-serif": "sem serifa",
    "sans serif": "sem serifa",
    "monospace": "monoespaçado",
    "cap": "maiúscula",
    # decorações de letra/dígito que a varredura dos testes mostrou vazando
    "circled": "circulado",
    "black circled": "circulado preto",
    "double circled": "duplo circulado",
    "parenthesized": "entre parênteses",
    "with period": "com ponto",
    "turned": "invertido",
    "turned sans-serif": "invertido sem serifa",
}

T_RE = re.compile(r'(?<![A-Za-z_])([Tt]|OT|ot):\s*"([^"]*)"')


def texto_de(regra: Regra) -> str:
    return " ".join(t for _, t, _ in regra.falas).strip()


def mapa_traducoes(pt_regras: list[Regra], en_idx: dict[str, Regra]) -> dict[str, str]:
    """en_texto -> pt_texto, para textos simples que o pt já traduziu em algum codepoint."""
    mapa: dict[str, str] = {}
    for r in pt_regras:
        if r.ramificada or len(r.falas) != 1:
            continue
        for c in expandir(r.chave):
            e = en_idx.get(c)
            if e and not e.ramificada and len(e.falas) == 1:
                mapa.setdefault(texto_de(e), texto_de(r))
    return mapa


def traduzir_texto(txt: str, mapa: dict[str, str]) -> tuple[str, bool]:
    """Devolve (texto, traduzido?)."""
    if txt in mapa:
        return mapa[txt], True
    if txt in ESTILO:
        return ESTILO[txt], True
    return txt, False


def reescrever_entrada(linhas: list[str], mapa: dict[str, str]) -> tuple[list[str], int, int]:
    """Aplica traduções às strings faladas; pospõe a palavra de estilo se houver spell:."""
    saida: list[str] = []
    n_trad = n_total = 0
    for l in linhas:
        def sub(m: re.Match) -> str:
            nonlocal n_trad, n_total
            n_total += 1
            novo, ok = traduzir_texto(m.group(2), mapa)
            if ok:
                n_trad += 1
                return f'T: "{novo}"'
            return f't: "{m.group(2)}"'  # ainda em inglês: minúsculo de propósito
        saida.append(T_RE.sub(sub, l))
    # ordem posposta (decisão 8.1): "- spell:" antes da linha "- T/t: <estilo>"
    idx_spell = next((i for i, l in enumerate(saida) if re.match(r"\s*-\s*spell:", l)), None)
    idx_estilo = next((i for i, l in enumerate(saida) if re.match(r'\s*-\s*[Tt]:\s*"', l)), None)
    if idx_spell is not None and idx_estilo is not None and idx_estilo < idx_spell:
        # só quando a entrada é do formato simples (estilo + spell, sem test:)
        if not any("test:" in l for l in saida):
            saida[idx_spell], saida[idx_estilo] = saida[idx_estilo], saida[idx_spell]
    return saida, n_trad, n_total


def entradas_brutas(caminho: Path) -> list[tuple[str, list[str]]]:
    """(chave, linhas da entrada) na ordem do arquivo, sem comentários soltos."""
    out: list[tuple[str, list[str]]] = []
    atual: list[str] | None = None
    chave = ""
    for l in caminho.read_text(encoding="utf-8").splitlines():
        m = re.match(r'^(\s*)-\s*"((?:[^"\\]|\\.)*)"\s*:', l)
        if m and len(m.group(1)) <= 1:
            if atual is not None:
                out.append((chave, atual))
            chave, atual = m.group(2), [l]
        elif atual is not None:
            if l.strip().startswith("#") or not l.strip():
                continue
            atual.append(l)
    if atual is not None:
        out.append((chave, atual))
    return out


def main() -> int:
    ap = argparse.ArgumentParser()
    ap.add_argument("--contar", action="store_true", help="só relata, não escreve")
    ap.add_argument("--rules-dir", default=None)
    a = ap.parse_args()
    raiz = Path(a.rules_dir) if a.rules_dir else Path(__file__).resolve().parent.parent / "Rules" / "Languages"
    pt_full = raiz / "pt" / "unicode-full.yaml"

    # estado do pt SEM o bloco gerado (para ser idempotente)
    texto_pt = pt_full.read_text(encoding="utf-8")
    if INICIO in texto_pt:
        texto_pt = texto_pt[: texto_pt.index(INICIO)].rstrip("\n") + "\n"
    tmp = pt_full.with_suffix(".sem-recuo.tmp")
    tmp.write_text(texto_pt, encoding="utf-8")
    try:
        pt_regras = ler_regras(raiz / "pt" / "unicode.yaml") + ler_regras(tmp)
    finally:
        tmp.unlink()
    pt_idx = indexar(pt_regras)
    en_idx = indexar(ler_regras(raiz / "en" / "unicode.yaml") + ler_regras(raiz / "en" / "unicode-full.yaml"))
    mapa = mapa_traducoes(pt_regras, en_idx)

    bloco: list[str] = []
    n_ent = n_cp = n_trad = n_total = 0
    for chave, linhas in entradas_brutas(raiz / "en" / "unicode-full.yaml"):
        cps = expandir(chave)
        if any(c in pt_idx for c in cps) or all(ord(c) < 128 for c in cps):
            continue  # o pt já cobre (parcial ou todo) — não duplicar codepoint
        novas, t, tot = reescrever_entrada(linhas, mapa)
        # marca própria: recuo deliberado, para não se confundir com `t:` de
        # tradução pendente de conferência (as duas coisas são contadas em
        # separado pelo conferir_vocabulario.py, seção 5)
        novas[0] = re.sub(r"\s*#.*$", "", novas[0]).rstrip() + "   # RECUO-EN"
        bloco.extend(novas)
        n_ent += 1
        n_cp += len(cps)
        n_trad += t
        n_total += tot

    print(f"entradas copiadas do en: {n_ent}  (codepoints: {n_cp})")
    print(f"strings faladas: {n_total}; reaproveitadas de tradução existente ou de estilo definido: {n_trad}; ainda em inglês (t:): {n_total - n_trad}")
    if a.contar:
        return 0

    cabecalho = f"""

{INICIO}
# Entradas copiadas de en/unicode-full.yaml para codepoints que o português
# ainda não cobria. Sem elas o caractere saía CRU para o sintetizador (ver
# ACHADOS 9.2). `T:` maiúsculo = reaproveitou tradução ou termo já definido;
# `t:` minúsculo = ainda em inglês. Toda entrada aqui leva a marca # RECUO-EN
# na linha da chave: é recuo deliberado, NÃO tradução pendente de conferência.
# NÃO EDITE À MÃO: rode `python3 PythonScripts/gerar_recuo_en.py`. Para
# traduzir um destes caracteres, mova a entrada para cima do marcador.
"""
    pt_full.write_text(texto_pt.rstrip("\n") + cabecalho + "\n".join(bloco) + f"\n{FIM}\n", encoding="utf-8")
    print(f"escrito em {pt_full}")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
