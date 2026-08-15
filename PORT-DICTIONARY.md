# Port divergence register

Every place the Typst port behaves differently from flint-py, or is written
differently enough that an upstream diff will not apply cleanly. Markers appear
in the source as `// PORT-XXX:`.

This file is what makes an upstream bump tractable: diff flint-py, and for each
changed function check here before applying.

**NaN and infinity** are tested with `py.typ`'s `is-nan` / `is-finite`, which
wrap Typst's own `float.is-nan()` and `float.is-infinite()`. Those methods exist
only on `float` — an `int` has neither — so the type guard lives in the wrapper
and no site open-codes `v != v`.

**Upstream pinned at:** `microsoft/flint-chart`, `flint-source/` shallow clone
taken 2026-08-15 (flint-py 0.1.0).

**Rule:** port upstream bugs faithfully. The conformance corpus is generated
*from* flint-py, so "fixing" something it gets wrong makes the gate fail.
Genuine bugs go in [Observations](#observations) and stay unfixed.

## Markers

| marker | meaning |
|---|---|
| `PORT-MUT` | in-place mutation threaded through a return instead |
| `PORT-EXC` | `try`/`except` replaced by a pre-validation guard |
| `PORT-DATE` | Python `datetime` replaced by datehog |
| `PORT-NUM` | numeric or number→string behaviour differs |
| `PORT-PERF` | restructured for speed |
| `PORT-IDIOM` | pure syntax, no behaviour change — listed only where non-obvious |

## Register

| module | function | marker | what and why |
|---|---|---|---|
| `__init__` | `js_round` | `PORT-NUM` | Upstream is `math.floor(x + 0.5)`, matching JS `Math.round` (half toward +∞). Typst's `calc.round` rounds half *away from zero* — `calc.round(-2.5)` is -3 where `Math.round(-2.5)` is -2 — so `calc.floor(x + 0.5)` is used. |
| `type_registry` | `TYPE_REGISTRY`, `UNKNOWN_ENTRY` | `PORT-IDIOM` | **Generated**, not hand-ported, into `src/core/type-registry-data.typ` by `test/gen_tables.py`. 45 entries × 9 fields of pure data; transcribing by hand would risk a silent wrong value no reviewer would catch. Regenerate on upstream bump; `--check` fails CI on drift. |
| `types` | `channels`, `channelGroups` | `PORT-IDIOM` | Generated, same reason. |
| `color_decisions` | `_count_distinct_values` | `PORT-IDIOM` | Upstream builds a Python `set`; Typst dictionary keys must be strings, so `array.dedup()` is used. It compares with `==` rather than hashing, which agrees with Python on the value kinds reaching this function. Possible `PORT-PERF` site if it ever appears in a hot path — `dedup` is not hash-based. |
| `encoding_actions` | `make_sort_action` | `PORT-IDIOM` | `set` is a reserved word in Typst, so the action dictionary's `set` key is quoted and read with `.at("set")`. The key *name* is unchanged — it is part of the template contract. |
| `encoding_actions` | `_resolve_sort_channels` | `PORT-IDIOM` | `next((c for c in candidates if pred(c)), None)` → `candidates.find(pred)`. |
| `semantic_types` | `measureTypes`, `categoricalTypes`, `ordinalTypes` | `PORT-IDIOM` | Upstream builds Python `set`s by comprehension. Typst has no set type, so these are arrays and membership is `in`. ~20 entries, membership-only, so the linear scan is immaterial. |
| `semantic_types` | `_is_number_like` | `PORT-EXC` | `try: float(s)` becomes a regex shape check (`_is_float_string`) — Typst cannot catch, and a failed `float()` aborts the whole document. The pattern accepts exactly what Python's `float()` does, including `inf`/`nan` words. |
| `semantic_types` | `_date_parse_succeeds` | `PORT-DATE` `PORT-EXC` | Upstream tries `datetime.fromisoformat` then `email.utils.parsedate_to_datetime`, both under bare `except`. Ported as shape check + datehog parse. **Deliberately narrower than `dh.parse-ms`**: the acceptance sets were measured against CPython 3.14 — ISO needs a *complete* date (`2020` and `2020-01` are rejected), RFC 2822 needs a time (`15 Feb 2020` is rejected). Using datehog's general parser would classify far more fields as temporal than flint-py does. Not covered: ISO week dates (`2020-W01-1`), which `fromisoformat` accepts and no fixture uses. |
| `semantic_types` | `_pick_scheme` | `PORT-IDIOM` | JS 32-bit signed hash. Typst's int has `bit-lshift`/`bit-and`, so it transcribes operator for operator; the mask and sign reinterpretation match Python's. |
| `semantic_types` | `_match_sequence` | `PORT-NUM` | Uses `py_str`, not Typst's `str`: Typst renders `1.0` as `"1"` and Python as `"1.0"`, and this function compares stringified values against a month-number table `("1", ..., "12")`. Under Typst's `str` a float `1.0` would wrongly match. Sort stability (which Typst has, like Python) matters when two differently-cased labels collapse to one index. |
| `semantic_types` | `compute_zero_decision`, `get_recommended_color_scheme` | `PORT-IDIOM` | Python's positional-with-default parameters become **named** parameters in Typst, which has no positional default. Parameter names are unchanged; `test/dispatch.typ` carries an adapter so `cases.py` can keep mirroring the Python call shape. |
| `js_date` | *(whole module)* | `PORT-DATE` | Delegates to **datehog** rather than transcribing. Upstream's 198 lines exist to make Python behave like V8's `Date.parse`; datehog was written for exactly that and is checked against flint-py directly (445/445 in `datehog/tests/compare.py`). Re-transcribing would give the two copies room to drift. Returns datehog moments where upstream returns `datetime` — nothing in the corpus carries a parsed date, so the representation is free. |
| `js_date` | `js_date_parse_ms` | `PORT-NUM` | Upstream returns `Optional[float]`; datehog returns an int. Coerced to float so the numeric *kind* matches, since these values reach layout arithmetic and JSON output. Non-finite floats are passed through as upstream does (`isinstance(v, (int,float))` with no finiteness check). |
| `js_date` | `js_date_parse` | `PORT-NUM` `PORT-EXC` | Upstream's `int(ms)` truncates toward zero; datehog's `from-ms` floors, which differs for negative fractions (`int(-2.5)` = -2, `floor` = -3) — `calc.trunc` is used. `int(inf)` raises `OverflowError` upstream; that becomes `none` here. |
| `js_date` | `_dateutil_fallback` | `PORT-NUM` | Upstream's step 3 is `dateutil.parse(s, default=1970-01-01)`, which fills missing components and so accepts non-dates. Reproduced for the shapes that occur in real data — bare **month names** (`"Jan"` → 1970-01-01, `"DEC"` → 1970-12-01), `"1".."31"` → day, `"32".."99"` → two-digit year, `"10-19"` → month-day (63 occurrences in the corpus as age bands), weekday names → first such day on/after 1970-01-01. **Two-digit years use a pinned reference year (2026), not the current one:** dateutil resolves them against `today`, so `"98"` is 1998 now and would become 2098 to a reader in 2049. Matching that would make document output depend on the compile date. Matches upstream for every input until 2076. |
| `field_semantics` | `_detect_precision` | `PORT-NUM` | Upstream renders each value with `"{:.10f}".format(...)` and counts the digits left after stripping trailing zeros. Typst has no fixed-point formatting, so the same question is asked directly: the fewest decimal places at which `calc.round` still equals the 10-dp rounding. Equivalent by construction — stripping trailing zeros from a 10-dp rendering *is* finding that minimum. Verified on the awkward cases (`0.1 + 0.2`, `1/3`). |
| `field_semantics` | `_try_float` | `PORT-EXC` | Shares `py.typ`'s `is-float-string` with `semantic_types._is_number_like`; both stand in for `try: float(s)`. |
| `field_semantics` | `resolve_default_vis_type` | `PORT-IDIOM` | Upstream's `all_numeric` test relies on Python precedence: `A and B or C` is `(A and B) or C`, so a numeric value passes on the left and a numeric *string* on the right. Parenthesised explicitly in the port. |
| `field_semantics` | `resolve_stackable` | `PORT-IDIOM` | Returns `"sum"`, `"normalize"` or `false` — the mixed return type is upstream's and is load-bearing at the call site, so it is preserved rather than normalised to `none`. |
| `field_semantics` | `resolve_zero_class_from_annotation`, `resolve_tick_constraint`, `resolve_reversed`, `resolve_nice`, `resolve_binning_suggested`, `_precision_format` | `PORT-IDIOM` | Positional-with-default parameters become named, as elsewhere; `test/dispatch.typ` adapts. |
| `resolve_semantics` | `infer_implicit_semantic_type` | `PORT-IDIOM` `PORT-EXC` | `re.sub(p, r"\1 \2", s)` becomes a replacement function reading `m.captures` (no backreferences in Typst's regex engine). `float(value)` under `except (TypeError, ValueError)` becomes an explicit number-or-numeric-string test. |
| `resolve_semantics` | `compute_data_votes` | `PORT-IDIOM` | Upstream writes `votes[5] += 1`. Factoring the increment into a helper closure does **not** work in Typst — a closure captures outer locals by value and cannot assign to them — so the increments stay inline, which happens to match upstream exactly. |
| `resolve_semantics` | `convert_temporal_data` | `PORT-MUT` | Upstream deep-copies the table and mutates rows in place. Typst copies on assignment, so rows are rebuilt from the original instead. Same result, no aliasing to reason about — and this is the mutation class the plan warned about, arriving exactly where predicted. |
| `resolve_semantics` | `_expand_to_full_year`, `_to_iso_z` | `PORT-DATE` | Delegate to datehog's `expand-two-digit-year` and `to-iso`, which implement the same rules (00-49 → 20xx; `Date.toISOString()` shape). |
| `filter_overflow` | `_default_overflow_strategy` | `PORT-IDIOM` | Upstream's last parameter is named `context`, a reserved word in Typst. Renamed `ctx` — the only upstream *parameter name* the port could not preserve. Passed positionally, so no call site changes. |
| `filter_overflow` | `_default_overflow_strategy` | `PORT-EXC` | `json.loads(sort_by)` under a bare `except` becomes a strict JSON-array-of-scalars regex followed by `json(bytes(..))`, since Typst's decoder cannot be caught. Covers what a stored custom sort order is; anything more exotic falls through to the same branch the `except` did. |
| `filter_overflow` | `_default_overflow_strategy` | `PORT-IDIOM` | The `value_aggregates` dict is keyed by raw field values; Typst dictionary keys must be strings, so the accumulator is parallel `values`/`totals` arrays with `==` lookup. Matches Python, whose dict also treats `1` and `1.0` as one key. O(n·u) rather than O(n), with `u` bounded by the layout budget. |
| `filter_overflow` | `_default_overflow_strategy` | `PORT-NUM` | Sorting by a `float()` key can produce NaN. **Typst errors on comparing NaN with NaN**; Python's sort tolerates it, since every NaN comparison is False and Timsort leaves those elements in place. With *every* key NaN — a string column on the colour channel, the case that actually occurs — Python returns the input untouched, and that is reproduced exactly. Mixed NaN/number keys sort NaNs last in original order, which is *not* Timsort's arrangement; see Known divergences. |
| `filter_overflow` | `filter_overflow` | `PORT-IDIOM` | `sorted(..., reverse=True)` is stable in Python and leaves equal elements in original order; Typst's `sorted(..).rev()` reverses them too. `sorted-desc` does `rev().sorted().rev()`, which is how CPython implements `reverse=True` and matches exactly. |
| `decisions` | `_looks_temporal_value`, `compute_effective_bar_count` | `PORT-NUM` | **Typst errors on any ordering comparison involving NaN** (`nan >= 1500` aborts); Python returns False. Both sites can receive NaN from a data column, so NaN is rejected before the comparison — which is the answer Python reaches anyway. Second occurrence of this hazard after `filter_overflow`; treat every `<`/`>` on a data-derived number as suspect. |
| `decisions` | `_apply_ordinal_guards` | `PORT-EXC` `PORT-NUM` | `try: float(v) / except ValueError` becomes `is-float-string`. The `v % 1 != 0` fraction test survives unchanged: Python and Typst disagree on the *sign* of a negative remainder but not on whether it is zero, which is all the test asks. |
| `decisions` | `compute_gas_pressure` | `PORT-IDIOM` | `{**DEFAULT, **params}` is `(: ..DEFAULT, ..params)` — the leading `:` is required, or a parenthesised list of only spreads parses as an array. The two nested closures port directly: they only *read* the captured params. |
| `resolve_semantics` | `resolve_channel_semantics` | `PORT-IDIOM` | `list({v for v in field_values})` only ever has its length taken, so `.dedup()` stands in for Python's set; both collapse `1` and `1.0`. The ISO-datetime override test uses `py_str`, since upstream stringifies with `str(v)` before matching. |
| *(all)* | Python truthiness | `PORT-IDIOM` | `if not encoding`, `if hint` etc. have no Typst equivalent — `not none` is a type error. Encoded once in `src/core/py.typ` as `truthy`/`falsy` and used at every site rather than translated ad hoc. |
| *(all)* | `str(v)` on negative numbers | `PORT-NUM` | **Typst renders a negative number with U+2212 MINUS SIGN, not ASCII `-`.** `str(-2.5)` is a three-codepoint string starting 8722. Correct for typesetting, wrong for every string comparison, JSON payload and data value here. `py.typ`'s `num-str` patches it and every numeric stringification in the port goes through it. lilaq hits the same thing and fixes it identically (`src/logic/tick-format.typ:59`). Caught only because a differential case happened to include `-2.5` — it had been latent through four modules. |
| *(all)* | `str(v)` on floats | `PORT-NUM` | Typst's `str(1.0)` is `"1"`; Python's is `"1.0"`. `py.typ`'s `py_str` restores the Python behaviour and is used wherever upstream stringifies a value for comparison. Python's exponent thresholds (1e16, 1e-5) are not reproduced; no fixture reaches them through a stringifying path. |

## Observations

Upstream behaviour that looks wrong but is reproduced faithfully.

| where | what |
|---|---|
| `js_date` | V8's `Date.parse` accepts `"Stage 1"`, `"Round 2"`, `"Wk 01"` as dates, reading the trailing digit as a month. flint-py rejects them; datehog follows flint-py. This means flint's **JS** implementation would classify a `"Stage"` column as temporal where the Python one does not — plausibly one of the 180 fixture tests where flint-py and the JS reference disagree. Not acted on. |

## Known divergences

Deliberate, allowlisted in `test/cases.py` so the differential stays green
while still counting and printing them on every run.

| where | inputs | why not matched |
|---|---|---|
| `filter_overflow._default_overflow_strategy` | mixed NaN/number sort keys | Python's Timsort produces an implementation-defined arrangement when the key order is not total (`sorted(['x', 3, 1], key=float-or-nan)` gives `[1, 'x', 3]`). Reproducing it means reimplementing Timsort. The all-NaN case — the one that occurs in the corpus, and the only one reachable from a real column — is matched exactly. No corpus fixture exercises the mixed case; if one appears, the gate will say so. |
| `js_date.js_date_parse_ms` and friends | `"1.5"`, `".5"`, `"-2"` | dateutil's tokenizer reads a leading digit run out of these and returns a day of January 1970 (`".5"` → 1970-01-05). Pinning its exact tokenizer would cost more than it protects: none of these is a date by any reasonable reading, and the plausible shapes are reproduced. Note the asymmetry that makes a simple rule impossible — dateutil accepts `"-2"` but rejects `"+3"`. |

## Timezone policy

`PORT-DATE`. flint-py reads zoneless date-time strings in the host's local zone
(`.astimezone()`). Typst cannot discover the local UTC offset — plugins are
sandboxed with one host import and no clock, and `datetime.today()` reports a
date but no time — so the port reads them as UTC, via datehog.

This affects 15 of the 705 corpus fixtures (`dates_hours__*`,
`dates_year_month__*_mon_yyyy_*`), which recorded machine-dependent values. The
corpus is now generated with `TZ=UTC` (`transpile/Makefile`) so it is
reproducible and matches the port. Documents that know their data's zone can
pass `--input tz=...` and read it with datehog's `local-offset`.
