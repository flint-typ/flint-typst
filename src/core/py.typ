// Porting primitives — ours, not upstream.
//
// flint-py leans on Python's truthiness constantly (`if not encoding`,
// `if hint`, `if not actions or not chart_properties`). Typst has no
// truthiness at all: `if` requires an actual boolean, and `not none` is a type
// error. Translating each site by hand invites drift, so the rule is encoded
// once here and every port site reads `truthy(x)`.
//
// Named without a hyphen despite being ours, because it appears inline in
// ported expressions constantly and `truthy` reads better there than
// `is-truthy`. Everything else we add keeps the kebab-case marker.

/// Python's truthiness, exactly.
///
/// Falsy: `none`, `false`, `0`, `0.0`, `""`, `()`, `(:)`.
/// Everything else is truthy — including `"0"` and `"false"`, which are
/// non-empty strings.
#let truthy(value) = {
  if value == none { return false }
  let t = type(value)
  if t == bool { return value }
  if t == int { return value != 0 }
  if t == float { return value != 0.0 }
  if t == str { return value != "" }
  if t == array { return value.len() != 0 }
  if t == dictionary { return value.len() != 0 }
  true
}

/// `not truthy(value)`, for the very common `if not x` shape.
#let falsy(value) = not truthy(value)

/// Python's `isinstance(v, (int, float))` excluding `bool`.
///
/// Upstream checks `isinstance(v, bool)` *before* the numeric test in several
/// places, because in Python `bool` is a subclass of `int` and `True` would
/// otherwise be read as the number 1. Typst keeps `bool` and `int` distinct,
/// so this is simply the two numeric types — but the ordering of checks is
/// preserved at each port site anyway, so the two files still read alike.
#let is_number(value) = {
  let t = type(value)
  t == int or t == float
}

/// Is `value` NaN?
///
/// Wraps Typst's `float.is-nan()`. The type test is part of the predicate for
/// two reasons: the method exists only on `float` (an `int` has none), and
/// coercing with `float(value)` first is not an option — these predicates are
/// called on raw column values, where `float("abc")`, `float(none)` and
/// `float(())` all raise an uncatchable error, and `float(true)` would quietly
/// admit bools that upstream excludes.
#let is-nan(value) = type(value) == float and value.is-nan()

/// Is `value` a finite number? (`math.isfinite`, excluding bools.)
///
/// Every Typst int is finite; a float must be neither NaN nor an infinity.
/// `bool` is excluded, matching upstream's `isinstance` checks which always
/// test `bool` before the numeric types — and see `is-nan` for why this cannot
/// be written as `float(value).is-infinite()`.
#let is-finite(value) = {
  let t = type(value)
  t == int or (t == float and not value.is-nan() and not value.is-infinite())
}

/// Typst's `str()` on a number, with an ASCII minus.
///
/// PORT-NUM. Typst renders a negative number with U+2212 MINUS SIGN, not
/// U+002D HYPHEN-MINUS -- `str(-2.5)` is not `"-2.5"`. That is right for
/// typesetting and wrong for every string comparison, JSON payload and data
/// value in this port. lilaq hits the same thing and patches it the same way
/// (`src/logic/tick-format.typ`).
///
/// Every stringification of a number in the port goes through here.
#let num-str(v) = str(v).replace(sym.minus, "-")

/// Python's `str(v)`, which is *not* Typst's `str(v)`.
///
/// PORT-NUM. The divergence that matters: Typst renders an integral float
/// without a fractional part, Python keeps one.
///
/// | value | Typst `str` | Python `str` |
/// |---|---|---|
/// | `1.0` | `"1"` | `"1.0"` |
/// | `1` | `"1"` | `"1"` |
/// | `1.5` | `"1.5"` | `"1.5"` |
///
/// This is load-bearing wherever upstream stringifies a value and compares it
/// against a label table — `_match_sequence` checks month numbers against
/// `("1", ..., "12")`, so a float `1.0` must *not* match, and under Typst's
/// `str` it would.
///
/// Not covered: Python switches to exponent form at 1e16 and 1e-5 with its own
/// thresholds. No fixture reaches that range through a stringifying path; if
/// one ever does, this is where to fix it.
#let py_str(value) = {
  if value == none { return "None" }
  let t = type(value)
  if t == bool { return if value { "True" } else { "False" } }
  if t == float {
    if value.is-nan() { return "nan" }
    if value.is-infinite() { return if value > 0 { "inf" } else { "-inf" } }
    let s = num-str(value)
    // Typst drops the fractional part of an integral float; Python does not.
    if not s.contains(".") and not s.contains("e") { return s + ".0" }
    return s
  }
  if t == int { return num-str(value) }
  str(value)
}

/// Python's `float(s)` acceptance test, as a look-before-you-leap guard.
///
/// PORT-EXC. Upstream writes `try: float(s) / except ValueError: ...` in
/// several places. Typst cannot catch, and a failed `float()` aborts the whole
/// document, so the shape is checked first. Accepts exactly what Python's
/// `float()` does: optional sign, decimal or exponent form, and the special
/// words `inf`, `infinity`, `nan`.
#let _FLOAT_RE = regex("(?i)^[+-]?(\\d+\\.?\\d*|\\.\\d+)(e[+-]?\\d+)?$")
#let _FLOAT_WORD_RE = regex("(?i)^[+-]?(inf|infinity|nan)$")
#let is-float-string(s) = {
  if type(s) != str { return false }
  s.match(_FLOAT_RE) != none or s.match(_FLOAT_WORD_RE) != none
}
