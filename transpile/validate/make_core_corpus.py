#!/usr/bin/env python3
"""Record a core-level golden corpus by tracing flint-py's real pipeline.

The fixtures under `shared/test-data/` pin the *Vega-Lite* output, which sits
downstream of backend-specific template code a lilaq/Typst backend will not
share. The contract that actually matters for a port is the boundary of
`flint/core/`: for a given chart declaration, what do
`resolve_channel_semantics`, `compute_channel_budgets`, `filter_overflow` and
`compute_layout` receive, and what do they return?

Rather than reimplement the Phase-0/Phase-1 sequence (which is genuinely
coupled to the vegalite template registry -- mark types, declare_fn,
paramOverrides), this wraps those four functions with recorders and runs the
*real* assembler. Every call is captured as an (args, result) pair. The result
is a backend-agnostic conformance corpus that any reimplementation of core --
transpiled Rust, hand-written Rust, anything -- can be replayed against with
`check_conformance.py`.

Usage:
    python make_core_corpus.py --flint-py DIR --fixtures DIR -o OUT_DIR
"""

from __future__ import annotations

import argparse
import copy
import gzip
import json
import sys
import traceback
from pathlib import Path
from typing import Any

# The core boundary we pin. Each entry is a name bound in the
# `flint.vegalite.assemble` module namespace.
TRACED = [
    "convert_temporal_data",
    "resolve_channel_semantics",
    "compute_channel_budgets",
    "filter_overflow",
    "compute_layout",
]


# JSON has no literal for the non-finite floats, but flint's layout code uses
# them meaningfully (Infinity as "no cap"). Python's json module papers over
# this by emitting bare `Infinity`, which is invalid JSON that serde_json --
# and Typst's `json()` -- both reject. So the corpus, the plugin ABI and the
# Rust core all agree on this tagged encoding instead. Keep it in sync with
# `nonfinite` in rust/src/core/mod.rs.
NONFINITE = {
    float("inf"): {"$f": "inf"},
    float("-inf"): {"$f": "-inf"},
}


def _jsonable(obj: Any) -> Any:
    """Deep-convert to something json.dumps can emit, deterministically.

    Sets become sorted lists (flint passes `all_mark_types` as a set); floats
    that are integral are left alone -- normalizing them here would hide real
    int/float divergence between the Python and Rust implementations, which is
    exactly the class of bug this corpus exists to catch.
    """
    if isinstance(obj, dict):
        return {str(k): _jsonable(v) for k, v in obj.items()}
    if isinstance(obj, (list, tuple)):
        return [_jsonable(v) for v in obj]
    if isinstance(obj, (set, frozenset)):
        return sorted(_jsonable(v) for v in obj)
    if isinstance(obj, float) and not isinstance(obj, bool):
        if obj != obj:  # NaN -- not hashable-comparable, check first
            return {"$f": "nan"}
        if obj in NONFINITE:
            return NONFINITE[obj]
        return obj
    if isinstance(obj, (str, int, bool)) or obj is None:
        return obj
    return repr(obj)


class Recorder:
    """Wraps a core function, capturing every (args, kwargs, result) triple."""

    def __init__(self, name: str, fn: Any, sink: dict[str, list[dict[str, Any]]]) -> None:
        self.name = name
        self.fn = fn
        self.sink = sink

    def __call__(self, *args: Any, **kwargs: Any) -> Any:
        # Snapshot inputs *before* the call: several core functions mutate the
        # dicts they are handed (compute_layout writes into channel_semantics),
        # so recording afterwards would pin post-mutation state as "input".
        rec: dict[str, Any] = {
            "args": _jsonable(copy.deepcopy(args)),
            "kwargs": _jsonable(copy.deepcopy(kwargs)),
        }
        try:
            result = self.fn(*args, **kwargs)
        except Exception as exc:
            rec["raised"] = f"{type(exc).__name__}: {exc}"
            self.sink.setdefault(self.name, []).append(rec)
            raise
        rec["result"] = _jsonable(copy.deepcopy(result))
        self.sink.setdefault(self.name, []).append(rec)
        return result


