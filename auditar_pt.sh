#!/usr/bin/env bash
# ============================================================================
# auditar_pt.sh — roda a ferramenta oficial de auditoria do MathCAT na pasta pt
#
# Uso, a partir da raiz do repositório MathCAT_pt:
#     bash auditar_pt.sh                       # auditoria completa
#     bash auditar_pt.sh --only missing        # só o que falta traduzir
#     bash auditar_pt.sh --file SharedRules/general.yaml
#     bash auditar_pt.sh --verbose             # mostra trechos lado a lado
#
# Por que este script existe: o projeto usa 'uv' e exige Python 3.14 no
# pyproject.toml. Se você tiver uv e 3.14, prefira o caminho oficial:
#     uv sync --project PythonScripts
#     uv run --project PythonScripts audit-translations pt
#
# Este script é o atalho para quem está em Python 3.12/3.13: as dependências
# reais da ferramenta funcionam nessas versões.
# ============================================================================

set -euo pipefail

if [ ! -d "PythonScripts/audit_translations" ]; then
  echo "ERRO: rode a partir da raiz do repositório MathCAT_pt."
  echo "      (a pasta PythonScripts/audit_translations não foi encontrada aqui)"
  exit 1
fi

echo ">> conferindo dependências..."
python3 - <<'PY' || pip install --quiet --break-system-packages rich "ruamel.yaml>=0.19.1" pyyaml "jsonpath-ng>=1.8.0"
import rich, ruamel.yaml, yaml, jsonpath_ng  # noqa: F401
PY

# unicode-full.yaml não foi traduzido por decisão; excluir evita milhares de
# avisos que abafam o resto. Remova --exclude quando começar a traduzi-lo.
cd PythonScripts
python3 - "$@" <<'PY'
import sys
from audit_translations.cli import main

args = sys.argv[1:]
if not any(a in ("--file", "--exclude") for a in args):
    args += ["--exclude", "unicode-full.yaml"]

sys.argv = ["audit-translations", "pt", *args]
main()
PY
