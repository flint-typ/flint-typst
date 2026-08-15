// Inside a `context` block state.get() yields a plain value, so arithmetic
// works. But an update made in the SAME block is NOT visible to a later read:
// a context sees one snapshot, keyed to document position. Expected: after == 5.
// works. Does an update made in the SAME block become visible to a later read?
#let cell = state("c3", 5)
#context {
  let before = cell.get()
  cell.update(v => v * 10)
  let after = cell.get()
  repr((before: before, after: after, arithmetic-ok: before * 2))
}
