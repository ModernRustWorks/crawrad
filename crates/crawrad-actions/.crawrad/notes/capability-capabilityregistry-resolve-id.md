---
id: capability-capabilityregistry-resolve-id
kind: doc
target_file: src/capability.rs
line_hint: 768
author: 
created: 
title: "CapabilityRegistry::resolve_id"
tags: []
signature: "pub fn resolve_id (& self , name : & str) -> Option < String >"
---

Resolves the canonical ID corresponding to a query.

Tries:
1. Exact match by `id`.
2. Match if a capability has `label == name`.
3. Match if the ID ends with `.{name}` (e.g. `vendor.ext.tool.analyze` called as `analyze`).

# Examples

```rust
use crawrad_actions::capability::CapabilityRegistry;

let reg = CapabilityRegistry::new();
assert_eq!(reg.resolve_id("unknown"), None);
```
