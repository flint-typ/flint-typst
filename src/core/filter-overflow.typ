// Port of flint/core/filter_overflow.py.
//
// When a discrete channel has more distinct values than its layout budget
// allows, decide which to keep and drop the rest from the data, recording a
// warning and a truncation record.

#import "semantic-types.typ": infer_vis_category
#import "py.typ": falsy, is-float-string, is-nan, is_number, num-str, py_str, truthy

// flint/core/filter_overflow.py _js_sort_key
//
// JS `Array.prototype.sort()` with no comparator coerces to string.
#let _js_sort_key(v) = {
  if v == none { return "null" }
  py_str(v)
}

// Ours. Python's `sorted(xs, key=k, reverse=True)` is stable and leaves equal
// elements in their original order; Typst's `sorted(...).rev()` reverses them
// too. Reversing before and after restores the Python semantics — which is
// literally how CPython implements `reverse=True`.
#let sorted-desc(xs, key: v => v) = xs.rev().sorted(key: key).rev()

// Ours. A conservative JSON-array-of-scalars check.
//
// PORT-EXC: upstream calls `json.loads(sort_by)` inside a bare `except`.
// Typst's `json(bytes(..))` cannot be caught, so the shape is validated first.
// This accepts an array of strings, numbers, booleans and nulls, which is what
// a stored custom sort order actually is.
#let _JSON_SCALAR_ARRAY = regex(
  "^\\s*\\[\\s*(" +
  "(\"([^\"\\\\]|\\\\.)*\"|-?\\d+(\\.\\d+)?([eE][+-]?\\d+)?|true|false|null)" +
  "(\\s*,\\s*(\"([^\"\\\\]|\\\\.)*\"|-?\\d+(\\.\\d+)?([eE][+-]?\\d+)?|true|false|null))*" +
  ")?\\s*\\]\\s*$",
)
#let _try-json-array(s) = {
  if type(s) != str { return none }
  if s.match(_JSON_SCALAR_ARRAY) == none { return none }
  json(bytes(s))
}

