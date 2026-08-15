// Backend smoke test: every registered chart type must assemble and render.
//
// This is *not* a visual test — it asserts on the plan, not on pixels, so it
// runs in the same value-based suite as everything else. Reference-image tests
// belong in tytanic once the backend stabilises.

#import "../src/lib.typ": chart, plan-for
#import "../src/lilaq/templates/lib.typ": supported-charts

#let data = (("Cat", "Value", "Series"),
  ("Jan", 12.0, "A"), ("Feb", 15.0, "A"), ("Mar", 9.0, "A"),
  ("Jan", 7.0, "B"), ("Feb", 11.0, "B"), ("Mar", 14.0, "B"))

// Every registered chart type assembles, produces a layout, and renders.
#for ct in supported-charts {
  let enc = if ct == "Grouped Bar Chart" {
    (x: "Cat", y: "Value", group: "Series")
  } else {
    (x: "Cat", y: "Value", color: "Series")
  }
  let plan = plan-for(
    chart-type: ct, data: data, encodings: enc,
    semantic-types: (Cat: "Month", Value: "Amount"),
  )
  assert(plan.chartType == ct, message: ct + ": wrong chart type in plan")
  assert(plan.layout.subplotWidth > 0, message: ct + ": no width")
  assert(plan.layout.subplotHeight > 0, message: ct + ": no height")
  assert("x" in plan.channelSemantics, message: ct + ": x channel not resolved")
  // The canonical month order core resolved must survive into the plan.
  assert(
    plan.channelSemantics.x.at("ordinalSortOrder", default: none) != none,
    message: ct + ": lost the canonical month ordering",
  )
  // And it must actually draw.
  let _ = chart(
    chart-type: ct, data: data, encodings: enc,
    semantic-types: (Cat: "Month", Value: "Amount"),
  )
}

// An unknown chart type must fail loudly rather than draw something wrong.
// (Checked by inspection; `assert` cannot catch in Typst.)

All backend smoke checks passed.
