# flint-py → Rust → wasm32: feasibility findings

Everything below was measured on this machine against
`microsoft/flint-chart` @ `flint-source/` (shallow clone, 2026-08-15).
Regenerate any number with the `make` target named in its section.

**Verdict up front.** The wasm/Typst half of the plan is settled and working:
a Rust `cdylib` builds to a 70 KiB MVP-only wasm module that Typst 0.15.1
loads and calls. The transpiler half does not hold up: neither py2many nor
depyler produces compilable Rust from flint-py's core, and depyler fails
*silently* in a way that would ship wrong charts rather than a build error.
The recommendation is to keep the pipeline and drop the transpiler — details
in [What to do next](#what-to-do-next).

---

## 1. Type completeness of flint-py

`make audit` → `audit/report-core.md`

Scope note: only `flint/core/` matters. The `flint/vegalite/` half (2 900 LOC,
23 templates) is the backend we are *replacing* with lilaq, so its type debt
is irrelevant. Auditing the whole package reports 4 383 strict errors; core
alone reports 384.

| metric | core (`flint/core/`) |
|---|---|
| files / LOC / functions | 13 / 4 106 / 136 |
| params **annotated** | 96.3 % |
| params **concrete** (annotation free of `Any`, `object`, bare containers) | **57.9 %** |
| returns **annotated** | 97.8 % |
| returns **concrete** | **72.1 %** |
| overall concreteness | **62.7 %** |
| pyright `--strict` diagnostics | 384 |
| pyright `basic` diagnostics | 15 |

The headline is not the percentage, it is this:

| nominal type | count |
|---|---|
| classes | **0** |
| dataclasses | **0** |
| TypedDicts | **0** |
| NamedTuples | **0** |

The port is annotated but *structurally untyped*. `types.py` says it outright:
"all TS interfaces are erased." Every payload crossing a function boundary is
`dict[str, Any]`, and every field access is a runtime string lookup:

| transpiler hazard | count |
|---|---|
| `.get()` dynamic dict access | 379 |
| `isinstance` runtime dispatch | 65 |
| f-strings | 18 |
| nested functions / closures | 17 |
| `try`/`except` | 14 |
| lambdas | 5 |

This is the opposite of what depyler's README asks for. The premise that
flint-py "mirrors the typed TS API so it is probably written with type hints
and dataclasses" turned out to be half right: the hints are there, the
dataclasses are not.

External dependencies: exactly one, `python-dateutil`, at a single
function-local import site in `js_date.py`. Core is otherwise stdlib-only —
genuinely good news for any port.

### Functional completeness vs the JS reference

| suite | result |
|---|---|
| `tests/unit` | **75 / 75 pass** |
| `tests/test_fixtures.py` (full Vega-Lite spec diff) | 600 pass / 180 fail |

The 180 failures are **not** distributed evenly, and the distribution is the
useful part. Aggregating every diff path:

- `$.data.values[N].<field>` — ~1 200 diffs, i.e. `filter_overflow` and the
  temporal data transform
- `$.encoding.*`, `$.layer[N].mark` — the vegalite template layer we are
  discarding anyway
- `$.width`, `$.height` — **zero diffs**

`compute_layout`, the 1 241-line module a lilaq backend leans on hardest,
reproduces the JS reference exactly across all 705 traced fixtures. That is
the single most encouraging measurement in this report.

### Which JS core modules exist in Python

11 of the JS core's 19 modules are ported (plus `js_date.py`, which has no TS
counterpart). Missing:

| module | TS LOC | needed for a lilaq backend? |
|---|---:|---|
| `recommendation` | 1 238 | no — chart-type suggestion, not rendering |
| `pivot` | 992 | **likely yes** for wide-form input |
| `chart-type-recommendation` | 373 | no |
| `static-series` | 289 | **yes** for multi-series without a color field |
| `index` | 214 | no — re-exports |
| `band-dodge` | 177 | **yes** for grouped bars |
| `chart-transitions` | 166 | no — animation |
| `aggregate` | 116 | **yes** |
| `normalize-properties` | 92 | probably |
| `axis-detection` | 89 | probably |

So flint-py is not a complete core port, and roughly 1 700 LOC of TS behind
the "yes/probably" rows will have to be ported by hand eventually regardless
of which route is taken.

---

## 2. py2many 0.9

`probe/py2many-out/`

Dead end, quickly. It has no representation for `dict[str, Any]` and emits a
placeholder that is not valid Rust syntax:

```rust
pub fn _decide_color_for_channel(channel: &str, ctx: &_) -> Option<_<['&str', 'Any']>> {
```

Of four probe files, only `types.py` — 24 lines of string constants, zero
`Any` — produced output that `rustfmt` would accept. It also emits reserved
words as identifiers (`let override = ...`) and `pub const VisCategory = str;`
for type aliases. Not pursued further.

