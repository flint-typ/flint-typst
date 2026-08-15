// Port of flint/core/semantic_types.py.
//
// The lookup tables (`SemanticTypes`, `_ORDINAL_SEQUENCES`) are generated into
// `semantic-types-data.typ`; everything here is hand-ported.

#import "type-registry.typ": get_registered_types, get_registry_entry, is_registered
#import "semantic-types-data.typ": SemanticTypes, _ORDINAL_SEQUENCES
#import "py.typ": falsy, is-float-string, is-nan, is_number, py_str, truthy
// datehog is a sibling package, not yet published. During development it is
// linked into Typst's local namespace:
//   ln -s <repo>/datehog ~/.local/share/typst/packages/local/datehog/0.1.0
// Becomes `@preview/datehog:0.1.0` once published.
#import "@local/datehog:0.1.0" as dh

#let SemanticTypes = SemanticTypes
#let _ORDINAL_SEQUENCES = _ORDINAL_SEQUENCES

// ---------------------------------------------------------------------------
// Type sets — derived from the registry
// ---------------------------------------------------------------------------
//
// PORT-IDIOM: upstream builds Python `set`s by comprehension. Typst has no
// set, so these are arrays and membership is `in`. They hold ~20 entries and
// are only ever tested for membership, so the linear scan is immaterial.

#let measureTypes = get_registered_types().filter(t => {
  let e = get_registry_entry(t)
  (
    (e.aggRole == "additive" or e.aggRole == "intensive" or e.aggRole == "signed-additive")
      and e.t1 != "Score"
  )
})

#let nonMeasureNumericTypes = (
  "Rank", "ID", "Score",
  "Year", "Month", "Day", "Hour",
  "Latitude", "Longitude",
)

#let categoricalTypes = get_registered_types().filter(t => {
  let e = get_registry_entry(t)
  ("nominal" in e.visEncodings and e.aggRole != "identifier") or e.t1 == "Binned"
})

#let ordinalTypes = get_registered_types().filter(t => "ordinal" in get_registry_entry(t).visEncodings)

// ---------------------------------------------------------------------------
// Helper functions
// ---------------------------------------------------------------------------

// flint/core/semantic_types.py get_vis_category
#let get_vis_category(semantic_type) = {
  if falsy(semantic_type) or not is_registered(semantic_type) { return none }
  let enc = get_registry_entry(semantic_type).visEncodings
  if enc.len() != 0 { enc.at(0) } else { none }
}

#let _DATE_LIKE_RE = regex("(?i)^\\d|^(jan|feb|mar|apr|may|jun|jul|aug|sep|oct|nov|dec)")

// flint/core/semantic_types.py _is_boolean
#let _is_boolean(v) = type(v) == bool

// flint/core/semantic_types.py _is_number_like
//
// Mirrors JS `!isNaN(+v) && !(v instanceof Date)`, including the coercions
// that make `true`, `null` and `""` all number-like.
#let _is_number_like(v) = {
  if type(v) == bool { return true } // +true === 1, +false === 0
  if is_number(v) { return not is-nan(v) } // exclude NaN
  if v == none { return true } // +null === 0
  if type(v) == str {
    let s = v.trim()
    if s == "" { return true } // +"" === 0
    // PORT-EXC: upstream is `try: float(s)`. Typst cannot catch, and a failed
    // `float()` aborts the whole document, so the shape is checked first.
    // The pattern accepts exactly what Python's `float()` does: optional sign,
    // decimal or exponent form, and the special words.
    return is-float-string(s)
  }
  false
}

// flint/core/semantic_types.py _looks_like_date_string
#let _looks_like_date_string(s) = s.trim().match(_DATE_LIKE_RE) != none

