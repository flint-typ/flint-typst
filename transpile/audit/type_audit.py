#!/usr/bin/env python3
"""Type-completeness audit for a Python package, scored for transpilability.

A source-to-source Python->Rust transpiler (depyler, py2many) can only emit a
concrete Rust type where the Python source names a concrete type. `Any`,
`dict[str, Any]`, bare `dict`/`list`, and missing annotations all degrade to
"unknown" and either fail the transpile or force a dynamic-value fallback.

This walks the AST and reports, per file:
  * annotation coverage   -- params/returns/attributes carrying an annotation
  * annotation *concreteness* -- of those, how many are Any-free
  * transpiler hazards    -- constructs the tools document as unsupported or
                             that reliably produce garbage output

Usage:
    python type_audit.py PATH [PATH ...] [--json OUT.json] [--md OUT.md]
"""

from __future__ import annotations

import argparse
import ast
import json
import sys
from collections import Counter
from dataclasses import asdict, dataclass, field
from pathlib import Path

# Annotations that carry no information a transpiler can lower to a Rust type.
OPAQUE_NAMES = {"Any", "object"}
# Bare containers: element type unknown.
BARE_CONTAINERS = {"dict", "list", "set", "tuple", "frozenset", "Dict", "List", "Set", "Tuple"}


def _ann_is_opaque(node: ast.AST | None) -> bool:
    """True if the annotation resolves to something with no concrete Rust type."""
    if node is None:
        return True
    for sub in ast.walk(node):
        if isinstance(sub, ast.Name) and sub.id in OPAQUE_NAMES:
            return True
        if isinstance(sub, ast.Attribute) and sub.attr in OPAQUE_NAMES:
            return True
        if isinstance(sub, ast.Constant) and isinstance(sub.value, str):
            # Stringified forward ref -- reparse and recurse.
            try:
                inner = ast.parse(sub.value, mode="eval").body
            except SyntaxError:
                return True
            if _ann_is_opaque(inner):
                return True
    # Bare `dict` / `list` with no subscript.
    if isinstance(node, ast.Name) and node.id in BARE_CONTAINERS:
        return True
    return False


@dataclass
class FileReport:
    path: str
    loc: int = 0
    functions: int = 0
    params: int = 0
    params_annotated: int = 0
    params_concrete: int = 0
    returns: int = 0
    returns_annotated: int = 0
    returns_concrete: int = 0
    classes: int = 0
    dataclasses: int = 0
    typeddicts: int = 0
    namedtuples: int = 0
    hazards: dict[str, int] = field(default_factory=dict)

    @property
    def concrete_score(self) -> float:
        total = self.params + self.returns
        if total == 0:
            return 1.0
        return (self.params_concrete + self.returns_concrete) / total


class Auditor(ast.NodeVisitor):
    def __init__(self, rep: FileReport) -> None:
        self.rep = rep
        self.hz: Counter[str] = Counter()
        self.depth = 0

    # -- helpers ----------------------------------------------------------
    def _record_args(self, node: ast.FunctionDef | ast.AsyncFunctionDef) -> None:
        a = node.args
        allargs = [*a.posonlyargs, *a.args, *a.kwonlyargs]
        if a.vararg:
            self.hz["star_args"] += 1
            allargs.append(a.vararg)
        if a.kwarg:
            self.hz["star_kwargs"] += 1
            allargs.append(a.kwarg)
        for i, arg in enumerate(allargs):
            if arg.arg in ("self", "cls") and i == 0:
                continue
            self.rep.params += 1
            if arg.annotation is not None:
                self.rep.params_annotated += 1
                if not _ann_is_opaque(arg.annotation):
                    self.rep.params_concrete += 1

    # -- visitors ---------------------------------------------------------
    def visit_FunctionDef(self, node: ast.FunctionDef | ast.AsyncFunctionDef) -> None:
        self.rep.functions += 1
        if self.depth > 0:
            self.hz["nested_function"] += 1
        self._record_args(node)
        self.rep.returns += 1
        if node.returns is not None:
            self.rep.returns_annotated += 1
            if not _ann_is_opaque(node.returns):
                self.rep.returns_concrete += 1
        self.depth += 1
        self.generic_visit(node)
        self.depth -= 1

    visit_AsyncFunctionDef = visit_FunctionDef  # type: ignore[assignment]

    def visit_ClassDef(self, node: ast.ClassDef) -> None:
        self.rep.classes += 1
        deco = {ast.unparse(d).split("(")[0].split(".")[-1] for d in node.decorator_list}
        bases = {ast.unparse(b).split("[")[0].split(".")[-1] for b in node.bases}
        if "dataclass" in deco:
            self.rep.dataclasses += 1
        if "TypedDict" in bases:
            self.rep.typeddicts += 1
        if "NamedTuple" in bases:
            self.rep.namedtuples += 1
        if len([b for b in node.bases if ast.unparse(b) != "object"]) > 1:
            self.hz["multiple_inheritance"] += 1
        self.depth += 1
        self.generic_visit(node)
        self.depth -= 1

    def visit_Lambda(self, node: ast.Lambda) -> None:
        self.hz["lambda"] += 1
        self.generic_visit(node)

    def visit_Try(self, node: ast.Try) -> None:
        self.hz["try_except"] += 1
        self.generic_visit(node)

    def visit_Call(self, node: ast.Call) -> None:
        fn = node.func
        name = fn.id if isinstance(fn, ast.Name) else (fn.attr if isinstance(fn, ast.Attribute) else "")
        if name == "isinstance":
            self.hz["isinstance_dispatch"] += 1
        elif name in ("eval", "exec", "compile", "getattr", "setattr", "globals", "locals", "vars"):
            self.hz[f"dynamic_{name}"] += 1
        elif name == "get" and isinstance(fn, ast.Attribute):
            self.hz["dict_get"] += 1
        elif name in ("deepcopy", "copy"):
            self.hz["copy_semantics"] += 1
        self.generic_visit(node)

    def visit_JoinedStr(self, node: ast.JoinedStr) -> None:
        self.hz["fstring"] += 1
        self.generic_visit(node)

    def visit_Global(self, node: ast.Global) -> None:
        self.hz["global_stmt"] += 1
        self.generic_visit(node)

    def visit_Nonlocal(self, node: ast.Nonlocal) -> None:
        self.hz["nonlocal_stmt"] += 1
        self.generic_visit(node)

    def visit_Yield(self, node: ast.Yield) -> None:
        self.hz["generator"] += 1
        self.generic_visit(node)


