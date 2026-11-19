---
id: capability-capabilityregistry-input-schema
kind: doc
target_file: src/capability.rs
line_hint: 755
author: 
created: 
title: "CapabilityRegistry::input_schema"
tags: []
signature: "pub fn input_schema (& self , id : & str) -> Option < serde_json :: Value >"
---

Returns the signature JSON Schema for a capability.

# Examples

```rust
use crawrad_actions::capability::CapabilityRegistry;

let reg = CapabilityRegistry::new();
assert!(reg.input_schema("unknown.cmd").is_none());
```
