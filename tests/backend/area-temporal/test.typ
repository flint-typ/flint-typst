// A temporal axis: coordinates are epoch milliseconds, and the ticks must read
// as dates using the format core resolved for the data's granularity — one
// point per month means "%b", so Jan..Jun rather than 1.578 × 10¹².
#import "/tests/lib.typ": chart, daily, setup
#show: setup
#chart(
  chart-type: "Area Chart",
  data: daily,
  encodings: (x: "Date", y: "Value"),
  semantic-types: (Date: "Date", Value: "Quantity"),
)
