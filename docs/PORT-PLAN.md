# Porting flint core to Typst — plan

Target: `flint/core/` (13 modules, 4 106 LOC, 119 top-level functions) as native
Typst, fast enough that a chart-heavy document does not feel like LaTeX, and
structured so that upstream changes remain mergeable by hand.

Three constraints, in priority order:

1. **Fidelity.** The Typst source should read like the Python source, so a
   future upstream diff can be applied by eye. Divergence is allowed but must
   be deliberate, marked, and registered.
2. **Speed.** Typst script is ~7× CPython. Core is 28 % redundant per-value
   coercion. Most of that is recoverable *without* restructuring — see §5.
3. **Traceable history.** One commit per upstream unit, so `git log` maps onto
   `flint/core/*.py` and re-porting later is a diff, not an archaeology project.

Background measurements: `bench/BENCH.md`, `PORTING-NOTES.md`,
`WHY-TYPST-NOT-WASM.md`.

---

## 1. Layout

```
src/
  core/
    types.typ                 ← flint/core/types.py
    js-round.typ              ← flint/core/__init__.py        (js_round)
    js-date.typ               ← flint/core/js_date.py
    type-registry.typ         ← flint/core/type_registry.py
    semantic-types.typ        ← flint/core/semantic_types.py
    color-decisions.typ       ← flint/core/color_decisions.py
    encoding-actions.typ      ← flint/core/encoding_actions.py
    encoding-overrides.typ    ← flint/core/encoding_overrides.py
    field-semantics.typ       ← flint/core/field_semantics.py
    decisions.typ             ← flint/core/decisions.py
    filter-overflow.typ       ← flint/core/filter_overflow.py
    compute-layout.typ        ← flint/core/compute_layout.py
    resolve-semantics.typ     ← flint/core/resolve_semantics.py
    replay.typ                ← conformance entry point (ours)
  date/                       ← standalone date package (see Phase 0)
test/
  conformance.py              ← drives corpus through Typst, reuses the differ
PORT-DICTIONARY.md            ← the divergence register
```

One `.typ` per upstream `.py`, same function order within each file. Filenames
are kebab-case, which also matches the original TypeScript (`resolve-semantics.ts`).

## 2. Naming — the fidelity lever

**Keep Python's `snake_case` for every upstream name.** Verified to work in
Typst. This breaks Typst convention (`kebab-case`), and that is the point: it
buys a direct grep between the two trees. `resolve_field_semantics` finds
`resolve_field_semantics`.

**Use `kebab-case` for anything we add** that has no upstream counterpart. The
naming convention then encodes provenance: if it has a hyphen, it is ours.

```typst
#let resolve_field_semantics(..) = { }   // upstream, mirrors flint-py
#let column-of(rows, key) = { }          // ours, no upstream counterpart
```

Module-level constants keep their upstream casing exactly (`TYPE_REGISTRY`,
`SemanticTypes`, `channelGroups`).

## 3. Per-function header

Every ported function carries its origin, so a later upstream diff can be
located without searching:

```typst
// flint/core/field_semantics.py:106 resolve_format
// PORT-EXC: `try: float(s)` -> is-numeric-string() guard (see PORT-DICTIONARY)
#let resolve_format(semantic_type, values, channel: none) = {
  ...
}
```

Line numbers drift; they are a hint, not a contract. The function name is the
key.

## 4. Divergence register

`PORT-DICTIONARY.md` is a table of every place the Typst differs semantically
from the Python, keyed by marker. Markers appear in the source as
`// PORT-XXX: <one line>`.

| marker | meaning | logged individually? |
|---|---|---|
| `PORT-MUT` | in-place mutation threaded through a return instead | **yes** |
| `PORT-EXC` | `try`/`except` replaced by a pre-validation guard | **yes** |
| `PORT-DATE` | Python `datetime` replaced by our date package | **yes** |
| `PORT-NUM` | numeric or number→string behaviour differs | **yes** |
| `PORT-PERF` | restructured for speed (§5, deferred class) | **yes** |
| `PORT-IDIOM` | pure syntax, no behaviour change | no — covered by §6 |

The register is the artefact that makes upstream tracking viable. A future
"flint-py bumped to 0.2.0" is then: diff upstream, for each changed function
check the register for that function, apply.

## 5. Speed — take the free wins, defer the expensive ones

The important finding from `PORTING-NOTES.md`: **the largest single win costs no
fidelity at all.** `js_date_parse_ms`, `_is_number`, `_js_to_number`,
`_js_to_date_number` are *already* top-level functions in flint-py, and they are
called with 67–152× redundancy. Ported as top-level Typst functions, Typst's
automatic memoization collapses that — measured 3.2× on the parse-heavy path,
which is ~14 % of core. Nothing to restructure; just do not inline them.

