// Chart plan -> lilaq diagram.
//
// Ours; no upstream counterpart, hence kebab-case. This is the lilaq analogue
// of `flint/vegalite/instantiate_spec.py`: everything up to here is
// backend-agnostic (see `src/assemble.typ`), and this is where the plan becomes
// something you can look at.

#import "@preview/lilaq:0.6.0" as lq
#import "format.typ": tick-formatter
#import "../core/py.typ": falsy, is-finite, is-nan, is_number, num-str, py_str, truthy
#import "../core/compute-layout.typ": _js_to_date_number, _js_to_number
#import "@local/datehog:0.1.0" as dh

/// Pull one column out of the table.
#let column-of(data, field) = data.map(row => row.at(field, default: none))

/// Distinct values of a discrete column, in first-seen order, honouring the
/// canonical ordering core resolved (month names, weekdays, a custom sort).
#let category-values(data, cs) = {
  if falsy(cs) or falsy(cs.at("field", default: none)) { return () }
  let seen = column-of(data, cs.field).map(py_str).dedup()
  let canonical = cs.at("ordinalSortOrder", default: none)
  if falsy(canonical) { return seen }
  let ordered = canonical.map(py_str).filter(v => v in seen)
  ordered + seen.filter(v => v not in ordered)
}

/// Is this channel drawn as discrete bands?
#let is-discrete(cs) = {
  if falsy(cs) { return false }
  let t = cs.at("type", default: none)
  t == "nominal" or t == "ordinal"
}

/// Coerce a column to plot coordinates the way core does for layout, so the
/// numbers lilaq draws are the ones layout sized the chart for.
///
/// A **discrete** channel has no numeric value to coerce — its labels are
/// plotted at integer band positions, in the canonical order core resolved.
/// Coercing them instead yields NaN for every row, which silently drops the
/// whole series.
#let numeric-column(data, cs) = {
  if falsy(cs) or falsy(cs.at("field", default: none)) { return () }
  if is-discrete(cs) {
    let order = category-values(data, cs)
    return column-of(data, cs.field).map(v => {
      if v == none { return none }
      let i = order.position(o => o == py_str(v))
      if i == none { none } else { i * 1.0 }
    })
  }
  let temporal = cs.at("type", default: none) == "temporal"
  column-of(data, cs.field).map(v => {
    if v == none { return none }
    let n = if temporal { _js_to_date_number(v) } else { _js_to_number(v) }
    if is-nan(n) { none } else { n }
  })
}

/// Layout gives sizes in px; lilaq wants lengths. 1px at 96dpi.
#let px(n) = n * 1pt * 0.75

/// The axis label for a channel: the field name, or the aggregated name core
/// already rewrote (`Sales_sum`).
#let axis-label(cs) = {
  if falsy(cs) { return none }
  let f = cs.at("field", default: none)
  if falsy(f) { none } else { f }
}

  let t = cs.at("type", default: none)
  t == "nominal" or t == "ordinal"
}

// The subset of strftime that core's `level_to_format` can emit:
//   %Y  %b  %b %Y  %b %d  %b %d, %Y  %H:00  %H:%M  %H:%M:%S  and %b %d + time
#let _strftime(moment, pattern) = {
  if moment == none { return "" }
  let two(n) = if n < 10 { "0" + str(n) } else { str(n) }
  pattern
    .replace("%Y", str(moment.year))
    .replace("%b", dh.month-name(moment.month, abbreviated: true))
    .replace("%d", two(moment.day))
    .replace("%H", two(moment.hour))
    .replace("%M", two(moment.minute))
    .replace("%S", two(moment.second))
}

/// Format an epoch-millisecond tick using core's resolved temporal format.
#let temporal-formatter(cs) = {
  let pattern = cs.at("temporalFormat", default: none)
  if falsy(pattern) { return none }
  (ticks, ..args) => ticks.map(ms => {
    let m = dh.from-ms(ms)
    if m == none { [#ms] } else { [#_strftime(m, pattern)] }
  })
}

// Ours. The range the axis will actually span, which is *not* the data range:
// core's zero decision can pull one end to zero. Both the limits and the tick
// step have to agree on this or the step is computed for the wrong span.
#let effective-range(cs, values) = {
  let nums = values.filter(v => v != none and is_number(v) and is-finite(v))
  if nums.len() == 0 { return none }
  let lo = calc.min(..nums)
  let hi = calc.max(..nums)
  let zero = cs.at("zero", default: none)
  if truthy(zero) and truthy(zero.at("zero", default: none)) {
    if lo > 0 { lo = 0 }
    if hi < 0 { hi = 0 }
  }
  if lo == hi { return none }
  (lo, hi)
}

