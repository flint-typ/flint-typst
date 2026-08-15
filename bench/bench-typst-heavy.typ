// Heavier: ~30 passes over the table, mimicking per-channel semantic
// resolution + layout stats that flint core runs for every encoding channel.
#let n = 3000
#let rows = range(n).map(i => (
  Date: "2020-01-01", Value: float(calc.rem(i, 97)),
  Category: "cat" + str(calc.rem(i, 20)), Extra: i,
))
#let distinct(vals) = { let s = (:); for v in vals { s.insert(str(v), true) }; s.keys().len() }
#let acc = {
  let total = 0
  for pass in range(30) {
    for key in ("Date", "Value", "Category", "Extra") {
      let vs = rows.map(r => r.at(key, default: none))
      total += distinct(vs)
      let nums = vs.filter(v => type(v) == float or type(v) == int)
      if nums.len() > 0 { total += int(calc.max(..nums.map(x => float(x)))) }
    }
  }
  total
}
#repr(acc)
