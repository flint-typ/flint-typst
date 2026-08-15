// How long does lilaq itself take to render a chart? Data varies per chart so
// Typst's memoization cannot collapse them. N and PTS are set via --input.
#import "@preview/lilaq:0.5.0" as lq
#let n = int(sys.inputs.at("n", default: "20"))
#let pts = int(sys.inputs.at("pts", default: "32"))
#for s in range(n) [
  #lq.diagram(
    width: 6cm, height: 4cm,
    lq.plot(
      range(pts).map(i => i + s * 0.001),
      range(pts).map(i => calc.sin(i / 3.0 + s) * 10 + calc.rem(i, 7)),
    ),
  )
]
