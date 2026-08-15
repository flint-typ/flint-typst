// Port of flint/core/encoding_actions.py.
//
// Reusable factories for Category-B encoding actions: declarative transforms
// applied to the user's encoding map at the top of assembly, so the whole
// pipeline sees the transformed encodings. `encoding-overrides.typ` composes
// them.

#import "py.typ": falsy, truthy

// flint/core/encoding_actions.py _is_measure_enc
#let _is_measure_enc(e) = {
  if falsy(e) or falsy(e.at("field", default: none)) { return false }
  truthy(e.at("aggregate", default: none)) or e.at("type", default: none) == "quantitative"
}

// flint/core/encoding_actions.py _is_discrete_category_enc
//
// Temporal axes are deliberately excluded: reordering a time axis by value
// scrambles the chronology, so Sort should not apply to them.
#let _is_discrete_category_enc(e) = {
  if falsy(e) or falsy(e.at("field", default: none)) { return false }
  (
    not truthy(e.at("aggregate", default: none))
      and e.at("type", default: none) != "quantitative"
      and e.at("type", default: none) != "temporal"
  )
}

// flint/core/encoding_actions.py _resolve_sort_channels
//
// PORT-IDIOM: `next((c for c in candidates if pred(c)), None)` is
// `candidates.find(pred)`, which returns `none` when nothing matches.
#let _resolve_sort_channels(encodings, candidates) = {
  let category = candidates.find(c => _is_discrete_category_enc(encodings.at(c, default: none)))
  let measure = candidates.find(c => _is_measure_enc(encodings.at(c, default: none)))
  if falsy(category) or falsy(measure) or category == measure { return none }
  (category: category, measure: measure)
}

// flint/core/encoding_actions.py make_sort_action
//
// Sort the category axis of a bar-like chart by the measure value. A value
// sort writes `sortBy = <measure channel>` on the category channel; "Default"
// clears it so the field's canonical ordering wins.
#let make_sort_action(key: "sort", label: "Sort", channels: ("x", "y")) = {
  let candidates = channels

  let is_applicable(ctx) = {
    let encodings = ctx.at("encodings", default: none)
    _resolve_sort_channels(if falsy(encodings) { (:) } else { encodings }, candidates) != none
  }

  let get_value(encodings) = {
    let resolved = _resolve_sort_channels(encodings, candidates)
    if falsy(resolved) { return none }
    let enc = encodings.at(resolved.category)
    if enc.at("sortBy", default: none) == resolved.measure {
      return if enc.at("sortOrder", default: none) == "descending" { "value-desc" } else { "value-asc" }
    }
    // Any other sort (label order, custom value order, sort-by-color) is not
    // representable by this control -> show as Default.
    none
  }

  let set_value(encodings, value) = {
    let resolved = _resolve_sort_channels(encodings, candidates)
    if falsy(resolved) { return encodings }
    let category = resolved.category
    let measure = resolved.measure
    let base = encodings.at(category)
    let nxt = if value == "value-asc" {
      (..base, sortBy: measure, sortOrder: "ascending")
    } else if value == "value-desc" {
      (..base, sortBy: measure, sortOrder: "descending")
    } else {
      (..base, sortBy: none, sortOrder: none)
    }
    (..encodings, (category): nxt)
  }

  (
    key: key,
    label: label,
    dependencies: candidates,
    isApplicable: is_applicable,
    control: (
      type: "discrete",
      options: (
        (value: none, label: "Default"),
        (value: "value-desc", label: "Value \u{2193}"),
        (value: "value-asc", label: "Value \u{2191}"),
      ),
    ),
    get: get_value,
    // PORT-IDIOM: `set` is a reserved word in Typst (the `set` rule), so the
    // key is quoted here and read with `.at("set")` at the call site. The key
    // name itself is unchanged, since the action dictionaries are part of the
    // template contract.
    "set": set_value,
  )
}
