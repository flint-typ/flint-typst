// Can a state-backed value be used as a plain return value in arithmetic?
#let cell = state("c2", 5)
#let compute() = { cell.get() * 2 }
#repr(compute())