def audit_file(path: Path, root: Path) -> FileReport:
    src = path.read_text(encoding="utf-8")
    rep = FileReport(path=str(path.relative_to(root)), loc=len(src.splitlines()))
    tree = ast.parse(src, filename=str(path))
    aud = Auditor(rep)
    aud.visit(tree)
    rep.hazards = dict(sorted(aud.hz.items(), key=lambda kv: -kv[1]))
    return rep


def collect(paths: list[Path]) -> tuple[list[FileReport], Path]:
    files: list[Path] = []
    for p in paths:
        if p.is_dir():
            files.extend(sorted(p.rglob("*.py")))
        else:
            files.append(p)
    root = Path.cwd()
    try:
        root = Path(min((str(p.parent) for p in files), key=len))
    except ValueError:
        pass
    return [audit_file(f, root.parent if root.name else root) for f in files], root


def totals(reps: list[FileReport]) -> FileReport:
    t = FileReport(path="TOTAL")
    hz: Counter[str] = Counter()
    for r in reps:
        t.loc += r.loc
        t.functions += r.functions
        t.params += r.params
        t.params_annotated += r.params_annotated
        t.params_concrete += r.params_concrete
        t.returns += r.returns
        t.returns_annotated += r.returns_annotated
        t.returns_concrete += r.returns_concrete
        t.classes += r.classes
        t.dataclasses += r.dataclasses
        t.typeddicts += r.typeddicts
        t.namedtuples += r.namedtuples
        hz.update(r.hazards)
    t.hazards = dict(hz.most_common())
    return t


def pct(n: int, d: int) -> str:
    return "n/a" if d == 0 else f"{100 * n / d:5.1f}%"


def render_md(reps: list[FileReport], t: FileReport) -> str:
    out: list[str] = []
    out.append("# Type-completeness audit\n")
    out.append(
        "`annotated` = has any annotation. `concrete` = annotation contains no "
        "`Any`/`object` and no bare container — i.e. a transpiler can lower it "
        "to a real Rust type.\n"
    )
    out.append("| file | loc | fns | params ann/concrete | returns ann/concrete | concrete score |")
    out.append("|---|---:|---:|---:|---:|---:|")
    for r in sorted(reps, key=lambda r: r.concrete_score):
        out.append(
            f"| `{r.path}` | {r.loc} | {r.functions} | "
            f"{pct(r.params_annotated, r.params)} / {pct(r.params_concrete, r.params)} | "
            f"{pct(r.returns_annotated, r.returns)} / {pct(r.returns_concrete, r.returns)} | "
            f"**{r.concrete_score * 100:.1f}%** |"
        )
    out.append(
        f"| **TOTAL** | **{t.loc}** | **{t.functions}** | "
        f"**{pct(t.params_annotated, t.params)} / {pct(t.params_concrete, t.params)}** | "
        f"**{pct(t.returns_annotated, t.returns)} / {pct(t.returns_concrete, t.returns)}** | "
        f"**{t.concrete_score * 100:.1f}%** |"
    )
    out.append("\n## Nominal types available to a transpiler\n")
    out.append(f"- classes: **{t.classes}**")
    out.append(f"- dataclasses: **{t.dataclasses}**")
    out.append(f"- TypedDicts: **{t.typeddicts}**")
    out.append(f"- NamedTuples: **{t.namedtuples}**")
    out.append("\n## Transpiler hazards\n")
    out.append("| construct | count |")
    out.append("|---|---:|")
    for k, v in t.hazards.items():
        out.append(f"| `{k}` | {v} |")
    return "\n".join(out) + "\n"


def main(argv: list[str] | None = None) -> int:
    ap = argparse.ArgumentParser(description=__doc__, formatter_class=argparse.RawDescriptionHelpFormatter)
    ap.add_argument("paths", nargs="+", type=Path)
    ap.add_argument("--json", type=Path, help="write machine-readable report here")
    ap.add_argument("--md", type=Path, help="write markdown report here")
    ap.add_argument(
        "--min-score",
        type=float,
        default=None,
        help="exit non-zero if the total concrete score falls below this (0..1). "
        "Use as a CI ratchet while typed/ is being built up.",
    )
    args = ap.parse_args(argv)

    reps, _ = collect(args.paths)
    if not reps:
        print("no .py files found", file=sys.stderr)
        return 2
    t = totals(reps)

    md = render_md(reps, t)
    if args.md:
        args.md.write_text(md, encoding="utf-8")
    else:
        print(md)
    if args.json:
        args.json.write_text(
            json.dumps({"files": [asdict(r) for r in reps], "total": asdict(t)}, indent=2),
            encoding="utf-8",
        )

    if args.min_score is not None and t.concrete_score < args.min_score:
        print(
            f"FAIL: concrete score {t.concrete_score:.3f} < required {args.min_score:.3f}",
            file=sys.stderr,
        )
        return 1
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
