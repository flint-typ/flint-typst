# flint-typst

A semantic chart compiler for Typst: give it a data table and a chart
declaration, and it works out what each channel *means* — semantic types,
scales, number formats, sort order — and how much room each part of the chart
needs. A backend then draws it.

It is a native Typst port of the python core of
[microsoft/flint-chart](https://github.com/microsoft/flint-chart), reproducing
its behaviour exactly on a 705-fixture corpus.

**Status: 0.2.2 — core complete and verified; one backend, partially covered.**
Five chart types draw; flint defines twenty-three. See [Roadmap](#roadmap).

| | |
|---|---|
| modules ported | 13 / 13 |
| functions verified against flint-py | 112 |
| pipeline stages green on the 705-case corpus | 5 / 5 (4230 / 4230) |
| differential cases | 19 874 / 19 889, with 15 registered divergences |
| chart types drawing | 5 (bar, grouped bar, line, area, scatter) |
| visual reference tests | 12, mirrored across backends |

```typst
#import "@local/flint-typst:0.2.2": chart

#chart(
  chart-type: "Bar Chart",
  data: (("Month", "Revenue"), ("Mar", 14200), ("Jan", 12000), ("Feb", 15500)),
  encodings: (x: "Month", y: "Revenue"),
  semantic-types: (Month: "Month", Revenue: (semanticType: "Amount", unit: "USD")),
)
```

That draws bars in calendar order (from the `Month` semantic type, not the row
order), anchored at zero (because a bar reads by length and `Amount`'s baseline
is meaningful), with `$2,500` tick labels (because `USD` makes it currency and
the values carry no decimals). None of those is configured; they are what the
compiler decided.

## What it does

```
data + encodings
   → convert_temporal_data        canonicalise every temporal column
   → resolve_channel_semantics    per channel: semantic type, vis type,
                                  format, scale, colour scheme, sort order
   → compute_channel_budgets      how many distinct values each channel can show
   → filter_overflow              drop what does not fit, with warnings
   → compute_layout               subplot size, band step, label angles, facets
```

Everything is plain Typst values in and out, so a backend is free to use as much
or as little as it wants.

## Backends

One so far: **[lilaq](https://typst.app/universe/package/lilaq)**, drawing bar,
grouped bar, line, area and scatter charts.

The core/backend split is the point of the design, though, so it is worth saying
what the boundary is. Core emits plain Typst values and never draws: it decides
*that* a column is a currency and how many decimals it warrants, and hands over a
d3 format spec, leaving the actual number formatting to the backend — the lilaq
one reads that decision into [`zero`](https://typst.app/universe/package/zero).

Filling the gaps the other way round is also part of a backend's job. Core was
written against Vega-Lite, which supplies a good deal of chart convention on its
own; lilaq is a plotting library rather than a grammar of graphics, so it has no
time scale, no automatic legend placement and no Vega-Lite default label angles.
Those decisions are made in `src/lilaq/`, and each one is written down in
[`PORT-DICTIONARY.md`](PORT-DICTIONARY.md) so it is clear what came from core and
what did not. Choices that are matters of taste rather than of correctness are
exposed as a `theme` argument instead of being settled there.

## Dependencies

`datehog` — date parsing and epoch arithmetic, written for this port but
standalone. It is headed for Typst Universe, after which it will import as
`@preview/datehog`.

Until then the import is `@local/datehog`, and it has to be installed by hand
from its [interim repository](https://github.com/zral0kh/datehog):

```sh
git clone https://github.com/zral0kh/datehog.git \
  ~/.local/share/typst/packages/local/datehog/0.1.0
```

## Testing

```sh
make test
```

Two harnesses, both value-based — `typst eval` in, JSON out, no images:

- **`tests/differential.py`** compares every ported function against flint-py
  call for call. Catches leaf functions the corpus never reaches.
- **`tests/conformance.py`** replays a corpus recorded from flint-py's real
  pipeline over 705 chart fixtures. The acceptance gate.

Both matter. `resolve_channel_semantics` passed the corpus 1410/1410 on its
first run while the differential found a genuine bug in date parsing.

Rendering is tested separately, with [tytanic](https://github.com/tingerrr/tytanic):

```sh
make visual          # compare against the reference images
make visual-update   # regenerate them after an intended change — then look
```

The split is deliberate. The value suites check that core's *decisions* match
flint-py; the visual ones check that the backend turns those decisions into the
right picture. The visual tests immediately caught four bugs the value suites
structurally cannot see: bars clipped at the frame, a grouped bar chart drawing
its series on top of one another, a line doubling back because the rows were
unsorted, and a category axis stretched to the row count instead of the
category count.

Running the tests needs the upstream Python package for comparison:

```sh
git clone --depth 1 https://github.com/microsoft/flint-chart.git flint-source
make corpus                  # regenerate the oracle (TZ=UTC is load-bearing)
```

## Roadmap

The core is done and verified. Everything below it is not.

**Chart types.** Five of flint's twenty-three draw. The remaining eighteen are
backend work, not core work — core already resolves them.

**Faceting.** Core computes a full facet grid (rows, columns, per-facet subplot
sizes) and the lilaq backend ignores all of it, drawing a single subplot. Charts
with `row`/`column` encodings therefore render, but not as small multiples.

**Log scales and automatic scale selection.** Core does neither. It emits a
`scaleType`, but only ever a linear or a discrete one, and nothing infers a log
scale from the data. The backend already maps whatever core says onto a lilaq
scale, so a core-side decision would carry through without backend changes.

**More backends.** Neither of these is started:

- **[primaviz](https://typst.app/universe/package/primaviz)** — broader
  chart-type coverage, and a good second consumer to prove the core boundary is
  genuinely backend-agnostic rather than lilaq-shaped
- **Vega-Lite via [ctxjs](https://typst.app/universe/package/ctxjs)** — a
  whole-pipeline alternative for HTML output, consuming core's d3 format strings
  directly

**Performance.** Two optimisations are identified and deliberately deferred,
because they would move the port away from flint-py's structure and so make
future upstream changes harder to apply: a single-pass variance in
`compute_banking_ar`, and a column-oriented restructuring of the row loops. See
[`bench/BENCH.md`](bench/BENCH.md).

**Publishing.** Both packages are headed for Typst Universe and import as
`@local` until they get there. `datehog` is close and goes first, since this
depends on it. This package is not ready: the items above are what "ready"
means, chart-type coverage most of all.

## Documentation

- [`PORT-DICTIONARY.md`](PORT-DICTIONARY.md) — every semantic divergence from
  flint-py, with reasons. Read this before applying an upstream change.
- [`docs/PORT-PLAN.md`](docs/PORT-PLAN.md) — layout, naming rules,
  the Python→Typst idiom table, backend notes
- [`docs/WHY-TYPST-NOT-WASM.md`](docs/WHY-TYPST-NOT-WASM.md) — why native Typst
  rather than a wasm plugin, with the capability probes
- [`docs/WHY-NOT-TRANSPILED.md`](docs/WHY-NOT-TRANSPILED.md) — why source-to-source
  transpilation was abandoned, with measurements
- [`docs/PORTING-NOTES.md`](docs/PORTING-NOTES.md) — where core's time
  goes, which Typst idioms are traps
- [`bench/BENCH.md`](bench/BENCH.md) — timings

## Licence

MIT. A port of `microsoft/flint-chart`, also MIT, © Microsoft Corporation — see
[`LICENSE`](LICENSE).
