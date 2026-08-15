// Port of flint/core/resolve_semantics.py.
//
// Ported in two halves, because the corpus records inputs *and* outputs per
// stage and so decouples them: the temporal half below reaches the
// `convert_temporal_data` gate without `decisions` existing yet.
// `resolve_channel_semantics` follows once `decisions` lands.

#import "field-semantics.typ": to_type_string
#import "semantic-types.typ": get_vis_category, infer_vis_category
#import "js-date.typ": js_date_parse
#import "py.typ": falsy, is-float-string, is-nan, is_number, num-str, py_str, truthy
#import "@local/datehog:0.1.0" as dh

#let MAX_TIMESTAMP_SEC = 4102444800
#let MAX_TIMESTAMP_MS = 4102444800000

// flint/core/resolve_semantics.py is_likely_timestamp
#let is_likely_timestamp(val) = {
  if val >= 1e9 and val <= MAX_TIMESTAMP_SEC { return true }
  if val > MAX_TIMESTAMP_SEC and val <= MAX_TIMESTAMP_MS { return true }
  false
}

// flint/core/resolve_semantics.py timestamp_to_ms
#let timestamp_to_ms(val) = if val <= MAX_TIMESTAMP_SEC { val * 1000 } else { val }

#let _CAMEL_RE = regex("([a-z0-9])([A-Z])")
#let _NON_ALNUM_RE = regex("[^a-z0-9]+")

// flint/core/resolve_semantics.py infer_implicit_semantic_type
//
// A field called "Year" / "fiscalYear" whose values are all plausible
// four-digit years gets typed as `Year` even without an annotation.
#let infer_implicit_semantic_type(field_name, values) = {
  // PORT-IDIOM: `re.sub(p, r"\1 \2", s)` -> a replacement function reading
  // `m.captures`, since Typst's regex engine has no backreferences.
  let tokenized = lower(field_name.replace(
    _CAMEL_RE, m => m.captures.at(0) + " " + m.captures.at(1),
  ))
  let tokens = tokenized.split(_NON_ALNUM_RE).filter(t => t != "")
  if "year" not in tokens { return "" }

  let observed = values.filter(v => v != none and v != "")
  if observed.map(py_str).dedup().len() <= 1 { return "" }
  for value in observed {
    if type(value) == bool { return "" }
    // PORT-EXC: `float(value)` under `except (TypeError, ValueError)`. A
    // number converts; a numeric string converts; anything else is the
    // TypeError branch and bails.
    let numeric = if is_number(value) {
      float(value)
    } else if type(value) == str and is-float-string(value) {
      float(value)
    } else {
      return ""
    }
    if numeric != calc.floor(numeric) or numeric < 1500 or numeric > 2200 { return "" }
  }
  "Year"
}

#let _DATE_LIKE_RE = regex("(?i)^\\d|^(jan|feb|mar|apr|may|jun|jul|aug|sep|oct|nov|dec)")

// flint/core/resolve_semantics.py looks_like_date_string
#let looks_like_date_string(s) = s.trim().match(_DATE_LIKE_RE) != none

// ---------------------------------------------------------------------------
// Temporal analysis
// ---------------------------------------------------------------------------

// flint/core/resolve_semantics.py _parse_date
//
// PORT-DATE: upstream short-circuits when handed a `datetime`; here the
// equivalents are a datehog moment (returned as-is) and a Typst `datetime`.
#let _parse_date(v) = {
  if dh.is-moment(v) { return v }
  if type(v) == datetime { return dh.from-typst-datetime(v) }
  js_date_parse(v)
}

// flint/core/resolve_semantics.py _utc_attrs
//
// Note the zero-based month: upstream returns `u.month - 1`, matching
// JavaScript's `getUTCMonth()`. Only used for equality comparisons below, but
// preserved so the two files read alike.
#let _utc_attrs(d) = (d.year, d.month - 1, d.day, d.hour, d.minute, d.second)

