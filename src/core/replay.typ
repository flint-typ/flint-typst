// Conformance entry point — ours, not upstream.
//
// `test/make_corpus.py` recorded, for 705 fixtures, exactly
// what each of the five pipeline stages received and returned in flint-py.
// This replays a recorded case through the port so `test/conformance.py` can
// diff the results.
//
// Because the corpus records inputs *and* outputs per stage, a stage can be
// verified without its upstream producer existing. Stages can therefore be
// ported in any order.

#import "py.typ": truthy
#import "resolve-semantics.typ" as resolve-semantics
#import "filter-overflow.typ" as filter-overflow
#import "compute-layout.typ" as compute-layout

/// Stages this build answers for real.
///
/// Kept in sync by hand. A stage listed here that is not implemented fails the
/// run loudly, which is the intended direction — a stage *not* listed is
/// skipped and counted separately, so a partial port can never report green.
#let IMPLEMENTED_STAGES = ("convert_temporal_data", "filter_overflow", "resolve_channel_semantics", "compute_channel_budgets", "compute_layout")

// Dispatch table. Entries appear as stages land.
#let STAGES = (
  convert_temporal_data: resolve-semantics.convert_temporal_data,
  filter_overflow: filter-overflow.filter_overflow,
  compute_channel_budgets: compute-layout.compute_channel_budgets,
  compute_layout: (..a) => {
    let p = a.pos()
    compute-layout.compute_layout(
      p.at(0), p.at(1), p.at(2), p.at(3),
      options: p.at(4, default: none), facet_grid: p.at(5, default: none),
    )
  },
  // Upstream's 4th parameter is positional-with-default; named in Typst.
  resolve_channel_semantics: (..a) => {
    let p = a.pos()
    resolve-semantics.resolve_channel_semantics(
      p.at(0), p.at(1), p.at(2), converted_data: p.at(3, default: none),
    )
  },
)

// Non-finite floats have no JSON literal; the corpus, the wasm plugin ABI and
// this all use the same tagged encoding. Keep in sync with `NONFINITE` in
// test/make_corpus.py.
#let encode(value) = {
  let t = type(value)
  if t == float {
    if value.is-nan() { return ("$f": "nan") }
    if value.is-infinite() { return ("$f": if value > 0 { "inf" } else { "-inf" }) }
    return value
  }
  if t == array { return value.map(encode) }
  if t == dictionary {
    let out = (:)
    for (k, v) in value.pairs() { out.insert(k, encode(v)) }
    return out
  }
  if t == function { return "<function>" }
  value
}


// Inverse of `encode`: recorded arguments arrive with non-finite floats tagged, and
// must be turned back into real floats before the ported stage sees them.
// Without this a tagged `inf` reaches the stage as a dictionary and every
// type test on it silently answers "not a number".
#let decode(value) = {
  let t = type(value)
  if t == array { return value.map(decode) }
  if t == dictionary {
    if value.len() == 1 and "$f" in value {
      let tag = value.at("$f")
      if tag == "nan" { return float.nan }
      if tag == "inf" { return float.inf }
      if tag == "-inf" { return -float.inf }
    }
    let out = (:)
    for (k, v) in value.pairs() { out.insert(k, decode(v)) }
    return out
  }
  value
}

/// Run every recorded call in `case` and return the results in the same shape.
#let run-case(case) = {
  let results = (:)
  for (stage, calls) in case.at("calls", default: (:)).pairs() {
    let fn = STAGES.at(stage, default: none)
    results.insert(stage, calls.map(call => {
      if fn == none {
        (__error__: "stage '" + stage + "' is not implemented in this build")
      } else {
        (ok: encode(fn(..decode(call.at("args", default: ())))))
      }
    }))
  }
  (
    slug: case.at("slug", default: none),
    implemented: IMPLEMENTED_STAGES,
    calls: results,
  )
}
