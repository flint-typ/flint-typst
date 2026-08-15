// Port of flint/core/decisions.py.
//
// Two unrelated halves sharing a file upstream: deciding a channel's encoding
// type from its semantic type and data, and the "gas pressure" sizing model
// that turns item counts into axis stretch.

#import "semantic-types.typ": infer_vis_category
#import "type-registry.typ": get_registry_entry, is_registered
#import "js-round.typ": js_round
#import "js-date.typ": is_js_parseable
#import "py.typ": falsy, is-float-string, is-nan, is_number, py_str, truthy

// ---------------------------------------------------------------------------
// Encoding type
// ---------------------------------------------------------------------------

// flint/core/decisions.py _vis_category_to_vl_type
#let _vis_category_to_vl_type(vc) = {
  if vc == "quantitative" { return "quantitative" }
  if vc == "ordinal" { return "ordinal" }
  if vc == "temporal" { return "temporal" }
  if vc == "geographic" { return "quantitative" }
  "nominal"
}

// flint/core/decisions.py _looks_temporal_value
#let _looks_temporal_value(val) = {
  if type(val) == bool { return false }
  if is_number(val) {
    // PORT-NUM: Typst *errors* on any ordering comparison involving NaN;
    // Python returns False. Both `1500 <= nan` chains below would therefore
    // abort the document, so NaN is rejected up front — which is the answer
    // Python arrives at anyway.
    if is-nan(val) { return false }
    if val >= 1500 and val <= 2200 and val == calc.floor(val) { return true }
    if val > 86400000 and val < 4200000000000 { return true }
    return false
  }
  if type(val) == str {
    let trimmed = val.trim()
    if trimmed == "" { return false }
    // Mirrors V8 `!Number.isNaN(Date.parse(trimmed))` — accepts any free-form
    // date string V8 parses (including "FY 2018") and rejects what it does not
    // (e.g. "15.01.2020").
    return is_js_parseable(trimmed)
  }
  false
}

// flint/core/decisions.py _validate_temporal_parsing
#let _validate_temporal_parsing(data, field_name, from_registry) = {
  let head = data.slice(0, calc.min(15, data.len()))
  let sample = head.map(r => r.at(field_name, default: none)).filter(v => v != none)
  if sample.len() == 0 { return false }
  let unique = sample.map(py_str).dedup()
  if unique.len() <= 1 { return false }
  let passing = sample.filter(_looks_temporal_value).len()
  let threshold = if from_registry { 0.3 } else { 0.5 }
  passing / sample.len() >= threshold
}

// flint/core/decisions.py _resolve_temporal_encoding
#let _resolve_temporal_encoding(vis_category, channel, data, field_name, from_registry) = {
  if channel == "size" or channel == "column" or channel == "row" {
    return (
      vlType: "ordinal", visCategory: vis_category,
      channelOverride: true, cardinalityGuard: false,
    )
  }
  if channel == "color" {
    let unique_count = data.map(r => r.at(field_name, default: none)).dedup().len()
    if unique_count <= 12 {
      return (
        vlType: "ordinal", visCategory: vis_category,
        channelOverride: true, cardinalityGuard: false,
      )
    }
  }
  if not _validate_temporal_parsing(data, field_name, from_registry) {
    return (
      vlType: "ordinal", visCategory: vis_category,
      channelOverride: false, cardinalityGuard: false,
    )
  }
  (
    vlType: "temporal", visCategory: vis_category,
    channelOverride: false, cardinalityGuard: false,
  )
}

