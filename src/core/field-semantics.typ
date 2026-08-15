// Port of flint/core/field_semantics.py.

#import "semantic-types.typ": get_zero_class, infer_ordinal_sort_order, infer_vis_category
#import "type-registry.typ": get_registry_entry, is_registered
#import "py.typ": falsy, is-finite, is-float-string, is-nan, is_number, truthy

// ---------------------------------------------------------------------------
// Annotation normalization
// ---------------------------------------------------------------------------

// flint/core/field_semantics.py to_type_string
#let to_type_string(input_value) = {
  if falsy(input_value) { return "" }
  if type(input_value) == str { return input_value }
  if type(input_value) == dictionary {
    let st = input_value.at("semanticType", default: "")
    return if truthy(st) { st } else { "" }
  }
  ""
}

// flint/core/field_semantics.py normalize_annotation
#let normalize_annotation(input_value) = {
  if falsy(input_value) { return (semanticType: "Unknown") }
  if type(input_value) == str { return (semanticType: input_value) }
  if type(input_value) == dictionary {
    let st = input_value.at("semanticType", default: none)
    return (..input_value, semanticType: if truthy(st) { st } else { "Unknown" })
  }
  (semanticType: "Unknown")
}

// ---------------------------------------------------------------------------
// Format resolution
// ---------------------------------------------------------------------------

#let CURRENCY_MAP = (
  "USD": "$", "EUR": "€", "GBP": "£", "JPY": "¥", "CNY": "¥",
  "KRW": "₩", "INR": "₹", "BRL": "R$", "CAD": "CA$", "AUD": "A$",
  "CHF": "CHF", "SEK": "kr", "NOK": "kr", "DKK": "kr",
)

#let UNIT_SUFFIX_MAP = (
  "°C": "°C", "°F": "°F", "C": "°C", "F": "°F",
  "kg": " kg", "lb": " lb",
  "km": " km", "mi": " mi", "m": " m", "ft": " ft",
  "km/h": " km/h", "mph": " mph",
  "sec": " s", "min": " min", "hr": " hr",
  "seconds": " s", "minutes": " min", "hours": " hr",
  "%": "%",
)

// flint/core/field_semantics.py _is_number
#let _is_number(v) = {
  if type(v) == bool { return false }
  is_number(v) and not is-nan(v)
}

// flint/core/field_semantics.py _detect_percentage_representation
#let _detect_percentage_representation(values) = {
  if values.len() == 0 { return "0-100" }
  let abs_vals = values.map(calc.abs)
  let count_below_1 = abs_vals.filter(v => v <= 1).len()
  if count_below_1 / abs_vals.len() >= 0.8 { return "0-1" }
  "0-100"
}

// flint/core/field_semantics.py _detect_precision
//
// PORT-NUM: upstream renders each value with `"{:.10f}".format(...)` and then
// counts the digits left after stripping trailing zeros. Typst has no
// fixed-point formatting, so the same question is asked directly: what is the
// fewest decimal places that still round-trips the value at 10 dp? The two
// agree because stripping trailing zeros from a 10-dp rendering is exactly
// finding that minimum.
#let _decimals-at-10dp(v) = {
  let target = calc.round(v, digits: 10)
  let d = 0
  while d < 10 {
    if calc.round(v, digits: d) == target { return d }
    d += 1
  }
  10
}

#let _detect_precision(values) = {
  let max_decimals = 0
  for v in values {
    if not is-finite(v) { continue }
    let decimals = _decimals-at-10dp(float(v))
    if decimals > max_decimals { max_decimals = decimals }
  }
  calc.min(max_decimals, 4)
}

// flint/core/field_semantics.py _precision_format
#let _precision_format(values, use_grouping: true, sign_mode: "") = {
  let p = _detect_precision(values)
  let group = if use_grouping { "," } else { "" }
  if p == 0 { return sign_mode + group + "d" }
  sign_mode + group + "." + str(p) + "f"
}

