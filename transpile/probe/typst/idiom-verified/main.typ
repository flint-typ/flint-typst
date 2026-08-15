#import "lib_a.typ": *
// 1. snake_case identifiers survive?  2. star-import?  3. module-level dict?
#let r1 = get_registry_entry("a")

// 4. early return, default args, keyword args
#let resolve_format(semantic_type, values: (), use_grouping: true, sign_mode: "") = {
  if semantic_type == "" { return none }
  if values.len() == 0 { return "empty" }
  (st: semantic_type, g: use_grouping, s: sign_mode)
}

// 5. tuple unpacking from a returning function
#let resolve_stretch_caps(o) = (o.at("x", default: 2.0), o.at("y", default: 2.0))
#let (max_x, max_y) = resolve_stretch_caps((x: 3.0))

// 6. dict spread:  python  {**enc, "type": t}
#let enc = (field: "Sales", aggregate: "sum")
#let typed = (..enc, type: "quantitative")

// 7. nested closure capturing an outer local (core has 17 of these)
#let compute_layout(opts) = {
  let default_step = opts.at("step", default: 6)
  let is_discrete_type(t) = t == "nominal" or t == "ordinal"
  (step: default_step, d: is_discrete_type("nominal"))
}

// 8. mutating a nested dict entry in a local structure
#let cs = (x: (field: "A"), y: (field: "B"))
#let cs2 = { let c = cs; c.x.insert("zero", true); c }

#repr((r1: r1, fmt: resolve_format("Price", values: (1,2)), early: resolve_format(""),
       caps: (max_x, max_y), spread: typed, layout: compute_layout((:)),
       nested_mut: cs2.x, original_untouched: cs.x))
