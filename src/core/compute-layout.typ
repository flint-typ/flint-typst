// Port of flint/core/compute_layout.py.
//
// The largest module upstream (1 241 lines) and the one a lilaq or primaviz
// backend leans on hardest: given resolved channel semantics and a data table,
// work out how much room each part of the chart needs.

#import "decisions.typ": (
  DEFAULT_GAS_PRESSURE_PARAMS, compute_axis_step, compute_gas_pressure, compute_label_sizing,
)
#import "js-round.typ": js_round
#import "js-date.typ": js_date_parse_ms
#import "py.typ": falsy, is-finite, is-float-string, is-nan, is_number, num-str, py_str, truthy

#let VL_SHORT_DISCRETE_CATEGORY_COUNT = 4
#let VL_SHORT_DISCRETE_LABEL_MAX_LEN = 8

/// Approximate width (px) of one label character at the given font size.
#let APPROX_CHAR_WIDTH_RATIO = 0.62

// flint/core/compute_layout.py _is_finite_number
//
// PORT-EXC: `try: float(s) / except (ValueError, TypeError)`.
#let _is_finite_number(s) = {
  if not is-float-string(s) { return false }
  is-finite(float(s))
}

// flint/core/compute_layout.py _compute_discrete_label_stats
//
// PORT-IDIOM: upstream collects into a `set` then lists it; `.dedup()` gives
// the same distinct labels. Only counts and maxima are read off, so the
// (unspecified) set ordering upstream never mattered.
#let _compute_discrete_label_stats(field, table) = {
  if falsy(field) { return none }
  let labels = table
    .map(row => row.at(field, default: none))
    .filter(v => v != none and v != "")
    .map(py_str)
    .dedup()
  if labels.len() == 0 { return none }
  (
    count: labels.len(),
    maxLen: calc.max(..labels.map(s => s.len())),
    allNumeric: labels.all(s => s.trim() != "" and _is_finite_number(s)),
  )
}

// flint/core/compute_layout.py _discrete_y_axis_should_use_horizontal_labels
//
// Few, short category strings keep the Y axis labels horizontal; banded Y
// labels read horizontally in the left margin regardless of band height.
#let _discrete_y_axis_should_use_horizontal_labels(field, channel_type, table) = {
  if falsy(field) { return false }
  if channel_type == "quantitative" { return true }
  let stats = _compute_discrete_label_stats(field, table)
  if stats == none { return false }
  if stats.count > VL_SHORT_DISCRETE_CATEGORY_COUNT { return false }
  stats.maxLen <= VL_SHORT_DISCRETE_LABEL_MAX_LEN
}

// flint/core/compute_layout.py _js_to_number
//
// Mirrors JS `+v` coercion, NaN for an unparseable string.
#let _js_to_number(v) = {
  if v == none { return 0.0 } // +null === 0
  if type(v) == bool { return if v { 1.0 } else { 0.0 } }
  if is_number(v) { return float(v) }
  if type(v) == str {
    let s = v.trim()
    if s == "" { return 0.0 }
    // PORT-EXC: `try: float(s) / except ValueError: return nan`.
    if is-float-string(s) { return float(s) }
    return float.nan
  }
  float.nan
}

// flint/core/compute_layout.py _js_to_date_number
#let _js_to_date_number(v) = {
  let ms = js_date_parse_ms(v)
  if ms == none { float.nan } else { ms }
}

// flint/core/compute_layout.py _is_nan
#let _is_nan(v) = is-nan(v)

// ---------------------------------------------------------------------------
// Stretch caps (per-dimension)
// ---------------------------------------------------------------------------

#let DEFAULT_BASE_SIZE = (width: 400, height: 320)

// flint/core/compute_layout.py resolve_base_size
//
// The target size layout aims for, clamped to the optional hard `ceiling` so a
// smaller canvas shrinks the chart rather than overflowing it.
#let resolve_base_size(spec_base_size, ceiling) = {
  let base = if truthy(spec_base_size) { spec_base_size } else { DEFAULT_BASE_SIZE }
  if falsy(ceiling) { return (width: base.width, height: base.height) }
  (
    width: calc.min(base.width, ceiling.width),
    height: calc.min(base.height, ceiling.height),
  )
}

// flint/core/compute_layout.py resolve_stretch_caps
#let resolve_stretch_caps(options) = {
  let default = options.at("maxStretch", default: 2)
  if default == none { default = 2 }
  let x = options.at("maxStretchX", default: none)
  let y = options.at("maxStretchY", default: none)
  if x == none { x = default }
  if y == none { y = default }
  (calc.max(1, x), calc.max(1, y))
}

// flint/core/compute_layout.py derive_stretch_caps
#let derive_stretch_caps(base_size, ceiling, options) = {
  let default = options.at("maxStretch", default: 2)
  if default == none { default = 2 }
  if truthy(ceiling) {
    return (
      maxStretchX: calc.max(1, ceiling.width / base_size.width),
      maxStretchY: calc.max(1, ceiling.height / base_size.height),
    )
  }
  (maxStretchX: default, maxStretchY: default)
}

// ---------------------------------------------------------------------------
// Series and banking
// ---------------------------------------------------------------------------

// flint/core/compute_layout.py count_distinct_series
#let count_distinct_series(channel_semantics, data) = {
  let field-of(ch) = {
    let cs = channel_semantics.at(ch, default: none)
    if truthy(cs) { cs.at("field", default: none) } else { none }
  }
  let series_fields = ()
  let color_field = field-of("color")
  let detail_field = field-of("detail")
  if truthy(color_field) { series_fields.push(color_field) }
  if truthy(detail_field) and detail_field != color_field { series_fields.push(detail_field) }
  if series_fields.len() == 0 { return 1 }
  data
    .map(row => series_fields
      .map(f => {
        let v = row.at(f, default: none)
        if v != none { py_str(v) } else { "" }
      })
      .join("\u{0}"))
    .dedup()
    .len()
}

