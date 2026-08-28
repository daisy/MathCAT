#!/usr/bin/env python3
# ============================================================================
# conferir_vocabulario.py — confere consistência de VOCABULÁRIO entre
# unicode.yaml e unicode-full.yaml de um idioma.
#
# Por que existe: quando o unicode-full.yaml foi montado, conferiu-se colisão
# de CODEPOINT ("nenhum símbolo aparece nos dois arquivos") e concluiu-se daí
# que não havia conflito. Mas duas entradas podem não colidir em codepoint e
# ainda assim falar o MESMO conceito com palavras diferentes — foi assim que
# passaram gótico/fraktur, cursivo/caligráfico e a ordem do adjetivo.
#
# Uso, a partir da raiz do repositório:
#     python3 PythonScripts/conferir_vocabulario.py          # idioma pt
#     python3 PythonScripts/conferir_vocabulario.py --idioma es
#     python3 PythonScripts/conferir_vocabulario.py --secao ordem
#
# Só usa a biblioteca padrão, de propósito: é para rodar sem preparar ambiente.
# ============================================================================

from __future__ import annotations

import argparse
import re
import sys
from collections import defaultdict
from dataclasses import dataclass, field
from pathlib import Path

# ---------------------------------------------------------------------------
# Leitura dos arquivos de regra
#
# Não usamos um parser YAML: estes arquivos têm chaves repetidas e valores que
# são diretivas do MathCAT, não dados. O que interessa aqui é a ORDEM em que
# as diretivas aparecem dentro de cada regra (é ela que define a ordem das
# palavras faladas), e um parser YAML normalizaria isso.
# ---------------------------------------------------------------------------

CHAVE = re.compile(r'^(\s*)-\s*"((?:[^"\\]|\\.)*)"\s*:\s*(.*)$')
FALA = re.compile(r'(?<![A-Za-z_])(OT|T|ot|t)\s*:\s*"([^"]*)"')
SOLETRA = re.compile(r'(?<![A-Za-z_])spell\s*:')
CONDICAO = re.compile(r"\$(\w+)\s*(?:=|!=)\s*'(\w+)'")
TEM_TESTE = re.compile(r"(?<![A-Za-z_])(test|if|else_if|then_test|else_test)\s*:")


@dataclass
class Regra:
    """Uma entrada `- "<símbolo>":` com o corpo que a segue."""

    chave: str
    arquivo: str
    linha: int
    # (tipo, texto, índice de ordem dentro do corpo)
    falas: list[tuple[str, str, int]] = field(default_factory=list)
    soletra_em: int | None = None
    # Uma entrada por LINHA que tem condição, com as condições daquela linha
    # agrupadas. O agrupamento é essencial: separar
    #     if: $P = 'Auto' or $P = 'Member'
    # em duas linhas
    #     if: $P = 'Auto'  /  else_if: $P = 'Member'
    # muda a fala, mas some se achatarmos tudo numa lista só.
    condicoes: list[tuple[tuple[str, str], ...]] = field(default_factory=list)
    ramificada: bool = False

    @property
    def texto_falado(self) -> str:
        return " ".join(t for _, t, _ in self.falas)

    @property
    def assinatura(self) -> tuple:
        """Estrutura de condições da regra, ignorando as palavras faladas."""
        return tuple(self.condicoes)

    @property
    def qualificador(self) -> str | None:
        """A palavra de estilo de uma regra de alfabeto (spell: + uma palavra).

        Ex.: ℜ -> "gótico", 𝔄-𝔜 -> "fraktur". Devolve None quando a regra não
        é desse formato.
        """
        if self.soletra_em is None or self.ramificada or len(self.falas) != 1:
            return None
        return self.falas[0][1]

    @property
    def ordem_qualificador(self) -> str | None:
        """'anteposto' se a palavra de estilo vem antes do spell:, senão 'posposto'."""
        if self.qualificador is None:
            return None
        return "anteposto" if self.falas[0][2] < self.soletra_em else "posposto"


def ler_regras(caminho: Path) -> list[Regra]:
    if not caminho.exists():
        return []
    linhas = caminho.read_text(encoding="utf-8").splitlines()
    regras: list[Regra] = []
    atual: Regra | None = None
    recuo_chave = 0

    def corpo(regra: Regra, texto: str, n: int) -> None:
        # `ordem` cresce a cada linha do corpo, para sabermos se a palavra
        # falada veio antes ou depois do spell:.
        ordem = regra.linha * 1000 + n
        for tipo, txt in FALA.findall(texto):
            regra.falas.append((tipo, txt, ordem))
        if SOLETRA.search(texto) and regra.soletra_em is None:
            regra.soletra_em = ordem
        conds = CONDICAO.findall(texto)
        if conds:
            regra.condicoes.append(tuple(conds))
        if TEM_TESTE.search(texto):
            regra.ramificada = True

    for n, linha in enumerate(linhas, 1):
        if not linha.strip() or linha.lstrip().startswith("#"):
            continue
        m = CHAVE.match(linha)
        # Uma chave nova só encerra a anterior se não estiver mais recuada
        # que ela (senão é um item do corpo, como `- T: "..."`).
        if m and (atual is None or len(m.group(1)) <= recuo_chave):
            atual = Regra(chave=m.group(2), arquivo=caminho.name, linha=n)
            regras.append(atual)
            recuo_chave = len(m.group(1))
            corpo(atual, m.group(3), 0)
        elif atual is not None:
            corpo(atual, linha, n)
    return regras


