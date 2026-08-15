// Differential-test dispatch table.
//
// `test/differential.py` sends `[{fn, cases: [[args...]]}]` and reads back the
// same shape with results. This maps upstream function names to the ported
// implementations so the Python side can drive them by name.
//
// Add an entry the same commit you port a function. A name missing here is
// reported as unimplemented rather than silently passing.

#import "../src/core/js-round.typ" as js-round
#import "../src/core/type-registry.typ" as type-registry
#import "../src/core/color-decisions.typ" as color-decisions
#import "../src/core/encoding-actions.typ" as encoding-actions
#import "../src/core/encoding-overrides.typ" as encoding-overrides
#import "../src/core/semantic-types.typ" as semantic-types
#import "../src/core/js-date.typ" as js-date
#import "../src/core/field-semantics.typ" as field-semantics
#import "../src/core/resolve-semantics.typ" as resolve-semantics
#import "../src/core/filter-overflow.typ" as filter-overflow
#import "../src/core/decisions.typ" as decisions
#import "../src/core/compute-layout.typ" as compute-layout
#import "../src/core/py.typ" as py
#import "@local/datehog:0.1.0" as dh

#let DISPATCH = (
  // flint/core/__init__.py
  js_round: js-round.js_round,

  // flint/core/type_registry.py
  get_registry_entry: type-registry.get_registry_entry,
  is_registered: type-registry.is_registered,
  get_registered_types: type-registry.get_registered_types,

  // flint/core/color_decisions.py
  _infer_color_channel_primary: color-decisions._infer_color_channel_primary,
  _decide_scheme_type_from_channel: color-decisions._decide_scheme_type_from_channel,
  _count_distinct_values: color-decisions._count_distinct_values,
  _decide_color_for_channel: color-decisions._decide_color_for_channel,
  decide_color_maps: color-decisions.decide_color_maps,

  // flint/core/js_date.py
  _looks_like_word_plus_year: js-date._looks_like_word_plus_year,
  _try_v8_numeric_date: js-date._try_v8_numeric_date,
  js_date_parse_ms: js-date.js_date_parse_ms,
  js_date_parse: js-date.js_date_parse,
  is_js_parseable: js-date.is_js_parseable,

  // flint/core/semantic_types.py
  get_vis_category: semantic-types.get_vis_category,
  _is_boolean: semantic-types._is_boolean,
  _is_number_like: semantic-types._is_number_like,
  _looks_like_date_string: semantic-types._looks_like_date_string,
  _date_parse_succeeds: semantic-types._date_parse_succeeds,
  _is_date_like: semantic-types._is_date_like,
  infer_vis_category: semantic-types.infer_vis_category,
  is_measure_type: semantic-types.is_measure_type,
  is_time_series_type: semantic-types.is_time_series_type,
  is_categorical_type: semantic-types.is_categorical_type,
  is_ordinal_type: semantic-types.is_ordinal_type,
  is_geo_type: semantic-types.is_geo_type,
  is_geo_coordinate_type: semantic-types.is_geo_coordinate_type,
  is_geo_location_string: semantic-types.is_geo_location_string,
  is_non_measure_numeric: semantic-types.is_non_measure_numeric,
  get_zero_class: semantic-types.get_zero_class,
  _data_far_from_zero: semantic-types._data_far_from_zero,
  // Python's positional-with-default parameters become *named* parameters in
  // Typst (there is no positional default). `cases.py` mirrors the Python call
  // shape, so these two get an adapter that maps trailing positionals onto the
  // named parameters. See PORT-DICTIONARY.
  compute_zero_decision: (..a) => {
    let p = a.pos()
    semantic-types.compute_zero_decision(
      p.at(0), p.at(1), p.at(2), values: p.at(3, default: none),
    )
  },
  compute_padded_domain: semantic-types.compute_padded_domain,
  _pick_scheme: semantic-types._pick_scheme,
  get_recommended_color_scheme: (..a) => {
    let p = a.pos()
    semantic-types.get_recommended_color_scheme(
      p.at(0), p.at(1),
      unique_value_count: p.at(2, default: 10),
      field_name: p.at(3, default: ""),
      values: p.at(4, default: none),
      color_hint: p.at(5, default: none),
    )
  },
  _build_lookup: semantic-types._build_lookup,
  _match_sequence: semantic-types._match_sequence,
  infer_ordinal_sort_order: semantic-types.infer_ordinal_sort_order,

  // flint/core/field_semantics.py
  to_type_string: field-semantics.to_type_string,
  normalize_annotation: field-semantics.normalize_annotation,
  _detect_percentage_representation: field-semantics._detect_percentage_representation,
  _detect_precision: field-semantics._detect_precision,
  _precision_format: (..a) => {
    let p = a.pos()
    field-semantics._precision_format(
      p.at(0), use_grouping: p.at(1, default: true), sign_mode: p.at(2, default: ""),
    )
  },
  resolve_format: field-semantics.resolve_format,
  _try_float: field-semantics._try_float,
  resolve_default_vis_type: field-semantics.resolve_default_vis_type,
  resolve_aggregation_default: field-semantics.resolve_aggregation_default,
  resolve_zero_class_from_annotation: (..a) => {
    let p = a.pos()
    field-semantics.resolve_zero_class_from_annotation(p.at(0), domain: p.at(1, default: none))
  },
  resolve_scale_type: field-semantics.resolve_scale_type,
  _merge_intrinsic_with_data: field-semantics._merge_intrinsic_with_data,
  snap_to_bound_heuristic: field-semantics.snap_to_bound_heuristic,
  resolve_domain_constraint: field-semantics.resolve_domain_constraint,
  resolve_tick_constraint: (..a) => {
    let p = a.pos()
    field-semantics.resolve_tick_constraint(p.at(0), domain: p.at(1, default: none))
  },
  resolve_canonical_order: field-semantics.resolve_canonical_order,
  resolve_cyclic: field-semantics.resolve_cyclic,
  resolve_reversed: (..a) => {
    let p = a.pos()
    field-semantics.resolve_reversed(p.at(0), channel: p.at(1, default: none))
  },
  resolve_nice: (..a) => {
    let p = a.pos()
    field-semantics.resolve_nice(p.at(0), domain_constraint: p.at(1, default: none))
  },
  resolve_diverging_info: field-semantics.resolve_diverging_info,
  resolve_color_scheme_hint: field-semantics.resolve_color_scheme_hint,
  resolve_binning_suggested: (..a) => {
    let p = a.pos()
    field-semantics.resolve_binning_suggested(p.at(0), domain: p.at(1, default: none))
  },
  resolve_stackable: field-semantics.resolve_stackable,
  resolve_sort_direction: field-semantics.resolve_sort_direction,
  resolve_field_semantics: field-semantics.resolve_field_semantics,

  // flint/core/resolve_semantics.py
  is_likely_timestamp: resolve-semantics.is_likely_timestamp,
  timestamp_to_ms: resolve-semantics.timestamp_to_ms,
  infer_implicit_semantic_type: resolve-semantics.infer_implicit_semantic_type,
  looks_like_date_string: resolve-semantics.looks_like_date_string,
  _utc_attrs: resolve-semantics._utc_attrs,
  compute_data_votes: resolve-semantics.compute_data_votes,
  pick_best_level: resolve-semantics.pick_best_level,
  level_to_format: resolve-semantics.level_to_format,
  _resolve_temporal_format: resolve-semantics._resolve_temporal_format,
  _expand_to_full_year: resolve-semantics._expand_to_full_year,
  _js_number_to_string: resolve-semantics._js_number_to_string,
  convert_temporal_data: resolve-semantics.convert_temporal_data,

  resolve_channel_semantics: (..a) => {
    let p = a.pos()
    resolve-semantics.resolve_channel_semantics(
      p.at(0), p.at(1), p.at(2), converted_data: p.at(3, default: none),
    )
  },

  // flint/core/filter_overflow.py
  _js_sort_key: filter-overflow._js_sort_key,
  _default_overflow_strategy: filter-overflow._default_overflow_strategy,
  filter_overflow: filter-overflow.filter_overflow,

  // flint/core/decisions.py
  _vis_category_to_vl_type: decisions._vis_category_to_vl_type,
  _looks_temporal_value: decisions._looks_temporal_value,
  _validate_temporal_parsing: decisions._validate_temporal_parsing,
  _resolve_temporal_encoding: decisions._resolve_temporal_encoding,
  _apply_ordinal_guards: decisions._apply_ordinal_guards,
  _disambiguate_multi_encoding: decisions._disambiguate_multi_encoding,
  _can_parse_float: decisions._can_parse_float,
  resolve_encoding_type: decisions.resolve_encoding_type,
  compute_gas_pressure: (..a) => {
    let p = a.pos()
    decisions.compute_gas_pressure(
      p.at(0), p.at(1), p.at(2), p.at(3), p.at(4), p.at(5), params: p.at(6, default: none),
    )
  },
  compute_elastic_budget: decisions.compute_elastic_budget,
  compute_axis_step: decisions.compute_axis_step,
  compute_facet_layout: decisions.compute_facet_layout,
  compute_label_sizing: decisions.compute_label_sizing,
  compute_overflow: decisions.compute_overflow,
  compute_circumference_pressure: (..a) => {
    let p = a.pos()
    decisions.compute_circumference_pressure(p.at(0), p.at(1), params: p.at(2, default: none))
  },
  compute_effective_bar_count: decisions.compute_effective_bar_count,

  // flint/core/compute_layout.py
  _is_finite_number: compute-layout._is_finite_number,
  _compute_discrete_label_stats: compute-layout._compute_discrete_label_stats,
  _discrete_y_axis_should_use_horizontal_labels: compute-layout._discrete_y_axis_should_use_horizontal_labels,
  _js_to_number: compute-layout._js_to_number,
  _js_to_date_number: compute-layout._js_to_date_number,
  _is_nan: compute-layout._is_nan,
  resolve_base_size: compute-layout.resolve_base_size,
  resolve_stretch_caps: compute-layout.resolve_stretch_caps,
  derive_stretch_caps: compute-layout.derive_stretch_caps,
  count_distinct_series: compute-layout.count_distinct_series,
  compute_banking_ar: compute-layout.compute_banking_ar,
  compute_facet_grid: compute-layout.compute_facet_grid,
  compute_channel_budgets: compute-layout.compute_channel_budgets,
  compute_min_subplot_dimensions: compute-layout.compute_min_subplot_dimensions,
  compute_layout: (..a) => {
    let p = a.pos()
    compute-layout.compute_layout(
      p.at(0), p.at(1), p.at(2), p.at(3),
      options: p.at(4, default: none), facet_grid: p.at(5, default: none),
    )
  },

  // flint/core/encoding_actions.py
  _is_measure_enc: encoding-actions._is_measure_enc,
  _is_discrete_category_enc: encoding-actions._is_discrete_category_enc,
  _resolve_sort_channels: encoding-actions._resolve_sort_channels,
)

