// Line and Area chart templates.
//
// Mirrors the shape of flint/vegalite/templates/line.py — a template is a
// dictionary with `chart`, `template`, `channels`, `markCognitiveChannel`, an
// optional `declareLayoutMode`, `encodingActions`, and an `instantiate` that
// produces the marks. Only `instantiate` is backend-specific.

#import "@preview/lilaq:0.6.0" as lq
#import "../render.typ": numeric-column, series-groups
#import "../../core/py.typ": falsy, truthy

// Ours: pair up the x/y coordinates of one series, dropping rows where either
// side failed to coerce — lilaq needs two arrays of equal length with no holes.
//
// Sorted by x, because `lq.plot` connects points in array order and the row
// order is whatever the data happened to arrive in. Without this a line over
// unsorted rows doubles back on itself.
#let paired-xy(rows, x-cs, y-cs, sort: true) = {
  let xs = numeric-column(rows, x-cs)
  let ys = numeric-column(rows, y-cs)
  let pairs = ()
  for i in range(calc.min(xs.len(), ys.len())) {
    if xs.at(i) == none or ys.at(i) == none { continue }
    pairs.push((xs.at(i), ys.at(i)))
  }
  if sort { pairs = pairs.sorted(key: p => p.at(0)) }
  (pairs.map(p => p.at(0)), pairs.map(p => p.at(1)))
}

#let _line_instantiate(plan) = {
  let cs = plan.channelSemantics
  let x-cs = cs.at("x", default: none)
  let y-cs = cs.at("y", default: none)
  if falsy(x-cs) or falsy(y-cs) { return () }
  series-groups(plan).map(group => {
    let (label, rows) = group
    let (xs, ys) = paired-xy(rows, x-cs, y-cs)
    if xs.len() == 0 { return none }
    lq.plot(xs, ys, mark: none, label: if label == none { none } else { [#label] })
  }).filter(m => m != none)
}

#let line_chart_def = (
  chart: "Line Chart",
  template: (mark: "line", encoding: (:)),
  channels: ("x", "y", "color", "column", "row"),
  markCognitiveChannel: "position",
  instantiate: _line_instantiate,
)

// ─── Area Chart ─────────────────────────────────────────────────────────────

#let _area_instantiate(plan) = {
  let cs = plan.channelSemantics
  let x-cs = cs.at("x", default: none)
  let y-cs = cs.at("y", default: none)
  if falsy(x-cs) or falsy(y-cs) { return () }
  series-groups(plan).map(group => {
    let (label, rows) = group
    let (xs, ys) = paired-xy(rows, x-cs, y-cs)
    if xs.len() == 0 { return none }
    lq.fill-between(xs, ys, label: if label == none { none } else { [#label] })
  }).filter(m => m != none)
}

#let area_chart_def = (
  chart: "Area Chart",
  template: (mark: "area", encoding: (:)),
  channels: ("x", "y", "color", "column", "row"),
  // Area marks read by length from a baseline, not by position.
  markCognitiveChannel: "length",
  instantiate: _area_instantiate,
)

// ─── Scatter Plot ───────────────────────────────────────────────────────────

#let _scatter_instantiate(plan) = {
  let cs = plan.channelSemantics
  let x-cs = cs.at("x", default: none)
  let y-cs = cs.at("y", default: none)
  if falsy(x-cs) or falsy(y-cs) { return () }
  series-groups(plan).map(group => {
    let (label, rows) = group
    let (xs, ys) = paired-xy(rows, x-cs, y-cs)
    if xs.len() == 0 { return none }
    lq.scatter(xs, ys, label: if label == none { none } else { [#label] })
  }).filter(m => m != none)
}

#let scatter_plot_def = (
  chart: "Scatter Plot",
  template: (mark: "point", encoding: (:)),
  channels: ("x", "y", "color", "size", "column", "row"),
  markCognitiveChannel: "position",
  instantiate: _scatter_instantiate,
)
