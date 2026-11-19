---
id: capability-capabilityregistry-get
kind: doc
target_file: src/capability.rs
line_hint: 679
author: 
created: 
title: "CapabilityRegistry::get"
tags: []
signature: "pub fn get (& self , id : & str) -> Option < & Capability >"
---

Looks up a capability by its canonical ID.

# Examples

```rust
use crawrad_actions::capability::CapabilityRegistry;

let reg = CapabilityRegistry::new();
assert!(reg.get("unknown.id").is_none());
```
