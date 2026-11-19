---
id: capability-capabilityregistry-list
kind: doc
target_file: src/capability.rs
line_hint: 699
author: 
created: 
title: "CapabilityRegistry::list"
tags: []
signature: "pub fn list (& self) -> Vec < & Capability >"
---

Lists all registered capabilities.

# Examples

```rust
use crawrad_actions::capability::CapabilityRegistry;

let reg = CapabilityRegistry::new();
assert!(reg.list().is_empty());
```
