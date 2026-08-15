// Memoization is not free: the cache lookup costs something per call. For a
// CHEAP function on a scalar, is memoizing a win or a loss?
// mode=inline  -> the check written inline, no function call
// mode=fn      -> same check as a top-level function (memoizable)
// distinct=1   -> few distinct args, memo should hit
// distinct=0   -> every arg distinct, memo always misses
#let n = int(sys.inputs.at("n", default: "200000"))
#let mode = sys.inputs.at("mode", default: "fn")
#let distinct = sys.inputs.at("distinct", default: "1")
#let is-num(v) = type(v) == float or type(v) == int
#let arg(i) = if distinct == "1" { calc.rem(i, 50) * 1.0 } else { i * 1.0 }
#let total = {
  let t = 0
  for i in range(n) {
    let v = arg(i)
    let ok = if mode == "fn" { is-num(v) } else { type(v) == float or type(v) == int }
    if ok { t += 1 }
  }
  t
}
#repr(total)
