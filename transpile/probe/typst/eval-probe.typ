#let compute_layout(args) = (width: 400, height: 300)
#let filter_overflow(args) = (filteredData: (), warnings: ())
#let run_case(case) = (
  slug: case.slug,
  calls: case.calls.pairs().fold((:), (acc, p) => {
    let (stage, calls) = p
    let f = if stage == "compute_layout" { compute_layout } else if stage == "filter_overflow" { filter_overflow } else { none }
    acc.insert(stage, if f == none { ((__error__: "unimplemented"),) } else { calls.map(c => (result: f(c))) })
    acc
  }),
)
