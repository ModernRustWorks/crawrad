---
id: capability-capabilityregistry-list-by-source
kind: doc
target_file: src/capability.rs
line_hint: 712
author: 
created: 
title: "CapabilityRegistry::list_by_source"
tags: []
signature: "pub fn list_by_source (& self , source : & CapabilitySource) -> Vec < & Capability >"
---

Lists capabilities belonging to a specific source.

# Examples

```rust
use crawrad_actions::capability::{CapabilityRegistry, CapabilitySource};

let reg = CapabilityRegistry::new();
assert!(reg.list_by_source(&CapabilitySource::Native).is_empty());
```
