# Pure Typst vs. Rust/wasm for flint core

Measured against Typst **0.15.1** on this machine. Re-runnable:
`make typst-probes` (capability probes) and the `bench-*.typ` files in
`test/typst-probes/`. Probes named `*-GAP.typ` are *expected to fail* — if one starts
passing, Typst gained a capability and this document is stale.

**Conclusion: pure Typst is the better default.** An earlier draft of this
document recommended wasm on the strength of three gaps and a performance
argument. Under scrutiny the gaps are one-time costs or code-style discipline,
and the performance argument does not survive contact with the real fixture
size distribution — core is linear in rows and, per point, cheaper than lilaq. What follows is the revised case, with the one genuine
correction that survived — `state()` will not do what it looks like it does.

---

## What Typst already covers

Nearly all of it. Mapping the 4 106-line core's actual usage:

| core needs | Typst 0.15.1 | notes |
|---|---|---|
| 61 `math.*` calls (floor, ceil, sqrt, exp, log, log2, log10) | ✅ `calc.*` | `calc.log(x, base: 2)` covers `log2` |
| `math.isnan`, `math.isfinite` | ✅ | `float.nan != float.nan` is `true`; `float.inf` exists |
| int vs float distinction | ✅ | `type(1)` ≠ `type(1.0)`, preserved through arithmetic |
| 11 `re.*` calls incl. `re.sub` with backrefs | ✅ | `str.replace(regex, m => m.captures.at(0) + …)` handles the `\1 \2` case |
| 379 `.get()` dict lookups | ✅ | `d.at(k, default: none)` maps 1:1 |
| 80 comprehensions, 5 lambdas, 17 closures | ✅ | `map`/`filter`/`fold`/`any`/`all`, first-class closures |
| `sorted(key=…)`, `.sort()` | ✅ | `array.sorted(key: fn)` |
| local mutation, `+=`, `while` | ✅ | `push`, `insert`, augmented assign all work |
| `json.loads` | ✅ | `json.decode`, and `json("f.json")` reads from disk |
| `copy.deepcopy` | ✅ free | values are copied by default |

The `dict[str, Any]` style that made the *transpilers* choke is a **good** fit
for Typst — a Typst dictionary is the same thing, and the 379 dynamic lookups
translate directly. Typst is a materially better target for this code shape
than Rust is, and the port is very likely *less* work than the Rust one: no
ownership to satisfy, no serialization boundary to design, no type-modelling
pass over 4 100 lines of untyped dict traffic.

## The three gaps, re-examined

### 1. No date parsing or epoch conversion — a one-time package

```
#let a = datetime("2020-03-14")   →  error: unexpected argument
#let t = d.timestamp()            →  error: type datetime has no method `timestamp`
```

Typst's `datetime` can only be *constructed* from `(year:, month:, day:)`. No
string parsing, no epoch conversion. Core needs both: `js_date.py` is 198 lines
of JavaScript `Date.parse` emulation, plus `fromisoformat`, `fromtimestamp`,
`.timestamp()` and `astimezone(utc)`.

This is a self-contained package, written once: an ISO/loose-date parser from
regex plus days-from-civil arithmetic. Call it 250–350 lines. Nothing about it
is coupled to flint, and it is independently useful.

Worth knowing: nothing on Typst Universe covers it today. `datify` and
`icu-datetime` are *formatting*, not parsing. So this is real work, just
bounded work — and it is the highest bug-density part of core, so budget test
cases for the JS quirks (V8 numeric-date fallback, two-digit year expansion,
timezone-less strings).

### 2. No try/catch — validate first, as normal Typst style

```
#repr(float("not-a-number"))   →  error: invalid float: not-a-number
```

There is no error recovery in Typst, and a bad value aborts the whole document
compile rather than one chart. Core has 14 `try`/`except` sites, mostly
`try: float(s) / except: fallback`.

Converting these to look-before-you-leap validation is standard Typst practice
and entirely mechanical — a `is-number(s)` regex guard ahead of each
conversion. The blast radius if you miss one is larger than in wasm (document,
not chart), but this is a property every Typst package lives with, and the
conformance corpus exercises all 705 fixtures' worth of real input against it.

### 3. In-place mutation — change the code style, **not** `state()`

flint core mutates dicts across call boundaries and callers depend on it:
`compute_layout` writes into the `channel_semantics` it was handed. Typst
dictionaries are values, so those writes land on a copy:

```
#let mutate(d) = { d.insert("added", 1); d }
#let outer = (a: 1)
#let _ = mutate(outer)
"added" in outer   →  false
```

**Threading the value through returns is the right fix** — mechanical,
reviewable, and the corpus catches mistakes (`make_core_corpus.py` deep-copies
arguments before each call precisely so recorded "inputs" are pre-mutation).

**`state()` is not an alternative, and this is worth knowing before you try
it.** It looks like a mutable cell and behaves like one in the simple case
(`30-state-as-cell.typ` passes), but it is an *introspection* mechanism keyed
to document position, not a variable:

```typst
// 32-state-needs-context-GAP.typ
#let compute() = { cell.get() * 2 }
  →  error: can only be used when context is known
```

```typst
// 33-state-no-readback.typ — inside one context block
let before = cell.get()      // 5
cell.update(v => v * 10)
let after  = cell.get()      // 5  ← the update is invisible
```

```typst
// 34-state-loop-lost.typ — accumulate across a loop
for i in range(5) { acc.update(d => { d.total += i; d }) }
acc.get()   →  (total: 0)    ← all five updates lost
```

