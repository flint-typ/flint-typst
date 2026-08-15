// Port of flint/core/encoding_overrides.py.
//
// Composes Category-B encoding-action overrides (stored by the host in
// `chartProperties`, keyed by the action's `key`) onto the base encoding map
// before any pipeline phase runs.

#import "py.typ": falsy, truthy

// flint/core/encoding_overrides.py apply_encoding_overrides
//
// Actions carry a `set` *function* in a dictionary field. Typst functions are
// first-class values and live in dictionaries the same way, so this ports
// directly — the action definitions themselves are backend-supplied and will
// come from the lilaq templates rather than the vegalite ones.
#let apply_encoding_overrides(template, encodings, chart_properties: none) = {
  let actions = template.at("encodingActions", default: none)
  if falsy(actions) or falsy(chart_properties) { return encodings }

  let result = encodings
  for action in actions {
    let key = action.at("key", default: none)
    if key == none { continue }
    // Upstream note: JS tests `override !== undefined`; Python's `.get` cannot
    // distinguish absent from explicitly-None, and the host never sends an
    // explicit None, so "not None" is the equivalent. Typst's `.at(default:)`
    // has the same ambiguity and the same resolution.
    let override_value = chart_properties.at(key, default: none)
    if override_value != none {
      result = (action.at("set"))(result, override_value)
    }
  }
  result
}