# ---------------------------------------------------------------------------
# Expansão das chaves em codepoints
# ---------------------------------------------------------------------------


def expandir(chave: str) -> list[str]:
    """"ℂℕℚℝℤ" -> 5 símbolos;  "𝔄-𝔜" -> a faixa;  "℃" -> 1 símbolo."""
    cars = list(chave)
    if len(cars) == 3 and cars[1] == "-":
        ini, fim = ord(cars[0]), ord(cars[2])
        if ini <= fim:
            return [chr(c) for c in range(ini, fim + 1)]
    return cars


def indexar(regras: list[Regra]) -> dict[str, Regra]:
    """codepoint -> regra que o cobre."""
    saida: dict[str, Regra] = {}
    for r in regras:
        for c in expandir(r.chave):
            saida.setdefault(c, r)
    return saida


# ---------------------------------------------------------------------------
# Apresentação
# ---------------------------------------------------------------------------


def nome(c: str) -> str:
    import unicodedata

    try:
        return unicodedata.name(c)
    except ValueError:
        return "sem nome"


def cp(c: str) -> str:
    return f"U+{ord(c):04X}"


def amostra(cars: list[str], n: int = 6) -> str:
    vis = " ".join(f"{c} ({cp(c)})" for c in cars[:n])
    return vis + (f" … +{len(cars) - n}" if len(cars) > n else "")


def titulo(txt: str) -> None:
    print()
    print("=" * 78)
    print(txt)
    print("=" * 78)


# ---------------------------------------------------------------------------
# Seção 1 — mesmo conceito, palavras diferentes
#
# A identidade do conceito vem do INGLÊS: se dois símbolos são falados com a
# mesma palavra em en e com palavras diferentes no idioma alvo, os tradutores
# dividiram um conceito só em dois termos.
# ---------------------------------------------------------------------------


def secao_conceitos(pt: dict[str, Regra], en: dict[str, Regra]) -> int:
    grupos: dict[str, dict[str, list[str]]] = defaultdict(lambda: defaultdict(list))
    for c, r_pt in pt.items():
        r_en = en.get(c)
        if r_en is None or r_pt.ramificada or r_en.ramificada:
            continue  # regras com test: dependem de preferência; fora do escopo
        t_en, t_pt = r_en.texto_falado.strip(), r_pt.texto_falado.strip()
        if t_en and t_pt:
            grupos[t_en][t_pt].append(c)

    achados = {en_t: v for en_t, v in grupos.items() if len(v) > 1}
    titulo("1. MESMO CONCEITO, PALAVRAS DIFERENTES")
    print("   (agrupado pelo termo em inglês; divergência = o en usa uma palavra")
    print("    e a tradução usa duas ou mais)")
    if not achados:
        print("\n   nada encontrado.")
        return 0

    for en_t in sorted(achados, key=lambda k: (-len(achados[k]), k)):
        variantes = achados[en_t]
        arquivos = {
            pt_t: sorted({pt[c].arquivo for c in cars}) for pt_t, cars in variantes.items()
        }
        # O sinal mais forte: cada variante mora num arquivo diferente.
        todos = [a for lista in arquivos.values() for a in lista]
        fronteira = len(set(todos)) > 1 and all(len(a) == 1 for a in arquivos.values())
        marca = "  <<< divide na fronteira dos arquivos" if fronteira else ""
        print(f"\n   en: {en_t!r}{marca}")
        for pt_t, cars in sorted(variantes.items(), key=lambda kv: -len(kv[1])):
            print(f"      {pt_t!r}  [{', '.join(arquivos[pt_t])}]  {len(cars)} símbolo(s)")
            print(f"         {amostra(sorted(cars))}")
    return len(achados)


# ---------------------------------------------------------------------------
# Seção 2 — ordem do adjetivo de estilo
# ---------------------------------------------------------------------------


