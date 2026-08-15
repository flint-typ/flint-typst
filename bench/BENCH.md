# Benchmarks behind ../docs/WHY-TYPST-NOT-WASM.md

> **Measured against the finished port, 2026-08-15.** Everything below the
> "Measured" section is the *pre-port estimate* that informed the decision,
> kept for comparison. The estimates held up: predicted ~2.9 / ~10.6 / ~153 ms
> at 32 / 180 / 3 000 points, measured 4 / 11 / 143 ms.

## Measured — the real port vs lilaq 0.6.0

`core-bench.typ` runs the full Phase-0/Phase-1 pipeline
(`convert_temporal_data` → `resolve_channel_semantics` →
`compute_channel_budgets` → `filter_overflow` → `compute_layout`) on N distinct
charts; `lilaq-bench.typ` renders the same shapes. Data varies per chart
so memoisation cannot collapse them. Baseline (empty document, ~180 ms)
subtracted.

| shape | lilaq 0.6.0 | core | core's share |
|---|---:|---:|---:|
| 32 pts, 1 series *(median fixture)* | 24 ms | **4 ms** | 14 % |
| 180 pts, 1 series | 32 ms | 11 ms | 25 % |
| 500 pts, 1 series | 54 ms | 26 ms | 32 % |
| 3 000 pts, 1 series | 205 ms | 143 ms | 41 % |
| 3 000 pts, 20 series | 187 ms | **254 ms** | 57 % |

**The Typst-native decision holds.** At the median chart size core is 14 % of
the cost — moving it to wasm would save about 3.5 ms out of 28. Core only
overtakes lilaq on wide data (many series), the ~1 % tail of the fixture corpus,
which is exactly where `../docs/WHY-TYPST-NOT-WASM.md` said the crossover would be.

Note lilaq 0.6.0 is faster than the 0.5.0 these estimates were built against
(24 ms vs 29 ms at 32 points; 205 vs 243 at 3 000), so core's share is slightly
*higher* than predicted while the absolute numbers are better.

Still unoptimised: the two `PORT-PERF` items deferred during the port — the
single-pass variance in `compute_banking_ar` (~4 % of core) and the
column-oriented restructuring (~28 % of core is redundant per-value coercion).
Both are now safe to attempt with the corpus green.

---

## Pre-port estimates (kept for comparison)

Typst 0.15.1, lilaq 0.5.0, this machine. Baseline (empty document, ~190–200 ms)
subtracted throughout. All repeat benchmarks vary their input per iteration:
Typst memoizes identical calls, which silently collapses a naive repeat loop to
a single evaluation.

> **Correction.** An earlier version of this file compared *bucket medians* of
> core cost against *single-series* lilaq benchmarks. The ">2000 rows" bucket
> mixed 3 000-row single-series fixtures with 6 000-row 60-series ones, which
> inflated core's apparent cost and produced a false "core scales worse than
> lilaq" conclusion. Core is linear in rows. The numbers below are matched by
> shape.

## How each side scales

**lilaq — linear in total points, flat in series count.**

`bench-lilaq.typ`, `bench-lilaq-series.typ`

| points | series | ms / chart |
|---:|---:|---:|
| 32 | 1 | 29 |
| 500 | 1 | 60 |
| 3 000 | 1 | 243 |
| 6 000 | 1 | 413 |
| 6 000 | 60 | 381 |
| 4 000 | 20 | 255 |

Fits ≈ **27 ms fixed + 72 ms per 1 000 points**. Splitting the same points
across 60 series costs nothing — slightly less, in fact.

**flint core — linear in rows, but ~2.5–3× more per row when there are many
series.**

Least-squares fit over all 705 fixtures (real five-stage pipeline timed in
CPython, vegalite excluded): `core_ms = -0.03 + 13.25 ms per 1 000 rows`,
residual σ 4.97 ms. The spread is explained by series count, not by any
superlinear term:

| rows | series | CPython ms | ms per 1 000 rows |
|---:|---:|---:|---:|
| 3 000 | 1 | 22.5 | **7.5** |
| 3 600 | 1 | 22.9 | 6.4 |
| 4 000 | 20 | 79.2 | **19.8** |
| 4 000 | 40 | 80.3 | 20.1 |
| 6 000 | 60 | 112.9 | 18.8 |

Per-stage share of total core time across all fixtures:

| stage | share |
|---|---:|
| `resolve_channel_semantics` | 49.2 % |
| `compute_layout` | 24.8 % |
| `compute_channel_budgets` | 16.2 % |
| `convert_temporal_data` | 8.9 % |
| `filter_overflow` | 0.9 % |

`resolve_channel_semantics` is half the cost and is the stage that does
per-channel/per-series work — which is why series count, not row count, drives
the outliers.

## Head to head, matched by shape

Core-in-Typst applies the ×7 CPython→Typst-script factor calibrated from an
identical loop run both ways (`bench-typst-heavy.typ`: 645 ms vs 93 ms in
CPython). That factor comes from one proxy workload, not a real Typst core, so
treat the estimates as locating a crossover rather than as precise figures.

| shape | lilaq | core Typst (est.) | core wasm | total Typst | total wasm | wasm saves |
|---|---:|---:|---:|---:|---:|---:|
| 32 pts, 1 series *(median fixture)* | 29 ms | ~2.8 ms | ~0.7 ms | ~32 ms | ~30 ms | **~7 %** |
| 500 pts, 1 series | 60 ms | ~28 ms | ~5 ms | ~88 ms | ~65 ms | ~26 % |
| 3 000 pts, 1 series | 243 ms | ~158 ms | ~30 ms | ~401 ms | ~273 ms | ~32 % |
| 4 000 pts, 20 series | 255 ms | ~554 ms | ~40 ms | ~809 ms | ~295 ms | ~64 % |
| 6 000 pts, 60 series | 381 ms | ~790 ms | ~60 ms | ~1 171 ms | ~441 ms | **~62 %** |

Marginal cost per 1 000 points, single series: lilaq ~72 ms, core-in-Typst
~52 ms. **Core is the cheaper of the two per point** until series count enters
the picture — at which point core's rate roughly triples while lilaq's does
not, and core becomes the dominant term.

## wasm boundary cost

`bench-wasm-small.typ`, `bench-wasm-boundary.typ` — JSON encode + plugin call +
decode with core stubbed, so this is pure boundary overhead. Rust-side core
work at these sizes is microseconds.

| rows | ms / chart |
|---:|---:|
| 32 | 0.7 |
| 3 000 | ~55 |

## Fixture size distribution

From the 705-case corpus — the weighting that makes the median column matter
more than the tail.

| rows | share |
|---|---:|
| ≤ 64 | 59 % |
| 65–500 | 33 % |
| 501–2 000 | 4 % |
| > 2 000 | 5 % |

Multi-series fixtures large enough for core to dominate are the
`line_area_stretch__*` group — 10 fixtures, ~1 %.
