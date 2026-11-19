---
id: capability-capabilityregistry-search
kind: doc
target_file: src/capability.rs
line_hint: 720
author: 
created: 
title: "CapabilityRegistry::search"
tags: []
signature: "pub fn search (& self , query : & str) -> Vec < & Capability >"
---

Searches capabilities by text query for the Command Palette.

Searches capabilities by text query (ID, label, or description) for the Command Palette.

# Examples

```rust
use crawrad_actions::capability::CapabilityRegistry;

let reg = CapabilityRegistry::new();
assert!(reg.search("copy").is_empty());
```
