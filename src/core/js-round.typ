// Port of flint/core/__init__.py.
//
// Upstream keeps this in the package `__init__`; here it gets its own module
// so the import graph stays explicit.

// flint/core/__init__.py js_round
//
// PORT-NUM: upstream uses `math.floor(x + 0.5)` to match JS `Math.round`,
// which rounds half-values toward +infinity. Typst's `calc.round` rounds half
// *away from zero* (`calc.round(-2.5) == -3`), so it must not be used here:
// `Math.round(-2.5)` is -2. `calc.floor(x + 0.5)` matches.
#let js_round(x) = {
  if type(x) == bool { return if x { 1 } else { 0 } }
  if x == none { return 0 }
  int(calc.floor(x + 0.5))
}
