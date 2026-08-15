#let n = int(sys.inputs.at("n", default: "10000"))
#let build() = { let d = (:); for i in range(n) { d.insert(str(i), i) }; d.len() }
#repr(build())
