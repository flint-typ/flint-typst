// Python: compute_layout mutates channel_semantics in place and the caller
// sees it. Typst dictionaries are values, not references -- does the mutation
// escape the callee?
#let mutate(d) = { d.insert("added", 1); d }
#let outer = (a: 1)
#let _ = mutate(outer)
#let nested = (x: (y: 1))
#let inner = nested.x
#let inner2 = { let t = inner; t.insert("y", 99); t }
#repr((
  caller-saw-mutation: "added" in outer,
  original-nested-untouched: nested.x.y,
  copy-changed: inner2.y,
))