// flint/core/decisions.py _apply_ordinal_guards
#let _apply_ordinal_guards(vis_category, channel, data, field_name, field_values, from_registry) = {
  let numeric_vals = ()
  for v in field_values {
    if v == none { continue }
    if type(v) == bool { continue }
    if is_number(v) {
      if not is-nan(v) { numeric_vals.push(float(v)) }
    } else if type(v) == str {
      // PORT-EXC: `try: float(v) / except ValueError: pass`.
      if is-float-string(v) { numeric_vals.push(float(v)) }
    }
  }

  if numeric_vals.len() > 0 {
    let unique_count = numeric_vals.dedup().len()
    // `v % 1 != 0` — Python and Typst disagree on the *sign* of the remainder
    // for negative operands, but not on whether it is zero, which is all this
    // asks.
    let has_fractions = numeric_vals.any(v => calc.rem(v, 1) != 0)

    if not from_registry and has_fractions and unique_count > 20 {
      return (
        vlType: "quantitative", visCategory: vis_category,
        channelOverride: false, cardinalityGuard: true,
      )
    }
    if not has_fractions and unique_count > 12 and (channel == "color" or channel == "group") {
      return (
        vlType: "quantitative", visCategory: vis_category,
        channelOverride: true, cardinalityGuard: true,
      )
    }
    if not has_fractions and unique_count > 12 and (channel == "x" or channel == "y") {
      return (
        vlType: "quantitative", visCategory: vis_category,
        channelOverride: true, cardinalityGuard: true,
      )
    }
  }
  (
    vlType: "ordinal", visCategory: vis_category,
    channelOverride: false, cardinalityGuard: false,
  )
}

// flint/core/decisions.py _disambiguate_multi_encoding
#let _disambiguate_multi_encoding(candidates, channel, data, field_name, field_values) = {
  let has(vc) = vc in candidates

  if has("temporal") and has("ordinal") {
    return _resolve_temporal_encoding("temporal", channel, data, field_name, true)
  }

  if has("quantitative") and has("ordinal") {
    if channel == "color" or channel == "group" {
      let unique_count = data.map(r => r.at(field_name, default: none)).dedup().len()
      if unique_count <= 12 {
        return (vlType: "ordinal", visCategory: "ordinal", channelOverride: false, cardinalityGuard: false)
      }
      return (vlType: "quantitative", visCategory: "quantitative", channelOverride: false, cardinalityGuard: true)
    }
    if channel == "column" or channel == "row" {
      return (vlType: "ordinal", visCategory: "ordinal", channelOverride: false, cardinalityGuard: false)
    }
    return (vlType: "quantitative", visCategory: "quantitative", channelOverride: false, cardinalityGuard: false)
  }

  if has("quantitative") and has("geographic") {
    return (vlType: "quantitative", visCategory: "quantitative", channelOverride: false, cardinalityGuard: false)
  }

  if has("ordinal") and has("nominal") {
    if channel == "color" or channel == "group" {
      return (vlType: "nominal", visCategory: "nominal", channelOverride: false, cardinalityGuard: false)
    }
    return (vlType: "ordinal", visCategory: "ordinal", channelOverride: false, cardinalityGuard: false)
  }

  let fallback = candidates.at(0)
  (
    vlType: _vis_category_to_vl_type(fallback), visCategory: fallback,
    channelOverride: false, cardinalityGuard: false,
  )
}

// flint/core/decisions.py _can_parse_float
//
// PORT-EXC: `try: float(s) / except ValueError`. Note upstream catches only
// ValueError here, so a non-string would raise TypeError — it is never called
// with one.
#let _can_parse_float(s) = is-float-string(s)

