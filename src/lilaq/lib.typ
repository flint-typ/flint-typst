// The lilaq backend's public entry point.

#import "../assemble.typ": assemble
#import "templates/lib.typ": get_template_def, supported-charts
#import "render.typ": diagram-for
#import "../core/py.typ": falsy, truthy

/// Compile a chart declaration into a lilaq diagram.
///
/// ```typc
/// #chart(
///   chart-type: "Line Chart",
///   data: (("Month", "Sales"), ("Jan", 120), ("Feb", 180)),
///   encodings: (x: "Month", y: "Sales"),
///   semantic-types: (Sales: "Amount"),
/// )
/// ```
///
/// `data` is either an array of row dictionaries or a table with a header row.
/// Everything else mirrors the field names flint uses.
#let chart(
  chart-type: none,
  data: (),
  encodings: (:),
  semantic-types: (:),
  size: none,
  canvas-size: none,
  properties: none,
  options: (:),
) = {
  let template = get_template_def(chart-type)
  assert(
    template != none,
    message: "unknown chart type "
      + repr(chart-type)
      + "; this backend draws "
      + repr(supported-charts),
  )

  // Accept a header-row table as well as row dictionaries.
  let rows = if data.len() > 0 and type(data.first()) == array {
    let header = data.first()
    data.slice(1).map(r => {
      let d = (:)
      for (i, name) in header.enumerate() { d.insert(name, r.at(i, default: none)) }
      d
    })
  } else { data }

  let plan = assemble(
    (
      chart_spec: (
        chartType: chart-type,
        encodings: encodings,
        baseSize: size,
        canvasSize: canvas-size,
        chartProperties: properties,
      ),
      data: (values: rows),
      semantic_types: semantic-types,
      options: options,
    ),
    template,
  )

  diagram-for(plan, (template.instantiate)(plan))
}

/// The chart plan without rendering it — the semantics and layout core
/// resolved. Useful for inspection, and for building a different backend.
#let plan-for(..args) = {
  let named = args.named()
  let chart-type = named.at("chart-type", default: none)
  let template = get_template_def(chart-type)
  assert(template != none, message: "unknown chart type " + repr(chart-type))
  let data = named.at("data", default: ())
  let rows = if data.len() > 0 and type(data.first()) == array {
    let header = data.first()
    data.slice(1).map(r => {
      let d = (:)
      for (i, name) in header.enumerate() { d.insert(name, r.at(i, default: none)) }
      d
    })
  } else { data }
  assemble(
    (
      chart_spec: (
        chartType: chart-type,
        encodings: named.at("encodings", default: (:)),
        baseSize: named.at("size", default: none),
        canvasSize: named.at("canvas-size", default: none),
        chartProperties: named.at("properties", default: none),
      ),
      data: (values: rows),
      semantic_types: named.at("semantic-types", default: (:)),
      options: named.at("options", default: (:)),
    ),
    template,
  )
}
