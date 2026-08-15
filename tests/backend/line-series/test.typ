// A colour channel splits the rows into series; each gets its own line and a
// legend entry.
#import "/tests/lib.typ": chart, months, setup
#show: setup
#chart(
  chart-type: "Line Chart",
  data: months,
  encodings: (x: "Month", y: "Revenue", color: "Region"),
  semantic-types: (Month: "Month", Revenue: "Amount"),
)
