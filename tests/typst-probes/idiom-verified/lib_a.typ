#let SOME_TABLE = (a: (t0: "x", t1: "y"), b: (t0: "p", t1: "q"))
#let get_registry_entry(t) = SOME_TABLE.at(t, default: none)
#let _private_helper(v) = v * 2
