---
id: capability-capabilityregistry-len
kind: doc
target_file: src/capability.rs
line_hint: 689
author: 
created: 
title: "CapabilityRegistry::len"
tags: []
signature: "pub fn len (& self) -> usize"
---

Returns the total count of registered capabilities.

# Examples

```rust
use crawrad_actions::capability::CapabilityRegistry;

let reg = CapabilityRegistry::new();
assert_eq!(reg.len(), 0);
```
