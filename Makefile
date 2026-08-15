# flint-typst — test, benchmark, release.
#
#   make test         everything (generated tables, differential, conformance)
#   make differential just the per-function comparison against flint-py
#   make conformance  just the 705-case corpus replay
#   make probes       re-check Typst's capabilities (after a Typst upgrade)
#   make bench        core vs lilaq, at five chart shapes
#   make corpus       regenerate the oracle from upstream flint-py
#   make tables       regenerate the generated data tables
#   make link         link datehog into Typst's local package namespace
#   make setup        create the venv the Python-side tests need
#
# Everything except `bench` and `probes` needs the venv; `corpus`,
# `differential` and `tables` additionally need the upstream clone.

SHELL := /bin/bash

VENV     := .venv
PY       := $(VENV)/bin/python
UPSTREAM := flint-source
FLINT_PY := $(UPSTREAM)/packages/flint-py
FIXTURES := $(UPSTREAM)/shared/test-data
DATEHOG  := $(HOME)/.local/share/typst/packages/local/datehog

.PHONY: all test differential conformance generated probes bench corpus tables setup link upstream clean

all: test

# ---------------------------------------------------------------- testing ---
test:
	PY=$(PY) ./test/run.sh

differential:
	PY=$(PY) ./test/run.sh differential

conformance:
	PY=$(PY) ./test/run.sh conformance

generated:
	PY=$(PY) ./test/run.sh generated

# Capability probes behind docs/WHY-TYPST-NOT-WASM.md. A `*-GAP.typ` probe that
# starts *passing* means Typst gained a capability and that document is stale.
probes:
	./test/typst-probes/run.sh

# ------------------------------------------------------------- benchmarks ---
# Numbers land in bench/BENCH.md by hand — these just print them.
bench:
	@printf '%-30s %11s %11s\n' shape lilaq core
	@printf 'x' > bench/_base.typ; \
	s=$$(date +%s%N); typst compile --root . --format pdf bench/_base.typ /tmp/flint-bench.pdf >/dev/null 2>&1; \
	e=$$(date +%s%N); base=$$(( (e-s)/1000000 )); rm -f bench/_base.typ; \
	for cfg in "20 32 1" "20 180 1" "20 500 1" "10 3000 1" "10 3000 20"; do \
	  set -- $$cfg; \
	  s=$$(date +%s%N); typst compile --root . --format pdf --input n=$$1 --input pts=$$2 --input series=$$3 bench/lilaq-bench.typ /tmp/flint-bench.pdf >/dev/null 2>&1; e=$$(date +%s%N); \
	  lq=$$(( ((e-s)/1000000 - base) / $$1 )); \
	  s=$$(date +%s%N); typst compile --root . --format pdf --input n=$$1 --input pts=$$2 --input series=$$3 bench/core-bench.typ /tmp/flint-bench.pdf >/dev/null 2>&1; e=$$(date +%s%N); \
	  co=$$(( ((e-s)/1000000 - base) / $$1 )); \
	  printf '%-30s %8s ms %8s ms\n' "$$2 pts, $$3 series" "$$lq" "$$co"; \
	done; rm -f /tmp/flint-bench.pdf

# ----------------------------------------------------------- regeneration ---
# TZ=UTC is load-bearing: flint reads zoneless date strings in the *host* zone,
# so 15 of the 705 fixtures would otherwise record machine-dependent values.
corpus: | $(FLINT_PY)
	TZ=UTC $(PY) test/make_corpus.py --flint-py $(FLINT_PY) --fixtures $(FIXTURES) -o test/corpus

tables: | $(FLINT_PY)
	$(PY) test/gen_tables.py

# ------------------------------------------------------------------ setup ---
setup: $(VENV)/bin/python link

$(VENV)/bin/python:
	uv venv $(VENV)
	VIRTUAL_ENV=$(VENV) uv pip install pytest python-dateutil

# datehog is a sibling package, not yet published; link it so `@local/datehog`
# resolves. Becomes `@preview/datehog` once it is on Typst Universe.
link:
	@mkdir -p $(DATEHOG)
	@ln -sfn "$(CURDIR)/../datehog" $(DATEHOG)/0.1.0
	@echo "linked $(DATEHOG)/0.1.0 -> ../datehog"

# The upstream clone is the reference implementation the tests compare against,
# and the source of the fixture corpus. Not vendored — it is someone else's repo.
upstream: $(FLINT_PY)

$(FLINT_PY):
	git clone --depth 1 https://github.com/microsoft/flint-chart.git $(UPSTREAM)

clean:
	rm -f test/_case.json test/_cases.json test/typst-probes/_*.typ bench/_*.typ
