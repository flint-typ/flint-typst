# Type-completeness audit

`annotated` = has any annotation. `concrete` = annotation contains no `Any`/`object` and no bare container — i.e. a transpiler can lower it to a real Rust type.

| file | loc | fns | params ann/concrete | returns ann/concrete | concrete score |
|---|---:|---:|---:|---:|---:|
| `core/encoding_overrides.py` | 38 | 1 | 100.0% /   0.0% | 100.0% /   0.0% | **0.0%** |
| `core/filter_overflow.py` | 210 | 7 | 100.0% /  37.5% | 100.0% /  71.4% | **47.8%** |
| `core/color_decisions.py` | 95 | 5 | 100.0% /  55.6% | 100.0% /  40.0% | **50.0%** |
| `core/encoding_actions.py` | 116 | 7 | 100.0% /  45.5% | 100.0% /  71.4% | **55.6%** |
| `core/decisions.py` | 457 | 18 |  84.6% /  63.1% |  88.9% /  44.4% | **59.0%** |
| `core/compute_layout.py` | 1241 | 22 | 100.0% /  50.0% | 100.0% /  81.8% | **60.0%** |
| `core/field_semantics.py` | 596 | 26 | 100.0% /  64.0% |  96.2% /  61.5% | **63.2%** |
| `core/resolve_semantics.py` | 453 | 17 | 100.0% /  52.0% | 100.0% /  82.4% | **64.3%** |
| `core/js_date.py` | 198 | 5 | 100.0% /  40.0% | 100.0% / 100.0% | **70.0%** |
| `core/semantic_types.py` | 549 | 24 | 100.0% /  72.2% | 100.0% /  91.7% | **80.0%** |
| `core/type_registry.py` | 119 | 3 | 100.0% / 100.0% | 100.0% /  66.7% | **80.0%** |
| `core/__init__.py` | 10 | 1 | 100.0% / 100.0% | 100.0% / 100.0% | **100.0%** |
| `core/types.py` | 24 | 0 | n/a / n/a | n/a / n/a | **100.0%** |
| **TOTAL** | **4106** | **136** | ** 96.3% /  57.9%** | ** 97.8% /  72.1%** | **62.7%** |

## Nominal types available to a transpiler

- classes: **0**
- dataclasses: **0**
- TypedDicts: **0**
- NamedTuples: **0**

## Transpiler hazards

| construct | count |
|---|---:|
| `dict_get` | 379 |
| `isinstance_dispatch` | 65 |
| `fstring` | 18 |
| `nested_function` | 17 |
| `try_except` | 14 |
| `dynamic_compile` | 6 |
| `lambda` | 5 |
| `copy_semantics` | 1 |
