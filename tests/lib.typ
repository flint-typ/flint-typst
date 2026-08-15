// Shared harness for the visual tests.
//
// These are the counterpart to the value suites in `tests/*.py`: those check
// that core's *decisions* match flint-py, these check that a backend turns
// those decisions into the right picture. The cases live in `cases.typ` and are
// mirrored by every backend, so adding one covers all of them.

#import "cases.typ": CASES

#let setup(body) = {
  set page(width: auto, height: auto, margin: 4pt, fill: white)
  set text(size: 9pt, font: "Libertinus Serif")
  body
}

/// Render one case with the named backend.
///
/// Backends are imported lazily inside the branch so a test only pulls in the
/// one it needs — importing every backend would make each test depend on all
/// of their plotting libraries.
#let render-case(backend, group, name) = {
  let case = CASES.at(group).at(name)
  let enc = case.encodings
  // The same fixture is a colour series on a line chart and a `group` on a
  // grouped bar chart.
  let channel = case.at("series-channel", default: none)
  if channel != none and "color" in enc {
    enc = (..enc, (channel): enc.color)
    let _ = enc.remove("color")
  }
  let args = (
    chart-type: case.chart-type,
    data: case.data,
    encodings: enc,
    semantic-types: case.at("semantic-types", default: (:)),
  )
  if backend == "lilaq" {
    import "/src/lilaq/lib.typ": chart
    chart(..args)
  } else {
    panic("no such backend: " + backend)
  }
}
