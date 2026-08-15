// Real-core benchmark: the full Phase-0/Phase-1 pipeline on N distinct charts.
// Data varies per chart so Typst's memoisation cannot collapse them.
#import "../src/core/resolve-semantics.typ": convert_temporal_data, resolve_channel_semantics
#import "../src/core/compute-layout.typ": compute_channel_budgets, compute_layout
#import "../src/core/filter-overflow.typ": filter_overflow

#let n = int(sys.inputs.at("n", default: "20"))
#let pts = int(sys.inputs.at("pts", default: "32"))
#let series = int(sys.inputs.at("series", default: "1"))

#let one(seed) = {
  let rows = range(pts).map(i => (
    Date: "2020-" + str(10 + calc.rem(i + seed, 2)) + "-" + str(10 + calc.rem(i, 19)),
    Value: float(calc.rem(i * 7 + seed, 97)),
    Series: "s" + str(calc.rem(i, series)),
  ))
  let enc = (
    x: (field: "Date"),
    y: (field: "Value"),
    color: if series > 1 { (field: "Series") } else { (:) },
  )
  let st = (Date: "Date", Value: "Amount")
  let converted = convert_temporal_data(rows, st)
  let cs = resolve_channel_semantics(enc, rows, st, converted_data: converted)
  let canvas = (width: 400.0, height: 320.0)
  let budgets = compute_channel_budgets(cs, (:), converted, canvas, (:))
  let of = filter_overflow(cs, (:), enc, converted, budgets, ("line",))
  let layout = compute_layout(
    cs, (:), of.filteredData, canvas,
    options: (:), facet_grid: budgets.facetGrid,
  )
  layout.subplotWidth + layout.subplotHeight
}
#repr(range(n).map(one).sum())
