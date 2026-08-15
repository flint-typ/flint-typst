# transpile/ — flint core → Rust → wasm32 → Typst

Tooling for getting `flint-chart`'s semantic chart core running inside Typst,
so a lilaq backend can be built on top of it.

**Read [`REPORT.md`](REPORT.md) first** — it has the measurements, including
why the transpiler step does not currently work and what to do instead.

[`PORT-PLAN.md`](PORT-PLAN.md) is the plan for that port: layout, naming,
the divergence register, phases and gates.
[`PORTING-NOTES.md`](PORTING-NOTES.md) is the measured background behind its
speed rules — where core's time goes, and which Typst idioms are traps.

[`TYPST-VS-WASM.md`](TYPST-VS-WASM.md) answers the follow-up question: could
core just be written in Typst script directly, skipping wasm entirely?
**Yes — that is the current recommendation.** The wasm scaffold below stays as
a proven fallback. Evidence lives in `probe/typst/` (`make typst-probes`).

## Pipeline

```
flint-source/packages/flint-py/flint/core/     upstream, 13 files, 4106 LOC
        │
        ├─ audit/type_audit.py ──────────────▶ audit/report-core.md
        │      how much of it carries types a transpiler can lower
        │
        ├─ validate/make_core_corpus.py ─────▶ corpus/core/*.json.gz
        │      705 fixtures traced through the real pipeline: the oracle
        │
        └─ audit/flatten.py ─────────────────▶ typed/flint_core_flat.py
               one translation unit (depyler drops cross-module imports)
                    │
                    └─ depyler transpile ────▶ rust/src/generated/flint_core.rs
                                                  ✗ does not compile — see REPORT.md

rust/                          cdylib + rlib
 ├─ src/core/mod.rs             the port  ← the part that needs writing
 ├─ src/lib.rs                  #[wasm_func] JSON exports for Typst
 ├─ src/bin/replay.rs           native driver for corpus replay
 └─ src/generated/              depyler output, behind --features generated
        │
        ├─ cargo build --target wasm32-unknown-unknown --release
        │       └─▶ flint_core_wasm.wasm  (70 KiB, MVP-only)
        │              ├─ validate/wasm_features.py   does wasmi accept it?
        │              └─ validate/typst_smoke.typ    does Typst load it?
        │
        └─ cargo build --bin replay
                └─▶ validate/check_conformance.py     does it match Python?
```

## Getting started

```bash
make setup        # venv + depyler (builds from source, a few minutes)
make audit        # type-completeness report
make corpus       # record the 705-case oracle from flint-py
make build        # wasm plugin + native replay driver
make validate     # wasm feature gate against Typst's wasmi
make smoke        # load the plugin in a real `typst compile`
make conformance  # replay the corpus against the Rust core
make typst-probes # re-check Typst's capabilities (after a Typst upgrade)
```

`make all` runs everything except `conformance` and the transpiler targets.

## The work loop

`rust/src/core/mod.rs` currently stubs all five stages. To port one:

1. Implement it in `rust/src/core/`.
2. Add its name to `IMPLEMENTED_STAGES`.
3. `make conformance` — it is now checked against all 705 fixtures instead of
   skipped. Failures name the exact JSON path that diverged.

Stages are listed in dependency order in `IMPLEMENTED_STAGES`' doc comment.
Never add a name without an implementation: the harness treats a declared
stage that errors as a failure, which is the intended direction.

## Layout

| path | what |
|---|---|
| `audit/type_audit.py` | AST type-completeness + transpiler-hazard scorer, with `--min-score` for CI |
| `audit/flatten.py` | package → single module, topologically ordered, collision-checked |
| `audit/report-core.md` | generated |
| `validate/make_core_corpus.py` | traces flint-py's real pipeline into a golden corpus |
| `validate/check_conformance.py` | replays the corpus against the Rust core |
| `validate/wasm_features.py` | rejects any proposal Typst's wasmi lacks (GC, threads, multi-memory, tags) |
| `validate/typst_smoke.typ` | proves the wasm boundary works under real Typst |
| `rust/` | the plugin crate |
| `corpus/core/` | 705 recorded cases, 8.5 MiB gzipped |
| `probe/typst/` | capability probes (`run.sh`), `bench-*.typ` timings, `perf-*.typ` idiom tests, `BENCH.md` |
| `probe/depyler-out/` | transpiler output kept as evidence for REPORT.md |
| `depyler-src/`, `py2many-src/`, `tools-bin/` | vendored transpilers (gitignore these) |

## Environment notes

- **Two rusts.** Arch's `/usr/bin/rustc` has no `wasm32-unknown-unknown` std
  and shadows rustup's on `PATH`; rustup's `cargo` proxy resolves `rustc` from
  `PATH` too. The `Makefile` points `CARGO` *and* `RUSTC` into
  `~/.rustup/toolchains/stable-*/bin` explicitly. Building by hand outside
  `make` will fail with `can't find crate for 'std'` unless you do the same.
- Verified against typst 0.15.1, rustc 1.97.1, depyler 4.1.2, py2many 0.9.
- The corpus is regenerated from upstream, not authored here — re-run
  `make corpus` after any `flint-source` bump and read the diff.
