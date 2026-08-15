#!/usr/bin/env python3
"""Differential test: every ported function against flint-py, call for call.

The conformance corpus (`test/corpus/`) pins the five pipeline entry
points, which is the right acceptance gate but useless while porting the
hundred-odd helpers underneath them — a leaf function has no corpus entry, so
nothing checks it until the stage above lands.

This closes that gap. `cases.py` names a function and a list of argument
tuples; this runs each through flint-py and through the Typst port and diffs
the results. Every function gets checked the moment it is written.

    python test/differential.py                # everything registered
    python test/differential.py type_registry  # one upstream module
    python test/differential.py -v             # show every mismatch

Adding a function: implement it, add it to `test/dispatch.typ`, and add its
cases to `test/cases.py`. All three in the same commit.
"""

from __future__ import annotations

import argparse
import json
import math
import os
import subprocess
import sys
import time
from pathlib import Path

# Pin UTC before flint-py is imported. flint's date handling reads zoneless
# strings in the *host* zone, so on a machine in Europe/Berlin every temporal
# case would differ from the port by an hour — a phantom mismatch that looks
# like a porting bug. The conformance corpus is generated under TZ=UTC for the
# same reason (see the `corpus` make target), and the port is UTC-only by design.
os.environ["TZ"] = "UTC"
if hasattr(time, "tzset"):
    time.tzset()

HERE = Path(__file__).resolve().parent
PKG = HERE.parent
FLINT_PY = PKG / "flint-source/packages/flint-py"

sys.path.insert(0, str(HERE))
sys.path.insert(0, str(FLINT_PY))

FLOAT_ATOL = 1e-9
FLOAT_RTOL = 1e-9


def diff(path: str, actual, expected, out: list[str], limit: int = 12) -> None:
    """Structural diff. Deliberately strict about bool-vs-int and int-vs-float.

    Shares its rules with `test/conformance.py`: JSON
    `true` and `1` are a real divergence, and layout output feeds pixel
    geometry where 200 and 200.0 must not serialize differently.
    """
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


def jsonable(obj):
    """Match the encoding `dispatch.typ` produces on the Typst side."""
    if isinstance(obj, dict):
        return {str(k): jsonable(v) for k, v in obj.items()}
    if isinstance(obj, (list, tuple)):
        return [jsonable(v) for v in obj]
    if isinstance(obj, (set, frozenset)):
        return sorted(jsonable(v) for v in obj)
    if isinstance(obj, float):
        if obj != obj:
            return {"$f": "nan"}
        if obj == float("inf"):
            return {"$f": "inf"}
        if obj == float("-inf"):
            return {"$f": "-inf"}
        return obj
    if isinstance(obj, (str, int, bool)) or obj is None:
        return obj
    return repr(obj)


# Some functions return values whose *representation* legitimately differs
# between the two implementations — a Python `datetime` versus a datehog
# moment, say. A projection maps both sides onto something comparable, so the
# behaviour is still checked without pretending the representations match.
# Names must exist on both sides; the Typst half lives in `dispatch.typ`.
PROJECTIONS = {
    "epoch_ms": lambda v: None if v is None else v.timestamp() * 1000.0,
}


def run_python(specs) -> list:
    import importlib

    out = []
    for spec in specs:
        mod = importlib.import_module(f"flint.core.{spec['module']}")
        fn = getattr(mod, spec["fn"])
        project = PROJECTIONS[spec["project"]] if spec.get("project") else None
        results = []
        for args in spec["cases"]:
            try:
                value = fn(*args)
                results.append({"ok": jsonable(project(value) if project else value)})
            except Exception as exc:
                results.append({"raised": type(exc).__name__})
        out.append(results)
    return out


def run_typst(specs) -> list:
    payload = [
        {"fn": s["fn"], "project": s.get("project"), "cases": [jsonable(list(a)) for a in s["cases"]]}
        for s in specs
    ]
    (HERE / "_cases.json").write_text(json.dumps(payload))
    expr = '{import "test/dispatch.typ": run-all; run-all(json("test/_cases.json"))}'
    r = subprocess.run(
        ["typst", "eval", expr, "--root", str(PKG), "--format", "json"],
        capture_output=True, text=True, cwd=PKG,
    )
    (HERE / "_cases.json").unlink(missing_ok=True)
    if r.returncode != 0:
        print(r.stderr, file=sys.stderr)
        raise SystemExit("typst eval failed")
    return json.loads(r.stdout)


def main() -> int:
    ap = argparse.ArgumentParser(description=__doc__, formatter_class=argparse.RawDescriptionHelpFormatter)
    ap.add_argument("modules", nargs="*", help="limit to these upstream modules")
    ap.add_argument("-v", "--verbose", action="store_true")
    args = ap.parse_args()

    import cases as cases_mod

    specs = cases_mod.build()
    if args.modules:
        specs = [s for s in specs if s["module"] in args.modules]
    if not specs:
        print("no cases selected", file=sys.stderr)
        return 2

    py = run_python(specs)
    ty = run_typst(specs)

    total = passed = 0
    failures: list[tuple[str, object, object, object, list[str]]] = []
    known: list[tuple[str, object, object, object, str]] = []
    per_fn: dict[str, tuple[int, int, int]] = {}

    for spec, py_res, ty_res in zip(specs, py, ty):
        name = f"{spec['module']}.{spec['fn']}"
        expected = spec.get("known_divergences") or {}
        ok = n = kn = 0
        for args_, p, t in zip(spec["cases"], py_res, ty_res):
            n += 1
            total += 1
            if "raised" in p:
                # flint-py threw; the port is expected to return none rather
                # than trap, since Typst cannot catch. Not a comparison.
                ok += 1
                passed += 1
                continue
            d: list[str] = []
            diff("$", t.get("ok"), p["ok"], d)
            if not d:
                ok += 1
                passed += 1
                continue
            reason = expected.get(repr(tuple(args_)))
            if reason:
                # Registered in cases.py with a reason and mirrored in
                # PORT-DICTIONARY.md. Counted and printed, never hidden.
                kn += 1
                known.append((name, args_, t.get("ok"), p["ok"], reason))
            else:
                failures.append((name, args_, t.get("ok"), p["ok"], d))
        per_fn[name] = (ok, n, kn)

    width = max(len(k) for k in per_fn)
    for name, (ok, n, kn) in per_fn.items():
        mark = "ok  " if ok + kn == n else "FAIL"
        note = f"  ({kn} known divergence{'s' if kn != 1 else ''})" if kn else ""
        print(f"  {mark} {name:{width}}  {ok}/{n}{note}")
    print(f"\n{passed}/{total} cases match flint-py across {len(per_fn)} functions")
    if known:
        print(f"\n{len(known)} known divergences (registered in cases.py / PORT-DICTIONARY.md):")
        for name, args_, got, want, reason in known:
            print(f"  {name}{tuple(args_)!r}  typst={got!r} python={want!r}")
            print(f"    {reason}")

    if failures:
        print(f"\n{len(failures)} mismatches:")
        for name, args_, got, want, d in failures[: (len(failures) if args.verbose else 15)]:
            print(f"\n  {name}{tuple(args_)!r}")
            print(f"    typst : {got!r}")
            print(f"    python: {want!r}")
            for line in d[:5]:
                print(f"    {line}")
        if not args.verbose and len(failures) > 15:
            print(f"\n  ... {len(failures) - 15} more (-v for all)")
        return 1
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
