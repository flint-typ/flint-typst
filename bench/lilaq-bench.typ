// lilaq rendering cost, matching the shapes in core-bench.typ.
#import "@preview/lilaq:0.6.0" as lq
#let n = int(sys.inputs.at("n", default: "20"))
#let pts = int(sys.inputs.at("pts", default: "32"))
#let series = int(sys.inputs.at("series", default: "1"))
#let per = calc.div-euclid(pts, series)
#for c in range(n) [
  #lq.diagram(width: 6cm, height: 4cm,
    ..range(series).map(k => lq.plot(
      range(per).map(i => i + c * 0.001),
      range(per).map(i => calc.sin(i / 3.0 + k) * 10),
    )))
]
