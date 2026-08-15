# Flint core -> Rust -> wasm32 pipeline.
#
#   make audit        type-completeness report for flint-py's core
#   make flatten      collapse flint/core/ into one module for the transpiler
#   make transpile    depyler: flattened Python -> Rust
#   make build        cargo build (wasm32 plugin + native replay driver)
#   make corpus       record the 705-case core conformance corpus from flint-py
#   make conformance  replay the corpus against the Rust core
#   make validate     check the .wasm against Typst's wasmi feature set
#   make smoke        load the plugin in a real `typst compile`
#   make all          audit + flatten + build + corpus + validate + smoke
#
# Set up once with `make setup`.

SHELL := /bin/bash

FLINT_SRC   ?= ../flint-source
FLINT_PY    := $(FLINT_SRC)/packages/flint-py
CORE_PY     := $(FLINT_PY)/flint/core
FIXTURES    := $(FLINT_SRC)/shared/test-data

VENV        := .venv
PY          := $(VENV)/bin/python
DEPYLER     := ./tools-bin/bin/depyler

# The Arch system rust in /usr/bin has no wasm32-unknown-unknown std, and it
# shadows rustup's on PATH. rustup's cargo proxy resolves `rustc` from PATH
# too, so pointing at the toolchain's own binaries is not optional here.
TOOLCHAIN   := $(HOME)/.rustup/toolchains/stable-x86_64-unknown-linux-gnu
CARGO       := RUSTC=$(TOOLCHAIN)/bin/rustc $(TOOLCHAIN)/bin/cargo
WASM_TARGET := wasm32-unknown-unknown
WASM_OUT    := rust/target/$(WASM_TARGET)/release/flint_core_wasm.wasm
REPLAY_BIN  := rust/target/release/replay

.PHONY: all setup audit flatten transpile build wasm replay corpus conformance validate smoke typst-probes clean distclean

all: audit flatten build corpus validate smoke

# ---------------------------------------------------------------- setup ----
setup: $(VENV)/bin/python $(DEPYLER)
	@echo "setup complete"

$(VENV)/bin/python:
	uv venv $(VENV)
	VIRTUAL_ENV=$(VENV) uv pip install pyright pytest python-dateutil

$(DEPYLER):
	@test -d depyler-src || git clone --depth 1 https://github.com/paiml/depyler.git depyler-src
	cargo install --path depyler-src/crates/depyler --root ./tools-bin --locked

# ----------------------------------------------------------- type audit ----
audit:
	$(PY) audit/type_audit.py $(CORE_PY) --md audit/report-core.md --json audit/report-core.json
	@echo "-> audit/report-core.md"

# Guard for CI once typed/ is being built up: fail if concreteness regresses.
audit-gate:
	$(PY) audit/type_audit.py $(CORE_PY) --min-score 0.62

# ------------------------------------------------------------- flatten ----
# depyler transpiles one file at a time and drops cross-module imports, so the
# package has to become a single translation unit first.
flatten:
	$(PY) audit/flatten.py $(CORE_PY) -o typed/flint_core_flat.py

# ----------------------------------------------------------- transpile ----
transpile: flatten
	$(DEPYLER) transpile typed/flint_core_flat.py -o rust/src/generated/flint_core.rs
	@echo "-> rust/src/generated/flint_core.rs (does NOT compile as of depyler 4.1.2, see REPORT.md)"

# Per-file transpile, kept for diagnosing which module breaks codegen.
transpile-probe:
	@mkdir -p probe/depyler-out
	@for f in $(CORE_PY)/*.py; do \
	  b=$$(basename $$f .py); \
	  $(DEPYLER) transpile "$$f" -o "probe/depyler-out/$$b.rs" > "probe/depyler-out/$$b.log" 2>&1 \
	    && echo "OK   $$b" || echo "FAIL $$b"; \
	done

# --------------------------------------------------------------- build ----
build: wasm replay

wasm:
	cd rust && $(CARGO) build --target $(WASM_TARGET) --release
	@ls -la $(WASM_OUT)

replay:
	cd rust && $(CARGO) build --release --bin replay

# Attempt the depyler output. Expected to fail today; kept so the failure is
# reproducible and the error tally in REPORT.md can be regenerated.
build-generated:
	cd rust && $(CARGO) build --release --features generated 2>&1 | tee ../probe/generated-build.log | tail -40
	@echo "errors: $$(grep -cE '^error(\[|:)' probe/generated-build.log)"

# -------------------------------------------------------------- corpus ----
# TZ=UTC is load-bearing: flint's js_date interprets non-ISO date forms in
# LOCAL time (`.astimezone()`), so 15 of the 705 fixtures record different
# values depending on the build machine's timezone (the dates_hours__* and
# dates_year_month__*_mon_yyyy_* groups). Pinning UTC makes the corpus
# reproducible and lets the port be UTC-only. See datehog's README.
corpus:
	TZ=UTC $(PY) validate/make_core_corpus.py --flint-py $(FLINT_PY) --fixtures $(FIXTURES) -o corpus/core

conformance: replay
	$(PY) validate/check_conformance.py --corpus corpus/core --replay-bin $(REPLAY_BIN) -v

# ------------------------------------------------------------ validate ----
validate: wasm
	$(PY) validate/wasm_features.py $(WASM_OUT) --json validate/wasm-features.json

smoke: wasm
	@cp $(WASM_OUT) validate/flint_core_wasm.wasm
	cd validate && typst compile --format pdf typst_smoke.typ smoke.pdf && rm -f smoke.pdf
	@echo "plugin loads and answers under typst $$(typst --version | cut -d' ' -f2)"

# Capability probes behind TYPST-VS-WASM.md. Re-run after a Typst upgrade:
# a `*-GAP.typ` probe that starts passing means that document is stale.
typst-probes:
	probe/typst/run.sh

# --------------------------------------------------------------- clean ----
clean:
	rm -rf rust/target probe/depyler-out probe/generated-build.log validate/flint_core_wasm.wasm

distclean: clean
	rm -rf $(VENV) tools-bin depyler-src py2many-src corpus typed/flint_core_flat.py