// flint/core/filter_overflow.py _default_overflow_strategy
// PORT-IDIOM: upstream names the last parameter `context`, which is a reserved
// word in Typst. Renamed to `ctx` — the only place in the port where an
// upstream *parameter name* could not be preserved. It is passed positionally,
// so no call site changes.
#let _default_overflow_strategy(channel, field_name, unique_values, max_to_keep, ctx) = {
  let data = ctx.data
  let channel_semantics = ctx.channelSemantics
  let encodings = ctx.encodings
  let all_mark_types = ctx.allMarkTypes

  let encoding = {
    let e = encodings.at(channel, default: none)
    if truthy(e) { e } else { (:) }
  }
  let sort_by = encoding.at("sortBy", default: none)
  let sort_order = encoding.at("sortOrder", default: none)

  let sort_field = none
  let sort_field_type = none
  let is_descending = false

  if truthy(sort_by) {
    if sort_by == "x" or sort_by == "y" or sort_by == "color" {
      let sort_cs = channel_semantics.at(sort_by, default: none)
      sort_field = if truthy(sort_cs) { sort_cs.at("field", default: none) } else { none }
      sort_field_type = if truthy(sort_cs) { sort_cs.at("type", default: none) } else { none }
      is_descending = (
        sort_order == "descending" or (sort_order != "ascending" and sort_by != channel)
      )
    } else {
      let sorted_list = _try-json-array(sort_by)
      if type(sorted_list) == array {
        let ordered_values = if sort_order == "descending" { sorted_list.rev() } else { sorted_list }
        let kept = ordered_values.filter(v => v in unique_values)
        return kept.slice(0, calc.min(max_to_keep, kept.len()))
      }
      is_descending = sort_order == "descending"
    }
  }

  if truthy(sort_field) and sort_field_type == "quantitative" {
    let color_cs = channel_semantics.at("color", default: none)
    let color_field = if truthy(color_cs) { color_cs.at("field", default: none) } else { none }
    let is_bar = "bar" in all_mark_types and sort_field != color_field

    // PORT-IDIOM: upstream keys a dict by the raw field value. Typst dictionary
    // keys must be strings, so the accumulator is an array of (value, total)
    // and lookup is by `==` — which is what Python's dict does here anyway,
    // since `1` and `1.0` hash and compare equal in both.
    let values = ()
    let totals = ()
    for row in data {
      let field_value = row.at(field_name, default: none)
      let raw = row.at(sort_field, default: none)
      let sort_value = if truthy(raw) { raw } else { 0 }
      let i = values.position(v => v == field_value)
      if i == none {
        values.push(field_value)
        totals.push(if is_bar { 0 + sort_value } else { calc.max(-float.inf, sort_value) })
      } else if is_bar {
        totals.at(i) += sort_value
      } else {
        totals.at(i) = calc.max(totals.at(i), sort_value)
      }
    }

    let entries = values.zip(totals)
    let ordered = if is_descending {
      entries.sorted(key: e => -e.at(1))
    } else {
      entries.sorted(key: e => e.at(1))
    }
    return ordered.slice(0, calc.min(max_to_keep, ordered.len())).map(e => e.at(0))
  }

  let cs = channel_semantics.at(channel, default: none)
  let canonical_order = if truthy(cs) { cs.at("ordinalSortOrder", default: none) } else { none }
  if falsy(sort_by) and falsy(sort_order) and truthy(canonical_order) {
    let ordered = canonical_order.filter(value => value in unique_values)
    ordered += unique_values.filter(value => value not in ordered)
    return ordered.slice(0, calc.min(max_to_keep, ordered.len()))
  }

  let field_original_type = infer_vis_category(data.map(r => r.at(field_name, default: none)))
  if field_original_type == "quantitative" or channel == "color" {
    // JS sorts with `a - b`, so numeric strings sort as numbers. Upstream
    // mirrors that with a `float()` key; unparseable values become NaN, whose
    // ordering is unspecified in both languages.
    let numeric_key(v) = {
      if type(v) == bool { return if v { 1.0 } else { 0.0 } }
      if is_number(v) { return float(v) }
      if type(v) == str and is-float-string(v) { return float(v) }
      float.nan
    }
    // PORT-NUM: Typst refuses to compare NaN with NaN and errors; Python's
    // sort tolerates it, because every NaN comparison is False and Timsort
    // then leaves those elements where they are. With *every* key NaN — a
    // string column on the colour channel, which is the case that actually
    // occurs — Python returns the input untouched, so that is reproduced
    // exactly. Mixed NaN/number keys fall back to sorting NaNs last in
    // original order; see PORT-DICTIONARY.
    let keys = unique_values.map(numeric_key)
    let finite = keys.filter(k => not is-nan(k))
    let ordered = if finite.len() == 0 {
      unique_values
    } else {
      unique_values.sorted(key: v => {
        let k = numeric_key(v)
        if is-nan(k) { float.inf } else { k }
      })
    }
    return ordered.slice(0, calc.min(max_to_keep, ordered.len()))
  }

  if channel == "column" or channel == "row" {
    return unique_values.slice(0, calc.min(max_to_keep, unique_values.len()))
  }

  if sort_order == "descending" {
    let ordered = sorted-desc(unique_values, key: _js_sort_key)
    return ordered.slice(0, calc.min(max_to_keep, ordered.len()))
  }

  if sort_order == "ascending" {
    let ordered = unique_values.sorted(key: _js_sort_key)
    return ordered.slice(0, calc.min(max_to_keep, ordered.len()))
  }

  unique_values.slice(0, calc.min(max_to_keep, unique_values.len()))
}

