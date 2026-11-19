---
id: capability-capabilityregistry-contains
kind: doc
target_file: src/capability.rs
line_hint: 684
author: 
created: 
title: "CapabilityRegistry::contains"
tags: []
signature: "pub fn contains (& self , id : & str) -> bool"
---

Checks whether a capability is registered.

# Examples

```rust
use crawrad_actions::capability::CapabilityRegistry;

let reg = CapabilityRegistry::new();
assert!(!reg.contains("unknown.id"));
```
