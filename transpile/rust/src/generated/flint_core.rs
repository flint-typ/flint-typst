#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_variables)]
#![allow(unreachable_patterns)]
#![allow(unused_assignments)]
#![allow(dead_code)]
#[doc = "// NOTE: Map Python module '__future__'(tracked in DEPYLER-0424)"] #[doc = "// NOTE: Map Python module 'copy'(tracked in DEPYLER-0424)"] use std::f64 as math;
    pub type VisCategory = String;
    pub type T0Family = String;
    pub type T1Category = String;
    pub type DomainShape = String;
    pub type AggRole = String;
    pub type DivergingClass = String;
    pub type FormatClass = String;
    pub type ZeroBaseline = String;
    pub type SortChoice = String;
    const STR_GEOGRAPHIC: &'static str = "geographic";
    const STR_CHANNELOVERRIDE: &'static str = "channelOverride";
    const STR_REDBLUE: &'static str = "redblue";
    const STR_LABEL: &'static str = "label";
    const STR_QUANTITATIVE: &'static str = "quantitative";
    const STR_UNKNOWN_1: &'static str = "unknown";
    const STR_RANK: &'static str = "Rank";
    const STR_X: &'static str = "x";
    const STR_SEQUENTIAL: &'static str = "sequential";
    const STR_BUDGET: &'static str = "budget";
    const STR_SEMANTICTYPE: &'static str = "semanticType";
    const STR_INHERENT: &'static str = "inherent";
    const STR_VLTYPE: &'static str = "vlType";
    const STR_ZERO: &'static str = "zero";
    const STR_ROW: &'static str = "row";
    const STR_DIVERGING: &'static str = "diverging";
    const STR_EMPTY: &'static str = "";
    const STR_YEAR: &'static str = "Year";
    const STR_TYPE: &'static str = "type";
    const STR_CLAMP: &'static str = "clamp";
    const STR_ZEROCLASS: &'static str = "zeroClass";
    const STR_ROWS: &'static str = "rows";
    const STR_SUM: &'static str = "sum";
    const STR_REASON: &'static str = "reason";
    const STR_TOOLTIPFORMAT: &'static str = "tooltipFormat";
    const STR_DECADE: &'static str = "Decade";
    const STR_UNKNOWN_2: &'static str = "Unknown";
    const STR_COLUMNS: &'static str = "columns";
    const STR_TEMPORAL: &'static str = "temporal";
    const STR_UNCERTAIN: &'static str = "uncertain";
    const STR_MIDPOINT: &'static str = "midpoint";
    const STR_VIRIDIS: &'static str = "viridis";
    const STR_SCHEMETYPE: &'static str = "schemeType";
    const STR_PATTERN: &'static str = "pattern";
    const STR_BLUES: &'static str = "blues";
    const STR_ORDINAL: &'static str = "ordinal";
    const STR_COLOR: &'static str = "color";
    const STR_GROUP: &'static str = "group";
    const STR_CARDINALITYGUARD: &'static str = "cardinalityGuard";
    const STR_SOURCE: &'static str = "source";
    const STR_FORCED: &'static str = "forced";
    const STR_NOMINAL: &'static str = "nominal";
    const STR_DOMAINPADFRACTION: &'static str = "domainPadFraction";
    const STR_MINSTEP: &'static str = "minStep";
    const STR_VISCATEGORY: &'static str = "visCategory";
    const STR_CATEGORICAL: &'static str = "categorical";
    const STR_COUNT: &'static str = "count";
    const STR_SCHEME: &'static str = "scheme";
    const STR_COLUMN: &'static str = "column";
    const STR_Y: &'static str = "y";
pub static _ISO_DATE_ONLY: std::sync::LazyLock<Vec<String>> = std::sync::LazyLock::new(|| Vec::new());
pub static _V8_NUMERIC_DATE: std::sync::LazyLock<Vec<String>> = std::sync::LazyLock::new(|| Vec::new());
pub static _WORD_RE: std::sync::LazyLock<Vec<String>> = std::sync::LazyLock::new(|| Vec::new());
pub static _YEAR_RE: std::sync::LazyLock<Vec<String>> = std::sync::LazyLock::new(|| Vec::new());
pub static TYPE_REGISTRY: std::sync::LazyLock<Vec<String>> = std::sync::LazyLock::new(|| Vec::new());
    map.insert("domainShape".to_string(), DepylerValue::Str("open".to_string()));
    map.insert(STR_DIVERGING.to_string(), DepylerValue::Str("none".to_string()));
    map.insert("formatClass".to_string(), DepylerValue::Str("plain".to_string()));
    map.insert("zeroBaseline".to_string(), DepylerValue::Str("none".to_string()));
    map.insert("zeroPad".to_string(), DepylerValue::Int(0 as i64));
    map
}
. into_iter().map(|(k, v) |(DepylerValue::Str(k), v)).collect()));
    map.insert(DepylerValue::Str("Status".to_string()), DepylerValue::Dict({ let mut map: HashMap<String, DepylerValue>= HashMap::new();
    map.insert("t0".to_string(), DepylerValue::Str("Categorical".to_string()));
    map.insert("t1".to_string(), DepylerValue::Str("Coded".to_string()));
    map.insert("visEncodings".to_string(), DepylerValue::List(vec! [DepylerValue::Str(STR_NOMINAL.to_string())]));
    map.insert("aggRole".to_string(), DepylerValue::Str("dimension".to_string()));
    map.insert("domainShape".to_string(), DepylerValue::Str("open".to_string()));
    map.insert(STR_DIVERGING.to_string(), DepylerValue::Str("none".to_string()));
    map.insert("formatClass".to_string(), DepylerValue::Str("plain".to_string()));
    map.insert("zeroBaseline".to_string(), DepylerValue::Str("none".to_string()));
    map.insert("zeroPad".to_string(), DepylerValue::Int(0 as i64));
    map
}
. into_iter().map(|(k, v) |(DepylerValue::Str(k), v)).collect()));
    map.insert(DepylerValue::Str("Boolean".to_string()), DepylerValue::Dict({ let mut map: HashMap<String, DepylerValue>= HashMap::new();
    map.insert("t0".to_string(), DepylerValue::Str("Categorical".to_string()));
    map.insert("t1".to_string(), DepylerValue::Str("Coded".to_string()));
    map.insert("visEncodings".to_string(), DepylerValue::List(vec! [DepylerValue::Str(STR_NOMINAL.to_string())]));
    map.insert("aggRole".to_string(), DepylerValue::Str("dimension".to_string()));
    map.insert("domainShape".to_string(), DepylerValue::Str("fixed".to_string()));
    map.insert(STR_DIVERGING.to_string(), DepylerValue::Str("none".to_string()));
    map.insert("formatClass".to_string(), DepylerValue::Str("plain".to_string()));
    map.insert("zeroBaseline".to_string(), DepylerValue::Str("none".to_string()));
    map.insert("zeroPad".to_string(), DepylerValue::Int(0 as i64));
    map
}
. into_iter().map(|(k, v) |(DepylerValue::Str(k), v)).collect()));
    map.insert(DepylerValue::Str("Direction".to_string()), DepylerValue::Dict({ let mut map: HashMap<String, DepylerValue>= HashMap::new();
    map.insert("t0".to_string(), DepylerValue::Str("Categorical".to_string()));
    map.insert("t1".to_string(), DepylerValue::Str("Coded".to_string()));
    map.insert("visEncodings".to_string(), DepylerValue::List(vec! [DepylerValue::Str(STR_ORDINAL.to_string()), DepylerValue::Str(STR_NOMINAL.to_string())]));
    map.insert("aggRole".to_string(), DepylerValue::Str("dimension".to_string()));
    map.insert("domainShape".to_string(), DepylerValue::Str("cyclic".to_string()));
    map.insert(STR_DIVERGING.to_string(), DepylerValue::Str("none".to_string()));
    map.insert("formatClass".to_string(), DepylerValue::Str("plain".to_string()));
    map.insert("zeroBaseline".to_string(), DepylerValue::Str("none".to_string()));
    map.insert("zeroPad".to_string(), DepylerValue::Int(0 as i64));
    map
}
. into_iter().map(|(k, v) |(DepylerValue::Str(k), v)).collect()));
    map.insert(DepylerValue::Str("Range".to_string()), DepylerValue::Dict({ let mut map: HashMap<String, DepylerValue>= HashMap::new();
    map.insert("t0".to_string(), DepylerValue::Str("Categorical".to_string()));
    map.insert("t1".to_string(), DepylerValue::Str("Binned".to_string()));
    map.insert("visEncodings".to_string(), DepylerValue::List(vec! [DepylerValue::Str(STR_ORDINAL.to_string())]));
    map.insert("aggRole".to_string(), DepylerValue::Str("dimension".to_string()));
    map.insert("domainShape".to_string(), DepylerValue::Str("open".to_string()));
    map.insert(STR_DIVERGING.to_string(), DepylerValue::Str("none".to_string()));
    map.insert("formatClass".to_string(), DepylerValue::Str("plain".to_string()));
    map.insert("zeroBaseline".to_string(), DepylerValue::Str("none".to_string()));
    map.insert("zeroPad".to_string(), DepylerValue::Int(0 as i64));
    map
}
. into_iter().map(|(k, v) |(DepylerValue::Str(k), v)).collect()));
    map.insert(DepylerValue::Str(STR_UNKNOWN_2.to_string()), DepylerValue::Dict({ let mut map: HashMap<String, DepylerValue>= HashMap::new();
    map.insert("t0".to_string(), DepylerValue::Str("Categorical".to_string()));
    map.insert("t1".to_string(), DepylerValue::Str("Entity".to_string()));
    map.insert("visEncodings".to_string(), DepylerValue::List(vec! [DepylerValue::Str(STR_NOMINAL.to_string())]));
    map.insert("aggRole".to_string(), DepylerValue::Str("dimension".to_string()));
    map.insert("domainShape".to_string(), DepylerValue::Str("open".to_string()));
    map.insert(STR_DIVERGING.to_string(), DepylerValue::Str("none".to_string()));
    map.insert("formatClass".to_string(), DepylerValue::Str("plain".to_string()));
    map.insert("zeroBaseline".to_string(), DepylerValue::Str("none".to_string()));
    map.insert("zeroPad".to_string(), DepylerValue::Int(0 as i64));
    map
}
. into_iter().map(|(k, v) |(DepylerValue::Str(k), v)).collect()));
    map });
pub static UNKNOWN_ENTRY: std::sync::LazyLock<Vec<String>> = std::sync::LazyLock::new(|| Vec::new());
pub type SemanticTypes = String;
    pub static measureTypes: std::sync::LazyLock<std::collections::HashSet<String>>= std::sync::LazyLock::new(|| get_registered_types().into_iter().filter(| t | {
    let t = t.clone();
   (["additive", "intensive", "signed-additive"].contains(&& get_registry_entry(t.to_string()).get("aggRole").cloned().unwrap_or_default().into())) &&(get_registry_entry(t.to_string()).get("t1").cloned().unwrap_or_default().into() != "Score") }).map(| t | t).collect::<std::collections::HashSet<_>>());
    pub static nonMeasureNumericTypes: std::sync::LazyLock<std::collections::HashSet<String>>= std::sync::LazyLock::new(|| {
    let mut set = std::collections::HashSet::new();
    set.insert("Rank".to_string());
    set.insert("ID".to_string());
    set.insert("Score".to_string());
    set.insert("Year".to_string());
    set.insert("Month".to_string());
    set.insert("Day".to_string());
    set.insert("Hour".to_string());
    set.insert("Latitude".to_string());
    set.insert("Longitude".to_string());
    set });
    pub static categoricalTypes: std::sync::LazyLock<std::collections::HashSet<String>>= std::sync::LazyLock::new(|| get_registered_types().into_iter().filter(| t | {
    let t = t.clone();
   ((get_registry_entry(t.to_string()).get("visEncodings").cloned().unwrap_or_default().into().contains("nominal")) &&(get_registry_entry(t.to_string()).get("aggRole").cloned().unwrap_or_default().into() != "identifier")) ||(get_registry_entry(t.to_string()).get("t1").cloned().unwrap_or_default().into() == "Binned") }).map(| t | t).collect::<std::collections::HashSet<_>>());
    pub static ordinalTypes: std::sync::LazyLock<std::collections::HashSet<String>>= std::sync::LazyLock::new(|| get_registered_types().into_iter().filter(| t | {
    let t = t.clone();
    get_registry_entry(t.to_string()).get("visEncodings").cloned().unwrap_or_default().into().contains("ordinal") }).map(| t | t).collect::<std::collections::HashSet<_>>());
pub static _DATE_LIKE_RE: std::sync::LazyLock<Vec<String>> = std::sync::LazyLock::new(|| Vec::new());
    pub const ZERO_BASELINE_GAP_THRESHOLD: f64 = 0.5;
pub static _MONTH_FULL: std::sync::LazyLock<Vec<String>> = std::sync::LazyLock::new(|| Vec::new());
pub static _MONTH_ABBR3: std::sync::LazyLock<Vec<String>> = std::sync::LazyLock::new(|| Vec::new());
pub static _MONTH_NUM: std::sync::LazyLock<Vec<String>> = std::sync::LazyLock::new(|| Vec::new());
pub static _DOW_FULL: std::sync::LazyLock<Vec<String>> = std::sync::LazyLock::new(|| Vec::new());
pub static _DOW_ABBR3: std::sync::LazyLock<Vec<String>> = std::sync::LazyLock::new(|| Vec::new());
pub static _DOW_ABBR2: std::sync::LazyLock<Vec<String>> = std::sync::LazyLock::new(|| Vec::new());
pub static _DOW_FULL_SUN: std::sync::LazyLock<Vec<String>> = std::sync::LazyLock::new(|| Vec::new());
pub static _DOW_ABBR3_SUN: std::sync::LazyLock<Vec<String>> = std::sync::LazyLock::new(|| Vec::new());
pub static _QUARTER_LABELS: std::sync::LazyLock<Vec<String>> = std::sync::LazyLock::new(|| Vec::new());
pub static _COMPASS_8: std::sync::LazyLock<Vec<String>> = std::sync::LazyLock::new(|| Vec::new());
pub static _COMPASS_8_FULL: std::sync::LazyLock<Vec<String>> = std::sync::LazyLock::new(|| Vec::new());
pub static _COMPASS_4: std::sync::LazyLock<Vec<String>> = std::sync::LazyLock::new(|| Vec::new());
pub static _COMPASS_4_FULL: std::sync::LazyLock<Vec<String>> = std::sync::LazyLock::new(|| Vec::new());
pub static _ORDINAL_SEQUENCES: std::sync::LazyLock<Vec<String>> = std::sync::LazyLock::new(|| Vec::new());
pub static DEFAULT_GAS_PRESSURE_PARAMS: std::sync::LazyLock<Vec<String>> = std::sync::LazyLock::new(|| Vec::new());
    pub const VL_SHORT_DISCRETE_CATEGORY_COUNT: i32 = 4;
    pub const VL_SHORT_DISCRETE_LABEL_MAX_LEN: i32 = 8;
    pub const APPROX_CHAR_WIDTH_RATIO: f64 = 0.62;
pub static DEFAULT_BASE_SIZE: std::sync::LazyLock<Vec<String>> = std::sync::LazyLock::new(|| Vec::new());
pub static CURRENCY_MAP: std::sync::LazyLock<Vec<String>> = std::sync::LazyLock::new(|| Vec::new());
pub static UNIT_SUFFIX_MAP: std::sync::LazyLock<Vec<String>> = std::sync::LazyLock::new(|| Vec::new());
    pub const MAX_TIMESTAMP_SEC: i32 = - 192522496;
    pub const MAX_TIMESTAMP_MS: i32 = 4102444800000i64;
pub static SEMANTIC_LEVEL: std::sync::LazyLock<Vec<String>> = std::sync::LazyLock::new(|| Vec::new());
    pub static channels: std::sync::LazyLock<Vec<String>>= std::sync::LazyLock::new(|| vec! [STR_X.to_string(), STR_Y.to_string(), "x2".to_string(), "y2".to_string(), "id".to_string(), STR_COLOR.to_string(), "opacity".to_string(), "size".to_string(), "shape".to_string(), "strokeDash".to_string(), STR_COLUMN.to_string(), STR_ROW.to_string(), "latitude".to_string(), "longitude".to_string(), "radius".to_string(), "detail".to_string(), STR_GROUP.to_string(), "open".to_string(), "high".to_string(), "low".to_string(), "close".to_string(), "angle".to_string(), "metric".to_string(), "value".to_string(), "goal".to_string()]);
    pub static channelGroups: std::sync::LazyLock<std::collections::HashMap<String, Vec<String>>>= std::sync::LazyLock::new(|| {
    let mut map = HashMap::new();
    map.insert(STR_EMPTY.to_string(), vec! [STR_X.to_string(), "x2".to_string(), STR_Y.to_string(), "y2".to_string(), "latitude".to_string(), "longitude".to_string(), "id".to_string(), "radius".to_string(), "detail".to_string()]);
    map.insert("legends".to_string(), vec! [STR_COLOR.to_string(), STR_GROUP.to_string(), "size".to_string(), "shape".to_string(), "text".to_string(), "opacity".to_string(), "strokeDash".to_string()]);
    map.insert("price".to_string(), vec! ["open".to_string(), "high".to_string(), "low".to_string(), "close".to_string()]);
    map.insert("facets".to_string(), vec! [STR_COLUMN.to_string(), STR_ROW.to_string()]);
    map.insert("kpi".to_string(), vec! ["metric".to_string(), "value".to_string(), "goal".to_string()]);
    map });
    use std::collections::HashMap;
    use std::collections::HashSet;
    use std::borrow::Cow;
    use std::sync::LazyLock;
    #[doc = r" Sum type for heterogeneous dictionary values(Python fidelity)"] #[doc = r" DEPYLER-1040b: Now implements Hash + Eq to support non-string dict keys"] #[derive(Debug, Clone, Default)] pub enum DepylerValue {
    Int(i64), Float(f64), Str(String), Bool(bool), #[default] None, List(Vec<DepylerValue>), Dict(std::collections::HashMap<DepylerValue, DepylerValue>), #[doc = r" DEPYLER-1050: Tuple variant for Python tuple support"] Tuple(Vec<DepylerValue>) ,
}
impl PartialEq for DepylerValue {
    fn eq(&self, other: & Self) -> bool {
    match(self, other) {
   (DepylerValue::Int(_dv_a), DepylerValue::Int(_dv_b)) =>_dv_a == _dv_b ,(DepylerValue::Float(_dv_a), DepylerValue::Float(_dv_b)) =>_dv_a.to_bits() == _dv_b.to_bits() ,(DepylerValue::Str(_dv_a), DepylerValue::Str(_dv_b)) =>_dv_a == _dv_b ,(DepylerValue::Bool(_dv_a), DepylerValue::Bool(_dv_b)) =>_dv_a == _dv_b ,(DepylerValue::None, DepylerValue::None) =>true ,(DepylerValue::List(_dv_a), DepylerValue::List(_dv_b)) =>_dv_a == _dv_b ,(DepylerValue::Dict(_dv_a), DepylerValue::Dict(_dv_b)) =>_dv_a == _dv_b ,(DepylerValue::Tuple(_dv_a), DepylerValue::Tuple(_dv_b)) =>_dv_a == _dv_b, _ =>false ,
}
}
}
impl Eq for DepylerValue {
   
}
impl std::hash::Hash for DepylerValue {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
    std::mem::discriminant(self).hash(state);
    match self {
    DepylerValue::Int(_dv_int) =>_dv_int.hash(state), DepylerValue::Float(_dv_float) =>_dv_float.to_bits().hash(state), DepylerValue::Str(_dv_str) =>_dv_str.hash(state), DepylerValue::Bool(_dv_bool) =>_dv_bool.hash(state), DepylerValue::None =>{
   
}
DepylerValue::List(_dv_list) =>_dv_list.hash(state), DepylerValue::Dict(_) =>{
    0u8.hash(state);
   
}
DepylerValue::Tuple(_dv_tuple) =>_dv_tuple.hash(state) ,
}
}
}
impl std::fmt::Display for DepylerValue {
    fn fmt(&self, _dv_fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
    DepylerValue::Int(_dv_int) =>write!(_dv_fmt, "{}", _dv_int), DepylerValue::Float(_dv_float) =>write!(_dv_fmt, "{}", _dv_float), DepylerValue::Str(_dv_str) =>write!(_dv_fmt, "{}", _dv_str), DepylerValue::Bool(_dv_bool) =>write!(_dv_fmt, "{}", _dv_bool), DepylerValue::None =>write!(_dv_fmt, "None"), DepylerValue::List(_dv_list) =>write!(_dv_fmt, "{:?}", _dv_list), DepylerValue::Dict(_dv_dict) =>write!(_dv_fmt, "{:?}", _dv_dict), DepylerValue::Tuple(_dv_tuple) =>write!(_dv_fmt, "{:?}", _dv_tuple) ,
}
}
}
impl DepylerValue {
    #[doc = r" Get length of string, list, or dict"] #[doc = r" DEPYLER-1060: Use _dv_ prefix to avoid shadowing user variables"] pub fn len(&self) -> usize {
    match self {
    DepylerValue::Str(_dv_str) =>_dv_str.len(), DepylerValue::List(_dv_list) =>_dv_list.len(), DepylerValue::Dict(_dv_dict) =>_dv_dict.len(), DepylerValue::Tuple(_dv_tuple) =>_dv_tuple.len(), _ =>0 ,
}
} #[doc = r" Check if empty"] pub fn is_empty(&self) -> bool {
    self.len() == 0
}
#[doc = r" DEPYLER-1404: Check if value is None variant"] pub fn is_none(&self) -> bool {
    matches !(self, DepylerValue::None)
}
#[doc = r" DEPYLER-1404: Check if value is not None variant"] pub fn is_some(&self) -> bool {
    ! self.is_none()
}
#[doc = r" DEPYLER-1404: Push a value into list"] pub fn push(&mut self, value: impl Into<DepylerValue>) {
    if let DepylerValue::List(_dv_list) = self {
    _dv_list.push(value.into());
   
}
} #[doc = r" DEPYLER-1404: Extend list with another iterable"] pub fn extend<T: Into<DepylerValue>>(&mut self, other: impl IntoIterator<Item = T>) {
    if let DepylerValue::List(_dv_list) = self {
    _dv_list.extend(other.into_iter().map(| x | x.into()));
   
}
} #[doc = r" Get chars iterator for string values"] pub fn chars(&self) -> std::str::Chars<'_>{
    match self {
    DepylerValue::Str(_dv_str) =>_dv_str.chars(), _ =>"".chars() ,
}
} #[doc = r" Insert into dict(mutates self if Dict variant)"] #[doc = r" DEPYLER-1040b: Now accepts DepylerValue keys for non-string dict keys"] pub fn insert(&mut self, key: impl Into<DepylerValue>, value: impl Into<DepylerValue>) {
    if let DepylerValue::Dict(_dv_dict) = self {
    _dv_dict.insert(key.into(), value.into());
   
}
} #[doc = r" Get value from dict by key"] #[doc = r" DEPYLER-1040b: Now accepts DepylerValue keys"] pub fn get(&self, key: & DepylerValue) -> Option<& DepylerValue>{
    if let DepylerValue::Dict(_dv_dict) = self {
    _dv_dict.get(key)
}
else {
    Option::None
}
} #[doc = r" Get value from dict by string key(convenience method)"] pub fn get_str(&self, key: & str) -> Option<& DepylerValue>{
    self.get(& DepylerValue::Str(key.to_string()))
}
#[doc = r" Check if dict contains key"] #[doc = r" DEPYLER-1040b: Now accepts DepylerValue keys"] pub fn contains_key(&self, key: & DepylerValue) -> bool {
    if let DepylerValue::Dict(_dv_dict) = self {
    _dv_dict.contains_key(key)
}
else {
    false
}
} #[doc = r" Check if dict contains string key(convenience method)"] pub fn contains_key_str(&self, key: & str) -> bool {
    self.contains_key(& DepylerValue::Str(key.to_string()))
}
#[doc = r" DEPYLER-1051: Get iterator over list values"] #[doc = r" Returns an empty iterator for non-list types"] pub fn iter(&self) -> std::slice::Iter<'_, DepylerValue>{
    match self {
    DepylerValue::List(_dv_list) =>_dv_list.iter(), _ =>[].iter() ,
}
} #[doc = r" DEPYLER-1051: Get mutable iterator over list values"] pub fn iter_mut(&mut self) -> std::slice::IterMut<'_, DepylerValue>{
    match self {
    DepylerValue::List(_dv_list) =>_dv_list.iter_mut(), _ =>[].iter_mut() ,
}
} #[doc = r" DEPYLER-1051: Get iterator over dict key-value pairs"] #[doc = r" DEPYLER-1040b: Now uses DepylerValue keys"] pub fn items(&self) -> std::collections::hash_map::Iter<'_, DepylerValue, DepylerValue>{
    static EMPTY_MAP: std::sync::LazyLock<std::collections::HashMap<DepylerValue, DepylerValue>>= std::sync::LazyLock::new(|| std::collections::HashMap::new());
    match self {
    DepylerValue::Dict(_dv_dict) =>_dv_dict.iter(), _ =>EMPTY_MAP.iter() ,
}
} #[doc = r" DEPYLER-1051: Get iterator over dict keys"] #[doc = r" DEPYLER-1040b: Now returns DepylerValue keys"] pub fn keys(&self) -> std::collections::hash_map::Keys<'_, DepylerValue, DepylerValue>{
    static EMPTY_MAP: std::sync::LazyLock<std::collections::HashMap<DepylerValue, DepylerValue>>= std::sync::LazyLock::new(|| std::collections::HashMap::new());
    match self {
    DepylerValue::Dict(_dv_dict) =>_dv_dict.keys(), _ =>EMPTY_MAP.keys() ,
}
} #[doc = r" DEPYLER-1051: Get iterator over dict values"] #[doc = r" DEPYLER-1040b: Now uses DepylerValue keys internally"] pub fn values(&self) -> std::collections::hash_map::Values<'_, DepylerValue, DepylerValue>{
    static EMPTY_MAP: std::sync::LazyLock<std::collections::HashMap<DepylerValue, DepylerValue>>= std::sync::LazyLock::new(|| std::collections::HashMap::new());
    match self {
    DepylerValue::Dict(_dv_dict) =>_dv_dict.values(), _ =>EMPTY_MAP.values() ,
}
} #[doc = r" Convert to String(renamed to avoid shadowing Display::to_string)"] #[doc = r" DEPYLER-1121: Renamed from to_string to as_string to fix clippy::inherent_to_string_shadow_display"] pub fn as_string(&self) -> String {
    match self {
    DepylerValue::Str(_dv_str) =>_dv_str.clone(), DepylerValue::Int(_dv_int) =>_dv_int.to_string(), DepylerValue::Float(_dv_float) =>_dv_float.to_string(), DepylerValue::Bool(_dv_bool) =>_dv_bool.to_string(), DepylerValue::None =>"None".to_string(), DepylerValue::List(_dv_list) =>format!("{:?}", _dv_list), DepylerValue::Dict(_dv_dict) =>format!("{:?}", _dv_dict), DepylerValue::Tuple(_dv_tuple) =>format!("{:?}", _dv_tuple) ,
}
} #[doc = r" DEPYLER-1215: Get as str reference(for string values only)"] pub fn as_str(&self) -> Option<& str>{
    match self {
    DepylerValue::Str(_dv_str) =>Some(_dv_str.as_str()), _ =>None ,
}
} #[doc = r" DEPYLER-1215: Get as i64(for integer values)"] pub fn as_i64(&self) -> Option<i64>{
    match self {
    DepylerValue::Int(_dv_int) =>Some(*_dv_int), _ =>None ,
}
} #[doc = r" DEPYLER-1215: Get as f64(for float values)"] pub fn as_f64(&self) -> Option<f64>{
    match self {
    DepylerValue::Float(_dv_float) =>Some(*_dv_float), DepylerValue::Int(_dv_int) =>Some(*_dv_int as f64), _ =>None ,
}
} #[doc = r" DEPYLER-1215: Get as bool(for boolean values)"] pub fn as_bool(&self) -> Option<bool>{
    match self {
    DepylerValue::Bool(_dv_bool) =>Some(*_dv_bool), _ =>None ,
}
} #[doc = r" Convert to i64"] pub fn to_i64(&self) -> i64 {
    match self {
    DepylerValue::Int(_dv_int) =>* _dv_int, DepylerValue::Float(_dv_float) =>* _dv_float as i64, DepylerValue::Bool(_dv_bool) =>if * _dv_bool {
    1
}
else {
    0
}
, DepylerValue::Str(_dv_str) =>_dv_str.parse().unwrap_or(0), _ =>0 ,
}
} #[doc = r" Convert to f64"] pub fn to_f64(&self) -> f64 {
    match self {
    DepylerValue::Float(_dv_float) =>* _dv_float, DepylerValue::Int(_dv_int) =>* _dv_int as f64, DepylerValue::Bool(_dv_bool) =>if * _dv_bool {
    1.0
}
else {
    0.0
}
, DepylerValue::Str(_dv_str) =>_dv_str.parse().unwrap_or(0.0), _ =>0.0 ,
}
} #[doc = r" Convert to bool"] pub fn to_bool(&self) -> bool {
    match self {
    DepylerValue::Bool(_dv_bool) =>* _dv_bool, DepylerValue::Int(_dv_int) =>* _dv_int != 0, DepylerValue::Float(_dv_float) =>* _dv_float != 0.0, DepylerValue::Str(_dv_str) =>! _dv_str.is_empty(), DepylerValue::List(_dv_list) =>! _dv_list.is_empty(), DepylerValue::Dict(_dv_dict) =>! _dv_dict.is_empty(), DepylerValue::Tuple(_dv_tuple) =>! _dv_tuple.is_empty(), DepylerValue::None =>false ,
}
} #[doc = r" DEPYLER-1064: Get tuple element by index for tuple unpacking"] #[doc = r" Returns the element at the given index, or panics with a readable error"] #[doc = r" Works on both Tuple and List variants(Python treats them similarly for unpacking)"] pub fn get_tuple_elem(&self, _dv_idx: usize) -> DepylerValue {
    match self {
    DepylerValue::Tuple(_dv_tuple) =>{
    if _dv_idx<_dv_tuple.len() {
    _dv_tuple [_dv_idx].clone()
}
else {
    panic!("Tuple index {} out of bounds(length {})", _dv_idx, _dv_tuple.len())
}
} DepylerValue::List(_dv_list) =>{
    if _dv_idx<_dv_list.len() {
    _dv_list [_dv_idx].clone()
}
else {
    panic!("List index {} out of bounds(length {})", _dv_idx, _dv_list.len())
}
} _dv_other =>panic!("Expected tuple or list for unpacking, found {:?}", _dv_other) ,
}
} #[doc = r" DEPYLER-1064: Extract tuple as Vec for multiple assignment"] #[doc = r" Validates that the value is a tuple/list with the expected number of elements"] pub fn extract_tuple(&self, _dv_expected_len: usize) -> Vec<DepylerValue>{
    match self {
    DepylerValue::Tuple(_dv_tuple) =>{
    if _dv_tuple.len() != _dv_expected_len {
    panic!("Expected tuple of length {}, got length {}", _dv_expected_len, _dv_tuple.len())
}
_dv_tuple.clone()
}
DepylerValue::List(_dv_list) =>{
    if _dv_list.len() != _dv_expected_len {
    panic!("Expected list of length {}, got length {}", _dv_expected_len, _dv_list.len())
}
_dv_list.clone()
}
_dv_other =>panic!("Expected tuple or list for unpacking, found {:?}", _dv_other) ,
}
} #[doc = r" DEPYLER-1137: Get tag name(XML element proxy)"] #[doc = r" Returns empty string for non-element types"] pub fn tag(&self) -> String {
    match self {
    DepylerValue::Str(_dv_s) =>_dv_s.clone(), _ =>String::new() ,
}
} #[doc = r" DEPYLER-1137: Get text content(XML element proxy)"] #[doc = r" Returns None for non-string types"] pub fn text(&self) -> Option<String>{
    match self {
    DepylerValue::Str(_dv_s) =>Some(_dv_s.clone()), DepylerValue::None =>Option::None, _ =>Option::None ,
}
} #[doc = r" DEPYLER-1137: Find child element by tag(XML element proxy)"] #[doc = r" Returns DepylerValue::None for non-matching/non-container types"] pub fn find(&self, _tag: & str) -> DepylerValue {
    match self {
    DepylerValue::List(_dv_list) =>{
    _dv_list.first().cloned().unwrap_or(DepylerValue::None)
}
DepylerValue::Dict(_dv_dict) =>{
    _dv_dict.get(& DepylerValue::Str(_tag.to_string())).cloned().unwrap_or(DepylerValue::None)
}
_ =>DepylerValue::None ,
}
} #[doc = r" DEPYLER-1137: Find all child elements by tag(XML element proxy)"] #[doc = r" Returns empty Vec for non-container types"] pub fn findall(&self, _tag: & str) -> Vec<DepylerValue>{
    match self {
    DepylerValue::List(_dv_list) =>_dv_list.clone(), _ =>Vec::new() ,
}
} #[doc = r" DEPYLER-1137: Set attribute(XML element proxy)"] #[doc = r" No-op for non-dict types"] pub fn set(&mut self, key: & str, value: & str) {
    if let DepylerValue::Dict(_dv_dict) = self {
    _dv_dict.insert(DepylerValue::Str(String::from(key)), DepylerValue::Str(String::from(value)));
   
}
}
}
impl std::ops::Index<usize>for DepylerValue {
    type Output = DepylerValue;
    fn index(&self, _dv_idx: usize) -> & Self::Output {
    match self {
    DepylerValue::List(_dv_list) =>& _dv_list [_dv_idx], DepylerValue::Tuple(_dv_tuple) =>& _dv_tuple [_dv_idx], _ =>panic!("Cannot index non-list/tuple DepylerValue") ,
}
}
}
impl std::ops::Index<& str>for DepylerValue {
    type Output = DepylerValue;
    fn index(&self, _dv_key: & str) -> & Self::Output {
    match self {
    DepylerValue::Dict(_dv_dict) =>_dv_dict.get(& DepylerValue::Str(_dv_key.to_string())).unwrap_or(& DepylerValue::None), _ =>panic!("Cannot index non-dict DepylerValue with string key") ,
}
}
}
impl std::ops::Index<DepylerValue>for DepylerValue {
    type Output = DepylerValue;
    fn index(&self, _dv_key: DepylerValue) -> & Self::Output {
    match self {
    DepylerValue::Dict(_dv_dict) =>_dv_dict.get(& _dv_key).unwrap_or(& DepylerValue::None), _ =>panic!("Cannot index non-dict DepylerValue") ,
}
}
}
impl std::ops::Index<i64>for DepylerValue {
    type Output = DepylerValue;
    fn index(&self, _dv_key: i64) -> & Self::Output {
    match self {
    DepylerValue::Dict(_dv_dict) =>_dv_dict.get(& DepylerValue::Int(_dv_key)).unwrap_or(& DepylerValue::None), DepylerValue::List(_dv_list) =>& _dv_list [_dv_key as usize], DepylerValue::Tuple(_dv_tuple) =>& _dv_tuple [_dv_key as usize], _ =>panic!("Cannot index DepylerValue with integer") ,
}
}
}
impl std::ops::Index<i32>for DepylerValue {
    type Output = DepylerValue;
    fn index(&self, _dv_key: i32) -> & Self::Output {
    &self [_dv_key as i64]
}
} impl From<i64>for DepylerValue {
    fn from(v: i64) -> Self {
    DepylerValue::Int(v)
}
} impl From<i32>for DepylerValue {
    fn from(v: i32) -> Self {
    DepylerValue::Int(v as i64)
}
} impl From<f64>for DepylerValue {
    fn from(v: f64) -> Self {
    DepylerValue::Float(v)
}
} impl From<String>for DepylerValue {
    fn from(v: String) -> Self {
    DepylerValue::Str(v)
}
} impl From<& str>for DepylerValue {
    fn from(v: & str) -> Self {
    DepylerValue::Str(String::from(v))
}
} impl From<bool>for DepylerValue {
    fn from(v: bool) -> Self {
    DepylerValue::Bool(v)
}
} impl From<Vec<DepylerValue>>for DepylerValue {
    fn from(v: Vec<DepylerValue>) -> Self {
    DepylerValue::List(v)
}
} impl From<Vec<String>>for DepylerValue {
    fn from(v: Vec<String>) -> Self {
    DepylerValue::List(v.into_iter().map(DepylerValue::Str).collect())
}
} impl From<Vec<i32>>for DepylerValue {
    fn from(v: Vec<i32>) -> Self {
    DepylerValue::List(v.into_iter().map(| x | DepylerValue::Int(x as i64)).collect())
}
} impl From<Vec<i64>>for DepylerValue {
    fn from(v: Vec<i64>) -> Self {
    DepylerValue::List(v.into_iter().map(DepylerValue::Int).collect())
}
} impl From<Vec<f64>>for DepylerValue {
    fn from(v: Vec<f64>) -> Self {
    DepylerValue::List(v.into_iter().map(DepylerValue::Float).collect())
}
} impl From<Vec<bool>>for DepylerValue {
    fn from(v: Vec<bool>) -> Self {
    DepylerValue::List(v.into_iter().map(DepylerValue::Bool).collect())
}
} impl From<Vec<& str>>for DepylerValue {
    fn from(v: Vec<& str>) -> Self {
    DepylerValue::List(v.into_iter().map(| s | DepylerValue::Str(s.to_string())).collect())
}
} impl From<std::collections::HashMap<DepylerValue, DepylerValue>>for DepylerValue {
    fn from(v: std::collections::HashMap<DepylerValue, DepylerValue>) -> Self {
    DepylerValue::Dict(v)
}
} impl From<std::collections::HashMap<String, DepylerValue>>for DepylerValue {
    fn from(v: std::collections::HashMap<String, DepylerValue>) -> Self {
    let converted: std::collections::HashMap<DepylerValue, DepylerValue>= v.into_iter().map(|(k, v) |(DepylerValue::Str(k), v)).collect();
    DepylerValue::Dict(converted)
}
} impl From<std::collections::HashSet<DepylerValue>>for DepylerValue {
    fn from(v: std::collections::HashSet<DepylerValue>) -> Self {
    DepylerValue::List(v.into_iter().collect())
}
} impl From<std::sync::Arc<std::collections::HashSet<DepylerValue>>>for DepylerValue {
    fn from(v: std::sync::Arc<std::collections::HashSet<DepylerValue>>) -> Self {
    DepylerValue::List(v.iter().cloned().collect())
}
} impl From<std::collections::HashSet<i32>>for DepylerValue {
    fn from(v: std::collections::HashSet<i32>) -> Self {
    DepylerValue::List(v.into_iter().map(| x | DepylerValue::Int(x as i64)).collect())
}
} impl From<std::collections::HashSet<i64>>for DepylerValue {
    fn from(v: std::collections::HashSet<i64>) -> Self {
    DepylerValue::List(v.into_iter().map(DepylerValue::Int).collect())
}
} impl From<std::collections::HashSet<String>>for DepylerValue {
    fn from(v: std::collections::HashSet<String>) -> Self {
    DepylerValue::List(v.into_iter().map(DepylerValue::Str).collect())
}
} impl From<std::sync::Arc<std::collections::HashSet<i32>>>for DepylerValue {
    fn from(v: std::sync::Arc<std::collections::HashSet<i32>>) -> Self {
    DepylerValue::List(v.iter().map(| x | DepylerValue::Int(*x as i64)).collect())
}
} impl From<std::sync::Arc<std::collections::HashSet<i64>>>for DepylerValue {
    fn from(v: std::sync::Arc<std::collections::HashSet<i64>>) -> Self {
    DepylerValue::List(v.iter().map(| x | DepylerValue::Int(*x)).collect())
}
} impl From<std::sync::Arc<std::collections::HashSet<String>>>for DepylerValue {
    fn from(v: std::sync::Arc<std::collections::HashSet<String>>) -> Self {
    DepylerValue::List(v.iter().map(| s | DepylerValue::Str(s.clone())).collect())
}
} impl From<DepylerValue>for i64 {
    fn from(v: DepylerValue) -> Self {
    v.to_i64()
}
} impl From<DepylerValue>for i32 {
    fn from(v: DepylerValue) -> Self {
    v.to_i64() as i32
}
} impl From<DepylerValue>for f64 {
    fn from(v: DepylerValue) -> Self {
    v.to_f64()
}
} impl From<DepylerValue>for f32 {
    fn from(v: DepylerValue) -> Self {
    v.to_f64() as f32
}
} impl From<DepylerValue>for String {
    fn from(v: DepylerValue) -> Self {
    v.as_string()
}
} impl From<DepylerValue>for bool {
    fn from(v: DepylerValue) -> Self {
    v.to_bool()
}
} impl std::ops::Add for DepylerValue {
    type Output = DepylerValue;
    fn add(self, rhs: Self) -> Self::Output {
    match(self, rhs) {
   (DepylerValue::Int(_dv_a), DepylerValue::Int(_dv_b)) =>DepylerValue::Int(_dv_a + _dv_b) ,(DepylerValue::Float(_dv_a), DepylerValue::Float(_dv_b)) =>DepylerValue::Float(_dv_a + _dv_b) ,(DepylerValue::Int(_dv_a), DepylerValue::Float(_dv_b)) =>DepylerValue::Float(_dv_a as f64 + _dv_b) ,(DepylerValue::Float(_dv_a), DepylerValue::Int(_dv_b)) =>DepylerValue::Float(_dv_a + _dv_b as f64) ,(DepylerValue::Str(_dv_a), DepylerValue::Str(_dv_b)) =>DepylerValue::Str(_dv_a + & _dv_b), _ =>DepylerValue::None ,
}
}
}
impl std::ops::Sub for DepylerValue {
    type Output = DepylerValue;
    fn sub(self, rhs: Self) -> Self::Output {
    match(self, rhs) {
   (DepylerValue::Int(_dv_a), DepylerValue::Int(_dv_b)) =>DepylerValue::Int(_dv_a - _dv_b) ,(DepylerValue::Float(_dv_a), DepylerValue::Float(_dv_b)) =>DepylerValue::Float(_dv_a - _dv_b) ,(DepylerValue::Int(_dv_a), DepylerValue::Float(_dv_b)) =>DepylerValue::Float(_dv_a as f64 - _dv_b) ,(DepylerValue::Float(_dv_a), DepylerValue::Int(_dv_b)) =>DepylerValue::Float(_dv_a - _dv_b as f64), _ =>DepylerValue::None ,
}
}
}
impl std::ops::Mul for DepylerValue {
    type Output = DepylerValue;
    fn mul(self, rhs: Self) -> Self::Output {
    match(self, rhs) {
   (DepylerValue::Int(_dv_a), DepylerValue::Int(_dv_b)) =>DepylerValue::Int(_dv_a * _dv_b) ,(DepylerValue::Float(_dv_a), DepylerValue::Float(_dv_b)) =>DepylerValue::Float(_dv_a * _dv_b) ,(DepylerValue::Int(_dv_a), DepylerValue::Float(_dv_b)) =>DepylerValue::Float(_dv_a as f64 * _dv_b) ,(DepylerValue::Float(_dv_a), DepylerValue::Int(_dv_b)) =>DepylerValue::Float(_dv_a * _dv_b as f64), _ =>DepylerValue::None ,
}
}
}
impl std::ops::Div for DepylerValue {
    type Output = DepylerValue;
    fn div(self, rhs: Self) -> Self::Output {
    match(self, rhs) {
   (DepylerValue::Int(_dv_a), DepylerValue::Int(_dv_b)) if _dv_b != 0 =>DepylerValue::Int(_dv_a / _dv_b) ,(DepylerValue::Float(_dv_a), DepylerValue::Float(_dv_b)) if _dv_b != 0.0 =>DepylerValue::Float(_dv_a / _dv_b) ,(DepylerValue::Int(_dv_a), DepylerValue::Float(_dv_b)) if _dv_b != 0.0 =>DepylerValue::Float(_dv_a as f64 / _dv_b) ,(DepylerValue::Float(_dv_a), DepylerValue::Int(_dv_b)) if _dv_b != 0 =>DepylerValue::Float(_dv_a / _dv_b as f64), _ =>DepylerValue::None ,
}
}
}
impl std::ops::Add<i64>for DepylerValue {
    type Output = DepylerValue;
    fn add(self, rhs: i64) -> Self::Output {
    match self {
    DepylerValue::Int(_dv_int) =>DepylerValue::Int(_dv_int + rhs), DepylerValue::Float(_dv_float) =>DepylerValue::Float(_dv_float + rhs as f64), _ =>DepylerValue::None ,
}
}
}
impl std::ops::Add<i32>for DepylerValue {
    type Output = DepylerValue;
    fn add(self, rhs: i32) -> Self::Output {
    self +(rhs as i64)
}
} impl std::ops::Add<DepylerValue>for i32 {
    type Output = i32;
    fn add(self, rhs: DepylerValue) -> Self::Output {
    self + rhs.to_i64() as i32
}
} impl std::ops::Add<DepylerValue>for i64 {
    type Output = i64;
    fn add(self, rhs: DepylerValue) -> Self::Output {
    self + rhs.to_i64()
}
} impl std::ops::Add<DepylerValue>for f64 {
    type Output = f64;
    fn add(self, rhs: DepylerValue) -> Self::Output {
    self + rhs.to_f64()
}
} impl std::ops::Sub<i64>for DepylerValue {
    type Output = DepylerValue;
    fn sub(self, rhs: i64) -> Self::Output {
    match self {
    DepylerValue::Int(_dv_int) =>DepylerValue::Int(_dv_int - rhs), DepylerValue::Float(_dv_float) =>DepylerValue::Float(_dv_float - rhs as f64), _ =>DepylerValue::None ,
}
}
}
impl std::ops::Sub<i32>for DepylerValue {
    type Output = DepylerValue;
    fn sub(self, rhs: i32) -> Self::Output {
    self -(rhs as i64)
}
} impl std::ops::Sub<f64>for DepylerValue {
    type Output = DepylerValue;
    fn sub(self, rhs: f64) -> Self::Output {
    match self {
    DepylerValue::Int(_dv_int) =>DepylerValue::Float(_dv_int as f64 - rhs), DepylerValue::Float(_dv_float) =>DepylerValue::Float(_dv_float - rhs), _ =>DepylerValue::None ,
}
}
}
impl std::ops::Sub<DepylerValue>for i32 {
    type Output = i32;
    fn sub(self, rhs: DepylerValue) -> Self::Output {
    self - rhs.to_i64() as i32
}
} impl std::ops::Sub<DepylerValue>for i64 {
    type Output = i64;
    fn sub(self, rhs: DepylerValue) -> Self::Output {
    self - rhs.to_i64()
}
} impl std::ops::Sub<DepylerValue>for f64 {
    type Output = f64;
    fn sub(self, rhs: DepylerValue) -> Self::Output {
    self - rhs.to_f64()
}
} impl std::ops::Mul<i64>for DepylerValue {
    type Output = DepylerValue;
    fn mul(self, rhs: i64) -> Self::Output {
    match self {
    DepylerValue::Int(_dv_int) =>DepylerValue::Int(_dv_int * rhs), DepylerValue::Float(_dv_float) =>DepylerValue::Float(_dv_float * rhs as f64), _ =>DepylerValue::None ,
}
}
}
impl std::ops::Mul<i32>for DepylerValue {
    type Output = DepylerValue;
    fn mul(self, rhs: i32) -> Self::Output {
    self *(rhs as i64)
}
} impl std::ops::Mul<f64>for DepylerValue {
    type Output = DepylerValue;
    fn mul(self, rhs: f64) -> Self::Output {
    match self {
    DepylerValue::Int(_dv_int) =>DepylerValue::Float(_dv_int as f64 * rhs), DepylerValue::Float(_dv_float) =>DepylerValue::Float(_dv_float * rhs), _ =>DepylerValue::None ,
}
}
}
impl std::ops::Mul<DepylerValue>for i32 {
    type Output = i32;
    fn mul(self, rhs: DepylerValue) -> Self::Output {
    self * rhs.to_i64() as i32
}
} impl std::ops::Mul<DepylerValue>for i64 {
    type Output = i64;
    fn mul(self, rhs: DepylerValue) -> Self::Output {
    self * rhs.to_i64()
}
} impl std::ops::Mul<DepylerValue>for f64 {
    type Output = f64;
    fn mul(self, rhs: DepylerValue) -> Self::Output {
    self * rhs.to_f64()
}
} impl std::ops::Div<i64>for DepylerValue {
    type Output = DepylerValue;
    fn div(self, rhs: i64) -> Self::Output {
    if rhs == 0 {
    return DepylerValue::None;
   
}
match self {
    DepylerValue::Int(_dv_int) =>DepylerValue::Int(_dv_int / rhs), DepylerValue::Float(_dv_float) =>DepylerValue::Float(_dv_float / rhs as f64), _ =>DepylerValue::None ,
}
}
}
impl std::ops::Div<i32>for DepylerValue {
    type Output = DepylerValue;
    fn div(self, rhs: i32) -> Self::Output {
    self /(rhs as i64)
}
} impl std::ops::Div<f64>for DepylerValue {
    type Output = DepylerValue;
    fn div(self, rhs: f64) -> Self::Output {
    if rhs == 0.0 {
    return DepylerValue::None;
   
}
match self {
    DepylerValue::Int(_dv_int) =>DepylerValue::Float(_dv_int as f64 / rhs), DepylerValue::Float(_dv_float) =>DepylerValue::Float(_dv_float / rhs), _ =>DepylerValue::None ,
}
}
}
impl std::ops::Div<DepylerValue>for i32 {
    type Output = i32;
    fn div(self, rhs: DepylerValue) -> Self::Output {
    let divisor = rhs.to_i64() as i32;
    if divisor == 0 {
    0
}
else {
    self / divisor
}
}
}
impl std::ops::Div<DepylerValue>for i64 {
    type Output = i64;
    fn div(self, rhs: DepylerValue) -> Self::Output {
    let divisor = rhs.to_i64();
    if divisor == 0 {
    0
}
else {
    self / divisor
}
}
}
impl std::ops::Div<DepylerValue>for f64 {
    type Output = f64;
    fn div(self, rhs: DepylerValue) -> Self::Output {
    let divisor = rhs.to_f64();
    if divisor == 0.0 {
    0.0
}
else {
    self / divisor
}
}
}
impl std::ops::Add<f64>for DepylerValue {
    type Output = DepylerValue;
    fn add(self, rhs: f64) -> Self::Output {
    match self {
    DepylerValue::Int(_dv_int) =>DepylerValue::Float(_dv_int as f64 + rhs), DepylerValue::Float(_dv_float) =>DepylerValue::Float(_dv_float + rhs), _ =>DepylerValue::None ,
}
}
}
impl std::ops::Neg for DepylerValue {
    type Output = DepylerValue;
    fn neg(self) -> Self::Output {
    match self {
    DepylerValue::Int(_dv_int) =>DepylerValue::Int(- _dv_int), DepylerValue::Float(_dv_float) =>DepylerValue::Float(- _dv_float), _ =>DepylerValue::None ,
}
}
}
impl std::ops::Not for DepylerValue {
    type Output = bool;
    fn not(self) -> Self::Output {
    ! self.to_bool()
}
} impl std::ops::BitXor<i64>for DepylerValue {
    type Output = DepylerValue;
    fn bitxor(self, rhs: i64) -> Self::Output {
    match self {
    DepylerValue::Int(_dv_int) =>DepylerValue::Int(_dv_int ^ rhs), _ =>DepylerValue::None ,
}
}
}
impl std::ops::BitAnd<i64>for DepylerValue {
    type Output = DepylerValue;
    fn bitand(self, rhs: i64) -> Self::Output {
    match self {
    DepylerValue::Int(_dv_int) =>DepylerValue::Int(_dv_int & rhs), _ =>DepylerValue::None ,
}
}
}
impl std::ops::BitOr<i64>for DepylerValue {
    type Output = DepylerValue;
    fn bitor(self, rhs: i64) -> Self::Output {
    match self {
    DepylerValue::Int(_dv_int) =>DepylerValue::Int(_dv_int | rhs), _ =>DepylerValue::None ,
}
}
}
impl IntoIterator for DepylerValue {
    type Item = DepylerValue;
    type IntoIter = std::vec::IntoIter<DepylerValue>;
    fn into_iter(self) -> Self::IntoIter {
    match self {
    DepylerValue::List(_dv_list) =>_dv_list.into_iter(), DepylerValue::Tuple(_dv_tuple) =>_dv_tuple.into_iter(), DepylerValue::Dict(_dv_dict) =>_dv_dict.into_keys().collect::<Vec<_>>().into_iter(), DepylerValue::Str(_dv_str) =>{
    _dv_str.chars().map(| _dv_c | DepylerValue::Str(_dv_c.to_string())).collect::<Vec<_>>().into_iter()
}
_ =>Vec::new().into_iter() ,
}
}
}
impl<'_dv_a>IntoIterator for & '_dv_a DepylerValue {
    type Item = DepylerValue;
    type IntoIter = std::vec::IntoIter<DepylerValue>;
    fn into_iter(self) -> Self::IntoIter {
    match self {
    DepylerValue::List(_dv_list) =>_dv_list.iter().cloned().collect::<Vec<_>>().into_iter(), DepylerValue::Tuple(_dv_tuple) =>_dv_tuple.iter().cloned().collect::<Vec<_>>().into_iter(), DepylerValue::Dict(_dv_dict) =>_dv_dict.keys().cloned().collect::<Vec<_>>().into_iter(), DepylerValue::Str(_dv_str) =>{
    _dv_str.chars().map(| _dv_c | DepylerValue::Str(_dv_c.to_string())).collect::<Vec<_>>().into_iter()
}
_ =>Vec::new().into_iter() ,
}
}
}
impl std::cmp::PartialOrd for DepylerValue {
    fn partial_cmp(&self, other: & Self) -> Option<std::cmp::Ordering>{
    match(self, other) {
   (DepylerValue::Int(_dv_a), DepylerValue::Int(_dv_b)) =>Some(_dv_a.cmp(_dv_b)) ,(DepylerValue::Float(_dv_a), DepylerValue::Float(_dv_b)) =>Some(_dv_a.total_cmp(_dv_b)) ,(DepylerValue::Str(_dv_a), DepylerValue::Str(_dv_b)) =>Some(_dv_a.cmp(_dv_b)) ,(DepylerValue::Bool(_dv_a), DepylerValue::Bool(_dv_b)) =>Some(_dv_a.cmp(_dv_b)) ,(DepylerValue::Int(_dv_a), DepylerValue::Float(_dv_b)) =>Some((*_dv_a as f64).total_cmp(_dv_b)) ,(DepylerValue::Float(_dv_a), DepylerValue::Int(_dv_b)) =>Some(_dv_a.total_cmp(&(*_dv_b as f64))) ,(DepylerValue::None, DepylerValue::None) =>Some(std::cmp::Ordering::Equal) ,(DepylerValue::None, _) =>Some(std::cmp::Ordering::Less) ,(_, DepylerValue::None) =>Some(std::cmp::Ordering::Greater) ,(DepylerValue::List(_dv_a), DepylerValue::List(_dv_b)) =>_dv_a.partial_cmp(_dv_b) ,(DepylerValue::Tuple(_dv_a), DepylerValue::Tuple(_dv_b)) =>_dv_a.partial_cmp(_dv_b), _ =>Option::None ,
}
}
}
impl std::cmp::Ord for DepylerValue {
    fn cmp(&self, other: & Self) -> std::cmp::Ordering {
    self.partial_cmp(other).unwrap_or(std::cmp::Ordering::Equal)
}
} impl std::cmp::PartialOrd<i32>for DepylerValue {
    fn partial_cmp(&self, other: & i32) -> Option<std::cmp::Ordering>{
    self.partial_cmp(& DepylerValue::Int(*other as i64))
}
} impl std::cmp::PartialOrd<i64>for DepylerValue {
    fn partial_cmp(&self, other: & i64) -> Option<std::cmp::Ordering>{
    self.partial_cmp(& DepylerValue::Int(*other))
}
} impl std::cmp::PartialOrd<f64>for DepylerValue {
    fn partial_cmp(&self, other: & f64) -> Option<std::cmp::Ordering>{
    self.partial_cmp(& DepylerValue::Float(*other))
}
} impl std::cmp::PartialOrd<DepylerValue>for i32 {
    fn partial_cmp(&self, other: & DepylerValue) -> Option<std::cmp::Ordering>{
    DepylerValue::Int(*self as i64).partial_cmp(other)
}
} impl std::cmp::PartialOrd<DepylerValue>for i64 {
    fn partial_cmp(&self, other: & DepylerValue) -> Option<std::cmp::Ordering>{
    DepylerValue::Int(*self).partial_cmp(other)
}
} impl std::cmp::PartialOrd<DepylerValue>for f64 {
    fn partial_cmp(&self, other: & DepylerValue) -> Option<std::cmp::Ordering>{
    DepylerValue::Float(*self).partial_cmp(other)
}
} impl std::cmp::PartialEq<i32>for DepylerValue {
    fn eq(&self, other: & i32) -> bool {
    self == & DepylerValue::Int(*other as i64)
}
} impl std::cmp::PartialEq<i64>for DepylerValue {
    fn eq(&self, other: & i64) -> bool {
    self == & DepylerValue::Int(*other)
}
} impl std::cmp::PartialEq<f64>for DepylerValue {
    fn eq(&self, other: & f64) -> bool {
    self == & DepylerValue::Float(*other)
}
} impl std::cmp::PartialEq<DepylerValue>for i32 {
    fn eq(&self, other: & DepylerValue) -> bool {
    & DepylerValue::Int(*self as i64) == other
}
} impl std::cmp::PartialEq<DepylerValue>for i64 {
    fn eq(&self, other: & DepylerValue) -> bool {
    & DepylerValue::Int(*self) == other
}
} impl std::cmp::PartialEq<DepylerValue>for f64 {
    fn eq(&self, other: & DepylerValue) -> bool {
    & DepylerValue::Float(*self) == other
}
} impl std::cmp::PartialEq<String>for DepylerValue {
    fn eq(&self, other: & String) -> bool {
    self == & DepylerValue::Str(other.clone())
}
} impl std::cmp::PartialEq<& str>for DepylerValue {
    fn eq(&self, other: && str) -> bool {
    self == & DepylerValue::Str((*other).to_string())
}
} impl std::cmp::PartialEq<bool>for DepylerValue {
    fn eq(&self, other: & bool) -> bool {
    self == & DepylerValue::Bool(*other)
}
} impl std::cmp::PartialEq<DepylerValue>for String {
    fn eq(&self, other: & DepylerValue) -> bool {
    & DepylerValue::Str(self.clone()) == other
}
} impl std::cmp::PartialEq<DepylerValue>for & str {
    fn eq(&self, other: & DepylerValue) -> bool {
    & DepylerValue::Str(self.to_string()) == other
}
} impl std::cmp::PartialEq<DepylerValue>for bool {
    fn eq(&self, other: & DepylerValue) -> bool {
    & DepylerValue::Bool(*self) == other
}
} impl std::cmp::PartialEq<Vec<DepylerValue>>for DepylerValue {
    fn eq(&self, other: & Vec<DepylerValue>) -> bool {
    if let DepylerValue::List(_dv_l) = self {
    _dv_l == other
}
else {
    false
}
}
}
impl std::cmp::PartialEq<Vec<i32>>for DepylerValue {
    fn eq(&self, other: & Vec<i32>) -> bool {
    if let DepylerValue::List(_dv_l) = self {
    _dv_l.len() == other.len() && _dv_l.iter().zip(other).all(|(a, b) | a == & DepylerValue::Int(*b as i64))
}
else {
    false
}
}
}
impl std::cmp::PartialEq<Vec<i64>>for DepylerValue {
    fn eq(&self, other: & Vec<i64>) -> bool {
    if let DepylerValue::List(_dv_l) = self {
    _dv_l.len() == other.len() && _dv_l.iter().zip(other).all(|(a, b) | a == & DepylerValue::Int(*b))
}
else {
    false
}
}
}
impl std::cmp::PartialEq<Vec<f64>>for DepylerValue {
    fn eq(&self, other: & Vec<f64>) -> bool {
    if let DepylerValue::List(_dv_l) = self {
    _dv_l.len() == other.len() && _dv_l.iter().zip(other).all(|(a, b) | a == & DepylerValue::Float(*b))
}
else {
    false
}
}
}
impl std::cmp::PartialEq<Vec<String>>for DepylerValue {
    fn eq(&self, other: & Vec<String>) -> bool {
    if let DepylerValue::List(_dv_l) = self {
    _dv_l.len() == other.len() && _dv_l.iter().zip(other).all(|(a, b) | a == & DepylerValue::Str(b.clone()))
}
else {
    false
}
}
}
impl std::cmp::PartialEq<Vec<Vec<String>>>for DepylerValue {
    fn eq(&self, other: & Vec<Vec<String>>) -> bool {
    if let DepylerValue::List(_dv_l) = self {
    _dv_l.len() == other.len() && _dv_l.iter().zip(other).all(|(a, inner) | {
    a == & DepylerValue::List(inner.iter().map(| s | DepylerValue::Str(s.clone())).collect()) })
}
else {
    false
}
}
}
impl<V: Into<DepylerValue>+ Clone>std::cmp::PartialEq<std::collections::HashMap<String, V>>for DepylerValue {
    fn eq(&self, other: & std::collections::HashMap<String, V>) -> bool {
    if let DepylerValue::Dict(_dv_dict) = self {
    if _dv_dict.len() != other.len() {
    return false;
   
}
other.iter().all(|(k, v) | {
    _dv_dict.get(& DepylerValue::Str(k.clone())).map_or(false, | dv | dv == & v.clone().into()) })
}
else {
    false
}
}
}
impl std::cmp::PartialEq <(i32, i32)>for DepylerValue {
    fn eq(&self, other: &(i32, i32)) -> bool {
    if let DepylerValue::Tuple(_dv_t) = self {
    _dv_t.len() == 2 && _dv_t [0] == DepylerValue::Int(other.0 as i64) && _dv_t [1] == DepylerValue::Int(other.1 as i64)
}
else {
    false
}
}
}
impl std::cmp::PartialEq <(i64, i64)>for DepylerValue {
    fn eq(&self, other: &(i64, i64)) -> bool {
    if let DepylerValue::Tuple(_dv_t) = self {
    _dv_t.len() == 2 && _dv_t [0] == DepylerValue::Int(other.0) && _dv_t [1] == DepylerValue::Int(other.1)
}
else {
    false
}
}
}
impl std::cmp::PartialEq <(f64, f64)>for DepylerValue {
    fn eq(&self, other: &(f64, f64)) -> bool {
    if let DepylerValue::Tuple(_dv_t) = self {
    _dv_t.len() == 2 && _dv_t [0] == DepylerValue::Float(other.0) && _dv_t [1] == DepylerValue::Float(other.1)
}
else {
    false
}
}
}
impl<const N: usize>std::cmp::PartialEq<& [u8;
    N]>for DepylerValue {
    fn eq(&self, other: && [u8;
    N]) -> bool {
    if let DepylerValue::List(_dv_l) = self {
    _dv_l.len() == N && _dv_l.iter().zip(other.iter()).all(|(a, b) | a == & DepylerValue::Int(*b as i64))
}
else if let DepylerValue::Str(_dv_s) = self {
    _dv_s.as_bytes() == other.as_slice()
}
else {
    false
}
}
}
pub fn depyler_min<T: std::cmp::PartialOrd>(a: T, b: T) -> T {
    if a.partial_cmp(& b).map_or(true, | c | c == std::cmp::Ordering::Less || c == std::cmp::Ordering::Equal) {
    a
}
else {
    b
}
} pub fn depyler_max<T: std::cmp::PartialOrd>(a: T, b: T) -> T {
    if a.partial_cmp(& b).map_or(true, | c | c == std::cmp::Ordering::Greater || c == std::cmp::Ordering::Equal) {
    a
}
else {
    b
}
} pub trait PyTruthy {
    #[doc = r#" Returns true if the value is "truthy" in Python semantics."#] fn is_true(&self) -> bool;
   
}
impl PyTruthy for bool {
    #[inline] fn is_true(&self) -> bool {
    * self
}
} impl PyTruthy for i32 {
    #[inline] fn is_true(&self) -> bool {
    * self != 0
}
} impl PyTruthy for i64 {
    #[inline] fn is_true(&self) -> bool {
    * self != 0
}
} impl PyTruthy for f32 {
    #[inline] fn is_true(&self) -> bool {
    * self != 0.0
}
} impl PyTruthy for f64 {
    #[inline] fn is_true(&self) -> bool {
    * self != 0.0
}
} impl PyTruthy for String {
    #[inline] fn is_true(&self) -> bool {
    ! self.is_empty()
}
} impl PyTruthy for & str {
    #[inline] fn is_true(&self) -> bool {
    ! self.is_empty()
}
} impl<T>PyTruthy for Vec<T>{
    #[inline] fn is_true(&self) -> bool {
    ! self.is_empty()
}
} impl<T>PyTruthy for Option<T>{
    #[inline] fn is_true(&self) -> bool {
    self.is_some()
}
} impl<K, V>PyTruthy for std::collections::HashMap<K, V>{
    #[inline] fn is_true(&self) -> bool {
    ! self.is_empty()
}
} impl<K, V>PyTruthy for std::collections::BTreeMap<K, V>{
    #[inline] fn is_true(&self) -> bool {
    ! self.is_empty()
}
} impl<T>PyTruthy for std::collections::HashSet<T>{
    #[inline] fn is_true(&self) -> bool {
    ! self.is_empty()
}
} impl<T>PyTruthy for std::collections::BTreeSet<T>{
    #[inline] fn is_true(&self) -> bool {
    ! self.is_empty()
}
} impl<T>PyTruthy for std::collections::VecDeque<T>{
    #[inline] fn is_true(&self) -> bool {
    ! self.is_empty()
}
} impl PyTruthy for DepylerValue {
    #[doc = r" Python truthiness for DepylerValue:"] #[doc = r#" - Int(0), Float(0.0), Str(""), Bool(false), None -> false"#] #[doc = r" - List([]), Dict({}), Tuple([]) -> false"] #[doc = r" - Everything else -> true"] #[inline] fn is_true(&self) -> bool {
    match self {
    DepylerValue::Bool(_dv_b) =>* _dv_b, DepylerValue::Int(_dv_i) =>* _dv_i != 0, DepylerValue::Float(_dv_f) =>* _dv_f != 0.0, DepylerValue::Str(_dv_s) =>! _dv_s.is_empty(), DepylerValue::List(_dv_l) =>! _dv_l.is_empty(), DepylerValue::Dict(_dv_d) =>! _dv_d.is_empty(), DepylerValue::Tuple(_dv_t) =>! _dv_t.is_empty(), DepylerValue::None =>false ,
}
}
}
pub trait PyAdd<Rhs = Self>{
    type Output;
    fn py_add(self, rhs: Rhs) -> Self::Output;
   
}
pub trait PySub<Rhs = Self>{
    type Output;
    fn py_sub(self, rhs: Rhs) -> Self::Output;
   
}
pub trait PyMul<Rhs = Self>{
    type Output;
    fn py_mul(self, rhs: Rhs) -> Self::Output;
   
}
pub trait PyDiv<Rhs = Self>{
    type Output;
    fn py_div(self, rhs: Rhs) -> Self::Output;
   
}
pub trait PyMod<Rhs = Self>{
    type Output;
    fn py_mod(self, rhs: Rhs) -> Self::Output;
   
}
pub trait PyIndex<Idx>{
    type Output;
    fn py_index(&self, index: Idx) -> Self::Output;
   
}
impl PyAdd for i32 {
    type Output = i32;
    #[inline] fn py_add(self, rhs: i32) -> i32 {
    self + rhs
}
} impl PyAdd<i64>for i32 {
    type Output = i64;
    #[inline] fn py_add(self, rhs: i64) -> i64 {
    self as i64 + rhs
}
} impl PyAdd<f64>for i32 {
    type Output = f64;
    #[inline] fn py_add(self, rhs: f64) -> f64 {
    self as f64 + rhs
}
} impl PyAdd for i64 {
    type Output = i64;
    #[inline] fn py_add(self, rhs: i64) -> i64 {
    self + rhs
}
} impl PyAdd<i32>for i64 {
    type Output = i64;
    #[inline] fn py_add(self, rhs: i32) -> i64 {
    self + rhs as i64
}
} impl PyAdd<f64>for i64 {
    type Output = f64;
    #[inline] fn py_add(self, rhs: f64) -> f64 {
    self as f64 + rhs
}
} impl PyAdd for f64 {
    type Output = f64;
    #[inline] fn py_add(self, rhs: f64) -> f64 {
    self + rhs
}
} impl PyAdd<i32>for f64 {
    type Output = f64;
    #[inline] fn py_add(self, rhs: i32) -> f64 {
    self + rhs as f64
}
} impl PyAdd<i64>for f64 {
    type Output = f64;
    #[inline] fn py_add(self, rhs: i64) -> f64 {
    self + rhs as f64
}
} impl PyAdd for String {
    type Output = String;
    #[inline] fn py_add(self, rhs: String) -> String {
    self + & rhs
}
} impl PyAdd<& str>for String {
    type Output = String;
    #[inline] fn py_add(self, rhs: & str) -> String {
    self + rhs
}
} impl PyAdd<& str>for & str {
    type Output = String;
    #[inline] fn py_add(self, rhs: & str) -> String {
    format!("{}{}", self, rhs)
}
} impl PyAdd<String>for & str {
    type Output = String;
    #[inline] fn py_add(self, rhs: String) -> String {
    format!("{}{}", self, rhs)
}
} impl PyAdd<char>for String {
    type Output = String;
    #[inline] fn py_add(mut self, rhs: char) -> String {
    self.push(rhs);
    self
}
} impl PyAdd<char>for & str {
    type Output = String;
    #[inline] fn py_add(self, rhs: char) -> String {
    format!("{}{}", self, rhs)
}
} impl PyAdd for DepylerValue {
    type Output = DepylerValue;
    fn py_add(self, rhs: DepylerValue) -> DepylerValue {
    match(self, rhs) {
   (DepylerValue::Int(_dv_a), DepylerValue::Int(_dv_b)) =>DepylerValue::Int(_dv_a + _dv_b) ,(DepylerValue::Float(_dv_a), DepylerValue::Float(_dv_b)) =>DepylerValue::Float(_dv_a + _dv_b) ,(DepylerValue::Int(_dv_a), DepylerValue::Float(_dv_b)) =>DepylerValue::Float(_dv_a as f64 + _dv_b) ,(DepylerValue::Float(_dv_a), DepylerValue::Int(_dv_b)) =>DepylerValue::Float(_dv_a + _dv_b as f64) ,(DepylerValue::Str(_dv_a), DepylerValue::Str(_dv_b)) =>DepylerValue::Str(_dv_a + & _dv_b), _ =>DepylerValue::None ,
}
}
}
impl PyAdd<DepylerValue>for i32 {
    type Output = i64;
    #[inline] fn py_add(self, rhs: DepylerValue) -> i64 {
    self as i64 + rhs.to_i64()
}
} impl PyAdd<DepylerValue>for i64 {
    type Output = i64;
    #[inline] fn py_add(self, rhs: DepylerValue) -> i64 {
    self + rhs.to_i64()
}
} impl PyAdd<DepylerValue>for f64 {
    type Output = f64;
    #[inline] fn py_add(self, rhs: DepylerValue) -> f64 {
    self + rhs.to_f64()
}
} impl PySub for i32 {
    type Output = i32;
    #[inline] fn py_sub(self, rhs: i32) -> i32 {
    self - rhs
}
} impl PySub<f64>for i32 {
    type Output = f64;
    #[inline] fn py_sub(self, rhs: f64) -> f64 {
    self as f64 - rhs
}
} impl PySub for i64 {
    type Output = i64;
    #[inline] fn py_sub(self, rhs: i64) -> i64 {
    self - rhs
}
} impl PySub<f64>for i64 {
    type Output = f64;
    #[inline] fn py_sub(self, rhs: f64) -> f64 {
    self as f64 - rhs
}
} impl PySub for f64 {
    type Output = f64;
    #[inline] fn py_sub(self, rhs: f64) -> f64 {
    self - rhs
}
} impl PySub<i32>for f64 {
    type Output = f64;
    #[inline] fn py_sub(self, rhs: i32) -> f64 {
    self - rhs as f64
}
} impl PySub<i64>for f64 {
    type Output = f64;
    #[inline] fn py_sub(self, rhs: i64) -> f64 {
    self - rhs as f64
}
} impl PySub for DepylerValue {
    type Output = DepylerValue;
    fn py_sub(self, rhs: DepylerValue) -> DepylerValue {
    match(self, rhs) {
   (DepylerValue::Int(_dv_a), DepylerValue::Int(_dv_b)) =>DepylerValue::Int(_dv_a - _dv_b) ,(DepylerValue::Float(_dv_a), DepylerValue::Float(_dv_b)) =>DepylerValue::Float(_dv_a - _dv_b) ,(DepylerValue::Int(_dv_a), DepylerValue::Float(_dv_b)) =>DepylerValue::Float(_dv_a as f64 - _dv_b) ,(DepylerValue::Float(_dv_a), DepylerValue::Int(_dv_b)) =>DepylerValue::Float(_dv_a - _dv_b as f64), _ =>DepylerValue::None ,
}
}
}
impl PySub<DepylerValue>for i32 {
    type Output = i64;
    #[inline] fn py_sub(self, rhs: DepylerValue) -> i64 {
    self as i64 - rhs.to_i64()
}
} impl PySub<DepylerValue>for i64 {
    type Output = i64;
    #[inline] fn py_sub(self, rhs: DepylerValue) -> i64 {
    self - rhs.to_i64()
}
} impl PySub<DepylerValue>for f64 {
    type Output = f64;
    #[inline] fn py_sub(self, rhs: DepylerValue) -> f64 {
    self - rhs.to_f64()
}
} impl<T: Eq + std::hash::Hash + Clone>PySub for std::collections::HashSet<T>{
    type Output = std::collections::HashSet<T>;
    fn py_sub(self, rhs: std::collections::HashSet<T>) -> Self::Output {
    self.difference(& rhs).cloned().collect()
}
} impl<T: Eq + std::hash::Hash + Clone>PySub<& std::collections::HashSet<T>>for std::collections::HashSet<T>{
    type Output = std::collections::HashSet<T>;
    fn py_sub(self, rhs: & std::collections::HashSet<T>) -> Self::Output {
    self.difference(rhs).cloned().collect()
}
} impl PyMul for i32 {
    type Output = i32;
    #[inline] fn py_mul(self, rhs: i32) -> i32 {
    self * rhs
}
} impl PyMul<f64>for i32 {
    type Output = f64;
    #[inline] fn py_mul(self, rhs: f64) -> f64 {
    self as f64 * rhs
}
} impl PyMul<i64>for i32 {
    type Output = i64;
    #[inline] fn py_mul(self, rhs: i64) -> i64 {
    self as i64 * rhs
}
} impl PyMul for i64 {
    type Output = i64;
    #[inline] fn py_mul(self, rhs: i64) -> i64 {
    self * rhs
}
} impl PyMul<f64>for i64 {
    type Output = f64;
    #[inline] fn py_mul(self, rhs: f64) -> f64 {
    self as f64 * rhs
}
} impl PyMul<i32>for i64 {
    type Output = i64;
    #[inline] fn py_mul(self, rhs: i32) -> i64 {
    self * rhs as i64
}
} impl PyMul for f64 {
    type Output = f64;
    #[inline] fn py_mul(self, rhs: f64) -> f64 {
    self * rhs
}
} impl PyMul<i32>for f64 {
    type Output = f64;
    #[inline] fn py_mul(self, rhs: i32) -> f64 {
    self * rhs as f64
}
} impl PyMul<i64>for f64 {
    type Output = f64;
    #[inline] fn py_mul(self, rhs: i64) -> f64 {
    self * rhs as f64
}
} impl PyMul<i32>for String {
    type Output = String;
    fn py_mul(self, rhs: i32) -> String {
    if rhs <= 0 {
    String::new()
}
else {
    self.repeat(rhs as usize)
}
}
}
impl PyMul<i64>for String {
    type Output = String;
    fn py_mul(self, rhs: i64) -> String {
    if rhs <= 0 {
    String::new()
}
else {
    self.repeat(rhs as usize)
}
}
}
impl PyMul<i32>for & str {
    type Output = String;
    fn py_mul(self, rhs: i32) -> String {
    if rhs <= 0 {
    String::new()
}
else {
    self.repeat(rhs as usize)
}
}
}
impl PyMul<i64>for & str {
    type Output = String;
    fn py_mul(self, rhs: i64) -> String {
    if rhs <= 0 {
    String::new()
}
else {
    self.repeat(rhs as usize)
}
}
}
impl PyMul for DepylerValue {
    type Output = DepylerValue;
    fn py_mul(self, rhs: DepylerValue) -> DepylerValue {
    match(self, rhs) {
   (DepylerValue::Int(_dv_a), DepylerValue::Int(_dv_b)) =>DepylerValue::Int(_dv_a * _dv_b) ,(DepylerValue::Float(_dv_a), DepylerValue::Float(_dv_b)) =>DepylerValue::Float(_dv_a * _dv_b) ,(DepylerValue::Int(_dv_a), DepylerValue::Float(_dv_b)) =>DepylerValue::Float(_dv_a as f64 * _dv_b) ,(DepylerValue::Float(_dv_a), DepylerValue::Int(_dv_b)) =>DepylerValue::Float(_dv_a * _dv_b as f64) ,(DepylerValue::Str(_dv_s), DepylerValue::Int(_dv_n)) =>{
    if _dv_n <= 0 {
    DepylerValue::Str(String::new())
}
else {
    DepylerValue::Str(_dv_s.repeat(_dv_n as usize))
}
} _ =>DepylerValue::None ,
}
}
}
impl PyMul<DepylerValue>for i32 {
    type Output = i64;
    #[inline] fn py_mul(self, rhs: DepylerValue) -> i64 {
    self as i64 * rhs.to_i64()
}
} impl PyMul<DepylerValue>for i64 {
    type Output = i64;
    #[inline] fn py_mul(self, rhs: DepylerValue) -> i64 {
    self * rhs.to_i64()
}
} impl PyMul<DepylerValue>for f64 {
    type Output = f64;
    #[inline] fn py_mul(self, rhs: DepylerValue) -> f64 {
    self * rhs.to_f64()
}
} impl<T: Clone>PyAdd<Vec<T>>for Vec<T>{
    type Output = Vec<T>;
    fn py_add(mut self, rhs: Vec<T>) -> Vec<T>{
    self.extend(rhs);
    self
}
} impl<T: Clone>PyAdd<& Vec<T>>for Vec<T>{
    type Output = Vec<T>;
    fn py_add(mut self, rhs: & Vec<T>) -> Vec<T>{
    self.extend(rhs.iter().cloned());
    self
}
} impl<T: Clone>PyAdd<Vec<T>>for & Vec<T>{
    type Output = Vec<T>;
    fn py_add(self, rhs: Vec<T>) -> Vec<T>{
    let mut result = self.clone();
    result.extend(rhs);
    result
}
} impl<T: Clone>PyMul<i32>for Vec<T>{
    type Output = Vec<T>;
    fn py_mul(self, rhs: i32) -> Vec<T>{
    if rhs <= 0 {
    Vec::new()
}
else {
    self.iter().cloned().cycle().take(self.len() * rhs as usize).collect()
}
}
}
impl<T: Clone>PyMul<i64>for Vec<T>{
    type Output = Vec<T>;
    fn py_mul(self, rhs: i64) -> Vec<T>{
    if rhs <= 0 {
    Vec::new()
}
else {
    self.iter().cloned().cycle().take(self.len() * rhs as usize).collect()
}
}
}
impl<T: Clone>PyMul<usize>for Vec<T>{
    type Output = Vec<T>;
    fn py_mul(self, rhs: usize) -> Vec<T>{
    self.iter().cloned().cycle().take(self.len() * rhs).collect()
}
} impl<T: Clone>PyMul<Vec<T>>for i32 {
    type Output = Vec<T>;
    fn py_mul(self, rhs: Vec<T>) -> Vec<T>{
    rhs.py_mul(self)
}
} impl<T: Clone>PyMul<Vec<T>>for i64 {
    type Output = Vec<T>;
    fn py_mul(self, rhs: Vec<T>) -> Vec<T>{
    rhs.py_mul(self)
}
} impl PySub<Vec<f64>>for Vec<f64>{
    type Output = Vec<f64>;
    fn py_sub(self, rhs: Vec<f64>) -> Vec<f64>{
    self.iter().zip(rhs.iter()).map(|(a, b) | a - b).collect()
}
} impl PySub<& Vec<f64>>for Vec<f64>{
    type Output = Vec<f64>;
    fn py_sub(self, rhs: & Vec<f64>) -> Vec<f64>{
    self.iter().zip(rhs.iter()).map(|(a, b) | a - b).collect()
}
} impl PySub<Vec<f64>>for & Vec<f64>{
    type Output = Vec<f64>;
    fn py_sub(self, rhs: Vec<f64>) -> Vec<f64>{
    self.iter().zip(rhs.iter()).map(|(a, b) | a - b).collect()
}
} impl PySub<& Vec<f64>>for & Vec<f64>{
    type Output = Vec<f64>;
    fn py_sub(self, rhs: & Vec<f64>) -> Vec<f64>{
    self.iter().zip(rhs.iter()).map(|(a, b) | a - b).collect()
}
} impl PySub<Vec<f32>>for Vec<f32>{
    type Output = Vec<f32>;
    fn py_sub(self, rhs: Vec<f32>) -> Vec<f32>{
    self.iter().zip(rhs.iter()).map(|(a, b) | a - b).collect()
}
} impl PySub<Vec<i64>>for Vec<i64>{
    type Output = Vec<i64>;
    fn py_sub(self, rhs: Vec<i64>) -> Vec<i64>{
    self.iter().zip(rhs.iter()).map(|(a, b) | a - b).collect()
}
} impl PySub<Vec<i32>>for Vec<i32>{
    type Output = Vec<i32>;
    fn py_sub(self, rhs: Vec<i32>) -> Vec<i32>{
    self.iter().zip(rhs.iter()).map(|(a, b) | a - b).collect()
}
} impl PyMul<Vec<f64>>for Vec<f64>{
    type Output = Vec<f64>;
    fn py_mul(self, rhs: Vec<f64>) -> Vec<f64>{
    self.iter().zip(rhs.iter()).map(|(a, b) | a * b).collect()
}
} impl PyMul<& Vec<f64>>for Vec<f64>{
    type Output = Vec<f64>;
    fn py_mul(self, rhs: & Vec<f64>) -> Vec<f64>{
    self.iter().zip(rhs.iter()).map(|(a, b) | a * b).collect()
}
} impl PyMul<Vec<f64>>for & Vec<f64>{
    type Output = Vec<f64>;
    fn py_mul(self, rhs: Vec<f64>) -> Vec<f64>{
    self.iter().zip(rhs.iter()).map(|(a, b) | a * b).collect()
}
} impl PyMul<& Vec<f64>>for & Vec<f64>{
    type Output = Vec<f64>;
    fn py_mul(self, rhs: & Vec<f64>) -> Vec<f64>{
    self.iter().zip(rhs.iter()).map(|(a, b) | a * b).collect()
}
} impl PyMul<Vec<f32>>for Vec<f32>{
    type Output = Vec<f32>;
    fn py_mul(self, rhs: Vec<f32>) -> Vec<f32>{
    self.iter().zip(rhs.iter()).map(|(a, b) | a * b).collect()
}
} impl PyMul<Vec<i64>>for Vec<i64>{
    type Output = Vec<i64>;
    fn py_mul(self, rhs: Vec<i64>) -> Vec<i64>{
    self.iter().zip(rhs.iter()).map(|(a, b) | a * b).collect()
}
} impl PyMul<Vec<i32>>for Vec<i32>{
    type Output = Vec<i32>;
    fn py_mul(self, rhs: Vec<i32>) -> Vec<i32>{
    self.iter().zip(rhs.iter()).map(|(a, b) | a * b).collect()
}
} impl PyDiv<Vec<f64>>for Vec<f64>{
    type Output = Vec<f64>;
    fn py_div(self, rhs: Vec<f64>) -> Vec<f64>{
    self.iter().zip(rhs.iter()).map(|(a, b) | if * b == 0.0 {
    f64::NAN
}
else {
    a / b }).collect()
}
} impl PyDiv<& Vec<f64>>for Vec<f64>{
    type Output = Vec<f64>;
    fn py_div(self, rhs: & Vec<f64>) -> Vec<f64>{
    self.iter().zip(rhs.iter()).map(|(a, b) | if * b == 0.0 {
    f64::NAN
}
else {
    a / b }).collect()
}
} impl PyDiv<Vec<f64>>for & Vec<f64>{
    type Output = Vec<f64>;
    fn py_div(self, rhs: Vec<f64>) -> Vec<f64>{
    self.iter().zip(rhs.iter()).map(|(a, b) | if * b == 0.0 {
    f64::NAN
}
else {
    a / b }).collect()
}
} impl PyDiv<& Vec<f64>>for & Vec<f64>{
    type Output = Vec<f64>;
    fn py_div(self, rhs: & Vec<f64>) -> Vec<f64>{
    self.iter().zip(rhs.iter()).map(|(a, b) | if * b == 0.0 {
    f64::NAN
}
else {
    a / b }).collect()
}
} impl PyDiv<Vec<f32>>for Vec<f32>{
    type Output = Vec<f32>;
    fn py_div(self, rhs: Vec<f32>) -> Vec<f32>{
    self.iter().zip(rhs.iter()).map(|(a, b) | if * b == 0.0 {
    f32::NAN
}
else {
    a / b }).collect()
}
} impl PyDiv<Vec<i64>>for Vec<i64>{
    type Output = Vec<f64>;
    fn py_div(self, rhs: Vec<i64>) -> Vec<f64>{
    self.iter().zip(rhs.iter()).map(|(a, b) | if * b == 0 {
    f64::NAN
}
else {
    * a as f64 / * b as f64 }).collect()
}
} impl PyDiv<Vec<i32>>for Vec<i32>{
    type Output = Vec<f64>;
    fn py_div(self, rhs: Vec<i32>) -> Vec<f64>{
    self.iter().zip(rhs.iter()).map(|(a, b) | if * b == 0 {
    f64::NAN
}
else {
    * a as f64 / * b as f64 }).collect()
}
} impl PyMul<f64>for Vec<f64>{
    type Output = Vec<f64>;
    fn py_mul(self, rhs: f64) -> Vec<f64>{
    self.iter().map(| a | a * rhs).collect()
}
} impl PyMul<Vec<f64>>for f64 {
    type Output = Vec<f64>;
    fn py_mul(self, rhs: Vec<f64>) -> Vec<f64>{
    rhs.iter().map(| a | a * self).collect()
}
} impl PyDiv<f64>for Vec<f64>{
    type Output = Vec<f64>;
    fn py_div(self, rhs: f64) -> Vec<f64>{
    if rhs == 0.0 {
    self.iter().map(| _ | f64::NAN).collect()
}
else {
    self.iter().map(| a | a / rhs).collect()
}
}
}
impl PySub<f64>for Vec<f64>{
    type Output = Vec<f64>;
    fn py_sub(self, rhs: f64) -> Vec<f64>{
    self.iter().map(| a | a - rhs).collect()
}
} impl PyAdd<f64>for Vec<f64>{
    type Output = Vec<f64>;
    fn py_add(self, rhs: f64) -> Vec<f64>{
    self.iter().map(| a | a + rhs).collect()
}
} impl PyDiv for i32 {
    type Output = f64;
    #[inline] fn py_div(self, rhs: i32) -> f64 {
    if rhs == 0 {
    f64::NAN
}
else {
    self as f64 / rhs as f64
}
}
}
impl PyDiv<f64>for i32 {
    type Output = f64;
    #[inline] fn py_div(self, rhs: f64) -> f64 {
    if rhs == 0.0 {
    f64::NAN
}
else {
    self as f64 / rhs
}
}
}
impl PyDiv for i64 {
    type Output = f64;
    #[inline] fn py_div(self, rhs: i64) -> f64 {
    if rhs == 0 {
    f64::NAN
}
else {
    self as f64 / rhs as f64
}
}
}
impl PyDiv<f64>for i64 {
    type Output = f64;
    #[inline] fn py_div(self, rhs: f64) -> f64 {
    if rhs == 0.0 {
    f64::NAN
}
else {
    self as f64 / rhs
}
}
}
impl PyDiv for f64 {
    type Output = f64;
    #[inline] fn py_div(self, rhs: f64) -> f64 {
    if rhs == 0.0 {
    f64::NAN
}
else {
    self / rhs
}
}
}
impl PyDiv<i32>for f64 {
    type Output = f64;
    #[inline] fn py_div(self, rhs: i32) -> f64 {
    if rhs == 0 {
    f64::NAN
}
else {
    self / rhs as f64
}
}
}
impl PyDiv<i64>for f64 {
    type Output = f64;
    #[inline] fn py_div(self, rhs: i64) -> f64 {
    if rhs == 0 {
    f64::NAN
}
else {
    self / rhs as f64
}
}
}
impl PyDiv for DepylerValue {
    type Output = DepylerValue;
    fn py_div(self, rhs: DepylerValue) -> DepylerValue {
    match(self, rhs) {
   (DepylerValue::Int(_dv_a), DepylerValue::Int(_dv_b)) if _dv_b != 0 =>DepylerValue::Float(_dv_a as f64 / _dv_b as f64) ,(DepylerValue::Float(_dv_a), DepylerValue::Float(_dv_b)) if _dv_b != 0.0 =>DepylerValue::Float(_dv_a / _dv_b) ,(DepylerValue::Int(_dv_a), DepylerValue::Float(_dv_b)) if _dv_b != 0.0 =>DepylerValue::Float(_dv_a as f64 / _dv_b) ,(DepylerValue::Float(_dv_a), DepylerValue::Int(_dv_b)) if _dv_b != 0 =>DepylerValue::Float(_dv_a / _dv_b as f64), _ =>DepylerValue::None ,
}
}
}
impl PyDiv<DepylerValue>for i32 {
    type Output = f64;
    #[inline] fn py_div(self, rhs: DepylerValue) -> f64 {
    let divisor = rhs.to_f64();
    if divisor == 0.0 {
    f64::NAN
}
else {
    self as f64 / divisor
}
}
}
impl PyDiv<DepylerValue>for i64 {
    type Output = f64;
    #[inline] fn py_div(self, rhs: DepylerValue) -> f64 {
    let divisor = rhs.to_f64();
    if divisor == 0.0 {
    f64::NAN
}
else {
    self as f64 / divisor
}
}
}
impl PyDiv<DepylerValue>for f64 {
    type Output = f64;
    #[inline] fn py_div(self, rhs: DepylerValue) -> f64 {
    let divisor = rhs.to_f64();
    if divisor == 0.0 {
    f64::NAN
}
else {
    self / divisor
}
}
}
impl PyMod for i32 {
    type Output = i32;
    #[inline] fn py_mod(self, rhs: i32) -> i32 {
    if rhs == 0 {
    0
}
else {
   ((self % rhs) + rhs) % rhs
}
}
}
impl PyMod<f64>for i32 {
    type Output = f64;
    #[inline] fn py_mod(self, rhs: f64) -> f64 {
    if rhs == 0.0 {
    f64::NAN
}
else {
   ((self as f64 % rhs) + rhs) % rhs
}
}
}
impl PyMod for i64 {
    type Output = i64;
    #[inline] fn py_mod(self, rhs: i64) -> i64 {
    if rhs == 0 {
    0
}
else {
   ((self % rhs) + rhs) % rhs
}
}
}
impl PyMod<f64>for i64 {
    type Output = f64;
    #[inline] fn py_mod(self, rhs: f64) -> f64 {
    if rhs == 0.0 {
    f64::NAN
}
else {
   ((self as f64 % rhs) + rhs) % rhs
}
}
}
impl PyMod for f64 {
    type Output = f64;
    #[inline] fn py_mod(self, rhs: f64) -> f64 {
    if rhs == 0.0 {
    f64::NAN
}
else {
   ((self % rhs) + rhs) % rhs
}
}
}
impl PyMod<i32>for f64 {
    type Output = f64;
    #[inline] fn py_mod(self, rhs: i32) -> f64 {
    if rhs == 0 {
    f64::NAN
}
else {
   ((self % rhs as f64) + rhs as f64) % rhs as f64
}
}
}
impl PyMod<i64>for f64 {
    type Output = f64;
    #[inline] fn py_mod(self, rhs: i64) -> f64 {
    if rhs == 0 {
    f64::NAN
}
else {
   ((self % rhs as f64) + rhs as f64) % rhs as f64
}
}
}
impl PyMod for DepylerValue {
    type Output = DepylerValue;
    fn py_mod(self, rhs: DepylerValue) -> DepylerValue {
    match(self, rhs) {
   (DepylerValue::Int(_dv_a), DepylerValue::Int(_dv_b)) if _dv_b != 0 =>{
    DepylerValue::Int(((_dv_a % _dv_b) + _dv_b) % _dv_b)
}
(DepylerValue::Float(_dv_a), DepylerValue::Float(_dv_b)) if _dv_b != 0.0 =>{
    DepylerValue::Float(((_dv_a % _dv_b) + _dv_b) % _dv_b)
}
(DepylerValue::Int(_dv_a), DepylerValue::Float(_dv_b)) if _dv_b != 0.0 =>{
    let a = _dv_a as f64;
    DepylerValue::Float(((a % _dv_b) + _dv_b) % _dv_b)
}
(DepylerValue::Float(_dv_a), DepylerValue::Int(_dv_b)) if _dv_b != 0 =>{
    let b = _dv_b as f64;
    DepylerValue::Float(((_dv_a % b) + b) % b)
}
_ =>DepylerValue::None ,
}
}
}
impl<T: Clone>PyIndex<i32>for Vec<T>{
    type Output = Option<T>;
    fn py_index(&self, index: i32) -> Option<T>{
    let _dv_len = self.len() as i32;
    let _dv_idx = if index<0 {
    _dv_len + index
}
else {
    index };
    if _dv_idx>= 0 &&(_dv_idx as usize)< self.len() {
    Some(self [_dv_idx as usize].clone())
}
else {
    Option::None
}
}
}
impl<T: Clone>PyIndex<i64>for Vec<T>{
    type Output = Option<T>;
    fn py_index(&self, index: i64) -> Option<T>{
    let _dv_len = self.len() as i64;
    let _dv_idx = if index<0 {
    _dv_len + index
}
else {
    index };
    if _dv_idx>= 0 &&(_dv_idx as usize)< self.len() {
    Some(self [_dv_idx as usize].clone())
}
else {
    Option::None
}
}
}
impl PyIndex<& str>for std::collections::HashMap<String, DepylerValue>{
    type Output = Option<DepylerValue>;
    fn py_index(&self, key: & str) -> Option<DepylerValue>{
    self.get(key).cloned()
}
} impl PyIndex<i32>for String {
    type Output = Option<char>;
    fn py_index(&self, index: i32) -> Option<char>{
    let _dv_len = self.len() as i32;
    let _dv_idx = if index<0 {
    _dv_len + index
}
else {
    index };
    if _dv_idx>= 0 {
    self.chars().nth(_dv_idx as usize)
}
else {
    Option::None
}
}
}
impl PyIndex<i64>for String {
    type Output = Option<char>;
    fn py_index(&self, index: i64) -> Option<char>{
    let _dv_len = self.len() as i64;
    let _dv_idx = if index<0 {
    _dv_len + index
}
else {
    index };
    if _dv_idx>= 0 {
    self.chars().nth(_dv_idx as usize)
}
else {
    Option::None
}
}
}
impl PyIndex<i32>for DepylerValue {
    type Output = DepylerValue;
    fn py_index(&self, index: i32) -> DepylerValue {
    match self {
    DepylerValue::List(_dv_list) =>{
    let _dv_len = _dv_list.len() as i32;
    let _dv_idx = if index<0 {
    _dv_len + index
}
else {
    index };
    if _dv_idx>= 0 &&(_dv_idx as usize)<_dv_list.len() {
    _dv_list [_dv_idx as usize].clone()
}
else {
    DepylerValue::None
}
} DepylerValue::Tuple(_dv_tuple) =>{
    let _dv_len = _dv_tuple.len() as i32;
    let _dv_idx = if index<0 {
    _dv_len + index
}
else {
    index };
    if _dv_idx>= 0 &&(_dv_idx as usize)<_dv_tuple.len() {
    _dv_tuple [_dv_idx as usize].clone()
}
else {
    DepylerValue::None
}
} DepylerValue::Str(_dv_str) =>{
    let _dv_len = _dv_str.len() as i32;
    let _dv_idx = if index<0 {
    _dv_len + index
}
else {
    index };
    if _dv_idx>= 0 {
    _dv_str.chars().nth(_dv_idx as usize).map(| _dv_c | DepylerValue::Str(_dv_c.to_string())).unwrap_or(DepylerValue::None)
}
else {
    DepylerValue::None
}
} _ =>DepylerValue::None ,
}
}
}
impl PyIndex<i64>for DepylerValue {
    type Output = DepylerValue;
    fn py_index(&self, index: i64) -> DepylerValue {
    self.py_index(index as i32)
}
} impl PyIndex<& str>for DepylerValue {
    type Output = DepylerValue;
    fn py_index(&self, key: & str) -> DepylerValue {
    match self {
    DepylerValue::Dict(_dv_dict) =>{
    _dv_dict.get(& DepylerValue::Str(key.to_string())).cloned().unwrap_or(DepylerValue::None)
}
_ =>DepylerValue::None ,
}
}
}
pub trait PyStringMethods {
    fn lower(&self) -> String;
    fn upper(&self) -> String;
    fn strip(&self) -> String;
    fn lstrip(&self) -> String;
    fn rstrip(&self) -> String;
    fn py_split(&self, sep: & str) -> Vec<String>;
    fn py_replace(&self, old: & str, new: & str) -> String;
    fn startswith(&self, prefix: & str) -> bool;
    fn endswith(&self, suffix: & str) -> bool;
    fn py_find(&self, sub: & str) -> i64;
    fn capitalize(&self) -> String;
    fn title(&self) -> String;
    fn swapcase(&self) -> String;
    fn isalpha(&self) -> bool;
    fn isdigit(&self) -> bool;
    fn isalnum(&self) -> bool;
    fn isspace(&self) -> bool;
    fn islower(&self) -> bool;
    fn isupper(&self) -> bool;
    fn center(&self, width: usize) -> String;
    fn ljust(&self, width: usize) -> String;
    fn rjust(&self, width: usize) -> String;
    fn zfill(&self, width: usize) -> String;
    fn count(&self, sub: & str) -> usize;
   
}
impl PyStringMethods for str {
    #[inline] fn lower(&self) -> String {
    self.to_lowercase()
}
#[inline] fn upper(&self) -> String {
    self.to_uppercase()
}
#[inline] fn strip(&self) -> String {
    self.trim().to_string()
}
#[inline] fn lstrip(&self) -> String {
    self.trim_start().to_string()
}
#[inline] fn rstrip(&self) -> String {
    self.trim_end().to_string()
}
#[inline] fn py_split(&self, sep: & str) -> Vec<String>{
    self.split(sep).map(| s | s.to_string()).collect()
}
#[inline] fn py_replace(&self, old: & str, new: & str) -> String {
    self.replace(old, new)
}
#[inline] fn startswith(&self, prefix: & str) -> bool {
    self.starts_with(prefix)
}
#[inline] fn endswith(&self, suffix: & str) -> bool {
    self.ends_with(suffix)
}
#[inline] fn py_find(&self, sub: & str) -> i64 {
    self.find(sub).map(| i | i as i64).unwrap_or(- 1)
}
#[inline] fn capitalize(&self) -> String {
    let mut chars = self.chars();
    match chars.next() {
    None =>String::new(), Some(c) =>c.to_uppercase().chain (chars.flat_map(| c | c.to_lowercase())).collect() ,
}
} #[inline] fn title(&self) -> String {
    let mut result = String::new();
    let mut capitalize_next = true;
    for c in self.chars() {
    if c.is_whitespace() {
    result.push(c);
    capitalize_next = true;
   
}
else if capitalize_next {
    result.extend(c.to_uppercase());
    capitalize_next = false;
   
}
else {
    result.extend(c.to_lowercase());
   
}
} result
}
#[inline] fn swapcase(&self) -> String {
    self.chars().map(| c | {
    if c.is_uppercase() {
    c.to_lowercase().collect::<String>()
}
else {
    c.to_uppercase().collect::<String>()
}
}).collect()
}
#[inline] fn isalpha(&self) -> bool {
    ! self.is_empty() &&self.chars().all(| c | c.is_alphabetic())
}
#[inline] fn isdigit(&self) -> bool {
    ! self.is_empty() &&self.chars().all(| c | c.is_ascii_digit())
}
#[inline] fn isalnum(&self) -> bool {
    ! self.is_empty() &&self.chars().all(| c | c.is_alphanumeric())
}
#[inline] fn isspace(&self) -> bool {
    ! self.is_empty() &&self.chars().all(| c | c.is_whitespace())
}
#[inline] fn islower(&self) -> bool {
    self.chars().any(| c | c.is_lowercase()) && ! self.chars().any(| c | c.is_uppercase())
}
#[inline] fn isupper(&self) -> bool {
    self.chars().any(| c | c.is_uppercase()) && ! self.chars().any(| c | c.is_lowercase())
}
#[inline] fn center(&self, width: usize) -> String {
    if self.len()>= width {
    return self.to_string();
   
}
let padding = width - self.len();
    let left = padding / 2;
    let right = padding - left;
    format!("{}{}{}", " ".repeat(left), self, " ".repeat(right))
}
#[inline] fn ljust(&self, width: usize) -> String {
    if self.len()>= width {
    return self.to_string();
   
}
format!("{}{}", self, " ".repeat(width - self.len()))
}
#[inline] fn rjust(&self, width: usize) -> String {
    if self.len()>= width {
    return self.to_string();
   
}
format!("{}{}", " ".repeat(width - self.len()), self)
}
#[inline] fn zfill(&self, width: usize) -> String {
    if self.len()>= width {
    return self.to_string();
   
}
format!("{}{}", "0".repeat(width - self.len()), self)
}
#[inline] fn count(&self, sub: & str) -> usize {
    self.matches(sub).count()
}
} impl PyStringMethods for String {
    #[inline] fn lower(&self) -> String {
    self.as_str().lower()
}
#[inline] fn upper(&self) -> String {
    self.as_str().upper()
}
#[inline] fn strip(&self) -> String {
    self.as_str().strip()
}
#[inline] fn lstrip(&self) -> String {
    self.as_str().lstrip()
}
#[inline] fn rstrip(&self) -> String {
    self.as_str().rstrip()
}
#[inline] fn py_split(&self, sep: & str) -> Vec<String>{
    self.as_str().py_split(sep)
}
#[inline] fn py_replace(&self, old: & str, new: & str) -> String {
    self.as_str().py_replace(old, new)
}
#[inline] fn startswith(&self, prefix: & str) -> bool {
    self.as_str().startswith(prefix)
}
#[inline] fn endswith(&self, suffix: & str) -> bool {
    self.as_str().endswith(suffix)
}
#[inline] fn py_find(&self, sub: & str) -> i64 {
    self.as_str().py_find(sub)
}
#[inline] fn capitalize(&self) -> String {
    self.as_str().capitalize()
}
#[inline] fn title(&self) -> String {
    self.as_str().title()
}
#[inline] fn swapcase(&self) -> String {
    self.as_str().swapcase()
}
#[inline] fn isalpha(&self) -> bool {
    self.as_str().isalpha()
}
#[inline] fn isdigit(&self) -> bool {
    self.as_str().isdigit()
}
#[inline] fn isalnum(&self) -> bool {
    self.as_str().isalnum()
}
#[inline] fn isspace(&self) -> bool {
    self.as_str().isspace()
}
#[inline] fn islower(&self) -> bool {
    self.as_str().islower()
}
#[inline] fn isupper(&self) -> bool {
    self.as_str().isupper()
}
#[inline] fn center(&self, width: usize) -> String {
    self.as_str().center(width)
}
#[inline] fn ljust(&self, width: usize) -> String {
    self.as_str().ljust(width)
}
#[inline] fn rjust(&self, width: usize) -> String {
    self.as_str().rjust(width)
}
#[inline] fn zfill(&self, width: usize) -> String {
    self.as_str().zfill(width)
}
#[inline] fn count(&self, sub: & str) -> usize {
    self.as_str().count(sub)
}
} impl PyStringMethods for DepylerValue {
    #[inline] fn lower(&self) -> String {
    match self {
    DepylerValue::Str(_dv_s) =>_dv_s.lower(), _ =>String::new()
}
} #[inline] fn upper(&self) -> String {
    match self {
    DepylerValue::Str(_dv_s) =>_dv_s.upper(), _ =>String::new()
}
} #[inline] fn strip(&self) -> String {
    match self {
    DepylerValue::Str(_dv_s) =>_dv_s.strip(), _ =>String::new()
}
} #[inline] fn lstrip(&self) -> String {
    match self {
    DepylerValue::Str(_dv_s) =>_dv_s.lstrip(), _ =>String::new()
}
} #[inline] fn rstrip(&self) -> String {
    match self {
    DepylerValue::Str(_dv_s) =>_dv_s.rstrip(), _ =>String::new()
}
} #[inline] fn py_split(&self, sep: & str) -> Vec<String>{
    match self {
    DepylerValue::Str(_dv_s) =>_dv_s.py_split(sep), _ =>Vec::new()
}
} #[inline] fn py_replace(&self, old: & str, new: & str) -> String {
    match self {
    DepylerValue::Str(_dv_s) =>_dv_s.py_replace(old, new), _ =>String::new()
}
} #[inline] fn capitalize(&self) -> String {
    match self {
    DepylerValue::Str(_dv_s) =>_dv_s.capitalize(), _ =>String::new()
}
} #[inline] fn title(&self) -> String {
    match self {
    DepylerValue::Str(_dv_s) =>_dv_s.title(), _ =>String::new()
}
} #[inline] fn swapcase(&self) -> String {
    match self {
    DepylerValue::Str(_dv_s) =>_dv_s.swapcase(), _ =>String::new()
}
} #[inline] fn startswith(&self, prefix: & str) -> bool {
    match self {
    DepylerValue::Str(_dv_s) =>_dv_s.startswith(prefix), _ =>false
}
} #[inline] fn endswith(&self, suffix: & str) -> bool {
    match self {
    DepylerValue::Str(_dv_s) =>_dv_s.endswith(suffix), _ =>false
}
} #[inline] fn py_find(&self, sub: & str) -> i64 {
    match self {
    DepylerValue::Str(_dv_s) =>_dv_s.py_find(sub), _ =>- 1
}
} #[inline] fn isalpha(&self) -> bool {
    match self {
    DepylerValue::Str(_dv_s) =>_dv_s.isalpha(), _ =>false
}
} #[inline] fn isdigit(&self) -> bool {
    match self {
    DepylerValue::Str(_dv_s) =>_dv_s.isdigit(), _ =>false
}
} #[inline] fn isalnum(&self) -> bool {
    match self {
    DepylerValue::Str(_dv_s) =>_dv_s.isalnum(), _ =>false
}
} #[inline] fn isspace(&self) -> bool {
    match self {
    DepylerValue::Str(_dv_s) =>_dv_s.isspace(), _ =>false
}
} #[inline] fn islower(&self) -> bool {
    match self {
    DepylerValue::Str(_dv_s) =>_dv_s.islower(), _ =>false
}
} #[inline] fn isupper(&self) -> bool {
    match self {
    DepylerValue::Str(_dv_s) =>_dv_s.isupper(), _ =>false
}
} #[inline] fn count(&self, sub: & str) -> usize {
    match self {
    DepylerValue::Str(_dv_s) =>_dv_s.count(sub), _ =>0
}
} #[inline] fn center(&self, width: usize) -> String {
    match self {
    DepylerValue::Str(_dv_s) =>_dv_s.center(width), _ =>String::new()
}
} #[inline] fn ljust(&self, width: usize) -> String {
    match self {
    DepylerValue::Str(_dv_s) =>_dv_s.ljust(width), _ =>String::new()
}
} #[inline] fn rjust(&self, width: usize) -> String {
    match self {
    DepylerValue::Str(_dv_s) =>_dv_s.rjust(width), _ =>String::new()
}
} #[inline] fn zfill(&self, width: usize) -> String {
    match self {
    DepylerValue::Str(_dv_s) =>_dv_s.zfill(width), _ =>String::new()
}
}
}
impl DepylerValue {
    #[doc = r" Check if string contains substring(Python's `in` operator for strings)"] #[inline] pub fn contains(&self, sub: & str) -> bool {
    match self {
    DepylerValue::Str(_dv_s) =>_dv_s.contains(sub), DepylerValue::List(_dv_l) =>_dv_l.iter().any(| v | {
    if let DepylerValue::Str(s) = v {
    s == sub
}
else {
    false
}
}), _ =>false ,
}
}
}
#[doc = r" DEPYLER-1202: Python integer operations for Rust integer types."] pub trait PythonIntOps {
    fn bit_length(&self) -> u32;
    fn bit_count(&self) -> u32;
   
}
impl PythonIntOps for i32 {
    fn bit_length(&self) -> u32 {
    if * self == 0 {
    0
}
else {
   (std::mem::size_of::<i32>() as u32 * 8) - self.unsigned_abs().leading_zeros()
}
} fn bit_count(&self) -> u32 {
    self.unsigned_abs().count_ones()
}
} impl PythonIntOps for i64 {
    fn bit_length(&self) -> u32 {
    if * self == 0 {
    0
}
else {
   (std::mem::size_of::<i64>() as u32 * 8) - self.unsigned_abs().leading_zeros()
}
} fn bit_count(&self) -> u32 {
    self.unsigned_abs().count_ones()
}
} impl PythonIntOps for u32 {
    fn bit_length(&self) -> u32 {
    if * self == 0 {
    0
}
else {
   (std::mem::size_of::<u32>() as u32 * 8) - self.leading_zeros()
}
} fn bit_count(&self) -> u32 {
    self.count_ones()
}
} impl PythonIntOps for u64 {
    fn bit_length(&self) -> u32 {
    if * self == 0 {
    0
}
else {
   (std::mem::size_of::<u64>() as u32 * 8) - self.leading_zeros()
}
} fn bit_count(&self) -> u32 {
    self.count_ones()
}
} impl PythonIntOps for usize {
    fn bit_length(&self) -> u32 {
    if * self == 0 {
    0
}
else {
   (std::mem::size_of::<usize>() as u32 * 8) - self.leading_zeros()
}
} fn bit_count(&self) -> u32 {
    self.count_ones()
}
} impl PythonIntOps for isize {
    fn bit_length(&self) -> u32 {
    if * self == 0 {
    0
}
else {
   (std::mem::size_of::<isize>() as u32 * 8) - self.unsigned_abs().leading_zeros()
}
} fn bit_count(&self) -> u32 {
    self.unsigned_abs().count_ones()
}
} #[doc = r" DEPYLER-1066: Wrapper for Python datetime.date"] #[doc = r" Provides .day(), .month(), .year() methods matching Python's API"] #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default, Hash)] pub struct DepylerDate(pub u32, pub u32, pub u32);
    impl DepylerDate {
    #[doc = r" Create a new date from year, month, day"] pub fn new(year: u32, month: u32, day: u32) -> Self {
    DepylerDate(year, month, day)
}
#[doc = r" Get today's date(NASA mode: computed from SystemTime)"] pub fn today() -> Self {
    use std::time::{
    SystemTime, UNIX_EPOCH };
    let secs = SystemTime::now().duration_since(UNIX_EPOCH).map(| d | d.as_secs()).unwrap_or(0);
    let days  = (secs / 86400) as i64;
    let z = days + 719468;
    let era = if z>= 0 {
    z
}
else {
    z - 146096
}
/ 146097;
    let doe  = (z - era * 146097) as u32;
    let yoe  = (doe - doe / 1460 + doe / 36524 - doe / 146096) / 365;
    let y = yoe as i64 + era * 400;
    let doy = doe -(365 * yoe + yoe / 4 - yoe / 100);
    let mp  = (5 * doy + 2) / 153;
    let d = doy -(153 * mp + 2) / 5 + 1;
    let mut m = if mp<10 {
    mp + 3
}
else {
    mp - 9 };
    let y = if m <= 2 {
    y + 1
}
else {
    y };
    DepylerDate(y as u32, m, d)
}
#[doc = r" Get the year component"] pub fn year(&self) -> u32 {
    self.0
}
#[doc = r" Get the month component(1-12)"] pub fn month(&self) -> u32 {
    self.1
}
#[doc = r" Get the day component(1-31)"] pub fn day(&self) -> u32 {
    self.2
}
#[doc = r" Convert to tuple(year, month, day) for interop"] pub fn to_tuple(&self) -> (u32, u32, u32) {
   (self.0, self.1, self.2)
}
#[doc = r" Get weekday(0 = Monday, 6 = Sunday) - Python datetime.date.weekday()"] pub fn weekday(&self) -> u32 {
    let(mut y, mut m, d)  = (self.0 as i32, self.1 as i32, self.2 as i32);
    if m<3 {
    m += 12;
    y -= 1;
   
}
let q = d;
    let k = y % 100;
    let j = y / 100;
    let h  = (q +(13 *(m + 1)) / 5 + k + k / 4 + j / 4 - 2 * j) % 7;
   ((h + 5) % 7) as u32
}
#[doc = r" Get ISO weekday(1 = Monday, 7 = Sunday) - Python datetime.date.isoweekday()"] pub fn isoweekday(&self) -> u32 {
    self.weekday() + 1
}
#[doc = r" Create date from ordinal(days since year 1, January 1 = ordinal 1)"] #[doc = r" Python: date.fromordinal(730120) -> date(2000, 1, 1)"] pub fn from_ordinal(ordinal: i64) -> Self {
    let mut days = ordinal - 719163 - 1;
    let z = days + 719468;
    let era = if z>= 0 {
    z
}
else {
    z - 146096
}
/ 146097;
    let doe  = (z - era * 146097) as u32;
    let yoe  = (doe - doe / 1460 + doe / 36524 - doe / 146096) / 365;
    let y = yoe as i64 + era * 400;
    let doy = doe -(365 * yoe + yoe / 4 - yoe / 100);
    let mp  = (5 * doy + 2) / 153;
    let d = doy -(153 * mp + 2) / 5 + 1;
    let mut m = if mp<10 {
    mp + 3
}
else {
    mp - 9 };
    let y = if m <= 2 {
    y + 1
}
else {
    y };
    DepylerDate(y as u32, m, d)
}
} impl std::fmt::Display for DepylerDate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{:04}-{:02}-{:02}", self.0, self.1, self.2)
}
} #[doc = r" DEPYLER-1067: Wrapper for Python datetime.datetime"] #[doc = r" Provides .year(), .month(), .day(), .hour(), .minute(), .second(), .microsecond() methods"] #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default, Hash)] pub struct DepylerDateTime {
    pub year: u32, pub month: u32, pub day: u32, pub hour: u32, pub minute: u32, pub second: u32, pub microsecond: u32 ,
}
impl DepylerDateTime {
    #[doc = r" Create a new datetime from components"] pub fn new(year: u32, month: u32, day: u32, hour: u32, minute: u32, second: u32, microsecond: u32) -> Self {
    DepylerDateTime {
    year, month, day, hour, minute, second, microsecond
}
} #[doc = r" Get current datetime(NASA mode: computed from SystemTime)"] pub fn now() -> Self {
    use std::time::{
    SystemTime, UNIX_EPOCH };
    let secs = SystemTime::now().duration_since(UNIX_EPOCH).map(| d | d.as_secs()).unwrap_or(0);
    let nanos = SystemTime::now().duration_since(UNIX_EPOCH).map(| d | d.subsec_nanos()).unwrap_or(0);
    let days  = (secs / 86400) as i64;
    let day_secs  = (secs % 86400) as u32;
    let z = days + 719468;
    let era = if z>= 0 {
    z
}
else {
    z - 146096
}
/ 146097;
    let doe  = (z - era * 146097) as u32;
    let yoe  = (doe - doe / 1460 + doe / 36524 - doe / 146096) / 365;
    let y = yoe as i64 + era * 400;
    let doy = doe -(365 * yoe + yoe / 4 - yoe / 100);
    let mp  = (5 * doy + 2) / 153;
    let d = doy -(153 * mp + 2) / 5 + 1;
    let mut m = if mp<10 {
    mp + 3
}
else {
    mp - 9 };
    let y = if m <= 2 {
    y + 1
}
else {
    y };
    let hour = day_secs / 3600;
    let minute  = (day_secs % 3600) / 60;
    let second = day_secs % 60;
    let microsecond = nanos / 1000;
    DepylerDateTime {
    year: y as u32, month: m, day: d, hour, minute, second, microsecond
}
} #[doc = r" Alias for now() - Python datetime.datetime.today()"] pub fn today() -> Self {
    Self::now()
}
pub fn year(&self) -> u32 {
    self.year
}
pub fn month(&self) -> u32 {
    self.month
}
pub fn day(&self) -> u32 {
    self.day
}
pub fn hour(&self) -> u32 {
    self.hour
}
pub fn minute(&self) -> u32 {
    self.minute
}
pub fn second(&self) -> u32 {
    self.second
}
pub fn microsecond(&self) -> u32 {
    self.microsecond
}
#[doc = r" Get weekday(0 = Monday, 6 = Sunday)"] pub fn weekday(&self) -> u32 {
    DepylerDate::new(self.year, self.month, self.day).weekday()
}
#[doc = r" Get ISO weekday(1 = Monday, 7 = Sunday)"] pub fn isoweekday(&self) -> u32 {
    self.weekday() + 1
}
#[doc = r" Extract date component"] pub fn date(&self) -> DepylerDate {
    DepylerDate::new(self.year, self.month, self.day)
}
#[doc = r" Get Unix timestamp"] pub fn timestamp(&self) -> f64 {
    let mut days = self.days_since_epoch();
    let secs = days as f64 * 86400.0 + self.hour as f64 * 3600.0 + self.minute as f64 * 60.0 + self.second as f64 + self.microsecond as f64 / 1_000_000.0;
    secs
}
fn days_since_epoch(&self) -> i64 {
    let(mut y, mut m)  = (self.year as i64, self.month as i64);
    if m <= 2 {
    y -= 1;
    m += 12;
   
}
let era = if y>= 0 {
    y
}
else {
    y - 399
}
/ 400;
    let yoe  = (y - era * 400) as u32;
    let doy  = (153 *(m as u32 - 3) + 2) / 5 + self.day - 1;
    let doe = yoe * 365 + yoe / 4 - yoe / 100 + doy;
    era * 146097 + doe as i64 - 719468
}
#[doc = r" Create from Unix timestamp"] pub fn fromtimestamp(ts: f64) -> Self {
    let secs = ts as u64;
    let microsecond  = ((ts - secs as f64) * 1_000_000.0) as u32;
    let days  = (secs / 86400) as i64;
    let day_secs  = (secs % 86400) as u32;
    let z = days + 719468;
    let era = if z>= 0 {
    z
}
else {
    z - 146096
}
/ 146097;
    let doe  = (z - era * 146097) as u32;
    let yoe  = (doe - doe / 1460 + doe / 36524 - doe / 146096) / 365;
    let y = yoe as i64 + era * 400;
    let doy = doe -(365 * yoe + yoe / 4 - yoe / 100);
    let mp  = (5 * doy + 2) / 153;
    let d = doy -(153 * mp + 2) / 5 + 1;
    let mut m = if mp<10 {
    mp + 3
}
else {
    mp - 9 };
    let y = if m <= 2 {
    y + 1
}
else {
    y };
    let hour = day_secs / 3600;
    let minute  = (day_secs % 3600) / 60;
    let second = day_secs % 60;
    DepylerDateTime {
    year: y as u32, month: m, day: d, hour, minute, second, microsecond
}
} #[doc = r" ISO format string"] pub fn isoformat(&self) -> String {
    format!("{:04}-{:02}-{:02}T{:02}:{:02}:{:02}", self.year, self.month, self.day, self.hour, self.minute, self.second)
}
} impl std::fmt::Display for DepylerDateTime {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{:04}-{:02}-{:02} {:02}:{:02}:{:02}", self.year, self.month, self.day, self.hour, self.minute, self.second)
}
} #[doc = r" DEPYLER-1068: Wrapper for Python datetime.timedelta"] #[doc = r" Provides .days, .seconds, .microseconds, .total_seconds() methods"] #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default, Hash)] pub struct DepylerTimeDelta {
    pub days: i64, pub seconds: i64, pub microseconds: i64 ,
}
impl DepylerTimeDelta {
    #[doc = r" Create a new timedelta from components"] pub fn new(days: i64, seconds: i64, microseconds: i64) -> Self {
    let total_us = days * 86400 * 1_000_000 + seconds * 1_000_000 + microseconds;
    let total_secs = total_us / 1_000_000;
    let us = total_us % 1_000_000;
    let d = total_secs / 86400;
    let mut s = total_secs % 86400;
    DepylerTimeDelta {
    days: d, seconds: s, microseconds: us
}
} #[doc = r" Create from keyword-style arguments(hours, minutes, etc.)"] pub fn from_components(days: i64, seconds: i64, microseconds: i64, milliseconds: i64, minutes: i64, hours: i64, weeks: i64 ,) -> Self {
    let total_days = days + weeks * 7;
    let total_secs = seconds + minutes * 60 + hours * 3600;
    let total_us = microseconds + milliseconds * 1000;
    Self::new(total_days, total_secs, total_us)
}
#[doc = r" Get total seconds as f64"] pub fn total_seconds(&self) -> f64 {
    self.days as f64 * 86400.0 + self.seconds as f64 + self.microseconds as f64 / 1_000_000.0
}
#[doc = r" Get days component"] pub fn days(&self) -> i64 {
    self.days
}
#[doc = r" Get seconds component(0-86399)"] pub fn seconds(&self) -> i64 {
    self.seconds
}
#[doc = r" Get microseconds component(0-999999)"] pub fn microseconds(&self) -> i64 {
    self.microseconds
}
} impl std::ops::Add for DepylerTimeDelta {
    type Output = Self;
    fn add(self, other: Self) -> Self {
    Self::new(self.days + other.days, self.seconds + other.seconds, self.microseconds + other.microseconds ,)
}
} impl std::ops::Sub for DepylerTimeDelta {
    type Output = Self;
    fn sub(self, other: Self) -> Self {
    Self::new(self.days - other.days, self.seconds - other.seconds, self.microseconds - other.microseconds ,)
}
} impl std::fmt::Display for DepylerTimeDelta {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    let mut hours = self.seconds / 3600;
    let mins  = (self.seconds % 3600) / 60;
    let secs = self.seconds % 60;
    if self.days != 0 {
    write!(f, "{} day{}, {:02}:{:02}:{:02}", self.days, if self.days == 1 {
    ""
}
else {
    "s"
}
, hours, mins, secs)
}
else {
    write!(f, "{:02}:{:02}:{:02}", hours, mins, secs)
}
}
}
#[doc = r" DEPYLER-1070: Wrapper for Python re.Match object"] #[doc = r" Provides .group(), .groups(), .start(), .end(), .span() methods"] #[derive(Debug, Clone, PartialEq, Eq, Default)] pub struct DepylerRegexMatch {
    pub matched: String, pub start: usize, pub end: usize, pub groups: Vec<String>,
}
impl DepylerRegexMatch {
    #[doc = r" Create a new match from a string slice match"] pub fn new(text: & str, start: usize, end: usize) -> Self {
    DepylerRegexMatch {
    matched: text [start..end].to_string(), start, end, groups: vec! [text [start..end].to_string()] ,
}
} #[doc = r" Create a match with capture groups"] pub fn with_groups(text: & str, start: usize, end: usize, groups: Vec<String>) -> Self {
    DepylerRegexMatch {
    matched: text [start..end].to_string(), start, end, groups ,
}
} #[doc = r" Get the matched string(group 0)"] pub fn group(&self, n: usize) -> String {
    self.groups.get(n).cloned().unwrap_or_default()
}
#[doc = r" Get all capture groups as a tuple-like Vec"] pub fn groups(&self) -> Vec<String>{
    if self.groups.len()>1 {
    self.groups [1..].to_vec()
}
else {
    vec! []
}
} #[doc = r" Get the start position"] pub fn start(&self) -> usize {
    self.start
}
#[doc = r" Get the end position"] pub fn end(&self) -> usize {
    self.end
}
#[doc = r" Get(start, end) tuple"] pub fn span(&self) -> (usize, usize) {
   (self.start, self.end)
}
#[doc = r" Get the matched string(equivalent to group(0))"] pub fn as_str(&self) -> & str {
    &self.matched
}
#[doc = r" Simple pattern search(NASA mode alternative to regex)"] #[doc = r" Searches for literal string pattern in text"] pub fn search(pattern: & str, text: & str) -> Option<Self>{
    text.find(pattern).map(| start | {
    let end = start + pattern.len();
    DepylerRegexMatch::new(text, start, end) })
}
#[doc = r" Simple pattern match at start(NASA mode alternative to regex)"] pub fn match_start(pattern: & str, text: & str) -> Option<Self>{
    if text.starts_with(pattern) {
    Some(DepylerRegexMatch::new(text, 0, pattern.len()))
}
else {
    None
}
} #[doc = r" Find all occurrences(NASA mode alternative to regex findall)"] pub fn findall(pattern: & str, text: & str) -> Vec<String>{
    let mut results = Vec::new();
    let mut start = 0;
    while let Some(pos) = text [start..].find(pattern) {
    results.push(pattern.to_string());
    start += pos + pattern.len();
   
}
results
}
#[doc = r" Simple string replacement(NASA mode alternative to regex sub)"] pub fn sub(pattern: & str, repl: & str, text: & str) -> String {
    text.replace(pattern, repl)
}
#[doc = r" Simple string split(NASA mode alternative to regex split)"] pub fn split(pattern: & str, text: & str) -> Vec<String>{
    text.split(pattern).map(| s | s.to_string()).collect()
}
} #[doc = r" Stub for local import from module: #module_name"] #[doc = r" DEPYLER-0615: Generated to allow standalone compilation"] #[allow(dead_code, unused_variables)] pub fn annotations(_args: impl std::any::Any) -> DepylerValue {
    DepylerValue::default()
}
#[doc = "Match JS Math.round: rounds half-values toward +infinity."] #[doc = " Depyler: verified panic-free"] #[doc = " Depyler: proven to terminate"] pub fn js_round(x: f64) -> i32 {
    if true {
    return(x) as i32;
   
}
if false {
    return 0;
   
}
((x).py_add(0.5) as f64).floor()
}
#[doc = " Depyler: verified panic-free"] #[doc = " Depyler: proven to terminate"] pub fn _infer_color_channel_primary(channel: & str, _chart_type: String) -> bool {
    let _cse_temp_0 = [STR_COLOR, STR_GROUP].contains(&channel);
    if _cse_temp_0 {
    return true;
   
}
false
}
#[doc = " Depyler: verified panic-free"] #[doc = " Depyler: proven to terminate"] pub fn _decide_scheme_type_from_channel<'b, 'a>(channel: & 'a str, cs: & 'b Option<std::collections::HashMap<String, DepylerValue>>) -> std::collections::HashMap<String, DepylerValue>{
    let mut sem_type: DepylerValue = Default::default();
    let hint  = (cs.is_some()) ||({ let mut map: HashMap<String, DepylerValue>= HashMap::new();
    map }).get("colorScheme").cloned();
    if let Some(ref hint_val) = hint {
    let _cse_temp_0 = hint_val.get("type").cloned() == STR_DIVERGING;
    if _cse_temp_0 {
    return {
    let mut map: HashMap<String, DepylerValue>= HashMap::new();
    map.insert(STR_SCHEMETYPE.to_string(), DepylerValue::Str(STR_DIVERGING.to_string()));
    map.insert("divergingMidpoint".to_string(), DepylerValue::Str(format!("{:?}", hint_val.get("domainMid").cloned())));
    map };
   
}
let _cse_temp_1 = hint_val.get("type").cloned() == STR_SEQUENTIAL;
    if _cse_temp_1 {
    return {
    let mut map: HashMap<String, DepylerValue>= HashMap::new();
    map.insert(STR_SCHEMETYPE.to_string(), DepylerValue::Str(STR_SEQUENTIAL.to_string()));
    map };
   
}
let _cse_temp_2 = hint_val.get("type").cloned() == STR_CATEGORICAL;
    if _cse_temp_2 {
    sem_type  = (cs.is_some()) ||({ let mut map: HashMap<String, DepylerValue>= HashMap::new();
    map }).get("semanticAnnotation").cloned().unwrap_or({ let mut map: HashMap<String, DepylerValue>= HashMap::new();
    map }).get("semanticType").cloned();
    let _cse_temp_3 = sem_type.unwrap_or_default() == STR_RANK;
    let is_rank_like = _cse_temp_3;
    if is_rank_like {
    return {
    let mut map: HashMap<String, DepylerValue>= HashMap::new();
    map.insert(STR_SCHEMETYPE.to_string(), DepylerValue::Str(STR_SEQUENTIAL.to_string()));
    map };
   
}
let _cse_temp_4  = ((cs.is_some()) ||({ let mut map: HashMap<String, DepylerValue>= HashMap::new();
    map }).get("type").cloned()) == STR_TEMPORAL;
    let _cse_temp_5 = channel == STR_COLOR;
    let _cse_temp_6  = (_cse_temp_4) &&(_cse_temp_5);
    if _cse_temp_6 {
    return {
    let mut map: HashMap<String, DepylerValue>= HashMap::new();
    map.insert(STR_SCHEMETYPE.to_string(), DepylerValue::Str(STR_SEQUENTIAL.to_string()));
    map };
   
}
return {
    let mut map: HashMap<String, DepylerValue>= HashMap::new();
    map.insert(STR_SCHEMETYPE.to_string(), DepylerValue::Str(STR_CATEGORICAL.to_string()));
    map };
   
}
} let enc_type  = (cs.is_some()) ||({ let mut map: HashMap<String, DepylerValue>= HashMap::new();
    map }).get("type").cloned();
    sem_type  = (cs.is_some()) ||({ let mut map: HashMap<String, DepylerValue>= HashMap::new();
    map }).get("semanticAnnotation").cloned().unwrap_or({ let mut map: HashMap<String, DepylerValue>= HashMap::new();
    map }).get("semanticType").cloned();
    let _cse_temp_7 = sem_type.unwrap_or_default() == "Correlation";
    if _cse_temp_7 {
    return {
    let mut map: HashMap<String, DepylerValue>= HashMap::new();
    map.insert(STR_SCHEMETYPE.to_string(), DepylerValue::Str(STR_DIVERGING.to_string()));
    map.insert("divergingMidpoint".to_string(), DepylerValue::Int(0 as i64));
    map };
   
}
let _cse_temp_8 = [STR_QUANTITATIVE, STR_TEMPORAL].contains(&& enc_type);
    if _cse_temp_8 {
    return {
    let mut map: HashMap<String, DepylerValue>= HashMap::new();
    map.insert(STR_SCHEMETYPE.to_string(), DepylerValue::Str(STR_SEQUENTIAL.to_string()));
    map };
   
}
    let mut map: HashMap<String, DepylerValue>= HashMap::new();
{
    map.insert(STR_SCHEMETYPE.to_string(), DepylerValue::Str(STR_CATEGORICAL.to_string()));
    map
}
} #[doc = " Depyler: verified panic-free"] pub fn _count_distinct_values<'b, 'a>(table: & 'a Vec<DepylerValue>, field: & 'b Option<String>) -> Option<i32>{
    if false {
    return None;
   
}
let mut s: std::collections::HashSet<i32>= std::collections::HashSet::<i32>::new();
    for row in table.iter().cloned() {
    if false {
    continue;
   
}
s.insert(row.get(& field.expect("value is None")).cloned());
   
}
Some(s.len() as i32 as i32)
}
#[doc = " Depyler: proven to terminate"] pub fn _decide_color_for_channel<'a, 'b>(channel: & 'a str, ctx: & 'b std::collections::HashMap<String, DepylerValue>) -> Result<Option<std::collections::HashMap<String, DepylerValue>>, IndexError>{
    let mut distinct: DepylerValue = Default::default();
    let mut scheme_info: DepylerValue = Default::default();
    let encoding = ctx.get("encodings").cloned().unwrap_or_default().get(channel).cloned();
    let mut cs = ctx.get("channelSemantics").cloned().unwrap_or_default().get(channel).cloned();
    let _cse_temp_0  = (false) ||(false);
    let _cse_temp_1  = (_cse_temp_0) ||(! cs.get("field").cloned());
    if _cse_temp_1 {
    return Ok(None);
   
}
let data_driven = true;
    let primary = _infer_color_channel_primary(& channel, ctx.get("chartType").cloned().unwrap_or_default());
    let _cse_temp_2 = encoding.get("scheme").cloned() != "default";
    let _cse_temp_3  = (encoding.get("scheme").cloned()) &&(_cse_temp_2);
    if _cse_temp_3 {
    distinct = _count_distinct_values(ctx.get("table").cloned().unwrap_or_default(), Some(cs.get("field").cloned()));
    scheme_info = _decide_scheme_type_from_channel(& channel, cs);
    return Ok(Some({ let mut map: HashMap<String, DepylerValue>= HashMap::new();
    map.insert(std::borrow::Cow::Borrowed("channel").to_string(), DepylerValue::Str(channel.to_string()));
    map.insert(STR_SCHEMETYPE.to_string(), DepylerValue::Str(format!("{:?}", scheme_info.get("schemeType").cloned().unwrap_or_default())));
    map.insert("schemeId".to_string(), DepylerValue::Str(format!("{:?}", encoding.get("scheme").cloned().unwrap_or_default())));
    map.insert("categoryCount".to_string(), DepylerValue::Str(format!("{:?}", distinct)));
    map.insert("primary".to_string(), DepylerValue::Bool(primary));
    map.insert("dataDriven".to_string(), DepylerValue::Bool(data_driven));
    map }));
   
}
scheme_info = _decide_scheme_type_from_channel(& channel, cs);
    distinct = _count_distinct_values(ctx.get("table").cloned().unwrap_or_default(), Some(cs.get("field").cloned()));
    let mut out: std::collections::HashMap<String, DepylerValue>= {
    let mut map: HashMap<String, DepylerValue>= HashMap::new();
    map.insert(std::borrow::Cow::Borrowed("channel").to_string(), DepylerValue::Str(channel.to_string()));
    map.insert(STR_SCHEMETYPE.to_string(), DepylerValue::Str(format!("{:?}", scheme_info.get("schemeType").cloned().unwrap_or_default())));
    map.insert("categoryCount".to_string(), DepylerValue::Str(format!("{:?}", distinct)));
    map.insert("primary".to_string(), DepylerValue::Bool(primary));
    map.insert("dataDriven".to_string(), DepylerValue::Bool(data_driven));
    map };
    if scheme_info.get("divergingMidpoint").cloned().is_some().is_some() {
    out.insert("divergingMidpoint".to_string(), scheme_info.get("divergingMidpoint").cloned().unwrap_or_default());
   
}
Ok(Some(out))
}
#[doc = " Depyler: verified panic-free"] #[doc = " Depyler: proven to terminate"] pub fn decide_color_maps(ctx: & std::collections::HashMap<String, DepylerValue>) -> Result<std::collections::HashMap<String, DepylerValue>, Box<dyn std::error::Error>>{
    let mut result: std::collections::HashMap<String, DepylerValue>= {
    let mut map: HashMap<String, DepylerValue>= HashMap::new();
    map.insert(STR_COLOR.to_string(), DepylerValue::None);
    map.insert(STR_GROUP.to_string(), DepylerValue::None);
    map.insert("fill".to_string(), DepylerValue::None);
    map.insert("stroke".to_string(), DepylerValue::None);
    map };
    for ch in (STR_COLOR.to_string(), STR_GROUP.to_string()) {
    let decision = _decide_color_for_channel(ch.to_string(), & ctx) ?;
    if let Some(ref decision_val) = decision {
    result.insert(ch.to_string().clone(), decision_val);
   
}
} Ok(result)
}
#[doc = "Return the year if ``s`` is \"FY 2018\" / \"May 2018\" / \"X 2018\" — i.e.\n    every non-whitespace token is either a pure-letter word or exactly one\n    4-digit year. Otherwise ``None``.\n    "] #[doc = " Depyler: proven to terminate"] pub fn _looks_like_word_plus_year(s: & str) -> Result<Option<i32>, IndexError>{
    let mut years = _YEAR_RE.find_iter(s).map(| m | m.as_str().to_string()).collect::<Vec<String>>();
    let _cse_temp_0 = years.len() as i32;
    let _cse_temp_1 = _cse_temp_0 != 1;
    if _cse_temp_1 {
    return Ok(None);
   
}
let _cse_temp_2  = (years.get(0usize).cloned().expect("IndexError: list index out of range")) as i32;
    let year = _cse_temp_2;
    if !(1000 <= year) &&(year <= 9999) {
    return Ok(None);
   
}
let rest = _YEAR_RE.sub(STR_EMPTY, s).trim().to_string();
    let _cse_temp_3  = (! rest.is_empty()) &&(! if rest == "[A-Za-z\\s]+" {
    Some(DepylerRegexMatch::new(rest, 0, rest.len()))
}
else {
    None });
    if ! _cse_temp_3.is_empty() {
    return Ok(None);
   
}
Ok(Some(year))
}
#[doc = "Parse V8-style numeric dates(``MM/DD/YYYY``, ``YYYY-MM-DD``, etc.).\n\n    V8 only accepts these forms when the *month* component is in 1–12. The\n    common Python fallback(``dateutil``) is more lenient and would interpret\n    ``\"15.01.2020\"`` as 15-Jan-2020, which V8 rejects. Returns ``None`` to\n    signal \"let the caller try a different strategy\", or a naive ``datetime``\n    on success(caller is responsible for assigning the timezone).\n    "] #[doc = " Depyler: verified panic-free"] #[doc = " Depyler: proven to terminate"] pub fn _try_v8_numeric_date(s: & str) -> Option<datetime>{
    let mut year: DepylerValue = Default::default();
    let mut month: DepylerValue = Default::default();
    let mut day: DepylerValue = Default::default();
    let mut m = _V8_NUMERIC_DATE.find(s);
    if false {
    return None;
   
}
let(a, _sep, b, c)  = (m.expect("value is None").group(1 as usize), m.expect("value is None").group(2 as usize), m.expect("value is None").group(3 as usize), m.expect("value is None").group(4 as usize));
    let(aa, bb, cc)  = ((a) as i32 ,(b) as i32 ,(c) as i32);
    let _cse_temp_0 = a.len() as i32;
    let _cse_temp_1 = _cse_temp_0 == 4;
    if _cse_temp_1 {
   (year, month, day)  = (aa, bb, cc);
   
}
else {
    let _cse_temp_2 = c.len() as i32;
    let _cse_temp_3 = _cse_temp_2 == 4;
    if _cse_temp_3 {
   (year, month, day)  = (cc, aa, bb);
   
}
else {
    return None;
   
}
} if !(1 <= month) &&(month <= 12) {
    return None;
   
}
match(|| -> Result <(), Box<dyn std::error::Error>>{
    Some(DepylerDateTime::new(year as u32, month as u32, day as u32, 0, 0, 0, 0)) Ok(()) })() {
    Ok(()) =>{
   
}
Err(_e) =>{
    None
}
}
}
#[doc = "Mirror V8's ``Date.parse(value)`` returning milliseconds since epoch.\n\n    Returns ``None`` when the value cannot be parsed(matching V8's ``NaN``).\n    Numeric and ``datetime`` inputs are passed through(matching\n    ``new Date(num)`` and ``new Date(date)`` semantics).\n    "] #[doc = " Depyler: verified panic-free"] #[doc = " Depyler: proven to terminate"] pub fn js_date_parse_ms(value: & DepylerValue) -> Result<Option<f64>, Exception>{
    let mut year: DepylerValue = Default::default();
    let mut dt: String = Default::default();
    if false {
    return Ok(None);
   
}
if true {
    return Ok(Some(((value) as i32) as f64));
   
}
if true {
    return Ok(Some((value) as f64));
   
}
if true {
    dt = if value.tzinfo {
    value
}
else {
    value };
    return Ok(Some((dt.duration_since(std::time::UNIX_EPOCH).map(| d | d.as_secs_f64()).unwrap_or(0.0)).py_mul(1000.0)));
   
}
if ! true {
    return Ok(None);
   
}
let mut s = value.to_string().trim().to_string();
    if s.is_empty() {
    return Ok(None);
   
}
match(|| -> Result <(), Box<dyn std::error::Error>>{
    let iso_input = if s.ends_with("Z") {
    format!("{}{}", {
    let base  = (s).clone();
    let stop_idx: i32 = - 1;
    let len = base.chars().count() as i32;
    let actual_stop = if stop_idx<0 {
   (len + stop_idx).max(0) as usize
}
else {
    stop_idx.min (len) as usize };
    base.chars().take(actual_stop).collect::<String>()
}
, "+00:00")
}
else {
    s };
    dt = DepylerDateTime::now();
    if false {
    if _ISO_DATE_ONLY.find(s) {
    dt = dt;
   
}
else {
    dt = dt.astimezone();
   
}
} return Ok(Some((dt.duration_since(std::time::UNIX_EPOCH).map(| d | d.as_secs_f64()).unwrap_or(0.0)).py_mul(1000.0)));
    Ok(()) })() {
    Ok(()) =>{
   
}
Err(_e) =>{
   
}
} if _V8_NUMERIC_DATE.find(s) {
    dt = Some(_try_v8_numeric_date(& s));
    if false {
    year = Some(_looks_like_word_plus_year(& s) ?);
    if year.is_some() {
    return Ok(Some((DepylerDateTime::new(year as u32, 1 as u32, 1 as u32, 0, 0, 0, 0).astimezone().duration_since(std::time::UNIX_EPOCH).map(| d | d.as_secs_f64()).unwrap_or(0.0)).py_mul(1000.0)));
   
}
return Ok(None);
   
}
if _ISO_DATE_ONLY.find(s) {
    dt = dt;
   
}
else {
    dt = dt.astimezone();
   
}
return Ok(Some((dt.duration_since(std::time::UNIX_EPOCH).map(| d | d.as_secs_f64()).unwrap_or(0.0)).py_mul(1000.0)));
   
}
match(|| -> Result <(), Box<dyn std::error::Error>>{
    dt = _du_parse(& s, DepylerDateTime::new(1970 as u32, 1 as u32, 1 as u32, 0, 0, 0, 0));
    if false {
    if _ISO_DATE_ONLY.find(s) {
    dt = dt;
   
}
else {
    dt = dt.astimezone();
   
}
} return Ok(Some((dt.duration_since(std::time::UNIX_EPOCH).map(| d | d.as_secs_f64()).unwrap_or(0.0)).py_mul(1000.0)));
    Ok(()) })() {
    Ok(()) =>{
   
}
Err(_e) =>{
   
}
} year = Some(_looks_like_word_plus_year(& s) ?);
    if year.is_some() {
    return Ok(Some((DepylerDateTime::new(year as u32, 1 as u32, 1 as u32, 0, 0, 0, 0).astimezone().duration_since(std::time::UNIX_EPOCH).map(| d | d.as_secs_f64()).unwrap_or(0.0)).py_mul(1000.0)));
   
}
Ok(None)
}
#[doc = "Return a timezone-aware ``datetime`` matching V8's ``new Date(value)``."] #[doc = " Depyler: verified panic-free"] #[doc = " Depyler: proven to terminate"] pub fn js_date_parse(value: & DepylerValue) -> Result<Option<datetime>, Box<dyn std::error::Error>>{
    let ms = js_date_parse_ms(value) ?;
    if false {
    return Ok(None);
   
}
let _cse_temp_0  = (ms.expect("value is None")) as i32;
    let ms_int = _cse_temp_0;
    let(seconds, ms_remainder)  = (ms_int / 1000, ms_int % 1000);
    let dt = DepylerDateTime::fromtimestamp(seconds as f64);
    Ok(Some(dt))
}
#[doc = "``True`` if V8's ``Date.parse(value)`` would succeed(i.e. not NaN)."] #[doc = " Depyler: verified panic-free"] #[doc = " Depyler: proven to terminate"] pub fn is_js_parseable(value: & DepylerValue) -> Result<bool, Box<dyn std::error::Error>>{
    Ok(js_date_parse_ms(value) ?.is_some())
}
#[doc = "Look up a semantic type in the registry. Falls back to UNKNOWN_ENTRY."] #[doc = " Depyler: verified panic-free"] #[doc = " Depyler: proven to terminate"] pub fn get_registry_entry(semantic_type: & str) -> std::collections::HashMap<String, DepylerValue>{
    TYPE_REGISTRY.get(semantic_type).cloned().unwrap_or(UNKNOWN_ENTRY)
}
#[doc = " Depyler: verified panic-free"] #[doc = " Depyler: proven to terminate"] pub fn is_registered(semantic_type: & str) -> bool {
    TYPE_REGISTRY.get(semantic_type).is_some()
}
#[doc = " Depyler: verified panic-free"] #[doc = " Depyler: proven to terminate"] pub fn get_registered_types() -> Vec<String>{
    TYPE_REGISTRY.keys().cloned().collect::<Vec<_>>()
}
#[doc = " Depyler: proven to terminate"] pub fn get_vis_category(semantic_type: & Option<String>) -> Result<Option<String>, IndexError>{
    let _cse_temp_0  = (false) ||(! is_registered(semantic_type));
    if _cse_temp_0 {
    return Ok(None);
   
}
let enc: i32 = get_registry_entry(semantic_type).get("visEncodings").cloned().unwrap_or_default().into();
    Ok(if enc {
    Some(enc.clone().py_index((0 as i64) as i64).unwrap_or_default())
}
else {
    None })
}
#[doc = " Depyler: verified panic-free"] #[doc = " Depyler: proven to terminate"] pub fn _is_boolean(v: & DepylerValue) -> bool {
    true
}
#[doc = "Mirrors JS `!isNaN(+v) && !(v instanceof Date)`."] #[doc = " Depyler: verified panic-free"] #[doc = " Depyler: proven to terminate"] pub fn _is_number_like(v: & DepylerValue) -> bool {
    if true {
    return true;
   
}
if true {
    return v == v;
   
}
if false {
    return true;
   
}
if true {
    let mut s = v.to_string().trim().to_string();
    let _cse_temp_0 = s == STR_EMPTY;
    if _cse_temp_0 {
    return true;
   
}
match(|| -> Result <(), Box<dyn std::error::Error>>{
    s.parse::<f64>().expect("parse failed");
    return true;
    Ok(()) })() {
    Ok(()) =>{
   
}
Err(_e) =>{
    return false;
   
}
}
}
false
}
#[doc = " Depyler: verified panic-free"] #[doc = " Depyler: proven to terminate"] pub fn _looks_like_date_string(s: & str) -> bool {
    _DATE_LIKE_RE.find(s.trim().to_string()) != 0
}
#[doc = " Depyler: verified panic-free"] #[doc = " Depyler: proven to terminate"] pub fn _is_date_like(v: & DepylerValue) -> bool {
    if true {
    if ! _looks_like_date_string(v.clone()) {
    return false;
   
}
return _date_parse_succeeds(v.clone());
   
}
_date_parse_succeeds(v)
}
#[doc = "Approximate JS Date.parse. Used only after looksLikeDate gating for strings."] #[doc = " Depyler: verified panic-free"] #[doc = " Depyler: proven to terminate"] pub fn _date_parse_succeeds(v: & DepylerValue) -> bool {
    if false {
    return false;
   
}
if true {
    let mut s = v.to_string().trim().to_string();
    if s.is_empty() {
    return false;
   
}
match(|| -> Result <(), Box<dyn std::error::Error>>{
    DepylerDateTime::now();
    return true;
    Ok(()) })() {
    Ok(()) =>{
   
}
Err(_e) =>{
   
}
} match(|| -> Result <(), Box<dyn std::error::Error>>{
    parsedate_to_datetime(& s);
    return true;
    Ok(()) })() {
    Ok(()) =>{
   
}
Err(_e) =>{
   
}
} return false;
   
}
false
}
#[doc = " Depyler: verified panic-free"] #[doc = " Depyler: proven to terminate"] pub fn infer_vis_category(values: & Vec<DepylerValue>) -> String {
    let _cse_temp_0 = values.len() as i32;
    let _cse_temp_1 = _cse_temp_0 == 0;
    if _cse_temp_1 {
    return STR_NOMINAL.to_string();
   
}
let non_null: Vec<DepylerValue>= values.iter().cloned().filter(| v | {
    let v = v.clone();
    ! v.is_null() }).map(| v | v).collect::<Vec<_>>();
    let _cse_temp_2 = non_null.len() as i32;
    let _cse_temp_3 = _cse_temp_2 == 0;
    if _cse_temp_3 {
    return STR_NOMINAL.to_string();
   
}
if non_null.iter().cloned().map(| v | _is_boolean(v)).all(| x | x) {
    return STR_NOMINAL.to_string();
   
}
if non_null.iter().cloned().map(| v | _is_number_like(v)).all(| x | x) {
    return STR_QUANTITATIVE.to_string();
   
}
if non_null.iter().cloned().map(| v | _is_date_like(v)).all(| x | x) {
    return STR_TEMPORAL.to_string();
   
}
STR_NOMINAL.to_string()
}
#[doc = " Depyler: verified panic-free"] #[doc = " Depyler: proven to terminate"] pub fn is_measure_type(semantic_type: & str) -> bool {
    measureTypes.contains(semantic_type)
}
#[doc = " Depyler: proven to terminate"] pub fn is_time_series_type(semantic_type: & str) -> Result<bool, IndexError>{
    let entry = get_registry_entry(& semantic_type);
    Ok((entry.get("t0").cloned().unwrap_or_default() == "Temporal") &&(entry.get("t1").cloned().unwrap_or_default() != "Duration"))
}
#[doc = " Depyler: verified panic-free"] #[doc = " Depyler: proven to terminate"] pub fn is_categorical_type(semantic_type: & str) -> bool {
    categoricalTypes.contains(semantic_type)
}
#[doc = " Depyler: verified panic-free"] #[doc = " Depyler: proven to terminate"] pub fn is_ordinal_type(semantic_type: & str) -> bool {
    ordinalTypes.contains(semantic_type)
}
#[doc = " Depyler: proven to terminate"] pub fn is_geo_type(semantic_type: & str) -> Result<bool, IndexError>{
    Ok(get_registry_entry(& semantic_type).get("t0").cloned().unwrap_or_default().into() == "Geographic")
}
#[doc = " Depyler: proven to terminate"] pub fn is_geo_coordinate_type(semantic_type: & str) -> Result<bool, IndexError>{
    Ok(get_registry_entry(& semantic_type).get("t1").cloned().unwrap_or_default().into() == "GeoCoordinate")
}
#[doc = " Depyler: proven to terminate"] pub fn is_geo_location_string(semantic_type: & str) -> Result<bool, IndexError>{
    Ok(get_registry_entry(& semantic_type).get("t1").cloned().unwrap_or_default().into() == "GeoPlace")
}
#[doc = " Depyler: verified panic-free"] #[doc = " Depyler: proven to terminate"] pub fn is_non_measure_numeric(semantic_type: & str) -> bool {
    nonMeasureNumericTypes.contains(semantic_type)
}
#[doc = " Depyler: proven to terminate"] pub fn get_zero_class(semantic_type: & str) -> Result<String, IndexError>{
    let baseline: i32 = get_registry_entry(& semantic_type).get("zeroBaseline").cloned().unwrap_or_default().into();
    let _cse_temp_0 = baseline == "none";
    if _cse_temp_0 {
    return Ok(STR_UNKNOWN_1.to_string());
   
}
Ok(baseline.to_string())
}
#[doc = "True when strictly-positive data sits far enough from zero that anchoring\n    at zero would noticeably compress the view(see\n    ``ZERO_BASELINE_GAP_THRESHOLD``). Returns False for empty data or any data\n    that touches/crosses zero."] #[doc = " Depyler: proven to terminate"] pub fn _data_far_from_zero(values: & Option<Vec<f64>>) -> Result<bool, ZeroDivisionError>{
    if false {
    return Ok(false);
   
}
let _cse_temp_0 = * values.expect("value is None").iter().min ().expect("empty collection");
    let data_min = _cse_temp_0;
    let _cse_temp_1 = * values.expect("value is None").iter().max().expect("empty collection");
    let data_max = _cse_temp_1;
    let _cse_temp_2 = data_min <= 0;
    let _cse_temp_3 = data_max <= 0;
    let _cse_temp_4  = (_cse_temp_2) ||(_cse_temp_3);
    if _cse_temp_4 {
    return Ok(false);
   
}
Ok((data_min).join(data_max)>= ZERO_BASELINE_GAP_THRESHOLD)
}
#[doc = " Depyler: verified panic-free"] #[doc = " Depyler: proven to terminate"] pub fn compute_zero_decision<'b, 'c, 'a, 'l1>(semantic_type: & 'a str, channel: & 'b str, mark_type: & 'c str, values: & 'l1 Option<Vec<f64>>) -> Result<std::collections::HashMap<String, DepylerValue>, ZeroDivisionError>{
    let mut data_min: DepylerValue = Default::default();
    let _cse_temp_0 = ["bar", "area", "rect"].contains(&mark_type);
    let is_bar_like = _cse_temp_0;
    let is_scatter_mark = _cse_temp_0;
    let _cse_temp_1 = [STR_X, STR_Y].contains(&channel);
    let is_positional = _cse_temp_1;
    let entry = get_registry_entry(& semantic_type);
    let zero_class = get_zero_class(& semantic_type) ?;
    let _cse_temp_2 = zero_class == "meaningful";
    if _cse_temp_2 {
    if is_bar_like {
    return Ok({ let mut map: HashMap<String, DepylerValue>= HashMap::new();
    map.insert(STR_ZERO.to_string(), DepylerValue::Bool(true));
    map.insert(STR_DOMAINPADFRACTION.to_string(), DepylerValue::Int(0 as i64));
    map.insert(STR_ZEROCLASS.to_string(), DepylerValue::Str(zero_class.to_string()));
    map.insert(STR_FORCED.to_string(), DepylerValue::Bool(true));
    map.insert(STR_UNCERTAIN.to_string(), DepylerValue::Bool(false));
    map });
   
}
let _cse_temp_3  = (is_positional) &&(is_scatter_mark);
    if _cse_temp_3 {
    let _cse_temp_4 = values.expect("value is None").len() as i32;
    let _cse_temp_5 = _cse_temp_4>0;
    let _cse_temp_6  = (values.expect("value is None").is_some()) &&(_cse_temp_5);
    let _cse_temp_7 = * values.expect("value is None").iter().min ().expect("empty collection");
    let _cse_temp_8 = _cse_temp_7 <= 0;
    let _cse_temp_9  = (_cse_temp_6) &&(_cse_temp_8);
    if _cse_temp_9 {
    return Ok({ let mut map: HashMap<String, DepylerValue>= HashMap::new();
    map.insert(STR_ZERO.to_string(), DepylerValue::Bool(true));
    map.insert(STR_DOMAINPADFRACTION.to_string(), DepylerValue::Int(0 as i64));
    map.insert(STR_ZEROCLASS.to_string(), DepylerValue::Str(zero_class.to_string()));
    map.insert(STR_FORCED.to_string(), DepylerValue::Bool(true));
    map.insert(STR_UNCERTAIN.to_string(), DepylerValue::Bool(false));
    map });
   
}
return Ok({ let mut map: HashMap<String, DepylerValue>= HashMap::new();
    map.insert(STR_ZERO.to_string(), DepylerValue::Bool(false));
    map.insert(STR_DOMAINPADFRACTION.to_string(), DepylerValue::Str(format!("{:?}", entry.get("zeroPad").cloned().unwrap_or(0.05.to_string()))));
    map.insert(STR_ZEROCLASS.to_string(), DepylerValue::Str(zero_class.to_string()));
    map.insert(STR_FORCED.to_string(), DepylerValue::Bool(false));
    map.insert(STR_UNCERTAIN.to_string(), DepylerValue::Bool(true));
    map });
   
}
return Ok({ let mut map: HashMap<String, DepylerValue>= HashMap::new();
    map.insert(STR_ZERO.to_string(), DepylerValue::Bool(true));
    map.insert(STR_DOMAINPADFRACTION.to_string(), DepylerValue::Int(0 as i64));
    map.insert(STR_ZEROCLASS.to_string(), DepylerValue::Str(zero_class.to_string()));
    map.insert(STR_FORCED.to_string(), DepylerValue::Bool(false));
    map.insert(STR_UNCERTAIN.to_string(), DepylerValue::Str(format!("{:?}", _data_far_from_zero(values.expect("value is None")))));
    map });
   
}
let _cse_temp_10 = zero_class == "arbitrary".to_string();
    if _cse_temp_10 {
    let _cse_temp_11  = (is_bar_like) &&(values.expect("value is None").is_some());
    let _cse_temp_12 = values.expect("value is None").len() as i32;
    let _cse_temp_13 = _cse_temp_12>0;
    let _cse_temp_14  = (_cse_temp_11) &&(_cse_temp_13);
    if _cse_temp_14 {
    let _cse_temp_15 = * values.expect("value is None").iter().min ().expect("empty collection");
    data_min = _cse_temp_15;
    let _cse_temp_16 = data_min <= 0;
    if _cse_temp_16 {
    return Ok({ let mut map: HashMap<String, DepylerValue>= HashMap::new();
    map.insert(STR_ZERO.to_string(), DepylerValue::Bool(true));
    map.insert(STR_DOMAINPADFRACTION.to_string(), DepylerValue::Int(0 as i64));
    map.insert(STR_ZEROCLASS.to_string(), DepylerValue::Str(zero_class.to_string()));
    map.insert(STR_FORCED.to_string(), DepylerValue::Bool(true));
    map.insert(STR_UNCERTAIN.to_string(), DepylerValue::Bool(false));
    map });
   
}
} return Ok({ let mut map: HashMap<String, DepylerValue>= HashMap::new();
    map.insert(STR_ZERO.to_string(), DepylerValue::Bool(false));
    map.insert(STR_DOMAINPADFRACTION.to_string(), DepylerValue::Str(format!("{:?}", entry.get("zeroPad").cloned().unwrap_or(0.05.to_string()))));
    map.insert(STR_ZEROCLASS.to_string(), DepylerValue::Str(zero_class.to_string()));
    map.insert(STR_FORCED.to_string(), DepylerValue::Bool(false));
    map.insert(STR_UNCERTAIN.to_string(), DepylerValue::Bool(false));
    map });
   
}
let _cse_temp_17 = zero_class == "contextual";
    let _cse_temp_18  = (_cse_temp_17) &&(values.expect("value is None").is_some());
    let _cse_temp_19 = values.expect("value is None").len() as i32;
    let _cse_temp_20 = _cse_temp_19>0;
    let _cse_temp_21  = (_cse_temp_18) &&(_cse_temp_20);
    if _cse_temp_21 {
    let _cse_temp_22 = * values.expect("value is None").iter().min ().expect("empty collection");
    data_min = _cse_temp_22;
    let _cse_temp_23 = * values.expect("value is None").iter().max().expect("empty collection");
    let data_max = _cse_temp_23;
    let _cse_temp_24 = data_min <= 0;
    if _cse_temp_24 {
    return Ok({ let mut map: HashMap<String, DepylerValue>= HashMap::new();
    map.insert(STR_ZERO.to_string(), DepylerValue::Bool(true));
    map.insert(STR_DOMAINPADFRACTION.to_string(), DepylerValue::Int(0 as i64));
    map.insert(STR_ZEROCLASS.to_string(), DepylerValue::Str(zero_class.to_string()));
    map.insert(STR_FORCED.to_string(), DepylerValue::Bool(true));
    map.insert(STR_UNCERTAIN.to_string(), DepylerValue::Bool(false));
    map });
   
}
let proximity = if data_max>0 {
   (data_min).join(data_max)
}
else {
    0 };
    let _cse_temp_25  = ((proximity as f64) as f64)<0.3;
    if _cse_temp_25 {
    return Ok({ let mut map: HashMap<String, DepylerValue>= HashMap::new();
    map.insert(STR_ZERO.to_string(), DepylerValue::Bool(true));
    map.insert(STR_DOMAINPADFRACTION.to_string(), DepylerValue::Int(0 as i64));
    map.insert(STR_ZEROCLASS.to_string(), DepylerValue::Str(zero_class.to_string()));
    map.insert(STR_FORCED.to_string(), DepylerValue::Bool(false));
    map.insert(STR_UNCERTAIN.to_string(), DepylerValue::Bool(false));
    map });
   
}
if is_bar_like {
    return Ok({ let mut map: HashMap<String, DepylerValue>= HashMap::new();
    map.insert(STR_ZERO.to_string(), DepylerValue::Bool(true));
    map.insert(STR_DOMAINPADFRACTION.to_string(), DepylerValue::Int(0 as i64));
    map.insert(STR_ZEROCLASS.to_string(), DepylerValue::Str(zero_class.to_string()));
    map.insert(STR_FORCED.to_string(), DepylerValue::Bool(true));
    map.insert(STR_UNCERTAIN.to_string(), DepylerValue::Bool(false));
    map });
   
}
return Ok({ let mut map: HashMap<String, DepylerValue>= HashMap::new();
    map.insert(STR_ZERO.to_string(), DepylerValue::Bool(false));
    map.insert(STR_DOMAINPADFRACTION.to_string(), DepylerValue::Float(0.05 as f64));
    map.insert(STR_ZEROCLASS.to_string(), DepylerValue::Str(zero_class.to_string()));
    map.insert(STR_FORCED.to_string(), DepylerValue::Bool(false));
    map.insert(STR_UNCERTAIN.to_string(), DepylerValue::Bool(false));
    map });
   
}
let _cse_temp_26  = (is_bar_like) &&(is_positional);
    if _cse_temp_26 {
    return Ok({ let mut map: HashMap<String, DepylerValue>= HashMap::new();
    map.insert(STR_ZERO.to_string(), DepylerValue::Bool(true));
    map.insert(STR_DOMAINPADFRACTION.to_string(), DepylerValue::Int(0 as i64));
    map.insert(STR_ZEROCLASS.to_string(), DepylerValue::Str(STR_UNKNOWN_1.to_string()));
    map.insert(STR_FORCED.to_string(), DepylerValue::Bool(true));
    map.insert(STR_UNCERTAIN.to_string(), DepylerValue::Bool(false));
    map });
   
}
Ok({ let mut map: HashMap<String, DepylerValue>= HashMap::new();
    map.insert(STR_ZERO.to_string(), DepylerValue::Bool(false));
    map.insert(STR_DOMAINPADFRACTION.to_string(), DepylerValue::Float(0.05 as f64));
    map.insert(STR_ZEROCLASS.to_string(), DepylerValue::Str(STR_UNKNOWN_1.to_string()));
    map.insert(STR_FORCED.to_string(), DepylerValue::Bool(true));
    map.insert(STR_UNCERTAIN.to_string(), DepylerValue::Bool(false));
    map })
}
#[doc = " Depyler: verified panic-free"] #[doc = " Depyler: proven to terminate"] pub fn compute_padded_domain (values: & Vec<f64>, pad_fraction: f64) -> Option<Vec<f64>>{
    let _cse_temp_0 = pad_fraction <= 0f64;
    let _cse_temp_1 = values.len() as i32;
    let _cse_temp_2 = _cse_temp_1<2;
    let _cse_temp_3  = (_cse_temp_0) ||(_cse_temp_2);
    if _cse_temp_3 {
    return None;
   
}
let _cse_temp_4 = * values.iter().min ().expect("empty collection");
    let data_min = _cse_temp_4;
    let _cse_temp_5 = * values.iter().max().expect("empty collection");
    let data_max = _cse_temp_5;
    let span  = ((data_max) - (data_min)) as i32;
    let _cse_temp_6 = span <= 0;
    if _cse_temp_6 {
    return None;
   
}
let _cse_temp_7  = (span).py_mul(pad_fraction);
    let padding = _cse_temp_7;
    Some(vec! [(data_min) - (padding) ,(data_max).py_add(padding)])
}
pub fn _pick_scheme<'a, 'b>(schemes: & 'a Vec<String>, name: & 'b str) -> Result<String, IndexError>{
    let mut h: i32 = Default::default();
    h = 0;
    for ch in name.chars() {
    let code = ch as u32 as i32;
    h  = ((((h <<5) - (h) as i32)).py_add(code)) as i32;
    h = h & - 1;
    if (h & i32::MIN) != 0 {
    h  = ((h) - (4294967296i64)) as i32;
   
}
} Ok({ let base = & schemes;
    let idx: i32  = ((h).abs()).py_mod(schemes.len() as i32);
    let actual_idx = if idx<0 {
    base.len().saturating_sub(idx.abs() as usize)
}
else {
    idx as usize };
    base.get(actual_idx).cloned().expect("IndexError: list index out of range") })
}
#[doc = " Depyler: proven to terminate"] pub fn get_recommended_color_scheme<'b, 'l1, 'a, 'c>(semantic_type: & 'a Option<String>, encoding_type: & 'b str, unique_value_count: i32, field_name: & 'c str, _values: Option<Vec<DepylerValue>>, color_hint: & 'l1 Option<std::collections::HashMap<String, DepylerValue>>) -> Result<std::collections::HashMap<String, DepylerValue>, IndexError>{
    if false {
    let _cse_temp_0 = encoding_type == STR_QUANTITATIVE;
    if _cse_temp_0 {
    return Ok({ let mut map: HashMap<String, DepylerValue>= HashMap::new();
    map.insert(STR_SCHEME.to_string(), DepylerValue::Str(STR_VIRIDIS.to_string()));
    map.insert(STR_TYPE.to_string(), DepylerValue::Str(STR_SEQUENTIAL.to_string()));
    map.insert(STR_REASON.to_string(), DepylerValue::Str("default for quantitative".to_string().to_string()));
    map });
   
}
let _cse_temp_1 = encoding_type == STR_ORDINAL;
    if _cse_temp_1 {
    return Ok({ let mut map: HashMap<String, DepylerValue>= HashMap::new();
    map.insert(STR_SCHEME.to_string(), DepylerValue::Str(STR_BLUES.to_string()));
    map.insert(STR_TYPE.to_string(), DepylerValue::Str(STR_SEQUENTIAL.to_string()));
    map.insert(STR_REASON.to_string(), DepylerValue::Str("default for ordinal".to_string().to_string()));
    map });
   
}
return Ok({ let mut map: HashMap<String, DepylerValue>= HashMap::new();
    map.insert(STR_SCHEME.to_string(), DepylerValue::Str(format!("{:?}", if unique_value_count>10 {
    "tableau20".to_string()
}
else {
    "tableau10" })));
    map.insert(STR_TYPE.to_string(), DepylerValue::Str(STR_CATEGORICAL.to_string()));
    map.insert(STR_REASON.to_string(), DepylerValue::Str("default for categorical".to_string().to_string()));
    map });
   
}
let _cse_temp_2 = semantic_type.unwrap_or_default() == "Temperature";
    if _cse_temp_2 {
    let _cse_temp_3 = color_hint.get("type").cloned() == STR_DIVERGING;
    let _cse_temp_4  = (color_hint.is_some()) &&(_cse_temp_3);
    if let Some(ref _cse_temp_4_val) = _cse_temp_4 {
    return Ok({ let mut map: HashMap<String, DepylerValue>= HashMap::new();
    map.insert(STR_SCHEME.to_string(), DepylerValue::Str(STR_REDBLUE.to_string()));
    map.insert(STR_TYPE.to_string(), DepylerValue::Str(STR_DIVERGING.to_string()));
    map.insert(STR_REASON.to_string(), DepylerValue::Str("temperature diverging around freezing point".to_string().to_string()));
    map });
   
}
return Ok({ let mut map: HashMap<String, DepylerValue>= HashMap::new();
    map.insert(STR_SCHEME.to_string(), DepylerValue::Str("reds".to_string().to_string()));
    map.insert(STR_TYPE.to_string(), DepylerValue::Str(STR_SEQUENTIAL.to_string()));
    map.insert(STR_REASON.to_string(), DepylerValue::Str("temperature single-direction uses sequential".to_string().to_string()));
    map });
   
}
let _cse_temp_5 = semantic_type.unwrap_or_default() == "Percentage";
    if _cse_temp_5 {
    let _cse_temp_6 = color_hint.get("type").cloned() == STR_DIVERGING;
    let _cse_temp_7  = (color_hint.is_some()) &&(_cse_temp_6);
    if let Some(ref _cse_temp_7_val) = _cse_temp_7 {
    return Ok({ let mut map: HashMap<String, DepylerValue>= HashMap::new();
    map.insert(STR_SCHEME.to_string(), DepylerValue::Str(STR_REDBLUE.to_string()));
    map.insert(STR_TYPE.to_string(), DepylerValue::Str(STR_DIVERGING.to_string()));
    map.insert(STR_REASON.to_string(), DepylerValue::Str("percentage spans positive and negative".to_string().to_string()));
    map });
   
}
return Ok({ let mut map: HashMap<String, DepylerValue>= HashMap::new();
    map.insert(STR_SCHEME.to_string(), DepylerValue::Str("oranges".to_string().to_string()));
    map.insert(STR_TYPE.to_string(), DepylerValue::Str(STR_SEQUENTIAL.to_string()));
    map.insert(STR_REASON.to_string(), DepylerValue::Str("percentage all same sign uses sequential".to_string().to_string()));
    map });
   
}
let _cse_temp_8 = ["Price", "Amount"].contains(&& semantic_type);
    if _cse_temp_8 {
    let _cse_temp_9 = color_hint.get("type").cloned() == STR_DIVERGING;
    let _cse_temp_10  = (color_hint.is_some()) &&(_cse_temp_9);
    if let Some(ref _cse_temp_10_val) = _cse_temp_10 {
    return Ok({ let mut map: HashMap<String, DepylerValue>= HashMap::new();
    map.insert(STR_SCHEME.to_string(), DepylerValue::Str(STR_REDBLUE.to_string()));
    map.insert(STR_TYPE.to_string(), DepylerValue::Str(STR_DIVERGING.to_string()));
    map.insert(STR_REASON.to_string(), DepylerValue::Str("financial data spans positive and negative".to_string().to_string()));
    map });
   
}
return Ok({ let mut map: HashMap<String, DepylerValue>= HashMap::new();
    map.insert(STR_SCHEME.to_string(), DepylerValue::Str("goldgreen".to_string().to_string()));
    map.insert(STR_TYPE.to_string(), DepylerValue::Str(STR_SEQUENTIAL.to_string()));
    map.insert(STR_REASON.to_string(), DepylerValue::Str("financial data uses gold-green".to_string().to_string()));
    map });
   
}
let _cse_temp_11 = semantic_type.unwrap_or_default() == "Score";
    if _cse_temp_11 {
    let _cse_temp_12 = color_hint.get("type").cloned() == STR_DIVERGING;
    let _cse_temp_13  = (color_hint.is_some()) &&(_cse_temp_12);
    if let Some(ref _cse_temp_13_val) = _cse_temp_13 {
    return Ok({ let mut map: HashMap<String, DepylerValue>= HashMap::new();
    map.insert(STR_SCHEME.to_string(), DepylerValue::Str(STR_REDBLUE.to_string()));
    map.insert(STR_TYPE.to_string(), DepylerValue::Str(STR_DIVERGING.to_string()));
    map.insert(STR_REASON.to_string(), DepylerValue::Str("score/rating diverging around midpoint".to_string().to_string()));
    map });
   
}
return Ok({ let mut map: HashMap<String, DepylerValue>= HashMap::new();
    map.insert(STR_SCHEME.to_string(), DepylerValue::Str("yelloworangebrown".to_string().to_string()));
    map.insert(STR_TYPE.to_string(), DepylerValue::Str(STR_SEQUENTIAL.to_string()));
    map.insert(STR_REASON.to_string(), DepylerValue::Str("scores use warm sequential".to_string().to_string()));
    map });
   
}
let _cse_temp_14 = semantic_type.unwrap_or_default() == STR_RANK;
    if _cse_temp_14 {
    return Ok({ let mut map: HashMap<String, DepylerValue>= HashMap::new();
    map.insert(STR_SCHEME.to_string(), DepylerValue::Str("purples".to_string().to_string()));
    map.insert(STR_TYPE.to_string(), DepylerValue::Str(STR_SEQUENTIAL.to_string()));
    map.insert(STR_REASON.to_string(), DepylerValue::Str("ranks use single-hue sequential".to_string().to_string()));
    map });
   
}
let _cse_temp_15 = semantic_type.unwrap_or_default() == "Range";
    if _cse_temp_15 {
    return Ok({ let mut map: HashMap<String, DepylerValue>= HashMap::new();
    map.insert(STR_SCHEME.to_string(), DepylerValue::Str(STR_BLUES.to_string()));
    map.insert(STR_TYPE.to_string(), DepylerValue::Str(STR_SEQUENTIAL.to_string()));
    map.insert(STR_REASON.to_string(), DepylerValue::Str("range groups use sequential".to_string().to_string()));
    map });
   
}
let _cse_temp_16 = ordinalTypes.contains(& semantic_type);
    let _cse_temp_17  = (_cse_temp_16) &&(_cse_temp_8);
    if _cse_temp_17 {
    return Ok({ let mut map: HashMap<String, DepylerValue>= HashMap::new();
    map.insert(STR_SCHEME.to_string(), DepylerValue::Str(STR_VIRIDIS.to_string()));
    map.insert(STR_TYPE.to_string(), DepylerValue::Str(STR_SEQUENTIAL.to_string()));
    map.insert(STR_REASON.to_string(), DepylerValue::Str("temporal granules use perceptually uniform".to_string().to_string()));
    map });
   
}
let _cse_temp_18 = get_registry_entry(if semantic_type.is_empty() {
    STR_EMPTY.to_string()
}
else {
    semantic_type.to_string() }).get("t1").cloned().unwrap_or_default().into() == "GeoPlace";
    if _cse_temp_18 {
    let _cse_temp_19 = unique_value_count <= 10;
    if _cse_temp_19 {
    return Ok({ let mut map: HashMap<String, DepylerValue>= HashMap::new();
    map.insert(STR_SCHEME.to_string(), DepylerValue::Str("set2".to_string().to_string()));
    map.insert(STR_TYPE.to_string(), DepylerValue::Str(STR_CATEGORICAL.to_string()));
    map.insert(STR_REASON.to_string(), DepylerValue::Str("geographic regions use distinct pastels".to_string().to_string()));
    map });
   
}
return Ok({ let mut map: HashMap<String, DepylerValue>= HashMap::new();
    map.insert(STR_SCHEME.to_string(), DepylerValue::Str("tableau20".to_string().to_string()));
    map.insert(STR_TYPE.to_string(), DepylerValue::Str(STR_CATEGORICAL.to_string()));
    map.insert(STR_REASON.to_string(), DepylerValue::Str("many regions use large categorical".to_string().to_string()));
    map });
   
}
if _cse_temp_8 {
    return Ok({ let mut map: HashMap<String, DepylerValue>= HashMap::new();
    map.insert(STR_SCHEME.to_string(), DepylerValue::Str("set1".to_string().to_string()));
    map.insert(STR_TYPE.to_string(), DepylerValue::Str(STR_CATEGORICAL.to_string()));
    map.insert(STR_REASON.to_string(), DepylerValue::Str("status uses high-contrast categorical".to_string().to_string()));
    map });
   
}
let _cse_temp_20 = semantic_type.unwrap_or_default() == "Category";
    if _cse_temp_20 {
    return Ok({ let mut map: HashMap<String, DepylerValue>= HashMap::new();
    map.insert(STR_SCHEME.to_string(), DepylerValue::Str(format!("{:?}", if unique_value_count>10 {
    "tableau20".to_string()
}
else {
    "tableau10" })));
    map.insert(STR_TYPE.to_string(), DepylerValue::Str(STR_CATEGORICAL.to_string()));
    map.insert(STR_REASON.to_string(), DepylerValue::Str("categories use standard categorical".to_string().to_string()));
    map });
   
}
let _cse_temp_21 = semantic_type.unwrap_or_default() == "Name";
    if _cse_temp_21 {
    return Ok({ let mut map: HashMap<String, DepylerValue>= HashMap::new();
    map.insert(STR_SCHEME.to_string(), DepylerValue::Str(format!("{:?}", if unique_value_count>8 {
    "tableau20".to_string()
}
else {
    "set2".to_string() })));
    map.insert(STR_TYPE.to_string(), DepylerValue::Str(STR_CATEGORICAL.to_string()));
    map.insert(STR_REASON.to_string(), DepylerValue::Str("names use readable categorical".to_string().to_string()));
    map });
   
}
let _cse_temp_22 = semantic_type.unwrap_or_default() == "Duration";
    if _cse_temp_22 {
    return Ok({ let mut map: HashMap<String, DepylerValue>= HashMap::new();
    map.insert(STR_SCHEME.to_string(), DepylerValue::Str("oranges".to_string().to_string()));
    map.insert(STR_TYPE.to_string(), DepylerValue::Str(STR_SEQUENTIAL.to_string()));
    map.insert(STR_REASON.to_string(), DepylerValue::Str("duration uses intensity-based sequential".to_string().to_string()));
    map });
   
}
let _cse_temp_23 = measureTypes.contains(& semantic_type);
    if _cse_temp_23 {
    let _cse_temp_24 = color_hint.get("type").cloned() == STR_DIVERGING;
    let _cse_temp_25  = (color_hint.is_some()) &&(_cse_temp_24);
    if let Some(ref _cse_temp_25_val) = _cse_temp_25 {
    return Ok({ let mut map: HashMap<String, DepylerValue>= HashMap::new();
    map.insert(STR_SCHEME.to_string(), DepylerValue::Str(STR_REDBLUE.to_string()));
    map.insert(STR_TYPE.to_string(), DepylerValue::Str(STR_DIVERGING.to_string()));
    map.insert(STR_REASON.to_string(), DepylerValue::Str("measure with diverging nature".to_string().to_string()));
    map });
   
}
let sequential_schemes: Vec<String>= vec! [STR_VIRIDIS.to_string(), STR_BLUES.to_string(), "greens".to_string(), "reds".to_string().to_string(), "yelloworangebrown".to_string().to_string(), "goldgreen".to_string().to_string()];
    return Ok({ let mut map: HashMap<String, DepylerValue>= HashMap::new();
    map.insert(STR_SCHEME.to_string(), DepylerValue::Str(format!("{:?}", _pick_scheme(& sequential_schemes, & field_name))));
    map.insert(STR_TYPE.to_string(), DepylerValue::Str(STR_SEQUENTIAL.to_string()));
    map.insert(STR_REASON.to_string(), DepylerValue::Str("measures use perceptually uniform sequential".to_string().to_string()));
    map });
   
}
let _cse_temp_26 = encoding_type == STR_ORDINAL;
    let _cse_temp_27  = (_cse_temp_16) ||(_cse_temp_26);
    if _cse_temp_27 {
    let ordinal_schemes: Vec<String>= vec! [STR_BLUES.to_string(), "greens".to_string(), "purples".to_string().to_string(), "oranges".to_string().to_string()];
    return Ok({ let mut map: HashMap<String, DepylerValue>= HashMap::new();
    map.insert(STR_SCHEME.to_string(), DepylerValue::Str(format!("{:?}", _pick_scheme(& ordinal_schemes, & field_name))));
    map.insert(STR_TYPE.to_string(), DepylerValue::Str(STR_SEQUENTIAL.to_string()));
    map.insert(STR_REASON.to_string(), DepylerValue::Str("ordinal data uses sequential scheme".to_string().to_string()));
    map });
   
}
let _cse_temp_28 = [STR_NOMINAL, STR_TEMPORAL].contains(&encoding_type);
    if _cse_temp_28 {
    return Ok({ let mut map: HashMap<String, DepylerValue>= HashMap::new();
    map.insert(STR_SCHEME.to_string(), DepylerValue::Str(format!("{:?}", if unique_value_count>10 {
    "tableau20".to_string()
}
else {
    "tableau10" })));
    map.insert(STR_TYPE.to_string(), DepylerValue::Str(STR_CATEGORICAL.to_string()));
    map.insert(STR_REASON.to_string(), DepylerValue::Str("default categorical palette".to_string().to_string()));
    map });
   
}
Ok({ let mut map: HashMap<String, DepylerValue>= HashMap::new();
    map.insert(STR_SCHEME.to_string(), DepylerValue::Str(STR_VIRIDIS.to_string()));
    map.insert(STR_TYPE.to_string(), DepylerValue::Str(STR_SEQUENTIAL.to_string()));
    map.insert(STR_REASON.to_string(), DepylerValue::Str("universal fallback".to_string().to_string()));
    map })
}
#[doc = " Depyler: proven to terminate"] pub fn _build_lookup(seq: & std::collections::HashMap<String, DepylerValue>) -> Result<std::collections::HashMap<String, i32>, IndexError>{
    let mut m: std::collections::HashMap<String, i32>= {
    let mut map: HashMap<String, i32>= HashMap::new();
    map };
    let labels = seq.get("labels").cloned().unwrap_or_default();
    let ci = seq.get("caseInsensitive").cloned().unwrap_or_default();
    for(i, label) in labels.iter().cloned().enumerate().map(|(i, x) |(i as i32, x)) {
    let i = i as i32;
    let key = if ci {
    label.to_lowercase()
}
else {
    label };
    m.insert(key.to_string().clone(), i);
   
}
Ok(m)
}
pub fn _match_sequence<'b, 'a>(values: & 'a Vec<DepylerValue>, sequences: & 'b Vec<std::collections::HashMap<String, DepylerValue>>) -> Result<Option<Vec<String>>, IndexError>{
    let mut seen: std::collections::HashMap<String ,()>= {
    let mut map: HashMap<String ,()>= HashMap::new();
    map };
    for v in values.iter().cloned() {
    let mut s = if v.is_some() {
   (v).to_string()
}
else {
    STR_EMPTY };
    if(! s.is_empty()) &&(seen.get(& s).is_none()) {
    seen.insert(s.to_string().clone(), None);
   
}
} let unique_values = seen.keys().cloned().collect::<Vec<_>>();
    if unique_values.is_empty() {
    return Ok(None);
   
}
for seq in sequences.iter().cloned() {
    let lookup = _build_lookup(& seq) ?;
    let ci = seq.get("caseInsensitive").cloned().unwrap_or_default();
    let mut matched: Vec <(String, i32)>= vec! [];
    let mut unmatched: Vec<String>= vec! [];
    for val in unique_values.iter().cloned() {
    let key = if ci {
    val.to_lowercase()
}
else {
    val };
    let idx = lookup.get(& key).cloned();
    if idx.is_some() {
    matched.append((val, idx));
   
}
else {
    unmatched.append(val);
   
}
} if((matched.len() as i32 as f64)>= (unique_values.len() as i32).py_mul(0.6)) &&(matched.len() as i32>= 2) {
    matched.sort_by_key(| x | move | p: i32 | p.clone().py_index((1 as i64) as i64).unwrap_or_default()(x));
    let mut result: Vec <(String, i32)>= matched.iter().cloned().map(|(v, _) | v).collect::<Vec<_>>();
    result.extend(unmatched.iter().cloned());
    return Ok(Some(result));
   
}
} Ok(None)
}
#[doc = " Depyler: verified panic-free"] #[doc = " Depyler: proven to terminate"] pub fn infer_ordinal_sort_order<'b, 'a>(semantic_type: & 'a str, values: & 'b Vec<DepylerValue>) -> Result<Option<Vec<String>>, Box<dyn std::error::Error>>{
    let sequences = _ORDINAL_SEQUENCES.get(semantic_type).cloned();
    if let Some(ref sequences_val) = sequences {
    return Ok(Some(_match_sequence(& values, sequences_val)));
   
}
let _cse_temp_0 = ["Category", STR_UNKNOWN_2].contains(&semantic_type);
    let _cse_temp_1  = (semantic_type.is_empty()) ||(_cse_temp_0);
    if _cse_temp_1 {
    for seqs in _ORDINAL_SEQUENCES.values() {
    let mut result = _match_sequence(& values, seqs) ?;
    if let Some(ref result_val) = result {
    return Ok(result_val);
   
}
}
}
Ok(None)
}
#[doc = " Depyler: verified panic-free"] #[doc = " Depyler: proven to terminate"] pub fn _vis_category_to_vl_type(vc: & str) -> String {
    let _cse_temp_0 = vc == STR_QUANTITATIVE;
    if _cse_temp_0 {
    return STR_QUANTITATIVE.to_string();
   
}
let _cse_temp_1 = vc == STR_ORDINAL;
    if _cse_temp_1 {
    return STR_ORDINAL.to_string();
   
}
let _cse_temp_2 = vc == STR_TEMPORAL;
    if _cse_temp_2 {
    return STR_TEMPORAL.to_string();
   
}
let _cse_temp_3 = vc == STR_GEOGRAPHIC;
    if _cse_temp_3 {
    return STR_QUANTITATIVE.to_string();
   
}
STR_NOMINAL.to_string()
}
#[doc = " Depyler: verified panic-free"] #[doc = " Depyler: proven to terminate"] pub fn _looks_temporal_value(val: & DepylerValue) -> Result<bool, Box<dyn std::error::Error>>{
    if true {
    return Ok(false);
   
}
if true {
    let _cse_temp_0 = 1500 <= val;
    let _cse_temp_1 = val <= 2200;
    let _cse_temp_2  = (_cse_temp_0) &&(_cse_temp_1);
    let _cse_temp_3  = (val) as i32;
    let _cse_temp_4 = val == _cse_temp_3;
    let _cse_temp_5  = (_cse_temp_2) &&(_cse_temp_4);
    if _cse_temp_5 {
    return Ok(true);
   
}
let _cse_temp_6 = 86400000<val;
    let _cse_temp_7 = val<4200000000000i64;
    let _cse_temp_8  = (_cse_temp_6) &&(_cse_temp_7);
    if _cse_temp_8 {
    return Ok(true);
   
}
return Ok(false);
   
}
if true {
    let trimmed = val.to_string().trim().to_string();
    if trimmed.is_empty() {
    return Ok(false);
   
}
return is_js_parseable(& trimmed);
   
}
Ok(false)
}
#[doc = " Depyler: proven to terminate"] pub fn _validate_temporal_parsing<'a, 'b>(data: & 'a Vec<DepylerValue>, field_name: & 'b str, from_registry: bool) -> Result<bool, ZeroDivisionError>{
    let mut sample: Vec<DepylerValue>= {
    let base = & * data;
    let stop_idx  = (15) as isize;
    let stop = if stop_idx<0 {
   (base.len() as isize + stop_idx).max(0) as usize
}
else {
    stop_idx as usize };
    base [..stop.min (base.len())].to_vec()
}
. into_iter().map(| r | r.get(field_name).cloned()).collect::<Vec<_>>();
    sample = sample.iter().cloned().filter(| v | {
    let v = v.clone();
    ! v.is_null() }).map(| v | v).collect::<Vec<_>>();
    if sample.is_empty() {
    return Ok(false);
   
}
let unique: std::collections::HashSet<DepylerValue>= sample.iter().cloned().map(| v |(v).to_string()).collect::<std::collections::HashSet<_>>();
    let _cse_temp_0 = unique.len() as i32;
    let _cse_temp_1 = _cse_temp_0 <= 1;
    if _cse_temp_1 {
    return Ok(false);
   
}
let _cse_temp_2 = sample.iter().cloned().filter(| & v | _looks_temporal_value(v)).map(| v | 1).sum::<i32>();
    let passing = _cse_temp_2;
    let threshold = if from_registry {
    0.3
}
else {
    0.5 };
    Ok(((passing).join(sample.len() as i32) as f64)>= threshold)
}
#[doc = " Depyler: verified panic-free"] #[doc = " Depyler: proven to terminate"] pub fn _resolve_temporal_encoding<'b, 'a, 'l1, 'c>(vis_category: & 'a str, channel: & 'b str, data: & 'c Vec<DepylerValue>, field_name: & 'l1 str, from_registry: bool) -> Result<std::collections::HashMap<String, DepylerValue>, Box<dyn std::error::Error>>{
    let _cse_temp_0 = ["size", STR_COLUMN, STR_ROW].contains(&channel);
    if _cse_temp_0 {
    return Ok({ let mut map: HashMap<String, DepylerValue>= HashMap::new();
    map.insert(STR_VLTYPE.to_string(), DepylerValue::Str(STR_ORDINAL.to_string()));
    map.insert(STR_VISCATEGORY.to_string(), DepylerValue::Str(vis_category.to_string()));
    map.insert(STR_CHANNELOVERRIDE.to_string(), DepylerValue::Bool(true));
    map.insert(STR_CARDINALITYGUARD.to_string(), DepylerValue::Bool(false));
    map });
   
}
let _cse_temp_1 = channel == STR_COLOR;
    if _cse_temp_1 {
    let _cse_temp_2 = data.iter().cloned().map(| r | r.get(field_name).cloned()).collect::<std::collections::HashSet<_>>().len() as i32;
    let unique_count = _cse_temp_2;
    let _cse_temp_3 = unique_count <= 12;
    if _cse_temp_3 {
    return Ok({ let mut map: HashMap<String, DepylerValue>= HashMap::new();
    map.insert(STR_VLTYPE.to_string(), DepylerValue::Str(STR_ORDINAL.to_string()));
    map.insert(STR_VISCATEGORY.to_string(), DepylerValue::Str(vis_category.to_string()));
    map.insert(STR_CHANNELOVERRIDE.to_string(), DepylerValue::Bool(true));
    map.insert(STR_CARDINALITYGUARD.to_string(), DepylerValue::Bool(false));
    map });
   
}
} if ! _validate_temporal_parsing(& data, & field_name, from_registry) ? {
    return Ok({ let mut map: HashMap<String, DepylerValue>= HashMap::new();
    map.insert(STR_VLTYPE.to_string(), DepylerValue::Str(STR_ORDINAL.to_string()));
    map.insert(STR_VISCATEGORY.to_string(), DepylerValue::Str(vis_category.to_string()));
    map.insert(STR_CHANNELOVERRIDE.to_string(), DepylerValue::Bool(false));
    map.insert(STR_CARDINALITYGUARD.to_string(), DepylerValue::Bool(false));
    map });
   
}
Ok({ let mut map: HashMap<String, DepylerValue>= HashMap::new();
    map.insert(STR_VLTYPE.to_string(), DepylerValue::Str(STR_TEMPORAL.to_string()));
    map.insert(STR_VISCATEGORY.to_string(), DepylerValue::Str(vis_category.to_string()));
    map.insert(STR_CHANNELOVERRIDE.to_string(), DepylerValue::Bool(false));
    map.insert(STR_CARDINALITYGUARD.to_string(), DepylerValue::Bool(false));
    map })
}
#[doc = " Depyler: verified panic-free"] pub fn _apply_ordinal_guards<'a, 'b, 'c>(vis_category: & 'a str, channel: & 'b str, _data: Vec<DepylerValue>, _field_name: String, field_values: & 'c Vec<DepylerValue>, from_registry: bool) -> std::collections::HashMap<String, DepylerValue>{
    let mut numeric_vals: Vec<f64>= vec! [];
    for v in field_values.iter().cloned() {
    if false {
    continue;
   
}
if true {
    continue;
   
}
if true {
    if !(true) &&((v as f64).is_nan()) {
    numeric_vals.append((v) as f64);
   
}
} else {
    if true {
    match(|| -> Result <(), Box<dyn std::error::Error>>{
    numeric_vals.append((v) as f64);
    Ok(()) })() {
    Ok(()) =>{
   
}
Err(_e) =>{
   
}
}
}
}
}
if ! numeric_vals.is_empty() {
    let _cse_temp_0 = numeric_vals.iter().cloned().collect::<std::collections::HashSet<_>>().len() as i32;
    let unique_count = _cse_temp_0;
    let has_fractions = numeric_vals.as_slice().iter().cloned().map(| v |(v).py_mod(1i32) != 0).any(| x | x);
    let _cse_temp_1  = (! from_registry) &&(has_fractions);
    let _cse_temp_2 = unique_count>20;
    let _cse_temp_3  = (_cse_temp_1) &&(_cse_temp_2);
    if _cse_temp_3 {
    return {
    let mut map: HashMap<String, DepylerValue>= HashMap::new();
    map.insert(STR_VLTYPE.to_string(), DepylerValue::Str(STR_QUANTITATIVE.to_string()));
    map.insert(STR_VISCATEGORY.to_string(), DepylerValue::Str(vis_category.to_string()));
    map.insert(STR_CHANNELOVERRIDE.to_string(), DepylerValue::Bool(false));
    map.insert(STR_CARDINALITYGUARD.to_string(), DepylerValue::Bool(true));
    map };
   
}
let _cse_temp_4 = unique_count>12;
    let _cse_temp_5  = (! has_fractions) &&(_cse_temp_4);
    let _cse_temp_6 = [STR_COLOR, STR_GROUP].contains(&channel);
    let _cse_temp_7  = (_cse_temp_5) &&(_cse_temp_6);
    if _cse_temp_7 {
    return {
    let mut map: HashMap<String, DepylerValue>= HashMap::new();
    map.insert(STR_VLTYPE.to_string(), DepylerValue::Str(STR_QUANTITATIVE.to_string()));
    map.insert(STR_VISCATEGORY.to_string(), DepylerValue::Str(vis_category.to_string()));
    map.insert(STR_CHANNELOVERRIDE.to_string(), DepylerValue::Bool(true));
    map.insert(STR_CARDINALITYGUARD.to_string(), DepylerValue::Bool(true));
    map };
   
}
if _cse_temp_7 {
    return {
    let mut map: HashMap<String, DepylerValue>= HashMap::new();
    map.insert(STR_VLTYPE.to_string(), DepylerValue::Str(STR_QUANTITATIVE.to_string()));
    map.insert(STR_VISCATEGORY.to_string(), DepylerValue::Str(vis_category.to_string()));
    map.insert(STR_CHANNELOVERRIDE.to_string(), DepylerValue::Bool(true));
    map.insert(STR_CARDINALITYGUARD.to_string(), DepylerValue::Bool(true));
    map };
   
}
} {
    let mut map: HashMap<String, DepylerValue>= HashMap::new();
    map.insert(STR_VLTYPE.to_string(), DepylerValue::Str(STR_ORDINAL.to_string()));
    map.insert(STR_VISCATEGORY.to_string(), DepylerValue::Str(vis_category.to_string()));
    map.insert(STR_CHANNELOVERRIDE.to_string(), DepylerValue::Bool(false));
    map.insert(STR_CARDINALITYGUARD.to_string(), DepylerValue::Bool(false));
    map
}
} #[doc = " Depyler: proven to terminate"] pub fn _disambiguate_multi_encoding<'a, 'c, 'b, 'l1>(candidates: & 'a Vec<String>, channel: & 'b str, data: & 'c Vec<DepylerValue>, field_name: & 'l1 str, _field_values: Vec<DepylerValue>) -> Result<std::collections::HashMap<String, DepylerValue>, IndexError>{
    let has = {
    let candidates_capture = candidates.clone();
    move | vc: i32 | candidates_capture.get(& vc).is_some() };
    let _cse_temp_0  = (has(STR_TEMPORAL)) &&(has(STR_ORDINAL));
    if _cse_temp_0 {
    return _resolve_temporal_encoding(STR_TEMPORAL, & channel, & data, & field_name, true);
   
}
let _cse_temp_1  = (has(STR_QUANTITATIVE)) &&(has(STR_ORDINAL));
    if _cse_temp_1 {
    let _cse_temp_2 = [STR_COLOR, STR_GROUP].contains(&channel);
    if _cse_temp_2 {
    let _cse_temp_3 = data.iter().cloned().map(| r | r.get(field_name).cloned()).collect::<std::collections::HashSet<_>>().len() as i32;
    let unique_count = _cse_temp_3;
    let _cse_temp_4 = unique_count <= 12;
    if _cse_temp_4 {
    return Ok({ let mut map: HashMap<String, DepylerValue>= HashMap::new();
    map.insert(STR_VLTYPE.to_string(), DepylerValue::Str(STR_ORDINAL.to_string()));
    map.insert(STR_VISCATEGORY.to_string(), DepylerValue::Str(STR_ORDINAL.to_string()));
    map.insert(STR_CHANNELOVERRIDE.to_string(), DepylerValue::Bool(false));
    map.insert(STR_CARDINALITYGUARD.to_string(), DepylerValue::Bool(false));
    map });
   
}
return Ok({ let mut map: HashMap<String, DepylerValue>= HashMap::new();
    map.insert(STR_VLTYPE.to_string(), DepylerValue::Str(STR_QUANTITATIVE.to_string()));
    map.insert(STR_VISCATEGORY.to_string(), DepylerValue::Str(STR_QUANTITATIVE.to_string()));
    map.insert(STR_CHANNELOVERRIDE.to_string(), DepylerValue::Bool(false));
    map.insert(STR_CARDINALITYGUARD.to_string(), DepylerValue::Bool(true));
    map });
   
}
if _cse_temp_2 {
    return Ok({ let mut map: HashMap<String, DepylerValue>= HashMap::new();
    map.insert(STR_VLTYPE.to_string(), DepylerValue::Str(STR_ORDINAL.to_string()));
    map.insert(STR_VISCATEGORY.to_string(), DepylerValue::Str(STR_ORDINAL.to_string()));
    map.insert(STR_CHANNELOVERRIDE.to_string(), DepylerValue::Bool(false));
    map.insert(STR_CARDINALITYGUARD.to_string(), DepylerValue::Bool(false));
    map });
   
}
return Ok({ let mut map: HashMap<String, DepylerValue>= HashMap::new();
    map.insert(STR_VLTYPE.to_string(), DepylerValue::Str(STR_QUANTITATIVE.to_string()));
    map.insert(STR_VISCATEGORY.to_string(), DepylerValue::Str(STR_QUANTITATIVE.to_string()));
    map.insert(STR_CHANNELOVERRIDE.to_string(), DepylerValue::Bool(false));
    map.insert(STR_CARDINALITYGUARD.to_string(), DepylerValue::Bool(false));
    map });
   
}
let _cse_temp_5  = (has(STR_QUANTITATIVE)) &&(has(STR_GEOGRAPHIC));
    if _cse_temp_5 {
    return Ok({ let mut map: HashMap<String, DepylerValue>= HashMap::new();
    map.insert(STR_VLTYPE.to_string(), DepylerValue::Str(STR_QUANTITATIVE.to_string()));
    map.insert(STR_VISCATEGORY.to_string(), DepylerValue::Str(STR_QUANTITATIVE.to_string()));
    map.insert(STR_CHANNELOVERRIDE.to_string(), DepylerValue::Bool(false));
    map.insert(STR_CARDINALITYGUARD.to_string(), DepylerValue::Bool(false));
    map });
   
}
let _cse_temp_6  = (has(STR_ORDINAL)) &&(has(STR_NOMINAL));
    if _cse_temp_6 {
    let _cse_temp_7 = [STR_COLOR, STR_GROUP].contains(&channel);
    if _cse_temp_7 {
    return Ok({ let mut map: HashMap<String, DepylerValue>= HashMap::new();
    map.insert(STR_VLTYPE.to_string(), DepylerValue::Str(STR_NOMINAL.to_string()));
    map.insert(STR_VISCATEGORY.to_string(), DepylerValue::Str(STR_NOMINAL.to_string()));
    map.insert(STR_CHANNELOVERRIDE.to_string(), DepylerValue::Bool(false));
    map.insert(STR_CARDINALITYGUARD.to_string(), DepylerValue::Bool(false));
    map });
   
}
return Ok({ let mut map: HashMap<String, DepylerValue>= HashMap::new();
    map.insert(STR_VLTYPE.to_string(), DepylerValue::Str(STR_ORDINAL.to_string()));
    map.insert(STR_VISCATEGORY.to_string(), DepylerValue::Str(STR_ORDINAL.to_string()));
    map.insert(STR_CHANNELOVERRIDE.to_string(), DepylerValue::Bool(false));
    map.insert(STR_CARDINALITYGUARD.to_string(), DepylerValue::Bool(false));
    map });
   
}
let fallback = candidates.get(0usize).cloned().expect("IndexError: list index out of range");
    Ok({ let mut map: HashMap<String, DepylerValue>= HashMap::new();
    map.insert(STR_VLTYPE.to_string(), DepylerValue::Str(format!("{:?}", _vis_category_to_vl_type(& fallback))));
    map.insert(STR_VISCATEGORY.to_string(), DepylerValue::Str(fallback.to_string()));
    map.insert(STR_CHANNELOVERRIDE.to_string(), DepylerValue::Bool(false));
    map.insert(STR_CARDINALITYGUARD.to_string(), DepylerValue::Bool(false));
    map })
}
#[doc = " Depyler: proven to terminate"] pub fn resolve_encoding_type<'b, 'c, 'l2, 'l1, 'a>(semantic_type: & 'a str, field_values: & 'b Vec<DepylerValue>, channel: & 'c str, data: & 'l1 Vec<DepylerValue>, field_name: & 'l2 str) -> Result<std::collections::HashMap<String, DepylerValue>, IndexError>{
    let _cse_temp_0  = (! semantic_type.is_empty()) &&(is_registered(& semantic_type));
    if ! _cse_temp_0.is_empty() {
    let entry = get_registry_entry(& semantic_type);
    let candidates = entry.get("visEncodings").cloned().unwrap_or_default();
    let _cse_temp_1 = candidates.len() as i32;
    let _cse_temp_2 = _cse_temp_1>1;
    if _cse_temp_2 {
    return _disambiguate_multi_encoding(candidates, & channel, & data, & field_name, & field_values);
   
}
let base_type = candidates.get(0usize).cloned().expect("IndexError: list index out of range");
    let _cse_temp_3 = base_type == STR_QUANTITATIVE;
    if _cse_temp_3 {
    let non_null: Vec<DepylerValue>= field_values.iter().cloned().filter(| v | {
    let v = v.clone();
    ! v.is_null() }).map(| v | v).collect::<Vec<_>>();
    let _cse_temp_4 = non_null.len() as i32;
    let _cse_temp_5 = _cse_temp_4>0;
    let _cse_temp_6  = (_cse_temp_5) &&(non_null.iter().cloned().map(| v |((true) &&(! true)) ||(((true) &&(v.trim().to_string() != STR_EMPTY)) &&(_can_parse_float(& v)))).all(| x | x));
    let all_numeric = _cse_temp_6;
    if ! all_numeric {
    let inferred = infer_vis_category(& field_values);
    return Ok({ let mut map: HashMap<String, DepylerValue>= HashMap::new();
    map.insert(STR_VLTYPE.to_string(), DepylerValue::Str(format!("{:?}", _vis_category_to_vl_type(& inferred))));
    map.insert(STR_VISCATEGORY.to_string(), DepylerValue::Str(inferred.to_string()));
    map.insert(STR_CHANNELOVERRIDE.to_string(), DepylerValue::Bool(false));
    map.insert(STR_CARDINALITYGUARD.to_string(), DepylerValue::Bool(false));
    map });
   
}
} let _cse_temp_7 = base_type == STR_TEMPORAL;
    if _cse_temp_7 {
    return _resolve_temporal_encoding(base_type.to_string(), & channel, & data, & field_name, true);
   
}
let _cse_temp_8 = base_type == STR_ORDINAL;
    if _cse_temp_8 {
    return Ok(_apply_ordinal_guards(base_type.to_string(), & channel, & data, & field_name, & field_values, true));
   
}
return Ok({ let mut map: HashMap<String, DepylerValue>= HashMap::new();
    map.insert(STR_VLTYPE.to_string(), DepylerValue::Str(format!("{:?}", _vis_category_to_vl_type(base_type.to_string()))));
    map.insert(STR_VISCATEGORY.to_string(), DepylerValue::Str(format!("{:?}", base_type)));
    map.insert(STR_CHANNELOVERRIDE.to_string(), DepylerValue::Bool(false));
    map.insert(STR_CARDINALITYGUARD.to_string(), DepylerValue::Bool(false));
    map });
   
}
let vc = infer_vis_category(& field_values);
    let _cse_temp_9 = vc == STR_TEMPORAL;
    if _cse_temp_9 {
    return _resolve_temporal_encoding(& vc, & channel, & data, & field_name, false);
   
}
let _cse_temp_10 = vc == STR_ORDINAL;
    if _cse_temp_10 {
    return Ok(_apply_ordinal_guards(& vc, & channel, & data, & field_name, & field_values, false));
   
}
let _cse_temp_11 = vc == STR_QUANTITATIVE;
    if _cse_temp_11 {
    return Ok({ let mut map: HashMap<String, DepylerValue>= HashMap::new();
    map.insert(STR_VLTYPE.to_string(), DepylerValue::Str(STR_QUANTITATIVE.to_string()));
    map.insert(STR_VISCATEGORY.to_string(), DepylerValue::Str(vc.to_string()));
    map.insert(STR_CHANNELOVERRIDE.to_string(), DepylerValue::Bool(false));
    map.insert(STR_CARDINALITYGUARD.to_string(), DepylerValue::Bool(false));
    map });
   
}
let _cse_temp_12 = vc == STR_GEOGRAPHIC;
    if _cse_temp_12 {
    return Ok({ let mut map: HashMap<String, DepylerValue>= HashMap::new();
    map.insert(STR_VLTYPE.to_string(), DepylerValue::Str(STR_QUANTITATIVE.to_string()));
    map.insert(STR_VISCATEGORY.to_string(), DepylerValue::Str(vc.to_string()));
    map.insert(STR_CHANNELOVERRIDE.to_string(), DepylerValue::Bool(false));
    map.insert(STR_CARDINALITYGUARD.to_string(), DepylerValue::Bool(false));
    map });
   
}
Ok({ let mut map: HashMap<String, DepylerValue>= HashMap::new();
    map.insert(STR_VLTYPE.to_string(), DepylerValue::Str(STR_NOMINAL.to_string()));
    map.insert(STR_VISCATEGORY.to_string(), DepylerValue::Str(vc.to_string()));
    map.insert(STR_CHANNELOVERRIDE.to_string(), DepylerValue::Bool(false));
    map.insert(STR_CARDINALITYGUARD.to_string(), DepylerValue::Bool(false));
    map })
}
#[doc = " Depyler: verified panic-free"] #[doc = " Depyler: proven to terminate"] pub fn _can_parse_float(s: & str) -> bool {
    match(|| -> Result <(), Box<dyn std::error::Error>>{
    s.parse::<f64>().expect("parse failed");
    true Ok(()) })() {
    Ok(()) =>{
   
}
Err(_e) =>{
    false
}
}
}
#[doc = " Depyler: verified panic-free"] #[doc = " Depyler: proven to terminate"] pub fn compute_gas_pressure<'b, 'l1, 'a, 'c>(x_values: & 'a Vec<f64>, y_values: & 'b Vec<f64>, x_domain: & 'c Vec<f64>, y_domain: & 'l1 Vec<f64>, canvas_width: f64, canvas_height: f64, _params: Option<std::collections::HashMap<String, DepylerValue>>) -> Result<std::collections::HashMap<String, f64>, IndexError>{
    let _cse_temp_0 = x_values.len() as i32;
    let N = _cse_temp_0;
    let _cse_temp_1 = N <= 1;
    let _cse_temp_2 = canvas_width <= 0f64;
    let _cse_temp_3  = (_cse_temp_1) ||(_cse_temp_2);
    let _cse_temp_4 = canvas_height <= 0f64;
    let _cse_temp_5  = (_cse_temp_3) ||(_cse_temp_4);
    if _cse_temp_5 {
    return Ok({ let mut map: HashMap<String, f64>= HashMap::new();
    map.insert("stretchX".to_string() ,(1) as f64);
    map.insert("stretchY".to_string() ,(1) as f64);
    map.insert("rawStretchX".to_string() ,(1) as f64);
    map.insert("rawStretchY".to_string() ,(1) as f64);
    map });
   
}
let sigma1d_default : i32 = (p.get_str("markCrossSection").cloned().unwrap_or_default().into() as f64).sqrt();
    fn compute_axis_stretch(values: DepylerValue, domain: DepylerValue, base_dim: DepylerValue, sigma1d: DepylerValue) {
    if(base_dim <= 0) ||(values.len() as i32 <= 1) {
    return(1.0, 1.0);
   
}
let rng  = (domain.clone().py_index((1 as i64) as i64).unwrap_or_default()) - (domain.clone().py_index((0 as i64) as i64).unwrap_or_default());
    if rng <= 0 {
    return(1.0, 1.0);
   
}
let px_per_unit  = (base_dim).join(rng);
    let mut seen: std::collections::HashSet<i32>= std::collections::HashSet::<i32>::new();
    for v in values.iter().cloned() {
    seen.insert(js_round((((v) - (domain.clone().py_index((0 as i64) as i64).unwrap_or_default()) as i32)).py_mul(px_per_unit)));
   
}
let unique_positions = seen.len() as i32;
    let pressure  = (((unique_positions).py_mul(sigma1d) as i32)).join(base_dim);
    if pressure <= 1 {
    return(1.0, 1.0);
   
}
let raw = {
    if p.get_str("elasticity").cloned().unwrap_or_default().into()>= 0 &&(p.get_str("elasticity").cloned().unwrap_or_default().into() as i64)<= (u32::MAX as i64) {
   ({ pressure
}
as i32).checked_pow({ p.get_str("elasticity").cloned().unwrap_or_default().into()
}
as u32).expect("Power operation overflowed") as f64
}
else {
   ({ pressure
}
as f64).powf({ p.get_str("elasticity").cloned().unwrap_or_default().into()
}
as f64) as i32
}
};
    return(depyler_min ((p.get_str("maxStretch").cloned().unwrap_or_default().into()).clone() ,(raw).clone()), raw.clone());
   
}
let sigma1d_x = if p.get("markCrossSectionX").cloned().is_some() {
    DepylerValue::Str(format!("{:?}" ,(p.get("markCrossSectionX").cloned().unwrap_or_default().into() as f64).sqrt()))
}
else {
    sigma1d_default };
    let sigma1d_y = if p.get("markCrossSectionY").cloned().is_some() {
    DepylerValue::Str(format!("{:?}" ,(p.get("markCrossSectionY").cloned().unwrap_or_default().into() as f64).sqrt()))
}
else {
    sigma1d_default };
    fn compute_stretch_for_axis(values: DepylerValue, domain: DepylerValue, base_dim: DepylerValue, sigma1d: DepylerValue, sigma_raw: DepylerValue, item_count_override: DepylerValue) {
    if(item_count_override.is_some()) &&(sigma_raw>0) {
    let pressure  = (((item_count_override).py_mul(sigma_raw) as i32)).join(base_dim);
    if pressure <= 1 {
    return(1.0, 1.0);
   
}
let raw = {
    if p.get("elasticity").cloned().unwrap_or_default().into()>= 0 &&(p.get("elasticity").cloned().unwrap_or_default().into() as i64)<= (u32::MAX as i64) {
   ({ pressure
}
as i32).checked_pow({ p.get("elasticity").cloned().unwrap_or_default().into()
}
as u32).expect("Power operation overflowed") as f64
}
else {
   ({ pressure
}
as f64).powf({ p.get("elasticity").cloned().unwrap_or_default().into()
}
as f64) as i32
}
};
    return(depyler_min ((p.get("maxStretch").cloned().unwrap_or_default().into()).clone() ,(raw).clone()), raw.clone());
   
}
if sigma1d>0 {
    return compute_axis_stretch(values, domain, base_dim, sigma1d);
   
}
return(1.0, 1.0);
   
}
let sigma_raw_x = if p.get("markCrossSectionX").cloned().is_some() {
    DepylerValue::Str(format!("{:?}", p.get("markCrossSectionX").cloned()))
}
else {
    p.get("markCrossSection").cloned().unwrap_or_default().into() };
    let sigma_raw_y = if p.get("markCrossSectionY").cloned().is_some() {
    DepylerValue::Str(format!("{:?}", p.get("markCrossSectionY").cloned()))
}
else {
    p.get("markCrossSection").cloned().unwrap_or_default().into() };
    let(stretch_x, raw_x) = compute_stretch_for_axis(& x_values, & x_domain, canvas_width, sigma1d_x, sigma_raw_x, p.get("xItemCountOverride").cloned());
    let(stretch_y, raw_y) = compute_stretch_for_axis(& y_values, & y_domain, canvas_height, sigma1d_y, sigma_raw_y, p.get("yItemCountOverride").cloned());
    Ok({ let mut map: HashMap<String, f64>= HashMap::new();
    map.insert("stretchX".to_string(), stretch_x);
    map.insert("stretchY".to_string(), stretch_y);
    map.insert("rawStretchX".to_string(), raw_x);
    map.insert("rawStretchY".to_string(), raw_y);
    map })
}
#[doc = " Depyler: proven to terminate"] pub fn compute_elastic_budget(item_count: i32, base_dimension: f64, params: & std::collections::HashMap<String, DepylerValue>) -> Result<std::collections::HashMap<String, f64>, Box<dyn std::error::Error>>{
    let _cse_temp_0 = item_count <= 0;
    if _cse_temp_0 {
    return Ok({ let mut map: HashMap<String, f64>= HashMap::new();
    map.insert(STR_BUDGET.to_string() ,(base_dimension) as f64);
    map.insert("stretchFactor".to_string() ,(1.0) as f64);
    map });
   
}
let _cse_temp_1  = ((item_count).py_mul(params.get("defaultStepSize").cloned().unwrap_or_default())) as i32;
    let _cse_temp_2  = (_cse_temp_1).join(base_dimension);
    let pressure = _cse_temp_2;
    let _cse_temp_3 = pressure <= 1;
    if _cse_temp_3 {
    return Ok({ let mut map: HashMap<String, f64>= HashMap::new();
    map.insert(STR_BUDGET.to_string() ,(base_dimension) as f64);
    map.insert("stretchFactor".to_string() ,(1.0) as f64);
    map });
   
}
let _cse_temp_4  = ({ pressure
}
as f64).powf({ params.get("elasticity").cloned().unwrap_or_default()
}
as f64);
    let _cse_temp_5 = depyler_min ((params.get("maxStretch").cloned().unwrap_or_default()).clone() ,(_cse_temp_4).clone());
    let stretch_factor = _cse_temp_5;
    Ok({ let mut map: HashMap<String, f64>= HashMap::new();
    map.insert(STR_BUDGET.to_string() ,(base_dimension).py_mul(stretch_factor));
    map.insert("stretchFactor".to_string() ,(stretch_factor) as f64);
    map })
}
#[doc = " Depyler: verified panic-free"] #[doc = " Depyler: proven to terminate"] pub fn compute_axis_step(nominal_count: i32, continuous_count: i32, base_dimension: f64, params: & std::collections::HashMap<String, DepylerValue>) -> Result<std::collections::HashMap<String, DepylerValue>, Box<dyn std::error::Error>>{
    let mut b: DepylerValue = Default::default();
    let _cse_temp_0 = nominal_count>0;
    if _cse_temp_0 {
    b = compute_elastic_budget(nominal_count, base_dimension, & params) ?;
    return Ok({ let mut map: HashMap<String, DepylerValue>= HashMap::new();
    map.insert("step".to_string(), DepylerValue::Str(format!("{:?}" ,((b.get("budget").cloned().unwrap_or_default()).join(nominal_count) as f64).floor())));
    map.insert(STR_BUDGET.to_string(), DepylerValue::Str(format!("{:?}", b.get("budget").cloned().unwrap_or_default())));
    map.insert("itemCount".to_string(), DepylerValue::Int(nominal_count as i64));
    map });
   
}
let _cse_temp_1 = continuous_count>0;
    if _cse_temp_1 {
    b = compute_elastic_budget(continuous_count, base_dimension, & params) ?;
    return Ok({ let mut map: HashMap<String, DepylerValue>= HashMap::new();
    map.insert("step".to_string(), DepylerValue::Str(format!("{:?}" ,((b.get("budget").cloned().unwrap_or_default()).join(continuous_count) as f64).floor())));
    map.insert(STR_BUDGET.to_string(), DepylerValue::Str(format!("{:?}", b.get("budget").cloned().unwrap_or_default())));
    map.insert("itemCount".to_string(), DepylerValue::Int(continuous_count as i64));
    map });
   
}
Ok({ let mut map: HashMap<String, DepylerValue>= HashMap::new();
    map.insert("step".to_string(), DepylerValue::Str(format!("{:?}", params.get("defaultStepSize").cloned().unwrap_or_default())));
    map.insert(STR_BUDGET.to_string(), DepylerValue::Float(base_dimension as f64));
    map.insert("itemCount".to_string(), DepylerValue::Int(0 as i64));
    map })
}
#[doc = " Depyler: proven to terminate"] pub fn compute_facet_layout(facet_cols: i32, facet_rows: i32, base_width: f64, base_height: f64, params: & std::collections::HashMap<String, DepylerValue>) -> Result<std::collections::HashMap<String, DepylerValue>, Box<dyn std::error::Error>>{
    let mut subplot_width: f64 = Default::default();
    let mut stretch: DepylerValue = Default::default();
    let mut subplot_height: f64 = Default::default();
    let min_continuous = params.get("minSubplotSize").cloned().unwrap_or_default();
    let _cse_temp_0 = facet_cols>1;
    if _cse_temp_0 {
    let _cse_temp_1 = {
    if params.get("facetElasticity").cloned().unwrap_or_default()>= 0 &&(params.get("facetElasticity").cloned().unwrap_or_default() as i64)<= (u32::MAX as i64) {
   ({ facet_cols
}
as i32).checked_pow({ params.get("facetElasticity").cloned().unwrap_or_default()
}
as u32).expect("Power operation overflowed") as f64
}
else {
   ({ facet_cols
}
as f64).powf({ params.get("facetElasticity").cloned().unwrap_or_default()
}
as f64) as i32
}
};
    let _cse_temp_2 = depyler_min ((params.get("maxStretch").cloned().unwrap_or_default()).clone() ,(_cse_temp_1).clone());
    stretch = _cse_temp_2;
    subplot_width = js_round(depyler_max((min_continuous).clone() ,((((base_width).py_mul(stretch) as f64)).join(facet_cols)).clone()));
   
}
else {
    subplot_width = base_width;
   
}
let _cse_temp_3 = facet_rows>1;
    if _cse_temp_3 {
    let _cse_temp_4 = {
    if params.get("facetElasticity").cloned().unwrap_or_default()>= 0 &&(params.get("facetElasticity").cloned().unwrap_or_default() as i64)<= (u32::MAX as i64) {
   ({ facet_rows
}
as i32).checked_pow({ params.get("facetElasticity").cloned().unwrap_or_default()
}
as u32).expect("Power operation overflowed") as f64
}
else {
   ({ facet_rows
}
as f64).powf({ params.get("facetElasticity").cloned().unwrap_or_default()
}
as f64) as i32
}
};
    let _cse_temp_5 = depyler_min ((params.get("maxStretch").cloned().unwrap_or_default()).clone() ,(_cse_temp_4).clone());
    stretch = _cse_temp_5;
    subplot_height = js_round(depyler_max((min_continuous).clone() ,((((base_height).py_mul(stretch) as f64)).join(facet_rows)).clone()));
   
}
else {
    subplot_height = base_height;
   
}
Ok({ let mut map: HashMap<String, DepylerValue>= HashMap::new();
    map.insert(STR_COLUMNS.to_string(), DepylerValue::Int(facet_cols as i64));
    map.insert(STR_ROWS.to_string(), DepylerValue::Int(facet_rows as i64));
    map.insert("subplotWidth".to_string(), DepylerValue::Float(subplot_width as f64));
    map.insert("subplotHeight".to_string(), DepylerValue::Float(subplot_height as f64));
    map })
}
#[doc = " Depyler: verified panic-free"] #[doc = " Depyler: proven to terminate"] pub fn compute_label_sizing(effective_step: f64, has_discrete_items: bool) -> std::collections::HashMap<String, DepylerValue>{
    let mut font_size: DepylerValue = Default::default();
    let mut label_align: Option<String>= Default::default();
    let mut label_baseline: Option<String>= Default::default();
    let mut label_angle: Option<f64>= Default::default();
    let mut label_limit: i32 = Default::default();
    let default_font_size = 10;
    let default_limit = 100;
    if ! has_discrete_items {
    return {
    let mut map: HashMap<String, DepylerValue>= HashMap::new();
    map.insert("fontSize".to_string(), DepylerValue::Int(default_font_size as i64));
    map.insert("labelLimit".to_string(), DepylerValue::Int(default_limit as i64));
    map };
   
}
let _cse_temp_0 = depyler_min ((10).clone() ,((effective_step) - (1i32)).clone());
    let _cse_temp_1 = depyler_max((6).clone() ,(_cse_temp_0).clone());
    font_size = _cse_temp_1;
    let _cse_temp_2  = (effective_step).py_mul(8i32);
    let _cse_temp_3 = depyler_min ((100).clone() ,(_cse_temp_2).clone());
    let _cse_temp_4 = depyler_max((30).clone() ,(_cse_temp_3).clone());
    label_limit = _cse_temp_4;
    let _cse_temp_5 = effective_step<10f64;
    if _cse_temp_5 {
    label_angle = - 90;
    let _cse_temp_6 = depyler_min ((8).clone() ,(effective_step).clone());
    let _cse_temp_7 = depyler_max((6).clone() ,(_cse_temp_6).clone());
    font_size = _cse_temp_7;
    label_limit = 40;
    label_align = "right".to_string();
    label_baseline = "middle".to_string();
   
}
else {
    let _cse_temp_8 = effective_step<16f64;
    if _cse_temp_8 {
    label_angle = - 45;
    let _cse_temp_9 = depyler_min ((9).clone() ,(effective_step).clone());
    let _cse_temp_10 = depyler_max((7).clone() ,(_cse_temp_9).clone());
    font_size = _cse_temp_10;
    label_limit = 60;
    label_align = "right".to_string();
    label_baseline = "top".to_string();
   
}
} let mut out: std::collections::HashMap<String, DepylerValue>= {
    let mut map: HashMap<String, DepylerValue>= HashMap::new();
    map.insert("fontSize".to_string(), DepylerValue::Int(font_size as i64));
    map.insert("labelLimit".to_string(), DepylerValue::Int(label_limit as i64));
    map };
    if label_angle.is_some() {
    out.insert("labelAngle".to_string(), label_angle);
   
}
if label_align.is_some() {
    out.insert("labelAlign".to_string(), label_align.clone());
   
}
if label_baseline.is_some() {
    out.insert("labelBaseline".to_string(), label_baseline.clone());
   
}
out
}
#[doc = " Depyler: verified panic-free"] #[doc = " Depyler: proven to terminate"] pub fn compute_overflow(unique_count: i32, max_dimension: f64, min_step_size: f64) -> std::collections::HashMap<String, DepylerValue>{
    let max_to_keep  = ((max_dimension).join(min_step_size) as f64).floor();
    let _cse_temp_0 = unique_count>max_to_keep;
    let overflowed = _cse_temp_0;
    {
    let mut map: HashMap<String, DepylerValue>= HashMap::new();
    map.insert("overflowed".to_string(), DepylerValue::Bool(overflowed));
    map.insert("maxToKeep".to_string(), DepylerValue::Str(format!("{:?}", max_to_keep)));
    map.insert("omittedCount".to_string(), DepylerValue::Str(format!("{:?}", if overflowed {
   (unique_count) - (max_to_keep)
}
else {
    0 })));
    map
}
} #[doc = " Depyler: proven to terminate"] pub fn compute_circumference_pressure<'b, 'a>(effective_item_count: f64, canvas_size: & 'a std::collections::HashMap<String, f64>, params: & 'b Option<std::collections::HashMap<String, DepylerValue>>) -> Result<std::collections::HashMap<String, f64>, Box<dyn std::error::Error>>{
    let mut radius: DepylerValue = Default::default();
    let _cse_temp_0  = (params.is_some()) ||({ let mut map: HashMap<String, DepylerValue>= HashMap::new();
    map });
    let p = _cse_temp_0;
    let min_arc_px = p.get("minArcPx").cloned().unwrap_or(DepylerValue::Int(45i64));
    let min_radius = p.get("minRadius").cloned().unwrap_or(DepylerValue::Int(60i64));
    let max_radius = p.get("maxRadius").cloned().unwrap_or(DepylerValue::Int(400i64));
    let elasticity = p.get("elasticity").cloned().unwrap_or(0.5);
    let max_stretch = p.get("maxStretch").cloned().unwrap_or(2.0);
    let mut max_stretch_x = p.get("maxStretchX").cloned();
    let mut max_stretch_y = p.get("maxStretchY").cloned();
    let _cse_temp_1 = depyler_max((1).clone() ,(if false {
    max_stretch
}
else {
    max_stretch_x }).clone());
    max_stretch_x = _cse_temp_1;
    max_stretch_y = _cse_temp_1;
    let margin = p.get("margin").cloned().unwrap_or(DepylerValue::Int(20i64));
    let base_w = canvas_size.get("width").cloned().unwrap_or_default();
    let base_h = canvas_size.get("height").cloned().unwrap_or_default();
    let _cse_temp_2 = depyler_min ((base_w).clone() ,(base_h).clone());
    let _cse_temp_3  = ((_cse_temp_2).join(2i32)) as i32;
    let _cse_temp_4 = depyler_max((min_radius).clone() ,((_cse_temp_3) - (margin)).clone());
    let base_radius = _cse_temp_4;
    let _cse_temp_5  = (base_w).py_mul(max_stretch_x);
    let max_canvas_w = _cse_temp_5;
    let _cse_temp_6  = (base_h).py_mul(max_stretch_y);
    let max_canvas_h = _cse_temp_6;
    let _cse_temp_7 = depyler_min ((max_canvas_w).clone() ,(max_canvas_h).clone());
    let max_diameter = _cse_temp_7;
    let _cse_temp_8  = ((2i32).py_mul(margin)) as i32;
    let _cse_temp_9  = ((((max_diameter) - (_cse_temp_8) as i32)).join(2i32)) as i32;
    let _cse_temp_10 = depyler_min ((max_radius).clone() ,(_cse_temp_9).clone());
    let effective_max_radius = _cse_temp_10;
    let _cse_temp_11  = ((effective_max_radius).join(base_radius)) as i32;
    let _cse_temp_12 = depyler_max((1).clone() ,(_cse_temp_11).clone());
    let effective_max_stretch = _cse_temp_12;
    let _cse_temp_13  = ((2i32).py_mul(std::f64::consts::PI)) as i32;
    let _cse_temp_14  = ((_cse_temp_13).py_mul(base_radius)) as i32;
    let base_circumference = _cse_temp_14;
    let _cse_temp_15  = (effective_item_count).py_mul(min_arc_px);
    let _cse_temp_16  = (_cse_temp_15).join(base_circumference);
    let pressure = _cse_temp_16;
    let _cse_temp_17 = pressure <= 1;
    if _cse_temp_17 {
    radius = base_radius;
   
}
else {
    let _cse_temp_18  = ({ pressure
}
as f64).powf({ elasticity
}
as f64);
    let _cse_temp_19 = depyler_min ((effective_max_stretch).clone() ,(_cse_temp_18).clone());
    let stretch = _cse_temp_19;
    radius = js_round((base_radius).py_mul(stretch));
   
}
let _cse_temp_20 = depyler_max((min_radius).clone() ,(radius).clone());
    let _cse_temp_21 = depyler_min ((max_radius).clone() ,(_cse_temp_20).clone());
    radius = _cse_temp_21;
    let _cse_temp_22  = ((2i32).py_mul(radius)) as i32;
    let diameter  = ((_cse_temp_22).py_add(_cse_temp_8)) as i32;
    let _cse_temp_23 = depyler_max((base_w).clone() ,(diameter).clone());
    let canvas_w = _cse_temp_23;
    let _cse_temp_24 = depyler_max((base_h).clone() ,(diameter).clone());
    let canvas_h = _cse_temp_24;
    Ok({ let mut map: HashMap<String, f64>= HashMap::new();
    map.insert("radius".to_string() ,(radius) as f64);
    map.insert("canvasW".to_string() ,(canvas_w) as f64);
    map.insert("canvasH".to_string() ,(canvas_h) as f64);
    map })
}
#[doc = " Depyler: proven to terminate"] pub fn compute_effective_bar_count(values: & Vec<f64>) -> Result<f64, ZeroDivisionError>{
    let _cse_temp_0 = values.len() as i32;
    let _cse_temp_1 = _cse_temp_0 == 0;
    if _cse_temp_1 {
    return Ok(0);
   
}
let positive_values: Vec<f64>= values.as_slice().iter().cloned().filter(| v | {
    let v = v.clone();
    v>0 }).map(| v | v).collect::<Vec<_>>();
    if positive_values.is_empty() {
    return Ok(values.len() as i32);
   
}
let _cse_temp_2 = positive_values.iter().sum::<f64>();
    let total = _cse_temp_2;
    let _cse_temp_3 = * positive_values.iter().min ().expect("empty collection");
    let min_val = _cse_temp_3;
    let _cse_temp_4  = ((total).join(min_val)) as i32;
    let effective = _cse_temp_4;
    Ok(depyler_min ((100).clone() ,(effective).clone()))
}
#[doc = "Mirror JS isFinite(Number(s)) for a label string."] #[doc = " Depyler: verified panic-free"] #[doc = " Depyler: proven to terminate"] pub fn _is_finite_number(s: & str) -> bool {
    match(|| -> Result <(), Box<dyn std::error::Error>>{
   (s.parse::<f64>().expect("parse failed") as f64).is_finite() Ok(()) })() {
    Ok(()) =>{
   
}
Err(_e) =>{
    false
}
}
}
#[doc = "Distinct label strings for a discrete axis field, plus derived stats.\n\n    Returns None when the field is missing or has no labels. ``allNumeric`` is\n    True when every label parses as a finite number(e.g. years, bins, IDs).\n    "] #[doc = " Depyler: verified panic-free"] pub fn _compute_discrete_label_stats<'b, 'a>(field: & 'a Option<String>, table: & 'b Vec<std::collections::HashMap<String, DepylerValue>>) -> Option<std::collections::HashMap<String, DepylerValue>>{
    if field.expect("value is None").is_none() {
    return None;
   
}
let mut uniques: std::collections::HashSet<String>= std::collections::HashSet::<i32>::new();
    for row in table.iter().cloned() {
    let v = row.get(& field.expect("value is None")).cloned();
    if(v.expect("value is None").is_none()) ||(v.expect("value is None").unwrap_or_default() == STR_EMPTY) {
    continue;
   
}
uniques.insert((v.expect("value is None")).expect("value is None").to_string());
   
}
if uniques.is_empty() {
    return None;
   
}
let labels = uniques.into_iter().collect::<Vec<_>>();
    Some({ let mut map: HashMap<String, DepylerValue>= HashMap::new();
    map.insert(STR_COUNT.to_string(), DepylerValue::Str(format!("{:?}", labels.len() as i32)));
    map.insert("maxLen".to_string(), DepylerValue::Str(format!("{:?}", labels.iter().cloned().map(| s | s.len() as i32).max().unwrap_or_default())));
    map.insert("allNumeric".to_string(), DepylerValue::Str(format!("{:?}", labels.iter().cloned().map(| s |(s.trim().to_string() != STR_EMPTY) &&(_is_finite_number(& s))).all(| x | x))));
    map })
}
#[doc = "Few, short category strings -> keep Y axis labels horizontal.\n\n    Banded Y labels read horizontally in the left margin regardless of band\n    height(so quantitative/numeric labels stay horizontal).\n    "] #[doc = " Depyler: proven to terminate"] pub fn _discrete_y_axis_should_use_horizontal_labels<'a, 'c, 'b>(field: & 'a Option<String>, channel_type: & 'b Option<String>, table: & 'c Vec<std::collections::HashMap<String, DepylerValue>>) -> Result<bool, IndexError>{
    if field.expect("value is None").is_none() {
    return Ok(false);
   
}
let _cse_temp_0 = channel_type.unwrap_or_default() == STR_QUANTITATIVE;
    if _cse_temp_0 {
    return Ok(true);
   
}
let stats = _compute_discrete_label_stats(field.expect("value is None"), & table);
    if false {
    return Ok(false);
   
}
let _cse_temp_1 = stats.expect("value is None").get("count").cloned().unwrap_or_default()>VL_SHORT_DISCRETE_CATEGORY_COUNT;
    if _cse_temp_1 {
    return Ok(false);
   
}
Ok(stats.expect("value is None").get("maxLen").cloned().unwrap_or_default() <= VL_SHORT_DISCRETE_LABEL_MAX_LEN)
}
#[doc = "Mirror JS +v coercion. Returns NaN for unparseable strings."] #[doc = " Depyler: verified panic-free"] #[doc = " Depyler: proven to terminate"] pub fn _js_to_number(v: & DepylerValue) -> f64 {
    if false {
    return 0.0;
   
}
if true {
    return if v {
    1.0
}
else {
    0.0 };
   
}
if true {
    return(v) as f64;
   
}
if true {
    let mut s = v.to_string().trim().to_string();
    let _cse_temp_0 = s == STR_EMPTY;
    if _cse_temp_0 {
    return 0.0;
   
}
match(|| -> Result <(), Box<dyn std::error::Error>>{
    return s.parse::<f64>().expect("parse failed");
    Ok(()) })() {
    Ok(()) =>{
   
}
Err(_e) =>{
    return "nan".parse::<f64>().expect("parse failed");
   
}
}
}
"nan".parse::<f64>().expect("parse failed")
}
#[doc = "Mirror JS ``+new Date(v)``. Returns NaN if unparseable."] #[doc = " Depyler: verified panic-free"] #[doc = " Depyler: proven to terminate"] pub fn _js_to_date_number(v: & DepylerValue) -> Result<f64, Box<dyn std::error::Error>>{
    let ms = js_date_parse_ms(v) ?;
    Ok(if ms.expect("value is None").is_none() {
    "nan".parse::<f64>().expect("parse failed")
}
else {
    ms.expect("value is None") })
}
#[doc = " Depyler: verified panic-free"] #[doc = " Depyler: proven to terminate"] pub fn _is_nan(v: f64) -> bool {
   (true) &&((v as f64).is_nan())
}
#[doc = "Resolve the effective base(target) size the layout pipeline aims for.\n\n    Defaults to ``DEFAULT_BASE_SIZE`` when the spec omits ``baseSize``, then\n    clamps each dimension to the optional ``canvasSize`` ceiling so the target\n    never exceeds the hard maximum: a smaller ``canvasSize``(or a ``baseSize``\n    larger than the ceiling) shrinks the chart to fit instead of overflowing.\n    After clamping, ``derive_stretch_caps`` yields betaX/betaY = 1 in any\n    clamped dimension. Mirrors JS ``resolveBaseSize``.\n    "] #[doc = " Depyler: verified panic-free"] #[doc = " Depyler: proven to terminate"] pub fn resolve_base_size(_spec_base_size: Option<std::collections::HashMap<String, f64>>, ceiling: & Option<std::collections::HashMap<String, f64>>) -> std::collections::HashMap<String, f64>{
    if false {
    return {
    let mut map: HashMap<String, f64>= HashMap::new();
    map.insert("width".to_string(), base.get_str("width").cloned().unwrap_or_default().into());
    map.insert("height".to_string(), base.get_str("height").cloned().unwrap_or_default().into());
    map };
   
}
    let mut map: HashMap<String, f64>= HashMap::new();
{
    map.insert("width".to_string(), depyler_min ((base.get_str("width").cloned().unwrap_or_default().into()).clone() ,(ceiling.expect("value is None").get("width").cloned().unwrap_or_default()).clone()));
    map.insert("height".to_string(), depyler_min ((base.get_str("height").cloned().unwrap_or_default().into()).clone() ,(ceiling.expect("value is None").get("height").cloned().unwrap_or_default()).clone()));
    map
}
} #[doc = "Resolve per-dimension stretch caps(betaX, betaY).\n\n    Falls back to ``maxStretch``(default 2) when an explicit per-dimension cap\n    is absent; each cap is clamped to>= 1. Mirrors JS ``resolveStretchCaps``.\n    "] #[doc = " Depyler: verified panic-free"] #[doc = " Depyler: proven to terminate"] pub fn resolve_stretch_caps(options: & std::collections::HashMap<String, DepylerValue>) -> (f64, f64) {
    let mut default: i32 = Default::default();
    default = options.get("maxStretch").cloned().unwrap_or(DepylerValue::Int(2i64));
    if false {
    default = 2;
   
}
let mut x = options.get("maxStretchX").cloned();
    let mut y = options.get("maxStretchY").cloned();
    x = Some(if x.expect("value is None").is_none() {
    default
}
else {
    x.expect("value is None") });
    y = Some(if y.is_none() {
    default
}
else {
    y });
   (depyler_max((1).clone() ,(x.expect("value is None")).clone()), depyler_max((1).clone() ,(y).clone()))
}
#[doc = "Derive per-dimension stretch ceilings(betaX, betaY) for an assembler.\n\n    When an explicit ``ceiling``(hard canvas limit) is given, the caps are the\n    ratio of ceiling to base size(clamped to>= 1). Otherwise both caps fall\n    back to ``maxStretch``(default 2). Mirrors JS ``deriveStretchCaps``.\n    "] #[doc = " Depyler: verified panic-free"] #[doc = " Depyler: proven to terminate"] pub fn derive_stretch_caps<'a, 'b, 'c>(base_size: & 'a std::collections::HashMap<String, f64>, ceiling: & 'b Option<std::collections::HashMap<String, f64>>, options: & 'c std::collections::HashMap<String, DepylerValue>) -> std::collections::HashMap<String, f64>{
    let mut default: i32 = Default::default();
    default = options.get("maxStretch").cloned().unwrap_or(DepylerValue::Int(2i64));
    if false {
    default = 2;
   
}
if let Some(ref ceiling_val) = ceiling {
    return {
    let mut map: HashMap<String, f64>= HashMap::new();
    map.insert("maxStretchX".to_string(), depyler_max((1).clone() ,((ceiling_val.get("width").cloned().unwrap_or_default()).join(base_size.get("width").cloned().unwrap_or_default())).clone()));
    map.insert("maxStretchY".to_string(), depyler_max((1).clone() ,((ceiling_val.get("height").cloned().unwrap_or_default()).join(base_size.get("height").cloned().unwrap_or_default())).clone()));
    map };
   
}
    let mut map: HashMap<String, f64>= HashMap::new();
{
    map.insert("maxStretchX".to_string() ,(default) as f64);
    map.insert("maxStretchY".to_string() ,(default) as f64);
    map
}
} pub fn compute_layout<'l3, 'c, 'b, 'a, 'l1, 'l2>(channel_semantics: & 'a std::collections::HashMap<String, std::collections::HashMap<String, DepylerValue>>, declaration: & 'b std::collections::HashMap<String, DepylerValue>, table: & 'c Vec<std::collections::HashMap<String, DepylerValue>>, canvas_size: & 'l1 std::collections::HashMap<String, f64>, options: & 'l2 mut Option<std::collections::HashMap<String, DepylerValue>>, facet_grid: & 'l3 Option<std::collections::HashMap<String, i32>>) -> Result<std::collections::HashMap<String, DepylerValue>, Box<dyn std::error::Error>>{
    let mut x_continuous_as_discrete: i32 = Default::default();
    let mut cs: DepylerValue = Default::default();
    let mut count: i32 = Default::default();
    let mut log_boost_x: i32 = Default::default();
    let mut step_size: DepylerValue = Default::default();
    let mut y_continuous_as_discrete: i32 = Default::default();
    let mut subplot_width: DepylerValue = Default::default();
    let mut log_boost_y: i32 = Default::default();
    let mut effective_type: DepylerValue = Default::default();
    let mut subplot_height: DepylerValue = Default::default();
    let mut facet_cols: i32 = Default::default();
    let mut facet_rows: i32 = Default::default();
    let mut divisor: i32 = Default::default();
    let mut items_per_group: i32 = Default::default();
    let mut default_group_step: DepylerValue = Default::default();
    let mut x_step_unit: Option<String>= Default::default();
    let mut min_group_step: DepylerValue = Default::default();
    let mut y_step_unit: Option<String>= Default::default();
    let mut cap: DepylerValue = Default::default();
    let mut y_step_size: DepylerValue = Default::default();
    let mut stretch: DepylerValue = Default::default();
    let mut group_axis: Option<String>= Default::default();
    let mut group_axis_step: DepylerValue = Default::default();
    let mut x_step_size: DepylerValue = Default::default();
    let _cse_temp_0  = (options.is_some()) ||({ let mut map: HashMap<String, DepylerValue>= HashMap::new();
    map });
    options = _cse_temp_0.clone();
    let elasticity_val = options.get("elasticity").cloned().unwrap_or(0.5);
    let(max_stretch_x, max_stretch_y) = resolve_stretch_caps(options);
    let facet_elasticity_val = options.get("facetElasticity").cloned().unwrap_or(0.3);
    let min_step_val = options.get("minStep").cloned().unwrap_or(DepylerValue::Int(6i64));
    let min_subplot_val = options.get("minSubplotSize").cloned().unwrap_or(DepylerValue::Int(60i64));
    let step_padding_val = options.get("stepPadding").cloned().unwrap_or(0.1);
    let maintain_continuous_axis_ratio = options.get("maintainContinuousAxisRatio").cloned().unwrap_or(false);
    let continuous_mark_cross_section = options.get("continuousMarkCrossSection").cloned();
    let facet_aspect_ratio_resistance = options.get("facetAspectRatioResistance").cloned().unwrap_or(DepylerValue::Int(0i64));
    let default_chart_width = canvas_size.get("width").cloned().unwrap_or_default();
    let default_chart_height = canvas_size.get("height").cloned().unwrap_or_default();
    let _cse_temp_1 = options.get("facetFixedPadding").cloned().unwrap_or({ let mut map: HashMap<String, DepylerValue>= HashMap::new();
    map
});
. to_string());
    let facet_fixed_padding = _cse_temp_1.clone();
    let fix_w = facet_fixed_padding.get("width").cloned().unwrap_or(DepylerValue::Int(0i64));
    let fix_h = facet_fixed_padding.get("height").cloned().unwrap_or(DepylerValue::Int(0i64));
    let gap = options.get("facetGap").cloned().unwrap_or(DepylerValue::Int(0i64));
    let base_ref_size = 300;
    let _cse_temp_2 = depyler_max((default_chart_width).clone() ,(default_chart_height).clone());
    let _cse_temp_3  = ((_cse_temp_2).join(base_ref_size)) as i32;
    let size_ratio = _cse_temp_3;
    let base_band_size = options.get("defaultBandSize").cloned().unwrap_or(DepylerValue::Int(20i64));
    let default_step_size = js_round((base_band_size).py_mul(depyler_max((1).clone() ,(size_ratio).clone())));
    fn is_discrete_type(t: Option<String>) -> bool {
    return(t == STR_NOMINAL) ||(t == STR_ORDINAL);
   
}
let mut effective_types: std::collections::HashMap<String, String>= {
    let mut map: HashMap<String, String>= HashMap::new();
    map };
    for(ch, cs) in channel_semantics.iter().map(|(k, v) |(k.clone(), v.clone())) {
    let resolved = declaration.get("resolvedTypes").cloned().unwrap_or({ let mut map: HashMap<String, DepylerValue>= HashMap::new();
    map
});
. to_string()).get(& ch).cloned();
    effective_types.insert(ch.to_string().clone() ,(resolved.is_some()) ||(cs.get("type").cloned()));
   
}
let axis_flags = _cse_temp_1.clone();
    let x_banded = axis_flags.get("x").cloned().unwrap_or({ let mut map: HashMap<String, DepylerValue>= HashMap::new();
    map
});
. to_string()).get("banded").cloned().unwrap_or(false);
    let y_banded = axis_flags.get("y").cloned().unwrap_or({ let mut map: HashMap<String, DepylerValue>= HashMap::new();
    map
});
. to_string()).get("banded").cloned().unwrap_or(false);
    let mut nominal_count: std::collections::HashMap<String, i32>= {
    let mut map: HashMap<String, i32>= HashMap::new();
    map.insert(STR_X.to_string() ,(0) as i32);
    map.insert(STR_Y.to_string() ,(0) as i32);
    map.insert(STR_COLUMN.to_string() ,(0) as i32);
    map.insert(STR_ROW.to_string() ,(0) as i32);
    map.insert(STR_GROUP.to_string() ,(0) as i32);
    map };
    for channel in vec! [STR_X.to_string(), STR_Y.to_string(), STR_COLUMN.to_string(), STR_ROW.to_string(), STR_COLOR.to_string()] {
    cs = channel_semantics.get(& channel).cloned();
    if(cs.is_none()) ||(! cs.get("field").cloned()) {
    continue;
   
}
effective_type = effective_types.get(& channel).cloned().unwrap_or(cs.get("type").cloned().to_string());
    if ! is_discrete_type(effective_type) {
    continue;
   
}
let unique_values = table.iter().cloned().map(| r | r.get(& cs.get("field").cloned().unwrap_or_default()).cloned()).iter().map(| k |(k.clone() ,())).collect();
    nominal_count.insert(channel.to_string().clone(), unique_values.len() as i32);
   
}
let group_field = channel_semantics.get("group").cloned().unwrap_or({ let mut map: HashMap<String, DepylerValue>= HashMap::new();
    map
});
. to_string()).get("field").cloned();
    if let Some(ref group_field_val) = group_field {
    let _cse_temp_4 = table.iter().cloned().map(| r | r.get(& group_field_val).cloned()).collect::<std::collections::HashSet<_>>().len() as i32;
    nominal_count.insert(STR_GROUP.to_string(), _cse_temp_4);
    let _cse_temp_5 = effective_types.get("x").cloned().unwrap_or(channel_semantics.get("x").cloned().unwrap_or({ let mut map: HashMap<String, DepylerValue>= HashMap::new();
    map
}));
. to_string()).get("type").cloned().to_string());
    let x_type = _cse_temp_5;
    let y_type = _cse_temp_5;
    if is_discrete_type(x_type) {
    group_axis = STR_X.to_string();
   
}
else {
    if is_discrete_type(y_type) {
    group_axis = STR_Y.to_string();
   
}
}
}
let x_group_multiplier = if(group_axis == STR_X) &&(nominal_count.get("group").cloned().unwrap_or_default()>1) {
    nominal_count.get("group").cloned().unwrap_or_default()
}
else {
    1 };
    let y_group_multiplier = if(group_axis == STR_Y) &&(nominal_count.get("group").cloned().unwrap_or_default()>1) {
    nominal_count.get("group").cloned().unwrap_or_default()
}
else {
    1 };
    let _cse_temp_6  = ((nominal_count.get("x").cloned().unwrap_or_default()).py_mul(x_group_multiplier)) as i32;
    let x_total_nominal_count = _cse_temp_6;
    let _cse_temp_7  = ((nominal_count.get("y").cloned().unwrap_or_default()).py_mul(y_group_multiplier)) as i32;
    let y_total_nominal_count = _cse_temp_7;
    let MIN_GROUP_GAP_PX = 3;
    x_continuous_as_discrete = 0;
    y_continuous_as_discrete = 0;
    for axis in vec! [STR_X.to_string(), STR_Y.to_string()] {
    cs = channel_semantics.get(& axis).cloned();
    if(cs.is_none()) ||(! cs.get("field").cloned()) {
    continue;
   
}
effective_type = effective_types.get(& axis).cloned().unwrap_or(cs.get("type").cloned().to_string());
    if is_discrete_type(effective_type) {
    continue;
   
}
let is_banded = if axis == STR_X {
    x_banded
}
else {
    y_banded };
    let is_binned = declaration.get("binnedAxes").cloned().unwrap_or({ let mut map: HashMap<String, DepylerValue>= HashMap::new();
    map
});
. to_string()).get(& axis).cloned();
    if(! is_banded) &&(is_binned.is_none()) {
    continue;
   
}
if let Some(ref is_binned_val) = is_binned {
    let bin_def = declaration.get("binnedAxes").cloned().unwrap_or({ let mut map: HashMap<String, DepylerValue>= HashMap::new();
    map
});
. to_string()).get(axis.to_i64() as usize).cloned().expect("IndexError: list index out of range");
    if {
    let _and_lhs = DepylerValue::from(true);
    if ! _and_lhs.is_true() {
    _and_lhs
}
else {
    bin_def.get("maxbins").cloned()
}
} {
    count = bin_def.get("maxbins").cloned().unwrap_or_default().into();
   
}
else {
    count = 10;
   
}
} else {
    count = table.iter().cloned().map(| r | r.get(& cs.get("field").cloned().unwrap_or_default()).cloned()).collect::<std::collections::HashSet<_>>().len() as i32;
   
}
if count <= 1 {
    continue;
   
}
if axis == STR_X {
    x_continuous_as_discrete = count;
   
}
else {
    y_continuous_as_discrete = count;
   
}
} facet_cols = 1;
    facet_rows = 1;
    if let Some(ref facet_grid_val) = facet_grid {
    facet_cols = facet_grid_val.get("columns").cloned().unwrap_or_default().into();
    facet_rows = facet_grid_val.get("rows").cloned().unwrap_or_default().into();
   
}
else {
    let _cse_temp_8 = nominal_count.get("column").cloned().unwrap_or_default()>0;
    if _cse_temp_8 {
    facet_cols = nominal_count.get("column").cloned().unwrap_or_default().into();
   
}
if _cse_temp_8 {
    facet_rows = nominal_count.get("row").cloned().unwrap_or_default().into();
   
}
} let LOG_PX_PER_DECADE = 40;
    log_boost_x = 0;
    log_boost_y = 0;
    for axis in vec! [STR_X.to_string(), STR_Y.to_string()] {
    cs = channel_semantics.get(& axis).cloned();
    if((cs.is_none()) ||(! cs.get("field").cloned())) ||(! cs.get("scaleType").cloned()) {
    continue;
   
}
if ! ["log", "symlog"].contains(&& cs.get("scaleType").cloned().unwrap_or_default()) {
    continue;
   
}
let vals: Vec<std::collections::HashMap<String, DepylerValue>>= table.iter().cloned().filter(| r | {
    let mut r = r.clone();
   (((true) &&(! true)) &&(r.get(& cs.get("field").cloned().unwrap_or_default()).cloned()>0)) &&((r.get(& cs.get("field").cloned().unwrap_or_default()).cloned() as f64).is_finite()) }).map(| r | r.get(& cs.get("field").cloned().unwrap_or_default()).cloned()).collect::<Vec<_>>();
    if(vals.len() as i32)<2 {
    continue;
   
}
let decades  = ((*vals.iter().max().expect("empty collection") as f64).log10()) - ((*vals.iter().min ().expect("empty collection") as f64).log10());
    let needed  = (((depyler_max((1).clone() ,(decades).clone()) as f64).ceil()).py_mul(LOG_PX_PER_DECADE)) as i32;
    if axis == STR_X {
    log_boost_x = needed;
   
}
else {
    log_boost_y = needed;
   
}
} let _cse_temp_9 = depyler_max((10).clone() ,(min_step_val).clone());
    let min_continuous_size = _cse_temp_9;
    let _cse_temp_10 = depyler_max((min_continuous_size).clone() ,(log_boost_x).clone());
    let min_continuous_size_x = _cse_temp_10;
    let _cse_temp_11 = depyler_max((min_continuous_size).clone() ,(log_boost_y).clone());
    let min_continuous_size_y = _cse_temp_11;
    let _cse_temp_12 = facet_cols>1;
    if _cse_temp_12 {
    let _cse_temp_13 = {
    if facet_elasticity_val>= 0 &&(facet_elasticity_val as i64)<= (u32::MAX as i64) {
   ({ facet_cols
}
as i32).checked_pow({ facet_elasticity_val
}
as u32).expect("Power operation overflowed") as f64
}
else {
   ({ facet_cols
}
as f64).powf({ facet_elasticity_val
}
as f64) as i32
}
};
    let _cse_temp_14 = depyler_min ((max_stretch_x).clone() ,(_cse_temp_13).clone());
    stretch = _cse_temp_14;
    subplot_width = js_round(depyler_max((min_continuous_size_x).clone() ,(((((((default_chart_width).py_mul(stretch) as f64)) - (fix_w) as f64)).join(facet_cols)) - (gap)).clone()));
   
}
else {
    subplot_width = default_chart_width;
   
}
let _cse_temp_15 = facet_rows>1;
    if _cse_temp_15 {
    let _cse_temp_16 = {
    if facet_elasticity_val>= 0 &&(facet_elasticity_val as i64)<= (u32::MAX as i64) {
   ({ facet_rows
}
as i32).checked_pow({ facet_elasticity_val
}
as u32).expect("Power operation overflowed") as f64
}
else {
   ({ facet_rows
}
as f64).powf({ facet_elasticity_val
}
as f64) as i32
}
};
    let _cse_temp_17 = depyler_min ((max_stretch_y).clone() ,(_cse_temp_16).clone());
    stretch = _cse_temp_17;
    subplot_height = js_round(depyler_max((min_continuous_size_y).clone() ,(((((((default_chart_height).py_mul(stretch) as f64)) - (fix_h) as f64)).join(facet_rows)) - (gap)).clone()));
   
}
else {
    subplot_height = default_chart_height;
   
}
let _cse_temp_18 = x_total_nominal_count == 0;
    let _cse_temp_19 = x_continuous_as_discrete == 0;
    let _cse_temp_20  = (_cse_temp_18) &&(_cse_temp_19);
    let x_is_continuous_non_banded = _cse_temp_20;
    let _cse_temp_21 = y_total_nominal_count == 0;
    let _cse_temp_22 = y_continuous_as_discrete == 0;
    let _cse_temp_23  = (_cse_temp_21) &&(_cse_temp_22);
    let y_is_continuous_non_banded = _cse_temp_23;
    let _cse_temp_24  = (x_is_continuous_non_banded) &&(y_is_continuous_non_banded);
    let both_continuous_non_banded = _cse_temp_24;
    let _cse_temp_25  = (facet_aspect_ratio_resistance).as_str()>0;
    let _cse_temp_26  = (_cse_temp_25) &&(! both_continuous_non_banded);
    let _cse_temp_27  = (_cse_temp_12) ||(_cse_temp_15);
    let _cse_temp_28  = (_cse_temp_26) &&(_cse_temp_27);
    if _cse_temp_28 {
    let _cse_temp_29  = (default_chart_width).join(default_chart_height);
    let base_ar = _cse_temp_29;
    let _cse_temp_30  = (subplot_width).join(subplot_height);
    let facet_ar = _cse_temp_30;
    let _cse_temp_31  = (facet_ar).join(base_ar);
    let ar_drift = _cse_temp_31;
    let _cse_temp_32 = ar_drift<1f64;
    if _cse_temp_32 {
    subplot_height = js_round(depyler_max((min_continuous_size_y).clone() ,((subplot_height).py_mul(({ ar_drift
}
as f64).powf({ facet_aspect_ratio_resistance
}
as f64))).clone()));
   
}
else {
    let _cse_temp_33 = ar_drift>1f64;
    if _cse_temp_33 {
    subplot_width = js_round(depyler_max((min_continuous_size_x).clone() ,((subplot_width).py_mul(({(1i32).join(ar_drift)
}
as f64).powf({ facet_aspect_ratio_resistance
}
as f64))).clone()));
   
}
}
}
let elastic_params_x: std::collections::HashMap<String, DepylerValue>= {
    let mut map: HashMap<String, DepylerValue>= HashMap::new();
    map.insert("elasticity".to_string(), DepylerValue::Str(elasticity_val.to_string()));
    map.insert("maxStretch".to_string(), DepylerValue::Str(format!("{:?}", max_stretch_x)));
    map.insert("defaultStepSize".to_string(), DepylerValue::Int(default_step_size as i64));
    map.insert(STR_MINSTEP.to_string(), DepylerValue::Str(min_step_val.to_string()));
    map };
    let elastic_params_y: std::collections::HashMap<String, DepylerValue>= {
    let mut map: HashMap<String, DepylerValue>= HashMap::new();
    map.insert("elasticity".to_string(), DepylerValue::Str(elasticity_val.to_string()));
    map.insert("maxStretch".to_string(), DepylerValue::Str(format!("{:?}", max_stretch_y)));
    map.insert("defaultStepSize".to_string(), DepylerValue::Int(default_step_size as i64));
    map.insert(STR_MINSTEP.to_string(), DepylerValue::Str(min_step_val.to_string()));
    map };
    let x_axis = compute_axis_step(x_total_nominal_count, x_continuous_as_discrete, subplot_width, & elastic_params_x) ?;
    let y_axis = compute_axis_step(y_total_nominal_count, y_continuous_as_discrete, subplot_height, & elastic_params_y) ?;
    let _cse_temp_34 = x_total_nominal_count>0;
    let x_is_discrete = _cse_temp_34;
    let _cse_temp_35 = y_total_nominal_count>0;
    let y_is_discrete = _cse_temp_35;
    let _cse_temp_36 = group_axis == STR_X;
    let _cse_temp_37 = nominal_count.get("group").cloned().unwrap_or_default()>0;
    let _cse_temp_38  = (_cse_temp_36) &&(_cse_temp_37);
    let x_has_grouping = _cse_temp_38;
    let _cse_temp_39 = group_axis == STR_Y;
    let _cse_temp_40  = (_cse_temp_39) &&(_cse_temp_37);
    let y_has_grouping = _cse_temp_40;
    let _cse_temp_41  = (x_is_discrete) &&(x_has_grouping);
    if _cse_temp_41 {
    items_per_group = nominal_count.get("group").cloned().unwrap_or_default().into();
    let _cse_temp_42  = ((items_per_group).py_mul(default_step_size)) as i32;
    default_group_step = _cse_temp_42;
    let _cse_temp_43  = ((2i32).py_mul(items_per_group)) as i32;
    let _cse_temp_44 = depyler_max((((MIN_GROUP_GAP_PX).join(step_padding_val) as f64).ceil()).clone() ,(_cse_temp_43).clone());
    min_group_step = _cse_temp_44;
    group_axis_step = compute_axis_step(nominal_count.get("x").cloned().unwrap_or_default(), 0, subplot_width, & elastic_params_x) ?;
    let _cse_temp_45 = depyler_min ((default_group_step).clone() ,(group_axis_step.get("step").cloned().unwrap_or_default()).clone());
    let _cse_temp_46 = depyler_max((min_group_step).clone() ,(_cse_temp_45).clone());
    x_step_size = _cse_temp_46;
    x_step_unit = STR_GROUP.to_string();
   
}
else {
    if x_is_discrete {
    let _cse_temp_47 = depyler_min ((default_step_size).clone() ,(x_axis.get("step").cloned().unwrap_or_default()).clone());
    let _cse_temp_48 = depyler_max((min_step_val).clone() ,(_cse_temp_47).clone());
    x_step_size = _cse_temp_48;
   
}
else {
    let _cse_temp_49 = x_continuous_as_discrete>0;
    if _cse_temp_49 {
    let _cse_temp_50 = depyler_min ((default_step_size).clone() ,(x_axis.get("step").cloned().unwrap_or_default()).clone());
    let _cse_temp_51 = depyler_max((min_step_val).clone() ,(_cse_temp_50).clone());
    x_step_size = _cse_temp_51;
   
}
else {
    x_step_size = default_step_size;
   
}
}
}
let _cse_temp_52  = (y_is_discrete) &&(y_has_grouping);
    if _cse_temp_52 {
    items_per_group = nominal_count.get("group").cloned().unwrap_or_default().into();
    let _cse_temp_53  = ((items_per_group).py_mul(default_step_size)) as i32;
    default_group_step = _cse_temp_53;
    let _cse_temp_54  = ((2i32).py_mul(items_per_group)) as i32;
    let _cse_temp_55 = depyler_max((((MIN_GROUP_GAP_PX).join(step_padding_val) as f64).ceil()).clone() ,(_cse_temp_54).clone());
    min_group_step = _cse_temp_55;
    group_axis_step = compute_axis_step(nominal_count.get("y").cloned().unwrap_or_default(), 0, subplot_height, & elastic_params_y) ?;
    let _cse_temp_56 = depyler_min ((default_group_step).clone() ,(group_axis_step.get("step").cloned().unwrap_or_default()).clone());
    let _cse_temp_57 = depyler_max((min_group_step).clone() ,(_cse_temp_56).clone());
    y_step_size = _cse_temp_57;
    y_step_unit = STR_GROUP.to_string();
   
}
else {
    if y_is_discrete {
    let _cse_temp_58 = depyler_min ((default_step_size).clone() ,(y_axis.get("step").cloned().unwrap_or_default()).clone());
    let _cse_temp_59 = depyler_max((min_step_val).clone() ,(_cse_temp_58).clone());
    y_step_size = _cse_temp_59;
   
}
else {
    let _cse_temp_60 = y_continuous_as_discrete>0;
    if _cse_temp_60 {
    let _cse_temp_61 = depyler_min ((default_step_size).clone() ,(y_axis.get("step").cloned().unwrap_or_default()).clone());
    let _cse_temp_62 = depyler_max((min_step_val).clone() ,(_cse_temp_61).clone());
    y_step_size = _cse_temp_62;
   
}
else {
    y_step_size = default_step_size;
   
}
}
}
for axis in vec! [STR_X.to_string(), STR_Y.to_string()] {
    count = if axis == STR_X {
    x_continuous_as_discrete
}
else {
    y_continuous_as_discrete };
    if count <= 0 {
    continue;
   
}
step_size = if axis == STR_X {
    x_step_size
}
else {
    y_step_size };
    let continuous_size = js_round((step_size).py_mul((count).py_add(1i32)));
    if axis == STR_X {
    subplot_width = continuous_size;
   
}
else {
    subplot_height = continuous_size;
   
}
} let _cse_temp_63  = (default_chart_width).py_mul(max_stretch_x);
    let _cse_temp_64  = (((_cse_temp_63) - (fix_w) as f64)).join(facet_cols);
    let max_subplot_w  = (_cse_temp_64) - (gap);
    let _cse_temp_65  = (default_chart_height).py_mul(max_stretch_y);
    let _cse_temp_66  = (((_cse_temp_65) - (fix_h) as f64)).join(facet_rows);
    let max_subplot_h  = (_cse_temp_66) - (gap);
    if _cse_temp_34 {
    divisor = if x_step_unit == STR_GROUP {
    nominal_count.get("x").cloned().unwrap_or_default()
}
else {
    x_total_nominal_count };
    let _cse_temp_67 = depyler_max((min_step_val).clone() ,(((max_subplot_w).join(divisor) as f64).floor()).clone());
    cap = _cse_temp_67;
    let _cse_temp_68 = x_step_size>cap;
    if _cse_temp_68 {
    x_step_size = cap;
   
}
} let _cse_temp_69 = x_continuous_as_discrete>0;
    if _cse_temp_69 {
    let _cse_temp_70 = depyler_max((min_step_val).clone() ,(((max_subplot_w).join((x_continuous_as_discrete).py_add(1i32)) as f64).floor()).clone());
    cap = _cse_temp_70;
    let _cse_temp_71 = x_step_size>cap;
    if _cse_temp_71 {
    x_step_size = cap;
   
}
} if _cse_temp_35 {
    divisor = if y_step_unit == STR_GROUP {
    nominal_count.get("y").cloned().unwrap_or_default()
}
else {
    y_total_nominal_count };
    let _cse_temp_72 = depyler_max((min_step_val).clone() ,(((max_subplot_h).join(divisor) as f64).floor()).clone());
    cap = _cse_temp_72;
    let _cse_temp_73 = y_step_size>cap;
    if _cse_temp_73 {
    y_step_size = cap;
   
}
} let _cse_temp_74 = y_continuous_as_discrete>0;
    if _cse_temp_74 {
    let _cse_temp_75 = depyler_max((min_step_val).clone() ,(((max_subplot_h).join((y_continuous_as_discrete).py_add(1i32)) as f64).floor()).clone());
    cap = _cse_temp_75;
    let _cse_temp_76 = y_step_size>cap;
    if _cse_temp_76 {
    y_step_size = cap;
   
}
} for axis in vec! [STR_X.to_string(), STR_Y.to_string()] {
    count = if axis == STR_X {
    x_continuous_as_discrete
}
else {
    y_continuous_as_discrete };
    if count <= 0 {
    continue;
   
}
step_size = if axis == STR_X {
    x_step_size
}
else {
    y_step_size };
    if axis == STR_X {
    subplot_width = js_round((step_size).py_mul((count).py_add(1i32)));
   
}
else {
    subplot_height = js_round((step_size).py_mul((count).py_add(1i32)));
   
}
} let _cse_temp_77 = depyler_min ((subplot_width).clone() ,(js_round(max_subplot_w)).clone());
    subplot_width = _cse_temp_77;
    let _cse_temp_78 = depyler_min ((subplot_height).clone() ,(js_round(max_subplot_h)).clone());
    subplot_height = _cse_temp_78;
    let target_band_ar = options.get("targetBandAR").cloned();
    let _cse_temp_79 = target_band_ar.unwrap_or_default()>0;
    let _cse_temp_80  = (target_band_ar.is_some()) &&(_cse_temp_79);
    if _cse_temp_80 {
    let _cse_temp_81  = (_cse_temp_34) ||(_cse_temp_69);
    let x_is_banded_eff = _cse_temp_81;
    let _cse_temp_82  = (_cse_temp_35) ||(_cse_temp_74);
    let y_is_banded_eff = _cse_temp_82;
    let _cse_temp_83  = (x_is_banded_eff) &&(! y_is_banded_eff);
    let mut actual_band_ar;
    if _cse_temp_83 {
    let _cse_temp_84  = ((subplot_height).join(x_step_size)) as i32;
    actual_band_ar = _cse_temp_84;
    let _cse_temp_85 = actual_band_ar>target_band_ar.unwrap_or(i32::MIN);
    if _cse_temp_85 {
    let _cse_temp_86  = ((x_step_size).py_mul(target_band_ar)) as i32;
    let ideal_h = _cse_temp_86;
    let blended_h  = ((((0.5).py_mul((subplot_height as f64).ln()) as f64)).py_add((0.5).py_mul((ideal_h as f64).ln())) as f64).exp();
    subplot_height = js_round(depyler_max((min_continuous_size_y).clone() ,(depyler_min ((blended_h).clone() ,(subplot_height).clone())).clone()));
   
}
} else {
    let _cse_temp_87  = (y_is_banded_eff) &&(! x_is_banded_eff);
    if _cse_temp_87 {
    let _cse_temp_88  = ((subplot_width).join(y_step_size)) as i32;
    actual_band_ar = _cse_temp_88;
    let _cse_temp_89 = actual_band_ar>target_band_ar.unwrap_or(i32::MIN);
    if _cse_temp_89 {
    let _cse_temp_90  = ((y_step_size).py_mul(target_band_ar)) as i32;
    let ideal_w = _cse_temp_90;
    let blended_w  = ((((0.5).py_mul((subplot_width as f64).ln()) as f64)).py_add((0.5).py_mul((ideal_w as f64).ln())) as f64).exp();
    subplot_width = js_round(depyler_max((min_continuous_size_x).clone() ,(depyler_min ((blended_w).clone() ,(subplot_width).clone())).clone()));
   
}
}
}
} let x_has_discrete_items = _cse_temp_34;
    let y_has_discrete_items = _cse_temp_35;
    let x_label = compute_label_sizing(x_step_size, x_has_discrete_items);
    let y_label = compute_label_sizing(y_step_size, y_has_discrete_items);
    let mut result: std::collections::HashMap<String, DepylerValue>= {
    let mut map: HashMap<String, DepylerValue>= HashMap::new();
    map.insert("subplotWidth".to_string(), DepylerValue::Int(subplot_width as i64));
    map.insert("subplotHeight".to_string(), DepylerValue::Int(subplot_height as i64));
    map.insert("xStep".to_string(), DepylerValue::Int(x_step_size as i64));
    map.insert("yStep".to_string(), DepylerValue::Int(y_step_size as i64));
    map.insert("xStepUnit".to_string(), DepylerValue::Str(x_step_unit.to_string()));
    map.insert("yStepUnit".to_string(), DepylerValue::Str(y_step_unit.to_string()));
    map.insert("xContinuousAsDiscrete".to_string(), DepylerValue::Int(x_continuous_as_discrete as i64));
    map.insert("yContinuousAsDiscrete".to_string(), DepylerValue::Int(y_continuous_as_discrete as i64));
    map.insert("xNominalCount".to_string(), DepylerValue::Int(x_total_nominal_count as i64));
    map.insert("yNominalCount".to_string(), DepylerValue::Int(y_total_nominal_count as i64));
    map.insert("xLabel".to_string(), DepylerValue::Str(format!("{:?}", x_label)));
    map.insert("yLabel".to_string(), DepylerValue::Str(format!("{:?}", y_label)));
    map.insert("stepPadding".to_string(), DepylerValue::Str(step_padding_val.to_string()));
    map.insert("effectiveFacetGap".to_string(), DepylerValue::Str(gap.to_string()));
    map.insert("truncations".to_string(), DepylerValue::List(vec! []));
    map };
    if _cse_temp_27 {
    result.insert("facet".to_string(), format!("{:?}", {
    let mut map: HashMap<String, DepylerValue>= HashMap::new();
    map.insert(STR_COLUMNS.to_string(), DepylerValue::Int(facet_cols as i64));
    map.insert(STR_ROWS.to_string(), DepylerValue::Int(facet_rows as i64));
    map.insert("subplotWidth".to_string(), DepylerValue::Int(subplot_width as i64));
    map.insert("subplotHeight".to_string(), DepylerValue::Int(subplot_height as i64));
    map }));
   
}
else {
    result.insert("facet".to_string(), None);
   
}
Ok(result)
}
#[doc = " Depyler: verified panic-free"] pub fn count_distinct_series<'b, 'a>(channel_semantics: & 'a std::collections::HashMap<String, DepylerValue>, data: & 'b Vec<std::collections::HashMap<String, DepylerValue>>) -> i32 {
    let mut series_fields: Vec<String>= vec! [];
    let color_field = channel_semantics.get("color").cloned().unwrap_or({ let mut map: HashMap<String, String>= HashMap::new();
    map
}
. to_string()).get("field").cloned();
    let detail_field = channel_semantics.get("detail").cloned().unwrap_or({ let mut map: HashMap<String, String>= HashMap::new();
    map
}
. to_string()).get("field").cloned();
    if let Some(ref color_field_val) = color_field {
    series_fields.append(color_field_val);
   
}
let _cse_temp_0 = detail_field != color_field;
    let _cse_temp_1  = (detail_field.is_some()) &&(_cse_temp_0);
    if _cse_temp_1 {
    series_fields.append(detail_field);
   
}
if series_fields.is_empty() {
    return 1;
   
}
let mut series_keys: std::collections::HashSet<i32>= std::collections::HashSet::<i32>::new();
    for row in data.iter().cloned() {
    let key = series_fields.iter().cloned().map(| f |(if row.get(& f).cloned().is_some() {
    row.get(& f).cloned()
}
else {
    STR_EMPTY }).to_string()).collect::<Vec<_>>().join ("\0");
    series_keys.insert(key);
   
}
series_keys.len() as i32 as i32
}
pub fn compute_banking_ar<'l2, 'a, 'l1, 'c, 'b>(x_values: & 'a Vec<f64>, y_values: & 'b Vec<f64>, x_domain: & 'c Vec<f64>, y_domain: & 'l1 Vec<f64>, series_keys: & 'l2 Vec<String>, is_connected: bool) -> Result<f64, Box<dyn std::error::Error>>{
    let mut max_series_len: i32 = Default::default();
    let mut n: i32 = Default::default();
    let mut median: f64 = Default::default();
    let mut ar: i32 = Default::default();
    let mut arr: Vec<DepylerValue>= Default::default();
    let MIN_AR = 0.5;
    let MAX_AR = 3.0;
    let _cse_temp_0  = (x_domain.get(1usize).cloned().expect("IndexError: list index out of range")) - (x_domain.get(0usize).cloned().expect("IndexError: list index out of range"));
    let x_range = _cse_temp_0;
    let y_range = _cse_temp_0;
    let _cse_temp_1 = x_range <= 0;
    let _cse_temp_2 = y_range <= 0;
    let _cse_temp_3  = (_cse_temp_1) ||(_cse_temp_2);
    if _cse_temp_3 {
    return Ok(1);
   
}
if ! is_connected {
    let _cse_temp_4 = x_values.len() as i32;
    n = _cse_temp_4;
    let _cse_temp_5  = (0..(n)).into_iter().map(| i |(((x_values.get(i.to_i64() as usize).cloned().expect("IndexError: list index out of range")) - (x_domain.get(0usize).cloned().expect("IndexError: list index out of range")) as f64)).join(x_range)).sum::<f64>();
    let sum_x = _cse_temp_5;
    let sum_y = _cse_temp_5;
    let _cse_temp_6  = ((sum_x).join(n)) as i32;
    let mean_x = _cse_temp_6;
    let _cse_temp_7  = ((sum_y).join(n)) as i32;
    let mean_y = _cse_temp_7;
    let mut var_x = 0.0;
    let mut var_y = 0.0;
    for i in 0..(n) {
    let mut dx  = (((((x_values.get(i as usize).cloned().expect("IndexError: list index out of range")) - (x_domain.get(0usize).cloned().expect("IndexError: list index out of range")) as f64)).join(x_range) as f64)) - (mean_x);
    let mut dy  = (((((y_values.get(i as usize).cloned().expect("IndexError: list index out of range")) - (y_domain.get(0usize).cloned().expect("IndexError: list index out of range")) as f64)).join(y_range) as f64)) - (mean_y);
    var_x  = (var_x).py_add((dx).py_mul(dx));
    var_y  = (var_y).py_add((dy).py_mul(dy));
   
}
let sd_x  = ((var_x).join(n) as f64).sqrt();
    let sd_y  = ((var_y).join(n) as f64).sqrt();
    let _cse_temp_8 = sd_y <= 0;
    if _cse_temp_8 {
    return Ok(MAX_AR);
   
}
let _cse_temp_9 = sd_x <= 0;
    if _cse_temp_9 {
    return Ok(MIN_AR);
   
}
let _cse_temp_10  = (sd_x).join(sd_y);
    let sd_ratio = _cse_temp_10;
    let _cse_temp_11 = sd_ratio>1;
    if _cse_temp_11 {
    let _cse_temp_12  = (((sd_ratio) - (1i32) as f64)).py_mul(0.3);
    ar  = (1i32).py_add(_cse_temp_12);
   
}
else {
    let _cse_temp_13  = (((1i32) - (sd_ratio) as f64)).py_mul(0.3);
    ar  = (1i32) - (_cse_temp_13);
   
}
return Ok(depyler_min ((MAX_AR).clone() ,(depyler_max((MIN_AR).clone() ,(ar).clone())).clone()));
   
}
let mut series_map: std::collections::HashMap<String, Vec<std::collections::HashMap<String, f64>>>= {
    let mut map: HashMap<String, Vec<std::collections::HashMap<String, f64>>>= HashMap::new();
    map };
    for i in 0..(x_values.len() as i32) {
    let key = series_keys.get(i as usize).cloned().expect("IndexError: list index out of range");
    arr = series_map.get(& key).cloned();
    if false {
    arr = Some(vec! []);
    series_map.insert(key.to_string().clone(), arr);
   
}
arr.append({ let mut map = HashMap::new();
    map.insert(STR_X.to_string(), x_values.get(i as usize).cloned().expect("IndexError: list index out of range"));
    map.insert(STR_Y.to_string(), y_values.get(i as usize).cloned().expect("IndexError: list index out of range"));
    map });
   
}
for pts in series_map.values() {
    pts.sort_by_key(| x | move | p: i32 | p.get_str("x").cloned().unwrap_or_default().into()(x));
   
}
let mut scale_medians: Vec<f64>= vec! [];
    max_series_len = 0;
    for pts in series_map.values() {
    if pts.len() as i32>max_series_len {
    max_series_len = pts.len() as i32;
   
}
} let _cse_temp_14 = max_series_len <= 0;
    if _cse_temp_14 {
    return Ok(1);
   
}
let _cse_temp_15  = ((((max_series_len as f64).log2() as f64).floor()) - (1i32)) as i32;
    let _cse_temp_16 = depyler_max((0).clone() ,(_cse_temp_15).clone());
    let max_scale = _cse_temp_16;
    for scale in 0..((max_scale).py_add(1i32)) {
    let window_size = 1 <<scale;
    let mut abs_slopes: Vec<f64>= vec! [];
    for pts in series_map.values() {
    n = pts.len() as i32;
    if n<2 {
    continue;
   
}
let mut smoothed: Vec<std::collections::HashMap<String, f64>>= vec! [];
    for i in {
    let step = window_size as usize;
    if step == 0 {
    panic!("range() arg 3 must not be zero");
   
}
(0..n).step_by(step)
}
    let end = depyler_min (((i).py_add(window_size)).clone() ,(n).clone());
{
    let mut sx = 0.0;
    let mut sy = 0.0;
    for j in (i)..(end) {
    sx  = (sx).py_add(pts.get(j as usize).cloned().expect("IndexError: list index out of range").get_str("x").cloned().unwrap_or_default().into());
    sy  = (sy).py_add(pts.get(j as usize).cloned().expect("IndexError: list index out of range").get_str("y").cloned().unwrap_or_default().into());
   
}
let cnt  = ((end) - (i)) as i32;
    smoothed.append({ let mut map = HashMap::new();
    map.insert(STR_X.to_string() ,(sx).join(cnt));
    map.insert(STR_Y.to_string() ,(sy).join(cnt));
    map });
   
}
for i in (1)..(smoothed.len() as i32) {
    let mut dx  = (((smoothed.get(i as usize).cloned().expect("IndexError: list index out of range").get("x").cloned().unwrap_or_default()) - ({ let base = & smoothed;
    let idx: i32  = (i) - (1i32);
    let actual_idx = if idx<0 {
    base.len().saturating_sub(idx.abs() as usize)
}
else {
    idx as usize };
    base.get(actual_idx).cloned().expect("IndexError: list index out of range")
}
. get("x").cloned().unwrap_or_default()) as f64)).join(x_range);
    let mut dy  = (((smoothed.get(i as usize).cloned().expect("IndexError: list index out of range").get("y").cloned().unwrap_or_default()) - ({ let base = & smoothed;
    let idx: i32  = (i) - (1i32);
    let actual_idx = if idx<0 {
    base.len().saturating_sub(idx.abs() as usize)
}
else {
    idx as usize };
    base.get(actual_idx).cloned().expect("IndexError: list index out of range")
}
. get("y").cloned().unwrap_or_default()) as f64)).join(y_range);
    if dx == 0f64 {
    continue;
   
}
abs_slopes.append(((dy).join(dx)).abs());
   
}
} if abs_slopes.is_empty() {
    continue;
   
}
abs_slopes.sort();
    let mid = abs_slopes.len() as i32>>1;
    if(abs_slopes.len() as i32).py_mod(2i32) == 1 {
    median = abs_slopes.get(mid as usize).cloned().expect("IndexError: list index out of range").into();
   
}
else {
    median  = ((({ let base = & abs_slopes;
    let idx: i32  = (mid) - (1i32);
    let actual_idx = if idx<0 {
    base.len().saturating_sub(idx.abs() as usize)
}
else {
    idx as usize };
    base.get(actual_idx).cloned().expect("IndexError: list index out of range") }).py_add(abs_slopes.get(mid as usize).cloned().expect("IndexError: list index out of range")) as f64)).join(2i32);
   
}
if median>0f64 {
    scale_medians.append(median);
   
}
} if scale_medians.is_empty() {
    return Ok(1);
   
}
let _cse_temp_17 = scale_medians.as_slice().iter().cloned().map(| m |(m as f64).ln()).sum::<f64>();
    let log_sum = _cse_temp_17;
    let combined_slope  = ((log_sum).join(scale_medians.len() as i32) as f64).exp();
    let _cse_temp_18 = combined_slope <= 0;
    if _cse_temp_18 {
    return Ok(MAX_AR);
   
}
let _cse_temp_19  = (1.0 as f64).max(combined_slope as f64);
    ar = _cse_temp_19;
    Ok(depyler_min ((MAX_AR).clone() ,(depyler_max((MIN_AR).clone() ,(ar).clone())).clone()))
}
#[doc = " Depyler: proven to terminate"] pub fn compute_channel_budgets<'l2, 'a, 'b, 'l1, 'c>(channel_semantics: & 'a std::collections::HashMap<String, std::collections::HashMap<String, DepylerValue>>, declaration: & 'b std::collections::HashMap<String, DepylerValue>, data: & 'c Vec<std::collections::HashMap<String, DepylerValue>>, canvas_size: & 'l1 std::collections::HashMap<String, f64>, options: & 'l2 std::collections::HashMap<String, DepylerValue>) -> Result<std::collections::HashMap<String, DepylerValue>, Box<dyn std::error::Error>>{
    let mut max_x_to_keep: DepylerValue = Default::default();
    let mut max_y_to_keep: DepylerValue = Default::default();
    let mut group_axis: Option<String>= Default::default();
    let mut group_count: i32 = Default::default();
    let(max_stretch_x, max_stretch_y) = resolve_stretch_caps(& options);
    let min_step_val = options.get("minStep").cloned().unwrap_or(DepylerValue::Int(6i64));
    let step_padding_val = options.get("stepPadding").cloned().unwrap_or(0.1);
    let max_color_val = options.get("maxColorValues").cloned().unwrap_or(DepylerValue::Int(24i64));
    let _cse_temp_0 = options.get("facetFixedPadding").cloned().unwrap_or({ let mut map: HashMap<String, DepylerValue>= HashMap::new();
    map
});
. to_string());
    let facet_fixed_padding = _cse_temp_0;
    let fix_w = facet_fixed_padding.get("width").cloned().unwrap_or(DepylerValue::Int(0i64));
    let fix_h = facet_fixed_padding.get("height").cloned().unwrap_or(DepylerValue::Int(0i64));
    let gap = options.get("facetGap").cloned().unwrap_or(DepylerValue::Int(0i64));
    fn is_discrete_type(t: Option<String>) -> bool {
    return(t == STR_NOMINAL) ||(t == STR_ORDINAL);
   
}
fn effective_type(ch: & str) -> Option<String>{
    let rt = declaration.get("resolvedTypes").cloned().unwrap_or({ let mut map: HashMap<String, String>= HashMap::new();
    map
}
. to_string()).get(& ch).cloned();
    return Some(if rt.is_some() {
    rt
}
else {
    channel_semantics.get(& ch).cloned().unwrap_or({ let mut map: HashMap<String, String>= HashMap::new();
    map
}
. to_string()).get("type").cloned() });
   
}
let mut facet_grid = compute_facet_grid(& channel_semantics, & declaration, & data, & canvas_size, & options) ?;
    let facet_cols = if facet_grid.is_some() {
    facet_grid.get("columns").cloned().unwrap_or_default()
}
else {
    1 };
    let facet_rows = if facet_grid.is_some() {
    facet_grid.get("rows").cloned().unwrap_or_default()
}
else {
    1 };
    let _cse_temp_1  = (canvas_size.get("width").cloned().unwrap_or_default()).py_mul(max_stretch_x);
    let _cse_temp_2  = (((_cse_temp_1) - (fix_w) as f64)).join(facet_cols);
    let _cse_temp_3 = depyler_max((options.get("minSubplotSize").cloned().unwrap_or(DepylerValue::Int(60i64))).clone() ,((_cse_temp_2) - (gap)).clone());
    let max_subplot_w = _cse_temp_3;
    let _cse_temp_4  = (canvas_size.get("height").cloned().unwrap_or_default()).py_mul(max_stretch_y);
    let _cse_temp_5  = (((_cse_temp_4) - (fix_h) as f64)).join(facet_rows);
    let _cse_temp_6 = depyler_max((options.get("minSubplotSize").cloned().unwrap_or(DepylerValue::Int(60i64))).clone() ,((_cse_temp_5) - (gap)).clone());
    let max_subplot_h = _cse_temp_6;
    let group_field = channel_semantics.get("group").cloned().unwrap_or({ let mut map: HashMap<String, DepylerValue>= HashMap::new();
    map
});
. to_string()).get("field").cloned();
    group_count = 0;
    if let Some(ref group_field_val) = group_field {
    let _cse_temp_7 = data.iter().cloned().map(| r | r.get(& group_field_val).cloned()).collect::<std::collections::HashSet<_>>().len() as i32;
    group_count = _cse_temp_7;
    if is_discrete_type(effective_type(STR_X)) {
    group_axis = STR_X.to_string();
   
}
else {
    if is_discrete_type(effective_type(STR_Y)) {
    group_axis = STR_Y.to_string();
   
}
}
}
let x_group_multiplier = if(group_axis == STR_X) &&(group_count>1) {
    group_count
}
else {
    1 };
    let y_group_multiplier = if(group_axis == STR_Y) &&(group_count>1) {
    group_count
}
else {
    1 };
    let MIN_GROUP_GAP_PX = 3;
    let x_min_group_step = if x_group_multiplier>1 {
    depyler_max((((MIN_GROUP_GAP_PX).join(step_padding_val) as f64).ceil()).clone() ,((2i32).py_mul(x_group_multiplier)).clone())
}
else {
    min_step_val };
    let y_min_group_step = if y_group_multiplier>1 {
    depyler_max((((MIN_GROUP_GAP_PX).join(step_padding_val) as f64).ceil()).clone() ,((2i32).py_mul(y_group_multiplier)).clone())
}
else {
    min_step_val };
    max_x_to_keep  = ((max_subplot_w).join(x_min_group_step) as f64).floor();
    max_y_to_keep  = ((max_subplot_h).join(y_min_group_step) as f64).floor();
    if let Some(ref facet_grid_val) = facet_grid {
    let _cse_temp_8 = depyler_max((1).clone() ,(((canvas_size.get("width").cloned().unwrap_or_default()).join(x_min_group_step) as f64).floor()).clone());
    let canvas_x_cap = _cse_temp_8;
    let canvas_y_cap = _cse_temp_8;
    let _cse_temp_9 = max_x_to_keep>canvas_x_cap;
    let _cse_temp_10 = max_y_to_keep>canvas_y_cap;
    let _cse_temp_11  = (_cse_temp_9) ||(_cse_temp_10);
    if _cse_temp_11 {
    let _cse_temp_12 = depyler_min ((max_x_to_keep).clone() ,(canvas_x_cap).clone());
    max_x_to_keep = _cse_temp_12;
    let _cse_temp_13 = depyler_min ((max_y_to_keep).clone() ,(canvas_y_cap).clone());
    max_y_to_keep = _cse_temp_13;
    let col_field = channel_semantics.get("column").cloned().unwrap_or({ let mut map: HashMap<String, DepylerValue>= HashMap::new();
    map
});
. to_string()).get("field").cloned();
    let row_field = channel_semantics.get("row").cloned().unwrap_or({ let mut map: HashMap<String, DepylerValue>= HashMap::new();
    map
});
. to_string()).get("field").cloned();
    let col_count = if col_field.is_some() {
    data.iter().cloned().map(| r | r.get(& col_field).cloned()).collect::<std::collections::HashSet<_>>().len() as i32
}
else {
    0 };
    let _cse_temp_14 = col_count>1;
    let _cse_temp_15  = (_cse_temp_14) &&(false);
    if _cse_temp_15 {
    let _cse_temp_16  = ((max_x_to_keep).py_mul(x_min_group_step)) as i32;
    let _cse_temp_17 = depyler_max((options.get("minSubplotSize").cloned().unwrap_or(DepylerValue::Int(60i64))).clone() ,(_cse_temp_16).clone());
    let tighter_w = _cse_temp_17;
    let total_w  = (_cse_temp_1) - (fix_w);
    let total_h  = (_cse_temp_4) - (fix_h);
    let revised_max_cols = _cse_temp_8;
    let revised_max_rows = _cse_temp_8;
    let _cse_temp_18  = ((revised_max_cols).py_mul(revised_max_rows)) as i32;
    let max_total = _cse_temp_18;
    let _cse_temp_19 = depyler_min ((col_count).clone() ,(max_total).clone());
    let effective_count = _cse_temp_19;
    let vis_rows  = ((effective_count).join(revised_max_cols) as f64).ceil();
    let vis_cols  = ((effective_count).join(vis_rows) as f64).ceil();
    facet_grid_val.insert(STR_COLUMNS.to_string(), vis_cols);
    facet_grid_val.insert(STR_ROWS.to_string(), vis_rows);
    facet_grid_val.insert("maxColumnValues".to_string(), max_total);
   
}
}
}
let mut max_values: std::collections::HashMap<String, DepylerValue>= {
    let mut map: HashMap<String, DepylerValue>= HashMap::new();
    map.insert(STR_X.to_string(), DepylerValue::Int(max_x_to_keep as i64));
    map.insert(STR_Y.to_string(), DepylerValue::Int(max_y_to_keep as i64));
    map.insert(STR_COLUMN.to_string(), DepylerValue::Str(format!("{:?}", if facet_grid.is_some() {
    facet_grid.as_ref().map(| facet_grid_val | facet_grid_val.get("maxColumnValues").cloned())
}
else {
    f64::INFINITY })));
    map.insert(STR_ROW.to_string(), DepylerValue::Str(format!("{:?}", if facet_grid.is_some() {
    facet_grid.as_ref().map(| facet_grid_val | facet_grid_val.get("maxRowValues").cloned())
}
else {
    f64::INFINITY })));
    map.insert(STR_COLOR.to_string(), DepylerValue::Str(format!("{:?}", max_color_val)));
    map };
    if max_values.get("column").cloned().unwrap_or_default().is_none() {
    max_values.insert(STR_COLUMN.to_string(), f64::INFINITY);
   
}
if max_values.get("row").cloned().unwrap_or_default().is_none() {
    max_values.insert(STR_ROW.to_string(), f64::INFINITY);
   
}
Ok({ let mut map: HashMap<String, DepylerValue>= HashMap::new();
    map.insert("maxValues".to_string(), DepylerValue::Str(format!("{:?}", max_values)));
    map.insert("facetGrid".to_string(), DepylerValue::Str(format!("{:?}", facet_grid)));
    map })
}
#[doc = " Depyler: proven to terminate"] pub fn compute_facet_grid<'l2, 'a, 'b, 'l1, 'c>(channel_semantics: & 'a std::collections::HashMap<String, std::collections::HashMap<String, DepylerValue>>, declaration: & 'b std::collections::HashMap<String, DepylerValue>, data: & 'c Vec<std::collections::HashMap<String, DepylerValue>>, canvas_size: & 'l1 std::collections::HashMap<String, f64>, options: & 'l2 std::collections::HashMap<String, DepylerValue>) -> Result<Option<std::collections::HashMap<String, DepylerValue>>, Box<dyn std::error::Error>>{
    let mut min_subplot_height: i32 = Default::default();
    let mut min_subplot_width: i32 = Default::default();
    let mut cs: DepylerValue = Default::default();
    let mut per_category_step: i32 = Default::default();
    let mut group_axis: Option<String>= Default::default();
    let mut group_count: i32 = Default::default();
    let(ms_x, ms_y) = resolve_stretch_caps(& options);
    let _cse_temp_0 = options.get("facetFixedPadding").cloned().unwrap_or({ let mut map: HashMap<String, String>= HashMap::new();
    map
}
. to_string());
    let facet_fixed_padding = _cse_temp_0;
    let fix_w = facet_fixed_padding.get("width").cloned().unwrap_or(DepylerValue::Int(0i64));
    let fix_h = facet_fixed_padding.get("height").cloned().unwrap_or(DepylerValue::Int(0i64));
    let gap = options.get("facetGap").cloned().unwrap_or(DepylerValue::Int(0i64));
    let min_step = options.get("minStep").cloned().unwrap_or(DepylerValue::Int(6i64));
    let step_padding = options.get("stepPadding").cloned().unwrap_or(0.1);
    let base_min_subplot = options.get("minSubplotSize").cloned().unwrap_or(DepylerValue::Int(60i64));
    fn is_discrete_type(t: Option<String>) -> bool {
    return(t == STR_NOMINAL) ||(t == STR_ORDINAL);
   
}
let _cse_temp_1  = (canvas_size.get("width").cloned().unwrap_or_default()).py_mul(ms_x);
    let max_w  = (_cse_temp_1) - (fix_w);
    let _cse_temp_2  = (canvas_size.get("height").cloned().unwrap_or_default()).py_mul(ms_y);
    let max_h  = (_cse_temp_2) - (fix_h);
    let MIN_GROUP_GAP_PX = 3;
    let group_field = channel_semantics.get("group").cloned().unwrap_or({ let mut map: HashMap<String, String>= HashMap::new();
    map
}
. to_string()).get("field").cloned();
    group_count = 0;
    if let Some(ref group_field_val) = group_field {
    let _cse_temp_3 = data.iter().cloned().map(| r | r.get(& group_field_val).cloned()).collect::<std::collections::HashSet<_>>().len() as i32;
    group_count = _cse_temp_3;
    let _cse_temp_4 = declaration.get("resolvedTypes").cloned().unwrap_or({ let mut map: HashMap<String, String>= HashMap::new();
    map
}
. to_string()).get("x").cloned().unwrap_or(channel_semantics.get("x").cloned().unwrap_or({ let mut map: HashMap<String, String>= HashMap::new();
    map
}
. to_string()).get("type").cloned().to_string());
    let x_type = _cse_temp_4;
    let y_type = _cse_temp_4;
    if is_discrete_type(x_type) {
    group_axis = STR_X.to_string();
   
}
else {
    if is_discrete_type(y_type) {
    group_axis = STR_Y.to_string();
   
}
}
}
min_subplot_width = base_min_subplot.clone().into();
    min_subplot_height = base_min_subplot.clone().into();
    let LOG_PX_PER_DECADE_FACET = 40;
    for axis in vec! [STR_X.to_string(), STR_Y.to_string()] {
    cs = channel_semantics.get(& axis).cloned();
    if((cs.is_none()) ||(! cs.get("field").cloned())) ||(! cs.get("scaleType").cloned()) {
    continue;
   
}
if ! ["log", "symlog"].contains(&& cs.get("scaleType").cloned().unwrap_or_default()) {
    continue;
   
}
let vals: Vec<std::collections::HashMap<String, DepylerValue>>= data.iter().cloned().filter(| r | {
    let mut r = r.clone();
   (((true) &&(! true)) &&(r.get(& cs.get("field").cloned().unwrap_or_default()).cloned()>0)) &&((r.get(& cs.get("field").cloned().unwrap_or_default()).cloned() as f64).is_finite()) }).map(| r | r.get(& cs.get("field").cloned().unwrap_or_default()).cloned()).collect::<Vec<_>>();
    if(vals.len() as i32)<2 {
    continue;
   
}
let decades  = ((*vals.iter().max().expect("empty collection") as f64).log10()) - ((*vals.iter().min ().expect("empty collection") as f64).log10());
    let needed  = (((depyler_max((1).clone() ,(decades).clone()) as f64).ceil()).py_mul(LOG_PX_PER_DECADE_FACET)) as i32;
    if axis == STR_X {
    min_subplot_width = depyler_max((min_subplot_width).clone() ,(needed).clone());
   
}
else {
    min_subplot_height = depyler_max((min_subplot_height).clone() ,(needed).clone());
   
}
} for axis in vec! [STR_X.to_string(), STR_Y.to_string()] {
    cs = channel_semantics.get(& axis).cloned();
    if(cs.is_none()) ||(! cs.get("field").cloned()) {
    continue;
   
}
let effective_type = declaration.get("resolvedTypes").cloned().unwrap_or({ let mut map: HashMap<String, String>= HashMap::new();
    map
}
. to_string()).get(& axis).cloned().unwrap_or(cs.get("type").cloned().to_string());
    let is_banded = declaration.get("axisFlags").cloned().unwrap_or({ let mut map: HashMap<String, String>= HashMap::new();
    map
}
. to_string()).get(& axis).cloned().unwrap_or({ let mut map: HashMap<String, String>= HashMap::new();
    map
}
. to_string()).get("banded").cloned() == true;
    if(! is_discrete_type(effective_type)) &&(! is_banded) {
    continue;
   
}
let value_count = data.iter().cloned().map(| r | r.get(& cs.get("field").cloned().unwrap_or_default()).cloned()).collect::<std::collections::HashSet<_>>().len() as i32;
    let axis_group_count = if(group_axis == axis) &&(group_count>1) {
    group_count
}
else {
    1 };
    let max_dim = if axis == STR_X {
    max_w
}
else {
    max_h };
    if axis_group_count>1 {
    let min_group_step = depyler_max((((MIN_GROUP_GAP_PX).join(step_padding) as f64).ceil()).clone() ,((2i32).py_mul(axis_group_count)).clone());
    per_category_step = depyler_max(((min_step).py_mul(axis_group_count)).clone() ,(min_group_step).clone());
   
}
else {
    per_category_step = min_step.clone().into();
   
}
let data_driven_min = depyler_min (((per_category_step).py_mul(value_count)).clone() ,(max_dim).clone());
    let min_dim = depyler_max((base_min_subplot).clone() ,(data_driven_min).clone());
    if axis == STR_X {
    min_subplot_width = min_dim;
   
}
else {
    min_subplot_height = min_dim;
   
}
} fn x_is_cont() -> bool {
    cs = channel_semantics.get("x").cloned();
    if(cs.is_none()) ||(! cs.get("field").cloned()) {
    return false;
   
}
let t = declaration.get("resolvedTypes").cloned().unwrap_or({ let mut map: HashMap<String, String>= HashMap::new();
    map
}
. to_string()).get("x").cloned().unwrap_or(cs.get("type").cloned().to_string());
    return(! is_discrete_type(t)) &&(! declaration.get("axisFlags").cloned().unwrap_or({ let mut map: HashMap<String, String>= HashMap::new();
    map
}
. to_string()).get("x").cloned().unwrap_or({ let mut map: HashMap<String, String>= HashMap::new();
    map
}
. to_string()).get("banded").cloned() == true);
   
}
fn y_is_cont() -> bool {
    cs = channel_semantics.get("y").cloned();
    if(cs.is_none()) ||(! cs.get("field").cloned()) {
    return false;
   
}
let t = declaration.get("resolvedTypes").cloned().unwrap_or({ let mut map: HashMap<String, String>= HashMap::new();
    map
}
. to_string()).get("y").cloned().unwrap_or(cs.get("type").cloned().to_string());
    return(! is_discrete_type(t)) &&(! declaration.get("axisFlags").cloned().unwrap_or({ let mut map: HashMap<String, String>= HashMap::new();
    map
}
. to_string()).get("y").cloned().unwrap_or({ let mut map: HashMap<String, String>= HashMap::new();
    map
}
. to_string()).get("banded").cloned() == true);
   
}
let _cse_temp_5  = (x_is_cont()) &&(y_is_cont());
    if _cse_temp_5 {
    let x_cs = channel_semantics.get("x").cloned();
    let y_cs = channel_semantics.get("y").cloned();
    let _cse_temp_6  = (x_cs.is_some()) &&(x_cs.get("field").cloned());
    let _cse_temp_7  = (_cse_temp_6.is_some()) &&(y_cs.is_some());
    let _cse_temp_8  = (_cse_temp_7.is_some()) &&(y_cs.get("field").cloned());
    if let Some(ref _cse_temp_8_val) = _cse_temp_8 {
    let _cse_temp_9 = declaration.get("resolvedTypes").cloned().unwrap_or({ let mut map: HashMap<String, String>= HashMap::new();
    map
}
. to_string()).get("x").cloned().unwrap_or(x_cs.get("type").cloned().to_string());
    let _cse_temp_10 = _cse_temp_9 == STR_TEMPORAL;
    let is_temp_x = _cse_temp_10;
    let is_temp_y = _cse_temp_10;
    let cmcs = options.get("continuousMarkCrossSection").cloned();
    let _cse_temp_11 = cmcs.get("seriesCountAxis").cloned() != 0;
    let _cse_temp_12  = (true) &&(_cse_temp_11);
    let is_conn = _cse_temp_12;
    let mut x_num: Vec<f64>= vec! [];
    let mut y_num: Vec<f64>= vec! [];
    let mut s_keys: Vec<String>= vec! [];
    let mut s_fields: Vec<String>= vec! [];
    let col_f = channel_semantics.get("column").cloned().unwrap_or({ let mut map: HashMap<String, String>= HashMap::new();
    map
}
. to_string()).get("field").cloned();
    let row_f = channel_semantics.get("row").cloned().unwrap_or({ let mut map: HashMap<String, String>= HashMap::new();
    map
}
. to_string()).get("field").cloned();
    if let Some(ref col_f_val) = col_f {
    s_fields.append(col_f_val);
   
}
if let Some(ref row_f_val) = row_f {
    s_fields.append(row_f_val);
   
}
let cf = channel_semantics.get("color").cloned().unwrap_or({ let mut map: HashMap<String, String>= HashMap::new();
    map
}
. to_string()).get("field").cloned();
    let df = channel_semantics.get("detail").cloned().unwrap_or({ let mut map: HashMap<String, String>= HashMap::new();
    map
}
. to_string()).get("field").cloned();
    if let Some(ref cf_val) = cf {
    s_fields.append(cf_val);
   
}
let _cse_temp_13 = df != cf;
    let _cse_temp_14  = (df.is_some()) &&(_cse_temp_13);
    if _cse_temp_14 {
    s_fields.append(df);
   
}
for row in data.iter().cloned() {
    let xv = row.get(& x_cs.get("field").cloned().unwrap_or_default()).cloned();
    let yv = row.get(& y_cs.get("field").cloned().unwrap_or_default()).cloned();
    if(xv.is_none()) ||(yv.is_none()) {
    continue;
   
}
let xn = if is_temp_x {
    _js_to_date_number(xv)
}
else {
    _js_to_number(xv) };
    let yn = if is_temp_y {
    _js_to_date_number(yv)
}
else {
    _js_to_number(yv) };
    if(_is_nan(xn.as_f64().unwrap_or_default())) ||(_is_nan(yn.as_f64().unwrap_or_default())) {
    continue;
   
}
x_num.append(xn);
    y_num.append(yn);
    s_keys.append(if ! s_fields.is_empty() {
    s_fields.iter().cloned().map(| f |(if row.get(& f).cloned().is_some() {
    row.get(& f).cloned()
}
else {
    STR_EMPTY }).to_string()).collect::<Vec<_>>().join ("\0")
}
else {
    STR_EMPTY });
   
}
let _cse_temp_15 = x_num.len() as i32;
    let _cse_temp_16 = _cse_temp_15>1;
    if _cse_temp_16 {
    let _cse_temp_17 = * x_num.iter().min ().expect("empty collection");
    let x_min = _cse_temp_17;
    let _cse_temp_18 = * x_num.iter().max().expect("empty collection");
    let x_max = _cse_temp_18;
    let _cse_temp_19 = * y_num.iter().min ().expect("empty collection");
    let y_min = _cse_temp_19;
    let _cse_temp_20 = * y_num.iter().max().expect("empty collection");
    let y_max = _cse_temp_20;
    let mut x_dom: Vec<i32>= vec! [x_min, x_max];
    let mut y_dom: Vec<i32>= vec! [y_min, y_max];
    if x_cs.get("zero").cloned().unwrap_or({ let mut map: HashMap<String, String>= HashMap::new();
    map
}
. to_string()).get("zero").cloned().is_some() {
    let _cse_temp_21 = x_dom.get(0usize).cloned().expect("IndexError: list index out of range")>0;
    if _cse_temp_21 {
    x_dom [(0) as usize] = 0;
   
}
let _cse_temp_22 = x_dom.get(1usize).cloned().expect("IndexError: list index out of range")<0;
    if _cse_temp_22 {
    x_dom [(1) as usize] = 0;
   
}
} if y_cs.get("zero").cloned().unwrap_or({ let mut map: HashMap<String, String>= HashMap::new();
    map
}
. to_string()).get("zero").cloned().is_some() {
    let _cse_temp_23 = y_dom.get(0usize).cloned().expect("IndexError: list index out of range")>0;
    if _cse_temp_23 {
    y_dom [(0) as usize] = 0;
   
}
let _cse_temp_24 = y_dom.get(1usize).cloned().expect("IndexError: list index out of range")<0;
    if _cse_temp_24 {
    y_dom [(1) as usize] = 0;
   
}
} let ar = compute_banking_ar(& x_num, & y_num, & x_dom, & y_dom, & s_keys, is_conn) ?;
    let _cse_temp_25 = ar>= 1f64;
    if _cse_temp_25 {
    let _cse_temp_26 = depyler_max((min_subplot_width).clone() ,(js_round((base_min_subplot).py_mul(depyler_min ((ar).clone() ,(ms_x).clone())))).clone());
    min_subplot_width = _cse_temp_26;
    let _cse_temp_27 = depyler_max((min_subplot_height).clone() ,(base_min_subplot).clone());
    min_subplot_height = _cse_temp_27;
   
}
else {
    let _cse_temp_28 = depyler_max((min_subplot_width).clone() ,(base_min_subplot).clone());
    min_subplot_width = _cse_temp_28;
    let _cse_temp_29 = depyler_max((min_subplot_height).clone() ,(js_round((base_min_subplot).py_mul(depyler_min (((1i32).join(ar)).clone() ,(ms_y).clone())))).clone());
    min_subplot_height = _cse_temp_29;
   
}
}
}
} let effective_w = max_w;
    let effective_h = max_h;
    let _cse_temp_30 = depyler_max((1).clone() ,(((effective_w).join((min_subplot_width).py_add(gap)) as f64).floor()).clone());
    let max_facet_columns = _cse_temp_30;
    let max_facet_rows = _cse_temp_30;
    let col_field = channel_semantics.get("column").cloned().unwrap_or({ let mut map: HashMap<String, String>= HashMap::new();
    map
}
. to_string()).get("field").cloned();
    let row_field = channel_semantics.get("row").cloned().unwrap_or({ let mut map: HashMap<String, String>= HashMap::new();
    map
}
. to_string()).get("field").cloned();
    let _cse_temp_31  = (false) &&(false);
    if _cse_temp_31 {
    return Ok(None);
   
}
let col_count = if col_field.is_some() {
    data.iter().cloned().map(| r | r.get(& col_field).cloned()).collect::<std::collections::HashSet<_>>().len() as i32
}
else {
    0 };
    let row_count = if row_field.is_some() {
    data.iter().cloned().map(| r | r.get(& row_field).cloned()).collect::<std::collections::HashSet<_>>().len() as i32
}
else {
    0 };
    let _cse_temp_32 = col_count == 0;
    let _cse_temp_33 = row_count == 0;
    let _cse_temp_34  = (_cse_temp_32) &&(_cse_temp_33);
    if _cse_temp_34 {
    return Ok(None);
   
}
let _cse_temp_35 = col_count>0;
    let _cse_temp_36  = (_cse_temp_35) &&(_cse_temp_33);
    if _cse_temp_36 {
    let _cse_temp_37 = col_count <= max_facet_columns;
    if _cse_temp_37 {
    return Ok(Some({ let mut map: HashMap<String, DepylerValue>= HashMap::new();
    map.insert(STR_COLUMNS.to_string(), DepylerValue::Int(col_count as i64));
    map.insert(STR_ROWS.to_string(), DepylerValue::Int(1 as i64));
    map.insert("maxColumnValues".to_string(), DepylerValue::Int(col_count as i64));
    map.insert("maxRowValues".to_string(), DepylerValue::Int(max_facet_rows as i64));
    map }));
   
}
let mut n_cols = max_facet_columns.clone();
    let mut n_rows  = ((col_count).join(n_cols) as f64).ceil();
    while(n_cols>2) &&((col_count).py_mod(n_cols) == 1) {
    n_cols  = ((n_cols) - (1i32)) as i32;
    n_rows  = ((col_count).join(n_cols) as f64).ceil();
   
}
let _cse_temp_38 = depyler_min ((n_rows).clone() ,(max_facet_rows).clone());
    let vis_rows = _cse_temp_38;
    let _cse_temp_39  = ((n_cols).py_mul(vis_rows)) as i32;
    let max_total = _cse_temp_39;
    return Ok(Some({ let mut map: HashMap<String, DepylerValue>= HashMap::new();
    map.insert(STR_COLUMNS.to_string(), DepylerValue::Int(n_cols as i64));
    map.insert(STR_ROWS.to_string(), DepylerValue::Int(vis_rows as i64));
    map.insert("maxColumnValues".to_string(), DepylerValue::Int(max_total as i64));
    map.insert("maxRowValues".to_string(), DepylerValue::Int(max_facet_rows as i64));
    map }));
   
}
Ok(Some({ let mut map: HashMap<String, DepylerValue>= HashMap::new();
    map.insert(STR_COLUMNS.to_string(), DepylerValue::Str(format!("{:?}", depyler_max((1).clone() ,(depyler_min ((col_count).clone() ,(max_facet_columns).clone())).clone()))));
    map.insert(STR_ROWS.to_string(), DepylerValue::Str(format!("{:?}", depyler_max((1).clone() ,(depyler_min ((row_count).clone() ,(max_facet_rows).clone())).clone()))));
    map.insert("maxColumnValues".to_string(), DepylerValue::Int(max_facet_columns as i64));
    map.insert("maxRowValues".to_string(), DepylerValue::Int(max_facet_rows as i64));
    map }))
}
#[doc = " Depyler: proven to terminate"] pub fn compute_min_subplot_dimensions<'b, 'a, 'l1, 'c>(channel_semantics: & 'a std::collections::HashMap<String, std::collections::HashMap<String, DepylerValue>>, declaration: & 'b std::collections::HashMap<String, DepylerValue>, data: & 'c Vec<std::collections::HashMap<String, DepylerValue>>, options: & 'l1 std::collections::HashMap<String, DepylerValue>) -> Result<std::collections::HashMap<String, f64>, IndexError>{
    let mut cs: DepylerValue = Default::default();
    let mut min_subplot_width: i32 = Default::default();
    let mut min_subplot_height: i32 = Default::default();
    let mut item_count: i32 = Default::default();
    let min_step = options.get("minStep").cloned().unwrap_or(DepylerValue::Int(6i64));
    let min_subplot = options.get("minSubplotSize").cloned().unwrap_or(DepylerValue::Int(60i64));
    min_subplot_width = min_subplot.clone().into();
    min_subplot_height = min_subplot.clone().into();
    let LOG_PX_PER_DECADE_MIN = 40;
    for axis in vec! [STR_X.to_string(), STR_Y.to_string()] {
    cs = channel_semantics.get(& axis).cloned();
    if((cs.is_none()) ||(! cs.get("field").cloned())) ||(! cs.get("scaleType").cloned()) {
    continue;
   
}
if ! ["log", "symlog"].contains(&& cs.get("scaleType").cloned().unwrap_or_default()) {
    continue;
   
}
let vals: Vec<std::collections::HashMap<String, DepylerValue>>= data.iter().cloned().filter(| r | {
    let mut r = r.clone();
   (((true) &&(! true)) &&(r.get(& cs.get("field").cloned().unwrap_or_default()).cloned()>0)) &&((r.get(& cs.get("field").cloned().unwrap_or_default()).cloned() as f64).is_finite()) }).map(| r | r.get(& cs.get("field").cloned().unwrap_or_default()).cloned()).collect::<Vec<_>>();
    if(vals.len() as i32)<2 {
    continue;
   
}
let decades  = ((*vals.iter().max().expect("empty collection") as f64).log10()) - ((*vals.iter().min ().expect("empty collection") as f64).log10());
    let needed  = (((depyler_max((1).clone() ,(decades).clone()) as f64).ceil()).py_mul(LOG_PX_PER_DECADE_MIN)) as i32;
    if axis == STR_X {
    min_subplot_width = depyler_max((min_subplot_width).clone() ,(needed).clone());
   
}
else {
    min_subplot_height = depyler_max((min_subplot_height).clone() ,(needed).clone());
   
}
} fn is_discrete_type(t: Option<String>) -> bool {
    return(t == STR_NOMINAL) ||(t == STR_ORDINAL);
   
}
for axis in vec! [STR_X.to_string(), STR_Y.to_string()] {
    cs = channel_semantics.get(& axis).cloned();
    if(cs.is_none()) ||(! cs.get("field").cloned()) {
    continue;
   
}
let effective_type = declaration.get("resolvedTypes").cloned().unwrap_or({ let mut map: HashMap<String, String>= HashMap::new();
    map
}
. to_string()).get(& axis).cloned().unwrap_or(cs.get("type").cloned().to_string());
    let is_banded = declaration.get("axisFlags").cloned().unwrap_or({ let mut map: HashMap<String, String>= HashMap::new();
    map
}
. to_string()).get(& axis).cloned().unwrap_or({ let mut map: HashMap<String, String>= HashMap::new();
    map
}
. to_string()).get("banded").cloned() == true;
    let is_discrete = is_discrete_type(effective_type);
    item_count = 0;
    if(is_banded) ||(is_discrete) {
    item_count = data.iter().cloned().map(| r | r.get(& cs.get("field").cloned().unwrap_or_default()).cloned()).collect::<std::collections::HashSet<_>>().len() as i32;
   
}
if item_count>0 {
    let min_dim = depyler_max((min_subplot).clone() ,((item_count).py_mul(min_step)).clone());
    if axis == STR_X {
    min_subplot_width = depyler_max((min_subplot_width).clone() ,(min_dim).clone());
   
}
else {
    min_subplot_height = depyler_max((min_subplot_height).clone() ,(min_dim).clone());
   
}
}
}
Ok({ let mut map: HashMap<String, f64>= HashMap::new();
    map.insert("minSubplotWidth".to_string(), min_subplot_width);
    map.insert("minSubplotHeight".to_string(), min_subplot_height);
    map })
}
#[doc = "A measure is a quantitative channel or any aggregated channel."] #[doc = " Depyler: verified panic-free"] #[doc = " Depyler: proven to terminate"] pub fn _is_measure_enc(e: & Option<std::collections::HashMap<String, DepylerValue>>) -> bool {
    let _cse_temp_0  = (false) ||(! e.get("field").cloned());
    if _cse_temp_0 {
    return false;
   
}
(e.get("aggregate").cloned() != 0) ||(e.get("type").cloned() == STR_QUANTITATIVE)
}
#[doc = "A sortable category axis is discrete(nominal/ordinal). Temporal axes\n    are deliberately excluded: reordering a time axis by value scrambles the\n    chronology, so Sort should not apply to them."] #[doc = " Depyler: verified panic-free"] #[doc = " Depyler: proven to terminate"] pub fn _is_discrete_category_enc(e: & Option<std::collections::HashMap<String, DepylerValue>>) -> bool {
    let _cse_temp_0  = (false) ||(! e.get("field").cloned());
    if _cse_temp_0 {
    return false;
   
}
((! e.get("aggregate").cloned()) &&(e.get("type").cloned() != STR_QUANTITATIVE)) &&(e.get("type").cloned() != STR_TEMPORAL)
}
#[doc = "Identify the discrete category axis and the measure axis among a pair\n    of position channels, so Sort works under either orientation(vertical or\n    horizontal) and only when a discrete axis actually exists.\n\n    Returns ``None`` when there is no discrete-category + measure pair to sort\n   (e.g. a temporal-x time series, or two quantitative axes).\n    "] #[doc = " Depyler: verified panic-free"] #[doc = " Depyler: proven to terminate"] pub fn _resolve_sort_channels<'b, 'a>(encodings: & 'a std::collections::HashMap<String, std::collections::HashMap<String, DepylerValue>>, candidates: & 'b(String, String)) -> Option<std::collections::HashMap<String, String>>{
    let category = candidates.iter().cloned().filter(| & c | _is_discrete_category_enc(Some(encodings.get(& c).cloned()))).map(| c | c).next();
    let measure = candidates.iter().cloned().filter(| & c | _is_measure_enc(Some(encodings.get(& c).cloned()))).map(| c | c).next();
    let _cse_temp_0  = (false) ||(false);
    let _cse_temp_1 = category == measure;
    let _cse_temp_2  = (_cse_temp_0) ||(_cse_temp_1);
    if _cse_temp_2 {
    return None;
   
}
Some({ let mut map: HashMap<String, String>= HashMap::new();
    map.insert("category".to_string(), category);
    map.insert("measure".to_string(), measure);
    map })
}
#[doc = "Sort the category axis of a bar-like chart by the measure value.\n\n    Encoding model: a value sort writes ``sortBy = <measure channel>``(one of\n    'x' | 'y', which the assembler understands) on the category channel.\n    \"Default\" clears the sort so the field's canonical ordering wins.\n    "] #[doc = " Depyler: verified panic-free"] #[doc = " Depyler: proven to terminate"] pub fn make_sort_action<'a, 'c, 'b>(key: & 'a str, label: & 'b str, channels: & 'c(String, String)) -> std::collections::HashMap<String, DepylerValue>{
    let candidates = channels;
    fn is_applicable(ctx: & std::collections::HashMap<String, DepylerValue>) -> bool {
    return _resolve_sort_channels(ctx.get("encodings").cloned().unwrap_or({ let mut map: HashMap<String, String>= HashMap::new();
    map
}
. to_string()), candidates.clone()).is_some();
   
}
fn get_value(encodings: & std::collections::HashMap<String, std::collections::HashMap<String, DepylerValue>>) -> Option<String>{
    let resolved = _resolve_sort_channels(encodings, candidates.clone());
    if false {
    return None;
   
}
let enc = encodings.clone().py_index((resolved.expect("value is None").get("category").cloned().unwrap_or_default() as i64) as i64).unwrap_or_default();
    if enc.get("sortBy").cloned() == resolved.expect("value is None").get("measure").cloned().unwrap_or_default() {
    return if enc.get("sortOrder").cloned() == "descending".to_string() {
    "value-desc".to_string()
}
else {
    "value-asc".to_string() };
   
}
return None;
   
}
    let mut map: HashMap<String, DepylerValue>= HashMap::new();
{
    map.insert(std::borrow::Cow::Borrowed("key").to_string(), DepylerValue::Str(key.to_string()));
    map.insert(STR_LABEL.to_string(), DepylerValue::Str(label.to_string()));
    map.insert("dependencies".to_string(), DepylerValue::Str(format!("{:?}", candidates.into_iter().collect::<Vec<_>>())));
    map.insert("isApplicable".to_string(), DepylerValue::Str(format!("{:?}", is_applicable)));
    map.insert("control".to_string(), DepylerValue::Dict({ let mut map: HashMap<String, DepylerValue>= HashMap::new();
    map.insert(STR_TYPE.to_string(), DepylerValue::Str("discrete".to_string().to_string()));
    map.insert("options".to_string(), DepylerValue::List(vec! [DepylerValue::Dict({ let mut map: HashMap<String, DepylerValue>= HashMap::new();
    map.insert("value".to_string(), DepylerValue::None);
    map.insert(STR_LABEL.to_string(), DepylerValue::Str("Default".to_string().to_string()));
    map
}
. into_iter().map(|(k, v) |(DepylerValue::Str(k), v)).collect()), DepylerValue::Dict({ let mut map: HashMap<String, DepylerValue>= HashMap::new();
    map.insert("value".to_string(), DepylerValue::Str("value-desc".to_string().to_string()));
    map.insert(STR_LABEL.to_string(), DepylerValue::Str("Value ↓".to_string().to_string()));
    map
}
. into_iter().map(|(k, v) |(DepylerValue::Str(k), v)).collect()), DepylerValue::Dict({ let mut map: HashMap<String, DepylerValue>= HashMap::new();
    map.insert("value".to_string(), DepylerValue::Str("value-asc".to_string().to_string()));
    map.insert(STR_LABEL.to_string(), DepylerValue::Str("Value ↑".to_string().to_string()));
    map
}
. into_iter().map(|(k, v) |(DepylerValue::Str(k), v)).collect())]));
    map
}
. into_iter().map(|(k, v) |(DepylerValue::Str(k), v)).collect()));
    map.insert("get".to_string(), DepylerValue::Str(format!("{:?}", get_value)));
    map.insert("set".to_string(), DepylerValue::Str(format!("{:?}", set_value)));
    map
}
} #[doc = "For each ``encodingAction`` whose override is present in\n    ``chartProperties``, apply the action's ``set(encodings, value)`` to\n    produce the transformed encodings. Absent(``None``/missing) overrides\n    are skipped so the base encoding value stands.\n    "] #[doc = " Depyler: verified panic-free"] pub fn apply_encoding_overrides<'a, 'b>(template: & 'a std::collections::HashMap<String, DepylerValue>, encodings: std::collections::HashMap<String, std::collections::HashMap<String, DepylerValue>>, chart_properties: & 'b Option<std::collections::HashMap<String, DepylerValue>>) -> std::collections::HashMap<String, std::collections::HashMap<String, DepylerValue>>{
    let mut result: std::collections::HashMap<String, std::collections::HashMap<String, DepylerValue>>= Default::default();
    let actions = template.get("encodingActions").cloned();
    let _cse_temp_0  = (false) ||(false);
    if _cse_temp_0 {
    return encodings;
   
}
result = encodings.clone();
    for action in actions.iter().cloned() {
    let key = action.get("key").cloned();
    if false {
    continue;
   
}
let r#override = chart_properties.get(& key.expect("value is None")).cloned();
    if r#override.is_some() {
    result  = (action.get("set").cloned().unwrap_or_default().into())(result, r#override);
   
}
} result
}
#[doc = " Depyler: verified panic-free"] #[doc = " Depyler: proven to terminate"] pub fn to_type_string(input_value: & DepylerValue) -> String {
    if ! input_value {
    return STR_EMPTY.to_string();
   
}
if true {
    return input_value.to_string();
   
}
if true {
    return if input_value.get("semanticType").cloned().unwrap_or("".to_string()).is_empty() {
    STR_EMPTY.to_string()
}
else {
    input_value.get("semanticType").cloned().unwrap_or("".to_string()).to_string() };
   
}
STR_EMPTY.to_string()
}
#[doc = " Depyler: verified panic-free"] #[doc = " Depyler: proven to terminate"] pub fn normalize_annotation(input_value: & DepylerValue) -> std::collections::HashMap<String, DepylerValue>{
    if ! input_value {
    return {
    let mut map: HashMap<String, DepylerValue>= HashMap::new();
    map.insert(STR_SEMANTICTYPE.to_string(), DepylerValue::Str(STR_UNKNOWN_2.to_string()));
    map };
   
}
if true {
    return {
    let mut map: HashMap<String, DepylerValue>= HashMap::new();
    map.insert(STR_SEMANTICTYPE.to_string(), DepylerValue::Str(format!("{:?}", if input_value.is_empty() {
    STR_UNKNOWN_2.to_string()
}
else {
    input_value.to_string() })));
    map };
   
}
    let mut map: HashMap<String, DepylerValue>= HashMap::new();
{
    map.insert(STR_SEMANTICTYPE.to_string(), DepylerValue::Str(STR_UNKNOWN_2.to_string()));
    map
}
} #[doc = " Depyler: verified panic-free"] #[doc = " Depyler: proven to terminate"] pub fn _is_number(v: & DepylerValue) -> bool {
    if true {
    return false;
   
}
(true) &&(!(true) &&((v as f64).is_nan()))
}
#[doc = " Depyler: proven to terminate"] pub fn _detect_percentage_representation(values: & Vec<f64>) -> Result<String, ZeroDivisionError>{
    let _cse_temp_0 = values.len() as i32;
    let _cse_temp_1 = _cse_temp_0 == 0;
    if _cse_temp_1 {
    return Ok("0-100".to_string().to_string());
   
}
let abs_vals: Vec<f64>= values.as_slice().iter().cloned().map(| v |(v).abs()).collect::<Vec<_>>();
    let _cse_temp_2 = abs_vals.as_slice().iter().cloned().filter(| & v | v <= 1).map(| v | 1).sum::<i32>();
    let count_below_1 = _cse_temp_2;
    let _cse_temp_3 = abs_vals.len() as i32;
    let _cse_temp_4  = ((count_below_1).join(_cse_temp_3)) as i32;
    let _cse_temp_5  = ((_cse_temp_4 as f64) as f64)>= 0.8;
    if _cse_temp_5 {
    return Ok("0-1".to_string().to_string());
   
}
Ok("0-100".to_string().to_string())
}
pub fn _detect_precision(values: & Vec<f64>) -> Result<i32, IndexError>{
    let mut max_decimals: i32 = Default::default();
    max_decimals = 0;
    for v in values.iter().cloned() {
    if !(true) &&((v as f64).is_finite()) {
    continue;
   
}
let mut s = "{:.10f}".replacen("{}", & format!("{}" ,(v) as f64), 1);
    let dot = s.find(".").map(| i | i as i32).unwrap_or(- 1);
    if dot.unwrap_or_default() == - 1 {
    continue;
   
}
let mut end  = ((s.len() as i32) - (1i32)) as i32;
    while(end>dot.unwrap_or(i32::MIN)) &&({ let base = & s;
    let idx: i32 = end;
    let actual_idx = if idx<0 {
    base.chars().count().saturating_sub(idx.abs() as usize)
}
else {
    idx as usize };
    base.chars().nth(actual_idx).map(| c | c.to_string()).unwrap_or_default()
}
== "0") {
    end  = ((end) - (1i32)) as i32;
   
}
let decimals = if end>dot.unwrap_or(i32::MIN) {
   (end) - (dot)
}
else {
    0 };
    if decimals>max_decimals {
    max_decimals = decimals;
   
}
} Ok(depyler_min ((max_decimals).clone() ,(4).clone()))
}
#[doc = " Depyler: verified panic-free"] #[doc = " Depyler: proven to terminate"] pub fn _precision_format(values: & Vec<f64>, use_grouping: bool, sign_mode: String) -> Result<String, Box<dyn std::error::Error>>{
    let p = _detect_precision(& values) ?;
    let group = if use_grouping {
    ","
}
else {
    STR_EMPTY };
    let _cse_temp_0 = p == 0;
    if _cse_temp_0 {
    return Ok(format!("{}{}d", sign_mode, group));
   
}
Ok(format!("{}{}.{}f", sign_mode, group, p))
}
#[doc = " Depyler: proven to terminate"] pub fn resolve_format<'c, 'b, 'a>(semantic_type: & 'a str, annotation: & 'b std::collections::HashMap<String, DepylerValue>, values: & 'c Vec<DepylerValue>) -> Result<std::collections::HashMap<String, DepylerValue>, IndexError>{
    let mut currency_prefix: DepylerValue = Default::default();
    let entry = get_registry_entry(& semantic_type);
    let unit = annotation.get("unit").cloned();
    if let Some(ref unit_val) = unit {
    let _cse_temp_0 = CURRENCY_MAP.get(& unit_val.to_uppercase()).cloned().unwrap_or(CURRENCY_MAP.get(& unit_val).cloned().to_string());
    currency_prefix = _cse_temp_0;
   
}
let unit_suffix = if unit.is_some() {
    UNIT_SUFFIX_MAP.get(& unit).cloned()
}
else {
    None };
    let nums: Vec<DepylerValue>= values.iter().cloned().filter(| v | {
    let v = v.clone();
    _is_number(v) }).map(| v | v).collect::<Vec<_>>();
    let fmt_class = entry.get("formatClass").cloned().unwrap_or_default();
    let _cse_temp_1 = fmt_class == "currency";
    if _cse_temp_1 {
    if currency_prefix {
    let axis_pattern = if semantic_type == "Price" {
    ",.2f".to_string()
}
else {
    _precision_format(& nums, true, "") };
    return Ok({ let mut map: HashMap<String, DepylerValue>= HashMap::new();
    map.insert("format".to_string(), DepylerValue::Dict({ let mut map: HashMap<String, DepylerValue>= HashMap::new();
    map.insert(STR_PATTERN.to_string(), DepylerValue::Str(axis_pattern.to_string()));
    map.insert("prefix".to_string(), DepylerValue::Str(format!("{:?}", currency_prefix)));
    map
}
. into_iter().map(|(k, v) |(DepylerValue::Str(k), v)).collect()));
    map.insert(STR_TOOLTIPFORMAT.to_string(), DepylerValue::Dict({ let mut map: HashMap<String, DepylerValue>= HashMap::new();
    map.insert(STR_PATTERN.to_string(), DepylerValue::Str(",.2f".to_string().to_string()));
    map.insert("prefix".to_string(), DepylerValue::Str(format!("{:?}", currency_prefix)));
    map
}
. into_iter().map(|(k, v) |(DepylerValue::Str(k), v)).collect()));
    map });
   
}
return Ok({ let mut map: HashMap<String, DepylerValue>= HashMap::new();
    map.insert(STR_TOOLTIPFORMAT.to_string(), DepylerValue::Dict({ let mut map: HashMap<String, DepylerValue>= HashMap::new();
    map.insert(STR_PATTERN.to_string(), DepylerValue::Str(",.2f".to_string().to_string()));
    map
}
. into_iter().map(|(k, v) |(DepylerValue::Str(k), v)).collect()));
    map });
   
}
let _cse_temp_2 = fmt_class == "percent";
    if _cse_temp_2 {
    if ! annotation.get("intrinsicDomain").cloned() {
    return Ok({ let mut map: HashMap<String, DepylerValue>= HashMap::new();
    map.insert(STR_TOOLTIPFORMAT.to_string(), DepylerValue::Dict({ let mut map: HashMap<String, DepylerValue>= HashMap::new();
    map.insert(STR_PATTERN.to_string(), DepylerValue::Str(format!("{:?}", _precision_format(& nums, true, ""))));
    map
}
. into_iter().map(|(k, v) |(DepylerValue::Str(k), v)).collect()));
    map });
   
}
let rep = _detect_percentage_representation(& nums) ?;
    let _cse_temp_3 = rep == "0-1".to_string();
    if _cse_temp_3 {
    let p = _detect_precision(& nums) ?;
    let _cse_temp_4 = depyler_max((0).clone() ,((p) - (2i32)).clone());
    let axis_p = _cse_temp_4;
    let _cse_temp_5 = depyler_min (((axis_p).py_add(1i32)).clone() ,(4).clone());
    let tip_p = _cse_temp_5;
    return Ok({ let mut map: HashMap<String, DepylerValue>= HashMap::new();
    map.insert("format".to_string(), DepylerValue::Dict({ let mut map: HashMap<String, DepylerValue>= HashMap::new();
    map.insert(STR_PATTERN.to_string(), DepylerValue::Str(format!("{:?}", format!(".{}~%", axis_p))));
    map
}
. into_iter().map(|(k, v) |(DepylerValue::Str(k), v)).collect()));
    map.insert(STR_TOOLTIPFORMAT.to_string(), DepylerValue::Dict({ let mut map: HashMap<String, DepylerValue>= HashMap::new();
    map.insert(STR_PATTERN.to_string(), DepylerValue::Str(format!("{:?}", format!(".{}%", tip_p))));
    map
}
. into_iter().map(|(k, v) |(DepylerValue::Str(k), v)).collect()));
    map });
   
}
return Ok({ let mut map: HashMap<String, DepylerValue>= HashMap::new();
    map.insert(STR_TOOLTIPFORMAT.to_string(), DepylerValue::Dict({ let mut map: HashMap<String, DepylerValue>= HashMap::new();
    map.insert(STR_PATTERN.to_string(), DepylerValue::Str(format!("{:?}", _precision_format(& nums, false, ""))));
    map.insert("suffix".to_string(), DepylerValue::Str("%".to_string().to_string()));
    map
}
. into_iter().map(|(k, v) |(DepylerValue::Str(k), v)).collect()));
    map });
   
}
let _cse_temp_6 = fmt_class == "unit-suffix";
    if _cse_temp_6 {
    if unit_suffix {
    return Ok({ let mut map: HashMap<String, DepylerValue>= HashMap::new();
    map.insert(STR_TOOLTIPFORMAT.to_string(), DepylerValue::Dict({ let mut map: HashMap<String, DepylerValue>= HashMap::new();
    map.insert(STR_PATTERN.to_string(), DepylerValue::Str(format!("{:?}", _precision_format(& nums, true, ""))));
    map.insert("suffix".to_string(), DepylerValue::Str(format!("{:?}", unit_suffix)));
    map
}
. into_iter().map(|(k, v) |(DepylerValue::Str(k), v)).collect()));
    map });
   
}
return Ok({ let mut map: HashMap<String, DepylerValue>= HashMap::new();
    map.insert(STR_TOOLTIPFORMAT.to_string(), DepylerValue::Dict({ let mut map: HashMap<String, DepylerValue>= HashMap::new();
    map.insert(STR_PATTERN.to_string(), DepylerValue::Str(format!("{:?}", _precision_format(& nums, true, ""))));
    map
}
. into_iter().map(|(k, v) |(DepylerValue::Str(k), v)).collect()));
    map });
   
}
let _cse_temp_7 = fmt_class == "integer";
    if _cse_temp_7 {
    let _cse_temp_8 = [STR_YEAR, STR_DECADE].contains(&semantic_type);
    if _cse_temp_8 {
    return Ok({ let mut map: HashMap<String, DepylerValue>= HashMap::new();
    map });
   
}
return Ok({ let mut map: HashMap<String, DepylerValue>= HashMap::new();
    map.insert(STR_TOOLTIPFORMAT.to_string(), DepylerValue::Dict({ let mut map: HashMap<String, DepylerValue>= HashMap::new();
    map.insert(STR_PATTERN.to_string(), DepylerValue::Str(",d".to_string().to_string()));
    map
}
. into_iter().map(|(k, v) |(DepylerValue::Str(k), v)).collect()));
    map });
   
}
let _cse_temp_9 = fmt_class == "decimal";
    if _cse_temp_9 {
    return Ok({ let mut map: HashMap<String, DepylerValue>= HashMap::new();
    map.insert(STR_TOOLTIPFORMAT.to_string(), DepylerValue::Dict({ let mut map: HashMap<String, DepylerValue>= HashMap::new();
    map.insert(STR_PATTERN.to_string(), DepylerValue::Str(format!("{:?}", _precision_format(& nums, true, ""))));
    map
}
. into_iter().map(|(k, v) |(DepylerValue::Str(k), v)).collect()));
    map });
   
}
Ok({ let mut map: HashMap<String, DepylerValue>= HashMap::new();
    map })
}
#[doc = " Depyler: proven to terminate"] pub fn resolve_default_vis_type<'a, 'b>(semantic_type: & 'a str, values: & 'b Vec<DepylerValue>) -> Result<String, IndexError>{
    let mut distinct: DepylerValue = Default::default();
    if ! is_registered(& semantic_type) {
    return Ok(infer_vis_category(& values));
   
}
let entry = get_registry_entry(& semantic_type);
    let candidates = entry.get("visEncodings").cloned().unwrap_or_default();
    let _cse_temp_0 = candidates.len() as i32;
    let _cse_temp_1 = _cse_temp_0 == 1;
    if _cse_temp_1 {
    let _cse_temp_2 = candidates.get(0usize).cloned().expect("IndexError: list index out of range") == STR_QUANTITATIVE;
    if _cse_temp_2 {
    let non_null: Vec<DepylerValue>= values.iter().cloned().filter(| v | {
    let v = v.clone();
    ! v.is_null() }).map(| v | v).collect::<Vec<_>>();
    let _cse_temp_3 = non_null.len() as i32;
    let _cse_temp_4 = _cse_temp_3>0;
    let _cse_temp_5  = (_cse_temp_4) &&(non_null.iter().cloned().map(| v |((true) &&(! true)) ||(((true) &&(v.trim().to_string() != STR_EMPTY)) &&(_try_float(& v)))).all(| x | x));
    let all_numeric = _cse_temp_5;
    if ! all_numeric {
    return Ok(infer_vis_category(& values));
   
}
} return Ok(candidates.get(0usize).cloned().expect("IndexError: list index out of range"));
   
}
let _cse_temp_6 = candidates.contains("quantitative");
    let _cse_temp_7 = candidates.contains("ordinal");
    let _cse_temp_8  = (_cse_temp_6) &&(_cse_temp_7);
    if _cse_temp_8 {
    let _cse_temp_9 = values.iter().cloned().filter(| v | {
    let v = v.clone();
    ! v.is_null() }).map(| v | v).collect::<std::collections::HashSet<_>>().len() as i32;
    distinct = _cse_temp_9;
    return Ok(if distinct <= 12 {
    STR_ORDINAL
}
else {
    STR_QUANTITATIVE });
   
}
let _cse_temp_10 = candidates.contains("temporal");
    let _cse_temp_11  = (_cse_temp_10) &&(_cse_temp_7);
    if _cse_temp_11 {
    let _cse_temp_12 = values.iter().cloned().filter(| v | {
    let v = v.clone();
    ! v.is_null() }).map(| v | v).collect::<std::collections::HashSet<_>>().len() as i32;
    distinct = _cse_temp_12;
    return Ok(if distinct <= 6 {
    STR_ORDINAL
}
else {
    STR_TEMPORAL });
   
}
let _cse_temp_13 = candidates.contains("geographic");
    let _cse_temp_14  = (_cse_temp_13) &&(_cse_temp_6);
    if _cse_temp_14 {
    return Ok(STR_QUANTITATIVE.to_string());
   
}
Ok(candidates.get(0usize).cloned().expect("IndexError: list index out of range"))
}
#[doc = " Depyler: verified panic-free"] #[doc = " Depyler: proven to terminate"] pub fn _try_float(s: & str) -> bool {
    match(|| -> Result <(), Box<dyn std::error::Error>>{
    s.parse::<f64>().expect("parse failed");
    true Ok(()) })() {
    Ok(()) =>{
   
}
Err(_e) =>{
    false
}
}
}
#[doc = " Depyler: proven to terminate"] pub fn resolve_aggregation_default(semantic_type: & str) -> Result<Option<String>, IndexError>{
    let entry = get_registry_entry(& semantic_type);
    let role = entry.get("aggRole").cloned().unwrap_or_default();
    let _cse_temp_0 = ["additive", "signed-additive"].contains(&& role);
    if _cse_temp_0 {
    return Ok(Some(STR_SUM.to_string()));
   
}
let _cse_temp_1 = role == "intensive";
    if _cse_temp_1 {
    return Ok(Some("average".to_string().to_string()));
   
}
Ok(None)
}
#[doc = " Depyler: proven to terminate"] pub fn resolve_zero_class_from_annotation<'a, 'b>(semantic_type: & 'a str, domain: & 'b Option<Vec<f64>>) -> Result<String, IndexError>{
    let _cse_temp_0 = domain.get(0usize).cloned().expect("IndexError: list index out of range")>0;
    let _cse_temp_1  = (domain.is_some()) &&(_cse_temp_0);
    if let Some(ref _cse_temp_1_val) = _cse_temp_1 {
    return Ok("arbitrary".to_string().to_string());
   
}
get_zero_class(& semantic_type)
}
#[doc = " Depyler: proven to terminate"] pub fn resolve_scale_type<'b, 'a>(semantic_type: & 'a str, values: & 'b Vec<f64>) -> Result<Option<String>, Box<dyn std::error::Error>>{
    let entry = get_registry_entry(& semantic_type);
    let _cse_temp_0 = entry.get("aggRole").cloned().unwrap_or_default() == "additive";
    let _cse_temp_1 = entry.get("domainShape").cloned().unwrap_or_default() == "open";
    let _cse_temp_2  = (_cse_temp_0) &&(_cse_temp_1);
    let _cse_temp_3 = entry.get("t1").cloned().unwrap_or_default() != "GenericMeasure";
    let _cse_temp_4  = (_cse_temp_2) &&(_cse_temp_3);
    let eligible = _cse_temp_4;
    if ! eligible {
    return Ok(None);
   
}
let _cse_temp_5 = values.len() as i32;
    let _cse_temp_6 = _cse_temp_5<10;
    if _cse_temp_6 {
    return Ok(None);
   
}
let filtered: Vec<f64>= values.as_slice().iter().cloned().filter(| v | {
    let v = v.clone();
   ((true) &&(!(true) &&((v as f64).is_nan()))) &&((v as f64).is_finite()) }).map(| v | v).collect::<Vec<_>>();
    let _cse_temp_7 = filtered.len() as i32;
    let _cse_temp_8 = _cse_temp_7<10;
    if _cse_temp_8 {
    return Ok(None);
   
}
let _cse_temp_9 = * filtered.iter().min ().expect("empty collection");
    let mn = _cse_temp_9;
    let _cse_temp_10 = * filtered.iter().max().expect("empty collection");
    let mx = _cse_temp_10;
    let _cse_temp_11 = mx <= 0;
    let _cse_temp_12 = mn == mx;
    let _cse_temp_13  = (_cse_temp_11) ||(_cse_temp_12);
    if _cse_temp_13 {
    return Ok(None);
   
}
let _cse_temp_14 = mn<0;
    if _cse_temp_14 {
    return Ok(None);
   
}
let positives: Vec<DepylerValue>= filtered.iter().cloned().filter(| v | {
    let v = v.clone();
    v>0 }).map(| v | v).collect::<Vec<_>>();
    if ! positives.is_empty() {
    let _cse_temp_15 = * positives.iter().min ().expect("empty collection");
    let positive_min = _cse_temp_15;
    let _cse_temp_16  = ((mx).join(positive_min)) as i32;
    let _cse_temp_17 = _cse_temp_16>= 1000000;
    if _cse_temp_17 {
    let has_zeros = filtered.iter().cloned().map(| v | v == 0).any(| x | x);
    return Ok(Some(if has_zeros {
    "symlog"
}
else {
    "log" }));
   
}
} Ok(None)
}
#[doc = " Depyler: verified panic-free"] #[doc = " Depyler: proven to terminate"] pub fn _merge_intrinsic_with_data<'a, 'b>(intrinsic: & 'a Vec<f64>, values: & 'b Vec<DepylerValue>, hard: bool) -> std::collections::HashMap<String, DepylerValue>{
    if hard {
    return {
    let mut map: HashMap<String, DepylerValue>= HashMap::new();
    map.insert("min".to_string(), DepylerValue::Str(format!("{:?}", intrinsic.get(0usize).cloned().expect("IndexError: list index out of range"))));
    map.insert("max".to_string(), DepylerValue::Str(format!("{:?}", intrinsic.get(1usize).cloned().expect("IndexError: list index out of range"))));
    map.insert(STR_CLAMP.to_string(), DepylerValue::Bool(true));
    map };
   
}
let nums: Vec<DepylerValue>= values.iter().cloned().filter(| v | {
    let v = v.clone();
    _is_number(v) }).map(| v | v).collect::<Vec<_>>();
    let _cse_temp_0 = nums.len() as i32;
    let _cse_temp_1 = _cse_temp_0 == 0;
    if _cse_temp_1 {
    return {
    let mut map: HashMap<String, DepylerValue>= HashMap::new();
    map.insert("min".to_string(), DepylerValue::Str(format!("{:?}", intrinsic.get(0usize).cloned().expect("IndexError: list index out of range"))));
    map.insert("max".to_string(), DepylerValue::Str(format!("{:?}", intrinsic.get(1usize).cloned().expect("IndexError: list index out of range"))));
    map.insert(STR_CLAMP.to_string(), DepylerValue::Bool(false));
    map };
   
}
let _cse_temp_2 = * nums.iter().min ().expect("empty collection");
    let data_min = _cse_temp_2;
    let _cse_temp_3 = * nums.iter().max().expect("empty collection");
    let data_max = _cse_temp_3;
    {
    let mut map: HashMap<String, DepylerValue>= HashMap::new();
    map.insert("min".to_string(), DepylerValue::Str(format!("{:?}", depyler_min ((intrinsic.get(0usize).cloned().expect("IndexError: list index out of range")).clone() ,(data_min).clone()))));
    map.insert("max".to_string(), DepylerValue::Str(format!("{:?}", depyler_max((intrinsic.get(1usize).cloned().expect("IndexError: list index out of range")).clone() ,(data_max).clone()))));
    map.insert(STR_CLAMP.to_string(), DepylerValue::Bool(false));
    map
}
} #[doc = " Depyler: verified panic-free"] #[doc = " Depyler: proven to terminate"] pub fn snap_to_bound_heuristic<'b, 'a>(intrinsic: & 'a Vec<f64>, values: & 'b Vec<DepylerValue>) -> Option<std::collections::HashMap<String, DepylerValue>>{
    let mut snap_min: Option<f64>= Default::default();
    let mut snap_max: Option<f64>= Default::default();
    let nums: Vec<DepylerValue>= values.iter().cloned().filter(| v | {
    let v = v.clone();
    _is_number(v) }).map(| v | v).collect::<Vec<_>>();
    let _cse_temp_0 = nums.len() as i32;
    let _cse_temp_1 = _cse_temp_0 == 0;
    if _cse_temp_1 {
    return None;
   
}
let(lo, hi) = intrinsic.clone();
    let rng  = (hi) - (lo);
    let _cse_temp_2 = rng <= 0;
    if _cse_temp_2 {
    return None;
   
}
let _cse_temp_3 = * nums.iter().min ().expect("empty collection");
    let data_min = _cse_temp_3;
    let _cse_temp_4 = * nums.iter().max().expect("empty collection");
    let data_max = _cse_temp_4;
    let _cse_temp_5 = lo<0;
    let _cse_temp_6 = hi>0;
    let _cse_temp_7  = (_cse_temp_5) &&(_cse_temp_6);
    let zero_inside = _cse_temp_7;
    let _cse_temp_8  = (0.25).py_mul(if zero_inside {
    DepylerValue::Str(format!("{:?}" ,(0i32) - (lo)))
}
else {
    rng });
    let threshold_lo = _cse_temp_8;
    let threshold_hi = _cse_temp_8;
    let _cse_temp_9 = data_min>= lo;
    let _cse_temp_10  = ((data_min as f64) as f64)<= (lo).py_add(threshold_lo);
    let _cse_temp_11  = (_cse_temp_9) &&(_cse_temp_10);
    if _cse_temp_11 {
    snap_min = Some(lo);
   
}
let _cse_temp_12 = data_max <= hi;
    let _cse_temp_13  = ((data_max as f64) as f64)>= (hi) - (threshold_hi);
    let _cse_temp_14  = (_cse_temp_12) &&(_cse_temp_13);
    if _cse_temp_14 {
    snap_max = Some(hi);
   
}
let _cse_temp_15  = (false) &&(false);
    if _cse_temp_15 {
    return None;
   
}
let mut out: std::collections::HashMap<String, DepylerValue>= {
    let mut map: HashMap<String, DepylerValue>= HashMap::new();
    map.insert(STR_CLAMP.to_string(), DepylerValue::Bool(false));
    map };
    if snap_min.is_some() {
    out.insert("min".to_string(), snap_min);
   
}
if snap_max.is_some() {
    out.insert("max".to_string(), snap_max);
   
}
Some(out)
}
#[doc = " Depyler: proven to terminate"] pub fn resolve_domain_constraint<'c, 'a, 'b>(semantic_type: & 'a str, annotation: & 'b std::collections::HashMap<String, DepylerValue>, values: & 'c Vec<DepylerValue>) -> Result<Option<std::collections::HashMap<String, DepylerValue>>, IndexError>{
    let entry = get_registry_entry(& semantic_type);
    if annotation.get("intrinsicDomain").cloned().is_some() {
    let _cse_temp_0 = ["Proportion", "SignedMeasure"].contains(&& entry.get("t1").cloned().unwrap_or_default());
    if _cse_temp_0 {
    return Ok(Some(snap_to_bound_heuristic(annotation.get("intrinsicDomain").cloned().unwrap_or_default(), & values)));
   
}
return Ok(Some(_merge_intrinsic_with_data(annotation.get("intrinsicDomain").cloned().unwrap_or_default(), & values, false)));
   
}
let _cse_temp_1 = semantic_type == "Latitude";
    if _cse_temp_1 {
    return Ok(Some(_merge_intrinsic_with_data(& vec! [- 90, 90], & values, true)));
   
}
let _cse_temp_2 = semantic_type == "Longitude";
    if _cse_temp_2 {
    return Ok(Some(_merge_intrinsic_with_data(& vec! [- 180, 180], & values, true)));
   
}
let _cse_temp_3 = semantic_type == "Correlation";
    if _cse_temp_3 {
    return Ok(Some(_merge_intrinsic_with_data(& vec! [- 1, 1], & values, true)));
   
}
let _cse_temp_4 = semantic_type == "Percentage";
    if _cse_temp_4 {
    let nums: Vec<DepylerValue>= values.iter().cloned().filter(| v | {
    let v = v.clone();
    _is_number(v) }).map(| v | v).collect::<Vec<_>>();
    let _cse_temp_5 = nums.len() as i32;
    let _cse_temp_6 = _cse_temp_5>0;
    if _cse_temp_6 {
    let rep = _detect_percentage_representation(& nums) ?;
    let M = if rep == "0-1".to_string() {
    1
}
else {
    100 };
    return Ok(Some(snap_to_bound_heuristic(& vec! [0, M], & values)));
   
}
} Ok(None)
}
#[doc = " Depyler: proven to terminate"] pub fn resolve_tick_constraint<'b, 'a>(semantic_type: & 'a str, domain: & 'b Option<Vec<f64>>) -> Result<Option<std::collections::HashMap<String, DepylerValue>>, IndexError>{
    let mut tc: std::collections::HashMap<String, DepylerValue>= Default::default();
    let mut span: DepylerValue = Default::default();
    let entry = get_registry_entry(& semantic_type);
    let _cse_temp_0 = entry.get("formatClass").cloned().unwrap_or_default() == "integer";
    if _cse_temp_0 {
    tc = {
    let mut map: HashMap<String, DepylerValue>= HashMap::new();
    map.insert("integersOnly".to_string(), DepylerValue::Bool(true));
    map.insert(STR_MINSTEP.to_string(), DepylerValue::Int(1 as i64));
    map };
    if let Some(ref domain_val) = domain {
    let _cse_temp_1  = (domain_val.get(1usize).cloned().expect("IndexError: list index out of range")) - (domain_val.get(0usize).cloned().expect("IndexError: list index out of range"));
    span = _cse_temp_1;
    let _cse_temp_2 = 0<span;
    let _cse_temp_3 = span <= 20;
    let _cse_temp_4  = (_cse_temp_2) &&(_cse_temp_3);
    if _cse_temp_4 {
    tc.insert("exactTicks".to_string() ,(((domain_val.get(0usize).cloned().expect("IndexError: list index out of range")) as i32)..(((domain_val.get(1usize).cloned().expect("IndexError: list index out of range")) as i32).py_add(1i32))).collect::<Vec<_>>());
   
}
} return Ok(Some(tc));
   
}
let _cse_temp_5 = semantic_type == "Score";
    let _cse_temp_6  = (_cse_temp_5) &&(domain.is_some());
    if _cse_temp_6 {
    let _cse_temp_7  = (domain.get(1usize).cloned().expect("IndexError: list index out of range")) - (domain.get(0usize).cloned().expect("IndexError: list index out of range"));
    span = _cse_temp_7;
    let _cse_temp_8 = span>= 2;
    if _cse_temp_8 {
    tc = {
    let mut map: HashMap<String, DepylerValue>= HashMap::new();
    map.insert("integersOnly".to_string(), DepylerValue::Bool(true));
    map.insert(STR_MINSTEP.to_string(), DepylerValue::Int(1 as i64));
    map };
    let _cse_temp_9 = span <= 20;
    if _cse_temp_9 {
    tc.insert("exactTicks".to_string(), DepylerValue::from((((domain.get(0usize).cloned().expect("IndexError: list index out of range")) as i32)..(((domain.get(1usize).cloned().expect("IndexError: list index out of range")) as i32).py_add(1i32))).collect::<Vec<_>>()));
   
}
return Ok(Some(tc));
   
}
} Ok(None)
}
#[doc = " Depyler: proven to terminate"] pub fn resolve_canonical_order<'c, 'b, 'a>(semantic_type: & 'a str, annotation: & 'b std::collections::HashMap<String, DepylerValue>, values: & 'c Vec<DepylerValue>) -> Result<Option<Vec<String>>, IndexError>{
    let _cse_temp_0 = annotation.get("sortOrder").cloned().unwrap_or_default().len() as i32;
    let _cse_temp_1 = _cse_temp_0>0;
    let _cse_temp_2  = (annotation.get("sortOrder").cloned()) &&(_cse_temp_1);
    if let Some(ref _cse_temp_2_val) = _cse_temp_2 {
    return Ok(Some(annotation.get("sortOrder").cloned().unwrap_or_default()));
   
}
Ok(Some(infer_ordinal_sort_order(& semantic_type, & values)))
}
#[doc = " Depyler: proven to terminate"] pub fn resolve_cyclic(semantic_type: & str) -> Result<bool, IndexError>{
    Ok(get_registry_entry(& semantic_type).get("domainShape").cloned().unwrap_or_default().into() == "cyclic")
}
#[doc = " Depyler: verified panic-free"] #[doc = " Depyler: proven to terminate"] pub fn resolve_reversed<'a, 'b>(semantic_type: & 'a str, channel: & 'b Option<String>) -> bool {
    let _cse_temp_0 = semantic_type == STR_RANK;
    if _cse_temp_0 {
    return channel.unwrap_or_default() != STR_X;
   
}
false
}
#[doc = " Depyler: proven to terminate"] pub fn resolve_nice<'a, 'b>(semantic_type: & 'a str, domain_constraint: & 'b Option<std::collections::HashMap<String, DepylerValue>>) -> Result<bool, IndexError>{
    let _cse_temp_0  = (domain_constraint.is_some()) &&(domain_constraint.get("clamp").cloned());
    if let Some(ref _cse_temp_0_val) = _cse_temp_0 {
    return Ok(false);
   
}
let _cse_temp_1  = (_cse_temp_0.is_some()) &&(domain_constraint.get("max").cloned().is_some());
    if let Some(ref _cse_temp_1_val) = _cse_temp_1 {
    return Ok(false);
   
}
let entry = get_registry_entry(& semantic_type);
    let _cse_temp_2 = entry.get("domainShape").cloned().unwrap_or_default() == "fixed";
    if _cse_temp_2 {
    return Ok(false);
   
}
Ok(true)
}
#[doc = " Depyler: proven to terminate"] pub fn resolve_diverging_info<'c, 'a, 'b>(semantic_type: & 'a str, annotation: & 'b std::collections::HashMap<String, DepylerValue>, values: & 'c Vec<f64>) -> Result<Option<std::collections::HashMap<String, DepylerValue>>, IndexError>{
    let entry = get_registry_entry(& semantic_type);
    let _cse_temp_0 = semantic_type == "Temperature";
    let _cse_temp_1  = (_cse_temp_0) &&(annotation.get("unit").cloned());
    if _cse_temp_1 {
    let unit_midpoints: std::collections::HashMap<String, DepylerValue>= {
    let mut map: HashMap<String, DepylerValue>= HashMap::new();
    map.insert("°C".to_string(), DepylerValue::Int(0 as i64));
    map.insert("°F".to_string(), DepylerValue::Int(32 as i64));
    map.insert("K".to_string(), DepylerValue::Float(273.15 as f64));
    map.insert("C".to_string(), DepylerValue::Int(0 as i64));
    map.insert("F".to_string(), DepylerValue::Int(32 as i64));
    map };
    let mid = unit_midpoints.get(& annotation.get("unit").cloned().unwrap_or_default()).cloned();
    if mid.is_some() {
    return Ok(Some({ let mut map: HashMap<String, DepylerValue>= HashMap::new();
    map.insert(STR_MIDPOINT.to_string(), DepylerValue::Str(format!("{:?}", mid)));
    map.insert(STR_INHERENT.to_string(), DepylerValue::Bool(false));
    map.insert(STR_SOURCE.to_string(), DepylerValue::Str("unit".to_string().to_string()));
    map }));
   
}
} let _cse_temp_2 = entry.get("diverging").cloned().unwrap_or_default() == STR_INHERENT;
    if _cse_temp_2 {
    return Ok(Some({ let mut map: HashMap<String, DepylerValue>= HashMap::new();
    map.insert(STR_MIDPOINT.to_string(), DepylerValue::Int(0 as i64));
    map.insert(STR_INHERENT.to_string(), DepylerValue::Bool(true));
    map.insert(STR_SOURCE.to_string(), DepylerValue::Str("type-intrinsic".to_string().to_string()));
    map }));
   
}
let _cse_temp_3 = entry.get("diverging").cloned().unwrap_or_default() == "conditional";
    if _cse_temp_3 {
    return Ok(Some({ let mut map: HashMap<String, DepylerValue>= HashMap::new();
    map.insert(STR_MIDPOINT.to_string(), DepylerValue::Int(0 as i64));
    map.insert(STR_INHERENT.to_string(), DepylerValue::Bool(false));
    map.insert(STR_SOURCE.to_string(), DepylerValue::Str("type-intrinsic".to_string().to_string()));
    map }));
   
}
if annotation.get("intrinsicDomain").cloned().is_some() {
    let d = annotation.get("intrinsicDomain").cloned().unwrap_or_default();
    return Ok(Some({ let mut map: HashMap<String, DepylerValue>= HashMap::new();
    map.insert(STR_MIDPOINT.to_string(), DepylerValue::Str(format!("{:?}" ,(((d.get(0usize).cloned().expect("IndexError: list index out of range")).py_add(d.get(1usize).cloned().expect("IndexError: list index out of range")) as i32)).join(2i32))));
    map.insert(STR_INHERENT.to_string(), DepylerValue::Bool(false));
    map.insert(STR_SOURCE.to_string(), DepylerValue::Str("domain".to_string().to_string()));
    map }));
   
}
let _cse_temp_4 = values.len() as i32;
    let _cse_temp_5 = _cse_temp_4>0;
    if _cse_temp_5 {
    let _cse_temp_6 = * values.iter().min ().expect("empty collection");
    let mn = _cse_temp_6;
    let _cse_temp_7 = * values.iter().max().expect("empty collection");
    let mx = _cse_temp_7;
    let _cse_temp_8 = mn<0;
    let _cse_temp_9 = mx>0;
    let _cse_temp_10  = (_cse_temp_8) &&(_cse_temp_9);
    if _cse_temp_10 {
    return Ok(Some({ let mut map: HashMap<String, DepylerValue>= HashMap::new();
    map.insert(STR_MIDPOINT.to_string(), DepylerValue::Int(0 as i64));
    map.insert(STR_INHERENT.to_string(), DepylerValue::Bool(false));
    map.insert(STR_SOURCE.to_string(), DepylerValue::Str("data".to_string().to_string()));
    map }));
   
}
} Ok(None)
}
#[doc = " Depyler: proven to terminate"] pub fn resolve_color_scheme_hint<'c, 'a, 'b>(semantic_type: & 'a str, annotation: & 'b std::collections::HashMap<String, DepylerValue>, values: & 'c Vec<DepylerValue>) -> Result<std::collections::HashMap<String, DepylerValue>, IndexError>{
    let entry = get_registry_entry(& semantic_type);
    let nums: Vec<DepylerValue>= values.iter().cloned().filter(| v | {
    let v = v.clone();
    _is_number(v) }).map(| v | v).collect::<Vec<_>>();
    let div_info = resolve_diverging_info(& semantic_type, & annotation, & nums) ?;
    if let Some(ref div_info_val) = div_info {
    let mn = if ! nums.is_empty() {
    * nums.iter().min ().expect("empty collection")
}
else {
    0 };
    let mx = if ! nums.is_empty() {
    * nums.iter().max().expect("empty collection")
}
else {
    0 };
    let _cse_temp_0 = mn<div_info_val.get("midpoint").cloned().unwrap_or_default();
    let _cse_temp_1 = mx>div_info_val.get("midpoint").cloned().unwrap_or_default();
    let _cse_temp_2  = (_cse_temp_0) &&(_cse_temp_1);
    let spans_both = _cse_temp_2;
    let _cse_temp_3  = (div_info_val.get("inherent").cloned().unwrap_or_default()) ||(spans_both);
    if _cse_temp_3 {
    return Ok({ let mut map: HashMap<String, DepylerValue>= HashMap::new();
    map.insert(STR_TYPE.to_string(), DepylerValue::Str(STR_DIVERGING.to_string()));
    map.insert("divergingMidpoint".to_string(), DepylerValue::Str(format!("{:?}", div_info_val.get("midpoint").cloned().unwrap_or_default())));
    map.insert("inherentlyDiverging".to_string(), DepylerValue::Str(format!("{:?}", div_info_val.get("inherent").cloned().unwrap_or_default())));
    map });
   
}
} let _cse_temp_4 = entry.get("visEncodings").cloned().unwrap_or_default().contains("quantitative");
    if _cse_temp_4 {
    return Ok({ let mut map: HashMap<String, DepylerValue>= HashMap::new();
    map.insert(STR_TYPE.to_string(), DepylerValue::Str(STR_SEQUENTIAL.to_string()));
    map });
   
}
Ok({ let mut map: HashMap<String, DepylerValue>= HashMap::new();
    map.insert(STR_TYPE.to_string(), DepylerValue::Str(STR_CATEGORICAL.to_string()));
    map })
}
#[doc = " Depyler: proven to terminate"] pub fn resolve_binning_suggested<'a, 'b>(semantic_type: & 'a str, domain: & 'b Option<Vec<f64>>) -> Result<bool, IndexError>{
    let entry = get_registry_entry(& semantic_type);
    let _cse_temp_0 = ! entry.get("visEncodings").cloned().unwrap_or_default().contains("quantitative");
    if _cse_temp_0 {
    return Ok(false);
   
}
let _cse_temp_1 = ["identifier", "dimension"].contains(&& entry.get("aggRole").cloned().unwrap_or_default());
    if _cse_temp_1 {
    return Ok(false);
   
}
let _cse_temp_2 = [STR_YEAR, STR_DECADE].contains(&semantic_type);
    if _cse_temp_2 {
    return Ok(false);
   
}
let _cse_temp_3  = (domain.get(1usize).cloned().expect("IndexError: list index out of range")) - (domain.get(0usize).cloned().expect("IndexError: list index out of range"));
    let _cse_temp_4 = _cse_temp_3 <= 20;
    let _cse_temp_5  = (domain.is_some()) &&(_cse_temp_4);
    if let Some(ref _cse_temp_5_val) = _cse_temp_5 {
    return Ok(false);
   
}
let _cse_temp_6 = semantic_type == "Score";
    let _cse_temp_7  = (_cse_temp_6) &&(false);
    if _cse_temp_7 {
    return Ok(false);
   
}
Ok(true)
}
#[doc = " Depyler: proven to terminate"] pub fn resolve_stackable(semantic_type: & str) -> Result<String, IndexError>{
    let entry = get_registry_entry(& semantic_type);
    let role = entry.get("aggRole").cloned().unwrap_or_default();
    let _cse_temp_0 = ["additive", "signed-additive"].contains(&& role);
    if _cse_temp_0 {
    return Ok(STR_SUM.to_string());
   
}
let _cse_temp_1 = role == "intensive";
    if _cse_temp_1 {
    let _cse_temp_2 = semantic_type == "Percentage";
    if _cse_temp_2 {
    return Ok("normalize".to_string().to_string());
   
}
return Ok(false);
   
}
Ok(false)
}
#[doc = " Depyler: verified panic-free"] #[doc = " Depyler: proven to terminate"] pub fn resolve_sort_direction(semantic_type: & str) -> String {
    let _cse_temp_0 = semantic_type == STR_RANK;
    if _cse_temp_0 {
    return "descending".to_string().to_string();
   
}
"ascending".to_string().to_string()
}
#[doc = " Depyler: proven to terminate"] pub fn resolve_field_semantics<'a, 'b>(input_value: & 'a DepylerValue, _field_name: String, values: & 'b Vec<DepylerValue>) -> Result<std::collections::HashMap<String, DepylerValue>, IndexError>{
    let mut zero_class: String = Default::default();
    let mut binning_suggested: bool = Default::default();
    let mut aggregation_default: String = Default::default();
    let annotation = normalize_annotation(input_value);
    let semantic_type = annotation.get("semanticType").cloned().unwrap_or_default();
    let numeric_values: Vec<DepylerValue>= values.iter().cloned().filter(| v | {
    let v = v.clone();
   (((true) &&(! true)) &&(!(true) &&((v as f64).is_nan()))) &&((v as f64).is_finite()) }).map(| v | v).collect::<Vec<_>>();
    let default_vis_type = resolve_default_vis_type(semantic_type.clone(), & values) ?;
    let fmt_result = resolve_format(semantic_type.clone(), & annotation, & values) ?;
    aggregation_default = Some(resolve_aggregation_default(semantic_type.clone()) ?);
    zero_class = resolve_zero_class_from_annotation(semantic_type.clone(), Some(annotation.get("intrinsicDomain").cloned())) ?;
    let scale_type = resolve_scale_type(semantic_type.clone(), & numeric_values) ?;
    let domain_constraint = resolve_domain_constraint(semantic_type.clone(), & annotation, & values) ?;
    let canonical_order = resolve_canonical_order(semantic_type.clone(), & annotation, & values) ?;
    let cyclic = resolve_cyclic(semantic_type.clone()) ?;
    binning_suggested = resolve_binning_suggested(semantic_type.clone(), Some(annotation.get("intrinsicDomain").cloned())) ?;
    let sort_direction = resolve_sort_direction(semantic_type.clone());
    let _cse_temp_0 = default_vis_type == STR_QUANTITATIVE;
    let _cse_temp_1  = (! is_registered(semantic_type)) &&(_cse_temp_0);
    if _cse_temp_1 {
    if false {
    aggregation_default = STR_SUM.to_string();
   
}
let _cse_temp_2 = zero_class == STR_UNKNOWN_1;
    if _cse_temp_2 {
    zero_class = "meaningful".to_string();
   
}
binning_suggested = true;
   
}
let mut out: std::collections::HashMap<String, DepylerValue>= {
    let mut map: HashMap<String, DepylerValue>= HashMap::new();
    map.insert("semanticAnnotation".to_string(), DepylerValue::Str(format!("{:?}", annotation)));
    map.insert("defaultVisType".to_string(), DepylerValue::Str(default_vis_type.to_string()));
    map.insert("aggregationDefault".to_string(), DepylerValue::Str(aggregation_default.to_string()));
    map.insert(STR_ZEROCLASS.to_string(), DepylerValue::Str(zero_class.to_string()));
    map.insert("scaleType".to_string(), DepylerValue::Str(format!("{:?}", scale_type)));
    map.insert("domainConstraint".to_string(), DepylerValue::Str(format!("{:?}", domain_constraint)));
    map.insert("canonicalOrder".to_string(), DepylerValue::Str(format!("{:?}", canonical_order)));
    map.insert("cyclic".to_string(), DepylerValue::Bool(cyclic));
    map.insert("sortDirection".to_string(), DepylerValue::Str(sort_direction.to_string()));
    map.insert("binningSuggested".to_string(), DepylerValue::Bool(binning_suggested));
    map };
    let _cse_temp_3 = fmt_result.get("format").is_some();
    if _cse_temp_3 {
    out.insert("format".to_string(), fmt_result.get("format").cloned().unwrap_or_default());
   
}
let _cse_temp_4 = fmt_result.get(STR_TOOLTIPFORMAT).is_some();
    if _cse_temp_4 {
    out.insert(STR_TOOLTIPFORMAT.to_string(), fmt_result.get("tooltipFormat").cloned().unwrap_or_default());
   
}
Ok(out)
}
#[doc = " Depyler: verified panic-free"] #[doc = " Depyler: proven to terminate"] pub fn filter_overflow<'a, 'b, 'c>(channel_semantics: & 'a std::collections::HashMap<String, std::collections::HashMap<String, DepylerValue>>, declaration: & 'b std::collections::HashMap<String, DepylerValue>, _encodings: std::collections::HashMap<String, std::collections::HashMap<String, DepylerValue>>, data: & 'c Vec<std::collections::HashMap<String, DepylerValue>>, _budgets: std::collections::HashMap<String, DepylerValue>, _all_mark_types: std::collections::HashSet<String>) -> std::collections::HashMap<String, DepylerValue>{
    fn effective_type(ch: & str) -> Option<String>{
    let rt = declaration.get("resolvedTypes").cloned().unwrap_or({ let mut map: HashMap<String, String>= HashMap::new();
    map
}
. to_string()).get(& ch).cloned();
    if rt.is_some() {
    return rt.to_string();
   
}
let mut cs = channel_semantics.get(& ch).cloned();
    return cs.as_ref().map(| cs_val | cs_val.get("type").cloned());
   
}
fn effective_field(ch: & str) -> Option<String>{
    let mut cs = channel_semantics.get(& ch).cloned();
    if(cs.is_some()) &&(cs.get("field").cloned()) {
    return Some(cs.get("field").cloned().unwrap_or_default());
   
}
return None;
   
}
fn is_discrete_type(t: Option<String>) -> bool {
    return(t == STR_NOMINAL) ||(t == STR_ORDINAL);
   
}
let mut nominal_counts: std::collections::HashMap<String, i32>= {
    let mut map: HashMap<String, i32>= HashMap::new();
    map.insert(STR_X.to_string() ,(0) as i32);
    map.insert(STR_Y.to_string() ,(0) as i32);
    map.insert(STR_COLUMN.to_string() ,(0) as i32);
    map.insert(STR_ROW.to_string() ,(0) as i32);
    map.insert(STR_GROUP.to_string() ,(0) as i32);
    map };
    let truncations: Vec<std::collections::HashMap<String, DepylerValue>>= vec! [];
    let warnings: Vec<std::collections::HashMap<String, DepylerValue>>= vec! [];
    let filtered_data = data.clone();
    let group_cs = channel_semantics.get("group").cloned();
    let group_field = group_cs.as_ref().map(| group_cs_val | group_cs_val.get("field").cloned());
    if group_field {
    let _cse_temp_0 = data.iter().cloned().map(| r | r.get(& group_field).cloned()).collect::<std::collections::HashSet<_>>().len() as i32;
    nominal_counts.insert(STR_GROUP.to_string(), _cse_temp_0);
   
}
let _cse_temp_1 = declaration.get("overflowStrategy").cloned().unwrap_or(_default_overflow_strategy.to_string());
    let strategy: impl Fn() = _cse_temp_1;
    {
    let mut map: HashMap<String, DepylerValue>= HashMap::new();
    map.insert("filteredData".to_string(), DepylerValue::Str(format!("{:?}", filtered_data)));
    map.insert("nominalCounts".to_string(), DepylerValue::Str(format!("{:?}", nominal_counts)));
    map.insert("truncations".to_string(), DepylerValue::Str(format!("{:?}", truncations)));
    map.insert("warnings".to_string(), DepylerValue::Str(format!("{:?}", warnings)));
    map
}
} #[doc = "JS Array.prototype.sort() default coerces to string."] #[doc = " Depyler: verified panic-free"] #[doc = " Depyler: proven to terminate"] pub fn _js_sort_key(v: & DepylerValue) -> String {
    if false {
    return "null".to_string().to_string();
   
}
(v).to_string()
}
#[doc = " Depyler: proven to terminate"] pub fn _default_overflow_strategy<'b, 'l1, 'a, 'c>(channel: & 'a str, field_name: & 'b str, unique_values: & 'c Vec<DepylerValue>, max_to_keep: i32, context: & 'l1 std::collections::HashMap<String, DepylerValue>) -> Result<Vec<DepylerValue>, Box<dyn std::error::Error>>{
    let mut sort_field: Option<String>= Default::default();
    let mut sort_field_type: Option<String>= Default::default();
    let mut initial_value: i32 = Default::default();
    let data = context.get("data").cloned().unwrap_or_default();
    let channel_semantics = context.get("channelSemantics").cloned().unwrap_or_default();
    let encodings = context.get("encodings").cloned().unwrap_or_default();
    let all_mark_types = context.get("allMarkTypes").cloned().unwrap_or_default();
    let _cse_temp_0 = encodings.get(channel).cloned().unwrap_or({ let mut map: HashMap<String, DepylerValue>= HashMap::new();
    map
});
. to_string());
    let encoding = _cse_temp_0.clone();
    let sort_by = encoding.get("sortBy").cloned();
    let sort_order = encoding.get("sortOrder").cloned();
    let mut is_descending = false;
    if let Some(ref sort_by_val) = sort_by {
    let _cse_temp_1 = [STR_X, STR_Y, STR_COLOR].contains(&& sort_by_val);
    if _cse_temp_1 {
    let sort_cs = channel_semantics.get(& sort_by_val).cloned();
    sort_field = sort_cs.as_ref().map(| sort_cs_val | sort_cs_val.get("field").cloned());
    sort_field_type = sort_cs.as_ref().map(| sort_cs_val | sort_cs_val.get("type").cloned());
    let _cse_temp_2 = sort_order.unwrap_or_default() == "descending".to_string();
    let _cse_temp_3 = sort_order.unwrap_or_default() != "ascending".to_string();
    let _cse_temp_4 = sort_by_val.unwrap_or_default() != channel;
    let _cse_temp_5  = (_cse_temp_3) &&(_cse_temp_4);
    let _cse_temp_6  = (_cse_temp_2) ||(_cse_temp_5);
    is_descending = _cse_temp_6;
   
}
else {
    match(|| -> Result <(), Box<dyn std::error::Error>>{
    let sorted_list = std::collections::HashMap::<String, DepylerValue>::new();
    if true {
    let ordered_values = if sort_order.unwrap_or_default() == "descending".to_string() {
    sorted_list.iter().cloned().rev().into_iter().collect::<Vec<_>>()
}
else {
    sorted_list };
    return Ok({ let base = & * ordered_values.iter().cloned().filter(| v | {
    let v = v.clone();
    unique_values.contains(& v) }).map(| v | v).collect::<Vec<_>>();
    let stop_idx  = (max_to_keep) as isize;
    let stop = if stop_idx<0 {
   (base.len() as isize + stop_idx).max(0) as usize
}
else {
    stop_idx as usize };
    base [..stop.min (base.len())].to_vec() });
   
}
Ok(()) })() {
    Ok(()) =>{
   
}
Err(_e) =>{
   
}
} let _cse_temp_7 = sort_order.unwrap_or_default() == "descending".to_string();
    is_descending = _cse_temp_7;
   
}
} let _cse_temp_8 = sort_field_type == STR_QUANTITATIVE;
    let _cse_temp_9  = (sort_field) &&(_cse_temp_8);
    if let Some(ref _cse_temp_9_val) = _cse_temp_9 {
    let _cse_temp_10 = all_mark_types.contains("bar");
    let _cse_temp_11 = sort_field != ((channel_semantics.get("color").cloned().unwrap_or({ let mut map: HashMap<String, String>= HashMap::new();
    map })) ||({ let mut map: HashMap<String, String>= HashMap::new();
    map }).get("field").cloned());
    let _cse_temp_12  = (_cse_temp_10) &&(_cse_temp_11);
    let is_bar = _cse_temp_12;
    let mut aggregate_op;
    if is_bar {
    aggregate_op = move | x: i32, y: i32 |(x).py_add(y);
    initial_value = 0;
   
}
else {
    aggregate_op = max;
    let _cse_temp_13 = "-inf".parse::<f64>().expect("parse failed");
    initial_value = _cse_temp_13;
   
}
let mut value_aggregates: std::collections::HashMap<DepylerValue, f64>= {
    let mut map: HashMap <(), f64>= HashMap::new();
    map };
    for row in data.iter().cloned() {
    let field_value = row.get(field_name).cloned();
    let sort_value = row.get(& sort_field).cloned().unwrap_or(0.to_string());
    if value_aggregates.get(& field_value).is_some() {
    value_aggregates.insert(field_value.to_string().clone(), aggregate_op(value_aggregates.get(& DepylerValue::Int(field_value as i64)).cloned().unwrap_or_default(), sort_value));
   
}
else {
    value_aggregates.insert(field_value.to_string().clone(), aggregate_op(initial_value, sort_value));
   
}
} let mut entries: Vec<DepylerValue>= value_aggregates.iter().map(|(k, v) |(k.clone(), v.clone())).collect::<Vec<_>>().into_iter().map(|(v, a) |(v, a)).collect::<Vec<_>>();
    entries.sort_by_key(| x | move | e: i32 | if is_descending {
    - e.clone().py_index((1 as i64) as i64).unwrap_or_default()
}
else {
    e.clone().py_index((1 as i64) as i64).unwrap_or_default()
}
(x));
    return Ok({ let base = & * entries;
    let stop_idx  = (max_to_keep) as isize;
    let stop = if stop_idx<0 {
   (base.len() as isize + stop_idx).max(0) as usize
}
else {
    stop_idx as usize };
    base [..stop.min (base.len())].to_vec()
}
. into_iter().map(| e | e.clone().py_index((0 as i64) as i64).unwrap_or_default()).collect::<Vec<_>>());
   
}
let canonical_order = channel_semantics.get(channel).cloned().unwrap_or({ let mut map: HashMap<String, String>= HashMap::new();
    map
}
. to_string()).get("ordinalSortOrder").cloned();
    let _cse_temp_14  = (false) &&(false);
    let _cse_temp_15  = (_cse_temp_14) &&(canonical_order.is_some());
    if _cse_temp_15 {
    let mut ordered: Vec<DepylerValue>= canonical_order.iter().cloned().filter(| value | {
    let value = value.clone();
    unique_values.contains(& value) }).map(| value | value).collect::<Vec<_>>();
    let ordered_set = ordered.iter().cloned().collect::<std::collections::HashSet<_>>();
    ordered.extend(unique_values.iter().cloned().filter(| & value | ! ordered_set.contains(& value)).map(| value | value).iter().cloned());
    return Ok({ let base = & * ordered;
    let stop_idx  = (max_to_keep) as isize;
    let stop = if stop_idx<0 {
   (base.len() as isize + stop_idx).max(0) as usize
}
else {
    stop_idx as usize };
    base [..stop.min (base.len())].to_vec() });
   
}
let field_original_type = infer_vis_category(data.as_slice().iter().cloned().map(| r | r.get(field_name).cloned()).collect::<Vec<_>>());
    let _cse_temp_16 = [STR_COLUMN, STR_ROW].contains(&channel);
    if _cse_temp_16 {
    return Ok({ let base = & * unique_values;
    let stop_idx  = (max_to_keep) as isize;
    let stop = if stop_idx<0 {
   (base.len() as isize + stop_idx).max(0) as usize
}
else {
    stop_idx as usize };
    base [..stop.min (base.len())].to_vec() });
   
}
Ok({ let base = & * unique_values;
    let stop_idx  = (max_to_keep) as isize;
    let stop = if stop_idx<0 {
   (base.len() as isize + stop_idx).max(0) as usize
}
else {
    stop_idx as usize };
    base [..stop.min (base.len())].to_vec() })
}
#[doc = " Depyler: verified panic-free"] #[doc = " Depyler: proven to terminate"] pub fn is_likely_timestamp(val: f64) -> bool {
    let _cse_temp_0 = 1000000000.0 <= val;
    let _cse_temp_1 = val<= (MAX_TIMESTAMP_SEC as f64);
    let _cse_temp_2  = (_cse_temp_0) &&(_cse_temp_1);
    if _cse_temp_2 {
    return true;
   
}
let _cse_temp_3  = (MAX_TIMESTAMP_SEC as f64)<val;
    let _cse_temp_4 = val<= (MAX_TIMESTAMP_MS as f64);
    let _cse_temp_5  = (_cse_temp_3) &&(_cse_temp_4);
    if _cse_temp_5 {
    return true;
   
}
false
}
#[doc = " Depyler: verified panic-free"] #[doc = " Depyler: proven to terminate"] pub fn timestamp_to_ms(val: f64) -> f64 {
    if val<= (MAX_TIMESTAMP_SEC as f64) {
   (val).py_mul(1000i32)
}
else {
    val
}
} #[doc = " Depyler: verified panic-free"] pub fn infer_implicit_semantic_type<'b, 'a>(field_name: & 'a str, values: & 'b Vec<DepylerValue>) -> String {
    let tokenized = DepylerRegexMatch::sub("([a-z0-9])([A-Z])", "\\1 \\2", field_name).to_lowercase();
    let tokens: Vec<DepylerValue>= DepylerRegexMatch::split("[^a-z0-9]+", tokenized).into_iter().filter(| token | {
    let token = token.clone();
    token }).map(| token | token).collect::<Vec<_>>();
    let _cse_temp_0 = ! tokens.contains("year");
    if _cse_temp_0 {
    return STR_EMPTY.to_string();
   
}
let observed: Vec<DepylerValue>= values.iter().cloned().filter(| value | {
    let value = value.clone();
   (! value.is_null()) &&(value != STR_EMPTY) }).map(| value | value).collect::<Vec<_>>();
    let _cse_temp_1 = observed.iter().cloned().map(| value |(value).to_string()).collect::<std::collections::HashSet<_>>().len() as i32;
    let _cse_temp_2 = _cse_temp_1 <= 1;
    if _cse_temp_2 {
    return STR_EMPTY.to_string();
   
}
for value in observed.iter().cloned() {
    if true {
    return STR_EMPTY.to_string();
   
}
match(|| -> Result <(), Box<dyn std::error::Error>>{
    let numeric  = (value) as f64;
    Ok(()) })() {
    Ok(()) =>{
   
}
Err(_e) =>{
    return STR_EMPTY.to_string();
   
}
} if(! numeric.is_integer()) ||(!(1500f64 <= numeric) &&(numeric <= 2200f64)) {
    return STR_EMPTY.to_string();
   
}
} STR_YEAR.to_string()
}
#[doc = " Depyler: verified panic-free"] #[doc = " Depyler: proven to terminate"] pub fn looks_like_date_string(s: & str) -> bool {
    let t = s.trim().to_string();
    DepylerRegexMatch::match_start("^\\d|^(jan|feb|mar|apr|may|jun|jul|aug|sep|oct|nov|dec)", t) != 0
}
#[doc = "Mirror JavaScript ``new Date(v)`` parsing. Returns a timezone-aware\n    ``datetime`` or ``None``(matching V8's NaN). Delegated to the shared\n    :mod:`flint.core.js_date` helpers so all date-parsing paths agree.\n    "] #[doc = " Depyler: verified panic-free"] #[doc = " Depyler: proven to terminate"] pub fn _parse_date(v: & DepylerValue) -> Result<Option<datetime>, Box<dyn std::error::Error>>{
    if true {
    return Ok(Some(v));
   
}
Ok(Some(js_date_parse(v)))
}
#[doc = " Depyler: verified panic-free"] #[doc = " Depyler: proven to terminate"] pub fn _utc_attrs(d: &mut DepylerDateTime) -> (i32, i32, i32, i32, i32, i32) {
    if false {
    d = d;
   
}
let u = d.astimezone(timezone.utc);
   (u.year() as i32 ,(u.month() as i32) - (1i32), u.day() as i32, u.hour() as i32, u.minute() as i32, u.second() as i32)
}
pub fn analyze_temporal_field(field_values: & Vec<DepylerValue>) -> Result<Option<std::collections::HashMap<String, DepylerValue>>, IndexError>{
    let mut d: DepylerValue = Default::default();
    let mut non_null: i32 = Default::default();
    let mut dates: Vec<DepylerDateTime>= vec! [];
    non_null = 0;
    for v in {
    let base = & * field_values;
    let stop_idx  = (100) as isize;
    let stop = if stop_idx<0 {
   (base.len() as isize + stop_idx).max(0) as usize
}
else {
    stop_idx as usize };
    base [..stop.min (base.len())].to_vec()
}
{
    if v.is_null() {
    continue;
   
}
non_null  = ((non_null).py_add(1i32)) as i32;
    d = Some(_parse_date(v) ?);
    if d.is_some() {
    dates.append(d);
   
}
} let _cse_temp_0 = dates.len() as i32;
    let _cse_temp_1 = _cse_temp_0<2;
    let _cse_temp_2  = (non_null).py_mul(0.5);
    let _cse_temp_3  = ((_cse_temp_0 as f64) as f64)<_cse_temp_2;
    let _cse_temp_4  = (_cse_temp_1) ||(_cse_temp_3);
    if _cse_temp_4 {
    return Ok(None);
   
}
let(mut years, mut months, mut days, mut hours, mut minutes, mut seconds)  = (0..(6)).into_iter().map(| _ | std::collections::HashSet::<i32>::new());
    for d in dates.iter().cloned() {
    let(y, mo, da, h, mi, se) = _utc_attrs(d);
    years.insert(DepylerValue::from(y));
    months.insert(DepylerValue::from(mo));
    days.insert(DepylerValue::from(da));
    hours.insert(DepylerValue::from(h));
    minutes.insert(DepylerValue::from(mi));
    seconds.insert(DepylerValue::from(se));
   
}
fn is_small_spread(s: &(), max_spread: i32) -> bool {
    if s.len() as i32 <= 1 {
    return Ok(true);
   
}
let arr = s.chars().collect::<Vec<_>>();
    return(*arr.iter().max().expect("empty collection")) - (*arr.iter().min ().expect("empty collection")) <= max_spread;
   
}
let same: std::collections::HashMap<String, DepylerValue>= {
    let mut map: HashMap<String, DepylerValue>= HashMap::new();
    map.insert("month".to_string(), DepylerValue::Str(format!("{:?}", months.len() as i32 == 1)));
    map.insert("day".to_string(), DepylerValue::Str(format!("{:?}", days.len() as i32 == 1)));
    map.insert("hour".to_string(), DepylerValue::Str(format!("{:?}", is_small_spread(& hours, 1))));
    map.insert("minute".to_string(), DepylerValue::Str(format!("{:?}", minutes.len() as i32 == 1)));
    map.insert("second".to_string(), DepylerValue::Str(format!("{:?}", seconds.len() as i32 == 1)));
    map };
    let _cse_temp_5 = years.len() as i32;
    let _cse_temp_6 = _cse_temp_5 == 1;
    let same_year = _cse_temp_6;
    let _cse_temp_7 : i32 = (same_year) &&(same.get("month").cloned().unwrap_or_default().into());
    let same_month = _cse_temp_7;
    let _cse_temp_8 : i32 = (same_month) &&(same.get("day").cloned().unwrap_or_default().into());
    let same_day = _cse_temp_8;
    Ok(Some({ let mut map: HashMap<String, DepylerValue>= HashMap::new();
    map.insert("dates".to_string(), DepylerValue::Str(format!("{:?}", dates)));
    map.insert("same".to_string(), DepylerValue::Str(format!("{:?}", same)));
    map.insert("sameYear".to_string(), DepylerValue::Bool(same_year));
    map.insert("sameMonth".to_string(), DepylerValue::Bool(same_month));
    map.insert("sameDay".to_string(), DepylerValue::Bool(same_day));
    map }))
}
#[doc = " Depyler: proven to terminate"] pub fn compute_data_votes(same: & std::collections::HashMap<String, bool>) -> Result<Vec<i32>, IndexError>{
    let mut votes: Vec<i32>= vec! [0, 0, 0, 0, 0, 0];
    if same.get("second").cloned().unwrap_or_default().as_str().is_some_and(| s | ! s.is_empty()) {
    let _cse_temp_0  = ((votes.get(5usize).cloned().expect("IndexError: list index out of range")).py_add(1i32)) as i32;
    votes [(5) as usize] = _cse_temp_0;
   
}
let _cse_temp_1 = {
    let _and_lhs = same.get("minute").cloned().unwrap_or_default();
    if ! _and_lhs.is_true() {
    _and_lhs
}
else {
    same.get("second").cloned().unwrap_or_default()
}
};
    if _cse_temp_1 {
    let _cse_temp_2  = ((votes.get(5usize).cloned().expect("IndexError: list index out of range")).py_add(1i32)) as i32;
    votes [(5) as usize] = _cse_temp_2;
   
}
let _cse_temp_3  = (_cse_temp_1) &&(same.get("second").cloned().unwrap_or_default());
    if _cse_temp_3 {
    let _cse_temp_4  = ((votes.get(5usize).cloned().expect("IndexError: list index out of range")).py_add(1i32)) as i32;
    votes [(5) as usize] = _cse_temp_4;
   
}
let _cse_temp_5  = (_cse_temp_3) &&(same.get("second").cloned().unwrap_or_default());
    if _cse_temp_5 {
    let _cse_temp_6  = ((votes.get(5usize).cloned().expect("IndexError: list index out of range")).py_add(2i32)) as i32;
    votes [(5) as usize] = _cse_temp_6;
   
}
let _cse_temp_7  = (_cse_temp_5) &&(same.get("second").cloned().unwrap_or_default());
    if _cse_temp_7 {
    let _cse_temp_8  = ((votes.get(5usize).cloned().expect("IndexError: list index out of range")).py_add(3i32)) as i32;
    votes [(5) as usize] = _cse_temp_8;
   
}
if same.get("second").cloned().unwrap_or_default().as_str().is_some_and(| s | ! s.is_empty()) {
    let _cse_temp_9  = ((votes.get(4usize).cloned().expect("IndexError: list index out of range")).py_add(1i32)) as i32;
    votes [(4) as usize] = _cse_temp_9;
   
}
if _cse_temp_1 {
    let _cse_temp_10  = ((votes.get(4usize).cloned().expect("IndexError: list index out of range")).py_add(1i32)) as i32;
    votes [(4) as usize] = _cse_temp_10;
   
}
if _cse_temp_3 {
    let _cse_temp_11  = ((votes.get(4usize).cloned().expect("IndexError: list index out of range")).py_add(1i32)) as i32;
    votes [(4) as usize] = _cse_temp_11;
   
}
if _cse_temp_5 {
    let _cse_temp_12  = ((votes.get(4usize).cloned().expect("IndexError: list index out of range")).py_add(2i32)) as i32;
    votes [(4) as usize] = _cse_temp_12;
   
}
let _cse_temp_13 = {
    let _and_lhs = DepylerValue::from(! same.get("month").cloned().unwrap_or_default());
    if ! _and_lhs.is_true() {
    _and_lhs
}
else {
    same.get("day").cloned().unwrap_or_default()
}
};
    let _cse_temp_14  = (_cse_temp_13) &&(same.get("hour").cloned().unwrap_or_default());
    let _cse_temp_15  = (_cse_temp_14) &&(same.get("minute").cloned().unwrap_or_default());
    let _cse_temp_16  = (_cse_temp_15) &&(same.get("second").cloned().unwrap_or_default());
    if _cse_temp_16 {
    let _cse_temp_17  = ((votes.get(4usize).cloned().expect("IndexError: list index out of range")).py_add(3i32)) as i32;
    votes [(4) as usize] = _cse_temp_17;
   
}
if same.get("second").cloned().unwrap_or_default().as_str().is_some_and(| s | ! s.is_empty()) {
    let _cse_temp_18  = ((votes.get(3usize).cloned().expect("IndexError: list index out of range")).py_add(1i32)) as i32;
    votes [(3) as usize] = _cse_temp_18;
   
}
if _cse_temp_1 {
    let _cse_temp_19  = ((votes.get(3usize).cloned().expect("IndexError: list index out of range")).py_add(1i32)) as i32;
    votes [(3) as usize] = _cse_temp_19;
   
}
if _cse_temp_3 {
    let _cse_temp_20  = ((votes.get(3usize).cloned().expect("IndexError: list index out of range")).py_add(1i32)) as i32;
    votes [(3) as usize] = _cse_temp_20;
   
}
if _cse_temp_15 {
    let _cse_temp_21  = ((votes.get(3usize).cloned().expect("IndexError: list index out of range")).py_add(3i32)) as i32;
    votes [(3) as usize] = _cse_temp_21;
   
}
if same.get("second").cloned().unwrap_or_default().as_str().is_some_and(| s | ! s.is_empty()) {
    let _cse_temp_22  = ((votes.get(2usize).cloned().expect("IndexError: list index out of range")).py_add(1i32)) as i32;
    votes [(2) as usize] = _cse_temp_22;
   
}
if _cse_temp_1 {
    let _cse_temp_23  = ((votes.get(2usize).cloned().expect("IndexError: list index out of range")).py_add(1i32)) as i32;
    votes [(2) as usize] = _cse_temp_23;
   
}
if _cse_temp_14 {
    let _cse_temp_24  = ((votes.get(2usize).cloned().expect("IndexError: list index out of range")).py_add(3i32)) as i32;
    votes [(2) as usize] = _cse_temp_24;
   
}
if same.get("second").cloned().unwrap_or_default().as_str().is_some_and(| s | ! s.is_empty()) {
    let _cse_temp_25  = ((votes.get(1usize).cloned().expect("IndexError: list index out of range")).py_add(1i32)) as i32;
    votes [(1) as usize] = _cse_temp_25;
   
}
if _cse_temp_13 {
    let _cse_temp_26  = ((votes.get(1usize).cloned().expect("IndexError: list index out of range")).py_add(3i32)) as i32;
    votes [(1) as usize] = _cse_temp_26;
   
}
if ! same.get("second").cloned().unwrap_or_default() {
    let _cse_temp_27  = ((votes.get(0usize).cloned().expect("IndexError: list index out of range")).py_add(4i32)) as i32;
    votes [(0) as usize] = _cse_temp_27;
   
}
Ok(votes)
}
#[doc = " Depyler: proven to terminate"] pub fn pick_best_level(votes: & Vec<i32>) -> Result<std::collections::HashMap<String, i32>, IndexError>{
    let mut best_level: i32 = Default::default();
    let mut best_score: i32 = Default::default();
    best_level = 0;
    best_score = votes.get(0usize).cloned().expect("IndexError: list index out of range").into();
    for i in (1)..(6) {
    if votes.get(i as usize).cloned().expect("IndexError: list index out of range")>= best_score {
    best_score = votes.get(i as usize).cloned().expect("IndexError: list index out of range").into();
    best_level = i;
   
}
} Ok({ let mut map: HashMap<String, i32>= HashMap::new();
    map.insert("level".to_string() ,(best_level) as i32);
    map.insert("score".to_string() ,(best_score) as i32);
    map })
}
#[doc = " Depyler: verified panic-free"] #[doc = " Depyler: proven to terminate"] pub fn level_to_format(level: i32, analysis: & std::collections::HashMap<String, DepylerValue>) -> Result<Option<String>, IndexError>{
    let _cse_temp_0 = level == 5;
    if _cse_temp_0 {
    return Ok(Some("%Y".to_string().to_string()));
   
}
let _cse_temp_1 = level == 4;
    if _cse_temp_1 {
    return Ok(Some(if analysis.get("sameYear").cloned().unwrap_or_default() {
    "%b"
}
else {
    "%b %Y" }));
   
}
let _cse_temp_2 = level == 3;
    if _cse_temp_2 {
    return Ok(Some(if analysis.get("sameYear").cloned().unwrap_or_default() {
    "%b %d"
}
else {
    "%b %d, %Y" }));
   
}
let _cse_temp_3 = level == 2;
    if _cse_temp_3 {
    return Ok(Some(if analysis.get("sameDay").cloned().unwrap_or_default() {
    "%H:00"
}
else {
    "%b %d %H:00" }));
   
}
let _cse_temp_4 = level == 1;
    if _cse_temp_4 {
    return Ok(Some(if analysis.get("sameDay").cloned().unwrap_or_default() {
    "%H:%M"
}
else {
    "%b %d %H:%M" }));
   
}
let _cse_temp_5 = level == 0;
    if _cse_temp_5 {
    return Ok(Some(if analysis.get("sameDay").cloned().unwrap_or_default() {
    "%H:%M:%S"
}
else {
    "%b %d %H:%M:%S" }));
   
}
Ok(None)
}
#[doc = " Depyler: proven to terminate"] pub fn _resolve_temporal_format<'b, 'a>(field_values: & 'a Vec<DepylerValue>, semantic_type: & 'b str) -> Result<Option<String>, IndexError>{
    let analysis = analyze_temporal_field(& field_values) ?;
    if false {
    return Ok(None);
   
}
let mut votes = compute_data_votes(analysis.expect("value is None").get("same").cloned().unwrap_or_default()) ?;
    let sem_level = SEMANTIC_LEVEL.get(semantic_type).cloned();
    if sem_level.is_some() {
    let _cse_temp_0  = ((votes.get(sem_level as usize).cloned().expect("IndexError: list index out of range")).py_add(3i32)) as i32;
    votes [(sem_level) as usize] = _cse_temp_0;
   
}
let pick = pick_best_level(& votes) ?;
    Ok(Some(level_to_format(pick.get("level").cloned().unwrap_or_default(), analysis.expect("value is None"))))
}
#[doc = " Depyler: verified panic-free"] #[doc = " Depyler: proven to terminate"] pub fn _expand_to_full_year(val: String) -> String {
    let trimmed = val.trim().to_string();
    if DepylerRegexMatch::match_start("^\\d{2}$", trimmed) {
    let _cse_temp_0 = trimmed.parse::<i32>().unwrap_or_default();
    let n = _cse_temp_0;
    return(if n <= 49 {
   (2000i32).py_add(n)
}
else {
   (1900i32).py_add(n) }).to_string();
   
}
val.to_string().to_string()
}
#[doc = "Mirror JS Date.toISOString(): YYYY-MM-DDTHH:mm:ss.sssZ(always Zulu)."] #[doc = " Depyler: proven to terminate"] pub fn _to_iso_z(dt: &mut DepylerDateTime) -> Result<String, ZeroDivisionError>{
    if false {
    dt = dt;
   
}
let u = dt.astimezone(timezone.utc);
    let _cse_temp_0  = ((u.microsecond() as i32).join(1000i32)) as i32;
    let _cse_temp_1  = (_cse_temp_0) as i32;
    let ms = _cse_temp_1;
    Ok(format!("{}-{}-{}T{}:{}:{}.{}Z", u.year() as i32, u.month() as i32, u.day() as i32, u.hour() as i32, u.minute() as i32, u.second() as i32, ms))
}
pub fn convert_temporal_data(data: Vec<std::collections::HashMap<String, DepylerValue>>, semantic_types: & std::collections::HashMap<String, DepylerValue>) -> Result<Vec<std::collections::HashMap<String, DepylerValue>>, IndexError>{
    let mut st = Default::default();
    if data.is_empty() {
    return Ok(data);
   
}
let keys = data.get(0usize).cloned().expect("IndexError: list index out of range").keys().cloned().collect::<Vec<_>>();
    let field_values: std::collections::HashMap<DepylerValue, Vec<DepylerValue>>= keys.iter().cloned().map(| key | {
    let _v = data.iter().cloned().map(| row | row.get(& key).cloned()).collect::<Vec<_>>();
   (key, _v) }).collect::<std::collections::HashMap<_, _>>();
    let effective_semantic_types: std::collections::HashMap<DepylerValue, DepylerValue>= keys.iter().cloned().map(| key | {
    let _v  = (to_type_string(semantic_types.get(& key).cloned())) ||(infer_implicit_semantic_type(key.to_string(), field_values.get(&(key)).cloned().unwrap_or_default()));
   (key, _v) }).collect::<std::collections::HashMap<_, _>>();
    let mut temporal_keys: Vec<String>= vec! [];
    for k in keys.iter().cloned() {
    st = effective_semantic_types.get(& DepylerValue::Int(k as i64)).cloned().unwrap_or_default();
    let vc = infer_vis_category(field_values.get(& DepylerValue::Int(k as i64)).cloned().unwrap_or_default());
    let st_category = if st {
    get_vis_category(Some(st))
}
else {
    None };
    if((vc == STR_TEMPORAL) ||(st_category == STR_TEMPORAL)) ||(st == STR_DECADE) {
    temporal_keys.append(k);
   
}
} if temporal_keys.is_empty() {
    return Ok(data);
   
}
let values  = (data).clone();
    for r in values.iter().cloned() {
    for temporal_key in temporal_keys.iter().cloned() {
    let val = r.get(& temporal_key).cloned();
    st = effective_semantic_types.get(&(temporal_key)).cloned().unwrap_or_default().into();
    if true {
    r.insert(temporal_key.to_string().clone() ,(val).expect("value is None").to_string().to_lowercase());
    continue;
   
}
if true {
    if [STR_YEAR, STR_DECADE].contains(&& st) {
    r.insert(temporal_key.to_string().clone(), format!("{}" ,((val as f64).floor()) as i32));
   
}
else {
    if is_likely_timestamp(val) {
    let dt = DepylerDateTime::fromtimestamp((timestamp_to_ms(val)).join(1000.0) as f64);
    r.insert(temporal_key.to_string().clone(), _to_iso_z(dt) ?);
   
}
else {
    r.insert(temporal_key.to_string().clone(), _js_number_to_string(val));
   
}
}
}
else {
    if true {
    r.insert(temporal_key.to_string().clone(), _to_iso_z(val) ?);
   
}
else {
    if true {
    if [STR_YEAR, STR_DECADE].contains(&& st) {
    r.insert(temporal_key.to_string().clone(), _expand_to_full_year(val));
   
}
else {
    r.insert(temporal_key.to_string().clone(), val);
   
}
} else {
    if false {
    r.insert(temporal_key.to_string().clone(), "null".to_string());
   
}
else {
    r.insert(temporal_key.to_string().clone() ,(val).expect("value is None").to_string());
   
}
}
}
}
}
} Ok(values)
}
#[doc = "Mirror JS String(<number>) — integers without trailing zeros."] #[doc = " Depyler: verified panic-free"] #[doc = " Depyler: proven to terminate"] pub fn _js_number_to_string(v: f64) -> String {
    if true {
    if v.is_integer() {
    return((v) as i32).to_string();
   
}
return format!("{:?}", v);
   
}
(v).to_string()
}
pub fn resolve_channel_semantics<'a, 'b, 'c, 'l1>(encodings: & 'a std::collections::HashMap<String, std::collections::HashMap<String, DepylerValue>>, data: & 'b Vec<std::collections::HashMap<String, DepylerValue>>, semantic_types: & 'c std::collections::HashMap<String, DepylerValue>, converted_data: & 'l1 Option<Vec<std::collections::HashMap<String, DepylerValue>>>) -> Result<std::collections::HashMap<String, std::collections::HashMap<String, DepylerValue>>, IndexError>{
    let mut supplied_semantic_type: String = Default::default();
    let mut resolved_type: i32 = Default::default();
    let mut result: std::collections::HashMap<String, std::collections::HashMap<String, DepylerValue>>= {
    let mut map: HashMap<String, std::collections::HashMap<String ,()>>= HashMap::new();
    map };
    let temporal_data = if converted_data.is_some() {
    converted_data
}
else {
    data };
    for(channel, encoding) in encodings.iter().map(|(k, v) |(k.clone(), v.clone())) {
    let field_name = encoding.get("field").cloned();
    if(field_name.is_none()) &&(encoding.get("aggregate").cloned() != STR_COUNT) {
    continue;
   
}
if(field_name.is_none()) &&(encoding.get("aggregate").cloned() == STR_COUNT) {
    result.insert(channel.to_string().clone(), {
    let mut map: HashMap<String, DepylerValue>= HashMap::new();
    map.insert("field".to_string(), DepylerValue::Str("_count".to_string()));
    map.insert("semanticAnnotation".to_string(), DepylerValue::Dict({ let mut map: HashMap<String, DepylerValue>= HashMap::new();
    map.insert(STR_SEMANTICTYPE.to_string(), DepylerValue::Str("Count".to_string()));
    map
}
. into_iter().map(|(k, v) |(DepylerValue::Str(k), v)).collect()));
    map.insert(STR_TYPE.to_string(), DepylerValue::Str(STR_QUANTITATIVE.to_string()));
    map.insert("aggregationDefault".to_string(), DepylerValue::Str(STR_SUM.to_string()));
    map });
    continue;
   
}
if false {
    continue;
   
}
let raw_annotation = semantic_types.get(& field_name.expect("value is None")).cloned();
    if true {
    supplied_semantic_type = Some(if raw_annotation.is_empty() {
    STR_EMPTY.to_string()
}
else {
    raw_annotation.to_string() });
   
}
else {
    if true {
    supplied_semantic_type = raw_annotation.get("semanticType").cloned().unwrap_or(STR_EMPTY.to_string());
   
}
else {
    supplied_semantic_type = STR_EMPTY.to_string();
   
}
} let field_values: Vec<std::collections::HashMap<String, DepylerValue>>= data.iter().cloned().map(| r | r.get(& field_name.expect("value is None")).cloned()).collect::<Vec<_>>();
    let semantic_type  = (! supplied_semantic_type.is_empty()) ||(infer_implicit_semantic_type(field_name.expect("value is None"), & field_values));
    let type_decision = resolve_encoding_type(semantic_type, & field_values, channel.to_string(), & data, field_name.expect("value is None")) ?;
    resolved_type = type_decision.get("vlType").cloned().unwrap_or_default().into();
    if encoding.get("type").cloned().is_some() {
    resolved_type = encoding.get("type").cloned().unwrap_or_default().into();
   
}
else {
    if [STR_COLUMN, STR_ROW].contains(&& channel) {
    if ! [STR_NOMINAL, STR_ORDINAL].contains(&& resolved_type) {
    resolved_type = STR_NOMINAL.to_string();
   
}
}
}
if resolved_type == STR_QUANTITATIVE {
    let sample_values: Vec<std::collections::HashMap<String, DepylerValue>>= {
    let base = & * data;
    let stop_idx  = (15) as isize;
    let stop = if stop_idx<0 {
   (base.len() as isize + stop_idx).max(0) as usize
}
else {
    stop_idx as usize };
    base [..stop.min (base.len())].to_vec()
}
. into_iter().filter(| r | {
    let mut r = r.clone();
    r.get(& field_name.expect("value is None")).cloned().is_some() }).map(| r | r.get(& field_name.expect("value is None")).cloned()).collect::<Vec<_>>();
    let iso_re = "^\\d{4}-\\d{2}-\\d{2}T\\d{2}:\\d{2}:\\d{2}(\\.\\d+)?(Z|[+-]\\d{2}:\\d{2})?$".to_string();
    if(! sample_values.is_empty()) &&(sample_values.iter().cloned().map(| v | iso_re.find((v).to_string().trim().to_string())).all(| x | x)) {
    resolved_type = STR_TEMPORAL.to_string();
   
}
} let fc = resolve_field_semantics((raw_annotation.is_some()) ||(semantic_type.is_some()), field_name.expect("value is None"), & field_values) ?;
    let annotation = fc.get("semanticAnnotation").cloned().unwrap_or_default();
    let tick_constraint = resolve_tick_constraint(annotation.get("semanticType").cloned().unwrap_or_default(), Some(annotation.get("intrinsicDomain").cloned())) ?;
    let reversed_ = resolve_reversed(annotation.get("semanticType").cloned().unwrap_or_default(), Some(channel));
    let nice = resolve_nice(annotation.get("semanticType").cloned().unwrap_or_default(), Some(fc.get("domainConstraint").cloned())) ?;
    let stackable = resolve_stackable(annotation.get("semanticType").cloned().unwrap_or_default()) ?;
    let mut cs: std::collections::HashMap<String, DepylerValue>= {
    let mut map: HashMap<String, DepylerValue>= HashMap::new();
    map.insert("field".to_string(), DepylerValue::Str(format!("{:?}", field_name.expect("value is None"))));
    map.insert("semanticAnnotation".to_string(), DepylerValue::Str(format!("{:?}", annotation)));
    map.insert(STR_TYPE.to_string(), DepylerValue::Str(resolved_type.to_string()));
    map };
    if fc.get("format").cloned().is_some().is_some() {
    cs.insert("format".to_string(), fc.get("format").cloned().unwrap_or_default());
   
}
if fc.get("tooltipFormat").cloned().is_some().is_some() {
    cs.insert(STR_TOOLTIPFORMAT.to_string(), fc.get("tooltipFormat").cloned().unwrap_or_default());
   
}
if fc.get("aggregationDefault").cloned().is_some().is_some() {
    cs.insert("aggregationDefault".to_string(), fc.get("aggregationDefault").cloned().unwrap_or_default());
   
}
if fc.get("scaleType").cloned().is_some().is_some() {
    cs.insert("scaleType".to_string(), fc.get("scaleType").cloned().unwrap_or_default());
   
}
if fc.get("domainConstraint").cloned().is_some().is_some() {
    cs.insert("domainConstraint".to_string(), fc.get("domainConstraint").cloned().unwrap_or_default());
   
}
if fc.get("cyclic").cloned().is_some() {
    cs.insert("cyclic".to_string(), DepylerValue::Bool(true));
   
}
if fc.get("sortDirection").cloned().is_some().is_some() {
    cs.insert("sortDirection".to_string(), fc.get("sortDirection").cloned().unwrap_or_default());
   
}
if fc.get("binningSuggested").cloned().is_some() {
    cs.insert("binningSuggested".to_string(), DepylerValue::Bool(true));
   
}
cs.insert("nice".to_string(), nice);
    if tick_constraint.is_some() {
    cs.insert("tickConstraint".to_string(), tick_constraint);
   
}
if reversed_ {
    cs.insert("reversed".to_string(), DepylerValue::Bool(true));
   
}
cs.insert("stackable".to_string(), stackable.clone());
    if encoding.get("aggregate").cloned().is_some() {
    if encoding.get("aggregate").cloned().unwrap_or_default().into() == STR_COUNT {
    cs.insert("field".to_string(), DepylerValue::Str("_count".to_string()));
    cs.insert(STR_TYPE.to_string(), DepylerValue::Str(STR_QUANTITATIVE.to_string()));
   
}
else {
    cs.insert("field".to_string(), format!("{:?}_{}", field_name.expect("value is None"), encoding.get("aggregate").cloned().unwrap_or_default().into()));
    cs.insert(STR_TYPE.to_string(), DepylerValue::Str(STR_QUANTITATIVE.to_string()));
   
}
} if([STR_COLOR, STR_GROUP].contains_key(&& channel)) &&(field_name.expect("value is None").is_some()) {
    if {
    let _and_lhs = encoding.get("scheme").cloned();
    if ! _and_lhs.is_true() {
    _and_lhs
}
else {
    DepylerValue::from(encoding.get("scheme").cloned().unwrap_or_default().into() != "default")
}
} {
    cs.insert("colorScheme".to_string(), format!("{:?}", {
    let mut map = HashMap::new();
    map.insert(STR_SCHEME.to_string(), encoding.get("scheme").cloned().unwrap_or_default().into());
    map.insert(STR_TYPE.to_string(), STR_CATEGORICAL.to_string());
    map.insert(STR_REASON.to_string(), "explicit user scheme".to_string());
    map }));
   
}
else {
    let encoding_vl_type = cs.get("type").cloned().unwrap_or_default();
    let color_hint = resolve_color_scheme_hint(semantic_type, annotation, & field_values) ?;
    let unique_values = field_values.iter().cloned().map(| v | v).collect::<std::collections::HashSet<_>>();
    cs.insert("colorScheme".to_string(), get_recommended_color_scheme(semantic_type, encoding_vl_type, unique_values.len() as i32, field_name.expect("value is None"), & field_values, & {
    let mut map = HashMap::new();
    map.insert(STR_TYPE.to_string(), color_hint.get("type").cloned().unwrap_or_default());
    map }) ?);
    if(cs.get("colorScheme").cloned().unwrap_or_default().get("type").cloned().unwrap_or_default() == STR_DIVERGING) &&(encoding_vl_type == STR_QUANTITATIVE) {
    let nums: Vec<DepylerValue>= field_values.iter().cloned().filter(| v | {
    let v = v.clone();
   ((true) &&(! true)) &&(!(true) &&((v as f64).is_nan())) }).map(| v | v).collect::<Vec<_>>();
    let div_info = resolve_diverging_info(semantic_type, annotation, & nums) ?;
    if let Some(ref div_info_val) = div_info {
    cs.get_mut("colorScheme").expect("key not found in dict").insert("domainMid".to_string(), div_info_val.get("midpoint").cloned().unwrap_or_default());
   
}
}
}
} if(cs.get("type").cloned().unwrap_or_default() == STR_TEMPORAL) ||((semantic_type.is_some()) &&(get_vis_category(semantic_type) ? == STR_TEMPORAL)) {
    let converted_field_values: Vec<DepylerValue>= temporal_data.as_slice().iter().cloned().map(| r | r.get(& field_name.expect("value is None")).cloned()).collect::<Vec<_>>();
    let fmt = _resolve_temporal_format(& converted_field_values, semantic_type) ?;
    if let Some(ref fmt_val) = fmt {
    cs.insert("temporalFormat".to_string(), fmt_val);
   
}
} if [STR_ORDINAL, STR_NOMINAL].contains(&& cs.get("type").cloned().unwrap_or_default()) {
    if(! encoding.get("sortOrder").cloned()) &&(! encoding.get("sortBy").cloned()) {
    let ordinal_sort = infer_ordinal_sort_order(semantic_type, & field_values) ?;
    if let Some(ref ordinal_sort_val) = ordinal_sort {
    cs.insert("ordinalSortOrder".to_string(), ordinal_sort_val);
   
}
}
}
result.insert(channel.to_string().clone(), cs.clone());
   
}
Ok(result)
}
#[doc = r" DEPYLER-1216: Auto-generated entry point wrapping top-level script statements"] #[doc = r" This file was transpiled from a Python script with executable top-level code."] pub fn main () -> Result <(), Box<dyn std::error::Error>>{
    Ok(())
}
#[cfg(test)] mod tests {
    use super::*;
    use quickcheck::{
    quickcheck, TestResult };
    #[test] fn test_is_js_parseable_examples() {
    let _ = is_js_parseable(Default::default());
   
}
#[test] fn test_is_registered_examples() {
    let _ = is_registered(Default::default());
   
}
#[test] fn test__is_boolean_examples() {
    let _ = _is_boolean(Default::default());
   
}
#[test] fn test__is_number_like_examples() {
    let _ = _is_number_like(Default::default());
   
}
#[test] fn test__looks_like_date_string_examples() {
    let _ = _looks_like_date_string(Default::default());
   
}
#[test] fn test__is_date_like_examples() {
    let _ = _is_date_like(Default::default());
   
}
#[test] fn test__date_parse_succeeds_examples() {
    let _ = _date_parse_succeeds(Default::default());
   
}
#[test] fn test_is_measure_type_examples() {
    let _ = is_measure_type(Default::default());
   
}
#[test] fn test_is_time_series_type_examples() {
    let _ = is_time_series_type(Default::default());
   
}
#[test] fn test_is_categorical_type_examples() {
    let _ = is_categorical_type(Default::default());
   
}
#[test] fn test_is_ordinal_type_examples() {
    let _ = is_ordinal_type(Default::default());
   
}
#[test] fn test_is_geo_type_examples() {
    let _ = is_geo_type(Default::default());
   
}
#[test] fn test_is_geo_coordinate_type_examples() {
    let _ = is_geo_coordinate_type(Default::default());
   
}
#[test] fn test_is_geo_location_string_examples() {
    let _ = is_geo_location_string(Default::default());
   
}
#[test] fn test_is_non_measure_numeric_examples() {
    let _ = is_non_measure_numeric(Default::default());
   
}
#[test] fn test__data_far_from_zero_examples() {
    let _ = _data_far_from_zero(Default::default());
   
}
#[test] fn quickcheck_infer_ordinal_sort_order() {
    fn prop(semantic_type: String, values: Vec <()>) -> TestResult {
    let mut result = infer_ordinal_sort_order((& * semantic_type).into(), & values);
    for i in 1..result.len() {
    if result [i - 1]>result [i] {
    return TestResult::failed();
   
}
} let mut input_sorted = semantic_type.clone();
    input_sorted.sort();
    let mut result = infer_ordinal_sort_order((& * semantic_type).into());
    result.sort();
    if input_sorted != result {
    return TestResult::failed();
   
}
TestResult::passed()
}
quickcheck(prop as fn(String, Vec <()>) -> TestResult);
   
}
#[test] fn test__looks_temporal_value_examples() {
    let _ = _looks_temporal_value(Default::default());
   
}
#[test] fn test__can_parse_float_examples() {
    let _ = _can_parse_float(Default::default());
   
}
#[test] fn test__is_finite_number_examples() {
    let _ = _is_finite_number(Default::default());
   
}
#[test] fn test__is_nan_examples() {
    let _ = _is_nan(Default::default());
   
}
#[test] fn test__is_measure_enc_examples() {
    let _ = _is_measure_enc(Default::default());
   
}
#[test] fn test__is_discrete_category_enc_examples() {
    let _ = _is_discrete_category_enc(Default::default());
   
}
#[test] fn quickcheck__resolve_sort_channels() {
    fn prop(encodings :(), candidates :()) -> TestResult {
    let mut result = _resolve_sort_channels(& encodings, candidates.clone());
    for i in 1..result.len() {
    if result [i - 1]>result [i] {
    return TestResult::failed();
   
}
} let mut input_sorted = encodings.clone();
    input_sorted.sort();
    let mut result = _resolve_sort_channels(& encodings);
    result.sort();
    if input_sorted != result {
    return TestResult::failed();
   
}
TestResult::passed()
}
quickcheck(prop as fn(() ,()) -> TestResult);
   
}
#[test] fn quickcheck_make_sort_action() {
    fn prop(key: String, label: String, channels :()) -> TestResult {
    let mut result = make_sort_action((& * key).into() ,(& * label).into(), channels.clone());
    for i in 1..result.len() {
    if result [i - 1]>result [i] {
    return TestResult::failed();
   
}
} let mut input_sorted = key.clone();
    input_sorted.sort();
    let mut result = make_sort_action((& * key).into());
    result.sort();
    if input_sorted != result {
    return TestResult::failed();
   
}
TestResult::passed()
}
quickcheck(prop as fn(String, String ,()) -> TestResult);
   
}
#[test] fn quickcheck_normalize_annotation() {
    fn prop(input_value :()) -> TestResult {
    let once = normalize_annotation(input_value.clone());
    let twice = normalize_annotation(once.clone());
    if once != twice {
    return TestResult::failed();
   
}
TestResult::passed()
}
quickcheck(prop as fn(()) -> TestResult);
   
}
#[test] fn test__is_number_examples() {
    let _ = _is_number(Default::default());
   
}
#[test] fn test__detect_precision_examples() {
    assert_eq!(_detect_precision(& vec! []), 0);
    assert_eq!(_detect_precision(& vec! [1]), 1);
    assert_eq!(_detect_precision(& vec! [1, 2, 3]), 3);
   
}
#[test] fn test__try_float_examples() {
    let _ = _try_float(Default::default());
   
}
#[test] fn test_resolve_cyclic_examples() {
    let _ = resolve_cyclic(Default::default());
   
}
#[test] fn quickcheck_resolve_sort_direction() {
    fn prop(semantic_type: String) -> TestResult {
    let mut result = resolve_sort_direction((& * semantic_type).into());
    for i in 1..result.len() {
    if result [i - 1]>result [i] {
    return TestResult::failed();
   
}
} let mut input_sorted = semantic_type.clone();
    input_sorted.sort();
    let mut result = resolve_sort_direction((& * semantic_type).into());
    result.sort();
    if input_sorted != result {
    return TestResult::failed();
   
}
TestResult::passed()
}
quickcheck(prop as fn(String) -> TestResult);
   
}
#[test] fn quickcheck__js_sort_key() {
    fn prop(v :()) -> TestResult {
    let mut result = _js_sort_key(v.clone());
    for i in 1..result.len() {
    if result [i - 1]>result [i] {
    return TestResult::failed();
   
}
} let mut input_sorted = v.clone();
    input_sorted.sort();
    let mut result = _js_sort_key(v.clone());
    result.sort();
    if input_sorted != result {
    return TestResult::failed();
   
}
TestResult::passed()
}
quickcheck(prop as fn(()) -> TestResult);
   
}
#[test] fn test_is_likely_timestamp_examples() {
    let _ = is_likely_timestamp(Default::default());
   
}
#[test] fn test_looks_like_date_string_examples() {
    let _ = looks_like_date_string(Default::default());
   
}
}