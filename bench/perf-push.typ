// Is array.push amortized O(1), or does each push copy? If it copies, every
// accumulate-in-a-loop in a ported core is silently O(n^2).
#let n = int(sys.inputs.at("n", default: "10000"))
#let build() = { let a = (); for i in range(n) { a.push(i) }; a.len() }
#repr(build())