// Ours. The exact acceptance sets of the two upstream calls, verified against
// CPython 3.14 rather than assumed:
//
//   datetime.fromisoformat  needs a COMPLETE date -- "2020-01-01" or the basic
//                           form "20200101". Rejects "2020", "2020-01" and
//                           "2020-1-1" (components must be zero-padded), and
//                           rejects impossible dates like "2020-02-30".
//   parsedate_to_datetime   needs day + month-name + year AND a time; bare
//                           "15 Feb 2020" is rejected, "15 Feb 2020 08:30" is not.
//
// Both are narrower than datehog's general parsers, which is the point: using
// `dh.parse-ms` here would classify far more fields as temporal than flint-py
// does, since it also accepts "2020", "Jan 2020" and "01/15/2020".
#let _ISO_FULL_RE = regex("^(\\d{4})-(\\d{2})-(\\d{2})([T ].*)?$")
#let _ISO_BASIC_RE = regex("^(\\d{4})(\\d{2})(\\d{2})([T ].*)?$")
#let _RFC_RE = regex(
  "(?i)^(?:[a-z]{3},?\\s+)?(?:" +
  "(\\d{1,2})\\s+([a-z]{3,9})\\.?\\s+(\\d{4})" +   // 15 Feb 2020
  "|([a-z]{3,9})\\.?\\s+(\\d{1,2}),?\\s+(\\d{4})" + // Feb 15 2020
  ")\\s+\\d{1,2}:\\d{2}(:\\d{2})?(\\s+.*)?$",
)

// flint/core/semantic_types.py _date_parse_succeeds
//
// PORT-DATE + PORT-EXC. Upstream tries `datetime.fromisoformat` (with `Z`
// rewritten) and then `email.utils.parsedate_to_datetime`, each under a bare
// `except`. Typst cannot catch, so each becomes a shape check followed by a
// datehog parse that returns `none` rather than raising.
#let _date_parse_succeeds(v) = {
  if v == none { return false }
  if type(v) != str { return false }
  let s = v.trim()
  if s == "" { return false }

  // Branch 1: ISO, complete date required.
  let m = s.match(_ISO_FULL_RE)
  let basic = false
  if m == none {
    m = s.match(_ISO_BASIC_RE)
    basic = m != none
  }
  if m != none {
    let y = int(m.captures.at(0))
    let mo = int(m.captures.at(1))
    let d = int(m.captures.at(2))
    if dh.is-valid-date(y, mo, d) {
      let rest = m.captures.at(3)
      // A date with no time part is accepted outright; with one, the time has
      // to parse. Rebuild in extended form so datehog sees a shape it knows.
      if rest == none { return true }
      let iso = (
        dh.pad(y, 4) + "-" + dh.pad(mo, 2) + "-" + dh.pad(d, 2)
          + rest.replace("Z", "+00:00").replace("z", "+00:00")
      )
      return dh.parse-iso(iso) != none
    }
    return false
  }

  // Branch 2: RFC 2822, which requires a time.
  if s.match(_RFC_RE) != none { return true }
  false
}

// flint/core/semantic_types.py _is_date_like
#let _is_date_like(v) = {
  if type(v) == str {
    if not _looks_like_date_string(v) { return false }
    return _date_parse_succeeds(v)
  }
  _date_parse_succeeds(v)
}

// flint/core/semantic_types.py infer_vis_category
#let infer_vis_category(values) = {
  if values.len() == 0 { return "nominal" }
  let non_null = values.filter(v => v != none)
  if non_null.len() == 0 { return "nominal" }
  if non_null.all(_is_boolean) { return "nominal" }
  if non_null.all(_is_number_like) { return "quantitative" }
  if non_null.all(_is_date_like) { return "temporal" }
  "nominal"
}

// flint/core/semantic_types.py is_measure_type
#let is_measure_type(semantic_type) = semantic_type in measureTypes

// flint/core/semantic_types.py is_time_series_type
#let is_time_series_type(semantic_type) = {
  let entry = get_registry_entry(semantic_type)
  entry.t0 == "Temporal" and entry.t1 != "Duration"
}

// flint/core/semantic_types.py is_categorical_type
#let is_categorical_type(semantic_type) = semantic_type in categoricalTypes

// flint/core/semantic_types.py is_ordinal_type
#let is_ordinal_type(semantic_type) = semantic_type in ordinalTypes

// flint/core/semantic_types.py is_geo_type
#let is_geo_type(semantic_type) = get_registry_entry(semantic_type).t0 == "Geographic"

// flint/core/semantic_types.py is_geo_coordinate_type
#let is_geo_coordinate_type(semantic_type) = get_registry_entry(semantic_type).t1 == "GeoCoordinate"

