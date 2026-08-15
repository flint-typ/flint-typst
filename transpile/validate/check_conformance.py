#!/usr/bin/env python3
"""Replay the core golden corpus against a Rust build and diff the results.

`make_core_corpus.py` recorded, for 705 fixtures, exactly what each core stage
received and returned in flint-py. This feeds those same inputs to the Rust
port and compares. It is the acceptance test for *any* route to Rust --
depyler output, a hand-port, or a mix -- because it pins behaviour at the
module boundary rather than at the Vega-Lite output.

Stages the Rust build reports as unimplemented are skipped, not failed, so the
harness stays usable while the port is partial; the summary always states how
many stages were skipped so a partial run cannot read as a clean one.

Float comparison is tolerant (layout arithmetic differs in the last bits
between CPython and Rust) but int-vs-float is *not* silently accepted: flint's
layout output feeds pixel geometry where 200 and 200.0 must not diverge in
serialization.

Usage:
    python check_conformance.py --corpus DIR --replay-bin PATH [--stage NAME] [-v]
"""

from __future__ import annotations

import argparse
import gzip
import json
import math
import subprocess
import sys
from collections import Counter
from pathlib import Path
from typing import Any

FLOAT_ATOL = 1e-9
FLOAT_RTOL = 1e-9


def load_case(path: Path) -> dict[str, Any]:
    raw = gzip.decompress(path.read_bytes()) if path.suffix == ".gz" else path.read_bytes()
    return json.loads(raw)


def diff(path: str, actual: Any, expected: Any, out: list[str], limit: int = 40) -> None:
    if len(out) >= limit:
        return
    if isinstance(expected, dict):
        if not isinstance(actual, dict):
            out.append(f"{path}: expected object, got {type(actual).__name__}")
            return
        for k in sorted(set(expected) - set(actual)):
            out.append(f"{path}.{k}: missing (expected {expected[k]!r})")
        for k in sorted(set(actual) - set(expected)):
            out.append(f"{path}.{k}: unexpected (got {actual[k]!r})")
        for k in sorted(set(actual) & set(expected)):
            diff(f"{path}.{k}", actual[k], expected[k], out, limit)
        return
    if isinstance(expected, list):
        if not isinstance(actual, list):
            out.append(f"{path}: expected array, got {type(actual).__name__}")
            return
        if len(actual) != len(expected):
            out.append(f"{path}: length {len(actual)} != {len(expected)}")
        for i in range(min(len(actual), len(expected))):
            diff(f"{path}[{i}]", actual[i], expected[i], out, limit)
        return
    # bool must be checked before int -- bool is a subclass of int in Python
    # and JSON `true` vs `1` is a real divergence, not a rounding artifact.
    if isinstance(expected, bool) or isinstance(actual, bool):
        if actual is not expected:
            out.append(f"{path}: {actual!r} != {expected!r}")
        return
    if isinstance(expected, (int, float)) and isinstance(actual, (int, float)):
        if isinstance(expected, int) != isinstance(actual, int):
            out.append(f"{path}: numeric kind differs ({actual!r} vs {expected!r})")
            return
        if isinstance(expected, float):
            if math.isnan(expected) and math.isnan(actual):
                return
            if abs(actual - expected) > FLOAT_ATOL + FLOAT_RTOL * max(abs(actual), abs(expected)):
                out.append(f"{path}: {actual!r} != {expected!r}")
            return
        if actual != expected:
            out.append(f"{path}: {actual!r} != {expected!r}")
        return
    if actual != expected:
        out.append(f"{path}: {actual!r} != {expected!r}")


def main(argv: list[str] | None = None) -> int:
    ap = argparse.ArgumentParser(description=__doc__, formatter_class=argparse.RawDescriptionHelpFormatter)
    ap.add_argument("--corpus", type=Path, required=True)
    ap.add_argument("--replay-bin", type=Path, required=True, help="path to the compiled `replay` binary")
    ap.add_argument("--stage", action="append", help="only check these stages (repeatable)")
    ap.add_argument("--limit", type=int, default=0)
    ap.add_argument("-v", "--verbose", action="store_true", help="print diffs for failing cases")
    args = ap.parse_args(argv)

    if not args.replay_bin.exists():
        print(f"replay binary not found: {args.replay_bin}\nrun `make build` first", file=sys.stderr)
        return 2

    cases = sorted(p for p in args.corpus.iterdir() if p.name != "_manifest.json" and p.suffix in (".gz", ".json"))
    if args.limit:
        cases = cases[: args.limit]
    if not cases:
        print(f"no corpus cases in {args.corpus}\nrun `make corpus` first", file=sys.stderr)
        return 2

    passed: Counter[str] = Counter()
    failed: Counter[str] = Counter()
    skipped: Counter[str] = Counter()
    first_diffs: dict[str, tuple[str, list[str]]] = {}
    implemented: list[str] | None = None

    for case_path in cases:
        case = load_case(case_path)
        proc = subprocess.run(
            [str(args.replay_bin)],
            input=json.dumps(case),
            capture_output=True,
            text=True,
        )
        if proc.returncode != 0:
            failed["<replay crashed>"] += 1
            first_diffs.setdefault("<replay crashed>", (case["slug"], [proc.stderr.strip()[:400]]))
            continue

        actual_doc = json.loads(proc.stdout)
        if implemented is None:
            implemented = list(actual_doc.get("implemented") or [])

        for stage, recorded_calls in case["calls"].items():
            if args.stage and stage not in args.stage:
                continue
            if implemented is not None and stage not in implemented:
                skipped[stage] += len(recorded_calls)
                continue
            got_calls = actual_doc["calls"].get(stage, [])
            for i, recorded in enumerate(recorded_calls):
                if "result" not in recorded:
                    continue  # the Python side raised; nothing to compare
                got = got_calls[i] if i < len(got_calls) else {"__error__": "no result produced"}
                if "__error__" in got:
                    failed[stage] += 1
                    first_diffs.setdefault(stage, (case["slug"], [got["__error__"]]))
                    continue
                d: list[str] = []
                diff("$", got["result"], recorded["result"], d)
                if d:
                    failed[stage] += 1
                    first_diffs.setdefault(stage, (case["slug"], d))
                else:
                    passed[stage] += 1

    stages = sorted(set(passed) | set(failed) | set(skipped))
    print(f"corpus: {len(cases)} cases from {args.corpus}")
    print(f"{'stage':30} {'pass':>7} {'fail':>7} {'skip':>7}")
    for s in stages:
        print(f"{s:30} {passed[s]:>7} {failed[s]:>7} {skipped[s]:>7}")
    total_pass, total_fail, total_skip = sum(passed.values()), sum(failed.values()), sum(skipped.values())
    print(f"{'TOTAL':30} {total_pass:>7} {total_fail:>7} {total_skip:>7}")

    if total_skip:
        print(f"\n{total_skip} calls skipped -- the Rust build reports these stages unimplemented: "
              f"{sorted(set(skipped))}")
    if args.verbose and first_diffs:
        print("\nfirst diff per failing stage:")
        for stage, (slug, d) in first_diffs.items():
            print(f"\n  [{stage}] {slug}")
            for line in d[:15]:
                print(f"    {line}")

    if total_fail:
        return 1
    if total_pass == 0:
        print("\nnothing was actually verified (no stage implemented yet)")
        return 3
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
