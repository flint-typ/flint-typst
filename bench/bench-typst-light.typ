// Realistic core-ish workload: dict-of-Any row access, the dominant pattern
// (379 `.get()` sites in flint core), over a fixture-sized table.
#let n = 3000
#let rows = range(n).map(i => (
  Date: "2020-01-01", Value: float(calc.rem(i, 97)),
  Category: "cat" + str(calc.rem(i, 20)), Extra: i,
))
#let field-values(rows, key) = rows.map(r => r.at(key, default: none))
#let distinct(vals) = {
  let seen = (:)
  for v in vals { seen.insert(str(v), true) }
  seen.keys().len()
}
#let stats(rows, key) = {
  let vs = field-values(rows, key).filter(v => type(v) == float)
  if vs.len() == 0 { return (min: none, max: none) }
  (min: calc.min(..vs), max: calc.max(..vs), n: vs.len())
}
#let out = (
  distinct-cat: distinct(field-values(rows, "Category")),
  distinct-date: distinct(field-values(rows, "Date")),
  vstats: stats(rows, "Value"),
)
#repr(out)
