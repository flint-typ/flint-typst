#!/usr/bin/env bash
# flint-typst test suite.
#
#   ./test/run.sh             everything
#   ./test/run.sh differential  just the per-function differential
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
  $PY test/gen_tables.py --check || fail=1
  echo
fi

if [[ $only == all || $only == differential ]]; then
  echo "== differential vs flint-py (per function) =="
  $PY test/differential.py || fail=1
  echo
fi

if [[ $only == all || $only == conformance ]]; then
  echo "== conformance vs the 705-case corpus (pipeline stages) =="
  if [[ -d test/corpus ]]; then
    $PY test/conformance.py || fail=1
  else
    echo "   corpus missing -- run: make corpus"
    fail=1
  fi
  echo
fi

if [[ $fail -eq 0 ]]; then echo "all suites passed"; else echo "FAILURES"; fi
exit $fail
