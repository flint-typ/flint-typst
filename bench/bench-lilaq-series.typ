// Same total point count, split across S series -- the shape of the fixtures
// where core's per-row cost jumps (100 dates x 60 series = 6000 rows).
#import "@preview/lilaq:0.5.0" as lq
#let n = int(sys.inputs.at("n", default: "5"))
#let total = int(sys.inputs.at("total", default: "6000"))
#let s = int(sys.inputs.at("s", default: "60"))
#let per = calc.div-euclid(total, s)
#for c in range(n) [
  #lq.diagram(width: 6cm, height: 4cm,
    ..range(s).map(k => lq.plot(
      range(per).map(i => i + c * 0.001),
      range(per).map(i => calc.sin(i / 3.0 + k) * 10),
    )))
]
