// The group channel splits into one bar series per region, and the legend
// appears because there is more than one series.
#import "/tests/lib.typ": chart, months, setup
#show: setup
#chart(
  chart-type: "Grouped Bar Chart",
  data: months,
  encodings: (x: "Month", y: "Revenue", group: "Region"),
  semantic-types: (Month: "Month", Revenue: "Amount"),
)
