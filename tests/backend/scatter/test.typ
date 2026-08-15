// A scatter's marks read by *position*, not length, so core does not force the
// y axis to zero — the axis fits the data. Contrast with the bar tests.
#import "/tests/lib.typ": chart, daily, setup
#show: setup
#chart(
  chart-type: "Scatter Plot",
  data: daily,
  encodings: (x: "Date", y: "Value"),
  semantic-types: (Date: "Date", Value: "Quantity"),
)
