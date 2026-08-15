// Calendar-aware tick placement for a temporal axis.
//
// Ours; no upstream counterpart. It exists because of a gap between the two
// halves:
//
//   * flint core decides the tick *format* but never tick *positions* — its
//     original backend was Vega-Lite, whose time scale places calendar-aware
//     ticks itself.
//   * lilaq has no date scale. A temporal channel reaches it as epoch
//     milliseconds, so its linear locator produces round *numbers*, which land
//     on arbitrary instants: "Jan 01 2020, Aug 07 2020, Mar 14 2021".
//
// So the backend places them. A tick on a time axis should fall on a boundary a
// reader recognises — a year, a quarter, a month, a Monday, a midnight — and
// the label should be written at the granularity of that boundary rather than
// the granularity of the data.

#import "@local/datehog:0.1.0" as dh

// Candidate steps, coarsest first. `level` is flint's own format level (see
// `level_to_format` in core/resolve-semantics.typ), so the label granularity
// follows the tick unit rather than being guessed separately.
//
//   5 year · 4 month · 3 day · 2 hour · 1 minute · 0 second
#let _UNITS = (
  (kind: "year", n: 1000, level: 5),
  (kind: "year", n: 500, level: 5),
  (kind: "year", n: 200, level: 5),
  (kind: "year", n: 100, level: 5),
  (kind: "year", n: 50, level: 5),
  (kind: "year", n: 20, level: 5),
  (kind: "year", n: 10, level: 5),
  (kind: "year", n: 5, level: 5),
  (kind: "year", n: 2, level: 5),
  (kind: "year", n: 1, level: 5),
  (kind: "month", n: 6, level: 4),
  (kind: "month", n: 3, level: 4), // quarters
  (kind: "month", n: 1, level: 4),
  (kind: "day", n: 7, level: 3), // weeks
  (kind: "day", n: 1, level: 3),
  (kind: "hour", n: 12, level: 2),
  (kind: "hour", n: 6, level: 2),
  (kind: "hour", n: 3, level: 2),
  (kind: "hour", n: 1, level: 2),
  (kind: "minute", n: 30, level: 1),
  (kind: "minute", n: 15, level: 1),
  (kind: "minute", n: 5, level: 1),
  (kind: "minute", n: 1, level: 1),
  (kind: "second", n: 30, level: 0),
  (kind: "second", n: 15, level: 0),
  (kind: "second", n: 5, level: 0),
  (kind: "second", n: 1, level: 0),
)

#let _MS-PER = (day: 86400000, hour: 3600000, minute: 60000, second: 1000)

/// The first tick of `unit` at or after `ms`.
///
/// Snapping matters as much as the step: a monthly tick must land on the 1st,
/// a yearly one on 1 January, or the axis reads as arbitrary again.
#let _first-tick(ms, unit) = {
  let m = dh.from-ms(ms)
  if m == none { return none }
  if unit.kind == "year" {
    // Align to a multiple of the step so 5-year ticks land on 2020, 2025.
    let y = calc.ceil(m.year / unit.n) * unit.n
    return dh.from-parts(y, 1, 1)
  }
  if unit.kind == "month" {
    let month-index = (m.year * 12 + m.month - 1)
    let snapped = calc.ceil(month-index / unit.n) * unit.n
    let start = dh.from-parts(calc.div-euclid(snapped, 12), calc.rem-euclid(snapped, 12) + 1, 1)
    if start == none { return none }
    // Already past the snapped boundary within that month?
    if start.ms < ms { return dh.add-months(start, unit.n) }
    return start
  }
  if unit.kind == "day" and unit.n == 7 {
    // Weeks start on Monday, so the axis reads as weeks rather than as
    // every-seventh-day-from-whenever.
    let midnight = dh.from-parts(m.year, m.month, m.day)
    let ahead = calc.rem-euclid(8 - midnight.weekday, 7)
    let start = dh.add-days(midnight, ahead)
    if start.ms < ms { return dh.add-days(start, 7) }
    return start
  }
  // Day, hour, minute, second: snap to a whole multiple of the unit.
  let size = _MS-PER.at(unit.kind) * unit.n
  let snapped = calc.ceil(ms / size) * size
  dh.from-ms(snapped)
}

