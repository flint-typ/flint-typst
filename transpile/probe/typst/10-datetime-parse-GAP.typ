// flint needs: parse ISO string -> date, epoch seconds <-> date, UTC normalize
#let probes = (:)
// A: parse from string?
#let a = datetime("2020-03-14")
#repr(a)
