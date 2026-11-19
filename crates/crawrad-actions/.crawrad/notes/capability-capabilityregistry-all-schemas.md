---
id: capability-capabilityregistry-all-schemas
kind: doc
target_file: src/capability.rs
line_hint: 760
author: 
created: 
title: "CapabilityRegistry::all_schemas"
tags: []
signature: "pub fn all_schemas (& self) -> BTreeMap < String , serde_json :: Value >"
---

Returns the schemas of all registered capabilities for MCP `tools/list`.

Returns a map containing the schemas of all registered capabilities (for MCP `tools/list`).

# Examples

```rust
use crawrad_actions::capability::CapabilityRegistry;

let reg = CapabilityRegistry::new();
assert!(reg.all_schemas().is_empty());
```
