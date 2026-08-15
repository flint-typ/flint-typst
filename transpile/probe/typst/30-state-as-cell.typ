// Can state() serve as a mutable cell inside pure computation?
#let cell = state("cell", (a: 1))
#let mutate() = { cell.update(d => { d.insert("added", 1); d }) }
#mutate()
#context repr(cell.get())
