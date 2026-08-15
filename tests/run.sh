#!/usr/bin/env bash
# flint-typst test suite.
#
#   ./tests/run.sh             everything
#   ./tests/run.sh differential  just the per-function differential
#
# PY can point at a venv with flint-py's dependencies installed:
#   make test
set -uo pipefail
cd "$(dirname "$0")/.."

PY=${PY:-.venv/bin/python}
[[ -x $PY ]] || PY=python3
only=${1:-all}
fail=0

if [[ $only == all || $only == generated ]]; then
  echo "== generated tables up to date =="
  $PY tests/gen_tables.py --check || fail=1
  echo
fi

if [[ $only == all || $only == differential ]]; then
  echo "== differential vs flint-py (per function) =="
  $PY tests/differential.py || fail=1
  echo
fi

if [[ $only == all || $only == backend ]]; then
  echo "== backend: every chart type assembles and renders =="
  out=$(mktemp -d); trap 'rm -rf "$out"' EXIT
  if typst compile --root . --format pdf tests/smoke-check.typ "$out/smoke.pdf" 2>"$out/err"; then
    echo "   pass"
  else
    sed 's/^/   /' "$out/err"; fail=1
  fi
  echo
fi

if [[ $only == all || $only == conformance ]]; then
  echo "== conformance vs the 705-case corpus (pipeline stages) =="
  if [[ -d tests/corpus ]]; then
    $PY tests/conformance.py || fail=1
  else
    echo "   corpus missing -- run: make corpus"
    fail=1
  fi
  echo
fi

if [[ $fail -eq 0 ]]; then echo "all suites passed"; else echo "FAILURES"; fi
exit $fail