def secao_ordem(regras: list[Regra]) -> int:
    por_ordem: dict[str, list[Regra]] = defaultdict(list)
    for r in regras:
        o = r.ordem_qualificador
        if o:
            por_ordem[o].append(r)

    titulo("2. ORDEM DO ADJETIVO DE ESTILO")
    print("   (regras com spell: + palavra de estilo — 'gótico maiúscula r' contra")
    print("    'maiúscula i fraktur'; a ordem sai da posição da palavra no corpo)")
    if len(por_ordem) < 2:
        unica = next(iter(por_ordem), "nenhuma")
        print(f"\n   consistente: todas as regras usam '{unica}'.")
        return 0

    # Sinal inequívoco: a MESMA palavra usada nas duas ordens. Qualificadores
    # diferentes em ordens diferentes podem ser legítimos (em inglês, 'with
    # period' pospõe de direito); a mesma palavra dos dois jeitos, não.
    ordens_por_palavra: dict[str, set[str]] = defaultdict(set)
    for o, rs in por_ordem.items():
        for r in rs:
            ordens_por_palavra[r.qualificador].add(o)
    ambiguas = sorted(p for p, os_ in ordens_por_palavra.items() if len(os_) > 1)
    if ambiguas:
        print("\n   >>> MESMA palavra nas duas ordens (não há leitura benigna):")
        for p in ambiguas:
            print(f"       {p!r}")
            for r in sorted(regras, key=lambda r: (r.arquivo, r.linha)):
                if r.qualificador == p:
                    print(f"          {r.ordem_qualificador:<10} {r.arquivo}:{r.linha}"
                          f"  {amostra(expandir(r.chave), 3)}")

    for o in ("anteposto", "posposto"):
        rs = por_ordem.get(o, [])
        if not rs:
            continue
        arqs = sorted({r.arquivo for r in rs})
        print(f"\n   {o.upper()}  ({len(rs)} regras em {', '.join(arqs)})")
        for r in sorted(rs, key=lambda r: (r.arquivo, r.linha)):
            ex = expandir(r.chave)
            print(f"      {r.arquivo}:{r.linha}  {r.qualificador!r}  ->  {amostra(ex, 3)}")
    print("\n   as duas ordens convivem: a fala fica inconsistente entre os arquivos.")
    print("   (qualificadores de várias palavras podem pospor de direito — confira")
    print("    caso a caso; a lista '>>>' acima é a que não tem defesa)")
    return 1


# ---------------------------------------------------------------------------
# Seção 3 — símbolos aparentados com regras estruturalmente divergentes
#
# "Aparentados" = símbolos que o inglês trata com a MESMA estrutura de
# condições. Se o en os trata igual e a tradução não, alguém editou um e
# esqueceu os irmãos.
# ---------------------------------------------------------------------------


LIMITE_FAMILIA = 20  # acima disso a assinatura é um padrão genérico, não um parentesco


def desenhar_sig(sig: tuple) -> str:
    """'Auto|In / Member / Element' — uma fatia por linha de condição."""
    if not sig:
        return "(sem condições)"
    return " / ".join("|".join(val for _, val in linha) for linha in sig)


def secao_estrutura(pt: dict[str, Regra], en: dict[str, Regra]) -> int:
    familias: dict[tuple, set[str]] = defaultdict(set)
    for c, r_en in en.items():
        if c in pt and r_en.assinatura:
            familias[r_en.assinatura].add(c)

    especificas, genericas = [], []
    for assinatura, cars in familias.items():
        if len(cars) < 2:
            continue
        por_assinatura: dict[tuple, set[str]] = defaultdict(set)
        for c in cars:
            por_assinatura[pt[c].assinatura].add(c)
        if len(por_assinatura) == 1:
            continue
        (especificas if len(cars) <= LIMITE_FAMILIA else genericas).append(
            (assinatura, por_assinatura, len(cars))
        )

    titulo("3. SÍMBOLOS APARENTADOS COM REGRAS DIVERGENTES")
    print("   (aparentados = o inglês lhes dá a MESMA estrutura de condições;")
    print("    divergência = a tradução deixou de tratá-los igual entre si)")
    if not especificas and not genericas:
        print("\n   nada encontrado.")
        return 0

    for assinatura, por_assinatura, _ in sorted(especificas, key=lambda t: t[2]):
        prefs = sorted({v for linha in assinatura for v, _ in linha})
        ordenadas = sorted(por_assinatura.items(), key=lambda kv: -len(kv[1]))
        print(f"\n   família de {sum(len(c) for c in por_assinatura.values())} símbolos"
              f" — preferência: {', '.join(prefs)}")
        # O rótulo compara os IRMÃOS entre si: a assinatura majoritária é a
        # linha de base, a minoritária é a que destoa. Comparar com o en aqui
        # confundiria divergência deliberada do idioma com edição pela metade.
        for pos, (pt_sig, cars) in enumerate(ordenadas):
            papel = "maioria da família" if pos == 0 else "DESTOA dos irmãos"
            marcas = sorted(cars)
            r = pt[marcas[0]]
            print(f"      {amostra(marcas)}   <{papel}>   {r.arquivo}:{r.linha}")
            print(f"         {desenhar_sig(pt_sig)}")
        if all(s != assinatura for s in por_assinatura):
            print(f"      nota: a família inteira já diverge do {'en'} — provável escolha")
            print(f"            deliberada do idioma, não é o que esta seção aponta:")
            print(f"         en: {desenhar_sig(assinatura)}")

    if genericas:
        print(f"\n   [padrões genéricos, >{LIMITE_FAMILIA} símbolos — provável divergência")
        print("    do en, não parentesco; conferir à parte]")
        for assinatura, por_assinatura, n in sorted(genericas, key=lambda t: -t[2]):
            prefs = sorted({v for linha in assinatura for v, _ in linha})
            fora = {
                s: cars for s, cars in por_assinatura.items() if s != assinatura
            }
            for s, cars in fora.items():
                print(f"      {', '.join(prefs)}: {len(cars)} de {n} símbolos divergem"
                      f" — {desenhar_sig(assinatura)}  ->  {desenhar_sig(s)}")
                print(f"         {amostra(sorted(cars), 8)}")
    return len(especificas) + len(genericas)


