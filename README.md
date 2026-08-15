# flint-typst

A semantic chart compiler for Typst: give it a data table and a chart
declaration, and it works out what each channel *means* — semantic types,
scales, number formats, sort order — and how much room each part of the chart
needs. A backend then draws it.

It is a native Typst port of the core of
[microsoft/flint-chart](https://github.com/microsoft/flint-chart), reproducing
its behaviour exactly on a 705-fixture corpus.

**Status: 0.1.0 — core complete, no backend yet.** The compiler works and is
verified; nothing draws a chart with it so far.

| | |
|---|---|
| modules ported | 13 / 13 |
| functions verified against flint-py | 112 |
| pipeline stages green on the 705-case corpus | 5 / 5 (4230 / 4230) |
| differential cases | 19 874 / 19 889, with 15 registered divergences |

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

None yet. Intended, in order:

- **[lilaq](https://typst.app/universe/package/lilaq)** — the primary target
- **[primaviz](https://typst.app/universe/package/primaviz)** — broader chart-type
  coverage, and a good second consumer to prove the core boundary is genuinely
  backend-agnostic
- **Vega-Lite via [ctxjs](https://typst.app/universe/package/ctxjs)** — a
  whole-pipeline alternative for HTML output, consuming core's d3 format strings
  directly

Core deliberately does **not** format numbers: it decides *how many decimals the
data warrants* and hands over a format spec. A lilaq backend should read that
structured decision into [`zero`](https://typst.app/universe/package/zero).

## Dependencies

[`datehog`](../datehog) — date parsing and epoch arithmetic, written for this
port but standalone. During development it is linked into Typst's local
namespace:

```sh
ln -s "$PWD/../datehog" ~/.local/share/typst/packages/local/datehog/0.1.0
```

## Testing

```sh
PY=transpile/.venv/bin/python ./test/run.sh
```

Two harnesses, both value-based — `typst eval` in, JSON out, no images:

- **`test/differential.py`** compares every ported function against flint-py
  call for call. Catches leaf functions the corpus never reaches.
- **`test/conformance.py`** replays a corpus recorded from flint-py's real
  pipeline over 705 chart fixtures. The acceptance gate.

Both matter. `resolve_channel_semantics` passed the corpus 1410/1410 on its
first run while the differential found a genuine bug in date parsing.

Rendering tests belong in [tytanic](https://github.com/tingerrr/tytanic) once a
backend exists; these suites deliberately do not touch images.

Running the tests needs the upstream Python package for comparison:

```sh
git clone --depth 1 https://github.com/microsoft/flint-chart.git flint-source
make -C transpile corpus     # regenerate the oracle (TZ=UTC is load-bearing)
```

## Documentation

- [`PORT-DICTIONARY.md`](PORT-DICTIONARY.md) — every semantic divergence from
  flint-py, with reasons. Read this before applying an upstream change.
- [`transpile/PORT-PLAN.md`](transpile/PORT-PLAN.md) — layout, naming rules,
  the Python→Typst idiom table, backend notes
- [`transpile/TYPST-VS-WASM.md`](transpile/TYPST-VS-WASM.md) — why native Typst
  rather than a wasm plugin, with the capability probes
- [`transpile/REPORT.md`](transpile/REPORT.md) — why source-to-source
  transpilation was abandoned, with measurements
- [`transpile/PORTING-NOTES.md`](transpile/PORTING-NOTES.md) — where core's time
  goes, which Typst idioms are traps
- [`transpile/probe/typst/BENCH.md`](transpile/probe/typst/BENCH.md) — timings

## Licence

MIT. A port of `microsoft/flint-chart`, also MIT, © Microsoft Corporation — see
[`LICENSE`](LICENSE).
