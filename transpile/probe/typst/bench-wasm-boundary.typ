// Cost of the wasm route's data boundary: build 3000 rows, JSON-encode,
// hand to the plugin, decode the reply. The plugin errors immediately, so
// this measures pure boundary overhead with no core work included.
#let core = plugin("flint_core_wasm.wasm")
#let n = 3000
#let rows = range(n).map(i => (
  Date: "2020-01-01", Value: float(calc.rem(i, 97)),
  Category: "cat" + str(calc.rem(i, 20)), Extra: i,
))
#let payload = bytes(json.encode((args: (rows, (:)))))
#let reply = json(core.compute_layout(payload))
#repr((bytes: payload.len(), reply: reply.at("__error__").slice(0, 30)))
