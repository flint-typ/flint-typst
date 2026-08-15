//! The flint core port.
//!
//! Each function here mirrors one entry point of `flint/core/` in flint-py.
//! The argument is the positional argument list from the Python signature,
//! encoded as a JSON array -- exactly the shape `make_core_corpus.py` records,
//! so a corpus case can be replayed verbatim.
//!
//! Stages listed in [`IMPLEMENTED_STAGES`] are checked by the conformance
//! harness; everything else is reported as unimplemented and skipped. Add a
//! stage name here only once it round-trips the corpus, so the harness never
//! reports a green run for a stub.

use serde_json::{json, Value};

/// Stage names this build answers for real. Kept in sync by hand: adding a
/// name here without an implementation makes the conformance run fail loudly,
/// which is the intended failure direction.
pub const IMPLEMENTED_STAGES: &[&str] = &[];

/// JSON has no literal for the non-finite floats, but flint's layout code uses
/// them meaningfully (`Infinity` as "no cap"). Bare `Infinity` in a JSON text
/// is rejected by `serde_json` and by Typst's `json()`, so the corpus, this
/// plugin's ABI and the core all agree on a tagged encoding instead:
/// `Infinity` -> `{"$f":"inf"}`, `-Infinity` -> `{"$f":"-inf"}`,
/// `NaN` -> `{"$f":"nan"}`.
///
/// Keep in sync with `NONFINITE` in `validate/make_core_corpus.py`.
pub fn nonfinite(x: f64) -> Value {
    if x.is_nan() {
        json!({ "$f": "nan" })
    } else if x == f64::INFINITY {
        json!({ "$f": "inf" })
    } else if x == f64::NEG_INFINITY {
        json!({ "$f": "-inf" })
    } else {
        json!(x)
    }
}

/// Inverse of [`nonfinite`]: read a number that may be tagged.
pub fn as_f64(v: &Value) -> Option<f64> {
    if let Some(n) = v.as_f64() {
        return Some(n);
    }
    match v.get("$f")?.as_str()? {
        "inf" => Some(f64::INFINITY),
        "-inf" => Some(f64::NEG_INFINITY),
        "nan" => Some(f64::NAN),
        _ => None,
    }
}


fn unimplemented_stage(name: &str) -> Result<Value, String> {
    Err(format!(
        "stage '{name}' is not implemented in this build; \
         see transpile/REPORT.md for the port status"
    ))
}

/// Pull positional argument `idx` out of the recorded `{"args": [...]}` payload.
#[allow(dead_code)]
pub(crate) fn arg(payload: &Value, idx: usize) -> Result<&Value, String> {
    payload
        .get("args")
        .and_then(|a| a.as_array())
        .ok_or_else(|| "payload has no 'args' array".to_string())?
        .get(idx)
        .ok_or_else(|| format!("missing positional argument {idx}"))
}

pub fn convert_temporal_data(_payload: Value) -> Result<Value, String> {
    unimplemented_stage("convert_temporal_data")
}

pub fn resolve_channel_semantics(_payload: Value) -> Result<Value, String> {
    unimplemented_stage("resolve_channel_semantics")
}

pub fn compute_channel_budgets(_payload: Value) -> Result<Value, String> {
    unimplemented_stage("compute_channel_budgets")
}

pub fn filter_overflow(_payload: Value) -> Result<Value, String> {
    unimplemented_stage("filter_overflow")
}

pub fn compute_layout(_payload: Value) -> Result<Value, String> {
    unimplemented_stage("compute_layout")
}
