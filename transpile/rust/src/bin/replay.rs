//! Corpus replay driver for the conformance harness.
//!
//! Reads one recorded corpus case as JSON on stdin, runs every recorded call
//! through the Rust core, and writes the actual results to stdout in the same
//! shape. `validate/check_conformance.py` diffs that against the recorded
//! Python results.
//!
//! This links the core as a native rlib rather than going through wasm: the
//! logic under test is identical, and skipping the wasm runtime keeps a
//! 705-case replay fast enough to run on every change. The wasm boundary
//! itself is covered separately by `validate/typst_smoke.typ`.

use std::io::{self, Read, Write};

use flint_core_wasm::core;
use serde_json::{json, Map, Value};

fn dispatch(stage: &str, payload: Value) -> Result<Value, String> {
    match stage {
        "convert_temporal_data" => core::convert_temporal_data(payload),
        "resolve_channel_semantics" => core::resolve_channel_semantics(payload),
        "compute_channel_budgets" => core::compute_channel_budgets(payload),
        "filter_overflow" => core::filter_overflow(payload),
        "compute_layout" => core::compute_layout(payload),
        other => Err(format!("unknown stage '{other}'")),
    }
}

fn main() -> io::Result<()> {
    let mut buf = String::new();
    io::stdin().read_to_string(&mut buf)?;

    let case: Value = match serde_json::from_str(&buf) {
        Ok(v) => v,
        Err(e) => {
            let out = json!({ "__error__": format!("bad corpus case: {e}") });
            io::stdout().write_all(serde_json::to_string(&out)?.as_bytes())?;
            return Ok(());
        }
    };

    let mut results: Map<String, Value> = Map::new();
    let empty = Map::new();
    let calls = case.get("calls").and_then(Value::as_object).unwrap_or(&empty);

    for (stage, recorded) in calls {
        let mut per_stage: Vec<Value> = Vec::new();
        for call in recorded.as_array().into_iter().flatten() {
            // Hand the driver the same {"args": [...], "kwargs": {...}} shape
            // the recorder captured, minus the expected result.
            let mut payload = Map::new();
            if let Some(a) = call.get("args") {
                payload.insert("args".into(), a.clone());
            }
            if let Some(k) = call.get("kwargs") {
                payload.insert("kwargs".into(), k.clone());
            }
            per_stage.push(match dispatch(stage, Value::Object(payload)) {
                Ok(v) => json!({ "result": v }),
                Err(e) => json!({ "__error__": e }),
            });
        }
        results.insert(stage.clone(), Value::Array(per_stage));
    }

    let out = json!({
        "slug": case.get("slug").cloned().unwrap_or(Value::Null),
        "implemented": core::IMPLEMENTED_STAGES,
        "calls": results,
    });
    io::stdout().write_all(serde_json::to_string(&out)?.as_bytes())?;
    Ok(())
}
