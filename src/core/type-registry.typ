// Port of flint/core/type_registry.py.
//
// The registry itself is generated from upstream into `type-registry-data.typ`
// (45 entries of pure data — see test/gen_tables.py for why). The lookups over
// it are hand-ported and live here.

#import "type-registry-data.typ": TYPE_REGISTRY, UNKNOWN_ENTRY

// Re-exported so callers import one module, matching upstream's layout where
// the data and the accessors share a file.
#let TYPE_REGISTRY = TYPE_REGISTRY
#let UNKNOWN_ENTRY = UNKNOWN_ENTRY

// flint/core/type_registry.py get_registry_entry
#let get_registry_entry(semantic_type) = {
  TYPE_REGISTRY.at(semantic_type, default: UNKNOWN_ENTRY)
}

// flint/core/type_registry.py is_registered
#let is_registered(semantic_type) = semantic_type in TYPE_REGISTRY

// flint/core/type_registry.py get_registered_types
#let get_registered_types() = TYPE_REGISTRY.keys()