// flint/core/field_semantics.py resolve_format
#let resolve_format(semantic_type, annotation, values) = {
  let entry = get_registry_entry(semantic_type)
  let unit = annotation.at("unit", default: none)

  let currency_prefix = none
  if truthy(unit) {
    let upper_hit = CURRENCY_MAP.at(upper(unit), default: none)
    currency_prefix = if truthy(upper_hit) { upper_hit } else { CURRENCY_MAP.at(unit, default: none) }
  }
  let unit_suffix = if truthy(unit) { UNIT_SUFFIX_MAP.at(unit, default: none) } else { none }

  let nums = values.filter(_is_number)
  let fmt_class = entry.formatClass

  if fmt_class == "currency" {
    if truthy(currency_prefix) {
      let axis_pattern = if semantic_type == "Price" { ",.2f" } else { _precision_format(nums) }
      return (
        format: (pattern: axis_pattern, prefix: currency_prefix),
        tooltipFormat: (pattern: ",.2f", prefix: currency_prefix),
      )
    }
    return (tooltipFormat: (pattern: ",.2f"))
  }

  if fmt_class == "percent" {
    if falsy(annotation.at("intrinsicDomain", default: none)) {
      return (tooltipFormat: (pattern: _precision_format(nums)))
    }
    let rep = _detect_percentage_representation(nums)
    if rep == "0-1" {
      let p = _detect_precision(nums)
      let axis_p = calc.max(0, p - 2)
      let tip_p = calc.min(axis_p + 1, 4)
      return (
        format: (pattern: "." + str(axis_p) + "~%"),
        tooltipFormat: (pattern: "." + str(tip_p) + "%"),
      )
    }
    return (tooltipFormat: (pattern: _precision_format(nums, use_grouping: false), suffix: "%"))
  }

  if fmt_class == "unit-suffix" {
    if truthy(unit_suffix) {
      return (tooltipFormat: (pattern: _precision_format(nums), suffix: unit_suffix))
    }
    return (tooltipFormat: (pattern: _precision_format(nums)))
  }

  if fmt_class == "integer" {
    if semantic_type == "Year" or semantic_type == "Decade" { return (:) }
    return (tooltipFormat: (pattern: ",d"))
  }

  if fmt_class == "decimal" {
    return (tooltipFormat: (pattern: _precision_format(nums)))
  }

  (:)
}

// ---------------------------------------------------------------------------
// Default vis type
// ---------------------------------------------------------------------------

// flint/core/field_semantics.py _try_float
//
// PORT-EXC: `try: float(s)` -> `is-float-string`, see src/core/py.typ.
#let _try_float(s) = is-float-string(s)

// flint/core/field_semantics.py resolve_default_vis_type
#let resolve_default_vis_type(semantic_type, values) = {
  if not is_registered(semantic_type) { return infer_vis_category(values) }

  let entry = get_registry_entry(semantic_type)
  let candidates = entry.visEncodings
  if candidates.len() == 1 {
    if candidates.at(0) == "quantitative" {
      let non_null = values.filter(v => v != none)
      // Note the precedence upstream relies on: `A and B or C` is
      // `(A and B) or C`, so a numeric value passes on the left and a numeric
      // *string* on the right.
      let all_numeric = non_null.len() > 0 and non_null.all(v => (
        (is_number(v) and type(v) != bool)
          or (type(v) == str and v.trim() != "" and _try_float(v))
      ))
      if not all_numeric { return infer_vis_category(values) }
    }
    return candidates.at(0)
  }

  if "quantitative" in candidates and "ordinal" in candidates {
    let distinct = values.filter(v => v != none).dedup().len()
    return if distinct <= 12 { "ordinal" } else { "quantitative" }
  }

  if "temporal" in candidates and "ordinal" in candidates {
    let distinct = values.filter(v => v != none).dedup().len()
    return if distinct <= 6 { "ordinal" } else { "temporal" }
  }

  if "geographic" in candidates and "quantitative" in candidates {
    return "quantitative"
  }

  candidates.at(0)
}

// ---------------------------------------------------------------------------
// Aggregation
// ---------------------------------------------------------------------------

// flint/core/field_semantics.py resolve_aggregation_default
#let resolve_aggregation_default(semantic_type) = {
  let role = get_registry_entry(semantic_type).aggRole
  if role == "additive" or role == "signed-additive" { return "sum" }
  if role == "intensive" { return "average" }
  none
}

// ---------------------------------------------------------------------------
// Zero baseline
// ---------------------------------------------------------------------------