// flint/core/semantic_types.py is_geo_location_string
#let is_geo_location_string(semantic_type) = get_registry_entry(semantic_type).t1 == "GeoPlace"

// flint/core/semantic_types.py is_non_measure_numeric
#let is_non_measure_numeric(semantic_type) = semantic_type in nonMeasureNumericTypes

// ---------------------------------------------------------------------------
// Zero baseline
// ---------------------------------------------------------------------------

// flint/core/semantic_types.py get_zero_class
#let get_zero_class(semantic_type) = {
  let baseline = get_registry_entry(semantic_type).zeroBaseline
  if baseline == "none" { return "unknown" }
  baseline
}

/// Above this ratio of dataMin/dataMax, strictly-positive data sits far enough
/// above zero that anchoring at zero would leave at least half the axis empty.
#let ZERO_BASELINE_GAP_THRESHOLD = 0.5

// flint/core/semantic_types.py _data_far_from_zero
#let _data_far_from_zero(values) = {
  if falsy(values) { return false }
  let data_min = calc.min(..values)
  let data_max = calc.max(..values)
  if data_min <= 0 or data_max <= 0 { return false }
  data_min / data_max >= ZERO_BASELINE_GAP_THRESHOLD
}

// flint/core/semantic_types.py compute_zero_decision
#let compute_zero_decision(semantic_type, channel, mark_type, values: none) = {
  let is_bar_like = mark_type == "bar" or mark_type == "area" or mark_type == "rect"
  let is_scatter_mark = mark_type == "circle" or mark_type == "point"
  let is_positional = channel == "x" or channel == "y"
  let entry = get_registry_entry(semantic_type)
  let zero_class = get_zero_class(semantic_type)

  // `entry.zeroPad or 0.05` — upstream relies on 0 being falsy here.
  let pad_or(fallback) = {
    let p = entry.at("zeroPad", default: none)
    if truthy(p) { p } else { fallback }
  }

  if zero_class == "meaningful" {
    // Length marks: zero is structurally required. Not debatable.
    if is_bar_like {
      return (zero: true, domainPadFraction: 0, zeroClass: zero_class, forced: true, uncertain: false)
    }
    // Scatter position: the read is correlation, not distance from zero, so
    // data-fit is the default and zero becomes an opt-in toggle.
    if is_positional and is_scatter_mark {
      if values != none and values.len() > 0 and calc.min(..values) <= 0 {
        return (zero: true, domainPadFraction: 0, zeroClass: zero_class, forced: true, uncertain: false)
      }
      return (
        zero: false, domainPadFraction: pad_or(0.05),
        zeroClass: zero_class, forced: false, uncertain: true,
      )
    }
    // Position marks (line/strip): zero is the conventional reference, so
    // default on; only offer a toggle when the data sits far from zero.
    return (
      zero: true, domainPadFraction: 0, zeroClass: zero_class,
      forced: false, uncertain: _data_far_from_zero(values),
    )
  }

  if zero_class == "arbitrary" {
    if is_bar_like and values != none and values.len() > 0 {
      let data_min = calc.min(..values)
      if data_min <= 0 {
        return (zero: true, domainPadFraction: 0, zeroClass: zero_class, forced: true, uncertain: false)
      }
    }
    return (
      zero: false, domainPadFraction: pad_or(0.05),
      zeroClass: zero_class, forced: false, uncertain: false,
    )
  }

  if zero_class == "contextual" and values != none and values.len() > 0 {
    let data_min = calc.min(..values)
    let data_max = calc.max(..values)

    if data_min <= 0 {
      return (zero: true, domainPadFraction: 0, zeroClass: zero_class, forced: true, uncertain: false)
    }

    let proximity = if data_max > 0 { data_min / data_max } else { 0 }
    if proximity < 0.3 {
      return (zero: true, domainPadFraction: 0, zeroClass: zero_class, forced: false, uncertain: false)
    }
    if is_bar_like {
      return (zero: true, domainPadFraction: 0, zeroClass: zero_class, forced: true, uncertain: false)
    }
    return (zero: false, domainPadFraction: 0.05, zeroClass: zero_class, forced: false, uncertain: false)
  }

  // Unknown class is never debatable: no basis for a toggle.
  if is_bar_like and is_positional {
    return (zero: true, domainPadFraction: 0, zeroClass: "unknown", forced: true, uncertain: false)
  }
  (zero: false, domainPadFraction: 0.05, zeroClass: "unknown", forced: true, uncertain: false)
}