// flint/core/compute_layout.py compute_banking_ar
//
// Picks an aspect ratio that "banks" the typical slope toward 45 degrees, the
// classic Cleveland result. Scatter data uses the ratio of standard
// deviations; connected series use a multi-scale median slope.
#let compute_banking_ar(x_values, y_values, x_domain, y_domain, series_keys, is_connected) = {
  let MIN_AR = 0.5
  let MAX_AR = 3.0
  let x_range = x_domain.at(1) - x_domain.at(0)
  let y_range = y_domain.at(1) - y_domain.at(0)
  if x_range <= 0 or y_range <= 0 { return 1 }

  if not is_connected {
    let n = x_values.len()
    // PORT-PERF candidate: upstream walks the data twice, once for the means
    // and once for the variances. Left as-is per the plan — port faithfully,
    // then optimise with the corpus as the guard.
    let sum_x = 0.0
    let sum_y = 0.0
    for i in range(n) {
      sum_x += (x_values.at(i) - x_domain.at(0)) / x_range
      sum_y += (y_values.at(i) - y_domain.at(0)) / y_range
    }
    let mean_x = sum_x / n
    let mean_y = sum_y / n
    let var_x = 0.0
    let var_y = 0.0
    for i in range(n) {
      let dx = (x_values.at(i) - x_domain.at(0)) / x_range - mean_x
      let dy = (y_values.at(i) - y_domain.at(0)) / y_range - mean_y
      var_x += dx * dx
      var_y += dy * dy
    }
    let sd_x = calc.sqrt(var_x / n)
    let sd_y = calc.sqrt(var_y / n)
    if sd_y <= 0 { return MAX_AR }
    if sd_x <= 0 { return MIN_AR }
    let sd_ratio = sd_x / sd_y
    let ar = if sd_ratio > 1 { 1 + (sd_ratio - 1) * 0.3 } else { 1 - (1 - sd_ratio) * 0.3 }
    return calc.min(MAX_AR, calc.max(MIN_AR, ar))
  }

  // Group points by series key, each series sorted by x.
  let series_keys_seen = ()
  let series_points = ()
  for i in range(x_values.len()) {
    let key = series_keys.at(i)
    let j = series_keys_seen.position(k => k == key)
    if j == none {
      series_keys_seen.push(key)
      series_points.push(((x: x_values.at(i), y: y_values.at(i)),))
    } else {
      series_points.at(j).push((x: x_values.at(i), y: y_values.at(i)))
    }
  }
  series_points = series_points.map(pts => pts.sorted(key: p => p.x))

  let max_series_len = 0
  for pts in series_points {
    if pts.len() > max_series_len { max_series_len = pts.len() }
  }
  if max_series_len <= 0 { return 1 }
  let max_scale = calc.max(0, calc.floor(calc.log(max_series_len, base: 2)) - 1)

  let scale_medians = ()
  for scale in range(max_scale + 1) {
    let window_size = 1.bit-lshift(scale)
    let abs_slopes = ()
    for pts in series_points {
      let n = pts.len()
      if n < 2 { continue }
      let smoothed = ()
      let i = 0
      while i < n {
        let end = calc.min(i + window_size, n)
        let sx = 0.0
        let sy = 0.0
        for j in range(i, end) {
          sx += pts.at(j).x
          sy += pts.at(j).y
        }
        let cnt = end - i
        smoothed.push((x: sx / cnt, y: sy / cnt))
        i += window_size
      }
      for k in range(1, smoothed.len()) {
        let dx = (smoothed.at(k).x - smoothed.at(k - 1).x) / x_range
        let dy = (smoothed.at(k).y - smoothed.at(k - 1).y) / y_range
        if dx == 0 { continue }
        abs_slopes.push(calc.abs(dy / dx))
      }
    }
    if abs_slopes.len() == 0 { continue }
    abs_slopes = abs_slopes.sorted()
    let mid = abs_slopes.len().bit-rshift(1)
    let median = if calc.rem(abs_slopes.len(), 2) == 1 {
      abs_slopes.at(mid)
    } else {
      (abs_slopes.at(mid - 1) + abs_slopes.at(mid)) / 2
    }
    if median > 0 { scale_medians.push(median) }
  }

  if scale_medians.len() == 0 { return 1 }
  let log_sum = scale_medians.map(calc.ln).sum()
  let combined_slope = calc.exp(log_sum / scale_medians.len())
  if combined_slope <= 0 { return MAX_AR }
  let ar = calc.max(1.0, combined_slope)
  calc.min(MAX_AR, calc.max(MIN_AR, ar))
}

// ---------------------------------------------------------------------------
// Facet grid
// ---------------------------------------------------------------------------

// Ours. The channel-semantics accessors upstream re-inlines at every site.
#let _cs-of(channel_semantics, ch) = {
  let cs = channel_semantics.at(ch, default: none)
  if truthy(cs) { cs } else { none }
}
#let _field-of(channel_semantics, ch) = {
  let cs = _cs-of(channel_semantics, ch)
  if cs == none { none } else { cs.at("field", default: none) }
}
#let _resolved-type(declaration, channel_semantics, ch) = {
  let rt = {
    let r = declaration.at("resolvedTypes", default: none)
    if truthy(r) { r } else { (:) }
  }.at(ch, default: none)
  if truthy(rt) { return rt }
  let cs = _cs-of(channel_semantics, ch)
  if cs == none { none } else { cs.at("type", default: none) }
}
#let _is-banded(declaration, ch) = {
  let flags = {
    let f = declaration.at("axisFlags", default: none)
    if truthy(f) { f } else { (:) }
  }.at(ch, default: none)
  let f = if truthy(flags) { flags } else { (:) }
  f.at("banded", default: none) == true
}
#let _distinct-count(data, field) = data.map(r => r.at(field, default: none)).dedup().len()

