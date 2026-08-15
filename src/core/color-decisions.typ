// Port of flint/core/color_decisions.py.

#import "py.typ": falsy, truthy

// flint/core/color_decisions.py _infer_color_channel_primary
#let _infer_color_channel_primary(channel, chart_type) = {
  if channel == "color" or channel == "group" { return true }
  false
}

// flint/core/color_decisions.py _decide_scheme_type_from_channel
#let _decide_scheme_type_from_channel(channel, cs) = {
  let cs = if cs == none { (:) } else { cs }
  let hint = cs.at("colorScheme", default: none)
  if truthy(hint) {
    if hint.at("type", default: none) == "diverging" {
      return (schemeType: "diverging", divergingMidpoint: hint.at("domainMid", default: none))
    }
    if hint.at("type", default: none) == "sequential" {
      return (schemeType: "sequential")
    }
    if hint.at("type", default: none) == "categorical" {
      let sem_type = cs.at("semanticAnnotation", default: (:)).at("semanticType", default: none)
      let is_rank_like = sem_type == "Rank"
      if is_rank_like { return (schemeType: "sequential") }
      if cs.at("type", default: none) == "temporal" and channel == "color" {
        return (schemeType: "sequential")
      }
      return (schemeType: "categorical")
    }
  }

  let enc_type = cs.at("type", default: none)
  let sem_type = cs.at("semanticAnnotation", default: (:)).at("semanticType", default: none)
  if sem_type == "Correlation" {
    return (schemeType: "diverging", divergingMidpoint: 0)
  }
  if enc_type == "quantitative" or enc_type == "temporal" {
    return (schemeType: "sequential")
  }
  (schemeType: "categorical")
}

// flint/core/color_decisions.py _count_distinct_values
//
// PORT-IDIOM: upstream builds a Python `set`; Typst dictionary keys must be
// strings, so `array.dedup()` is the faithful equivalent — it compares with
// `==` rather than hashing, and Typst's `==` agrees with Python's on the value
// kinds that reach here.
#let _count_distinct_values(table, field) = {
  if falsy(field) { return none }
  let seen = ()
  for row in table {
    if row == none { continue }
    seen.push(row.at(field, default: none))
  }
  seen.dedup().len()
}

// flint/core/color_decisions.py _decide_color_for_channel
#let _decide_color_for_channel(channel, ctx) = {
  let encoding = ctx.encodings.at(channel, default: none)
  let cs = ctx.channelSemantics.at(channel, default: none)
  if falsy(encoding) or falsy(cs) or falsy(cs.at("field", default: none)) { return none }

  let data_driven = true
  let primary = _infer_color_channel_primary(channel, ctx.chartType)

  let scheme = encoding.at("scheme", default: none)
  if truthy(scheme) and scheme != "default" {
    let distinct = _count_distinct_values(ctx.table, cs.at("field", default: none))
    let scheme_info = _decide_scheme_type_from_channel(channel, cs)
    return (
      channel: channel,
      schemeType: scheme_info.schemeType,
      schemeId: scheme,
      categoryCount: distinct,
      primary: primary,
      dataDriven: data_driven,
    )
  }

  let scheme_info = _decide_scheme_type_from_channel(channel, cs)
  let distinct = _count_distinct_values(ctx.table, cs.at("field", default: none))

  let out = (
    channel: channel,
    schemeType: scheme_info.schemeType,
    categoryCount: distinct,
    primary: primary,
    dataDriven: data_driven,
  )
  if scheme_info.at("divergingMidpoint", default: none) != none {
    out.insert("divergingMidpoint", scheme_info.divergingMidpoint)
  }
  out
}

// flint/core/color_decisions.py decide_color_maps
#let decide_color_maps(ctx) = {
  let result = (color: none, group: none, fill: none, stroke: none)
  for ch in ("color", "group") {
    let decision = _decide_color_for_channel(ch, ctx)
    if truthy(decision) { result.insert(ch, decision) }
  }
  result
}