// flint/core/decisions.py resolve_encoding_type
#let resolve_encoding_type(semantic_type, field_values, channel, data, field_name) = {
  if truthy(semantic_type) and is_registered(semantic_type) {
    let entry = get_registry_entry(semantic_type)
    let candidates = entry.visEncodings
    if candidates.len() > 1 {
      return _disambiguate_multi_encoding(candidates, channel, data, field_name, field_values)
    }
    let base_type = candidates.at(0)
    if base_type == "quantitative" {
      let non_null = field_values.filter(v => v != none)
      let all_numeric = non_null.len() > 0 and non_null.all(v => (
        (is_number(v) and type(v) != bool)
          or (type(v) == str and v.trim() != "" and _can_parse_float(v))
      ))
      if not all_numeric {
        let inferred = infer_vis_category(field_values)
        return (
          vlType: _vis_category_to_vl_type(inferred), visCategory: inferred,
          channelOverride: false, cardinalityGuard: false,
        )
      }
    }
    if base_type == "temporal" {
      return _resolve_temporal_encoding(base_type, channel, data, field_name, true)
    }
    if base_type == "ordinal" {
      return _apply_ordinal_guards(base_type, channel, data, field_name, field_values, true)
    }
    return (
      vlType: _vis_category_to_vl_type(base_type), visCategory: base_type,
      channelOverride: false, cardinalityGuard: false,
    )
  }

  let vc = infer_vis_category(field_values)
  if vc == "temporal" { return _resolve_temporal_encoding(vc, channel, data, field_name, false) }
  if vc == "ordinal" { return _apply_ordinal_guards(vc, channel, data, field_name, field_values, false) }
  if vc == "quantitative" {
    return (vlType: "quantitative", visCategory: vc, channelOverride: false, cardinalityGuard: false)
  }
  if vc == "geographic" {
    return (vlType: "quantitative", visCategory: vc, channelOverride: false, cardinalityGuard: false)
  }
  (vlType: "nominal", visCategory: vc, channelOverride: false, cardinalityGuard: false)
}

// ---------------------------------------------------------------------------
// Gas pressure model
// ---------------------------------------------------------------------------

#let DEFAULT_GAS_PRESSURE_PARAMS = (
  markCrossSection: 30,
  elasticity: 0.3,
  maxStretch: 1.5,
)

// flint/core/decisions.py compute_gas_pressure
//
// Treats marks as particles: the more distinct pixel positions a column wants,
// the more the axis stretches, with a bounded elastic response.
#let compute_gas_pressure(
  x_values, y_values, x_domain, y_domain, canvas_width, canvas_height, params: none,
) = {
  // PORT-IDIOM: `{**A, **B}` is `(: ..A, ..B)`. The leading `:` is required —
  // a parenthesised list of only spreads parses as an *array* otherwise.
  let p = (: ..DEFAULT_GAS_PRESSURE_PARAMS, ..(if truthy(params) { params } else { (:) }))
  let N = x_values.len()
  if N <= 1 or canvas_width <= 0 or canvas_height <= 0 {
    return (stretchX: 1, stretchY: 1, rawStretchX: 1, rawStretchY: 1)
  }

  let sigma1d_default = calc.sqrt(p.markCrossSection)

  // Nested in upstream too; the capture is read-only, which Typst allows.
  let compute_axis_stretch(values, domain, base_dim, sigma1d) = {
    if base_dim <= 0 or values.len() <= 1 { return (1.0, 1.0) }
    let rng = domain.at(1) - domain.at(0)
    if rng <= 0 { return (1.0, 1.0) }
    let px_per_unit = base_dim / rng
    let unique_positions = values.map(v => js_round((v - domain.at(0)) * px_per_unit)).dedup().len()
    let pressure = (unique_positions * sigma1d) / base_dim
    if pressure <= 1 { return (1.0, 1.0) }
    let raw = calc.pow(pressure, p.elasticity)
    (calc.min(p.maxStretch, raw), raw)
  }

  let mcs_x = p.at("markCrossSectionX", default: none)
  let mcs_y = p.at("markCrossSectionY", default: none)
  let sigma1d_x = if mcs_x != none { calc.sqrt(mcs_x) } else { sigma1d_default }
  let sigma1d_y = if mcs_y != none { calc.sqrt(mcs_y) } else { sigma1d_default }

  let compute_stretch_for_axis(values, domain, base_dim, sigma1d, sigma_raw, item_count_override) = {
    if item_count_override != none and sigma_raw > 0 {
      let pressure = (item_count_override * sigma_raw) / base_dim
      if pressure <= 1 { return (1.0, 1.0) }
      let raw = calc.pow(pressure, p.elasticity)
      return (calc.min(p.maxStretch, raw), raw)
    }
    if sigma1d > 0 { return compute_axis_stretch(values, domain, base_dim, sigma1d) }
    (1.0, 1.0)
  }

  let sigma_raw_x = if mcs_x != none { mcs_x } else { p.markCrossSection }
  let sigma_raw_y = if mcs_y != none { mcs_y } else { p.markCrossSection }

  let (stretch_x, raw_x) = compute_stretch_for_axis(
    x_values, x_domain, canvas_width, sigma1d_x, sigma_raw_x,
    p.at("xItemCountOverride", default: none),
  )
  let (stretch_y, raw_y) = compute_stretch_for_axis(
    y_values, y_domain, canvas_height, sigma1d_y, sigma_raw_y,
    p.at("yItemCountOverride", default: none),
  )

  (stretchX: stretch_x, stretchY: stretch_y, rawStretchX: raw_x, rawStretchY: raw_y)
}

