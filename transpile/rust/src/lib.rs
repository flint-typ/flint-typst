//! Flint chart core as a Typst wasm plugin.
//!
//! Typst's plugin runtime is `wasmi`, which implements the WebAssembly MVP
//! plus a handful of finished proposals but **not** the GC proposal. That
//! rules out the JS-preserving AOT compilers (Porffor, js2wasm) which target
//! WasmGC, and is why this crate exists: Rust on `wasm32-unknown-unknown`
//! manages its own memory in the linear heap and needs no GC support.
//!
//! ## ABI
//!
//! Every export takes one UTF-8 JSON argument and returns one UTF-8 JSON
//! result, so the Typst side is just:
//!
//! ```typst
//! #let core = plugin("flint_core_wasm.wasm")
//! #let out = json(core.compute_layout(bytes(json.encode(args))))
//! ```
//!
//! The argument shape mirrors the Python signature positionally, as recorded
//! by `validate/make_core_corpus.py`. Keeping the two in lockstep is what lets
//! `validate/check_conformance.py` replay 705 golden fixtures straight through
//! this plugin.

use wasm_minimal_protocol::*;

initiate_protocol!();

#[cfg(feature = "generated")]
pub mod generated;

pub mod core;

/// Decode a JSON argument, run `f`, and encode the result.
///
/// Errors come back as `{"__error__": "..."}` rather than a trap: a trapping
/// plugin gives Typst nothing to show the user but "plugin panicked", whereas
/// a structured error can be surfaced at the call site.
fn json_call<F>(input: &[u8], f: F) -> Vec<u8>
where
    F: FnOnce(serde_json::Value) -> Result<serde_json::Value, String>,
{
    let parsed = match serde_json::from_slice::<serde_json::Value>(input) {
        Ok(v) => v,
        Err(e) => return err_blob(&format!("invalid JSON argument: {e}")),
    };
    match f(parsed) {
        Ok(v) => serde_json::to_vec(&v).unwrap_or_else(|e| err_blob(&format!("unserializable result: {e}"))),
        Err(e) => err_blob(&e),
    }
}

fn err_blob(msg: &str) -> Vec<u8> {
    serde_json::to_vec(&serde_json::json!({ "__error__": msg })).expect("error blob is always serializable")
}

/// Report which core stages this build actually implements.
///
/// `check_conformance.py` calls this first and skips stages reported as
/// unimplemented, so the harness stays useful while the port is partial.
#[wasm_func]
pub fn capabilities() -> Vec<u8> {
    serde_json::to_vec(&serde_json::json!({
        "version": env!("CARGO_PKG_VERSION"),
        "backend": if cfg!(feature = "generated") { "depyler-generated" } else { "handwritten" },
        "stages": core::IMPLEMENTED_STAGES,
    }))
    .expect("capabilities blob is always serializable")
}

#[wasm_func]
pub fn convert_temporal_data(arg: &[u8]) -> Vec<u8> {
    json_call(arg, core::convert_temporal_data)
}

#[wasm_func]
pub fn resolve_channel_semantics(arg: &[u8]) -> Vec<u8> {
    json_call(arg, core::resolve_channel_semantics)
}

#[wasm_func]
pub fn compute_channel_budgets(arg: &[u8]) -> Vec<u8> {
    json_call(arg, core::compute_channel_budgets)
}

#[wasm_func]
pub fn filter_overflow(arg: &[u8]) -> Vec<u8> {
    json_call(arg, core::filter_overflow)
}

#[wasm_func]
pub fn compute_layout(arg: &[u8]) -> Vec<u8> {
    json_call(arg, core::compute_layout)
}
