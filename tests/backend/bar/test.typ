// A bar chart exercises three core decisions at once:
//   * the category axis is *banded* (declareLayoutMode)
//   * ticks read Jan/Feb/Mar although the data is Mar/Jan/Feb — the canonical
//     ordering core infers from the Month semantic type
//   * bars start at zero, because Amount's zeroBaseline is "meaningful" and a
//     bar reads by length
#import "/tests/lib.typ": chart, months, setup
#show: setup
#chart(
  chart-type: "Bar Chart",
  data: months.slice(0, 4),
  encodings: (x: "Month", y: "Revenue"),
  semantic-types: (Month: "Month", Revenue: "Amount"),
)