// flint/core/semantic_types.py compute_padded_domain
#let compute_padded_domain(values, pad_fraction) = {
  if pad_fraction <= 0 or values.len() < 2 { return none }
  let data_min = calc.min(..values)
  let data_max = calc.max(..values)
  let span = data_max - data_min
  if span <= 0 { return none }
  let padding = span * pad_fraction
  (data_min - padding, data_max + padding)
}

// ---------------------------------------------------------------------------
// Color schemes
// ---------------------------------------------------------------------------

// flint/core/semantic_types.py _pick_scheme
//
// Emulates JS `hash = ((hash << 5) - hash) + charCode` under signed 32-bit
// wraparound. Typst's int carries `bit-lshift`/`bit-and`, so this transcribes
// the Python (which is itself transcribing the JS) operator for operator.
#let _pick_scheme(schemes, name) = {
  let h = 0
  for ch in name.clusters() {
    let code = str.to-unicode(ch)
    h = h.bit-lshift(5) - h + code
    // Truncate to 32 bits, then reinterpret as signed.
    h = h.bit-and(4294967295)
    if h.bit-and(2147483648) != 0 { h = h - 4294967296 }
  }
  schemes.at(calc.rem-euclid(calc.abs(h), schemes.len()))
}

// flint/core/semantic_types.py get_recommended_color_scheme
#let get_recommended_color_scheme(
  semantic_type,
  encoding_type,
  unique_value_count: 10,
  field_name: "",
  values: none,
  color_hint: none,
) = {
  let hint_is_diverging = truthy(color_hint) and color_hint.at("type", default: none) == "diverging"

  if falsy(semantic_type) {
    if encoding_type == "quantitative" {
      return (scheme: "viridis", type: "sequential", reason: "default for quantitative")
    }
    if encoding_type == "ordinal" {
      return (scheme: "blues", type: "sequential", reason: "default for ordinal")
    }
    return (
      scheme: if unique_value_count > 10 { "tableau20" } else { "tableau10" },
      type: "categorical",
      reason: "default for categorical",
    )
  }

  if semantic_type == "Temperature" {
    if hint_is_diverging {
      return (scheme: "redblue", type: "diverging", reason: "temperature diverging around freezing point")
    }
    return (scheme: "reds", type: "sequential", reason: "temperature single-direction uses sequential")
  }

  if semantic_type == "Percentage" {
    if hint_is_diverging {
      return (scheme: "redblue", type: "diverging", reason: "percentage spans positive and negative")
    }
    return (scheme: "oranges", type: "sequential", reason: "percentage all same sign uses sequential")
  }

  if semantic_type == "Price" or semantic_type == "Amount" {
    if hint_is_diverging {
      return (scheme: "redblue", type: "diverging", reason: "financial data spans positive and negative")
    }
    return (scheme: "goldgreen", type: "sequential", reason: "financial data uses gold-green")
  }

  if semantic_type == "Score" {
    if hint_is_diverging {
      return (scheme: "redblue", type: "diverging", reason: "score/rating diverging around midpoint")
    }
    return (scheme: "yelloworangebrown", type: "sequential", reason: "scores use warm sequential")
  }

  if semantic_type == "Rank" {
    return (scheme: "purples", type: "sequential", reason: "ranks use single-hue sequential")
  }

  if semantic_type == "Range" {
    return (scheme: "blues", type: "sequential", reason: "range groups use sequential")
  }

  if (
    semantic_type in ordinalTypes
      and semantic_type in ("Year", "Quarter", "Month", "Week", "Day", "Hour", "Decade")
  ) {
    return (scheme: "viridis", type: "sequential", reason: "temporal granules use perceptually uniform")
  }

  if get_registry_entry(semantic_type).t1 == "GeoPlace" {
    if unique_value_count <= 10 {
      return (scheme: "set2", type: "categorical", reason: "geographic regions use distinct pastels")
    }
    return (scheme: "tableau20", type: "categorical", reason: "many regions use large categorical")
  }

  if semantic_type == "Status" or semantic_type == "Boolean" {
    return (scheme: "set1", type: "categorical", reason: "status uses high-contrast categorical")
  }

  if semantic_type == "Category" {
    return (
      scheme: if unique_value_count > 10 { "tableau20" } else { "tableau10" },
      type: "categorical",
      reason: "categories use standard categorical",
    )
  }

  if semantic_type == "Name" {
    return (
      scheme: if unique_value_count > 8 { "tableau20" } else { "set2" },
      type: "categorical",
      reason: "names use readable categorical",
    )
  }

  if semantic_type == "Duration" {
    return (scheme: "oranges", type: "sequential", reason: "duration uses intensity-based sequential")
  }

  if semantic_type in measureTypes {
    if hint_is_diverging {
      return (scheme: "redblue", type: "diverging", reason: "measure with diverging nature")
    }
    let sequential_schemes = ("viridis", "blues", "greens", "reds", "yelloworangebrown", "goldgreen")
    return (
      scheme: _pick_scheme(sequential_schemes, field_name),
      type: "sequential",
      reason: "measures use perceptually uniform sequential",
    )
  }

  if semantic_type in ordinalTypes or encoding_type == "ordinal" {
    let ordinal_schemes = ("blues", "greens", "purples", "oranges")
    return (
      scheme: _pick_scheme(ordinal_schemes, field_name),
      type: "sequential",
      reason: "ordinal data uses sequential scheme",
    )
  }

  if encoding_type == "nominal" or encoding_type == "temporal" {
    return (
      scheme: if unique_value_count > 10 { "tableau20" } else { "tableau10" },
      type: "categorical",
      reason: "default categorical palette",
    )
  }

  (scheme: "viridis", type: "sequential", reason: "universal fallback")
}