# ---------------------------------------------------------------------------
# Seção 4 — a conferência original (colisão de codepoint), mantida
# ---------------------------------------------------------------------------


def secao_colisao(base: list[Regra], full: list[Regra]) -> int:
    a, b = indexar(base), indexar(full)
    comuns = sorted(set(a) & set(b))
    titulo("4. COLISÃO DE CODEPOINT (a conferência original)")
    if not comuns:
        print("\n   nenhum símbolo aparece nos dois arquivos.")
        print("   ATENÇÃO: isto NÃO implica consistência de vocabulário — ver seções 1-3.")
        return 0
    print(f"\n   {len(comuns)} símbolo(s) definidos nos dois arquivos:")
    for c in comuns[:40]:
        print(f"      {c} {cp(c)}  {a[c].arquivo}:{a[c].linha}  x  {b[c].arquivo}:{b[c].linha}")
    return len(comuns)


# ---------------------------------------------------------------------------


def main() -> int:
    p = argparse.ArgumentParser(
        description="Confere consistência de vocabulário entre unicode.yaml e unicode-full.yaml."
    )
    p.add_argument("--idioma", default="pt", help="código do idioma (padrão: pt)")
    p.add_argument("--referencia", default="en", help="idioma que define os conceitos (padrão: en)")
    p.add_argument(
        "--secao",
        choices=["conceitos", "ordem", "estrutura", "colisao", "todas"],
        default="todas",
    )
    p.add_argument("--rules-dir", default=None, help="caminho de Rules/Languages")
    args = p.parse_args()

    raiz = Path(args.rules_dir) if args.rules_dir else Path(__file__).resolve().parent.parent / "Rules" / "Languages"
    if not raiz.is_dir():
        print(f"ERRO: não encontrei {raiz}. Rode a partir da raiz do repositório.", file=sys.stderr)
        return 2

    def carregar(idioma: str) -> tuple[list[Regra], list[Regra]]:
        d = raiz / idioma
        return ler_regras(d / "unicode.yaml"), ler_regras(d / "unicode-full.yaml")

    pt_base, pt_full = carregar(args.idioma)
    en_base, en_full = carregar(args.referencia)
    if not pt_base and not pt_full:
        print(f"ERRO: nenhuma regra lida para '{args.idioma}' em {raiz}.", file=sys.stderr)
        return 2

    pt_regras = pt_base + pt_full
    pt_idx = indexar(pt_regras)
    en_idx = indexar(en_base + en_full)

    print(f"idioma: {args.idioma}   referência de conceitos: {args.referencia}")
    print(
        f"regras lidas: {len(pt_base)} em unicode.yaml + {len(pt_full)} em unicode-full.yaml"
        f"  ({len(pt_idx)} codepoints)"
    )
    print(f"referência: {len(en_idx)} codepoints")

    total = 0
    if args.secao in ("conceitos", "todas"):
        total += secao_conceitos(pt_idx, en_idx)
    if args.secao in ("ordem", "todas"):
        total += secao_ordem(pt_regras)
    if args.secao in ("estrutura", "todas"):
        total += secao_estrutura(pt_idx, en_idx)
    if args.secao in ("colisao", "todas"):
        secao_colisao(pt_base, pt_full)

    titulo("RESUMO")
    print(f"   {total} grupo(s) de inconsistência encontrados nas seções 1-3.")
    print("   Saída 1 = há o que conferir; 0 = limpo.")
    return 1 if total else 0


if __name__ == "__main__":
    raise SystemExit(main())
