// The pattern a ported compute_layout would need: accumulate across a loop
// and read the result back. Every update is lost. Expected: total == 0.
// structure across a loop, reading intermediate results back.
#let acc = state("acc", (total: 0))
#context {
  for i in range(5) {
    acc.update(d => { d.total += i; d })
  }
  repr(acc.get())
}
