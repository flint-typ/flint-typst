// The real hot spot: js_date_parse_ms -- regex work on a string scalar,
// called 180k times over only 1181 distinct values (152x redundant).
// Is Typst's automatic memoization enough, or is manual dedup needed?
#let n = int(sys.inputs.at("n", default: "100000"))
#let distinct = int(sys.inputs.at("distinct", default: "1181"))
#let mode = sys.inputs.at("mode", default: "fn")

#let ISO = regex("^(\\d{4})-(\\d{2})-(\\d{2})")
#let parse-date(s) = {
  let m = s.match(ISO)
  if m == none { return none }
  let y = int(m.captures.at(0)); let mo = int(m.captures.at(1)); let d = int(m.captures.at(2))
  (y * 10000 + mo * 100 + d)
}
#let dates = range(distinct).map(i => "20" + str(20 + calc.rem(i, 5)) + "-" + str(10 + calc.rem(i, 2)) + "-" + str(10 + calc.rem(i, 28)))
#let total = {
  let t = 0
  for i in range(n) {
    let s = dates.at(calc.rem(i, distinct))
    t += parse-date(s)
  }
  t
}
#repr(total)
