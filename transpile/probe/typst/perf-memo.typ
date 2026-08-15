// Does Typst memoize a user function called repeatedly with the same large
// array argument? If so, a column-analysis pass shared across channels is free
// after the first call -- and core calls resolve_channel_semantics twice.
#let n = int(sys.inputs.at("n", default: "3000"))
#let reps = int(sys.inputs.at("reps", default: "20"))
#let col = range(n).map(i => calc.rem(i, 97) * 1.0)
#let analyse(c) = {
  let s = (:)
  for v in c { s.insert(str(v), true) }
  (distinct: s.keys().len(), max: calc.max(..c), n: c.len())
}
#repr(range(reps).map(i => analyse(col)).first())
