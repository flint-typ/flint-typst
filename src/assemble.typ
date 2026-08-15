// Port of the backend-agnostic half of flint/vegalite/assemble.py.
//
// Upstream mixes the pipeline coordinator and the Vega-Lite spec builder in one
// file. Everything up to and including `compute_layout` is backend-agnostic —
// it decides *what the chart means and how big its parts are* — and only the
// last phase turns that into a spec. That split is made explicit here:
// `assemble` runs the shared phases and returns a **chart plan**; a backend
// turns the plan into content.
//
// PORT-IDIOM: functions ported from `assemble.py` keep upstream's `snake_case`;
// the plan record and anything with no upstream counterpart is `kebab-case`,
// per docs/PORT-PLAN.md §2.

#import "core/resolve-semantics.typ": convert_temporal_data, resolve_channel_semantics
#import "core/encoding-overrides.typ": apply_encoding_overrides
#import "core/filter-overflow.typ": filter_overflow
#import "core/compute-layout.typ": (
  compute_channel_budgets, compute_layout, derive_stretch_caps, resolve_base_size,
)
#import "core/semantic-types.typ": compute_zero_decision
#import "core/py.typ": falsy, is-finite, is_number, py_str, truthy

// flint/vegalite/assemble.py _coerce_encoding_value
#let _coerce_encoding_value(value) = {
  if type(value) == str { return (field: value) }
  if type(value) == array {
    return value.map(v => if type(v) == str { (field: v) } else { v })
  }
  value
}

// flint/vegalite/assemble.py normalize_encoding_shorthand
//
// Expands the bare-string channel shorthand: `"weight"` -> `(field: "weight")`.
#let normalize_encoding_shorthand(encodings) = {
  let out = (:)
  for (ch, v) in encodings.pairs() { out.insert(ch, _coerce_encoding_value(v)) }
  out
}