// Ours. A "nice" tick step for a target count — the 1 / 2 / 2.5 / 5 / 10
// progression every tick locator uses.
#let nice-step(span, target) = {
  if span <= 0 or target <= 0 { return none }
  let raw = span / target
  let mag = calc.pow(10, calc.floor(calc.log(raw)))
  let norm = raw / mag
  let mult = if norm <= 1 { 1 } else if norm <= 2 { 2 } else if norm <= 2.5 { 2.5 } else if norm <= 5 { 5 } else { 10 }
  mult * mag
}

// Ours. How many ticks an axis of this pixel length should carry.
//
// lilaq derives its own suggestion from the axis *length* alone
// (`length / 2em` for y, `axis.typ:610`), which assumes labels about as wide as
// a bare number. Ours are not — core's format decision produces `$16,000`
// rather than `1.6` — so at the sizes core asks for, that suggestion crowds the
// axis. One tick per ~40px is the same rule Vega applies, and lands in the
// 5-10 range that reads well.
#let tick-target(px-length) = calc.max(4, calc.min(9, int(px-length / 40)))

/// Build the `lq.axis` arguments core has an opinion about.
///
/// Three cases, in the order they take precedence:
///
/// - **discrete** — the marks are drawn at integer positions, so the axis gets
///   one tick per category and a formatter that looks the label back up.
///   Without this the axis would print the indices.
/// - **temporal** — coordinates are epoch milliseconds, so the axis gets the
///   date format core resolved (`temporalFormat`) rather than printing
///   `1.578 × 10¹²`.
/// - **quantitative** — core's number-format decision, via `zero`.
///
/// Anything core had no opinion about is left to lilaq.
#let axis-args(cs, data, px-length: 300) = {
  if falsy(cs) { return (:) }
  let args = (:)

  if is-discrete(cs) {
    let labels = category-values(data, cs)
    args.insert("ticks", range(labels.len()))
    args.insert("format-ticks", (ticks, ..a) => ticks.map(i => {
      let idx = int(i)
      if idx >= 0 and idx < labels.len() { [#labels.at(idx)] } else { [] }
    }))
    // There is nothing between January and February to subdivide. A discrete
    // axis has no positions other than the bands themselves, so subticks would
    // draw gridlines at values the data cannot take.
    args.insert("subticks", none)
    return args
  }

  if cs.at("type", default: none) == "temporal" {
    let fmt = temporal-formatter(cs)
    if fmt != none { args.insert("format-ticks", fmt) }
    // Same reasoning as a discrete axis when the ticks *are* the data points:
    // subdividing between two months implies a resolution the column lacks.
    args.insert("subticks", none)
    // Core chose the format for the granularity the *data* has — `%b` when
    // every point is a distinct month. Left to pick its own ticks lilaq
    // subdivides the range linearly and the labels repeat ("Jan Jan Feb Feb").
    // Where the points are few enough to be their own ticks, use them.
    let coords = numeric-column(data, cs).filter(v => v != none).dedup().sorted()
    if coords.len() > 0 and coords.len() <= 12 { args.insert("ticks", coords) }
    return args
  }

  // Core distinguishes an *axis* format from a *tooltip* one because Vega-Lite
  // renders both. A Typst chart has no tooltip, so where core only expressed a
  // tooltip opinion that is still its opinion about how the number reads, and
  // using it beats letting the axis fall back to `1.5 × 10⁴`.
  let fmt = tick-formatter(cs.at("format", default: none))
  if fmt == none {
    // A tooltip's precision is deliberately generous — currency tooltips are
    // always `,.2f` — which reads as noise on an axis whose ticks are already
    // round numbers. Keep the decision's currency/unit part, drop its digits.
    fmt = tick-formatter(cs.at("tooltipFormat", default: none), digits: auto)
  }
  if fmt != none { args.insert("format-ticks", fmt) }

  let tc = cs.at("tickConstraint", default: none)
  if truthy(tc) {
    // Core pinned the exact ticks (an integers-only axis over a short span).
    let exact = tc.at("exactTicks", default: none)
    if truthy(exact) {
      args.insert("ticks", exact)
      args.insert("subticks", none)
      return args
    }
    // An integers-only axis (Year, Count, Rank) cannot take a fractional
    // value, so a subtick between two ticks marks a position the data can
    // never occupy — unless the step is wide enough that the subdivisions
    // are themselves whole numbers.
    if truthy(tc.at("integersOnly", default: none)) {
      args.insert("subticks", none)
    }
  }

  // Otherwise steer the density. `tick-distance` takes precedence over the
  // length-derived suggestion, which the axis overwrites unconditionally, so a
  // `num-ticks-suggestion` passed here would be ignored.
  let range-of = effective-range(cs, numeric-column(data, cs))
  if range-of != none {
    let step = nice-step(range-of.at(1) - range-of.at(0), tick-target(px-length))
    if step != none {
      // An integers-only axis must not get a fractional step.
      let min-step = if truthy(tc) { tc.at("minStep", default: none) } else { none }
      if min-step != none { step = calc.max(step, min-step) }
      args.insert("tick-distance", step)
    }
  }
  args
}

/// Domain limits from core's zero decision and domain constraint.
///
/// Returns `auto` when core expressed no opinion, which is lilaq's own default
/// and better than a guess.
#let limits-for(cs, values, data) = {
  if falsy(cs) { return auto }
  if is-discrete(cs) {
    // The band count is the number of *categories*, not the number of rows —
    // `numeric-column` yields one coordinate per row, and several rows share a
    // band whenever the chart has series. Using the row count stretches the
    // axis past the data and leaves empty bands on the right.
    let n = category-values(data, cs).len()
    if n == 0 { return auto }
    return (-0.6, n - 0.4)
  }
  let dc = cs.at("domainConstraint", default: none)
  if truthy(dc) {
    let lo = dc.at("min", default: none)
    let hi = dc.at("max", default: none)
    if lo != none and hi != none { return (lo, hi) }
  }
  let zero = cs.at("zero", default: none)
  if truthy(zero) and truthy(zero.at("zero", default: none)) {
    let nums = values.filter(v => v != none and is_number(v) and is-finite(v))
    if nums.len() == 0 { return auto }
    // Core's decision is only about the *baseline*: anchor the side zero falls
    // on and leave the other `auto`, so lilaq still picks a round limit with
    // headroom. Pinning both ends to the data range clips the tallest bar.
    let lo = calc.min(..nums)
    let hi = calc.max(..nums)
    if lo >= 0 { return (0, auto) }
    if hi <= 0 { return (auto, 0) }
    return auto  // data spans zero; it is already included
  }
  auto
}

