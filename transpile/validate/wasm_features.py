#!/usr/bin/env python3
"""Check a .wasm binary against the proposal set Typst's runtime supports.

Typst executes plugins with `wasmi`. It implements the MVP plus a set of
finished proposals, but **not** GC, function-references, multi-memory, threads,
or the component model. A plugin that uses any of those loads fine in
wasmtime/browsers and then fails inside Typst, so this gate runs on every build.

The check is structural: it walks the binary's sections and type encodings
looking for the encodings only a post-MVP proposal can produce. It deliberately
does not need a wasm runtime -- adding one would mean pinning a *different*
engine's opinion of what is valid.

Usage:
    python wasm_features.py FILE.wasm [--json OUT.json]
"""

from __future__ import annotations

import argparse
import json
import struct
import sys
from pathlib import Path

# Sections wasmi accepts. Anything else is either a custom section (0, always
# ignorable) or something newer than the engine.
SECTION_NAMES = {
    0: "custom", 1: "type", 2: "import", 3: "function", 4: "table", 5: "memory",
    6: "global", 7: "export", 8: "start", 9: "element", 10: "code", 11: "data",
    12: "data-count", 13: "tag",
}

# Proposals wasmi does NOT implement -- presence here is a hard failure.
UNSUPPORTED = {
    "gc": "WasmGC (struct/array/i31 heap types) -- wasmi has no GC support",
    "function-references": "typed function references",
    "multi-memory": "more than one memory",
    "threads": "shared memory / atomic instructions",
    "exception-handling": "tag section / throw-ref",
    "component-model": "component binary, not a core module",
}

# Type-section leading bytes introduced by GC / function-references.
GC_TYPE_BYTES = {
    0x4E: "rec (recursive type group)",
    0x50: "sub (subtype declaration)",
    0x4F: "sub final",
    0x5E: "array type",
    0x5F: "struct type",
}
# Heap-type encodings from GC.
GC_HEAP_BYTES = {
    0x6A: "arrayref", 0x6B: "structref", 0x6C: "i31ref", 0x6D: "eqref",
    0x6E: "anyref", 0x71: "nullref", 0x72: "nullexternref", 0x73: "nullfuncref",
}


class Reader:
    def __init__(self, data: bytes) -> None:
        self.d = data
        self.i = 0

    def u8(self) -> int:
        b = self.d[self.i]
        self.i += 1
        return b

    def uleb(self) -> int:
        result = shift = 0
        while True:
            b = self.u8()
            result |= (b & 0x7F) << shift
            if not b & 0x80:
                return result
            shift += 7

    def bytes(self, n: int) -> bytes:
        out = self.d[self.i : self.i + n]
        self.i += n
        return out


def analyze(path: Path) -> dict[str, object]:
    data = path.read_bytes()
    findings: list[str] = []
    info: dict[str, object] = {"file": str(path), "size_bytes": len(data)}

    if data[:4] != b"\0asm":
        return {**info, "ok": False, "findings": ["not a wasm binary (bad magic)"]}
    version, layer = struct.unpack_from("<HH", data, 4)
    info["version"] = version
    if layer != 0:
        # Component binaries carry a non-zero layer field.
        findings.append(f"component-model: {UNSUPPORTED['component-model']} (layer={layer})")
        return {**info, "ok": False, "findings": findings}
    if version != 1:
        findings.append(f"unexpected core module version {version}")

    r = Reader(data)
    r.i = 8
    sections: dict[str, int] = {}
    memories = 0
    exports: list[str] = []

    while r.i < len(data):
        sid = r.u8()
        size = r.uleb()
        body = r.bytes(size)
        name = SECTION_NAMES.get(sid, f"unknown({sid})")
        sections[name] = sections.get(name, 0) + 1

        if sid == 13:
            findings.append(f"exception-handling: {UNSUPPORTED['exception-handling']}")
        elif sid not in SECTION_NAMES:
            findings.append(f"unknown section id {sid} -- newer than the engine")
        elif sid == 5:  # memory
            sr = Reader(body)
            memories = sr.uleb()
            for _ in range(memories):
                flags = sr.uleb()
                sr.uleb()  # min
                if flags & 0x01:
                    sr.uleb()  # max
                if flags & 0x02:
                    findings.append(f"threads: {UNSUPPORTED['threads']} (shared memory)")
        elif sid == 1:  # type
            sr = Reader(body)
            count = sr.uleb()
            for _ in range(count):
                lead = sr.u8()
                if lead in GC_TYPE_BYTES:
                    findings.append(f"gc: {UNSUPPORTED['gc']} -- found {GC_TYPE_BYTES[lead]}")
                    break
                if lead != 0x60:  # not a plain functype
                    findings.append(f"gc: unrecognised type-section form 0x{lead:02x}")
                    break
                # params, then results -- scan value types for GC heap types.
                for _side in range(2):
                    n = sr.uleb()
                    for _ in range(n):
                        vt = sr.u8()
                        if vt in GC_HEAP_BYTES:
                            findings.append(f"gc: {UNSUPPORTED['gc']} -- value type {GC_HEAP_BYTES[vt]}")
                        elif vt in (0x63, 0x64):  # (ref null ht) / (ref ht)
                            findings.append(
                                f"function-references: {UNSUPPORTED['function-references']}"
                            )
                            sr.u8()
        elif sid == 7:  # export
            sr = Reader(body)
            for _ in range(sr.uleb()):
                nlen = sr.uleb()
                nm = sr.bytes(nlen).decode("utf-8", "replace")
                sr.u8()  # kind
                sr.uleb()  # index
                exports.append(nm)

    if memories > 1:
        findings.append(f"multi-memory: {UNSUPPORTED['multi-memory']} ({memories} memories)")

    info["sections"] = sections
    info["memories"] = memories
    info["exports"] = sorted(exports)
    # Deduplicate -- a repeated GC type would otherwise report once per entry.
    info["findings"] = sorted(set(findings))
    info["ok"] = not info["findings"]
    return info


def main(argv: list[str] | None = None) -> int:
    ap = argparse.ArgumentParser(description=__doc__, formatter_class=argparse.RawDescriptionHelpFormatter)
    ap.add_argument("wasm", type=Path)
    ap.add_argument("--json", type=Path)
    args = ap.parse_args(argv)

    if not args.wasm.exists():
        print(f"no such file: {args.wasm}", file=sys.stderr)
        return 2

    info = analyze(args.wasm)
    if args.json:
        args.json.write_text(json.dumps(info, indent=2), encoding="utf-8")

    size_kb = int(info["size_bytes"]) / 1024
    print(f"{args.wasm}  ({size_kb:.1f} KiB)")
    print(f"  sections : {info.get('sections')}")
    print(f"  memories : {info.get('memories')}")
    print(f"  exports  : {', '.join(info.get('exports') or []) or '(none)'}")
    findings = info["findings"]
    if findings:
        print("  UNSUPPORTED BY TYPST'S wasmi RUNTIME:")
        for f in findings:  # type: ignore[union-attr]
            print(f"    - {f}")
        return 1
    print("  OK: MVP-compatible, no proposal wasmi lacks")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
