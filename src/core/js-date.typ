// Port of flint/core/js_date.py.
//
// PORT-DATE. This module is the one place where the port is a *delegation*
// rather than a transcription. Upstream's 198 lines exist to make Python's
// date handling behave like V8's `Date.parse`: strict ISO first, then V8's
// numeric-date rules, then `dateutil` for free-form input, then V8's
// trailing-year heuristic. That is exactly what datehog was written to do, and
// datehog is checked against flint-py directly — 445/445 on every date-shaped
// string in the fixture corpus plus hand-picked edges (`datehog/tests/compare.py`).
//
// Re-transcribing the layering here would duplicate that logic in a second
// place and give the two copies room to drift. So each upstream function maps
// to its datehog equivalent, and the mapping is spelled out below.
//
//   _looks_like_word_plus_year  ->  dh.year-from-words
//   _try_v8_numeric_date        ->  dh.parse-numeric
//   js_date_parse_ms            ->  dh.js.parse-ms
//   js_date_parse               ->  dh.js.parse
//   is_js_parseable             ->  dh.js.is-parseable
//
// One representational difference: upstream returns Python `datetime` objects
// and this returns datehog moments. Nothing recorded in the conformance corpus
// contains a parsed date — `analyze_temporal_field`'s `dates` list stays
// internal to `_resolve_temporal_format` — so the representation is free.

#import "@local/datehog:0.1.0" as dh
#import "py.typ": is-finite

/// Upstream's `MAX_TIMESTAMP_SEC` / `MAX_TIMESTAMP_MS`, used by
/// `resolve_semantics` to tell a Unix timestamp from an ordinary number.
#let MAX_TIMESTAMP_SEC = dh.js.MAX-TIMESTAMP-SECONDS
#let MAX_TIMESTAMP_MS = dh.js.MAX-TIMESTAMP-MS

// flint/core/js_date.py _looks_like_word_plus_year
//
// V8 accepts "FY 2018" / "May 2018" / "hello world 2018" by extracting the
// trailing year, provided every other token is a pure-letter word.
#let _looks_like_word_plus_year(s) = dh.year-from-words(s)

// flint/core/js_date.py _try_v8_numeric_date
//
// V8 reads `01/15/2020` as month-first and rejects `15.01.2020` outright,
// where a lenient parser would read the latter as 15 January. datehog
// implements V8's rule; upstream returns a naive datetime here and assigns the
// zone at the call site, which under this port's UTC policy is the same thing.
#let _try_v8_numeric_date(s) = dh.parse-numeric(s)

// ---------------------------------------------------------------------------
// The dateutil fallback
// ---------------------------------------------------------------------------
//
// Upstream's step 3 is `dateutil.parser.parse(s, default=datetime(1970,1,1))`,
// which fills missing components from that default and so accepts inputs that
// are not really dates at all. Measured against the installed dateutil:
//
//   "Jan", "DEC"  -> that month of 1970
//   "Monday"      -> the first such weekday on/after 1970-01-01
//   "1" .. "31"   -> that day of January 1970
//   "32" .. "99"  -> a two-digit YEAR (see the pivot note below)
//   "100"+        -> that year
//   "10-19"       -> 1970-10-19; month must be 1-12, so "20-29" is rejected
//   "0", "1e5"    -> rejected
//
// This matters because real columns look like this: `"10-19"` and `"20-29"`
// (age bands) appear 63 times in the fixture corpus. Both sides must agree, or
// a band column could be read as temporal in one implementation and not the
// other.
#let _WEEKDAY_INDEX = {
  let m = (:)
  for (i, name) in dh.WEEKDAY-NAMES.enumerate() { m.insert(lower(name), i + 1) }
  for (i, name) in dh.WEEKDAY-ABBR.enumerate() { m.insert(lower(name), i + 1) }
  m
}

#let _BARE_INT_RE = regex("^\\d{1,4}$")
#let _TWO_PART_RE = regex("^(\\d{1,2})[-/](\\d{1,2})$")

