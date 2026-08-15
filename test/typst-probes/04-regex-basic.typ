#let s = "camelCaseValue"
#repr((
  m: s.match(regex("([a-z0-9])([A-Z])")),
  all: s.matches(regex("[A-Z]")).len(),
  split: "a1b22c".split(regex("[0-9]+")),
))
