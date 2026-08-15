// Shared preamble for the visual tests.
//
// These are the counterpart to the value-based suites in `test/`: those check
// that core's *decisions* match flint-py, these check that the lilaq backend
// turns those decisions into the right picture. Each test targets a specific
// decision rather than "a chart appeared", so a diff points at a cause.

#import "/src/lib.typ": chart, plan-for

#let setup(body) = {
  set page(width: auto, height: auto, margin: 4pt, fill: white)
  set text(size: 9pt, font: "Libertinus Serif")
  body
}

/// Categorical sales, deliberately *not* in calendar order — core should
/// restore Jan/Feb/Mar from the Month semantic type.
#let months = (("Month", "Revenue", "Region"),
  ("Mar", 14200.0, "North"), ("Jan", 12000.0, "North"), ("Feb", 15500.0, "North"),
  ("Mar", 13200.0, "South"), ("Jan", 9000.0, "South"),  ("Feb", 11500.0, "South"))

/// A short daily time series.
#let daily = (("Date", "Value"),
  ("2020-01-01", 12.0), ("2020-02-01", 15.5), ("2020-03-01", 14.2),
  ("2020-04-01", 19.8), ("2020-05-01", 21.3), ("2020-06-01", 20.1))