def install(assemble_mod: Any, sink: dict[str, list[dict[str, Any]]]) -> list[tuple[str, Any]]:
    """Swap each traced core function for a Recorder. Returns undo pairs."""
    undo: list[tuple[str, Any]] = []
    for name in TRACED:
        original = getattr(assemble_mod, name, None)
        if original is None:
            continue
        undo.append((name, original))
        setattr(assemble_mod, name, Recorder(name, original, sink))
    return undo


def uninstall(assemble_mod: Any, undo: list[tuple[str, Any]]) -> None:
    for name, original in undo:
        setattr(assemble_mod, name, original)


def main(argv: list[str] | None = None) -> int:
    ap = argparse.ArgumentParser(description=__doc__, formatter_class=argparse.RawDescriptionHelpFormatter)
    ap.add_argument("--flint-py", type=Path, required=True, help="flint-py package root")
    ap.add_argument("--fixtures", type=Path, required=True, help="shared/test-data directory")
    ap.add_argument("-o", "--outdir", type=Path, required=True)
    ap.add_argument("--limit", type=int, default=0, help="stop after N fixtures (0 = all)")
    ap.add_argument("--no-gzip", action="store_true", help="write plain .json (~20x larger)")
    args = ap.parse_args(argv)

    sys.path.insert(0, str(args.flint_py))
    from flint.vegalite import assemble as assemble_mod  # noqa: E402

    args.outdir.mkdir(parents=True, exist_ok=True)

    ok = 0
    unported = 0
    failed: list[tuple[str, str]] = []
    call_totals: dict[str, int] = {}

    cases = sorted(p for p in args.fixtures.iterdir() if (p / "input.json").exists())
    if args.limit:
        cases = cases[: args.limit]

    for case in cases:
        slug = case.name
        doc = json.loads((case / "input.json").read_text(encoding="utf-8"))
        spec = doc.get("input", doc)

        sink: dict[str, list[dict[str, Any]]] = {}
        undo = install(assemble_mod, sink)
        try:
            assemble_mod.assemble_vegalite(copy.deepcopy(spec))
        except ValueError as exc:
            # Chart types whose vegalite template is not ported yet still
            # exercise core up to the point they fail; keep whatever was
            # recorded but flag the case.
            if "Unknown chart type" in str(exc):
                unported += 1
                uninstall(assemble_mod, undo)
                continue
            failed.append((slug, traceback.format_exc(limit=4)))
            uninstall(assemble_mod, undo)
            continue
        except Exception:
            failed.append((slug, traceback.format_exc(limit=4)))
            uninstall(assemble_mod, undo)
            continue
        finally:
            uninstall(assemble_mod, undo)

        for name, calls in sink.items():
            call_totals[name] = call_totals.get(name, 0) + len(calls)

        payload = {"slug": slug, "chartType": doc.get("chartType"), "calls": sink}
        # Every call echoes the full data table, so an uncompressed corpus runs
        # to ~150 MB for 705 fixtures. gzip brings it into git-friendly range;
        # check_conformance.py reads either form.
        blob = json.dumps(payload, sort_keys=True, separators=(",", ":")).encode("utf-8")
        if args.no_gzip:
            (args.outdir / f"{slug}.json").write_bytes(blob)
        else:
            (args.outdir / f"{slug}.json.gz").write_bytes(gzip.compress(blob, 6))
        ok += 1

    manifest = {
        "generated_from": str(args.fixtures),
        "traced_functions": TRACED,
        "cases": ok,
        "unported_chart_types": unported,
        "calls_recorded": dict(sorted(call_totals.items())),
        "failed": [{"slug": s, "error": e.strip().splitlines()[-1]} for s, e in failed],
    }
    (args.outdir / "_manifest.json").write_text(json.dumps(manifest, indent=2), encoding="utf-8")

    print(f"core corpus -> {args.outdir}")
    print(f"  cases recorded : {ok}")
    print(f"  unported types : {unported}")
    print(f"  errored        : {len(failed)}")
    for name, n in sorted(call_totals.items()):
        print(f"  {name:28} {n:6d} calls")
    for slug, tb in failed[:5]:
        print(f"  FAIL {slug}: {tb.strip().splitlines()[-1]}")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
