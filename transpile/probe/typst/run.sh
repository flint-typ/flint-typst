#!/usr/bin/env bash
# Capability probes for a pure-Typst port of flint core.
#
# Each probe is a standalone .typ that either compiles (capability present) or
# fails (gap). Probes named *-GAP.typ are EXPECTED to fail -- they document a
# missing feature, so a *success* there means Typst gained the capability and
# TYPST-VS-WASM.md needs revisiting.
#
# Re-run after any Typst upgrade:  ./run.sh
set -uo pipefail
cd "$(dirname "$0")"

OUT=$(mktemp -d)
trap 'rm -rf "$OUT"' EXIT

pass=0 fail=0 unexpected=0
printf '%-34s %-10s %s\n' PROBE EXPECT RESULT
printf '%.0s-' {1..72}; echo

for f in [0-9]*.typ; do
  [[ $f == 23r.typ ]] && continue
  expect=ok
  [[ $f == *-GAP.typ ]] && expect=gap

  if typst compile --format pdf "$f" "$OUT/o.pdf" >"$OUT/err" 2>&1; then
    got=ok
  else
    got=gap
  fi

  if [[ $got == $expect ]]; then
    status=PASS; ((pass++))
  else
    status="UNEXPECTED ($got)"; ((unexpected++)); ((fail++))
  fi
  printf '%-34s %-10s %s\n' "$f" "$expect" "$status"
  [[ $status == PASS && $expect == gap ]] && sed -n '1p' "$OUT/err" | sed 's/^/    /'
done

echo
echo "typst $(typst --version | cut -d' ' -f2): $pass as expected, $unexpected unexpected"
[[ $unexpected -eq 0 ]] || {
  echo "A probe changed behaviour -- re-read ../../TYPST-VS-WASM.md before trusting it."
  exit 1
}