// Non-finite floats have no JSON literal, so both sides agree on a tagged
// encoding. Keep in sync with `jsonable` in differential.py and with
// `NONFINITE` in test/make_corpus.py.
#let encode(value) = {
  let t = type(value)
  if t == float {
    if value.is-nan() { return ("$f": "nan") }
    if value.is-infinite() { return ("$f": if value > 0 { "inf" } else { "-inf" }) }
    return value
  }
  if t == array { return value.map(encode) }
  if t == dictionary {
    let out = (:)
    for (k, v) in value.pairs() { out.insert(k, encode(v)) }
    return out
  }
  // Functions and other opaque values cannot cross the boundary; render them
  // the way Python's `repr` fallback does so a mismatch is visible rather than
  // an error.
  if t == function { return "<function>" }
  value
}


// Inverse of `encode`: arguments arrive with non-finite floats tagged, and
// must be turned back into real floats before the ported function sees them.
// Without this a tagged `inf` reaches the callee as a dictionary and every
// type test on it silently answers "not a number".
#let decode(value) = {
  let t = type(value)
  if t == array { return value.map(decode) }
  if t == dictionary {
    if value.len() == 1 and "$f" in value {
      let tag = value.at("$f")
      if tag == "nan" { return float.nan }
      if tag == "inf" { return float.inf }
      if tag == "-inf" { return -float.inf }
    }
    let out = (:)
    for (k, v) in value.pairs() { out.insert(k, decode(v)) }
    return out
  }
  value
}

// Typst half of `PROJECTIONS` in differential.py. Keep the names in step.
#let PROJECTIONS = (
  epoch_ms: v => if v == none { none } else { float(dh.to-ms(v)) },
)

#let run-one(name, args, project) = {
  let fn = DISPATCH.at(name, default: none)
  if fn == none { return (unimplemented: name) }
  let value = fn(..decode(args))
  if project != none { value = (PROJECTIONS.at(project))(value) }
  (ok: encode(value))
}

#let run-all(specs) = specs.map(spec => spec.cases.map(args => run-one(spec.fn, args, spec.at("project", default: none))))