/// The tick after `moment`, one `unit` on.
#let _next-tick(moment, unit) = {
  if unit.kind == "year" { return dh.add-months(moment, 12 * unit.n) }
  if unit.kind == "month" { return dh.add-months(moment, unit.n) }
  dh.add-ms(moment, _MS-PER.at(unit.kind) * unit.n)
}

/// Roughly how long one `unit` is, in milliseconds.
///
/// Approximate for years and months, which have no fixed length — good enough
/// to choose a unit, never used to place a tick.
#let _approx-ms(unit) = if unit.kind == "year" {
  365.2425 * 86400000 * unit.n
} else if unit.kind == "month" {
  30.436875 * 86400000 * unit.n
} else {
  _MS-PER.at(unit.kind) * unit.n
}

/// Roughly how many ticks `unit` yields over `span` milliseconds.
#let _count-for(span, unit) = span / _approx-ms(unit)

/// Every tick of `unit` in `[lo, hi]`, on that unit's own boundaries.
#let _ticks-of(lo, hi, unit, guard: 500) = {
  let out = ()
  let t = _first-tick(lo, unit)
  if t == none { return out }
  let n = 0
  while t != none and t.ms <= hi and n < guard {
    out.push(t.ms)
    t = _next-tick(t, unit)
    n += 1
  }
  out
}

/// The unit to draw unlabelled ticks at, one step under `major`.
///
/// A time axis labelled only by year says nothing about where the quarters
/// fall; unlabelled ticks give the reader that resolution without adding
/// clutter. The finest unit that divides `major` into at most `_MAX-SUBDIVISIONS`
/// parts, so a year subdivides into quarters rather than into twelve months,
/// and a half-year into months.
#let _MAX-SUBDIVISIONS = 7

#let _minor-for(major, min-gap) = {
  let major-ms = _approx-ms(major)
  let chosen = none
  for unit in _UNITS {
    // Never subdivide past the data. A monthly series gains nothing from weekly
    // ticks: they mark positions no observation can occupy, and they do not even
    // line up with the months they sit between.
    if min-gap != none and _approx-ms(unit) < min-gap * 0.9 { continue }
    let ratio = major-ms / _approx-ms(unit)
    // `> 1.5` skips units equal to or barely finer than the major.
    if ratio > 1.5 and ratio <= _MAX-SUBDIVISIONS + 0.5 { chosen = unit }
  }
  chosen
}

/// Calendar ticks across `[lo, hi]`, aiming for about `target` labelled ones.
///
/// `min-gap` is the smallest spacing between two observations, in
/// milliseconds. It caps how fine the unlabelled ticks may go, so a monthly
/// series does not get weekly ticks marking positions its data cannot take.
///
/// Returns `(ticks: (..ms), level: n)` — `level` being flint's own format
/// level, so the caller can label them with core's `level_to_format` and get a
/// granularity that matches the tick unit. Returns `none` when the range is
/// degenerate or outside datehog's representable span.
#let calendar-ticks(lo, hi, target: 6, min-gap: none) = {
  if lo == none or hi == none or hi <= lo { return none }
  let span = hi - lo

  // The units run coarsest to finest, so their counts increase. Take the
  // *finest* unit that still fits within `target` — the densest axis that does
  // not overflow.
  //
  // Not "the coarsest unit with at least `target` ticks": consecutive units
  // jump by 7x (day to week) or 30x (day to month), so the first one to clear
  // the threshold routinely clears it by an order of magnitude. That is how a
  // six-month series ends up with 152 daily ticks.
  let chosen = none
  for unit in _UNITS {
    if _count-for(span, unit) <= target { chosen = unit } else { break }
  }
  // Every unit overflows (a very short span): take the coarsest, which is the
  // fewest ticks available.
  if chosen == none { chosen = _UNITS.first() }

  // The guard stops a pathological range from producing millions of ticks.
  let ticks = _ticks-of(lo, hi, chosen)
  if ticks.len() == 0 { return none }

  // Unlabelled ticks one unit down, minus any that a labelled tick already
  // sits on — lilaq draws both, so a shared position would be drawn twice.
  let minor = _minor-for(chosen, min-gap)
  let subticks = if minor == none { () } else {
    let major-set = ticks.dedup()
    _ticks-of(lo, hi, minor, guard: 2000).filter(ms => ms not in major-set)
  }

  (ticks: ticks, subticks: subticks, level: chosen.level)
}