// flint/core/resolve_semantics.py analyze_temporal_field
#let analyze_temporal_field(field_values) = {
  let dates = ()
  let non_null = 0
  for v in field_values.slice(0, calc.min(100, field_values.len())) {
    if v == none { continue }
    non_null += 1
    let d = _parse_date(v)
    if d != none { dates.push(d) }
  }
  if dates.len() < 2 or dates.len() < non_null * 0.5 { return none }

  let years = ()
  let months = ()
  let days = ()
  let hours = ()
  let minutes = ()
  let seconds = ()
  for d in dates {
    let (y, mo, da, h, mi, se) = _utc_attrs(d)
    years.push(y)
    months.push(mo)
    days.push(da)
    hours.push(h)
    minutes.push(mi)
    seconds.push(se)
  }
  years = years.dedup()
  months = months.dedup()
  days = days.dedup()
  hours = hours.dedup()
  minutes = minutes.dedup()
  seconds = seconds.dedup()

  let is_small_spread(s, max_spread: 1) = {
    if s.len() <= 1 { return true }
    calc.max(..s) - calc.min(..s) <= max_spread
  }

  let same = (
    month: months.len() == 1,
    day: days.len() == 1,
    hour: is_small_spread(hours, max_spread: 1),
    minute: minutes.len() == 1,
    second: seconds.len() == 1,
  )
  let same_year = years.len() == 1
  let same_month = same_year and same.month
  let same_day = same_month and same.day

  (
    dates: dates,
    same: same,
    sameYear: same_year,
    sameMonth: same_month,
    sameDay: same_day,
  )
}

// flint/core/resolve_semantics.py compute_data_votes
//
// Each level scores how well a format granularity fits the data: the more
// components that are constant across the column, the coarser the format can
// be. Transcribed clause for clause.
#let compute_data_votes(same) = {
  let votes = (0, 0, 0, 0, 0, 0)

  // PORT-IDIOM: upstream writes `votes[5] += 1`. A helper closure cannot do
  // this in Typst — a closure captures outer locals by value and cannot assign
  // to them — so the increments stay inline, which also matches upstream.
  if same.second { votes.at(5) += 1 }
  if same.minute and same.second { votes.at(5) += 1 }
  if same.hour and same.minute and same.second { votes.at(5) += 1 }
  if same.day and same.hour and same.minute and same.second { votes.at(5) += 2 }
  if same.month and same.day and same.hour and same.minute and same.second { votes.at(5) += 3 }

  if same.second { votes.at(4) += 1 }
  if same.minute and same.second { votes.at(4) += 1 }
  if same.hour and same.minute and same.second { votes.at(4) += 1 }
  if same.day and same.hour and same.minute and same.second { votes.at(4) += 2 }
  if (not same.month) and same.day and same.hour and same.minute and same.second { votes.at(4) += 3 }

  if same.second { votes.at(3) += 1 }
  if same.minute and same.second { votes.at(3) += 1 }
  if same.hour and same.minute and same.second { votes.at(3) += 1 }
  if (not same.day) and same.hour and same.minute and same.second { votes.at(3) += 3 }

  if same.second { votes.at(2) += 1 }
  if same.minute and same.second { votes.at(2) += 1 }
  if (not same.hour) and same.minute and same.second { votes.at(2) += 3 }

  if same.second { votes.at(1) += 1 }
  if (not same.minute) and same.second { votes.at(1) += 3 }

  if not same.second { votes.at(0) += 4 }

  votes
}

#let SEMANTIC_LEVEL = (
  Year: 5, Decade: 5,
  YearMonth: 4, Month: 4, YearQuarter: 4, Quarter: 4,
  Date: 3, Day: 3,
  Hour: 2,
  DateTime: 1,
  Timestamp: 0,
)

// flint/core/resolve_semantics.py pick_best_level
//
// `>=` rather than `>`: ties go to the coarser level, since the loop runs
// upward.
#let pick_best_level(votes) = {
  let best_level = 0
  let best_score = votes.at(0)
  for i in range(1, 6) {
    if votes.at(i) >= best_score {
      best_score = votes.at(i)
      best_level = i
    }
  }
  (level: best_level, score: best_score)
}