---

## 3. depyler 4.1.2

`make transpile-probe`, `make build-generated`

Depyler is a substantially more serious tool, and it does one important thing
right that I did not expect: it ships a `DepylerValue` dynamic-value enum
(Int/Float/Str/Bool/None/List/Dict/Tuple) and maps `dict[str, Any]` onto
`HashMap<String, DepylerValue>`. The `compute_layout` signature it produces is
genuinely reasonable:

```rust
pub fn compute_layout<'c, 'a, 'l2, 'b, 'l1, 'l3>(
    channel_semantics: &'a HashMap<String, HashMap<String, DepylerValue>>,
    declaration: &'b HashMap<String, DepylerValue>,
    table: &'c Vec<HashMap<String, DepylerValue>>,
    canvas_size: &'l1 HashMap<String, f64>,
    options: &'l2 mut Option<HashMap<String, DepylerValue>>,
    facet_grid: &'l3 Option<HashMap<String, i32>>,
) -> Result<HashMap<String, DepylerValue>, Box<dyn std::error::Error>>
```

So the *type-mapping* strategy is sound. The body codegen is not.

### It reports success it has not earned

| stage | result |
|---|---|
| `depyler check` on all 13 core files | ✓ "can be transpiled" for every one |
| `depyler transpile` on all 13 | ✓ 13/13, no errors |
| `depyler transpile` on the flattened module | ✓ all 119 top-level functions emitted, 0 missing |
| `cargo build` | ✗ **syntax error at `flint_core.rs:78`** |

### The silent failure

This is the finding that decides it. Depyler replaced **26 module-level data
tables** with empty vectors, emitting no diagnostic:

```rust
pub static TYPE_REGISTRY:  LazyLock<Vec<String>> = LazyLock::new(|| Vec::new());
pub static _ISO_DATE_ONLY: LazyLock<Vec<String>> = LazyLock::new(|| Vec::new());
pub static CURRENCY_MAP:   LazyLock<Vec<String>> = LazyLock::new(|| Vec::new());
```

`TYPE_REGISTRY` *is* flint's semantics — the table every `resolve_*` decision
reads. Also emptied: all four compiled regexes, `UNIT_SUFFIX_MAP`,
`SEMANTIC_LEVEL`, `_ORDINAL_SEQUENCES`, `DEFAULT_BASE_SIZE`,
`DEFAULT_GAS_PRESSURE_PARAMS`, and the month/weekday/quarter/compass label
tables. Had the syntax error not stopped the build, this would have compiled
into a plugin that runs and produces confidently wrong charts.

A transpiler that drops data silently is disqualifying for an unattended
pipeline, independent of how many compile errors remain.

### Error volume

Per-file `cargo build` counts. The caveat matters: rustc aborts at the first
*parse* error, so a low number means "stopped early", not "nearly there".

| file | py LOC | rs LOC | cargo errors | note |
|---|---:|---:|---:|---|
| `type_registry` | 119 | 5 255 | **619** | only file that parsed far enough to type-check |
| `field_semantics` | 596 | 3 574 | 187 | |
| `decisions` | 457 | 3 459 | 149 | |
| `color_decisions` | 95 | 3 674 | 42 | |
| `js_date` | 198 | 2 805 | 37 | |
| `encoding_overrides` | 38 | 3 393 | 4 | |
| `encoding_actions` | 116 | 3 625 | 3 | |
| `compute_layout` | 1 241 | 4 315 | 2 | parse abort |
| `resolve_semantics` | 453 | 4 621 | 2 | parse abort |
| `semantic_types` | 549 | 5 004 | 2 | parse abort |
| `filter_overflow` | 210 | 2 846 | 2 | parse abort |
| `__init__` | 10 | 3 372 | 2 | parse abort |
| `types` | 24 | 3 446 | **0** | constants only |

619 errors from 119 lines of Python is the honest extrapolation base. Scaling
that to 4 106 lines is not a fixup job; it is a rewrite performed through a
lossy intermediate, with no way to keep it in sync with upstream afterwards.

### Cross-module imports

Depyler drops them (`// NOTE: Map Python module 'decisions' (tracked in
DEPYLER-0424)`), so every cross-file call becomes an unresolved symbol. That
is why `audit/flatten.py` exists: flint core's import graph is acyclic and its
167 top-level names collide zero times, so the package collapses cleanly into
one 2 576-line translation unit. The flattener works and is worth keeping
regardless of which route wins — but it only removes one obstacle out of
several.

---

## 4. What does work: the wasm/Typst path

`make build validate smoke`

