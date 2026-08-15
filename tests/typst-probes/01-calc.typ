#let r = (
  floor: calc.floor(3.7), ceil: calc.ceil(3.2), sqrt: calc.sqrt(2.0),
  exp: calc.exp(1.0), ln: calc.ln(10.0), log10: calc.log(100.0),
  log2: calc.log(8.0, base: 2), pow: calc.pow(2.0, 10),
  abs: calc.abs(-3), rem: calc.rem(7, 3), max: calc.max(1,5,3), min: calc.min(1,5,3),
)
#repr(r)