**Take immediately (zero or near-zero fidelity cost):**

| rule | why | fidelity cost |
|---|---|---|
| `arr.push(x)`, never `arr = arr + (x,)` | `+` is O(n²): 20 k elements = 1 693 ms vs 6 ms | none — mirrors `.append()` |
| keep upstream's small helpers as top-level functions | Typst memoizes them; 67–152× redundancy | none — they are already functions |
| do not wrap *new* trivial checks in functions | call + cache costs more than a type test | none |
| single-pass mean/variance in `compute_banking_ar` | walks the data twice upstream; ~4 % of core | small, local, `PORT-PERF` |

**Defer behind measurement (real fidelity cost):**

Column-oriented restructuring — extracting each field once and passing an
analysed column instead of re-walking rows — is the change that would take the
remaining coercion cost to near zero. It also restructures
`resolve_channel_semantics` and `field_semantics` enough to make upstream diffs
hard to apply.

So: **port those two faithfully first, get them green, measure, and only then
decide.** If it is needed it becomes its own commit series, every function
marked `PORT-PERF`, with the pre-restructure version reachable in history. Do
not do it speculatively.

## 6. Idiom table (verified in `probe/typst/idiom/`)

| Python | Typst |
|---|---|
| `d.get(k)` / `d.get(k, dflt)` | `d.at(k, default: none)` / `d.at(k, default: dflt)` |
| `k in d` | `k in d` |
| `d[k] = v` | `d.insert(k, v)` |
| `{**enc, "type": t}` | `(..enc, type: t)` |
| `{**a, **b}` (only spreads) | `(: ..a, ..b)` — the leading `:` is **required**, or it parses as an array |
| `lst.append(x)` | `lst.push(x)` |
| `[f(x) for x in xs if p(x)]` | `xs.filter(p).map(f)` |
| `sorted(xs, key=f)` | `xs.sorted(key: f)` |
| `a, b = f()` | `let (a, b) = f()` |
| `def f(a, b=1, *, c=2)` | `let f(a, b: 1, c: 2)` — **note: Python positional defaults become named** |
| `return x` early | `return x` |
| `isinstance(v, (int, float))` | `type(v) == int or type(v) == float` |
| `isinstance(v, bool)` | `type(v) == bool` — check **before** numeric, as upstream does |
| `math.floor/ceil/sqrt/exp/log` | `calc.floor/ceil/sqrt/exp/ln`; `log2` → `calc.log(x, base: 2)` |
| `float('inf')`, `float('nan')` | `float.inf`, `float.nan` — **never `1.0/0.0`**, that errors |
| `math.isnan(v)` | `py.typ`'s `is-nan(v)` — wraps `float.is-nan()`, which exists only on `float` (an `int` has no such method) |
| `math.isfinite(v)` | `py.typ`'s `is-finite(v)` — wraps `float.is-infinite()`, with the same type guard |
| any `<`/`>` on data-derived numbers | **Typst errors comparing NaN**; Python returns False. Guard with `is-nan` first |
| `re.compile(p)` | `regex(p)` at module level (memoized) |
| `re.sub(p, r"\1 \2", s)` | `s.replace(regex(p), m => m.captures.at(0) + " " + m.captures.at(1))` |
| `copy.deepcopy(x)` | `x` — values are copied already |
| nested `def` closing over a local | `let f(..) = ..` inside the block; works, capture is by value |

The one that changes call sites: Python's positional-with-default parameters
become named parameters in Typst. Every such call site is `PORT-IDIOM`, not
logged individually, but keep the parameter *names* identical to upstream.

## 7. Known-risk spikes — do these before the bulk work

**`js_number_to_string`.** flint mirrors JavaScript's `String(n)`. Typst's
`str()` is closer to JS than Python's `repr()` is, but diverges at the exponent
boundaries:

| value | Typst `str()` | Python `repr()` | JS `String()` |
|---|---|---|---|
| `100.0` | `"100"` | `'100.0'` | `'100'` ✔ Typst matches |
| `1e21` | `"1000000000000000000000"` | `'1e+21'` | `'1e+21'` ✘ |
| `1e-7` | `"0.0000001"` | `'1e-07'` | `'1e-7'` ✘ |

This affects every string-formatted output and therefore every conformance
comparison. Write `js_number_to_string` and its tests first, mark `PORT-NUM`.

**`js_round`.** Upstream is `math.floor(x + 0.5)`, matching JS. `calc.floor(x + 0.5)`
matches; do **not** use `calc.round`, which rounds half away from zero
(`calc.round(-2.5)` ≠ `Math.round(-2.5)`).

