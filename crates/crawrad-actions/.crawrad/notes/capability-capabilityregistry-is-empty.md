---
id: capability-capabilityregistry-is-empty
kind: doc
target_file: src/capability.rs
line_hint: 694
author: 
created: 
title: "CapabilityRegistry::is_empty"
tags: []
signature: "pub fn is_empty (& self) -> bool"
---

Checks whether the catalogue is empty.

# Examples

```rust
use crawrad_actions::capability::CapabilityRegistry;

let reg = CapabilityRegistry::new();
assert!(reg.is_empty());
```
