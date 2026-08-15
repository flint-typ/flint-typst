// Re-exports for the whole core, so backends import one module.
//
// Mirrors flint/core/__init__.py's role. The five pipeline entry points are
// what a backend needs; the rest is exposed for templates that reach deeper.

#import "resolve-semantics.typ": convert_temporal_data, resolve_channel_semantics
#import "filter-overflow.typ": filter_overflow
#import "compute-layout.typ": (
  compute_channel_budgets, compute_layout, compute_min_subplot_dimensions,
  count_distinct_series, derive_stretch_caps, resolve_base_size, resolve_stretch_caps,
)
#import "encoding-overrides.typ": apply_encoding_overrides
#import "encoding-actions.typ": make_sort_action
#import "field-semantics.typ": normalize_annotation, resolve_field_semantics, to_type_string
#import "semantic-types.typ": (
  compute_zero_decision, get_recommended_color_scheme, get_vis_category,
  infer_ordinal_sort_order, infer_vis_category,
)
#import "type-registry.typ": get_registry_entry, get_registered_types, is_registered
#import "decisions.typ": resolve_encoding_type
#import "js-round.typ": js_round
#import "types.typ": channelGroups, channels
