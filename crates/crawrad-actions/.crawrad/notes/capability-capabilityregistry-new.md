---
id: capability-capabilityregistry-new
kind: doc
target_file: src/capability.rs
line_hint: 487
author: 
created: 
title: "CapabilityRegistry::new"
tags: []
signature: "pub fn new () -> Self"
---

Creates an empty registry.

# Examples

```rust
use crawrad_actions::capability::CapabilityRegistry;

let reg = CapabilityRegistry::new();
assert_eq!(reg.len(), 0);
```
