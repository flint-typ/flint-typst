// 200 DISTINCT charts of median fixture size (32 rows). Varying the data per
// iteration defeats Typst's memoization, which otherwise collapses identical
// calls to one evaluation and makes any repeat-benchmark meaningless.
#let distinct(vs) = { let s = (:); for v in vs { s.insert(str(v), true) }; s.keys().len() }
#let once(seed) = {
  let rows = range(32).map(i => (Date: "2020-01-" + str(seed), Value: float(i + seed), Category: "c" + str(calc.rem(i, 4))))
  let t = 0
  for pass in range(30) {
    for key in ("Date", "Value", "Category") {
      let vs = rows.map(r => r.at(key, default: none))
      t += distinct(vs)
      let nums = vs.filter(v => type(v) == float)
      if nums.len() > 0 { t += int(calc.max(..nums)) }
    }
  }
  t
}
#repr((total: range(200).map(s => once(s)).sum(),))
