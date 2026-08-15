#import "../src/lib.typ": chart, plan-for
#set page(width: 21cm, height: auto, margin: 1cm)
#set text(size: 9pt)

= flint-typst → lilaq

#let sales = (("Month", "Revenue", "Region"),
  ("Jan", 12000.0, "North"), ("Feb", 15500.0, "North"), ("Mar", 14200.0, "North"),
  ("Jan", 9000.0, "South"),  ("Feb", 11500.0, "South"), ("Mar", 13200.0, "South"))
#let ts = (("Date", "Value"),
  ("2020-01-01", 12.0), ("2020-02-01", 15.5), ("2020-03-01", 14.2),
  ("2020-04-01", 19.8), ("2020-05-01", 21.3), ("2020-06-01", 20.1))

== Bar Chart — banded x, aggregated, currency ticks
#chart(chart-type: "Bar Chart", data: sales.slice(0,4),
  encodings: (x: "Month", y: "Revenue"),
  semantic-types: (Month: "Month", Revenue: (semanticType: "Amount", unit: "USD")))

== Grouped Bar Chart — series on group
#chart(chart-type: "Grouped Bar Chart", data: sales,
  encodings: (x: "Month", y: "Revenue", group: "Region"),
  semantic-types: (Month: "Month", Revenue: "Amount"))

== Line Chart — temporal x, multi-series
#chart(chart-type: "Line Chart", data: sales,
  encodings: (x: "Month", y: "Revenue", color: "Region"),
  semantic-types: (Month: "Month", Revenue: "Amount"))

== Area Chart — temporal
#chart(chart-type: "Area Chart", data: ts,
  encodings: (x: "Date", y: "Value"),
  semantic-types: (Date: "Date", Value: "Quantity"))

== Scatter Plot
#chart(chart-type: "Scatter Plot", data: ts,
  encodings: (x: "Date", y: "Value"),
  semantic-types: (Date: "Date", Value: "Quantity"))