// flint/core/resolve_semantics.py level_to_format
#let level_to_format(level, analysis) = {
  if level == 5 { return "%Y" }
  if level == 4 { return if analysis.sameYear { "%b" } else { "%b %Y" } }
  if level == 3 { return if analysis.sameYear { "%b %d" } else { "%b %d, %Y" } }
  if level == 2 { return if analysis.sameDay { "%H:00" } else { "%b %d %H:00" } }
  if level == 1 { return if analysis.sameDay { "%H:%M" } else { "%b %d %H:%M" } }
  if level == 0 { return if analysis.sameDay { "%H:%M:%S" } else { "%b %d %H:%M:%S" } }
  none
}

// flint/core/resolve_semantics.py _resolve_temporal_format
#let _resolve_temporal_format(field_values, semantic_type) = {
  let analysis = analyze_temporal_field(field_values)
  if falsy(analysis) { return none }
  let votes = compute_data_votes(analysis.same)
  let sem_level = SEMANTIC_LEVEL.at(semantic_type, default: none)
  if sem_level != none { votes.at(sem_level) = votes.at(sem_level) + 3 }
  let pick = pick_best_level(votes)
  level_to_format(pick.level, analysis)
}

// ---------------------------------------------------------------------------
// Temporal data conversion
// ---------------------------------------------------------------------------

#let _TWO_DIGIT_RE = regex("^\\d{2}$")

// flint/core/resolve_semantics.py _expand_to_full_year
//
// Same rule as datehog's `expand-two-digit-year`, kept as its own function so
// the port still reads against upstream.
#let _expand_to_full_year(val) = dh.expand-two-digit-year(val)

// flint/core/resolve_semantics.py _to_iso_z
//
// PORT-DATE: mirrors `Date.toISOString()`; datehog's `to-iso` emits exactly
// that shape.
#let _to_iso_z(dt) = dh.to-iso(dt)

// flint/core/resolve_semantics.py _js_number_to_string
//
// PORT-NUM: mirrors JS `String(<number>)` — integers without a trailing `.0`.
// Typst's `str` already does this for integral floats, unlike Python's `repr`.
// It also renders a negative with U+2212, hence `num-str`.
#let _js_number_to_string(v) = num-str(v)

// flint/core/resolve_semantics.py convert_temporal_data
//
// Rewrites every temporal column into a canonical string form so downstream
// stages see one representation. Non-temporal columns pass through untouched.
#let convert_temporal_data(data, semantic_types) = {
  if falsy(data) { return data }

  let keys = data.at(0).keys()
  let field_values = (:)
  for key in keys { field_values.insert(key, data.map(r => r.at(key, default: none))) }

  let effective_semantic_types = (:)
  for key in keys {
    let annotated = to_type_string(semantic_types.at(key, default: none))
    effective_semantic_types.insert(
      key,
      if truthy(annotated) { annotated } else {
        infer_implicit_semantic_type(key, field_values.at(key))
      },
    )
  }

  let temporal_keys = ()
  for k in keys {
    let st = effective_semantic_types.at(k)
    let vc = infer_vis_category(field_values.at(k))
    let st_category = if truthy(st) { get_vis_category(st) } else { none }
    if vc == "temporal" or st_category == "temporal" or st == "Decade" { temporal_keys.push(k) }
  }

  if temporal_keys.len() == 0 { return data }

  // PORT-MUT: upstream deep-copies the table and mutates the copy in place.
  // Typst values are copied on assignment, so the rows are rebuilt instead —
  // same result, no aliasing to reason about.
  data.map(r => {
    let row = r
    for temporal_key in temporal_keys {
      let val = row.at(temporal_key, default: none)
      let st = effective_semantic_types.at(temporal_key)

      // `isinstance(val, bool)` is checked *before* the numeric branch, as
      // upstream does: in Python `bool` is a subclass of `int`.
      if type(val) == bool {
        row.insert(temporal_key, if val { "true" } else { "false" })
      } else if is_number(val) {
        if st == "Year" or st == "Decade" {
          row.insert(temporal_key, num-str(int(calc.floor(val))))
        } else if is_likely_timestamp(val) {
          row.insert(temporal_key, _to_iso_z(dh.from-ms(timestamp_to_ms(val))))
        } else {
          row.insert(temporal_key, _js_number_to_string(val))
        }
      } else if dh.is-moment(val) or type(val) == datetime {
        row.insert(temporal_key, _to_iso_z(_parse_date(val)))
      } else if type(val) == str {
        if st == "Year" or st == "Decade" {
          row.insert(temporal_key, _expand_to_full_year(val))
        } else {
          row.insert(temporal_key, val)
        }
      } else if val == none {
        // JS `String(null)` is "null"; upstream preserves that.
        row.insert(temporal_key, "null")
      } else {
        row.insert(temporal_key, py_str(val))
      }
    }
    row
  })
}

