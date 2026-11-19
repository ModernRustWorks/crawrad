---
id: capability-capabilityregistry-list-by-group
kind: doc
target_file: src/capability.rs
line_hint: 704
author: 
created: 
title: "CapabilityRegistry::list_by_group"
tags: []
signature: "pub fn list_by_group (& self , group : & str) -> Vec < & Capability >"
---

Lists capabilities belonging to a specific group.

# Examples

```rust
use crawrad_actions::capability::CapabilityRegistry;

let reg = CapabilityRegistry::new();
assert!(reg.list_by_group("File").is_empty());
```
