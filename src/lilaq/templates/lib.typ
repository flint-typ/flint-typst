// The template registry.
//
// Mirrors flint/vegalite/templates/__init__.py: chart type name -> definition.
// A backend supports exactly the chart types it registers here; anything else
// is a clear error rather than a wrong-looking chart.

#import "bar.typ": bar_chart_def, grouped_bar_chart_def
#import "line.typ": area_chart_def, line_chart_def, scatter_plot_def

#let TEMPLATES = (
  "Line Chart": line_chart_def,
  "Area Chart": area_chart_def,
  "Scatter Plot": scatter_plot_def,
  "Bar Chart": bar_chart_def,
  "Grouped Bar Chart": grouped_bar_chart_def,
)

// flint/vegalite/templates/__init__.py vl_get_template_def
#let get_template_def(chart_type) = TEMPLATES.at(chart_type, default: none)

/// Chart types this backend can draw.
#let supported-charts = TEMPLATES.keys()