// PORT-NUM: dateutil resolves a two-digit year against the *current* year --
// it picks the candidate within 50 years of today, so "98" is 1998 today but
// would become 2098 to a reader in 2049. Reproducing that would make a
// document's output depend on the date it was compiled, which is exactly the
// non-reproducibility this port avoids elsewhere. The reference year is pinned
// instead. Matches upstream for every input until 2076.
#let _DATEUTIL_REFERENCE_YEAR = 2026
#let _expand_dateutil_year(n) = {
  let candidate = 2000 + n
  if candidate - _DATEUTIL_REFERENCE_YEAR >= 50 { candidate - 100 } else { candidate }
}

#let _dateutil_fallback(s) = {
  let t = s.trim()
  if t.match(_BARE_INT_RE) != none {
    let n = int(t)
    if n == 0 { return none }
    if n <= 31 { return dh.from-parts(1970, 1, n) }
    if n <= 99 { return dh.from-parts(_expand_dateutil_year(n), 1, 1) }
    return dh.from-parts(n, 1, 1)
  }
  let m = t.match(_TWO_PART_RE)
  if m != none {
    let month = int(m.captures.at(0))
    let day = int(m.captures.at(1))
    if dh.is-valid-date(1970, month, day) { return dh.from-parts(1970, month, day) }
    return none
  }
  // A bare month name is that month of the default year: "Jan" is 1970-01-01,
  // "DEC" is 1970-12-01. Common enough in real data to matter — a column of
  // month abbreviations reaches here whenever the field is typed temporal.
  let mo = dh.month-from-name(t)
  if mo != none { return dh.from-parts(1970, mo, 1) }
  // A bare weekday name resolves to the first such weekday on or after the
  // 1970-01-01 default: "Monday" is 1970-01-05, "Sunday" is 1970-01-04.
  let wd = _WEEKDAY_INDEX.at(lower(t), default: none)
  if wd != none {
    // 1970-01-01 was a Thursday (ISO weekday 4).
    let delta = calc.rem-euclid(wd - 4, 7)
    return dh.from-parts(1970, 1, 1 + if delta == 0 { 7 } else { delta })
  }
  none
}

// flint/core/js_date.py js_date_parse_ms
//
// Milliseconds since the epoch, mirroring `Date.parse`. `none` where V8 gives
// `NaN`. Numbers pass through, booleans coerce to 1/0.
//
// PORT-NUM: upstream is annotated `Optional[float]` and always returns a float
// (`.timestamp() * 1000.0`); datehog returns an int, since epoch milliseconds
// are exact integers. Coerced here so the numeric *kind* matches upstream —
// int-vs-float is a real difference downstream, where these values reach
// layout arithmetic and JSON output.
#let js_date_parse_ms(value) = {
  // Upstream passes any int/float straight through, including NaN and the
  // infinities — `isinstance(v, (int, float))` then `return float(v)`, with no
  // finiteness check. datehog rejects non-finite input, so they are restored
  // here rather than weakening datehog for everyone.
  if type(value) == float and not is-finite(value) { return value }
  let ms = dh.js.parse-ms(value)
  if ms != none { return float(ms) }
  // datehog stops where V8 does; upstream then falls through to dateutil.
  if type(value) != str { return none }
  let m = _dateutil_fallback(value)
  if m == none { return none }
  float(dh.to-ms(m))
}

// flint/core/js_date.py js_date_parse
//
// Upstream returns a timezone-aware `datetime`; this returns a datehog moment.
// Built from the ported `js_date_parse_ms` for the same reason as above.
#let js_date_parse(value) = {
  let ms = js_date_parse_ms(value)
  if ms == none { return none }
  // PORT-EXC: `int(inf)` raises OverflowError upstream, and NaN raises too.
  // Typst cannot catch, so the non-finite values `js_date_parse_ms` passes
  // through become `none` here instead of aborting the document.
  if not is-finite(ms) { return none }
  // PORT-NUM: upstream does `int(ms)` before splitting into seconds and
  // milliseconds, and Python's `int()` truncates toward zero. datehog's
  // `from-ms` floors, which differs for negative fractional input:
  // `int(-2.5)` is -2, `floor(-2.5)` is -3.
  dh.from-ms(calc.trunc(ms))
}

// flint/core/js_date.py is_js_parseable
//
// Upstream is literally `js_date_parse_ms(value) is not None`, so it must go
// through the *ported* parse — delegating to `dh.js.is-parseable` would skip
// the dateutil fallback above and disagree on "98", "10-19" and weekday names.
#let is_js_parseable(value) = js_date_parse_ms(value) != none