// flint/core/field_semantics.py resolve_zero_class_from_annotation
#let resolve_zero_class_from_annotation(semantic_type, domain: none) = {
  if truthy(domain) and domain.at(0) > 0 { return "arbitrary" }
  get_zero_class(semantic_type)
}

// ---------------------------------------------------------------------------
// Scale type
// ---------------------------------------------------------------------------

// flint/core/field_semantics.py resolve_scale_type
#let resolve_scale_type(semantic_type, values) = {
  let entry = get_registry_entry(semantic_type)
  let eligible = (
    entry.aggRole == "additive" and entry.domainShape == "open" and entry.t1 != "GenericMeasure"
  )
  if not eligible { return none }
  if values.len() < 10 { return none }
  let filtered = values.filter(is-finite)
  if filtered.len() < 10 { return none }
  let mn = calc.min(..filtered)
  let mx = calc.max(..filtered)
  if mx <= 0 or mn == mx { return none }
  if mn < 0 { return none }
  let positives = filtered.filter(v => v > 0)
  if positives.len() > 0 {
    let positive_min = calc.min(..positives)
    if mx / positive_min >= 1000000 {
      let has_zeros = filtered.any(v => v == 0)
      return if has_zeros { "symlog" } else { "log" }
    }
  }
  none
}

// ---------------------------------------------------------------------------
// Domain constraints
// ---------------------------------------------------------------------------

// flint/core/field_semantics.py _merge_intrinsic_with_data
#let _merge_intrinsic_with_data(intrinsic, values, hard) = {
  if hard { return (min: intrinsic.at(0), max: intrinsic.at(1), clamp: true) }
  let nums = values.filter(_is_number)
  if nums.len() == 0 { return (min: intrinsic.at(0), max: intrinsic.at(1), clamp: false) }
  let data_min = calc.min(..nums)
  let data_max = calc.max(..nums)
  (
    min: calc.min(intrinsic.at(0), data_min),
    max: calc.max(intrinsic.at(1), data_max),
    clamp: false,
  )
}

// flint/core/field_semantics.py snap_to_bound_heuristic
#let snap_to_bound_heuristic(intrinsic, values) = {
  let nums = values.filter(_is_number)
  if nums.len() == 0 { return none }
  let lo = intrinsic.at(0)
  let hi = intrinsic.at(1)
  let rng = hi - lo
  if rng <= 0 { return none }
  let data_min = calc.min(..nums)
  let data_max = calc.max(..nums)

  let zero_inside = lo < 0 and hi > 0
  let threshold_lo = 0.25 * (if zero_inside { 0 - lo } else { rng })
  let threshold_hi = 0.25 * (if zero_inside { hi } else { rng })

  let snap_min = none
  let snap_max = none

  if data_min >= lo and data_min <= lo + threshold_lo { snap_min = lo }
  if data_max <= hi and data_max >= hi - threshold_hi { snap_max = hi }

  if snap_min == none and snap_max == none { return none }
  let out = (clamp: false)
  if snap_min != none { out.insert("min", snap_min) }
  if snap_max != none { out.insert("max", snap_max) }
  out
}

// flint/core/field_semantics.py resolve_domain_constraint
#let resolve_domain_constraint(semantic_type, annotation, values) = {
  let entry = get_registry_entry(semantic_type)
  let intrinsic = annotation.at("intrinsicDomain", default: none)

  if truthy(intrinsic) {
    if entry.t1 == "Proportion" or entry.t1 == "SignedMeasure" {
      return snap_to_bound_heuristic(intrinsic, values)
    }
    return _merge_intrinsic_with_data(intrinsic, values, false)
  }

  if semantic_type == "Latitude" { return _merge_intrinsic_with_data((-90, 90), values, true) }
  if semantic_type == "Longitude" { return _merge_intrinsic_with_data((-180, 180), values, true) }
  if semantic_type == "Correlation" { return _merge_intrinsic_with_data((-1, 1), values, true) }

  if semantic_type == "Percentage" {
    let nums = values.filter(_is_number)
    if nums.len() > 0 {
      let rep = _detect_percentage_representation(nums)
      let M = if rep == "0-1" { 1 } else { 100 }
      return snap_to_bound_heuristic((0, M), values)
    }
  }

  none
}

