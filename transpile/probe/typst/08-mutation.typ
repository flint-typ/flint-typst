#let f() = {
  let a = ()
  for i in range(5) { a.push(i * 2) }
  let d = (:)
  d.insert("k", 1)
  d.k += 10
  let total = 0
  let i = 0
  while i < 5 { total += i; i += 1 }
  (arr: a, dict: d, total: total)
}
#repr(f())