/// Map core's scaleType onto a lilaq scale name.
#let scale-for(cs) = {
  if falsy(cs) { return "linear" }
  let st = cs.at("scaleType", default: none)
  if st == "log" { "log" } else if st == "symlog" { "symlog" } else { "linear" }
}

/// Split the rows into one group per distinct series value.
///
/// Returns `((label, rows), ..)`; a single unlabelled group when the chart has
/// no series channel.
#let series-groups(plan) = {
  let cs = plan.channelSemantics
  let series-cs = {
    let c = cs.at("color", default: none)
    if truthy(c) and truthy(c.at("field", default: none)) { c } else {
      let g = cs.at("group", default: none)
      if truthy(g) and truthy(g.at("field", default: none)) { g } else { none }
    }
  }
  if series-cs == none { return ((none, plan.data),) }
  let field = series-cs.field
  category-values(plan.data, series-cs).map(label => (
    label,
    plan.data.filter(row => py_str(row.at(field, default: none)) == label),
  ))
}

/// Assemble the diagram from a plan and the marks a template produced.
#let diagram-for(plan, marks) = {
  let cs = plan.channelSemantics
  let layout = plan.layout
  let x-cs = cs.at("x", default: none)
  let y-cs = cs.at("y", default: none)

  // `numeric-column` already maps a discrete channel onto its band positions.
  let x-values = numeric-column(plan.data, x-cs)
  let y-values = numeric-column(plan.data, y-cs)

  lq.diagram(
    width: px(layout.subplotWidth),
    height: px(layout.subplotHeight),
    xlabel: axis-label(x-cs),
    ylabel: axis-label(y-cs),
    xscale: scale-for(x-cs),
    yscale: scale-for(y-cs),
    xlim: limits-for(x-cs, x-values, plan.data),
    ylim: limits-for(y-cs, y-values, plan.data),
    // `xaxis`/`yaxis` take an argument *dictionary*, which lilaq spreads into
    // its own `axis()` — passing a constructed `lq.axis` re-spreads the
    // element's own fields and it rejects them.
    xaxis: axis-args(x-cs, plan.data, px-length: layout.subplotWidth),
    yaxis: axis-args(y-cs, plan.data, px-length: layout.subplotHeight),
    // lilaq draws a legend for any labelled plot; `(:)` is its default and
    // means "show one if there is anything to show". Only suppress it when
    // there is a single unlabelled series.
    legend: if series-groups(plan).len() > 1 { (:) } else { none },
    ..marks,
  )
}