// ---------------------------------------------------------------------------
// Tick constraints
// ---------------------------------------------------------------------------

// flint/core/field_semantics.py resolve_tick_constraint
#let resolve_tick_constraint(semantic_type, domain: none) = {
  let entry = get_registry_entry(semantic_type)

  if entry.formatClass == "integer" {
    let tc = (integersOnly: true, minStep: 1)
    if truthy(domain) {
      let span = domain.at(1) - domain.at(0)
      if span > 0 and span <= 20 {
        tc.insert("exactTicks", range(int(domain.at(0)), int(domain.at(1)) + 1))
      }
    }
    return tc
  }

  if semantic_type == "Score" and truthy(domain) {
    let span = domain.at(1) - domain.at(0)
    if span >= 2 {
      let tc = (integersOnly: true, minStep: 1)
      if span <= 20 {
        tc.insert("exactTicks", range(int(domain.at(0)), int(domain.at(1)) + 1))
      }
      return tc
    }
  }

  none
}

// ---------------------------------------------------------------------------
// Canonical ordering
// ---------------------------------------------------------------------------

// flint/core/field_semantics.py resolve_canonical_order
#let resolve_canonical_order(semantic_type, annotation, values) = {
  let sort_order = annotation.at("sortOrder", default: none)
  if truthy(sort_order) and sort_order.len() > 0 { return sort_order }
  infer_ordinal_sort_order(semantic_type, values)
}

// flint/core/field_semantics.py resolve_cyclic
#let resolve_cyclic(semantic_type) = get_registry_entry(semantic_type).domainShape == "cyclic"

// ---------------------------------------------------------------------------
// Reversed axis
// ---------------------------------------------------------------------------

// flint/core/field_semantics.py resolve_reversed
#let resolve_reversed(semantic_type, channel: none) = {
  if semantic_type == "Rank" { return channel != "x" }
  false
}

// ---------------------------------------------------------------------------
// Nice
// ---------------------------------------------------------------------------

// flint/core/field_semantics.py resolve_nice
#let resolve_nice(semantic_type, domain_constraint: none) = {
  if truthy(domain_constraint) and truthy(domain_constraint.at("clamp", default: none)) {
    return false
  }
  if (
    truthy(domain_constraint)
      and domain_constraint.at("min", default: none) != none
      and domain_constraint.at("max", default: none) != none
  ) {
    return false
  }
  if get_registry_entry(semantic_type).domainShape == "fixed" { return false }
  true
}

// ---------------------------------------------------------------------------
// Diverging info + color scheme hint
// ---------------------------------------------------------------------------

#let _UNIT_MIDPOINTS = ("°C": 0, "°F": 32, "K": 273.15, "C": 0, "F": 32)

// flint/core/field_semantics.py resolve_diverging_info
#let resolve_diverging_info(semantic_type, annotation, values) = {
  let entry = get_registry_entry(semantic_type)
  let unit = annotation.at("unit", default: none)

  if semantic_type == "Temperature" and truthy(unit) {
    let mid = _UNIT_MIDPOINTS.at(unit, default: none)
    if mid != none { return (midpoint: mid, inherent: false, source: "unit") }
  }

  if entry.diverging == "inherent" {
    return (midpoint: 0, inherent: true, source: "type-intrinsic")
  }
  if entry.diverging == "conditional" {
    return (midpoint: 0, inherent: false, source: "type-intrinsic")
  }

  let intrinsic = annotation.at("intrinsicDomain", default: none)
  if truthy(intrinsic) {
    return (midpoint: (intrinsic.at(0) + intrinsic.at(1)) / 2, inherent: false, source: "domain")
  }

  if values.len() > 0 {
    let mn = calc.min(..values)
    let mx = calc.max(..values)
    if mn < 0 and mx > 0 { return (midpoint: 0, inherent: false, source: "data") }
  }

  none
}

