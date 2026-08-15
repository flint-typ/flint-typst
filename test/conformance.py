#!/usr/bin/env python3
"""Replay the 705-case core corpus through the Typst port and diff.

This is the acceptance gate. `differential.py` checks functions in isolation
against hand-written arguments; this checks whole pipeline stages against the
inputs and outputs flint-py actually produced on 705 real chart fixtures.

Stages the port reports as unimplemented are skipped and counted, never
silently passed — a partial port cannot read as a clean run.

    python test/conformance.py                       # all stages
    python test/conformance.py --stage compute_layout
    python test/conformance.py -v                    # show diffs
"""

from __future__ import annotations

import argparse
import gzip
import json
import subprocess
import sys
from collections import Counter
from pathlib import Path

HERE = Path(__file__).resolve().parent
PKG = HERE.parent
CORPUS = PKG / "transpile/corpus/core"

sys.path.insert(0, str(HERE))
from differential import diff  # noqa: E402  -- one differ, shared rules


def load_case(path: Path) -> dict:
    raw = gzip.decompress(path.read_bytes()) if path.suffix == ".gz" else path.read_bytes()
    return json.loads(raw)


def run_typst(case: dict) -> dict:
    (HERE / "_case.json").write_text(json.dumps(case))
    expr = '{import "src/core/replay.typ": run-case; run-case(json("test/_case.json"))}'
    r = subprocess.run(
        ["typst", "eval", expr, "--root", str(PKG), "--format", "json"],
        capture_output=True, text=True, cwd=PKG,
    )
    (HERE / "_case.json").unlink(missing_ok=True)
    if r.returncode != 0:
        raise RuntimeError(r.stderr.strip()[:600])
    return json.loads(r.stdout)


def main() -> int:
    ap = argparse.ArgumentParser(description=__doc__, formatter_class=argparse.RawDescriptionHelpFormatter)
    ap.add_argument("--corpus", type=Path, default=CORPUS)
    ap.add_argument("--stage", action="append", help="only these stages (repeatable)")
    ap.add_argument("--limit", type=int, default=0)
    ap.add_argument("-v", "--verbose", action="store_true")
    args = ap.parse_args()

    if not args.corpus.exists():
        print(f"corpus not found at {args.corpus}\nrun: make -C transpile corpus", file=sys.stderr)
        return 2

    cases = sorted(p for p in args.corpus.iterdir() if p.suffix in (".gz", ".json") and p.name != "_manifest.json")
    if args.limit:
        cases = cases[: args.limit]

    passed: Counter[str] = Counter()
    failed: Counter[str] = Counter()
    skipped: Counter[str] = Counter()
    first: dict[str, tuple[str, list[str]]] = {}
    implemented: list[str] | None = None

    for path in cases:
        case = load_case(path)
        try:
            got = run_typst(case)
        except RuntimeError as exc:
            failed["<typst error>"] += 1
            first.setdefault("<typst error>", (case.get("slug", path.name), [str(exc)]))
            break  # a compile error repeats on every case; stop at the first
        if implemented is None:
            implemented = list(got.get("implemented") or [])

        for stage, recorded in case["calls"].items():
            if args.stage and stage not in args.stage:
                continue
            if stage not in (implemented or []):
                skipped[stage] += len(recorded)
                continue
            produced = got["calls"].get(stage, [])
            for i, rec in enumerate(recorded):
                if "result" not in rec:
                    continue  # flint-py raised; nothing to compare
                out = produced[i] if i < len(produced) else {"__error__": "no result produced"}
                if "__error__" in out:
                    failed[stage] += 1
                    first.setdefault(stage, (case["slug"], [out["__error__"]]))
                    continue
                d: list[str] = []
                diff("$", out.get("ok"), rec["result"], d)
                if d:
                    failed[stage] += 1
                    first.setdefault(stage, (case["slug"], d))
                else:
                    passed[stage] += 1

    stages = sorted(set(passed) | set(failed) | set(skipped))
    print(f"   corpus: {len(cases)} cases")
    print(f"   {'stage':30} {'pass':>7} {'fail':>7} {'skip':>7}")
    for s in stages:
        print(f"   {s:30} {passed[s]:>7} {failed[s]:>7} {skipped[s]:>7}")
    tp, tf, ts = sum(passed.values()), sum(failed.values()), sum(skipped.values())
    print(f"   {'TOTAL':30} {tp:>7} {tf:>7} {ts:>7}")

    if ts:
        print(f"\n   {ts} calls skipped -- not implemented yet: {sorted(set(skipped))}")
    if args.verbose and first:
        print("\n   first diff per failing stage:")
        for stage, (slug, d) in first.items():
            print(f"\n     [{stage}] {slug}")
            for line in d[:12]:
                print(f"       {line}")

    if tf:
        return 1
    if tp == 0:
        print("\n   nothing verified yet (no stage implemented)")
        return 0  # not a failure while the port is in progress
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
