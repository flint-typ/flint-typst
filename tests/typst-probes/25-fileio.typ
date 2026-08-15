// Typst can read JSON from disk, so a pure-Typst core could be replayed
// against the same 705-case conformance corpus the Rust route uses.
#let c = json("fixture.json")
#repr((slug: c.slug, stages: c.calls.keys().len()))