// flint/core/field_semantics.py resolve_color_scheme_hint
#let resolve_color_scheme_hint(semantic_type, annotation, values) = {
  let entry = get_registry_entry(semantic_type)
  let nums = values.filter(_is_number)

  let div_info = resolve_diverging_info(semantic_type, annotation, nums)
  if truthy(div_info) {
    let mn = if nums.len() > 0 { calc.min(..nums) } else { 0 }
    let mx = if nums.len() > 0 { calc.max(..nums) } else { 0 }
    let spans_both = mn < div_info.midpoint and mx > div_info.midpoint
    if div_info.inherent or spans_both {
      return (
        type: "diverging",
        divergingMidpoint: div_info.midpoint,
        inherentlyDiverging: div_info.inherent,
      )
    }
  }

  if "quantitative" in entry.visEncodings { return (type: "sequential") }
  (type: "categorical")
}

// ---------------------------------------------------------------------------
// Binning
// ---------------------------------------------------------------------------

// flint/core/field_semantics.py resolve_binning_suggested
#let resolve_binning_suggested(semantic_type, domain: none) = {
  let entry = get_registry_entry(semantic_type)
  if "quantitative" not in entry.visEncodings { return false }
  if entry.aggRole == "identifier" or entry.aggRole == "dimension" { return false }
  if semantic_type == "Year" or semantic_type == "Decade" { return false }
  if truthy(domain) and (domain.at(1) - domain.at(0)) <= 20 { return false }
  if semantic_type == "Score" and falsy(domain) { return false }
  true
}

// ---------------------------------------------------------------------------
// Stacking
// ---------------------------------------------------------------------------

// flint/core/field_semantics.py resolve_stackable
//
// Returns `"sum"`, `"normalize"` or `false` — upstream is untyped here and the
// mixed return is load-bearing at the call site.
#let resolve_stackable(semantic_type) = {
  let role = get_registry_entry(semantic_type).aggRole
  if role == "additive" or role == "signed-additive" { return "sum" }
  if role == "intensive" {
    if semantic_type == "Percentage" { return "normalize" }
    return false
  }
  false
}

// ---------------------------------------------------------------------------
// Sort direction
// ---------------------------------------------------------------------------

// flint/core/field_semantics.py resolve_sort_direction
#let resolve_sort_direction(semantic_type) = {
  if semantic_type == "Rank" { return "descending" }
  "ascending"
}

// ---------------------------------------------------------------------------
// Builder
// ---------------------------------------------------------------------------

// flint/core/field_semantics.py resolve_field_semantics
#let resolve_field_semantics(input_value, field_name, values) = {
  let annotation = normalize_annotation(input_value)
  let semantic_type = annotation.semanticType

  let numeric_values = values.filter(v => type(v) != bool and is-finite(v))

  let default_vis_type = resolve_default_vis_type(semantic_type, values)
  let fmt_result = resolve_format(semantic_type, annotation, values)
  let aggregation_default = resolve_aggregation_default(semantic_type)
  let zero_class = resolve_zero_class_from_annotation(
    semantic_type, domain: annotation.at("intrinsicDomain", default: none),
  )
  let scale_type = resolve_scale_type(semantic_type, numeric_values)
  let domain_constraint = resolve_domain_constraint(semantic_type, annotation, values)
  let canonical_order = resolve_canonical_order(semantic_type, annotation, values)
  let cyclic = resolve_cyclic(semantic_type)
  let binning_suggested = resolve_binning_suggested(
    semantic_type, domain: annotation.at("intrinsicDomain", default: none),
  )
  let sort_direction = resolve_sort_direction(semantic_type)

  if not is_registered(semantic_type) and default_vis_type == "quantitative" {
    if falsy(aggregation_default) { aggregation_default = "sum" }
    if zero_class == "unknown" { zero_class = "meaningful" }
    binning_suggested = true
  }

  let out = (
    semanticAnnotation: annotation,
    defaultVisType: default_vis_type,
    aggregationDefault: aggregation_default,
    zeroClass: zero_class,
    scaleType: scale_type,
    domainConstraint: domain_constraint,
    canonicalOrder: canonical_order,
    cyclic: cyclic,
    sortDirection: sort_direction,
    binningSuggested: binning_suggested,
  )
  if "format" in fmt_result { out.insert("format", fmt_result.format) }
  if "tooltipFormat" in fmt_result { out.insert("tooltipFormat", fmt_result.tooltipFormat) }
  out
}