// ---------------------------------------------------------------------------
// Elastic budget
// ---------------------------------------------------------------------------

// flint/core/decisions.py compute_elastic_budget
#let compute_elastic_budget(item_count, base_dimension, params) = {
  if item_count <= 0 { return (budget: base_dimension, stretchFactor: 1.0) }
  let pressure = (item_count * params.defaultStepSize) / base_dimension
  if pressure <= 1 { return (budget: base_dimension, stretchFactor: 1.0) }
  let stretch_factor = calc.min(params.maxStretch, calc.pow(pressure, params.elasticity))
  (budget: base_dimension * stretch_factor, stretchFactor: stretch_factor)
}

// flint/core/decisions.py compute_axis_step
#let compute_axis_step(nominal_count, continuous_count, base_dimension, params) = {
  if nominal_count > 0 {
    let b = compute_elastic_budget(nominal_count, base_dimension, params)
    return (step: calc.floor(b.budget / nominal_count), budget: b.budget, itemCount: nominal_count)
  }
  if continuous_count > 0 {
    let b = compute_elastic_budget(continuous_count, base_dimension, params)
    return (step: calc.floor(b.budget / continuous_count), budget: b.budget, itemCount: continuous_count)
  }
  (step: params.defaultStepSize, budget: base_dimension, itemCount: 0)
}

// ---------------------------------------------------------------------------
// Facet layout
// ---------------------------------------------------------------------------

// flint/core/decisions.py compute_facet_layout
#let compute_facet_layout(facet_cols, facet_rows, base_width, base_height, params) = {
  let min_continuous = params.minSubplotSize
  let subplot_width = if facet_cols > 1 {
    let stretch = calc.min(params.maxStretch, calc.pow(facet_cols, params.facetElasticity))
    js_round(calc.max(min_continuous, base_width * stretch / facet_cols))
  } else {
    base_width
  }
  let subplot_height = if facet_rows > 1 {
    let stretch = calc.min(params.maxStretch, calc.pow(facet_rows, params.facetElasticity))
    js_round(calc.max(min_continuous, base_height * stretch / facet_rows))
  } else {
    base_height
  }
  (columns: facet_cols, rows: facet_rows, subplotWidth: subplot_width, subplotHeight: subplot_height)
}

// ---------------------------------------------------------------------------
// Label sizing
// ---------------------------------------------------------------------------

