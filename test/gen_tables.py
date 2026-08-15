#!/usr/bin/env python3
"""Generate Typst source for flint core's pure-data tables.

Some of what core holds is not logic but lookup tables — `TYPE_REGISTRY` alone
is 45 entries of nine fields. Hand-transcribing those into Typst would be the
highest-risk, lowest-value work in the whole port: a single mistyped
`"intensive"` produces a wrong chart that no reviewer would spot.

So they are generated from flint-py instead. That removes transcription risk
entirely, and on an upstream bump the table is a re-run rather than a manual
diff. The generated file carries a header saying so.

Only *data* is generated. Every function is hand-ported, because functions are
where the interesting decisions live and where a reviewer needs to read the two
side by side.

    python test/gen_tables.py            # write the files
    python test/gen_tables.py --check    # fail if they are out of date (CI)
"""

from __future__ import annotations

import argparse
import sys
from pathlib import Path

HERE = Path(__file__).resolve().parent
PKG = HERE.parent
sys.path.insert(0, str(PKG / "flint-source/packages/flint-py"))

CORE = PKG / "src/core"


def typst_value(v, indent: int = 0) -> str:
    """Render a Python value as a Typst literal."""
    pad = "  " * indent
    inner = "  " * (indent + 1)
    if isinstance(v, bool):
        return "true" if v else "false"
    if v is None:
        return "none"
    if isinstance(v, str):
        return '"' + v.replace("\\", "\\\\").replace('"', '\\"') + '"'
    if isinstance(v, int):
        return str(v)
    if isinstance(v, float):
        # Typst prints an integral float without the .0, but the *type* still
        # differs from an int and the differ checks that, so keep it explicit.
        return repr(v)
    if isinstance(v, (list, tuple)):
        if not v:
            return "()"
        if all(isinstance(x, (str, int, float, bool)) for x in v):
            body = ", ".join(typst_value(x) for x in v)
            return f"({body})" if len(v) > 1 else f"({body},)"
        items = ",\n".join(inner + typst_value(x, indent + 1) for x in v)
        return "(\n" + items + ",\n" + pad + ")"
    if isinstance(v, dict):
        if not v:
            return "(:)"
        items = []
        for k, val in v.items():
            key = str(k)
            # Typst dict keys are bare identifiers when they look like one,
            # quoted otherwise. Upstream keys are camelCase and safe, but do
            # not assume it.
            if key.replace("-", "_").replace("_", "a").isalnum() and not key[0].isdigit():
                items.append(f"{inner}{key}: {typst_value(val, indent + 1)}")
            else:
                items.append(f"{inner}{typst_value(key)}: {typst_value(val, indent + 1)}")
        return "(\n" + ",\n".join(items) + ",\n" + pad + ")"
    raise TypeError(f"cannot render {type(v).__name__} as Typst: {v!r}")


def header(upstream: str) -> str:
    return (
        f"// GENERATED from {upstream} by test/gen_tables.py -- do not edit.\n"
        f"//\n"
        f"// Regenerate after any upstream change:  python test/gen_tables.py\n"
        f"// Check for drift in CI:                 python test/gen_tables.py --check\n"
        f"//\n"
        f"// Only pure-data tables are generated. Functions in this module are\n"
        f"// hand-ported and live alongside this file.\n\n"
    )


def build() -> dict[Path, str]:
    from flint.core import type_registry as tr
    from flint.core import types as ty

    out: dict[Path, str] = {}

    # --- type-registry data ------------------------------------------------
    body = header("flint/core/type_registry.py")
    body += "/// Every recognised semantic type, with its tier membership and the\n"
    body += "/// orthogonal dimensions the compiler reads off it.\n"
    body += "#let TYPE_REGISTRY = " + typst_value(tr.TYPE_REGISTRY) + "\n\n"
    body += "/// Fallback entry for a semantic type not in the registry.\n"
    body += "#let UNKNOWN_ENTRY = " + typst_value(tr.UNKNOWN_ENTRY) + "\n"
    out[CORE / "type-registry-data.typ"] = body

    # --- semantic-types data ----------------------------------------------
    from flint.core import semantic_types as st

    body = header("flint/core/semantic_types.py")
    body += "/// Every semantic type name, as an identity map (upstream keeps the\n"
    body += "/// constants in a dict so callers can write `SemanticTypes.Price`).\n"
    body += "#let SemanticTypes = " + typst_value(st.SemanticTypes) + "\n\n"
    body += "/// Canonical orderings for the ordinal types that have one. Tried in\n"
    body += "/// order; the first sequence matching enough of the data wins.\n"
    body += "#let _ORDINAL_SEQUENCES = " + typst_value(st._ORDINAL_SEQUENCES) + "\n"
    out[CORE / "semantic-types-data.typ"] = body

    # --- channel tables ----------------------------------------------------
    body = header("flint/core/types.py")
    body += "/// Every encoding channel the compiler knows about.\n"
    body += "#let channels = " + typst_value(ty.channels) + "\n\n"
    body += "/// Channels grouped by the part of the chart they drive.\n"
    body += "#let channelGroups = " + typst_value(ty.channelGroups) + "\n"
    out[CORE / "types.typ"] = body

    return out


def main() -> int:
    ap = argparse.ArgumentParser(description=__doc__, formatter_class=argparse.RawDescriptionHelpFormatter)
    ap.add_argument("--check", action="store_true", help="exit non-zero if any file is out of date")
    args = ap.parse_args()

    files = build()
    stale: list[Path] = []
    for path, content in files.items():
        current = path.read_text(encoding="utf-8") if path.exists() else None
        if current == content:
            status = "unchanged"
        else:
            stale.append(path)
            status = "STALE" if args.check else "written"
            if not args.check:
                path.parent.mkdir(parents=True, exist_ok=True)
                path.write_text(content, encoding="utf-8")
        print(f"  {status:9} {path.relative_to(PKG)}  ({len(content.splitlines())} lines)")

    if args.check and stale:
        print(f"\n{len(stale)} generated file(s) out of date -- run: python test/gen_tables.py", file=sys.stderr)
        return 1
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
