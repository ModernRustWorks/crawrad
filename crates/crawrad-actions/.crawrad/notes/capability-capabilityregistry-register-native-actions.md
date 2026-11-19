---
id: capability-capabilityregistry-register-native-actions
kind: doc
target_file: src/capability.rs
line_hint: 552
author: 
created: 
title: "CapabilityRegistry::register_native_actions"
tags: []
signature: "pub fn register_native_actions < 'a > (& mut self , actions : impl IntoIterator < Item = & 'a Action > ,) -> Result < usize , CapabilityError >"
---

Registers a collection of native actions, returning the registered count.

Registers a collection of native actions. Returns the count successfully registered.

# Errors

Returns [`CapabilityError`] if any action fails registration.

# Examples

```rust
use crawrad_actions::capability::CapabilityRegistry;
use crawrad_actions::catalogue;

let mut reg = CapabilityRegistry::new();
let acts = catalogue();
assert!(reg.register_native_actions(&acts).is_ok());
```
