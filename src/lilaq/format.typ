// Core's format decision -> a lilaq tick formatter built on `zero`.
//
// Ours; no upstream counterpart, hence kebab-case throughout.
//
// Core does not format numbers. `resolve_format` decides *how many decimals the
// data warrants* (`_detect_precision`), whether the field is currency, percent
// or unit-suffixed, and supplies the prefix or suffix — then serialises that
// decision as a d3 format specifier, because flint's original backend was
// Vega-Lite.
//
// This reads the decision back out. It deliberately does **not** re-derive the
// digit count by parsing the pattern beyond what the pattern uniquely encodes:
// the pattern is the only serialised form core offers, so it is parsed once
// here and turned into `zero.num` arguments.

#import "@preview/zero:0.7.0" as zero
#import "../core/py.typ": falsy, is-finite, is_number, num-str, truthy

// A d3 format specifier, restricted to the closed set core emits:
//   ",d"  ",.2f"  ".1f"  ".0~%"  ".2%"  "+,.1f"
// group = thousands separator, precision = digits, kind = d | f | % ,
// `~` = trim trailing zeros.
#let _D3 = regex("^([+ -]?)(,?)(?:\\.(\\d+))?(~?)([dfs%])$")

/// Parse a d3 format specifier into the parts `zero` needs.
///
/// Returns `(digits, percent, group, trim, sign)`, or `none` if the pattern is
/// not one core produces — in which case the caller should fall back to
/// lilaq's own formatter rather than guess.
#let parse-d3-pattern(pattern) = {
  if type(pattern) != str { return none }
  let m = pattern.match(_D3)
  if m == none { return none }
  let (sign, group, digits, trim, kind) = m.captures
  (
    digits: if digits == none { if kind == "d" { 0 } else { auto } } else { int(digits) },
    percent: kind == "%",
    group: group == ",",
    trim: trim == "~",
    sign: sign == "+",
  )
}

/// Build a tick formatter for lilaq from core's format decision.
///
/// `format-decision` is the `format` (or `tooltipFormat`) record a channel
/// carries: `(pattern: ".2f", prefix: "$", suffix: "°C")`, any part optional.
/// Returns a function suitable for lilaq's `format-ticks`, or `none` when core
/// expressed no opinion — passing `none` leaves lilaq's own formatter in place,
/// which is the right default rather than a worse guess.
#let tick-formatter(format-decision, digits: none) = {
  if falsy(format-decision) { return none }
  let spec = parse-d3-pattern(format-decision.at("pattern", default: none))
  if spec == none { return none }
  // `digits: auto` lets the caller keep the currency/unit part of a decision
  // while discarding its precision — see the tooltip fallback in render.typ.
  if digits != none { spec.digits = digits }

  let prefix = format-decision.at("prefix", default: none)
  let suffix = format-decision.at("suffix", default: none)

  // `zero` groups digits by default and takes the separator as a dict; d3's
  // `,` flag is the same intent, so a pattern *without* `,` has to turn
  // grouping off rather than the other way round.
  let group-opt = if spec.group {
    (separator: ",", threshold: 4)
  } else {
    (separator: "", threshold: 100)
  }
  let suffix-text = if spec.percent { "%" } else { suffix }

  (ticks, ..args) => ticks.map(value => {
    if not is_number(value) or not is-finite(value) { return [#value] }
    // A percent pattern means the *data* is a fraction and d3 does the
    // multiply, so `zero` sees the scaled number and a literal "%" suffix.
    let shown = if spec.percent { value * 100 } else { value }
    zero.num(
      num-str(shown),
      digits: spec.digits,
      group: group-opt,
      positive-sign: spec.sign,
      prefix: prefix,
      suffix: suffix-text,
    )
  })
}

/// Format one value with the same decision, for labels and legends.
#let format-value(format-decision, value) = {
  let f = tick-formatter(format-decision)
  if f == none { return [#value] }
  f((value,)).first()
}