// flint/core/decisions.py compute_label_sizing
#let compute_label_sizing(effective_step, has_discrete_items) = {
  let default_font_size = 10
  let default_limit = 100
  if not has_discrete_items {
    return (fontSize: default_font_size, labelLimit: default_limit)
  }

  let font_size = calc.max(6, calc.min(10, effective_step - 1))
  let label_limit = calc.max(30, calc.min(100, effective_step * 8))
  let label_angle = none
  let label_align = none
  let label_baseline = none

  if effective_step < 10 {
    label_angle = -90
    font_size = calc.max(6, calc.min(8, effective_step))
    label_limit = 40
    label_align = "right"
    label_baseline = "middle"
  } else if effective_step < 16 {
    label_angle = -45
    font_size = calc.max(7, calc.min(9, effective_step))
    label_limit = 60
    label_align = "right"
    label_baseline = "top"
  }

  let out = (fontSize: font_size, labelLimit: label_limit)
  if label_angle != none { out.insert("labelAngle", label_angle) }
  if label_align != none { out.insert("labelAlign", label_align) }
  if label_baseline != none { out.insert("labelBaseline", label_baseline) }
  out
}

// ---------------------------------------------------------------------------
// Overflow
// ---------------------------------------------------------------------------

// flint/core/decisions.py compute_overflow
#let compute_overflow(unique_count, max_dimension, min_step_size) = {
  let max_to_keep = calc.floor(max_dimension / min_step_size)
  let overflowed = unique_count > max_to_keep
  (
    overflowed: overflowed,
    maxToKeep: max_to_keep,
    omittedCount: if overflowed { unique_count - max_to_keep } else { 0 },
  )
}

// ---------------------------------------------------------------------------
// Circumference pressure (radial charts)
// ---------------------------------------------------------------------------

// flint/core/decisions.py compute_circumference_pressure
#let compute_circumference_pressure(effective_item_count, canvas_size, params: none) = {
  let p = if truthy(params) { params } else { (:) }
  let min_arc_px = p.at("minArcPx", default: 45)
  let min_radius = p.at("minRadius", default: 60)
  let max_radius = p.at("maxRadius", default: 400)
  let elasticity = p.at("elasticity", default: 0.5)
  let max_stretch = p.at("maxStretch", default: 2.0)
  let msx = p.at("maxStretchX", default: none)
  let msy = p.at("maxStretchY", default: none)
  let max_stretch_x = calc.max(1, if msx == none { max_stretch } else { msx })
  let max_stretch_y = calc.max(1, if msy == none { max_stretch } else { msy })
  let margin = p.at("margin", default: 20)

  let base_w = canvas_size.width
  let base_h = canvas_size.height

  let base_radius = calc.max(min_radius, (calc.min(base_w, base_h) / 2) - margin)
  let max_canvas_w = base_w * max_stretch_x
  let max_canvas_h = base_h * max_stretch_y
  let max_diameter = calc.min(max_canvas_w, max_canvas_h)
  let effective_max_radius = calc.min(max_radius, (max_diameter - 2 * margin) / 2)
  let effective_max_stretch = calc.max(1, effective_max_radius / base_radius)

  let base_circumference = 2 * calc.pi * base_radius
  let pressure = (effective_item_count * min_arc_px) / base_circumference

  let radius = if pressure <= 1 {
    base_radius
  } else {
    let stretch = calc.min(effective_max_stretch, calc.pow(pressure, elasticity))
    js_round(base_radius * stretch)
  }

  radius = calc.min(max_radius, calc.max(min_radius, radius))
  let diameter = 2 * radius + 2 * margin
  (
    radius: radius,
    canvasW: calc.max(base_w, diameter),
    canvasH: calc.max(base_h, diameter),
  )
}

// flint/core/decisions.py compute_effective_bar_count
#let compute_effective_bar_count(values) = {
  if values.len() == 0 { return 0 }
  // Same NaN guard as `_looks_temporal_value`: `nan > 0` errors in Typst and
  // is False in Python, so NaN is filtered out first.
  let positive_values = values.filter(v => not is-nan(v) and v > 0)
  if positive_values.len() == 0 { return values.len() }
  let total = positive_values.sum()
  let min_val = calc.min(..positive_values)
  calc.min(100, total / min_val)
}
