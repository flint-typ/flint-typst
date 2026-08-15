# Making the Typst core port fast

flint-py's core is a line-by-line port of TypeScript. Ported line-by-line again
into Typst it would work, but it would inherit a shape that is wasteful in
Python and wasteful in Typst for the same reason: **the same values get
re-tested and re-parsed hundreds of times.**

Everything below is measured. Probes are `probe/typst/perf-*.typ`; the profile
numbers come from running the real pipeline over the 41 fixtures with ≥1 000
rows.

---

## Where core's time actually goes

Profiled over fixtures ≥1 000 rows (3 594 ms total inside `flint/core`):

| function | ms | calls | distinct args | redundancy |
|---|---:|---:|---:|---:|
| `js_date_parse_ms` | 341 | 180 030 | 1 181 | **152×** |
| `_is_number` | 246 | 811 600 | 7 357 | **110×** |
| `_js_to_date_number` | 147 | 171 120 | 1 179 | **145×** |
| `_is_nan` | 139 | 460 640 | — | — |
| `_js_to_number` | 114 | 289 520 | 4 324 | **67×** |

**Per-value coercion is 28 % of core time, and 67–152× of it is redundant.**
That is the optimization target. `resolve_channel_semantics` — 49 % of core, and
the stage whose cost scales with series count — is mostly this.

---

## 1. Never accumulate with `+`. This one is not a micro-optimization.

`probe/typst/perf-push.typ` vs `perf-concat.typ`, time to build an array:

| elements | `a.push(x)` | `a = a + (x,)` |
|---:|---:|---:|
| 5 000 | 2 ms | 119 ms |
| 10 000 | 5 ms | 432 ms |
| 20 000 | 6 ms | **1 693 ms** |

`push` is amortized O(1). `+` copies, so accumulating in a loop is O(n²).
`dict.insert` is linear and fast (16 ms for 20 000).

A port that writes `result = result + (row,)` anywhere in a per-row loop will be
hundreds of times slower than one that writes `result.push(row)`, and it will
look fine at fixture sizes and fall over on real data. Grep for it before
anything else.

## 2. Let Typst's memoization do the caching — at the right granularity

Typst automatically memoizes pure function calls, including on large array
arguments, and the cache lookup is effectively free.

**Expensive function, high redundancy — big win.** `perf-memo-parse.typ`, a
regex date parser over 100 000 calls, at core's real redundancy ratio:

| distinct inputs | time |
|---:|---:|
| 1 181 (real ratio) | **157 ms** |
| 100 000 (all distinct) | 502 ms |

**3.2× for free**, just from making `parse-date` a top-level pure function
rather than inlining its body. That directly addresses `js_date_parse_ms` and
`_js_to_date_number` — together 488 ms, ~14 % of core.

**Whole-column analysis — even bigger.** `perf-memo.typ`, a distinct-count +
max over a 3 000-element column:

| repeats, same column | time |
|---:|---:|
| 1 | 6 ms |
| 50 | **5 ms** |

Flat. Fifty calls cost the same as one. Compare `perf-memo-distinct.typ`, same
work on different columns each time: 20 repeats → 141 ms, linear.

**Trivial function, scalar argument — memoizing loses.** `perf-memo-scalar.typ`,
200 000 calls of a `type(v) == float` check:

| | few distinct args | all distinct |
|---|---:|---:|
| inline | **576 ms** | 495 ms |
| as a function | 652 ms | 732 ms |

The call plus cache lookup costs more than the check saves. So:

> Wrap it in a function when the work is a regex, a parse, or a pass over a
> column. Inline it when it is a type test or a comparison.

## 3. Go column-oriented

Core is row-oriented: it repeatedly walks `rows` pulling one field at a time.
That is where the 110× redundancy on `_is_number` comes from — the same column
is re-scanned per channel, per stage, and per pass.

Extract each field **once**:

```typst
#let column(rows, key) = rows.map(r => r.at(key, default: none))

// Pure in (column) -> memoized across every channel and stage that asks.
#let analyse-field(col) = (
  kind: ...,        // one pass classifying the values
  numeric: ...,     // pre-coerced numbers
  parsed: ...,      // pre-parsed dates
  distinct: ...,
  min: ..., max: ...,
)
```

Two things fall out. Downstream code reads `analyse-field(col).numeric` instead
of calling `_is_number` per cell, and because `analyse-field` is pure in the
column, the second, third and fourth caller get it free — including the
**second `resolve_channel_semantics` pass**, which the pipeline runs on every
chart (2.0 calls per fixture; `assemble` resolves preliminarily, enriches the
encodings with the inferred types, then re-resolves).

This is the change that turns 28 % of core into roughly nothing, and it is
worth designing in from the first stage rather than retrofitting.

## 4. Single-pass statistics

`compute_banking_ar` is the largest single core function by self-time. It walks
the data twice — once for the means, once for the variances:

```python
sum_x = sum(...); mean_x = sum_x / n      # pass 1
for i in range(n): var_x += dx * dx       # pass 2
```

One pass accumulating `n`, `Σx`, `Σx²` gives the same standard deviations. Free
2× on ~8 % of core. Watch the numerics — the naive `Σx² − (Σx)²/n` form loses
precision on large offset values, so subtract the domain minimum first (which
this code already does) or use Welford.

## 5. Watch `str(v)` for distinct counting

Typst dictionary keys must be strings, so the natural distinct-count idiom
stringifies every value. That cost is real at 3 000+ rows and it is paid inside
the hottest stage. Where the column is already known to be numeric,
`array.dedup()` avoids the stringification — worth measuring against your
actual data rather than assuming either way.

---

## What this is worth

Rough, and stated as such — these compose, but not independently, and none of it
is measured on a real Typst core because none exists yet:

| change | effect |
|---|---|
| coercion via memoized pure functions + column extraction | most of the 28 % coercion cost |
| single-pass banking AR | ~4 % |
| avoiding `+` accumulation | not a saving — a landmine |

A carefully written port plausibly lands **1.5–2.5× faster** than a naive
line-by-line one. Against the numbers in `probe/typst/BENCH.md`, that moves the
worst case (6 000 points, 60 series) from ~790 ms to roughly 320–530 ms —
comparable to lilaq's own 381 ms rather than double it, which would remove the
only shape where core dominates.

Treat that as motivation to write it this way from the start, not as a promise.
The way to find out is to port `convert_temporal_data` and
`resolve_channel_semantics` first — between them the temporal parsing and the
per-series work, i.e. both hot spots — and measure before committing to the
rest.

## Do not micro-optimize ahead of the corpus

Every one of these changes alters the shape of the code relative to flint-py,
which makes divergence easier. Port a stage plainly, get `check_conformance`
green for it, *then* restructure with the corpus as the guard. The corpus is
what makes optimization safe; without it these are all opportunities to
introduce a subtly different chart.
