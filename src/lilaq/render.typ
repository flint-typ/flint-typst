// Chart plan -> lilaq diagram.
//
// Ours; no upstream counterpart, hence kebab-case. This is the lilaq analogue
// of `flint/vegalite/instantiate_spec.py`: everything up to here is
// backend-agnostic (see `src/assemble.typ`), and this is where the plan becomes
// something you can look at.

#import "@preview/lilaq:0.6.0" as lq
#import "format.typ": tick-formatter
#import "../core/py.typ": falsy, is-finite, is-nan, is_number, num-str, py_str, truthy
#import "../core/compute-layout.typ": APPROX_CHAR_WIDTH_RATIO, _js_to_date_number, _js_to_number
#import "@local/datehog:0.1.0" as dh
#import "time-ticks.typ": calendar-ticks
#import "../core/resolve-semantics.typ": level_to_format

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
// An x-axis tick label rotated to `angle` degrees and anchored by its *end*
// rather than its middle.
//
// lilaq places a bottom-axis label with `place(top + center, label)`
// (model/axis.typ), so it centres whatever box it is handed on the tick. A
// rotated label's bounding box is much wider than the label is tall, and
// centring that box leaves the text visibly beside its own bar — at 20
// categories the last label lands well past the last bar.
//
// `rotate(origin: ...)` alone cannot fix it: the origin controls where the ink
// goes inside the box, and lilaq then centres that box anyway. So the box is
// made zero-width, which leaves lilaq's centring with nothing to centre and
// pins x = the tick, and the label is `place`d against that point with
// `reflow: false` so it hangs off it. The height is measured and kept honest,
// or the axis would reserve no room and the labels would run into the axis
// title.
//
// The ink extends left and down from the anchor: rotating by -45deg about the
// top-right corner sends the text's left end that way, so the label ascends
// left-to-right and finishes at its own tick.
#let angled-label(body, angle) = context {
  let turned = rotate(angle * 1deg, origin: top + right, reflow: false, box(body))
  box(
    width: 0pt,
    height: measure(rotate(angle * 1deg, origin: top + right, reflow: true, box(body))).height,
    place(top + right, turned),
  )
}

