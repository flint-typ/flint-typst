// The format decision, end to end: semantic type Amount plus unit USD makes
// core emit a currency format, which the backend turns into `zero.num` calls.
// Ticks must read "$12,000" — grouped, prefixed, and with no decimals because
// `_detect_precision` sees whole values.
#import "/tests/lib.typ": chart, months, setup
#show: setup
#chart(
  chart-type: "Bar Chart",
  data: months.slice(0, 4),
  encodings: (x: "Month", y: "Revenue"),
  semantic-types: (Month: "Month", Revenue: (semanticType: "Amount", unit: "USD")),
)