// flint/core/filter_overflow.py filter_overflow
#let filter_overflow(
  channel_semantics, declaration, encodings, data, budgets, all_mark_types,
) = {
  let effective_type(ch) = {
    let rt = {
      let r = declaration.at("resolvedTypes", default: none)
      if truthy(r) { r } else { (:) }
    }.at(ch, default: none)
    if rt != none { return rt }
    let cs = channel_semantics.at(ch, default: none)
    if truthy(cs) { cs.at("type", default: none) } else { none }
  }

  let effective_field(ch) = {
    let cs = channel_semantics.at(ch, default: none)
    if truthy(cs) and truthy(cs.at("field", default: none)) { return cs.field }
    none
  }

  let is_discrete_type(t) = t == "nominal" or t == "ordinal"

  let nominal_counts = (x: 0, y: 0, column: 0, row: 0, group: 0)
  let truncations = ()
  let warnings = ()
  let filtered_data = data

  let group_cs = channel_semantics.at("group", default: none)
  let group_field = if truthy(group_cs) { group_cs.at("field", default: none) } else { none }
  if truthy(group_field) {
    nominal_counts.group = data.map(r => r.at(group_field, default: none)).dedup().len()
  }

  let strategy_context = (
    data: data,
    channelSemantics: channel_semantics,
    encodings: encodings,
    allMarkTypes: all_mark_types,
  )

  // Upstream allows the declaration to supply its own strategy function.
  let declared = declaration.at("overflowStrategy", default: none)
  let strategy = if truthy(declared) { declared } else { _default_overflow_strategy }

  for channel in ("x", "y", "column", "row", "color") {
    let field_name = effective_field(channel)
    let type_ = effective_type(channel)
    if falsy(field_name) { continue }

    let max_to_keep = {
      let mv = budgets.at("maxValues", default: none)
      let m = if truthy(mv) { mv.at(channel, default: none) } else { none }
      if m == none { float.inf } else { m }
    }

    if not is_discrete_type(type_) {
      if channel == "column" or channel == "row" {
        let unique_values = filtered_data.map(r => r.at(field_name, default: none)).dedup()
        nominal_counts.insert(channel, int(calc.min(unique_values.len(), max_to_keep)))
        if unique_values.len() > max_to_keep {
          let sorted_values = unique_values.sorted(key: _js_sort_key)
          let values_to_keep = sorted_values.slice(0, int(max_to_keep))
          let omitted_count = unique_values.len() - values_to_keep.len()
          warnings.push((
            severity: "warning", code: "overflow",
            message: (
              num-str(omitted_count) + " of " + num-str(unique_values.len()) + " values in '"
                + field_name + "' were omitted (showing first "
                + num-str(values_to_keep.len()) + ")."
            ),
            channel: channel, field: field_name,
          ))
          filtered_data = filtered_data.filter(row => row.at(field_name, default: none) in values_to_keep)
        }
      }
      continue
    }

    let unique_values = filtered_data.map(r => r.at(field_name, default: none)).dedup()
    nominal_counts.insert(channel, int(calc.min(unique_values.len(), max_to_keep)))

    if unique_values.len() > max_to_keep {
      let values_to_keep = strategy(
        channel, field_name, unique_values, int(max_to_keep), strategy_context,
      )

      let omitted_count = unique_values.len() - values_to_keep.len()
      let placeholder = "..." + num-str(omitted_count) + " items omitted"
      let message = (
        num-str(omitted_count) + " of " + num-str(unique_values.len()) + " values in '"
          + field_name + "' were omitted (showing first "
          + num-str(values_to_keep.len()) + " in sort order)."
      )

      warnings.push((
        severity: "warning", code: "overflow",
        message: message, channel: channel, field: field_name,
      ))

      truncations.push((
        severity: "warning", code: "overflow",
        message: message, channel: channel, field: field_name,
        keptValues: values_to_keep, omittedCount: omitted_count,
        placeholder: placeholder,
      ))

      if channel != "color" {
        filtered_data = filtered_data.filter(row => row.at(field_name, default: none) in values_to_keep)
      }
    }
  }

  (
    filteredData: filtered_data,
    nominalCounts: nominal_counts,
    truncations: truncations,
    warnings: warnings,
  )
}