#let axis-args(
  cs, data, px-length: 300, font-size: 10, band-px: 40, label-angle: none, horizontal: true,
) = {
  if falsy(cs) { return (:) }
  let args = (:)

  if is-discrete(cs) {
    let labels = category-values(data, cs)
    args.insert("ticks", range(labels.len()))

    // Angle the labels when they will not fit the band horizontally.
    //
    // Core decides this where it can (`compute_label_sizing` sets `labelAngle`
    // at narrow steps) but deliberately leaves it unset in one case, noting in
    // its own source that "omitting labelAngle leaves VL defaults (e.g. -45 on
    // ordinal)". lilaq has no such default, so the backend supplies the same
    // one — otherwise twenty category labels overprint each other.
    let angle = label-angle
    if angle == none and horizontal {
      let widest = if labels.len() == 0 { 0 } else { calc.max(..labels.map(l => l.len())) }
      let label-px = widest * font-size * APPROX_CHAR_WIDTH_RATIO
      if label-px > band-px { angle = -45 }
    }

    args.insert("format-ticks", (ticks, ..a) => ticks.map(i => {
      let idx = int(i)
      let body = if idx >= 0 and idx < labels.len() { [#labels.at(idx)] } else { [] }
      if angle == none or angle == 0 { body } else { angled-label(body, angle) }
    }))
    // There is nothing between January and February to subdivide. A discrete
    // axis has no positions other than the bands themselves, so subticks would
    // draw gridlines at values the data cannot take.
    args.insert("subticks", none)
    return args
  }

  if cs.at("type", default: none) == "temporal" {
    let coords = numeric-column(data, cs).filter(v => v != none).dedup().sorted()
    if coords.len() == 0 { return args }

    // How many labels fit depends on how wide one is, not just how long the
    // axis is. Core's `temporalFormat` cannot be used for the estimate here:
    // it describes the *data's* granularity, while the label will be written at
    // the granularity of whichever tick unit gets chosen below — and that
    // choice depends on the estimate. A nominal eight characters covers the
    // forms this produces ("Jan 2020", "Jan 01", "12:00") and breaks the
    // circularity.
    let nominal-label-px = 8 * font-size * APPROX_CHAR_WIDTH_RATIO
    let target = calc.max(2, int(px-length / calc.max(1, nominal-label-px * 1.4)))

    // Ticks land on calendar boundaries — a year, a quarter, a month, a
    // Monday, a midnight — rather than on arbitrary instants. lilaq has no date
    // scale, so its linear locator would produce round *numbers* here; core
    // never emits tick positions because Vega-Lite's time scale did this.
    // The tightest spacing the data actually has, which bounds how far the
    // ticks may usefully subdivide.
    let min-gap = if coords.len() < 2 { none } else {
      calc.min(..coords.windows(2).map(w => w.at(1) - w.at(0)))
    }
    let cal = calendar-ticks(coords.first(), coords.last(), target: target, min-gap: min-gap)
    if cal != none {
      args.insert("ticks", cal.ticks)
      // Label at the granularity of the *tick unit*, not of the data. Core's
      // own level -> format table does that mapping, so quarterly ticks read
      // "Jan 2020" rather than "Jan 01, 2020".
      let same-year = {
        let a = dh.from-ms(cal.ticks.first())
        let b = dh.from-ms(cal.ticks.last())
        a != none and b != none and a.year == b.year
      }
      let same-day = {
        let a = dh.from-ms(cal.ticks.first())
        let b = dh.from-ms(cal.ticks.last())
        a != none and b != none and a.year == b.year and a.ordinal == b.ordinal
      }
      let tick-pattern = level_to_format(cal.level, (sameYear: same-year, sameDay: same-day))
      if tick-pattern != none {
        args.insert("format-ticks", (ticks, ..a) => ticks.map(ms => {
          let m = dh.from-ms(ms)
          if m == none { [#ms] } else { [#_strftime(m, tick-pattern)] }
        }))
      }
      // Unlabelled ticks at the next calendar unit down: a year-labelled axis
      // still shows where the quarters fall. `subticks` itself only takes a
      // count of evenly spaced divisions, which months and quarters are not, so
      // the positions go through `locate-subticks`.
      if cal.subticks.len() > 0 {
        args.insert("locate-subticks", (x0, x1, ..a) => (ticks: cal.subticks))
      } else {
        args.insert("subticks", none)
      }
      return args
    }

    // No calendar unit fitted (a degenerate range): fall back to the data
    // points themselves, labelled with core's format.
    let fmt = temporal-formatter(cs)
    if fmt != none { args.insert("format-ticks", fmt) }
    args.insert("ticks", coords)
    args.insert("subticks", none)
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
/// The decisions core does not make because they are matters of taste, not of
/// semantics. Core is deliberately silent on all of these, so something has to
/// choose, and it should be the reader rather than this file.
#let DEFAULT-THEME = (
  /// Half a band of padding at each end of a category axis that is *not*
  /// banded. Off by default: a line already makes its own points findable, so
  /// the padding is empty margin. A scatter over categories may prefer it on.
  pad-categories: false,
  /// Breathing room on a zero baseline the data actually reaches, so a point at
  /// zero does not sit on top of the opposite spine. Ignored by marks that read
  /// by length — a bar has to start at the axis.
  pad-zero: true,
  /// Which corner the legend goes in, or `auto` to pick the emptiest one.
  legend-position: auto,
)

/// Axis limits from core's decisions.
///
/// `anchored` is whether the mark reads by *length* along this axis, in core's
/// own vocabulary (`markCognitiveChannel`). It is the difference between a
/// limit that is required and one that is merely convenient: a bar measured
/// from a baseline must touch it, a scatter point need only have it in view.
///
/// The distinction matters because in lilaq an explicit limit also switches off
/// the automatic margin on that side (`diagram.margin`, applied only where a
/// limit is `auto`). So pinning an end for semantic reasons silently removes
/// the padding too, which is how a point at x = 0 ends up drawn on the y axis.
#let limits-for(cs, values, data, banded: false, anchored: true, theme: DEFAULT-THEME) = {
  if falsy(cs) { return auto }
  if is-discrete(cs) {
    // The band count is the number of *categories*, not the number of rows —
    // `numeric-column` yields one coordinate per row, and several rows share a
    // band whenever the chart has series. Using the row count stretches the
    // axis past the data and leaves empty bands on the right.
    let n = category-values(data, cs).len()
    if n == 0 { return auto }
    // Half a band of padding at each end, but only when the axis is *banded*:
    // a bar occupies its whole band and would otherwise be clipped by the
    // frame. A line or scatter draws *at* the category position, so the same
    // padding is just empty margin before the first point and after the last.
    // Core already decided which axes are banded (`declareLayoutMode`).
    if banded { return (-0.6, n - 0.4) }
    if theme.pad-categories { return (-0.5, n - 0.5) }
    return (0, n - 1)
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
    // A mark that reads by position needs zero *in view*, not underfoot. When
    // the data stops short of zero, pinning the end there does both at once. It
    // is only when the data reaches zero exactly that the pin costs the
    // padding, and the point lands on the opposite spine — there, `auto`
    // includes zero just as well and keeps the margin.
    let touches-zero = lo == 0 or hi == 0
    if not anchored and theme.pad-zero and touches-zero { return auto }
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

/// The emptiest corner, for the legend to sit in.
///
/// Ours; lilaq places the legend at `top + right` unconditionally and has no
/// equivalent of matplotlib's `loc="best"`, so on a rising series it lands on
/// the data. This counts how many points fall in each corner and takes the
/// quietest, preferring lilaq's own default on a tie.
#let legend-corner(xs, ys, anchored: false) = {
  let pts = ()
  for i in range(calc.min(xs.len(), ys.len())) {
    let (x, y) = (xs.at(i), ys.at(i))
    if x == none or y == none or not is-finite(x) or not is-finite(y) { continue }
    pts.push((x, y))
  }
  if pts.len() == 0 { return top + right }
  let xs-f = pts.map(p => p.at(0))
  let ys-f = pts.map(p => p.at(1))
  let (x0, x1) = (calc.min(..xs-f), calc.max(..xs-f))
  let (y0, y1) = (calc.min(..ys-f), calc.max(..ys-f))
  let xr = if x1 == x0 { 1 } else { x1 - x0 }
  let yr = if y1 == y0 { 1 } else { y1 - y0 }
  // How much of each edge the legend is assumed to cover.
  let zone = 0.3
  let occupancy(hi-x, hi-y) = pts.filter(p => {
    let u = (p.at(0) - x0) / xr
    let v = (p.at(1) - y0) / yr
    let in-x = if hi-x { u > 1 - zone } else { u < zone }
    let in-y = if hi-y { v > 1 - zone } else { v < zone }
    in-x and in-y
  }).len()
  // A mark that reads by length is filled all the way down to its baseline, so
  // the bottom corners are never actually free however few points sit there.
  let candidates = if anchored {
    ((top + right, occupancy(true, true)), (top + left, occupancy(false, true)))
  } else {
    (
      (top + right, occupancy(true, true)),
      (top + left, occupancy(false, true)),
      (bottom + right, occupancy(true, false)),
      (bottom + left, occupancy(false, false)),
    )
  }
  let best = candidates.first()
  for c in candidates { if c.at(1) < best.at(1) { best = c } }
  best.at(0)
}

/// Assemble the diagram from a plan and the marks a template produced.
#let diagram-for(plan, marks, mark-reads: "position", theme: DEFAULT-THEME) = {
  let theme = DEFAULT-THEME + theme
  // Core's word for a mark measured from a baseline rather than placed at a
  // coordinate: a bar or an area, as against a line or a scatter point.
  let anchored = mark-reads == "length"
  let cs = plan.channelSemantics
  let layout = plan.layout
  let axis-flags = {
    let f = plan.declaration.at("axisFlags", default: none)
    if truthy(f) { f } else { (:) }
  }
  let is-banded(ch) = {
    let f = axis-flags.at(ch, default: none)
    truthy(f) and f.at("banded", default: false) == true
  }
  let x-cs = cs.at("x", default: none)
  let y-cs = cs.at("y", default: none)
  let length-axis = if is-banded("y") { "x" } else { "y" }

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
    // Only the axis the length actually runs along is anchored: a bar's value
    // axis, not its category axis. That is the axis opposite the banded one,
    // and y when neither is banded — an area over a temporal x still grows
    // upwards from its baseline.
    xlim: limits-for(
      x-cs, x-values, plan.data,
      banded: is-banded("x"), anchored: anchored and length-axis == "x", theme: theme,
    ),
    ylim: limits-for(
      y-cs, y-values, plan.data,
      banded: is-banded("y"), anchored: anchored and length-axis == "y", theme: theme,
    ),
    // `xaxis`/`yaxis` take an argument *dictionary*, which lilaq spreads into
    // its own `axis()` — passing a constructed `lq.axis` re-spreads the
    // element's own fields and it rejects them.
    // Core sized the labels too (`compute_label_sizing`), and the backend needs
    // the same number to estimate how many will fit.
    xaxis: axis-args(
      x-cs, plan.data,
      px-length: layout.subplotWidth,
      font-size: layout.xLabel.at("fontSize", default: 10),
      band-px: layout.at("xStep", default: 40),
      label-angle: layout.xLabel.at("labelAngle", default: none),
    ),
    yaxis: axis-args(
      y-cs, plan.data,
      px-length: layout.subplotHeight,
      font-size: layout.yLabel.at("fontSize", default: 10),
      band-px: layout.at("yStep", default: 40),
      label-angle: layout.yLabel.at("labelAngle", default: none),
      // A y-axis label sits in the left margin and reads horizontally whatever
      // the band height; core's `_discrete_y_axis_should_use_horizontal_labels`
      // says the same.
      horizontal: false,
    ),
    // lilaq draws a legend for any labelled plot; `(:)` is its default and
    // means "show one if there is anything to show". Only suppress it when
    // there is a single unlabelled series.
    legend: if series-groups(plan).len() <= 1 { none } else if theme.legend-position == auto {
      (position: legend-corner(x-values, y-values, anchored: anchored))
    } else {
      (position: theme.legend-position)
    },
    ..marks,
  )
}
