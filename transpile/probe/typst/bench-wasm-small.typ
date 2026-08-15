// Same 200 distinct charts through the wasm boundary.
#let core = plugin("flint_core_wasm.wasm")
#let once(seed) = {
  let rows = range(32).map(i => (Date: "2020-01-" + str(seed), Value: float(i + seed), Category: "c" + str(calc.rem(i, 4))))
  json(core.compute_layout(bytes(json.encode((args: (rows, (:)))))))
}
#repr((calls: range(200).map(s => once(s)).len(),))
