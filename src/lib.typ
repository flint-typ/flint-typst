// flint-typst — semantic chart compiler for Typst.
//
// A port of the core of microsoft/flint-chart: given a data table and a chart
// declaration, work out what each channel *means* (semantic types, scales,
// formats, sort order) and how much room each part of the chart needs, then
// hand that to a backend to draw.
//
// The port tracks flint-py function-for-function so upstream changes stay
// mergeable; see ../../docs/PORT-PLAN.md for the rules and
// ../../PORT-DICTIONARY.md for every place the behaviour diverges.

#import "core/lib.typ" as core
