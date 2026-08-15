// GENERATED from flint/core/types.py by test/gen_tables.py -- do not edit.
//
// Regenerate after any upstream change:  python test/gen_tables.py
// Check for drift in CI:                 python test/gen_tables.py --check
//
// Only pure-data tables are generated. Functions in this module are
// hand-ported and live alongside this file.

/// Every encoding channel the compiler knows about.
#let channels = ("x", "y", "x2", "y2", "id", "color", "opacity", "size", "shape", "strokeDash", "column", "row", "latitude", "longitude", "radius", "detail", "group", "open", "high", "low", "close", "angle", "metric", "value", "goal")

/// Channels grouped by the part of the chart they drive.
#let channelGroups = (
  "": ("x", "x2", "y", "y2", "latitude", "longitude", "id", "radius", "detail"),
  legends: ("color", "group", "size", "shape", "text", "opacity", "strokeDash"),
  price: ("open", "high", "low", "close"),
  facets: ("column", "row"),
  kpi: ("metric", "value", "goal"),
)
