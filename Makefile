# flint-typst — test, benchmark, release.
#
#   make test         value tests (generated tables, differential, backend, conformance)
#   make visual       reference-image tests for the lilaq backend (tytanic)
#   make visual-update  regenerate the reference images after an intended change
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

.PHONY: fixtures all test differential conformance generated backend gallery visual visual-update visual-mirror visual-cases probes bench corpus tables setup link upstream clean

all: test

# ---------------------------------------------------------------- testing ---
test:
	PY=$(PY) ./tests/run.sh

differential:
	PY=$(PY) ./tests/run.sh differential

conformance:
	PY=$(PY) ./tests/run.sh conformance

backend:
	PY=$(PY) ./tests/run.sh backend

# ------------------------------------------------------------ visual tests ---
# The value suites check that core's *decisions* match flint-py; these check
# that the backend turns those decisions into the right picture. They caught two
# bugs the value tests structurally cannot see: bars clipped at the frame, and a
# grouped bar chart drawing its series on top of each other instead of dodged.
#
# `tt update` is the only thing that may write into `ref/` — never hand-edit a
# reference. After an intended visual change, run `make visual-update` and *look
# at the diff* before committing it.
visual:
	tt run

visual-update:
	tt update
	@echo "references updated — LOOK AT THEM before committing"

# The cases live once in tests/cases.typ and every backend renders all of them,
# so adding a case covers every backend and adding a backend is one command.
visual-mirror:
	@test -n "$(BACKEND)" || { echo "usage: make visual-mirror BACKEND=lilaq"; exit 1; }
	$(PY) tests/gen_visual.py "$(BACKEND)"

visual-cases:
	@$(PY) tests/gen_visual.py --list

# Renders every chart type to a PDF you can actually look at.
gallery:
	typst compile --root . --format pdf tests/gallery.typ gallery.pdf
	@echo "-> gallery.pdf"

generated:
	PY=$(PY) ./tests/run.sh generated

# Capability probes behind docs/WHY-TYPST-NOT-WASM.md. A `*-GAP.typ` probe that
# starts *passing* means Typst gained a capability and that document is stale.
probes:
	./tests/typst-probes/run.sh

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
	TZ=UTC $(PY) tests/make_corpus.py --flint-py $(FLINT_PY) --fixtures $(FIXTURES) -o tests/corpus

tables: | $(FLINT_PY)
	$(PY) tests/gen_tables.py

# Lifts a few real inputs out of flint's fixture corpus into tests/fixtures.typ,
# so the scale cases draw data the conformance suite already vouches for.
fixtures: | $(FLINT_PY)
	$(PY) tests/gen_fixtures.py

# ------------------------------------------------------------------ setup ---
setup: $(VENV)/bin/python link

$(VENV)/bin/python:
	uv venv $(VENV)
	VIRTUAL_ENV=$(VENV) uv pip install -r requirements.txt

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
	rm -f tests/_case.json tests/_cases.json tests/typst-probes/_*.typ bench/_*.typ