// flint/core/compute_layout.py compute_facet_grid
#let compute_facet_grid(channel_semantics, declaration, data, canvas_size, options) = {
  let (ms_x, ms_y) = resolve_stretch_caps(options)
  let facet_fixed_padding = {
    let f = options.at("facetFixedPadding", default: none)
    if truthy(f) { f } else { (:) }
  }
  let fix_w = facet_fixed_padding.at("width", default: 0)
  let fix_h = facet_fixed_padding.at("height", default: 0)
  let gap = options.at("facetGap", default: 0)
  let min_step = options.at("minStep", default: 6)
  let step_padding = options.at("stepPadding", default: 0.1)
  let base_min_subplot = options.at("minSubplotSize", default: 60)

  let is_discrete_type(t) = t == "nominal" or t == "ordinal"

  let max_w = canvas_size.width * ms_x - fix_w
  let max_h = canvas_size.height * ms_y - fix_h
  let MIN_GROUP_GAP_PX = 3

  let group_field = _field-of(channel_semantics, "group")
  let group_count = 0
  let group_axis = none
  if truthy(group_field) {
    group_count = _distinct-count(data, group_field)
    let x_type = _resolved-type(declaration, channel_semantics, "x")
    let y_type = _resolved-type(declaration, channel_semantics, "y")
    if is_discrete_type(x_type) { group_axis = "x" } else if is_discrete_type(y_type) { group_axis = "y" }
  }

  let min_subplot_width = base_min_subplot
  let min_subplot_height = base_min_subplot

  let LOG_PX_PER_DECADE_FACET = 40
  for axis in ("x", "y") {
    let cs = _cs-of(channel_semantics, axis)
    if cs == none or falsy(cs.at("field", default: none)) or falsy(cs.at("scaleType", default: none)) {
      continue
    }
    if cs.scaleType != "log" and cs.scaleType != "symlog" { continue }
    let vals = data
      .map(r => r.at(cs.field, default: none))
      .filter(v => type(v) != bool and is_number(v) and is-finite(v) and v > 0)
    if vals.len() < 2 { continue }
    let decades = calc.log(calc.max(..vals)) - calc.log(calc.min(..vals))
    let needed = calc.ceil(calc.max(1, decades)) * LOG_PX_PER_DECADE_FACET
    if axis == "x" {
      min_subplot_width = calc.max(min_subplot_width, needed)
    } else {
      min_subplot_height = calc.max(min_subplot_height, needed)
    }
  }

  for axis in ("x", "y") {
    let cs = _cs-of(channel_semantics, axis)
    if cs == none or falsy(cs.at("field", default: none)) { continue }
    let effective_type = _resolved-type(declaration, channel_semantics, axis)
    let is_banded = _is-banded(declaration, axis)
    if not is_discrete_type(effective_type) and not is_banded { continue }
    let value_count = _distinct-count(data, cs.field)
    let axis_group_count = if group_axis == axis and group_count > 1 { group_count } else { 1 }
    let max_dim = if axis == "x" { max_w } else { max_h }
    let per_category_step = if axis_group_count > 1 {
      let min_group_step = calc.max(
        calc.ceil(MIN_GROUP_GAP_PX / step_padding), 2 * axis_group_count,
      )
      calc.max(min_step * axis_group_count, min_group_step)
    } else {
      min_step
    }
    let data_driven_min = calc.min(per_category_step * value_count, max_dim)
    let min_dim = calc.max(base_min_subplot, data_driven_min)
    if axis == "x" { min_subplot_width = min_dim } else { min_subplot_height = min_dim }
  }

  let axis_is_cont(axis) = {
    let cs = _cs-of(channel_semantics, axis)
    if cs == none or falsy(cs.at("field", default: none)) { return false }
    let t = _resolved-type(declaration, channel_semantics, axis)
    not is_discrete_type(t) and not _is-banded(declaration, axis)
  }

  if axis_is_cont("x") and axis_is_cont("y") {
    let x_cs = _cs-of(channel_semantics, "x")
    let y_cs = _cs-of(channel_semantics, "y")
    if (
      x_cs != none and truthy(x_cs.at("field", default: none))
        and y_cs != none and truthy(y_cs.at("field", default: none))
    ) {
      let is_temp_x = _resolved-type(declaration, channel_semantics, "x") == "temporal"
      let is_temp_y = _resolved-type(declaration, channel_semantics, "y") == "temporal"
      let cmcs = options.at("continuousMarkCrossSection", default: none)
      let is_conn = type(cmcs) == dictionary and truthy(cmcs.at("seriesCountAxis", default: none))

      let s_fields = ()
      let col_f = _field-of(channel_semantics, "column")
      let row_f = _field-of(channel_semantics, "row")
      if truthy(col_f) { s_fields.push(col_f) }
      if truthy(row_f) { s_fields.push(row_f) }
      let cf = _field-of(channel_semantics, "color")
      let df = _field-of(channel_semantics, "detail")
      if truthy(cf) { s_fields.push(cf) }
      if truthy(df) and df != cf { s_fields.push(df) }

      let x_num = ()
      let y_num = ()
      let s_keys = ()
      for row in data {
        let xv = row.at(x_cs.field, default: none)
        let yv = row.at(y_cs.field, default: none)
        if xv == none or yv == none { continue }
        let xn = if is_temp_x { _js_to_date_number(xv) } else { _js_to_number(xv) }
        let yn = if is_temp_y { _js_to_date_number(yv) } else { _js_to_number(yv) }
        if _is_nan(xn) or _is_nan(yn) { continue }
        x_num.push(xn)
        y_num.push(yn)
        s_keys.push(if s_fields.len() > 0 {
          s_fields.map(f => {
            let v = row.at(f, default: none)
            if v != none { py_str(v) } else { "" }
          }).join("\u{0}")
        } else { "" })
      }

      if x_num.len() > 1 {
        let x_dom = (calc.min(..x_num), calc.max(..x_num))
        let y_dom = (calc.min(..y_num), calc.max(..y_num))
        let x_zero = {
          let z = x_cs.at("zero", default: none)
          if truthy(z) { truthy(z.at("zero", default: none)) } else { false }
        }
        let y_zero = {
          let z = y_cs.at("zero", default: none)
          if truthy(z) { truthy(z.at("zero", default: none)) } else { false }
        }
        if x_zero {
          if x_dom.at(0) > 0 { x_dom.at(0) = 0 }
          if x_dom.at(1) < 0 { x_dom.at(1) = 0 }
        }
        if y_zero {
          if y_dom.at(0) > 0 { y_dom.at(0) = 0 }
          if y_dom.at(1) < 0 { y_dom.at(1) = 0 }
        }
        let ar = compute_banking_ar(x_num, y_num, x_dom, y_dom, s_keys, is_conn)
        if ar >= 1 {
          min_subplot_width = calc.max(
            min_subplot_width, js_round(base_min_subplot * calc.min(ar, ms_x)),
          )
          min_subplot_height = calc.max(min_subplot_height, base_min_subplot)
        } else {
          min_subplot_width = calc.max(min_subplot_width, base_min_subplot)
          min_subplot_height = calc.max(
            min_subplot_height, js_round(base_min_subplot * calc.min(1 / ar, ms_y)),
          )
        }
      }
    }
  }

  let max_facet_columns = calc.max(1, calc.floor(max_w / (min_subplot_width + gap)))
  let max_facet_rows = calc.max(1, calc.floor(max_h / (min_subplot_height + gap)))

  let col_field = _field-of(channel_semantics, "column")
  let row_field = _field-of(channel_semantics, "row")
  if falsy(col_field) and falsy(row_field) { return none }

  let col_count = if truthy(col_field) { _distinct-count(data, col_field) } else { 0 }
  let row_count = if truthy(row_field) { _distinct-count(data, row_field) } else { 0 }
  if col_count == 0 and row_count == 0 { return none }

  if col_count > 0 and row_count == 0 {
    if col_count <= max_facet_columns {
      return (
        columns: col_count, rows: 1,
        maxColumnValues: col_count, maxRowValues: max_facet_rows,
      )
    }
    let n_cols = max_facet_columns
    let n_rows = calc.ceil(col_count / n_cols)
    // Avoid a final row holding a single subplot.
    while n_cols > 2 and calc.rem(col_count, n_cols) == 1 {
      n_cols -= 1
      n_rows = calc.ceil(col_count / n_cols)
    }
    let vis_rows = calc.min(n_rows, max_facet_rows)
    return (
      columns: n_cols, rows: vis_rows,
      maxColumnValues: n_cols * vis_rows, maxRowValues: max_facet_rows,
    )
  }

  (
    columns: calc.max(1, calc.min(col_count, max_facet_columns)),
    rows: calc.max(1, calc.min(row_count, max_facet_rows)),
    maxColumnValues: max_facet_columns,
    maxRowValues: max_facet_rows,
  )
}

// ---------------------------------------------------------------------------
// Channel budgets
// ---------------------------------------------------------------------------

