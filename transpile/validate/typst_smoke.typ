// Smoke test for the wasm boundary: proves the plugin loads under Typst's
// wasmi runtime and answers on the JSON ABI. Run via `make smoke`.
//
// This deliberately does NOT check core logic -- that is what the 705-case
// conformance corpus is for, replayed natively. What can only be checked here
// is the part the native replay skips: that the binary is loadable by wasmi at
// all, and that the (ptr, len) -> i32 protocol round-trips.

#let core = plugin("flint_core_wasm.wasm")

#let caps = json(core.capabilities())

= Flint core plugin smoke test

Loaded plugin version *#caps.version* (backend: #caps.backend).

Stages implemented in this build:
#if caps.stages.len() == 0 [
  _none yet -- the plugin is a scaffold; see `../REPORT.md`._
] else [
  #caps.stages.join(", ")
]

== ABI round-trip

A call into an unimplemented stage must come back as a structured error rather
than trapping, so a partial build still renders something a user can read:

#let probe = json(core.compute_layout(bytes("{\"args\":[]}")))
#raw(probe.at("__error__", default: "unexpected: call succeeded"))

#assert(
  caps.version != none,
  message: "plugin did not answer capabilities()",
)
