// Conformance transport: can a Typst run emit structured results that the
// harness reads back? `typst query` reads metadata out without a PDF.
#let case = json("fixture.json")
#let fake_stage(args) = (width: 400, height: 300, note: "from typst")
#metadata((
  slug: case.slug,
  results: case.calls.keys().map(k => (stage: k, out: fake_stage(()))),
)) <conformance>
