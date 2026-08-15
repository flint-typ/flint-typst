#!/usr/bin/env python3
"""Generate a Typst data module from real flint fixtures.

The visual tests started on six hand-made rows, which exercises the decisions
but not the *shapes* — a 200-point multi-series time series lays out very
differently from three bars, and tick density, overflow and label angling only
misbehave at scale.

Rather than invent large data, this lifts a few fixtures out of flint's own
corpus (`shared/test-data/`), so the visual tests draw exactly the inputs the
conformance suite already checks the decisions for.

Selection is by hand (see `FIXTURES`) rather than by rule: each entry is chosen
because it stresses something specific, and the reason is recorded next to it.

    python tests/gen_fixtures.py            # write tests/fixtures.typ
    python tests/gen_fixtures.py --check    # fail if stale (CI)
"""

from __future__ import annotations

import argparse
import json
import sys
from pathlib import Path

HERE = Path(__file__).resolve().parent
PKG = HERE.parent
CORPUS = PKG / "flint-source/shared/test-data"
OUT = HERE / "fixtures.typ"

# name in the generated module -> (fixture slug, why it is here, row cap)
FIXTURES = {
    "line-200": (
        "line_chart__01__t_q_color_n_4_200_pts",
        "200 points across 4 series on a temporal axis — the shape where tick "
        "density and series colouring actually matter",
        200,
    ),
    "line-sparse": (
        "line_chart__04__t_q_color_n_3_sparse_180_pts",
        "3 series that do not share x positions, so each line has its own gaps",
        180,
    ),
    "bar-20": (
        "bar_chart__01__n_20_q_20_pts",
        "20 categories — enough that the labels have to angle or the band has "
        "to widen",
        20,
    ),
    "bar-grouped": (
        "bar_chart__04__n_5_q_color_n_3_15_pts",
        "5 categories x 3 series — dodging with more than two series",
        15,
    ),
    "scatter-150": (
        "line_chart__14__q_q_color_n_3_150_pts",
        "150 points, quantitative on both axes, 3 series — no banding anywhere",
        150,
    ),
}


def typst_value(v) -> str:
    if isinstance(v, bool):
        return "true" if v else "false"
    if v is None:
        return "none"
    if isinstance(v, str):
        return '"' + v.replace("\\", "\\\\").replace('"', '\\"') + '"'
    if isinstance(v, int):
        return str(v)
    if isinstance(v, float):
        return repr(v)
    raise TypeError(f"unexpected cell type {type(v).__name__}: {v!r}")


def load(slug: str, cap: int) -> tuple[list[str], list[list], dict, dict]:
    doc = json.loads((CORPUS / slug / "input.json").read_text())
    spec = doc.get("input", doc)
    rows = ((spec.get("data") or {}).get("values") or [])[:cap]
    if not rows:
        raise SystemExit(f"{slug}: no rows")
    header = list(rows[0].keys())
    table = [[r.get(k) for k in header] for r in rows]
    encodings = (spec.get("chart_spec") or {}).get("encodings") or {}
    # Encodings arrive either as a bare field name or as an object.
    enc = {ch: (v if isinstance(v, str) else (v or {}).get("field")) for ch, v in encodings.items()}
    enc = {ch: f for ch, f in enc.items() if f}
    return header, table, enc, spec.get("semantic_types") or {}


def build() -> str:
    out = [
        "// GENERATED from flint's own fixture corpus by tests/gen_fixtures.py",
        "// -- do not edit.",
        "//",
        "// Regenerate:  python tests/gen_fixtures.py",
        "// Check drift: python tests/gen_fixtures.py --check",
        "//",
        "// These are the same inputs the conformance suite checks core's",
        "// decisions against, so a visual test on one of them is testing the",
        "// backend and nothing else.",
        "",
    ]
    for name, (slug, why, cap) in FIXTURES.items():
        header, table, enc, sem = load(slug, cap)
        ident = name.replace("-", "-")
        out.append(f"// {slug}")
        for line in why.split(" — "):
            out.append(f"// {line}")
        out.append(f"#let {ident} = (")
        out.append(f"  encodings: {typst_dict(enc)},")
        out.append(f"  semantic-types: {typst_dict(sem)},")
        out.append("  data: (")
        out.append("    (" + ", ".join(typst_value(h) for h in header) + "),")
        for row in table:
            out.append("    (" + ", ".join(typst_value(c) for c in row) + "),")
        out.append("  ),")
        out.append(")")
        out.append("")
    return "\n".join(out)


def typst_dict(d: dict) -> str:
    if not d:
        return "(:)"
    parts = []
    for k, v in d.items():
        key = k if (k.replace("_", "a").isalnum() and not k[0].isdigit()) else typst_value(k)
        parts.append(f"{key}: {typst_value(v) if not isinstance(v, dict) else typst_dict(v)}")
    return "(" + ", ".join(parts) + ")"


def main() -> int:
    ap = argparse.ArgumentParser(description=__doc__, formatter_class=argparse.RawDescriptionHelpFormatter)
    ap.add_argument("--check", action="store_true")
    args = ap.parse_args()

    if not CORPUS.exists():
        print(f"fixture corpus not found at {CORPUS}\nrun: make upstream", file=sys.stderr)
        return 2

    content = build()
    current = OUT.read_text(encoding="utf-8") if OUT.exists() else None
    if current == content:
        print(f"  unchanged {OUT.relative_to(PKG)}")
        return 0
    if args.check:
        print(f"STALE {OUT.relative_to(PKG)} -- run: python tests/gen_fixtures.py", file=sys.stderr)
        return 1
    OUT.write_text(content, encoding="utf-8")
    print(f"  written   {OUT.relative_to(PKG)}  ({len(content.splitlines())} lines)")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