// flint/core/compute_layout.py compute_channel_budgets
#let compute_channel_budgets(channel_semantics, declaration, data, canvas_size, options) = {
  let (max_stretch_x, max_stretch_y) = resolve_stretch_caps(options)
  let min_step_val = options.at("minStep", default: 6)
  let step_padding_val = options.at("stepPadding", default: 0.1)
  let max_color_val = options.at("maxColorValues", default: 24)

  let facet_fixed_padding = {
    let f = options.at("facetFixedPadding", default: none)
    if truthy(f) { f } else { (:) }
  }
  let fix_w = facet_fixed_padding.at("width", default: 0)
  let fix_h = facet_fixed_padding.at("height", default: 0)
  let gap = options.at("facetGap", default: 0)
  let min_subplot = options.at("minSubplotSize", default: 60)

  let is_discrete_type(t) = t == "nominal" or t == "ordinal"

  let facet_grid = compute_facet_grid(channel_semantics, declaration, data, canvas_size, options)
  let facet_cols = if truthy(facet_grid) { facet_grid.columns } else { 1 }
  let facet_rows = if truthy(facet_grid) { facet_grid.rows } else { 1 }

  let max_subplot_w = calc.max(
    min_subplot, (canvas_size.width * max_stretch_x - fix_w) / facet_cols - gap,
  )
  let max_subplot_h = calc.max(
    min_subplot, (canvas_size.height * max_stretch_y - fix_h) / facet_rows - gap,
  )

  let group_field = _field-of(channel_semantics, "group")
  let group_count = 0
  let group_axis = none
  if truthy(group_field) {
    group_count = _distinct-count(data, group_field)
    if is_discrete_type(_resolved-type(declaration, channel_semantics, "x")) {
      group_axis = "x"
    } else if is_discrete_type(_resolved-type(declaration, channel_semantics, "y")) {
      group_axis = "y"
    }
  }

  let x_group_multiplier = if group_axis == "x" and group_count > 1 { group_count } else { 1 }
  let y_group_multiplier = if group_axis == "y" and group_count > 1 { group_count } else { 1 }

  let MIN_GROUP_GAP_PX = 3
  let x_min_group_step = if x_group_multiplier > 1 {
    calc.max(calc.ceil(MIN_GROUP_GAP_PX / step_padding_val), 2 * x_group_multiplier)
  } else { min_step_val }
  let y_min_group_step = if y_group_multiplier > 1 {
    calc.max(calc.ceil(MIN_GROUP_GAP_PX / step_padding_val), 2 * y_group_multiplier)
  } else { min_step_val }

  let max_x_to_keep = calc.floor(max_subplot_w / x_min_group_step)
  let max_y_to_keep = calc.floor(max_subplot_h / y_min_group_step)

  if truthy(facet_grid) {
    let canvas_x_cap = calc.max(1, calc.floor(canvas_size.width / x_min_group_step))
    let canvas_y_cap = calc.max(1, calc.floor(canvas_size.height / y_min_group_step))

    if max_x_to_keep > canvas_x_cap or max_y_to_keep > canvas_y_cap {
      max_x_to_keep = calc.min(max_x_to_keep, canvas_x_cap)
      max_y_to_keep = calc.min(max_y_to_keep, canvas_y_cap)

      let col_field = _field-of(channel_semantics, "column")
      let row_field = _field-of(channel_semantics, "row")
      let col_count = if truthy(col_field) { _distinct-count(data, col_field) } else { 0 }

      if col_count > 1 and falsy(row_field) {
        let tighter_w = calc.max(min_subplot, max_x_to_keep * x_min_group_step)
        let total_w = canvas_size.width * max_stretch_x - fix_w
        let total_h = canvas_size.height * max_stretch_y - fix_h
        let revised_max_cols = calc.max(1, calc.floor(total_w / (tighter_w + gap)))
        let revised_max_rows = calc.max(1, calc.floor(total_h / (min_subplot + gap)))
        let max_total = revised_max_cols * revised_max_rows
        let effective_count = calc.min(col_count, max_total)
        let vis_rows = calc.ceil(effective_count / revised_max_cols)
        let vis_cols = calc.ceil(effective_count / vis_rows)
        // PORT-MUT: upstream mutates the facet_grid dict in place; here it is a
        // local value that is returned, so reassignment is equivalent.
        facet_grid.insert("columns", vis_cols)
        facet_grid.insert("rows", vis_rows)
        facet_grid.insert("maxColumnValues", max_total)
      }
    }
  }

  let column_max = if truthy(facet_grid) {
    let v = facet_grid.at("maxColumnValues", default: none)
    if v == none { float.inf } else { v }
  } else { float.inf }
  let row_max = if truthy(facet_grid) {
    let v = facet_grid.at("maxRowValues", default: none)
    if v == none { float.inf } else { v }
  } else { float.inf }

  (
    maxValues: (
      x: max_x_to_keep,
      y: max_y_to_keep,
      column: column_max,
      row: row_max,
      color: max_color_val,
    ),
    facetGrid: facet_grid,
  )
}

// ---------------------------------------------------------------------------
// Minimum subplot dimensions
// ---------------------------------------------------------------------------

// flint/core/compute_layout.py compute_min_subplot_dimensions
#let compute_min_subplot_dimensions(channel_semantics, declaration, data, options) = {
  let min_step = options.at("minStep", default: 6)
  let min_subplot = options.at("minSubplotSize", default: 60)

  let min_subplot_width = min_subplot
  let min_subplot_height = min_subplot

  let LOG_PX_PER_DECADE_MIN = 40
  for axis in ("x", "y") {
    let cs = _cs-of(channel_semantics, axis)
    if cs == none or falsy(cs.at("field", default: none)) or falsy(cs.at("scaleType", default: none)) {
      continue
    }
    if cs.scaleType != "log" and cs.scaleType != "symlog" { continue }
    let vals = data
      .map(r => r.at(cs.field, default: none))
      .filter(v => type(v) != bool and is_number(v) and is-finite(v) and v > 0)
    if vals.len() < 2 { continue }
    let decades = calc.log(calc.max(..vals)) - calc.log(calc.min(..vals))
    let needed = calc.ceil(calc.max(1, decades)) * LOG_PX_PER_DECADE_MIN
    if axis == "x" {
      min_subplot_width = calc.max(min_subplot_width, needed)
    } else {
      min_subplot_height = calc.max(min_subplot_height, needed)
    }
  }

  let is_discrete_type(t) = t == "nominal" or t == "ordinal"

  for axis in ("x", "y") {
    let cs = _cs-of(channel_semantics, axis)
    if cs == none or falsy(cs.at("field", default: none)) { continue }
    let effective_type = _resolved-type(declaration, channel_semantics, axis)
    let is_banded = _is-banded(declaration, axis)
    let is_discrete = is_discrete_type(effective_type)
    let item_count = if is_banded or is_discrete { _distinct-count(data, cs.field) } else { 0 }
    if item_count > 0 {
      let min_dim = calc.max(min_subplot, item_count * min_step)
      if axis == "x" {
        min_subplot_width = calc.max(min_subplot_width, min_dim)
      } else {
        min_subplot_height = calc.max(min_subplot_height, min_dim)
      }
    }
  }

  (minSubplotWidth: min_subplot_width, minSubplotHeight: min_subplot_height)
}

// ---------------------------------------------------------------------------
// Public API: compute_layout
// ---------------------------------------------------------------------------

