#let n = int(sys.inputs.at("n", default: "10000"))
#let build() = { let a = (); for i in range(n) { a = a + (i,) }; a.len() }
#repr(build())
