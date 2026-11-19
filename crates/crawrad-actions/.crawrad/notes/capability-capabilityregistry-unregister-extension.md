---
id: capability-capabilityregistry-unregister-extension
kind: doc
target_file: src/capability.rs
line_hint: 659
author: 
created: 
title: "CapabilityRegistry::unregister_extension"
tags: []
signature: "pub fn unregister_extension (& mut self , extension_id : & str) -> Vec < String >"
---

Unregisters all capabilities belonging to a specific extension.

# Examples

```rust
use crawrad_actions::capability::CapabilityRegistry;

let mut reg = CapabilityRegistry::new();
assert!(reg.unregister_extension("ext.id").is_empty());
```
