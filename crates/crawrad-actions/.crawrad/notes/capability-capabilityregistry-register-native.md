---
id: capability-capabilityregistry-register-native
kind: doc
target_file: src/capability.rs
line_hint: 547
author: 
created: 
title: "CapabilityRegistry::register_native"
tags: []
signature: "pub fn register_native (& mut self , action : & Action) -> Result < () , CapabilityError >"
---

Registers a native [`Action`] of the IDE.

# Errors

Returns [`CapabilityError`] if ID validation fails or if the action is already registered.

# Examples

```rust
use crawrad_actions::capability::CapabilityRegistry;
use crawrad_actions::catalogue;

let mut reg = CapabilityRegistry::new();
let acts = catalogue();
if let Some(act) = acts.first() {
    assert!(reg.register_native(act).is_ok());
}
```