/// Run the shared phases of the pipeline and return a chart plan.
///
/// `input_doc` mirrors upstream's shape:
///
/// ```typc
/// (
///   chart_spec: (chartType: "Bar Chart", encodings: (..), canvasSize: (..),
///                baseSize: (..), chartProperties: (..)),
///   data: (values: (..)),
///   semantic_types: (Field: "Amount", ..),
///   options: (..),
/// )
/// ```
///
/// `template` is the backend's definition for this chart type — the same shape
/// upstream's vegalite templates use (`markCognitiveChannel`,
/// `declareLayoutMode`, `encodingActions`, `template.encoding`), minus anything
/// Vega-Lite-specific.
#let assemble(input_doc, template) = {
  let chart_spec = input_doc.chart_spec
  let chart_type = chart_spec.chartType
  let raw_encodings = normalize_encoding_shorthand(
    {
      let e = chart_spec.at("encodings", default: none)
      if truthy(e) { e } else { (:) }
    },
  )
  let data = {
    let d = input_doc.at("data", default: none)
    let v = if truthy(d) { d.at("values", default: none) } else { none }
    if truthy(v) { v } else { () }
  }
  let semantic_types = {
    let s = input_doc.at("semantic_types", default: none)
    if truthy(s) { s } else { (:) }
  }

  // The optional `canvasSize` is a hard ceiling; layout targets `baseSize` and
  // stretch is capped so a smaller canvas shrinks the chart rather than
  // overflowing it.
  let size_ceiling = chart_spec.at("canvasSize", default: none)
  let base_size = resolve_base_size(chart_spec.at("baseSize", default: none), size_ceiling)
  let canvas_size = base_size
  let chart_properties = chart_spec.at("chartProperties", default: none)
  let options = {
    let o = input_doc.at("options", default: none)
    if truthy(o) { o } else { (:) }
  }

  // Category-B encoding overrides are composed onto the encodings before any
  // phase runs. Some actions need each channel's resolved TYPE to decide which
  // position axis is the category, so semantics are resolved once first, the
  // encodings are type-enriched, and semantics are then re-resolved.
  let converted_data = convert_temporal_data(data, semantic_types)
  let prelim_semantics = resolve_channel_semantics(
    raw_encodings, data, semantic_types, converted_data: converted_data,
  )
  let typed_raw_encodings = (:)
  for (ch, enc) in raw_encodings.pairs() {
    typed_raw_encodings.insert(ch, if truthy(enc.at("type", default: none)) {
      enc
    } else {
      let cs = prelim_semantics.at(ch, default: none)
      (..enc, type: if truthy(cs) { cs.at("type", default: none) } else { none })
    })
  }

  // Axis dtype override: the user can force a position channel between a
  // continuous time scale and discrete bands for date-like fields.
  for axis in ("x", "y") {
    let choice = {
      let cp = if truthy(chart_properties) { chart_properties } else { (:) }
      cp.at(axis + "AxisType", default: none)
    }
    let enc = typed_raw_encodings.at(axis, default: none)
    if (
      (choice == "temporal" or choice == "nominal")
        and truthy(enc) and truthy(enc.at("field", default: none))
    ) {
      typed_raw_encodings.insert(axis, (..enc, type: choice))
    }
  }

  let encodings = apply_encoding_overrides(
    template, typed_raw_encodings, chart_properties: chart_properties,
  )

  let warnings = ()

  // ── Phase 0: resolve semantics ────────────────────────────────────────────
  let tpl_mark = {
    let t = template.at("template", default: none)
    if truthy(t) { t.at("mark", default: none) } else { none }
  }
  let template_mark_type = if type(tpl_mark) == str {
    tpl_mark
  } else if type(tpl_mark) == dictionary {
    tpl_mark.at("type", default: none)
  } else { none }

  let channel_semantics = resolve_channel_semantics(
    encodings, data, semantic_types, converted_data: converted_data,
  )

  let effective_mark_type = if truthy(template_mark_type) { template_mark_type } else { "point" }
  for (channel, cs) in channel_semantics.pairs() {
    if (channel == "x" or channel == "y") and cs.at("type", default: none) == "quantitative" {
      let numeric_values = data
        .map(r => r.at(cs.field, default: none))
        .filter(v => v != none and type(v) != bool and is_number(v) and is-finite(v))
      let annotation = {
        let a = cs.at("semanticAnnotation", default: none)
        if truthy(a) { a } else { (:) }
      }
      channel_semantics.at(channel).insert("zero", compute_zero_decision(
        annotation.at("semanticType", default: none),
        channel, effective_mark_type, values: numeric_values,
      ))
    }
  }

  // ── Zero-baseline override (position-cognitive axes) ──────────────────────
  // `compute_zero_decision` is the single authority; where it flags the call as
  // a genuine toss-up the host may override it, and that choice is written back
  // so every downstream consumer sees it — including banking in layout.
  if template.at("markCognitiveChannel", default: none) == "position" {
    for axis in ("x", "y") {
      let cs = channel_semantics.at(axis, default: none)
      if (
        falsy(cs) or falsy(cs.at("field", default: none))
          or cs.at("type", default: none) != "quantitative"
          or falsy(cs.at("zero", default: none))
      ) { continue }
      let choice = {
        let cp = if truthy(chart_properties) { chart_properties } else { (:) }
        cp.at("includeZero_" + axis, default: none)
      }
      if choice == none { continue }
      channel_semantics.at(axis).insert("zero", (..cs.zero, zero: choice))
    }
  }

  // ── Log-scale override ────────────────────────────────────────────────────
  // A log scale only makes sense on a continuous quantitative *position* axis;
  // on length or area marks any recommended log is stripped so they render
  // linearly from their baseline.
  if template.at("markCognitiveChannel", default: none) == "position" {
    for axis in ("x", "y") {
      let cs = channel_semantics.at(axis, default: none)
      if (
        falsy(cs) or falsy(cs.at("field", default: none))
          or cs.at("type", default: none) != "quantitative"
      ) { continue }
      let tpl_enc = {
        let t = template.at("template", default: none)
        let e = if truthy(t) { t.at("encoding", default: none) } else { none }
        if truthy(e) { e.at(axis, default: none) } else { none }
      }
      if truthy(tpl_enc) and truthy(tpl_enc.at("bin", default: none)) { continue }
      let choice = {
        let cp = if truthy(chart_properties) { chart_properties } else { (:) }
        cp.at("logScale_" + axis, default: none)
      }
      if choice == none { continue }
      let field = cs.field
      let has_zero = data.any(row => {
        let v = row.at(field, default: none)
        type(v) != bool and v == 0
      })
      channel_semantics.at(axis).insert(
        "scaleType",
        if choice == false { none } else if has_zero { "symlog" } else { "log" },
      )
    }
  } else {
    for axis in ("x", "y") {
      let cs = channel_semantics.at(axis, default: none)
      let st = if truthy(cs) { cs.at("scaleType", default: none) } else { none }
      if st == "log" or st == "symlog" { channel_semantics.at(axis).insert("scaleType", none) }
    }
  }

  // ── Step 0a: declareLayoutMode ────────────────────────────────────────────
  let declare_fn = template.at("declareLayoutMode", default: none)
  let declaration = if truthy(declare_fn) {
    let d = declare_fn(channel_semantics, data, chart_properties)
    if truthy(d) { d } else { (:) }
  } else { (:) }

  // Auto-detect binned axes from the template's own encoding.
  if falsy(declaration.at("binnedAxes", default: none)) {
    let template_enc = {
      let t = template.at("template", default: none)
      if truthy(t) { t.at("encoding", default: none) } else { none }
    }
    if truthy(template_enc) {
      let binned_axes = (:)
      for axis in ("x", "y") {
        let ax_enc = template_enc.at(axis, default: none)
        let bin = if truthy(ax_enc) { ax_enc.at("bin", default: none) } else { none }
        if truthy(bin) {
          let prop_bins = {
            let cp = if truthy(chart_properties) { chart_properties } else { (:) }
            cp.at("binCount", default: none)
          }
          binned_axes.insert(axis, if truthy(prop_bins) {
            (maxbins: prop_bins)
          } else if type(bin) == dictionary and truthy(bin.at("maxbins", default: none)) {
            bin
          } else {
            (maxbins: 10)
          })
        }
      }
      if binned_axes.len() > 0 { declaration.insert("binnedAxes", binned_axes) }
    }
  }

  // Merge the declaration's parameter overrides, then resolve the stretch caps.
  let effective_options = (
    : ..options,
    ..{
      let p = declaration.at("paramOverrides", default: none)
      if truthy(p) { p } else { (:) }
    },
  )
  if effective_options.at("facetFixedPadding", default: none) == none {
    effective_options.insert("facetFixedPadding", (width: 50, height: 40))
  }
  if effective_options.at("facetGap", default: none) == none {
    effective_options.insert("facetGap", 10)
  }
  if effective_options.at("targetBandAR", default: none) == none {
    effective_options.insert("targetBandAR", 10)
  }
  let caps = derive_stretch_caps(base_size, size_ceiling, effective_options)
  effective_options.insert("maxStretchX", caps.maxStretchX)
  effective_options.insert("maxStretchY", caps.maxStretchY)

  // ── Step 0b: budgets and overflow ─────────────────────────────────────────
  let all_mark_types = ()
  if truthy(template_mark_type) { all_mark_types.push(template_mark_type) }

  let budgets = compute_channel_budgets(
    channel_semantics, declaration, converted_data, canvas_size, effective_options,
  )
  let facet_grid_result = budgets.at("facetGrid", default: none)

  let overflow_result = filter_overflow(
    channel_semantics, declaration, encodings, converted_data, budgets, all_mark_types,
  )
  let values = overflow_result.filteredData
  warnings += overflow_result.warnings

  // ── Phase 1: layout ───────────────────────────────────────────────────────
  let layout_result = compute_layout(
    channel_semantics, declaration, values, canvas_size,
    options: effective_options, facet_grid: facet_grid_result,
  )
  layout_result.insert("truncations", overflow_result.truncations)

  (
    chartType: chart_type,
    template: template,
    encodings: encodings,
    channelSemantics: channel_semantics,
    declaration: declaration,
    data: values,
    allData: converted_data,
    canvasSize: canvas_size,
    chartProperties: chart_properties,
    options: effective_options,
    budgets: budgets,
    layout: layout_result,
    nominalCounts: overflow_result.nominalCounts,
    warnings: warnings,
  )
}
