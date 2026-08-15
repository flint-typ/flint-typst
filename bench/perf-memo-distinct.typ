// Control: same work, but each call gets a DIFFERENT array, so memoization
// cannot apply. The gap between this and perf-memo.typ is the memo benefit.
#let n = int(sys.inputs.at("n", default: "3000"))
#let reps = int(sys.inputs.at("reps", default: "20"))
#let analyse(c) = {
  let s = (:)
  for v in c { s.insert(str(v), true) }
  (distinct: s.keys().len(), max: calc.max(..c), n: c.len())
}
#let out = range(reps).map(k => analyse(range(n).map(i => calc.rem(i, 97) * 1.0 + k)))
#repr(out.first())
