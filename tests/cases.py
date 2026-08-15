"""Argument sets for the differential test.

Each entry names an upstream module + function and a list of argument tuples.
`differential.py` runs every tuple through flint-py and through the Typst port
and diffs the results.

Two sources of cases, both used:

  * exhaustive where the domain is small and known — every semantic type in
    the registry, every channel, every combination of the flags a decision
    function branches on;
  * sampled from the real fixture corpus where the domain is open, so the
    port meets the shapes it will actually see.

Prefer adding a case that once failed over adding a plausible-looking one.
"""

from __future__ import annotations

import json
from pathlib import Path

PKG = Path(__file__).resolve().parent.parent
FIXTURES = PKG / "flint-source/shared/test-data"


def _registry_types() -> list[str]:
    from flint.core.type_registry import TYPE_REGISTRY

    return list(TYPE_REGISTRY.keys())


def _fixture_rows(limit: int = 6) -> list[list[dict]]:
    """A few real data tables, for the functions that scan rows."""
    out: list[list[dict]] = []
    if not FIXTURES.exists():
        return out
    for case in sorted(FIXTURES.iterdir()):
        f = case / "input.json"
        if not f.exists():
            continue
        doc = json.loads(f.read_text())
        rows = (doc.get("input", doc).get("data") or {}).get("values") or []
        if 2 <= len(rows) <= 60:
            out.append(rows)
        if len(out) >= limit:
            break
    return out


def _scalar_values() -> list:
    """Values of every kind that reach the type predicates."""
    return [
        None, True, False, 0, 1, -1, 2.5, -2.5, float("nan"), float("inf"),
        "", "   ", "0", "1", "1.5", "-2", "1e5", ".5", "abc", "NaN", "inf",
    ]


def _strings() -> list[str]:
    """Date-shaped and not-date-shaped strings, incl. real fixture values."""
    base = [
        "2020-01-01", "2020-01", "2020", "2020-01-01T08:00:00", "2020-01-01T08:00:00Z",
        "Jan 2020", "January 2020", "Jan 15 2020", "15 Jan 2020", "Feb 2020",
        "01/15/2020", "15.01.2020", "2020/01/15", "Q1 2018", "FY 2018",
        "Mon", "Monday", "Wk 01", "Stage 1", "Cat01", "abc", "", "   ",
        "1950", "98", "10-19", "Tue, 15 Feb 2020 08:30:00 GMT",
    ]
    seen = set(base)
    for rows in _fixture_rows(20):
        for row in rows[:5]:
            for v in row.values():
                if isinstance(v, str) and len(v) <= 32 and v not in seen:
                    seen.add(v)
                    base.append(v)
    return base[:120]


def _column_samples() -> list[list]:
    """Real columns, so infer_vis_category meets the shapes it will see."""
    out = []
    for rows in _fixture_rows(10):
        for field in list(rows[0].keys())[:4]:
            out.append([r.get(field) for r in rows][:40])
    return out


def _label_columns() -> list[list]:
    """Columns that should (and should not) match a canonical ordinal order."""
    return [
        ["Jan", "Feb", "Mar"], ["January", "February"], ["MAR", "jan", "FEB"],
        ["1", "2", "3", "12"], [1, 2, 3], [1.0, 2.0, 3.0],
        ["Mon", "Tue", "Wed"], ["Monday", "Sunday"], ["Mo", "Tu", "We"],
        ["Sun", "Mon", "Tue"], ["Q1", "Q4", "Q2"], ["q1", "q2"],
        ["N", "SE", "W"], ["North", "South"], ["Jan", "notamonth", "Feb"],
        ["Jan"], [], [None, None], ["", "Jan", "Feb"], ["a", "b", "c"],
    ]


# Channel-semantics shapes that exercise every branch of the colour decisions.
_CS_SHAPES = [
    None,
    {},
    {"field": "Sales", "type": "quantitative"},
    {"field": "Cat", "type": "nominal"},
    {"field": "Date", "type": "temporal"},
    {"field": "R", "type": "ordinal", "semanticAnnotation": {"semanticType": "Rank"}},
    {"field": "C", "type": "quantitative", "semanticAnnotation": {"semanticType": "Correlation"}},
    {"field": "S", "colorScheme": {"type": "diverging", "domainMid": 0}},
    {"field": "S", "colorScheme": {"type": "sequential"}},
    {"field": "S", "colorScheme": {"type": "categorical"}},
    {"field": "S", "type": "temporal", "colorScheme": {"type": "categorical"}},
    {
        "field": "S",
        "colorScheme": {"type": "categorical"},
        "semanticAnnotation": {"semanticType": "Rank"},
    },
]

_ENC_SHAPES = [
    None,
    {},
    {"field": "Sales"},
    {"field": "Sales", "type": "quantitative"},
    {"field": "Sales", "aggregate": "sum"},
    {"field": "Cat", "type": "nominal"},
    {"field": "Cat", "type": "ordinal"},
    {"field": "Date", "type": "temporal"},
    {"field": "Cat", "type": "nominal", "sortBy": "y", "sortOrder": "descending"},
    {"type": "quantitative"},  # no field
]


