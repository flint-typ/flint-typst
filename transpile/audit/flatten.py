#!/usr/bin/env python3
"""Flatten a Python package into a single module for single-file transpilers.

depyler 4.1.2 transpiles one file at a time and drops cross-module imports
(it emits `// NOTE: Map Python module 'decisions'` and no `use`), so every
call across a package boundary becomes an unresolved symbol in the generated
Rust. Flattening the package into one module sidesteps that: the transpiler
then sees a single translation unit with every definition in scope.

The package's intra-package import graph must be acyclic; modules are emitted
in topological order so definitions precede their uses. Top-level name
collisions between modules are a hard error -- flattening cannot resolve them
and silently shadowing one would change behaviour.

Usage:
    python flatten.py PKG_DIR -o OUT.py [--check-only]
"""

from __future__ import annotations

import argparse
import ast
import sys
from pathlib import Path

STDLIB_HEADER = "from __future__ import annotations\n"


def module_deps(tree: ast.Module) -> set[str]:
    """Intra-package (relative) imports, by module name."""
    out: set[str] = set()
    for node in ast.walk(tree):
        if isinstance(node, ast.ImportFrom) and node.level:
            out.add(node.module or "__init__")
    return out


def absolute_imports(tree: ast.Module) -> list[str]:
    """Non-relative import statements, as source lines."""
    out: list[str] = []
    for node in tree.body:
        if isinstance(node, ast.Import):
            out.append(ast.unparse(node))
        elif isinstance(node, ast.ImportFrom) and not node.level:
            if node.module == "__future__":
                continue
            out.append(ast.unparse(node))
    return out


def toplevel_names(tree: ast.Module) -> set[str]:
    out: set[str] = set()
    for node in tree.body:
        if isinstance(node, (ast.FunctionDef, ast.AsyncFunctionDef, ast.ClassDef)):
            out.add(node.name)
        elif isinstance(node, ast.Assign):
            for t in node.targets:
                for sub in ast.walk(t):
                    if isinstance(sub, ast.Name):
                        out.add(sub.id)
        elif isinstance(node, ast.AnnAssign) and isinstance(node.target, ast.Name):
            out.add(node.target.id)
    return out


def body_without_imports(tree: ast.Module, src: str) -> str:
    """Source of the module with all top-level import statements removed."""
    keep: list[ast.stmt] = []
    for node in tree.body:
        if isinstance(node, (ast.Import, ast.ImportFrom)):
            continue
        # Drop the module docstring; it becomes noise once concatenated.
        if isinstance(node, ast.Expr) and isinstance(node.value, ast.Constant) and isinstance(node.value.value, str):
            if not keep:
                continue
        keep.append(node)
    return "\n".join(ast.unparse(n) for n in keep)


def topo_sort(deps: dict[str, set[str]]) -> list[str]:
    order: list[str] = []
    seen: set[str] = set()
    temp: list[str] = []

    def visit(m: str) -> None:
        if m in seen:
            return
        if m in temp:
            raise SystemExit(f"import cycle: {' -> '.join(temp)} -> {m}")
        temp.append(m)
        for d in sorted(deps.get(m, ())):
            if d in deps:
                visit(d)
        temp.pop()
        seen.add(m)
        order.append(m)

    for m in sorted(deps):
        visit(m)
    return order


def main(argv: list[str] | None = None) -> int:
    ap = argparse.ArgumentParser(description=__doc__, formatter_class=argparse.RawDescriptionHelpFormatter)
    ap.add_argument("pkg", type=Path, help="package directory containing .py modules")
    ap.add_argument("-o", "--output", type=Path, help="write flattened module here")
    ap.add_argument("--check-only", action="store_true", help="report collisions/order, write nothing")
    args = ap.parse_args(argv)

    files = {p.stem: p for p in sorted(args.pkg.glob("*.py"))}
    if not files:
        print(f"no .py files in {args.pkg}", file=sys.stderr)
        return 2

    trees = {m: ast.parse(p.read_text(encoding="utf-8"), filename=str(p)) for m, p in files.items()}
    deps = {m: module_deps(t) for m, t in trees.items()}
    order = topo_sort(deps)

    # Collision check -- flattening cannot resolve shadowed top-level names.
    owner: dict[str, str] = {}
    collisions: list[str] = []
    for m in order:
        for name in sorted(toplevel_names(trees[m])):
            if name in owner:
                collisions.append(f"{name!r}: {owner[name]} and {m}")
            else:
                owner[name] = m
    if collisions:
        print("top-level name collisions (must be renamed upstream before flattening):", file=sys.stderr)
        for c in collisions:
            print(f"  {c}", file=sys.stderr)
        return 1

    imports: list[str] = []
    for m in order:
        for line in absolute_imports(trees[m]):
            if line not in imports:
                imports.append(line)

    if args.check_only:
        print(f"order: {order}")
        print(f"top-level names: {len(owner)}, collisions: 0")
        print("stdlib imports:")
        for line in imports:
            print(f"  {line}")
        return 0

    parts = [
        f'"""Flattened from {args.pkg} -- generated by audit/flatten.py, do not edit."""',
        STDLIB_HEADER.rstrip(),
        *imports,
        "",
    ]
    for m in order:
        parts.append(f"# {'=' * 70}\n# module: {m}\n# {'=' * 70}")
        parts.append(body_without_imports(trees[m], files[m].read_text(encoding="utf-8")))
        parts.append("")
    text = "\n".join(parts) + "\n"

    # Sanity: the flattened module must still parse.
    ast.parse(text)

    if args.output:
        args.output.parent.mkdir(parents=True, exist_ok=True)
        args.output.write_text(text, encoding="utf-8")
        print(f"wrote {args.output} ({len(text.splitlines())} lines, {len(order)} modules)")
    else:
        sys.stdout.write(text)
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