## 8. Test harness — build this first, before any core code

The transport is `typst eval`, which returns JSON on stdout with no PDF:

```bash
typst eval '{import "src/core/replay.typ": run_case; run_case(json(sys.inputs.case))}' \
  --format json --input case=case.json
```

Verified working. `src/core/replay.typ` mirrors `rust/src/bin/replay.rs`: read a
recorded case, dispatch each recorded call to the matching stage, return results
in the same shape.

`test/conformance.py` decompresses a corpus case, shells out to `typst eval`,
and diffs against the recorded result — reusing `validate/check_conformance.py`'s
differ verbatim, including its two deliberate strictnesses (`bool` compared
before `int`; int-vs-float a failure, not a rounding artefact) and the
`{"$f":"inf"}` non-finite encoding.

Keep `IMPLEMENTED_STAGES` in `replay.typ`: unimplemented stages are *skipped and
counted*, never silently passed. The harness must be able to say "nothing was
actually verified".

**Why this comes first:** the corpus records inputs *and* outputs per stage, so
stages can be verified in any order — a stage does not need its upstream
producer to exist. That decouples the port from the pipeline order entirely.

## 9. Phases

Module dependency order is forced by Typst's define-before-use; the conformance
gates are not, thanks to §8.

| # | phase | modules | LOC | gate |
|---|---|---|---|---|
| 0 | **date package** | ✅ `datehog` (sibling package) | 664 | ✅ done — see below |
| 0b | **harness** | `replay.typ`, `test/conformance.py` | ~150 | runs, reports all-skipped |
| 1 | **leaves** | `types`, `js-round`, `type-registry`, `encoding-overrides`, `encoding-actions`, `color-decisions` | 402 | — |
| 2 | **semantic types** | `semantic-types` | 549 | — |
| 3 | **js-date** | `js-date` | 198 | spot-test vs Python |
| 4 | **field semantics** | `field-semantics` | 596 | — |
| 5 | **temporal** | `resolve-semantics` (`convert_temporal_data` + helpers) | ~200 | ✅ `convert_temporal_data` green (705 cases) |
| 6 | **overflow** | `filter-overflow` | 210 | ✅ `filter_overflow` green |
| 7 | **decisions** | `decisions` | 457 | — |
| 8 | **semantics** | `resolve-semantics` (remainder) | ~250 | ✅ `resolve_channel_semantics` green (1 410 calls) |
| 9 | **layout** | `compute-layout` | 1 241 | ✅ `compute_channel_budgets` + `compute_layout` green |
| 10 | **measure** | — | — | benchmark vs `BENCH.md`; decide on §5 deferred work |

**Phase 0 is complete.** `datehog` lives beside this package and covers
everything `js_date.py`, `datetime.fromisoformat`, `fromtimestamp`,
`.timestamp()` and `astimezone()` were doing. Status: unit tests pass,
29 031/29 031 calendar dates match Python's `datetime` over 1600-2400, and
**445/445 parser cases match flint-py** (every date-shaped string in the
fixture corpus plus hand-picked edges). `parse-ms` is memoised, giving 3.1x at
the corpus's real redundancy ratio.

Three findings from building it that the rest of the port inherits:

- **`PORT-DATE`: zoneless input is UTC, not local.** ECMAScript reads a
  zoneless date-time in the host zone; Typst does not expose the local UTC
  offset (`datetime.today()` gives a date but no time, and 14 offsets reproduce
  it), so this is not merely a choice. `assume-offset` is available where the
  zone is known. This is also the better default: local-time parsing makes a
  document render differently depending on the machine that built it.
- **The corpus had to be pinned to `TZ=UTC`** for the same reason — 15 of the
  705 fixtures (`dates_hours__*`, `dates_year_month__*_mon_yyyy_*`) recorded
  different values depending on the build machine's timezone. `make corpus`
  now sets it; the corpus has been regenerated.
- **Typst's regex engine has no backreferences.** `_V8_NUMERIC_DATE` uses `\2`
  to require matching separators; datehog captures both and compares them in
  code. Expect the same wherever upstream leans on a backreference.

Phase 5 is the first real gate and the first honest signal. If
`convert_temporal_data` goes green on all 705 cases, the approach is validated
end to end — harness, date package, idiom rules and all.

## 10. Commit discipline

- One commit per upstream function, or per tight group of small helpers.
- Message: `port(field_semantics): resolve_format` — the upstream module and
  function, so `git log --grep` finds it.
- A commit that introduces a divergence updates `PORT-DICTIONARY.md` in the same
  commit. No exceptions; the register is worthless if it lags.
