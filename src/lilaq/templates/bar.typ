// Bar chart templates.
//
// Mirrors flint/vegalite/templates/bar.py. A bar chart's category axis is
// *banded*: `declareLayoutMode` says which axis that is, and core sizes the
// bands from it.

#import "@preview/lilaq:0.6.0" as lq
#import "../render.typ": category-values, column-of, numeric-column, series-groups
#import "../../core/encoding-actions.typ": make_sort_action
#import "../../core/py.typ": falsy, is-finite, is_number, py_str, truthy

// Ours. Upstream's `detect_banded_axis_from_semantics` lives in
// vegalite/templates/utils.py, which is Vega-Lite-specific glue; the decision
// it makes is not. A bar's category axis is the discrete one, preferring x.
#let banded-axis(cs, prefer: "x") = {
  let discrete(ch) = {
    let c = cs.at(ch, default: none)
    if falsy(c) or falsy(c.at("field", default: none)) { return false }
    let t = c.at("type", default: none)
    t == "nominal" or t == "ordinal"
  }
  let other = if prefer == "x" { "y" } else { "x" }
  if discrete(prefer) { prefer } else if discrete(other) { other } else { prefer }
}

#let _bar_declare(cs, table, chart_properties) = {
  (axisFlags: ((banded-axis(cs)): (banded: true)))
}

// Ours. Bars need one value per category, so rows are aggregated by the
// category column. Core already decided *how* (`aggregationDefault`); with no
// opinion the values are summed, matching Vega-Lite's default for a bar.
#let aggregate-by-category(rows, cat-cs, val-cs) = {
  let how = {
    let a = val-cs.at("aggregationDefault", default: none)
    if truthy(a) { a } else { "sum" }
  }
  let cats = ()
  let sums = ()
  let counts = ()
  let values = numeric-column(rows, val-cs)
  let labels = column-of(rows, cat-cs.field).map(py_str)
  for i in range(labels.len()) {
    let v = values.at(i, default: none)
    if v == none { continue }
    let j = cats.position(c => c == labels.at(i))
    if j == none {
      cats.push(labels.at(i))
      sums.push(v)
      counts.push(1)
    } else {
      sums.at(j) += v
      counts.at(j) += 1
    }
  }
  let out = if how == "average" {
    range(sums.len()).map(i => sums.at(i) / counts.at(i))
  } else {
    sums
  }
  (cats, out)
}

#let _bar_instantiate(plan) = {
  let cs = plan.channelSemantics
  let x-cs = cs.at("x", default: none)
  let y-cs = cs.at("y", default: none)
  if falsy(x-cs) or falsy(y-cs) { return () }

  let axis = banded-axis(cs)
  let horizontal = axis == "y"
  let cat-cs = if horizontal { y-cs } else { x-cs }
  let val-cs = if horizontal { x-cs } else { y-cs }
  if falsy(cat-cs.at("field", default: none)) { return () }

  let groups = series-groups(plan)
  let ordering = category-values(plan.data, cat-cs)

  // Dodge the series within each band rather than drawing them on top of one
  // another. Core already sized the band for this — `compute_layout` multiplies
  // the step by the group count and reports `xStepUnit: "group"` — so the band
  // is one data unit wide and each series takes an equal share of it, minus
  // the padding core resolved.
  let n = groups.len()
  let padding = plan.layout.at("stepPadding", default: 0.1)
  let usable = 1.0 - padding
  let bar-width = if n > 1 { usable / n } else { usable }

  groups.enumerate().map(entry => {
    let (i, group) = entry
    let (label, rows) = group
    let (cats, values) = aggregate-by-category(rows, cat-cs, val-cs)
    if cats.len() == 0 { return none }
    // Positions follow the canonical order core resolved, so a month column
    // reads Jan..Dec rather than in first-seen order.
    let xs = cats.map(c => ordering.position(o => o == c)).map(j => if j == none { 0 } else { j })
    let offset = if n > 1 { (i - (n - 1) / 2) * bar-width } else { 0.0 }
    let bar-fn = if horizontal { lq.hbar } else { lq.bar }
    bar-fn(
      xs, values,
      width: bar-width,
      offset: offset,
      label: if label == none { none } else { [#label] },
    )
  }).filter(m => m != none)
}

#let bar_chart_def = (
  chart: "Bar Chart",
  template: (mark: "bar", encoding: (:)),
  channels: ("x", "y", "color", "column", "row"),
  markCognitiveChannel: "length",
  declareLayoutMode: _bar_declare,
  instantiate: _bar_instantiate,
  encodingActions: (make_sort_action(),),
)

// ─── Grouped Bar Chart ──────────────────────────────────────────────────────

#let _grouped_bar_declare(cs, table, chart_properties) = {
  (axisFlags: ((banded-axis(cs)): (banded: true)))
}

#let grouped_bar_chart_def = (
  chart: "Grouped Bar Chart",
  template: (mark: "bar", encoding: (:)),
  channels: ("x", "y", "group", "column", "row"),
  markCognitiveChannel: "length",
  declareLayoutMode: _grouped_bar_declare,
  instantiate: _bar_instantiate,
  encodingActions: (make_sort_action(),),
)