A `context` sees one snapshot. Any function reading state becomes
context-dependent and yields content rather than a value, so it cannot
participate in arithmetic. Using `state()` for core's mutation patterns would
produce silently wrong numbers — the exact failure mode the code-style fix
avoids.

### Not a problem: call depth

Typst caps call depth at ~64 frames (64 succeeds, 100 fails). Core has **no
recursive functions** and a deepest static call chain of **9 frames**. Clear.
Worth remembering for the backend layer.

## Performance — both linear in points; core scales with *series*, lilaq does not

Full numbers and method in `bench/BENCH.md`.

Matched by shape (core-in-Typst estimated as CPython × 7):

| shape | lilaq | core Typst | core wasm | total Typst | total wasm | wasm saves |
|---|---:|---:|---:|---:|---:|---:|
| 32 pts, 1 series *(median fixture)* | 29 ms | ~2.8 ms | ~0.7 ms | ~32 ms | ~30 ms | **~7 %** |
| 500 pts, 1 series | 60 ms | ~28 ms | ~5 ms | ~88 ms | ~65 ms | ~26 % |
| 3 000 pts, 1 series | 243 ms | ~158 ms | ~30 ms | ~401 ms | ~273 ms | ~32 % |
| 4 000 pts, 20 series | 255 ms | ~554 ms | ~40 ms | ~809 ms | ~295 ms | ~64 % |
| 6 000 pts, 60 series | 381 ms | ~790 ms | ~60 ms | ~1 171 ms | ~441 ms | **~62 %** |

The shape of the two curves:

- **lilaq**: ≈ 27 ms fixed + 72 ms per 1 000 points, and *flat in series count*
  — 6 000 points costs the same whether it is 1 series or 60.
- **core**: linear in rows (`13.25 ms per 1 000 rows` fitted across all 705
  fixtures, residual σ 4.97 ms), but the per-row rate roughly triples with many
  series: 7.5 ms/1k rows single-series vs ~20 ms/1k at 20–60 series.

Per point, single series, core-in-Typst (~52 ms/1k) is actually **cheaper** than
lilaq (~72 ms/1k). Core only becomes the dominant term once series count is
high — which follows from where the time goes: `resolve_channel_semantics` is
49 % of core, and it is the stage doing per-channel/per-series work.

So the crossover is not a row-count threshold. It is **wide data**: many series
on one chart. In the fixture corpus that is the `line_area_stretch__*` group,
10 fixtures out of 705 (~1 %). For the 59 % of fixtures at ≤ 64 rows, moving
core to wasm saves about 2 ms out of ~32 — invisible.

Caveat on the estimates: the ×7 CPython→Typst factor comes from one proxy
workload, not a real Typst core. It locates the crossover; it is not precise.
The lilaq figures are its own plot rendering, so a flint→lilaq backend adds
further Typst-side work to both columns.

## What still argues for wasm

Honest residue, none of it decisive:

- **Wide data.** Charts with many series are where core overtakes lilaq and
  wasm roughly halves total render time (~62 % at 6 000 points / 60 series).
  Row count alone does not trigger this; series count does. If dashboards with
  dozens of series become a normal use case rather than the ~1 % they are in
  the fixture corpus, this is the argument that flips.
- **Error containment.** A bug in Typst-side core kills the user's document; in
  wasm it renders an error box and the document survives.
- **Portability.** A wasm core runs anywhere — a CLI, a web preview, another
  host. Typst-side core only runs in Typst. Only matters if that is ever wanted.
- **Ecosystem precedent.** `icu-datetime`, `lure` and `jogs` all ship wasm
  plugins on Typst Universe, so the route is well-trodden and distribution is
  a non-issue. But their existence also shows people reach for wasm only when
  pure Typst genuinely cannot do the job — which, for core, it can.

## What argues for pure Typst

- No build step, no Rust toolchain, no binary artifact — contributors need only Typst
- One language for core *and* the lilaq backend
- Likely **less** total work than the Rust port, given how directly the
  dict-heavy code maps
- Debuggable in place, no serialization boundary
- Fast enough for realistic chart sizes by a wide margin

## Recommendation

**Port core to Typst directly** — and note that this decision is cheap to
reverse, which is the strongest reason not to agonise over it.

Core is five pure data-in/data-out functions. That is not incidental: it is why
the tracer-based corpus worked at all, and it means the implementation sits
behind a narrow interface. Keeping that interface explicit on the Typst side —
one module exposing exactly those five entry points, with the lilaq backend
calling nothing else — means swapping in the wasm plugin later is a contained
change, not a rewrite. The scaffold is already built, validated and proven to
load, so the fallback stays live for free.

That also suggests a **hybrid** if the tail ever matters: dispatch on row count
and send only the large charts through wasm. Both implementations answer the
same interface and are held to the same corpus, so they are interchangeable per
call.

Order of work, so the risky parts land first while the corpus is the check:

1. **The date package**, standalone, with its own tests. It is the only piece
   with no Typst substrate under it, and everything temporal depends on it.
2. **The validation pass** — enumerate all 14 `try`/`except` sites and give
   each a guard before porting the function that contains it.
3. **Core, stage by stage**, in dependency order: `convert_temporal_data` →
   `resolve_channel_semantics` → `compute_channel_budgets` → `filter_overflow`
   → `compute_layout`. Thread values through returns; never reach for
   `state()`.

The corpus is route-agnostic — Typst reads the corpus JSON directly
(`probe/typst/25-fileio.typ` proves it), so a pure-Typst core is held to
exactly the same 705 cases. Building the Typst-side equivalent of
`check_conformance.py` is the first thing worth writing.