- A commit that closes a gate says so and includes the conformance output.
- Perf restructuring never rides along with a port commit. Port faithfully,
  green, then optimise as a separate marked commit.

## 10b. Backend notes (decided, not yet built)

**Core does not format numbers, and should not start.** `resolve_format`
returns d3-format *specifier strings* (`",.2f"`, `".0~%"`, `",d"`) plus a
prefix/suffix — no rendering happens in core. What it actually computes is a
decision: `_detect_precision` reads the data and says how many decimals it
warrants; the rest picks currency vs percent vs unit-suffix.

That decision is backend-agnostic and belongs in core. The d3 *string* is a
Vega-Lite artifact.

- **lilaq backend:** lilaq already depends on `zero` (`src/logic/tick-format.typ`
  imports `@preview/zero:0.5.0`) and its axis takes a `format-ticks` function.
  The adapter should read core's **structured** decision — precision, prefix,
  suffix — and build a `zero.num` closure. It must *not* parse the d3 pattern
  back into digits: that would re-derive what `_detect_precision` already
  returned. `_precision_format` (the d3 string builder) is vestigial on this
  path, kept for conformance.
### Backend targets

Three, in intended order:

| backend | package | notes |
|---|---|---|
| **lilaq** | `@preview/lilaq` (0.6.0) | the primary target; scientific plotting, already depends on `zero` for number formatting |
| **primaviz** | `@preview/primaviz` (0.9.1) | 50+ chart types, 7 themes, zero dependencies — broader chart-type coverage than lilaq, so a good second consumer to prove the core boundary is genuinely backend-agnostic |
| **Vega-Lite via JS** | `@preview/nulite` on `@preview/ctxjs` (0.5.0) | see the constraint below |

### Why the Typst port is still required

`ctxjs` runs JavaScript **inside a `context` block**, and that is a one-way
door: a context block yields *content*, and values computed inside it cannot
escape to the top level. It is the same constraint that rules out `state()` as
a mutable cell (see WHY-TYPST-NOT-WASM.md §3).

So a JS pipeline can *render* a chart — it draws inside its own context — but it
cannot hand computed layout back out to drive a top-level `lq.diagram(..)` call.
Any Typst-native backend therefore needs the semantics and layout decisions
available as ordinary Typst values, which is exactly what this port provides.

That makes the JS route an **alternative whole-pipeline backend** rather than a
component of the others: useful for Vega-Lite fidelity or HTML export
(`target() == "html"`), sharing core's d3 format strings directly, and requiring
no layout from us at all.

- Consequence for layout: a JS backend does its own layout, so `compute_layout`
  output is lilaq/primaviz-facing and a Vega-Lite backend would ignore most of
  it. Worth remembering before optimising layout for a single consumer.
- **Version drift to watch:** measurements in `bench/BENCH.md` were taken
  against lilaq 0.5.0 and zero 0.5.0; both have since moved (0.6.0 / 0.7.0).
  Re-run the benchmarks before trusting the numbers for a backend decision.

## 10c. Deferred infrastructure (agreed, not yet built)

- **tytanic for render tests.** The suites here are deliberately value-based —
  `typst eval` in, JSON out, no images — which is why they can run 705 cases in
  seconds and diff precisely. But once a lilaq backend exists, *rendering* needs
  its own tests, and those belong in [tytanic](https://github.com/tingerrr/tytanic)
  with reference images. Value tests for core, image tests for the backend; the
  two should stay separate suites.
- **A top-level Makefile** governing test and deploy across both packages
  (`flint-typst`, `datehog`) — today each has its own `test/run.sh` and the
  transpile tooling has its own `Makefile`. Fold them into one entry point,
  including the `@local` → `@preview` package-publish step.

## 11. Upstream drift

flint-py is v0.1.0 and 180 of 780 fixture tests behind the JS reference. Assume
it will move.

- Pin the `flint-source` commit in the repo; record it in `PORT-DICTIONARY.md`.
- **Port upstream bugs faithfully.** The corpus is generated *from flint-py*, so
  "fixing" something upstream got wrong makes the gate fail. If a genuine bug is
  spotted, record it as an observation and leave the behaviour alone — that is a
  separate conversation with upstream.
- On an upstream bump: re-run `make corpus`, diff the corpus, and the fixtures
  that changed tell you which stages need attention. That turns drift into a
  reviewable diff instead of a silent divergence.

## 12. Definition of done, per commit

1. Function ported, header comment with upstream path and name.
2. Any divergence marked in-source and registered in `PORT-DICTIONARY.md`.
3. No `arr + (x,)` accumulation anywhere in the diff.
4. `make conformance-typst` no worse than before; if the commit closes a gate,
   that stage is green across all 705 cases.
5. Commit touches one upstream unit.
