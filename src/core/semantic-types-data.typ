// GENERATED from flint/core/semantic_types.py by tests/gen_tables.py -- do not edit.
//
// Regenerate after any upstream change:  python tests/gen_tables.py
// Check for drift in CI:                 python tests/gen_tables.py --check
//
// Only pure-data tables are generated. Functions in this module are
// hand-ported and live alongside this file.

/// Every semantic type name, as an identity map (upstream keeps the
/// constants in a dict so callers can write `SemanticTypes.Price`).
#let SemanticTypes = (
  DateTime: "DateTime",
  Date: "Date",
  Time: "Time",
  Timestamp: "Timestamp",
  Year: "Year",
  Quarter: "Quarter",
  Month: "Month",
  Week: "Week",
  Day: "Day",
  Hour: "Hour",
  YearMonth: "YearMonth",
  YearQuarter: "YearQuarter",
  YearWeek: "YearWeek",
  Decade: "Decade",
  Duration: "Duration",
  Quantity: "Quantity",
  Count: "Count",
  Amount: "Amount",
  Price: "Price",
  Percentage: "Percentage",
  Temperature: "Temperature",
  Profit: "Profit",
  PercentageChange: "PercentageChange",
  Sentiment: "Sentiment",
  Correlation: "Correlation",
  Rank: "Rank",
  ID: "ID",
  Score: "Score",
  Latitude: "Latitude",
  Longitude: "Longitude",
  Country: "Country",
  State: "State",
  City: "City",
  Region: "Region",
  Address: "Address",
  ZipCode: "ZipCode",
  Category: "Category",
  Name: "Name",
  Status: "Status",
  Boolean: "Boolean",
  Direction: "Direction",
  Range: "Range",
  Number: "Number",
  Unknown: "Unknown",
)

/// Canonical orderings for the ordinal types that have one. Tried in
/// order; the first sequence matching enough of the data wins.
#let _ORDINAL_SEQUENCES = (
  Month: (
    (
      labels: ("January", "February", "March", "April", "May", "June", "July", "August", "September", "October", "November", "December"),
      caseInsensitive: true,
    ),
    (
      labels: ("Jan", "Feb", "Mar", "Apr", "May", "Jun", "Jul", "Aug", "Sep", "Oct", "Nov", "Dec"),
      caseInsensitive: true,
    ),
    (
      labels: ("1", "2", "3", "4", "5", "6", "7", "8", "9", "10", "11", "12"),
      caseInsensitive: false,
    ),
  ),
  Day: (
    (
      labels: ("Monday", "Tuesday", "Wednesday", "Thursday", "Friday", "Saturday", "Sunday"),
      caseInsensitive: true,
    ),
    (
      labels: ("Mon", "Tue", "Wed", "Thu", "Fri", "Sat", "Sun"),
      caseInsensitive: true,
    ),
    (
      labels: ("Mo", "Tu", "We", "Th", "Fr", "Sa", "Su"),
      caseInsensitive: true,
    ),
    (
      labels: ("Sunday", "Monday", "Tuesday", "Wednesday", "Thursday", "Friday", "Saturday"),
      caseInsensitive: true,
    ),
    (
      labels: ("Sun", "Mon", "Tue", "Wed", "Thu", "Fri", "Sat"),
      caseInsensitive: true,
    ),
  ),
  Quarter: (
    (
      labels: ("Q1", "Q2", "Q3", "Q4"),
      caseInsensitive: true,
    ),
  ),
  Direction: (
    (
      labels: ("N", "NE", "E", "SE", "S", "SW", "W", "NW"),
      caseInsensitive: true,
    ),
    (
      labels: ("North", "Northeast", "East", "Southeast", "South", "Southwest", "West", "Northwest"),
      caseInsensitive: true,
    ),
    (
      labels: ("N", "E", "S", "W"),
      caseInsensitive: true,
    ),
    (
      labels: ("North", "East", "South", "West"),
      caseInsensitive: true,
    ),
  ),
)
