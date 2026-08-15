// The visual test cases, defined once and rendered by every backend.
//
// A case says *what chart to draw*, never how — so adding a backend means
// mirroring this list under `tests/<backend>/`, not writing new cases, and
// adding a case covers every backend at once. The pictures will differ in
// styling between backends; what must not differ is which decisions core made,
// and each case names the decision it is there for.
//
// Two groups:
//
//   simple/  hand-made rows, so the expected result is obvious by inspection
//   scale/   real inputs lifted from flint's own fixture corpus
//            (`tests/fixtures.typ`, generated) — the same data the conformance
//            suite checks core's decisions against, so a visual test on one is
//            testing the backend and nothing else. This is where tick density,
//            label collision and overflow actually misbehave.

#import "fixtures.typ": bar-20, bar-grouped, line-200, line-sparse, scatter-150

/// Categorical sales, deliberately *not* in calendar order — core should
/// restore Jan/Feb/Mar from the Month semantic type.
#let months = (("Month", "Revenue", "Region"),
  ("Mar", 14200.0, "North"), ("Jan", 12000.0, "North"), ("Feb", 15500.0, "North"),
  ("Mar", 13200.0, "South"), ("Jan", 9000.0, "South"),  ("Feb", 11500.0, "South"))

/// A short monthly series.
#let daily = (("Date", "Value"),
  ("2020-01-01", 12.0), ("2020-02-01", 15.5), ("2020-03-01", 14.2),
  ("2020-04-01", 19.8), ("2020-05-01", 21.3), ("2020-06-01", 20.1))

#let CASES = (
  simple: (
    bar: (
      why: "banded category axis; ticks read Jan/Feb/Mar although the rows are Mar/Jan/Feb; bars start at zero because a bar reads by length",
      chart-type: "Bar Chart",
      data: months.slice(0, 4),
      encodings: (x: "Month", y: "Revenue"),
      semantic-types: (Month: "Month", Revenue: "Amount"),
    ),
    currency-zero: (
      why: "Amount plus unit USD makes core emit a currency format, so ticks read $2,500 — grouped, prefixed, and with no decimals because the values are whole",
      chart-type: "Bar Chart",
      data: months.slice(0, 4),
      encodings: (x: "Month", y: "Revenue"),
      semantic-types: (Month: "Month", Revenue: (semanticType: "Amount", unit: "USD")),
    ),
    bar-zero: (
      why: "a category whose value is zero — a bar reads by length, so its axis stays pinned to the baseline and the empty bar sits on it rather than floating above a padded axis",
      chart-type: "Bar Chart",
      data: (("Month", "Revenue"), ("Jan", 12000.0), ("Feb", 0.0), ("Mar", 15500.0)),
      encodings: (x: "Month", y: "Revenue"),
      semantic-types: (Month: "Month", Revenue: "Amount"),
    ),
    grouped-bar: (
      why: "the group channel dodges the series within each band rather than drawing them on top of one another",
      chart-type: "Grouped Bar Chart",
      data: months,
      encodings: (x: "Month", y: "Revenue", group: "Region"),
      semantic-types: (Month: "Month", Revenue: "Amount"),
    ),
    line-series: (
      why: "a colour channel splits the rows into series; each line runs in x order rather than row order, and a line axis gets no band padding",
      chart-type: "Line Chart",
      data: months,
      encodings: (x: "Month", y: "Revenue", color: "Region"),
      semantic-types: (Month: "Month", Revenue: "Amount"),
    ),
    area-temporal: (
      why: "temporal coordinates are epoch milliseconds; a 6-month span picks months as its tick unit, so every point gets a tick and none is invented between them",
      chart-type: "Area Chart",
      data: daily,
      encodings: (x: "Date", y: "Value"),
      semantic-types: (Date: "Date", Value: "Quantity"),
    ),
    scatter: (
      why: "a scatter reads by position, so core does not force zero — the axis fits the data, unlike the bar cases",
      chart-type: "Scatter Plot",
      data: daily,
      encodings: (x: "Date", y: "Value"),
      semantic-types: (Date: "Date", Value: "Quantity"),
    ),
  ),

  scale: (
    line-200: (
      why: "200 points across 4 series on a temporal axis — ticks must fall on calendar boundaries (half-years here) rather than on sampled data points, and be labelled at the granularity of that boundary",
      ..line-200,
      chart-type: "Line Chart",
    ),
    line-sparse: (
      why: "3 series that do not share x positions; each line spans only its own points, and a 3-year span picks whole years as its tick unit",
      ..line-sparse,
      chart-type: "Line Chart",
    ),
    bar-20: (
      why: "20 categories — the labels stop fitting the band and have to angle, each anchored by its end so it stays over its own bar",
      ..bar-20,
      chart-type: "Bar Chart",
    ),
    bar-grouped: (
      why: "5 categories by 3 series — dodging beyond the two-series case",
      ..bar-grouped,
      chart-type: "Grouped Bar Chart",
      series-channel: "group",
    ),
    scatter-150: (
      why: "150 points, quantitative on both axes — nothing is banded anywhere",
      ..scatter-150,
      chart-type: "Scatter Plot",
    ),
  ),
)