| check | result |
|---|---|
| `cargo build --target wasm32-unknown-unknown --release` | ✓ |
| binary size | **70.0 KiB** (`opt-level="z"`, LTO, `panic=abort`, stripped) |
| wasm feature audit vs wasmi | ✓ MVP only — 1 memory, no GC / threads / multi-memory / tags |
| loads in `typst compile` (0.15.1) | ✓ |
| JSON-in / JSON-out ABI round-trip | ✓ |
| structured error path (no trap on unimplemented stage) | ✓ |

The handoff's core premise is confirmed: Rust on `wasm32-unknown-unknown`
sidesteps wasmi's missing GC proposal, and `wasm-minimal-protocol` gives
Typst-callable exports with no ceremony. Nothing about the target environment
is a blocker.

One environment gotcha, now handled in the `Makefile`: Arch's system rust at
`/usr/bin/rustc` has no `wasm32-unknown-unknown` std and shadows rustup's on
`PATH`. rustup's `cargo` proxy resolves `rustc` from `PATH` too, so both
`cargo` *and* `RUSTC` must point into the toolchain directory explicitly.

---

## 5. The conformance corpus

`make corpus conformance`

Rather than reimplement flint's Phase-0/Phase-1 sequence (which is genuinely
coupled to the vegalite template registry), `test/make_corpus.py`
wraps the five core entry points with recorders and runs the *real* assembler
over every fixture.

| | |
|---|---|
| fixtures traced | **705** (0 errors, 0 unported chart types) |
| calls recorded | 4 230 |
| stages pinned | `convert_temporal_data`, `resolve_channel_semantics`, `compute_channel_budgets`, `filter_overflow`, `compute_layout` |
| corpus size | 8.5 MiB gzipped (148 MiB raw) |

`validate/check_conformance.py` replays those inputs through the Rust core and
diffs. It is **verified non-vacuous**: with `convert_temporal_data` stubbed as
a passthrough, it reports 672 pass / 33 fail and names the exact divergence —

```
[convert_temporal_data] bump_chart__01__temporal_rank_color_yearly_ranking
  $[0].Year: 2015 != '2015'
```

— which is precisely the int→string year coercion that stub omits. Stages a
build declares unimplemented are skipped and counted separately, so a partial
port can never report as green.

Two details the harness pins deliberately: `bool` is compared before `int`
(JSON `true` vs `1` is a real divergence), and int-vs-float is a failure
rather than a rounding artifact, because layout output feeds pixel geometry.

Non-finite floats needed an ABI decision. Flint uses `Infinity` as "no cap",
but bare `Infinity` is invalid JSON that both `serde_json` and Typst's
`json()` reject. Corpus, plugin ABI and Rust core now agree on a tagged
encoding — `{"$f":"inf"}` / `{"$f":"-inf"}` / `{"$f":"nan"}` — defined in
`make_core_corpus.py` and `rust/src/core/mod.rs`.

---

## What to do next

The transpiler was the means, not the goal. It does not work, but the two
things that make the goal reachable both do: a proven wasm/Typst plugin
boundary, and a 705-case oracle that pins core's behaviour independently of
how core is implemented.

**Recommended: hand-port `flint/core/` to Rust, corpus-driven.** Take the
stages in dependency order — `convert_temporal_data` → `resolve_channel_semantics`
→ `compute_channel_budgets` → `filter_overflow` → `compute_layout` — adding
each to `IMPLEMENTED_STAGES` only once `make conformance` is green for it. The
case for this over fixing up transpiler output:

- The target is ~4 100 lines of pure, side-effect-free, single-dependency
  logic. As idiomatic Rust with real enums and structs replacing 379 string
  lookups, it should land smaller than the Python.
- 705 golden cases mean the port is checkable at every step, not at the end.
- The result is maintainable and reviewable. Transpiler output is neither, and
  re-running the transpiler after any upstream change re-breaks every fixup.
- flint-py is missing ~1 700 LOC of core the JS side has (`pivot`,
  `static-series`, `band-dodge`, `aggregate`). Those need hand-porting from
  TypeScript no matter what, so a hand-port is partly unavoidable already.

The real risk is upstream drift, and it is the same risk under either route:
flint-py is at v0.1.0 and 180 fixtures behind the JS reference. Regenerating
the corpus (`make corpus`) after each upstream bump turns drift into a diff
you can read, which is the best available handle on it.

**If you want to keep a transpiler in the loop anyway**, the prerequisite is
upstream: give `flint/core/` real dataclasses instead of `dict[str, Any]`.
That is a ~4 100-line refactor of someone else's v0.1.0 package, it would need
to land upstream or be maintained as a fork, and depyler's silent table-drop
would still need fixing before the output could be trusted. `audit/type_audit.py
--min-score` exists as a ratchet if you go that way. I would not recommend it.

Either way `make corpus` and the plugin scaffold are the load-bearing pieces,
and both are done.