// flint/core/compute_layout.py compute_layout
//
// The whole sizing pipeline: count discrete items per axis, size the facet
// grid, apply gas pressure or banking to continuous axes, derive the per-band
// step, clamp to the stretch budget, then size the labels.
#let compute_layout(
  channel_semantics, declaration, table, canvas_size, options: none, facet_grid: none,
) = {
  let options = if truthy(options) { options } else { (:) }
  let elasticity_val = options.at("elasticity", default: 0.5)
  let (max_stretch_x, max_stretch_y) = resolve_stretch_caps(options)
  let facet_elasticity_val = options.at("facetElasticity", default: 0.3)
  let min_step_val = options.at("minStep", default: 6)
  let min_subplot_val = options.at("minSubplotSize", default: 60)
  let step_padding_val = options.at("stepPadding", default: 0.1)
  let maintain_continuous_axis_ratio = options.at("maintainContinuousAxisRatio", default: false)
  let continuous_mark_cross_section = options.at("continuousMarkCrossSection", default: none)
  let facet_aspect_ratio_resistance = options.at("facetAspectRatioResistance", default: 0)

  let default_chart_width = canvas_size.width
  let default_chart_height = canvas_size.height

  let facet_fixed_padding = {
    let f = options.at("facetFixedPadding", default: none)
    if truthy(f) { f } else { (:) }
  }
  let fix_w = facet_fixed_padding.at("width", default: 0)
  let fix_h = facet_fixed_padding.at("height", default: 0)
  let gap = options.at("facetGap", default: 0)

  let base_ref_size = 300
  let size_ratio = calc.max(default_chart_width, default_chart_height) / base_ref_size
  let base_band_size = options.at("defaultBandSize", default: 20)
  let default_step_size = js_round(base_band_size * calc.max(1, size_ratio))

  let is_discrete_type(t) = t == "nominal" or t == "ordinal"

  let resolved_types = {
    let r = declaration.at("resolvedTypes", default: none)
    if truthy(r) { r } else { (:) }
  }
  let effective_types = (:)
  for (ch, cs) in channel_semantics.pairs() {
    let resolved = resolved_types.at(ch, default: none)
    effective_types.insert(ch, if truthy(resolved) { resolved } else { cs.at("type", default: none) })
  }
  let eff_type(ch) = {
    let e = effective_types.at(ch, default: none)
    if truthy(e) { return e }
    let cs = _cs-of(channel_semantics, ch)
    if cs == none { none } else { cs.at("type", default: none) }
  }

  let axis_flags = {
    let f = declaration.at("axisFlags", default: none)
    if truthy(f) { f } else { (:) }
  }
  let banded_of(ch) = {
    let f = axis_flags.at(ch, default: none)
    if truthy(f) { f.at("banded", default: false) } else { false }
  }
  let x_banded = banded_of("x")
  let y_banded = banded_of("y")

  let nominal_count = (x: 0, y: 0, column: 0, row: 0, group: 0)

  for channel in ("x", "y", "column", "row", "color") {
    let cs = _cs-of(channel_semantics, channel)
    if cs == none or falsy(cs.at("field", default: none)) { continue }
    if not is_discrete_type(eff_type(channel)) { continue }
    nominal_count.insert(channel, _distinct-count(table, cs.field))
  }

  let group_field = _field-of(channel_semantics, "group")
  let group_axis = none
  if truthy(group_field) {
    nominal_count.insert("group", _distinct-count(table, group_field))
    if is_discrete_type(eff_type("x")) {
      group_axis = "x"
    } else if is_discrete_type(eff_type("y")) {
      group_axis = "y"
    }
  }

  let x_group_multiplier = if group_axis == "x" and nominal_count.group > 1 { nominal_count.group } else { 1 }
  let y_group_multiplier = if group_axis == "y" and nominal_count.group > 1 { nominal_count.group } else { 1 }
  let x_total_nominal_count = nominal_count.x * x_group_multiplier
  let y_total_nominal_count = nominal_count.y * y_group_multiplier

  let MIN_GROUP_GAP_PX = 3
  let x_min_group_step = if x_group_multiplier > 1 {
    calc.max(calc.ceil(MIN_GROUP_GAP_PX / step_padding_val), 2 * x_group_multiplier)
  } else { min_step_val }
  let y_min_group_step = if y_group_multiplier > 1 {
    calc.max(calc.ceil(MIN_GROUP_GAP_PX / step_padding_val), 2 * y_group_multiplier)
  } else { min_step_val }

  let binned_axes = {
    let b = declaration.at("binnedAxes", default: none)
    if truthy(b) { b } else { (:) }
  }
  let x_continuous_as_discrete = 0
  let y_continuous_as_discrete = 0
  for axis in ("x", "y") {
    let cs = _cs-of(channel_semantics, axis)
    if cs == none or falsy(cs.at("field", default: none)) { continue }
    if is_discrete_type(eff_type(axis)) { continue }
    let is_banded = if axis == "x" { x_banded } else { y_banded }
    let is_binned = binned_axes.at(axis, default: none)
    if falsy(is_banded) and falsy(is_binned) { continue }
    let count = if truthy(is_binned) {
      if type(is_binned) == dictionary and truthy(is_binned.at("maxbins", default: none)) {
        is_binned.maxbins
      } else { 10 }
    } else {
      _distinct-count(table, cs.field)
    }
    if count <= 1 { continue }
    if axis == "x" { x_continuous_as_discrete = count } else { y_continuous_as_discrete = count }
  }

  // Facet grid
  let facet_cols = 1
  let facet_rows = 1
  if truthy(facet_grid) {
    facet_cols = facet_grid.columns
    facet_rows = facet_grid.rows
  } else {
    if nominal_count.column > 0 { facet_cols = nominal_count.column }
    if nominal_count.row > 0 { facet_rows = nominal_count.row }
  }

  // Log boost
  let LOG_PX_PER_DECADE = 40
  let log_boost_x = 0
  let log_boost_y = 0
  for axis in ("x", "y") {
    let cs = _cs-of(channel_semantics, axis)
    if cs == none or falsy(cs.at("field", default: none)) or falsy(cs.at("scaleType", default: none)) {
      continue
    }
    if cs.scaleType != "log" and cs.scaleType != "symlog" { continue }
    let vals = table
      .map(r => r.at(cs.field, default: none))
      .filter(v => type(v) != bool and is_number(v) and is-finite(v) and v > 0)
    if vals.len() < 2 { continue }
    let decades = calc.log(calc.max(..vals)) - calc.log(calc.min(..vals))
    let needed = calc.ceil(calc.max(1, decades)) * LOG_PX_PER_DECADE
    if axis == "x" { log_boost_x = needed } else { log_boost_y = needed }
  }

  let min_continuous_size = calc.max(10, min_step_val)
  let min_continuous_size_x = calc.max(min_continuous_size, log_boost_x)
  let min_continuous_size_y = calc.max(min_continuous_size, log_boost_y)

  let subplot_width = if facet_cols > 1 {
    let stretch = calc.min(max_stretch_x, calc.pow(facet_cols, facet_elasticity_val))
    js_round(calc.max(
      min_continuous_size_x, (default_chart_width * stretch - fix_w) / facet_cols - gap,
    ))
  } else { default_chart_width }

  let subplot_height = if facet_rows > 1 {
    let stretch = calc.min(max_stretch_y, calc.pow(facet_rows, facet_elasticity_val))
    js_round(calc.max(
      min_continuous_size_y, (default_chart_height * stretch - fix_h) / facet_rows - gap,
    ))
  } else { default_chart_height }

  // Facet aspect-ratio resistance
  let x_is_continuous_non_banded = x_total_nominal_count == 0 and x_continuous_as_discrete == 0
  let y_is_continuous_non_banded = y_total_nominal_count == 0 and y_continuous_as_discrete == 0
  let both_continuous_non_banded = x_is_continuous_non_banded and y_is_continuous_non_banded

  if (
    facet_aspect_ratio_resistance > 0 and not both_continuous_non_banded
      and (facet_cols > 1 or facet_rows > 1)
  ) {
    let base_ar = default_chart_width / default_chart_height
    let facet_ar = subplot_width / subplot_height
    let ar_drift = facet_ar / base_ar
    if ar_drift < 1 {
      subplot_height = js_round(calc.max(
        min_continuous_size_y, subplot_height * calc.pow(ar_drift, facet_aspect_ratio_resistance),
      ))
    } else if ar_drift > 1 {
      subplot_width = js_round(calc.max(
        min_continuous_size_x, subplot_width * calc.pow(1 / ar_drift, facet_aspect_ratio_resistance),
      ))
    }
  }

  if both_continuous_non_banded {
    let x_cs = _cs-of(channel_semantics, "x")
    let y_cs = _cs-of(channel_semantics, "y")
    if (
      x_cs != none and truthy(x_cs.at("field", default: none))
        and y_cs != none and truthy(y_cs.at("field", default: none))
    ) {
      let is_temp_x = eff_type("x") == "temporal"
      let is_temp_y = eff_type("y") == "temporal"
      let x_numeric = ()
      let y_numeric = ()
      for row in table {
        let xv = row.at(x_cs.field, default: none)
        let yv = row.at(y_cs.field, default: none)
        if xv == none or yv == none { continue }
        let xv2 = if is_temp_x { _js_to_date_number(xv) } else { _js_to_number(xv) }
        let yv2 = if is_temp_y { _js_to_date_number(yv) } else { _js_to_number(yv) }
        if _is_nan(xv2) or _is_nan(yv2) { continue }
        x_numeric.push(xv2)
        y_numeric.push(yv2)
      }

      if x_numeric.len() > 1 {
        let x_min = calc.min(..x_numeric)
        let x_max = calc.max(..x_numeric)
        let y_min = calc.min(..y_numeric)
        let y_max = calc.max(..y_numeric)

        let x_domain = (x_min, x_max)
        let y_domain = (y_min, y_max)
        let zero_of(cs) = {
          let z = cs.at("zero", default: none)
          if truthy(z) { truthy(z.at("zero", default: none)) } else { false }
        }
        if zero_of(x_cs) {
          if x_domain.at(0) > 0 { x_domain.at(0) = 0 }
          if x_domain.at(1) < 0 { x_domain.at(1) = 0 }
        }
        if zero_of(y_cs) {
          if y_domain.at(0) > 0 { y_domain.at(0) = 0 }
          if y_domain.at(1) < 0 { y_domain.at(1) = 0 }
        }

        let x_span = x_domain.at(1) - x_domain.at(0)
        let y_span = y_domain.at(1) - y_domain.at(0)
        let x_data_coverage = if x_span > 0 { (x_max - x_min) / x_span } else { 1 }
        let y_data_coverage = if y_span > 0 { (y_max - y_min) / y_span } else { 1 }
        let BANKING_COVERAGE_THRESHOLD = 0.2

        let gas_pressure_params = DEFAULT_GAS_PRESSURE_PARAMS
        if continuous_mark_cross_section != none {
          if is_number(continuous_mark_cross_section) {
            gas_pressure_params = (
              : ..DEFAULT_GAS_PRESSURE_PARAMS, markCrossSection: continuous_mark_cross_section,
            )
          } else {
            let cmcs = continuous_mark_cross_section
            let max_cs = calc.max(cmcs.x, cmcs.y)
            gas_pressure_params = (
              : ..DEFAULT_GAS_PRESSURE_PARAMS,
              markCrossSection: max_cs,
              markCrossSectionX: cmcs.x,
              markCrossSectionY: cmcs.y,
            )
            if cmcs.at("elasticity", default: none) != none {
              gas_pressure_params.insert("elasticity", cmcs.elasticity)
            }
            if cmcs.at("maxStretch", default: none) != none {
              gas_pressure_params.insert("maxStretch", cmcs.maxStretch)
            }
            if truthy(cmcs.at("seriesCountAxis", default: none)) {
              let resolved_axis = if cmcs.seriesCountAxis == "auto" { "y" } else { cmcs.seriesCountAxis }
              let n_series = count_distinct_series(channel_semantics, table)
              if resolved_axis == "y" {
                gas_pressure_params.insert("yItemCountOverride", n_series)
              } else {
                gas_pressure_params.insert("xItemCountOverride", n_series)
              }
            }
          }
        }

        let per_subplot_canvas_w = if facet_cols > 1 {
          calc.max(min_continuous_size_x, (
            default_chart_width * calc.min(max_stretch_x, calc.pow(facet_cols, facet_elasticity_val))
              - fix_w
          ) / facet_cols - gap)
        } else { default_chart_width }
        let per_subplot_canvas_h = if facet_rows > 1 {
          calc.max(min_continuous_size_y, (
            default_chart_height * calc.min(max_stretch_y, calc.pow(facet_rows, facet_elasticity_val))
              - fix_h
          ) / facet_rows - gap)
        } else { default_chart_height }

        let ideal_result = compute_gas_pressure(
          x_numeric, y_numeric, x_domain, y_domain,
          per_subplot_canvas_w, per_subplot_canvas_h, params: gas_pressure_params,
        )

        let is_connected = (
          type(continuous_mark_cross_section) == dictionary
            and truthy(continuous_mark_cross_section.at("seriesCountAxis", default: none))
        )
        let use_banking = (
          x_data_coverage >= BANKING_COVERAGE_THRESHOLD
            and y_data_coverage >= BANKING_COVERAGE_THRESHOLD
        )

        let raw_w = per_subplot_canvas_w * ideal_result.rawStretchX
        let raw_h = per_subplot_canvas_h * ideal_result.rawStretchY

        let ideal_w = raw_w
        let ideal_h = raw_h
        if use_banking {
          let series_fields = ()
          let color_field = _field-of(channel_semantics, "color")
          let detail_field = _field-of(channel_semantics, "detail")
          if truthy(color_field) { series_fields.push(color_field) }
          if truthy(detail_field) and detail_field != color_field { series_fields.push(detail_field) }

          let per_point_series_keys = ()
          if series_fields.len() > 0 {
            for row in table {
              let xv = row.at(x_cs.field, default: none)
              let yv = row.at(y_cs.field, default: none)
              if xv == none or yv == none { continue }
              let xn = if is_temp_x { _js_to_date_number(xv) } else { _js_to_number(xv) }
              let yn = if is_temp_y { _js_to_date_number(yv) } else { _js_to_number(yv) }
              if _is_nan(xn) or _is_nan(yn) { continue }
              per_point_series_keys.push(series_fields.map(f => {
                let v = row.at(f, default: none)
                if v != none { py_str(v) } else { "" }
              }).join("\u{0}"))
            }
          } else {
            per_point_series_keys = x_numeric.map(_ => "")
          }

          let banking_ar = compute_banking_ar(
            x_numeric, y_numeric, x_domain, y_domain, per_point_series_keys, is_connected,
          )

          let BANKING_BLEND = 0.5
          let gas_ar = raw_w / raw_h
          let blended_ar = if gas_ar > 0 and banking_ar > 0 {
            calc.exp((1 - BANKING_BLEND) * calc.ln(gas_ar) + BANKING_BLEND * calc.ln(banking_ar))
          } else { banking_ar }

          let raw_area = raw_w * raw_h
          let max_area = per_subplot_canvas_w * per_subplot_canvas_h * calc.max(max_stretch_x, max_stretch_y)
          let area = calc.min(raw_area, max_area)

          ideal_w = calc.sqrt(area * blended_ar)
          ideal_h = calc.sqrt(area / blended_ar)
        }

        let avail_w = if facet_cols > 1 {
          calc.max(min_continuous_size_x, (default_chart_width * max_stretch_x - fix_w) / facet_cols - gap)
        } else { default_chart_width * max_stretch_x }
        let avail_h = if facet_rows > 1 {
          calc.max(min_continuous_size_y, (default_chart_height * max_stretch_y - fix_h) / facet_rows - gap)
        } else { default_chart_height * max_stretch_y }

        let scale_x = if ideal_w > avail_w { avail_w / ideal_w } else { 1 }
        let scale_y = if ideal_h > avail_h { avail_h / ideal_h } else { 1 }
        let fit_scale = calc.min(scale_x, scale_y)

        subplot_width = js_round(calc.max(ideal_w * fit_scale, min_continuous_size_x))
        subplot_height = js_round(calc.max(ideal_h * fit_scale, min_continuous_size_y))
      }
    }
  } else if x_is_continuous_non_banded or y_is_continuous_non_banded {
    let cont_axis = if x_is_continuous_non_banded { "x" } else { "y" }
    let other_axis_has_discrete = if cont_axis == "x" {
      y_total_nominal_count > 0 or y_continuous_as_discrete > 0
    } else {
      x_total_nominal_count > 0 or x_continuous_as_discrete > 0
    }

    let series_stretch_applied = false
    let cmcs = continuous_mark_cross_section
    if type(cmcs) == dictionary and truthy(cmcs.at("seriesCountAxis", default: none)) {
      let resolved_axis = if cmcs.seriesCountAxis == "auto" { cont_axis } else { cmcs.seriesCountAxis }
      if resolved_axis == cont_axis {
        let sigma_per_series = if cont_axis == "x" { cmcs.x } else { cmcs.y }
        let base_dim = if cont_axis == "x" { subplot_width } else { subplot_height }
        let n_series = count_distinct_series(channel_semantics, table)
        let pressure = (n_series * sigma_per_series) / base_dim
        let elast = cmcs.at("elasticity", default: DEFAULT_GAS_PRESSURE_PARAMS.elasticity)
        let max_s = cmcs.at("maxStretch", default: DEFAULT_GAS_PRESSURE_PARAMS.maxStretch)
        if pressure > 1 {
          let stretch = calc.min(max_s, calc.pow(pressure, elast))
          if cont_axis == "x" {
            subplot_width = js_round(subplot_width * stretch)
          } else {
            subplot_height = js_round(subplot_height * stretch)
          }
        }
        series_stretch_applied = true
      }
    }

    if not series_stretch_applied and not other_axis_has_discrete {
      let cont_cs = _cs-of(channel_semantics, cont_axis)
      if cont_cs != none and truthy(cont_cs.at("field", default: none)) {
        let is_temporal = eff_type(cont_axis) == "temporal"
        let cont_values = ()
        for row in table {
          let v = row.at(cont_cs.field, default: none)
          if v == none { continue }
          let v2 = if is_temporal { _js_to_date_number(v) } else { _js_to_number(v) }
          if not _is_nan(v2) { cont_values.push(v2) }
        }
        let sigma1d = calc.sqrt(DEFAULT_GAS_PRESSURE_PARAMS.markCrossSection)
        let base_dim = if cont_axis == "x" { subplot_width } else { subplot_height }
        let pressure1d = (cont_values.len() * sigma1d) / base_dim
        if pressure1d > 1 {
          let stretch1d = calc.min(
            DEFAULT_GAS_PRESSURE_PARAMS.maxStretch,
            calc.pow(pressure1d, DEFAULT_GAS_PRESSURE_PARAMS.elasticity),
          )
          if cont_axis == "x" {
            subplot_width = js_round(subplot_width * stretch1d)
          } else {
            subplot_height = js_round(subplot_height * stretch1d)
          }
        }
      }
    }
  }

  // Elastic stretch for discrete axes
  let elastic_params_x = (
    elasticity: elasticity_val, maxStretch: max_stretch_x,
    defaultStepSize: default_step_size, minStep: min_step_val,
  )
  let elastic_params_y = (
    elasticity: elasticity_val, maxStretch: max_stretch_y,
    defaultStepSize: default_step_size, minStep: min_step_val,
  )
  let x_axis = compute_axis_step(
    x_total_nominal_count, x_continuous_as_discrete, subplot_width, elastic_params_x,
  )
  let y_axis = compute_axis_step(
    y_total_nominal_count, y_continuous_as_discrete, subplot_height, elastic_params_y,
  )

  let x_is_discrete = x_total_nominal_count > 0
  let y_is_discrete = y_total_nominal_count > 0
  let x_has_grouping = group_axis == "x" and nominal_count.group > 0
  let y_has_grouping = group_axis == "y" and nominal_count.group > 0

  let x_step_unit = none
  let y_step_unit = none
  let x_step_size = 0
  let y_step_size = 0

  if x_is_discrete and x_has_grouping {
    let items_per_group = nominal_count.group
    let default_group_step = items_per_group * default_step_size
    let min_group_step = calc.max(
      calc.ceil(MIN_GROUP_GAP_PX / step_padding_val), 2 * items_per_group,
    )
    let group_axis_step = compute_axis_step(nominal_count.x, 0, subplot_width, elastic_params_x)
    x_step_size = calc.max(min_group_step, calc.min(default_group_step, group_axis_step.step))
    x_step_unit = "group"
  } else if x_is_discrete or x_continuous_as_discrete > 0 {
    x_step_size = calc.max(min_step_val, calc.min(default_step_size, x_axis.step))
  } else {
    x_step_size = default_step_size
  }

  if y_is_discrete and y_has_grouping {
    let items_per_group = nominal_count.group
    let default_group_step = items_per_group * default_step_size
    let min_group_step = calc.max(
      calc.ceil(MIN_GROUP_GAP_PX / step_padding_val), 2 * items_per_group,
    )
    let group_axis_step = compute_axis_step(nominal_count.y, 0, subplot_height, elastic_params_y)
    y_step_size = calc.max(min_group_step, calc.min(default_group_step, group_axis_step.step))
    y_step_unit = "group"
  } else if y_is_discrete or y_continuous_as_discrete > 0 {
    y_step_size = calc.max(min_step_val, calc.min(default_step_size, y_axis.step))
  } else {
    y_step_size = default_step_size
  }

  // Banded continuous canvas size
  if x_continuous_as_discrete > 0 {
    subplot_width = js_round(x_step_size * (x_continuous_as_discrete + 1))
  }
  if y_continuous_as_discrete > 0 {
    subplot_height = js_round(y_step_size * (y_continuous_as_discrete + 1))
  }

  // Unified stretch budget
  let max_subplot_w = (default_chart_width * max_stretch_x - fix_w) / facet_cols - gap
  let max_subplot_h = (default_chart_height * max_stretch_y - fix_h) / facet_rows - gap

  if x_total_nominal_count > 0 {
    let divisor = if x_step_unit == "group" { nominal_count.x } else { x_total_nominal_count }
    let cap = calc.max(min_step_val, calc.floor(max_subplot_w / divisor))
    if x_step_size > cap { x_step_size = cap }
  }
  if x_continuous_as_discrete > 0 {
    let cap = calc.max(min_step_val, calc.floor(max_subplot_w / (x_continuous_as_discrete + 1)))
    if x_step_size > cap { x_step_size = cap }
  }
  if y_total_nominal_count > 0 {
    let divisor = if y_step_unit == "group" { nominal_count.y } else { y_total_nominal_count }
    let cap = calc.max(min_step_val, calc.floor(max_subplot_h / divisor))
    if y_step_size > cap { y_step_size = cap }
  }
  if y_continuous_as_discrete > 0 {
    let cap = calc.max(min_step_val, calc.floor(max_subplot_h / (y_continuous_as_discrete + 1)))
    if y_step_size > cap { y_step_size = cap }
  }

  if x_continuous_as_discrete > 0 {
    subplot_width = js_round(x_step_size * (x_continuous_as_discrete + 1))
  }
  if y_continuous_as_discrete > 0 {
    subplot_height = js_round(y_step_size * (y_continuous_as_discrete + 1))
  }

  // Clamp continuous subplot dimensions
  subplot_width = calc.min(subplot_width, js_round(max_subplot_w))
  subplot_height = calc.min(subplot_height, js_round(max_subplot_h))

  // Band AR blending
  let target_band_ar = options.at("targetBandAR", default: none)
  if truthy(target_band_ar) and target_band_ar > 0 {
    let x_is_banded_eff = x_total_nominal_count > 0 or x_continuous_as_discrete > 0
    let y_is_banded_eff = y_total_nominal_count > 0 or y_continuous_as_discrete > 0
    if x_is_banded_eff and not y_is_banded_eff {
      let actual_band_ar = subplot_height / x_step_size
      if actual_band_ar > target_band_ar {
        let ideal_h = x_step_size * target_band_ar
        let blended_h = calc.exp(0.5 * calc.ln(subplot_height) + 0.5 * calc.ln(ideal_h))
        subplot_height = js_round(calc.max(
          min_continuous_size_y, calc.min(blended_h, subplot_height),
        ))
      }
    } else if y_is_banded_eff and not x_is_banded_eff {
      let actual_band_ar = subplot_width / y_step_size
      if actual_band_ar > target_band_ar {
        let ideal_w = y_step_size * target_band_ar
        let blended_w = calc.exp(0.5 * calc.ln(subplot_width) + 0.5 * calc.ln(ideal_w))
        subplot_width = js_round(calc.max(
          min_continuous_size_x, calc.min(blended_w, subplot_width),
        ))
      }
    }
  }

  // Label sizing
  let x_has_discrete_items = x_total_nominal_count > 0
  let y_has_discrete_items = y_total_nominal_count > 0
  let x_label = compute_label_sizing(x_step_size, x_has_discrete_items)
  let y_label = compute_label_sizing(y_step_size, y_has_discrete_items)

  if x_has_discrete_items {
    let xf = _field-of(channel_semantics, "x")
    let xt = eff_type("x")
    let stats = _compute_discrete_label_stats(xf, table)
    if stats != none {
      // Numeric-like labels compete for the band's width when horizontal, so
      // the choice between horizontal and angled is whether the widest label
      // fits within one band.
      let numeric_like = xt == "quantitative" or stats.allNumeric
      let label_px = stats.maxLen * x_label.fontSize * APPROX_CHAR_WIDTH_RATIO
      let few_short_strings = (
        not numeric_like
          and stats.count <= VL_SHORT_DISCRETE_CATEGORY_COUNT
          and stats.maxLen <= VL_SHORT_DISCRETE_LABEL_MAX_LEN
      )
      if few_short_strings or (numeric_like and label_px <= x_step_size) {
        // Try widening the band within the stretch budget before committing to
        // horizontal; angle the labels only if even that will not fit.
        if label_px > x_step_size {
          let desired_step = calc.ceil(label_px) + 6
          let cap = calc.max(min_step_val, calc.floor(max_subplot_w / stats.count))
          if desired_step <= cap {
            x_step_size = calc.max(x_step_size, desired_step)
            x_label = compute_label_sizing(x_step_size, x_has_discrete_items)
            label_px = stats.maxLen * x_label.fontSize * APPROX_CHAR_WIDTH_RATIO
          }
        }

        if label_px <= x_step_size {
          // Must be explicit: omitting labelAngle leaves the backend's default.
          x_label = (: ..x_label, labelAngle: 0, labelAlign: "center", labelBaseline: "top")
        } else {
          x_label = (: ..x_label, labelAngle: -45, labelAlign: "right", labelBaseline: "top")
        }
      } else if (
        numeric_like and label_px > x_step_size
          and x_label.at("labelAngle", default: none) == none
      ) {
        x_label = (: ..x_label, labelAngle: -45, labelAlign: "right", labelBaseline: "top")
      }
    }
  }
  if y_has_discrete_items {
    let yf = _field-of(channel_semantics, "y")
    let yt = eff_type("y")
    if _discrete_y_axis_should_use_horizontal_labels(yf, yt, table) {
      y_label = (: ..y_label, labelAngle: 0, labelAlign: "right", labelBaseline: "middle")
    }
  }

  (
    subplotWidth: subplot_width,
    subplotHeight: subplot_height,
    xStep: x_step_size,
    yStep: y_step_size,
    xStepUnit: x_step_unit,
    yStepUnit: y_step_unit,
    xContinuousAsDiscrete: x_continuous_as_discrete,
    yContinuousAsDiscrete: y_continuous_as_discrete,
    xNominalCount: x_total_nominal_count,
    yNominalCount: y_total_nominal_count,
    xLabel: x_label,
    yLabel: y_label,
    stepPadding: step_padding_val,
    effectiveFacetGap: gap,
    truncations: (),
    facet: if facet_cols > 1 or facet_rows > 1 {
      (
        columns: facet_cols, rows: facet_rows,
        subplotWidth: subplot_width, subplotHeight: subplot_height,
      )
    } else { none },
  )
}