// ---------------------------------------------------------------------------
// Canonical ordinal sort orders
// ---------------------------------------------------------------------------

// flint/core/semantic_types.py _build_lookup
#let _build_lookup(seq) = {
  let m = (:)
  let ci = seq.caseInsensitive
  for (i, label) in seq.labels.enumerate() {
    let key = if ci { lower(label) } else { label }
    m.insert(key, i)
  }
  m
}

// flint/core/semantic_types.py _match_sequence
//
// PORT-IDIOM: upstream uses a dict as an insertion-ordered set to collect the
// distinct values; Typst dictionaries preserve insertion order too, so the
// same shape ports directly.
#let _match_sequence(values, sequences) = {
  let seen = (:)
  for v in values {
    let s = if v != none { py_str(v) } else { "" }
    if s != "" and s not in seen { seen.insert(s, none) }
  }
  let unique_values = seen.keys()
  if unique_values.len() == 0 { return none }

  for seq in sequences {
    let lookup = _build_lookup(seq)
    let ci = seq.caseInsensitive
    let matched = ()
    let unmatched = ()
    for val in unique_values {
      let key = if ci { lower(val) } else { val }
      let idx = lookup.at(key, default: none)
      if idx != none { matched.push((val, idx)) } else { unmatched.push(val) }
    }
    if matched.len() >= unique_values.len() * 0.6 and matched.len() >= 2 {
      // Typst's `sorted` is stable, like Python's, which matters when two
      // differently-cased labels collapse to the same index.
      let ordered = matched.sorted(key: p => p.at(1))
      let result = ordered.map(p => p.at(0))
      result += unmatched
      return result
    }
  }
  none
}

// flint/core/semantic_types.py infer_ordinal_sort_order
#let infer_ordinal_sort_order(semantic_type, values) = {
  let sequences = _ORDINAL_SEQUENCES.at(semantic_type, default: none)
  if truthy(sequences) { return _match_sequence(values, sequences) }

  if falsy(semantic_type) or semantic_type == "Category" or semantic_type == "Unknown" {
    for seqs in _ORDINAL_SEQUENCES.values() {
      let result = _match_sequence(values, seqs)
      if truthy(result) { return result }
    }
  }

  none
}
