---
id: capability-capabilityregistry
kind: doc
target_file: src/capability.rs
line_hint: 472
author: 
created: 
title: "CapabilityRegistry"
tags: []
signature: "pub struct CapabilityRegistry"
---

Unified registry of IDE capabilities.

# Examples

```rust
use crawrad_actions::capability::CapabilityRegistry;

let reg = CapabilityRegistry::new();
assert!(reg.is_empty());
```