def to_type(a):
    from flint.core.field_semantics import to_type_string

    return to_type_string(a)


def norm(a):
    from flint.core.field_semantics import normalize_annotation

    return normalize_annotation(a)


def build() -> list[dict]:
    specs: list[dict] = []

    def add(module, fn, cases, project=None, known=None):
        """`known` maps `repr(args_tuple)` -> reason, for divergences that are
        deliberate. They are counted and printed, never silently passed."""
        specs.append({
            "module": module, "fn": fn, "project": project,
            "known_divergences": known or {},
            "cases": [list(c) for c in cases],
        })

    # --- flint/core/__init__.py -------------------------------------------
    add("__init__", "js_round", [
        (x,) for x in [
            0, 1, -1, 0.5, -0.5, 1.5, -1.5, 2.5, -2.5, 0.49999, -0.49999,
            2.4999999, 100.5, -100.5, 1e9 + 0.5, 0.0, -0.0, 3, -3,
            True, False, None,
        ]
    ])

    # --- flint/core/type_registry.py --------------------------------------
    types = _registry_types()
    add("type_registry", "get_registry_entry", [(t,) for t in types] + [("Nonexistent",), ("",)])
    add("type_registry", "is_registered", [(t,) for t in types] + [("Nonexistent",), ("",)])
    add("type_registry", "get_registered_types", [()])

    # --- flint/core/color_decisions.py ------------------------------------
    add("color_decisions", "_infer_color_channel_primary", [
        (ch, ct)
        for ch in ("color", "group", "x", "y", "size", "shape", "")
        for ct in ("Bar Chart", "Line Chart")
    ])
    add("color_decisions", "_decide_scheme_type_from_channel", [
        (ch, cs) for ch in ("color", "group", "size") for cs in _CS_SHAPES
    ])

    rows_sets = _fixture_rows()
    distinct_cases: list[tuple] = [([], None), ([], "Sales"), ([{"a": 1}, None, {"a": 1}], "a")]
    for rows in rows_sets:
        for field in list(rows[0].keys())[:3]:
            distinct_cases.append((rows, field))
        distinct_cases.append((rows, None))
        distinct_cases.append((rows, "NotAField"))
    add("color_decisions", "_count_distinct_values", distinct_cases)

    color_ctx_cases: list[tuple] = []
    for cs in _CS_SHAPES:
        for enc in _ENC_SHAPES:
            for scheme in (None, "default", "tableau10"):
                e = None if enc is None else {**enc, **({"scheme": scheme} if scheme else {})}
                ctx = {
                    "encodings": {"color": e},
                    "channelSemantics": {"color": cs},
                    "chartType": "Bar Chart",
                    "table": [{"Sales": 1}, {"Sales": 2}, {"Sales": 2}],
                }
                color_ctx_cases.append(("color", ctx))
    add("color_decisions", "_decide_color_for_channel", color_ctx_cases)
    add("color_decisions", "decide_color_maps", [(ctx,) for _, ctx in color_ctx_cases[:40]])

    # --- flint/core/encoding_actions.py -----------------------------------
    add("encoding_actions", "_is_measure_enc", [(e,) for e in _ENC_SHAPES])
    add("encoding_actions", "_is_discrete_category_enc", [(e,) for e in _ENC_SHAPES])
    add("encoding_actions", "_resolve_sort_channels", [
        ({"x": x, "y": y}, ("x", "y"))
        for x in _ENC_SHAPES
        for y in _ENC_SHAPES
    ])

    # --- flint/core/js_date.py --------------------------------------------
    date_inputs = _scalar_values() + _strings() + [
        1584174600000, 1584174600, 0, -1, 1.5,
    ]
    # dateutil's tokenizer reads a leading digit run out of strings that are
    # not dates by any reasonable reading: "1.5" and "-2" and ".5" become days
    # of January 1970. The plausible shapes ("98", "10-19", weekday names) are
    # reproduced; these three are not, because pinning dateutil's exact
    # tokenizer would cost more than it protects. See PORT-DICTIONARY.
    DATEUTIL_TOKENIZER = {
        repr(("1.5",)): "dateutil reads a leading digit run: '1.5' -> 1970-01-01",
        repr((".5",)): "dateutil reads a leading digit run: '.5' -> 1970-01-05",
        repr(("-2",)): "dateutil reads a leading digit run: '-2' -> 1970-01-02",
    }
    add("js_date", "js_date_parse_ms", [(v,) for v in date_inputs], known=DATEUTIL_TOKENIZER)
    add("js_date", "is_js_parseable", [(v,) for v in date_inputs], known=DATEUTIL_TOKENIZER)
    add("js_date", "_looks_like_word_plus_year", [(s,) for s in _strings()])
    # These two return a datetime upstream and a datehog moment here, so both
    # sides are projected onto epoch milliseconds before diffing.
    add("js_date", "js_date_parse", [(v,) for v in date_inputs], project="epoch_ms",
        known=DATEUTIL_TOKENIZER)
    add("js_date", "_try_v8_numeric_date", [(s,) for s in _strings()], project="epoch_ms")

    # --- flint/core/semantic_types.py -------------------------------------
    from flint.core.semantic_types import _ORDINAL_SEQUENCES

    add("semantic_types", "get_vis_category",
        [(t,) for t in types] + [("Nonexistent",), ("",), (None,)])
    for fn in ("is_measure_type", "is_time_series_type", "is_categorical_type",
               "is_ordinal_type", "is_geo_type", "is_geo_coordinate_type",
               "is_geo_location_string", "is_non_measure_numeric", "get_zero_class"):
        add("semantic_types", fn, [(t,) for t in types] + [("Nonexistent",), ("",)])

    scalars = _scalar_values()
    add("semantic_types", "_is_boolean", [(v,) for v in scalars])
    add("semantic_types", "_is_number_like", [(v,) for v in scalars])
    add("semantic_types", "_looks_like_date_string", [(s,) for s in _strings()])
    add("semantic_types", "_date_parse_succeeds", [(v,) for v in scalars + _strings()])
    add("semantic_types", "_is_date_like", [(v,) for v in scalars + _strings()])

    add("semantic_types", "infer_vis_category",
        [(vals,) for vals in _column_samples()] + [
            ([],), ([None, None],), ([True, False],), ([1, 2.5, "3"],),
            (["2020-01-01", "2020-02-01"],), (["a", "b"],), ([1, "a"],),
        ])

    # Zero-baseline decisions: every zero class x mark x channel x data shape.
    zero_cases = []
    value_shapes = [None, [], [1.0, 2.0], [0.0, 5.0], [-3.0, 4.0], [8.0, 10.0], [1.0, 100.0]]
    for t in ("Amount", "Temperature", "Percentage", "Score", "Rank", "Year", "Category", "Count"):
        for mark in ("bar", "area", "rect", "circle", "point", "line", "strip"):
            for ch in ("x", "y", "color"):
                for vals in value_shapes:
                    zero_cases.append((t, ch, mark, vals))
    add("semantic_types", "compute_zero_decision", zero_cases)
    add("semantic_types", "_data_far_from_zero", [(v,) for v in value_shapes])
    add("semantic_types", "compute_padded_domain",
        [(v, p) for v in value_shapes if v for p in (0, 0.05, 0.5, -1)])

    # The JS 32-bit hash: field names from the fixtures plus adversarial ones.
    names = sorted({f for rows in _fixture_rows(12) for f in rows[0].keys()})
    names += ["", "a", "Sales", "A very long field name with spaces", "\u00fcml\u00e4ut", "123"]
    add("semantic_types", "_pick_scheme",
        [(["viridis", "blues", "greens", "reds", "yelloworangebrown", "goldgreen"], n) for n in names]
        + [(["blues", "greens", "purples", "oranges"], n) for n in names])

    scheme_cases = []
    for t in [None, ""] + types:
        for enc in ("quantitative", "ordinal", "nominal", "temporal"):
            for count in (3, 10, 11):
                for hint in (None, {"type": "diverging"}, {"type": "sequential"}):
                    scheme_cases.append((t, enc, count, "Sales", None, hint))
    add("semantic_types", "get_recommended_color_scheme", scheme_cases)

    add("semantic_types", "_build_lookup",
        [(seq,) for seqs in _ORDINAL_SEQUENCES.values() for seq in seqs])
    seq_cases = []
    for label_set in _label_columns():
        for key, seqs in _ORDINAL_SEQUENCES.items():
            seq_cases.append((label_set, seqs))
    add("semantic_types", "_match_sequence", seq_cases)
    add("semantic_types", "infer_ordinal_sort_order",
        [(t, vals) for t in ("Month", "Day", "Quarter", "Direction", "Category", "Unknown", "", "Amount")
         for vals in _label_columns()])

    # --- flint/core/field_semantics.py ------------------------------------
    ANNOTATIONS = [
        None, "", {}, "Price", "Amount", "Percentage", "Temperature", "Score",
        {"semanticType": "Price", "unit": "USD"},
        {"semanticType": "Price", "unit": "usd"},
        {"semanticType": "Price", "unit": "EUR"},
        {"semanticType": "Amount", "unit": "ZZZ"},
        {"semanticType": "Temperature", "unit": "°C"},
        {"semanticType": "Temperature", "unit": "F"},
        {"semanticType": "Temperature", "unit": "K"},
        {"semanticType": "Quantity", "unit": "kg"},
        {"semanticType": "Duration", "unit": "sec"},
        {"semanticType": "Percentage", "intrinsicDomain": [0, 1]},
        {"semanticType": "Percentage", "intrinsicDomain": [0, 100]},
        {"semanticType": "Score", "intrinsicDomain": [0, 10]},
        {"semanticType": "Profit", "intrinsicDomain": [-100, 100]},
        {"semanticType": "Correlation"},
        {"semanticType": "Latitude"}, {"semanticType": "Longitude"},
        {"semanticType": "Month", "sortOrder": ["Jan", "Feb"]},
        {"semanticType": None},
        {"semanticType": "NotARealType"},
    ]
    NUMERIC_COLS = [
        [], [1, 2, 3], [1.0, 2.5, 3.25], [0.001, 0.5], [0.1, 0.2, 0.30000000000000004],
        [1 / 3, 2 / 3], [0, 100], [-50, 50], [0.05, 0.95], [5, 95],
        [1e-7, 1], [1, 10**7], [0, 1, 10**7], list(range(12)), list(range(30)),
        [1.123456789, 2.2], [100.0] * 12, [None, 1, 2, "x", True, float("nan")],
    ]
    VALUE_COLS = NUMERIC_COLS + _label_columns() + _column_samples()[:6]

    add("field_semantics", "to_type_string", [(a,) for a in ANNOTATIONS] + [(1,), (True,), ([],)])
    add("field_semantics", "normalize_annotation", [(a,) for a in ANNOTATIONS] + [(1,), (True,)])
    add("field_semantics", "_detect_percentage_representation", [(c,) for c in NUMERIC_COLS if all(isinstance(v, (int, float)) and not isinstance(v, bool) for v in c)])
    add("field_semantics", "_detect_precision", [(c,) for c in NUMERIC_COLS])
    add("field_semantics", "_precision_format",
        [(c, g, sm) for c in NUMERIC_COLS[:8] for g in (True, False) for sm in ("", "+")])
    add("field_semantics", "_try_float", [(s,) for s in _strings()])
    add("field_semantics", "resolve_aggregation_default", [(t,) for t in types] + [("Nope",)])
    add("field_semantics", "resolve_cyclic", [(t,) for t in types])
    add("field_semantics", "resolve_sort_direction", [(t,) for t in types])
    add("field_semantics", "resolve_stackable", [(t,) for t in types])
    add("field_semantics", "resolve_reversed",
        [(t, ch) for t in ("Rank", "Score", "Amount") for ch in ("x", "y", "color", None)])
    add("field_semantics", "resolve_zero_class_from_annotation",
        [(t, d) for t in types for d in (None, [0, 10], [5, 10], [-5, 5])])
    add("field_semantics", "resolve_scale_type",
        [(t, c) for t in ("Amount", "Quantity", "Count", "Duration", "Price", "Category")
         for c in NUMERIC_COLS])
    add("field_semantics", "resolve_default_vis_type",
        [(t, c) for t in types for c in VALUE_COLS[:10]])
    add("field_semantics", "resolve_format",
        [(to_type(a), norm(a), c) for a in ANNOTATIONS for c in NUMERIC_COLS[:10]])
    add("field_semantics", "_merge_intrinsic_with_data",
        [(i, c, h) for i in ([0, 1], [-90, 90], [0, 100]) for c in NUMERIC_COLS[:8] for h in (True, False)])
    add("field_semantics", "snap_to_bound_heuristic",
        [(i, c) for i in ([0, 1], [0, 100], [-100, 100], [5, 5]) for c in NUMERIC_COLS[:10]])
    add("field_semantics", "resolve_domain_constraint",
        [(to_type(a), norm(a), c) for a in ANNOTATIONS for c in NUMERIC_COLS[:8]])
    add("field_semantics", "resolve_tick_constraint",
        [(t, d) for t in types for d in (None, [0, 10], [0, 25], [0, 1], [2, 4], [-5, 5])])
    add("field_semantics", "resolve_canonical_order",
        [(to_type(a), norm(a), c) for a in ANNOTATIONS[:12] for c in _label_columns()[:8]])
    add("field_semantics", "resolve_nice",
        [(t, dc) for t in types for dc in (
            None, {}, {"clamp": True}, {"clamp": False},
            {"min": 0, "max": 1, "clamp": False}, {"min": 0, "clamp": False},
        )])
    add("field_semantics", "resolve_diverging_info",
        [(to_type(a), norm(a), c) for a in ANNOTATIONS for c in NUMERIC_COLS[:8]
         if all(isinstance(v, (int, float)) and not isinstance(v, bool) for v in c)])
    add("field_semantics", "resolve_color_scheme_hint",
        [(to_type(a), norm(a), c) for a in ANNOTATIONS for c in NUMERIC_COLS[:8]])
    add("field_semantics", "resolve_binning_suggested",
        [(t, d) for t in types for d in (None, [0, 10], [0, 100], [0, 20], [0, 21])])
    add("field_semantics", "resolve_field_semantics",
        [(a, "Field", c) for a in ANNOTATIONS for c in VALUE_COLS[:12]])

    # --- flint/core/resolve_semantics.py (temporal half) ------------------
    add("resolve_semantics", "is_likely_timestamp",
        [(v,) for v in [0, 1, 1e8, 1e9, 1.5e9, 4102444800, 4102444801, 4102444800000,
                        4102444800001, -1, 2020]])
    add("resolve_semantics", "timestamp_to_ms",
        [(v,) for v in [0, 1e9, 4102444800, 4102444801, 1584174600, 1584174600000]])
    add("resolve_semantics", "looks_like_date_string", [(s,) for s in _strings()])
    add("resolve_semantics", "_expand_to_full_year",
        [(s,) for s in ["20", "49", "50", "99", "2020", "abc", "", "  20  ", "1"]])
    add("resolve_semantics", "_js_number_to_string",
        [(v,) for v in [1, 1.0, 1.5, 100.0, -2.5, 0, 0.0, 2020]])

    YEAR_FIELDS = ["Year", "year", "fiscalYear", "FY", "Sales", "yearOfBirth",
                   "Report Year", "year_end", "Category"]
    YEAR_COLS = [
        [2020, 2021, 2022], [2020.0, 2021.0], ["2020", "2021"], [2020], [],
        [1499, 2020], [2020, 2201], [2020, None, ""], [True, False],
        [2020, "x"], [2020.5, 2021], [None], ["", ""],
    ]
    add("resolve_semantics", "infer_implicit_semantic_type",
        [(f, c) for f in YEAR_FIELDS for c in YEAR_COLS])

    SAME_SHAPES = []
    import itertools
    for bits in itertools.product([True, False], repeat=5):
        SAME_SHAPES.append(dict(zip(("month", "day", "hour", "minute", "second"), bits)))
    add("resolve_semantics", "compute_data_votes", [(s,) for s in SAME_SHAPES])
    add("resolve_semantics", "pick_best_level",
        [(v,) for v in [[0]*6, [1,2,3,4,5,6], [6,5,4,3,2,1], [3,3,3,3,3,3], [0,0,0,0,0,9]]])
    add("resolve_semantics", "level_to_format",
        [(lvl, {"sameYear": sy, "sameDay": sd})
         for lvl in range(-1, 7) for sy in (True, False) for sd in (True, False)])

    DATE_COLS = [
        ["2020-01-01", "2020-02-01", "2020-03-01"],
        ["2020-01-01T08:00:00", "2020-01-01T09:30:00"],
        ["2020-01-01T08:00:00Z", "2020-01-02T08:00:00Z"],
        ["2019-06-01", "2020-06-01"],
        ["2020-01-01"], [], [None, None], ["notadate", "alsonot"],
        ["2020-01-01", "notadate"],
        [1584174600000, 1584261000000],
    ]
    add("resolve_semantics", "_resolve_temporal_format",
        [(c, st) for c in DATE_COLS
         for st in ("Year", "Month", "Date", "Hour", "DateTime", "Timestamp", "Decade", "Category", "")])

    TABLES = [
        [], [{"Date": "2020-01-01", "V": 1}, {"Date": "2020-02-01", "V": 2}],
        [{"Year": 2020, "V": 1}, {"Year": 2021, "V": 2}],
        [{"Year": "20", "V": 1}, {"Year": "21", "V": 2}],
        [{"T": 1584174600, "V": 1}, {"T": 1584261000, "V": 2}],
        [{"T": 1584174600000, "V": 1}, {"T": 1584261000000, "V": 2}],
        [{"D": "2020-01-01T08:00:00", "V": 1}, {"D": "2020-01-02T09:00:00", "V": 2}],
        [{"C": "Cat01", "V": 1}, {"C": "Cat02", "V": 2}],
        [{"D": None, "V": 1}, {"D": "2020-01-01", "V": 2}],
        [{"D": True, "V": 1}, {"D": False, "V": 2}],
        [{"Y": 2020.7, "V": 1}, {"Y": 2021.2, "V": 2}],
    ] + [rows[:8] for rows in _fixture_rows(8)]
    SEMTYPES = [
        {}, {"Date": "Date"}, {"Year": "Year"}, {"T": "Timestamp"}, {"D": "DateTime"},
        {"Y": "Decade"}, {"D": {"semanticType": "Date"}}, {"C": "Category"},
    ]
    add("resolve_semantics", "convert_temporal_data",
        [(t, st) for t in TABLES for st in SEMTYPES])

    ENC_SETS = [
        {}, {"x": {"field": "F"}}, {"x": {"field": "F", "type": "nominal"}},
        {"x": {"field": "F"}, "y": {"field": "V"}},
        {"y": {"aggregate": "count"}}, {"y": {"field": "V", "aggregate": "sum"}},
        {"color": {"field": "F"}}, {"color": {"field": "F", "scheme": "tableau10"}},
        {"color": {"field": "F", "scheme": "default"}},
        {"column": {"field": "V"}}, {"group": {"field": "F"}},
        {"x": {"field": "Missing"}}, {"x": {}},
        {"x": {"field": "F", "sortBy": "y"}},
    ]
    RCS_DATA = [
        [{"F": "a", "V": 1}, {"F": "b", "V": 2}, {"F": "c", "V": 3}],
        [{"F": "2020-01-01", "V": 1.5}, {"F": "2020-02-01", "V": -2.5}],
        [{"F": "2020-01-01T08:00:00Z", "V": 1}, {"F": "2020-01-02T08:00:00Z", "V": 2}],
        [{"F": i, "V": i * 1.5} for i in range(20)],
        [{"F": "Jan", "V": 1}, {"F": "Feb", "V": 2}, {"F": "Mar", "V": 3}],
        [],
    ]
    RCS_ST = [
        {}, {"F": "Category"}, {"F": "Date"}, {"F": "Year"}, {"V": "Amount"},
        {"V": {"semanticType": "Price", "unit": "USD"}},
        {"V": {"semanticType": "Percentage", "intrinsicDomain": [0, 1]}},
        {"F": "Month"}, {"V": "Correlation"}, {"F": "NotAType"},
    ]
    add("resolve_semantics", "resolve_channel_semantics",
        [(e, d, st, None) for e in ENC_SETS for d in RCS_DATA for st in RCS_ST[:6]])

    # --- flint/core/filter_overflow.py ------------------------------------
    add("filter_overflow", "_js_sort_key",
        [(v,) for v in [None, "a", 1, 1.0, -2.5, True, False, "", 2020]])

    OF_UNIQUE = [
        ["a", "b", "c", "d"], ["3", "1", "2"], [3, 1, 2], ["x", 3, "y", 1],
        ["Cat01", "Cat02", "Cat03"], [None, "a"], [], ["b"],
    ]
    OF_CTX = []
    for sort_by in (None, "x", "y", "color", '["c","a","b"]', "not json", "[1,2,"):
        for sort_order in (None, "ascending", "descending"):
            for marks in ([], ["bar"], ["line"]):
                OF_CTX.append({
                    "data": [{"F": "a", "V": 3}, {"F": "b", "V": 1},
                             {"F": "c", "V": 2}, {"F": "a", "V": 5}],
                    "channelSemantics": {
                        "x": {"field": "F", "type": "nominal"},
                        "y": {"field": "V", "type": "quantitative"},
                        "color": {"field": "F", "type": "nominal"},
                    },
                    "encodings": {"x": {"sortBy": sort_by, "sortOrder": sort_order}},
                    "allMarkTypes": marks,
                })
    add("filter_overflow", "_default_overflow_strategy",
        [("x", "F", u, k, c) for u in OF_UNIQUE[:5] for k in (2, 3, 100) for c in OF_CTX[:18]])

    OF_CS = [
        {"x": {"field": "F", "type": "nominal"}, "y": {"field": "V", "type": "quantitative"}},
        {"x": {"field": "F", "type": "ordinal"}},
        {"x": {"field": "F", "type": "quantitative"}},
        {"column": {"field": "F", "type": "nominal"}},
        {"column": {"field": "F", "type": "quantitative"}},
        {"color": {"field": "F", "type": "nominal"}},
        {"group": {"field": "F", "type": "nominal"}},
        {},
    ]
    OF_DATA = [
        [{"F": c, "V": i} for i, c in enumerate("abcdefgh")],
        [{"F": "a", "V": 1}], [],
        [{"F": str(i), "V": i} for i in range(10)],
    ]
    OF_BUDGETS = [
        {}, {"maxValues": {}}, {"maxValues": {"x": 3}}, {"maxValues": {"x": 100}},
        {"maxValues": {"column": 2}}, {"maxValues": {"color": 2}},
    ]
    add("filter_overflow", "filter_overflow",
        [(cs, {}, {"x": {}}, d, b, set(m))
         for cs in OF_CS for d in OF_DATA for b in OF_BUDGETS for m in ([], ["bar"])])

    # --- flint/core/decisions.py ------------------------------------------
    add("decisions", "_vis_category_to_vl_type",
        [(v,) for v in ["quantitative", "ordinal", "temporal", "geographic", "nominal", "", "x"]])
    # Inherited: `_looks_temporal_value` calls `is_js_parseable`, so the three
    # dateutil-tokenizer strings registered under js_date surface here too.
    # Same root cause, same reason — see PORT-DICTIONARY.
    add("decisions", "_looks_temporal_value",
        [(v,) for v in _scalar_values() + _strings() + [1500, 2200, 1499, 2201, 1500.5,
                                                        86400001, 4199999999999, 4200000000001]],
        known={k: v + " (inherited via is_js_parseable)"
               for k, v in DATEUTIL_TOKENIZER.items()})
    add("decisions", "_can_parse_float", [(s,) for s in _strings()])

    DEC_DATA = [
        [{"F": "2020-01-01"}, {"F": "2020-02-01"}, {"F": "2020-03-01"}],
        [{"F": "a"}, {"F": "b"}, {"F": "c"}],
        [{"F": i} for i in range(20)],
        [{"F": 2020}, {"F": 2021}, {"F": 2022}],
        [{"F": 1.5}, {"F": 2.5}, {"F": 3.5}],
        [{"F": "a"}], [], [{"F": None}, {"F": "a"}],
        [{"F": str(i)} for i in range(15)],
    ]
    CHANNELS = ["x", "y", "color", "group", "size", "column", "row"]
    add("decisions", "_validate_temporal_parsing",
        [(d, "F", fr) for d in DEC_DATA for fr in (True, False)])
    add("decisions", "_resolve_temporal_encoding",
        [("temporal", ch, d, "F", fr) for ch in CHANNELS for d in DEC_DATA[:6] for fr in (True, False)])
    add("decisions", "_apply_ordinal_guards",
        [("ordinal", ch, d, "F", [r.get("F") for r in d], fr)
         for ch in CHANNELS for d in DEC_DATA for fr in (True, False)])
    CANDIDATE_SETS = [
        ["temporal", "ordinal"], ["quantitative", "ordinal"], ["quantitative", "geographic"],
        ["ordinal", "nominal"], ["nominal"], ["quantitative"], ["temporal"], ["geographic"],
    ]
    add("decisions", "_disambiguate_multi_encoding",
        [(c, ch, d, "F", [r.get("F") for r in d])
         for c in CANDIDATE_SETS for ch in CHANNELS for d in DEC_DATA[:5]])
    add("decisions", "resolve_encoding_type",
        [(t, [r.get("F") for r in d], ch, d, "F")
         for t in types[:20] + ["", "Nope"] for ch in ("x", "color") for d in DEC_DATA[:5]])

    GP_PARAMS = [
        None, {}, {"elasticity": 0.5}, {"maxStretch": 3.0}, {"markCrossSection": 60},
        {"markCrossSectionX": 10}, {"markCrossSectionY": 10},
        {"xItemCountOverride": 50}, {"yItemCountOverride": 50},
        {"markCrossSectionX": 0}, {"xItemCountOverride": 50, "markCrossSectionX": 0},
    ]
    GP_VALS = [[], [1.0], [float(i) for i in range(20)], [1.0, 1.0, 1.0], [0.0, 100.0]]
    add("decisions", "compute_gas_pressure",
        [(xv, xv, [0, 100], [0, 100], w, h, pr)
         for xv in GP_VALS for (w, h) in ((400, 300), (0, 300), (400, 0)) for pr in GP_PARAMS])

    EB_PARAMS = [
        {"defaultStepSize": 20, "maxStretch": 2.0, "elasticity": 0.5},
        {"defaultStepSize": 6, "maxStretch": 1.5, "elasticity": 0.3},
    ]
    add("decisions", "compute_elastic_budget",
        [(n, d, p) for n in (0, 1, 10, 100, 1000) for d in (100, 400.0) for p in EB_PARAMS])
    add("decisions", "compute_axis_step",
        [(n, c, d, p) for n in (0, 5, 50) for c in (0, 5, 50) for d in (100, 400.0) for p in EB_PARAMS])
    FL_PARAMS = [
        {"minSubplotSize": 60, "maxStretch": 2.0, "facetElasticity": 0.3},
        {"minSubplotSize": 20, "maxStretch": 1.2, "facetElasticity": 0.5},
    ]
    add("decisions", "compute_facet_layout",
        [(c, r, 400, 300, p) for c in (1, 2, 5, 12) for r in (1, 2, 5) for p in FL_PARAMS])
    add("decisions", "compute_label_sizing",
        [(s, h) for s in (0, 5, 9, 9.5, 10, 15, 16, 40, 100) for h in (True, False)])
    add("decisions", "compute_overflow",
        [(u, d, m) for u in (0, 5, 100) for d in (100, 400.0) for m in (6, 20, 0.5)])
    CP_PARAMS = [
        None, {}, {"minArcPx": 20}, {"maxRadius": 100}, {"minRadius": 10},
        {"maxStretchX": 1.0, "maxStretchY": 3.0}, {"margin": 0}, {"elasticity": 0.9},
    ]
    add("decisions", "compute_circumference_pressure",
        [(n, {"width": w, "height": h}, p)
         for n in (0, 5, 60, 500) for (w, h) in ((400, 300), (100, 100)) for p in CP_PARAMS])
    add("decisions", "compute_effective_bar_count",
        [(v,) for v in [[], [1.0], [1.0, 2.0, 3.0], [-1.0, -2.0], [0.0, 0.0],
                        [1.0, 1000.0], [0.001, 1.0], [100.0] * 5]])

    # --- flint/core/compute_layout.py -------------------------------------
    add("compute_layout", "_is_finite_number", [(s,) for s in _strings()])
    add("compute_layout", "_js_to_number", [(v,) for v in _scalar_values()])
    add("compute_layout", "_js_to_date_number", [(v,) for v in _scalar_values() + _strings()[:30]],
        known={k: v + " (inherited via js_date_parse_ms)" for k, v in DATEUTIL_TOKENIZER.items()})
    add("compute_layout", "_is_nan", [(v,) for v in _scalar_values()])

    CL_TABLES = [
        [{"F": c, "V": i * 1.0} for i, c in enumerate("abcdefgh")],
        [{"F": "2020-0%d-01" % (i + 1), "V": i * 1.5} for i in range(9)],
        [{"F": i * 1.0, "V": i * 2.0} for i in range(30)],
        [{"F": "a", "V": 1}], [],
        [{"F": str(i), "G": "g%d" % (i % 3), "V": i} for i in range(12)],
    ]
    add("compute_layout", "_compute_discrete_label_stats",
        [(f, t) for f in ("F", "V", "Missing", None) for t in CL_TABLES])
    add("compute_layout", "_discrete_y_axis_should_use_horizontal_labels",
        [(f, ct, t) for f in ("F", None) for ct in ("quantitative", "nominal", "ordinal", None)
         for t in CL_TABLES])
    add("compute_layout", "resolve_base_size",
        [(b, c) for b in (None, {"width": 500, "height": 400}, {"width": 100, "height": 100})
         for c in (None, {"width": 300, "height": 200}, {"width": 1000, "height": 1000})])
    OPT_SETS = [
        {}, {"maxStretch": 3}, {"maxStretchX": 1.2}, {"maxStretchY": 4},
        {"maxStretch": None}, {"maxStretchX": 0.5}, {"minStep": 10, "stepPadding": 0.2},
        {"minSubplotSize": 100}, {"facetGap": 10, "facetFixedPadding": {"width": 50, "height": 40}},
        {"targetBandAR": 10}, {"defaultBandSize": 30}, {"facetElasticity": 0.5},
        {"elasticity": 0.8}, {"facetAspectRatioResistance": 0.5},
        {"continuousMarkCrossSection": 40},
        {"continuousMarkCrossSection": {"x": 20, "y": 20}},
        {"continuousMarkCrossSection": {"x": 20, "y": 20, "seriesCountAxis": "auto"}},
        {"continuousMarkCrossSection": {"x": 20, "y": 20, "seriesCountAxis": "x",
                                        "elasticity": 0.4, "maxStretch": 2.5}},
    ]
    add("compute_layout", "resolve_stretch_caps", [(o,) for o in OPT_SETS])
    add("compute_layout", "derive_stretch_caps",
        [({"width": 400, "height": 320}, c, o)
         for c in (None, {"width": 800, "height": 640}, {"width": 200, "height": 160})
         for o in OPT_SETS[:6]])

    CL_CS = [
        {"x": {"field": "F", "type": "nominal"}, "y": {"field": "V", "type": "quantitative"}},
        {"x": {"field": "F", "type": "temporal"}, "y": {"field": "V", "type": "quantitative"}},
        {"x": {"field": "F", "type": "quantitative"}, "y": {"field": "V", "type": "quantitative"}},
        {"x": {"field": "F", "type": "quantitative", "scaleType": "log"},
         "y": {"field": "V", "type": "quantitative"}},
        {"x": {"field": "F", "type": "nominal"}, "y": {"field": "V", "type": "quantitative"},
         "group": {"field": "G", "type": "nominal"}},
        {"x": {"field": "F", "type": "nominal"}, "column": {"field": "G", "type": "nominal"}},
        {"x": {"field": "F", "type": "quantitative"}, "y": {"field": "V", "type": "quantitative"},
         "color": {"field": "G", "type": "nominal"}},
        {"x": {"field": "F", "type": "quantitative",
               "zero": {"zero": True}}, "y": {"field": "V", "type": "quantitative"}},
        {},
    ]
    CL_DECL = [
        {}, {"resolvedTypes": {"x": "nominal"}}, {"resolvedTypes": {"y": "nominal"}},
        {"axisFlags": {"x": {"banded": True}}}, {"axisFlags": {"y": {"banded": True}}},
        {"binnedAxes": {"x": {"maxbins": 10}}}, {"binnedAxes": {"x": True}},
    ]
    CANVAS = [{"width": 400, "height": 320}, {"width": 200, "height": 600}]
    add("compute_layout", "count_distinct_series",
        [(cs, t) for cs in CL_CS for t in CL_TABLES])
    add("compute_layout", "compute_facet_grid",
        [(cs, d, t, c, o) for cs in CL_CS for d in CL_DECL[:4] for t in CL_TABLES[:4]
         for c in CANVAS[:1] for o in OPT_SETS[:8]])
    add("compute_layout", "compute_channel_budgets",
        [(cs, d, t, c, o) for cs in CL_CS for d in CL_DECL[:4] for t in CL_TABLES[:4]
         for c in CANVAS[:1] for o in OPT_SETS[:8]])
    add("compute_layout", "compute_min_subplot_dimensions",
        [(cs, d, t, o) for cs in CL_CS for d in CL_DECL for t in CL_TABLES for o in OPT_SETS[:4]])
    BANK_PTS = [
        ([1.0, 2.0, 3.0, 4.0], [1.0, 2.0, 3.0, 4.0]),
        ([1.0, 2.0, 3.0, 4.0], [4.0, 3.0, 2.0, 1.0]),
        ([float(i) for i in range(16)], [float(i * i) for i in range(16)]),
        ([1.0, 1.0], [1.0, 2.0]), ([1.0, 2.0], [1.0, 1.0]),
    ]
    add("compute_layout", "compute_banking_ar",
        [(xs, ys, [min(xs), max(xs)], [min(ys), max(ys)], ["s"] * len(xs), conn)
         for (xs, ys) in BANK_PTS for conn in (True, False)]
        + [(xs, ys, [0.0, 0.0], [0.0, 1.0], ["s"] * len(xs), False) for (xs, ys) in BANK_PTS[:2]])
    add("compute_layout", "compute_layout",
        [(cs, d, t, c, o, None) for cs in CL_CS for d in CL_DECL for t in CL_TABLES
         for c in CANVAS for o in OPT_SETS[:6]])

    return specs