// ---------------------------------------------------------------------------
// Channel semantics
// ---------------------------------------------------------------------------

#import "decisions.typ": resolve_encoding_type
#import "field-semantics.typ": (
  resolve_color_scheme_hint, resolve_diverging_info, resolve_field_semantics,
  resolve_nice, resolve_reversed, resolve_stackable, resolve_tick_constraint,
)
#import "semantic-types.typ": get_recommended_color_scheme, infer_ordinal_sort_order

#let _ISO_DATETIME_RE = regex(
  "^\\d{4}-\\d{2}-\\d{2}T\\d{2}:\\d{2}:\\d{2}(\\.\\d+)?(Z|[+-]\\d{2}:\\d{2})?$",
)

// flint/core/resolve_semantics.py resolve_channel_semantics
//
// The Phase-0 entry point: for every encoded channel, work out what its field
// *means* — semantic type, vis type, format, scale, colour scheme, sort order —
// and return one record per channel.
#let resolve_channel_semantics(encodings, data, semantic_types, converted_data: none) = {
  let result = (:)
  let temporal_data = if converted_data != none { converted_data } else { data }

  for (channel, encoding) in encodings.pairs() {
    let field_name = encoding.at("field", default: none)
    let aggregate = encoding.at("aggregate", default: none)

    if falsy(field_name) and aggregate != "count" { continue }

    if falsy(field_name) and aggregate == "count" {
      result.insert(channel, (
        field: "_count",
        semanticAnnotation: (semanticType: "Count"),
        type: "quantitative",
        aggregationDefault: "sum",
      ))
      continue
    }

    if falsy(field_name) { continue }

    let raw_annotation = semantic_types.at(field_name, default: none)
    let supplied_semantic_type = if type(raw_annotation) == str {
      if truthy(raw_annotation) { raw_annotation } else { "" }
    } else if type(raw_annotation) == dictionary {
      let st = raw_annotation.at("semanticType", default: none)
      if truthy(st) { st } else { "" }
    } else {
      ""
    }
    let field_values = data.map(r => r.at(field_name, default: none))
    let semantic_type = if truthy(supplied_semantic_type) {
      supplied_semantic_type
    } else {
      infer_implicit_semantic_type(field_name, field_values)
    }

    let type_decision = resolve_encoding_type(
      semantic_type, field_values, channel, data, field_name,
    )

    let resolved_type = type_decision.vlType
    if truthy(encoding.at("type", default: none)) {
      resolved_type = encoding.type
    } else if channel == "column" or channel == "row" {
      if resolved_type != "nominal" and resolved_type != "ordinal" { resolved_type = "nominal" }
    }

    if resolved_type == "quantitative" {
      let head = data.slice(0, calc.min(15, data.len()))
      let sample_values = head
        .map(r => r.at(field_name, default: none))
        .filter(v => v != none)
      if (
        sample_values.len() > 0
          and sample_values.all(v => py_str(v).trim().match(_ISO_DATETIME_RE) != none)
      ) {
        resolved_type = "temporal"
      }
    }

    let fc = resolve_field_semantics(
      if truthy(raw_annotation) { raw_annotation } else { semantic_type },
      field_name,
      field_values,
    )
    let annotation = fc.semanticAnnotation

    let tick_constraint = resolve_tick_constraint(
      annotation.semanticType, domain: annotation.at("intrinsicDomain", default: none),
    )
    let reversed_ = resolve_reversed(annotation.semanticType, channel: channel)
    let nice = resolve_nice(
      annotation.semanticType, domain_constraint: fc.at("domainConstraint", default: none),
    )
    let stackable = resolve_stackable(annotation.semanticType)

    let cs = (
      field: field_name,
      semanticAnnotation: annotation,
      type: resolved_type,
    )
    // FieldSemantics-derived
    if fc.at("format", default: none) != none { cs.insert("format", fc.format) }
    if fc.at("tooltipFormat", default: none) != none { cs.insert("tooltipFormat", fc.tooltipFormat) }
    if fc.at("aggregationDefault", default: none) != none {
      cs.insert("aggregationDefault", fc.aggregationDefault)
    }
    if fc.at("scaleType", default: none) != none { cs.insert("scaleType", fc.scaleType) }
    if fc.at("domainConstraint", default: none) != none {
      cs.insert("domainConstraint", fc.domainConstraint)
    }
    if truthy(fc.at("cyclic", default: none)) { cs.insert("cyclic", true) }
    if fc.at("sortDirection", default: none) != none { cs.insert("sortDirection", fc.sortDirection) }
    if truthy(fc.at("binningSuggested", default: none)) { cs.insert("binningSuggested", true) }

    // Channel-level
    cs.insert("nice", nice)
    if tick_constraint != none { cs.insert("tickConstraint", tick_constraint) }
    if truthy(reversed_) { cs.insert("reversed", true) }
    cs.insert("stackable", stackable)

    // Adjust aggregated field name
    if truthy(aggregate) {
      if aggregate == "count" {
        cs.insert("field", "_count")
        cs.insert("type", "quantitative")
      } else {
        cs.insert("field", field_name + "_" + aggregate)
        cs.insert("type", "quantitative")
      }
    }

    // Color scheme
    if (channel == "color" or channel == "group") and truthy(field_name) {
      let scheme = encoding.at("scheme", default: none)
      if truthy(scheme) and scheme != "default" {
        cs.insert("colorScheme", (
          scheme: scheme, type: "categorical", reason: "explicit user scheme",
        ))
      } else {
        let encoding_vl_type = cs.type
        let color_hint = resolve_color_scheme_hint(semantic_type, annotation, field_values)
        // Only the *count* is used, so `.dedup()` stands in for Python's set;
        // both collapse `1` and `1.0` into one entry.
        let unique_values = field_values.dedup()
        let color_scheme = get_recommended_color_scheme(
          semantic_type, encoding_vl_type,
          unique_value_count: unique_values.len(),
          field_name: field_name,
          values: field_values,
          color_hint: (type: color_hint.type),
        )
        if color_scheme.type == "diverging" and encoding_vl_type == "quantitative" {
          let nums = field_values.filter(v => (
            type(v) != bool and is_number(v) and not is-nan(v)
          ))
          let div_info = resolve_diverging_info(semantic_type, annotation, nums)
          if truthy(div_info) { color_scheme.insert("domainMid", div_info.midpoint) }
        }
        cs.insert("colorScheme", color_scheme)
      }
    }

    // Temporal format
    if (
      cs.type == "temporal"
        or (truthy(semantic_type) and get_vis_category(semantic_type) == "temporal")
    ) {
      let converted_field_values = temporal_data.map(r => r.at(field_name, default: none))
      let fmt = _resolve_temporal_format(converted_field_values, semantic_type)
      if truthy(fmt) { cs.insert("temporalFormat", fmt) }
    }

    // Ordinal sort
    if cs.type == "ordinal" or cs.type == "nominal" {
      if falsy(encoding.at("sortOrder", default: none)) and falsy(encoding.at("sortBy", default: none)) {
        let ordinal_sort = infer_ordinal_sort_order(semantic_type, field_values)
        if truthy(ordinal_sort) { cs.insert("ordinalSortOrder", ordinal_sort) }
      }
    }

    result.insert(channel, cs)
  }

  result
}
