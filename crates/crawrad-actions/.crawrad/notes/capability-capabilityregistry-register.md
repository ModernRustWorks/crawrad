---
id: capability-capabilityregistry-register
kind: doc
target_file: src/capability.rs
line_hint: 522
author: 
created: 
title: "CapabilityRegistry::register"
tags: []
signature: "pub fn register (& mut self , capability : Capability) -> Result < () , CapabilityError >"
---

Registers a unified capability with strict collision and ID format validation.

# Errors

Returns [`CapabilityError::InvalidId`] if the ID is malformed, or
[`CapabilityError::IdCollision`] if the ID is already registered.

# Examples

```rust
use crawrad_actions::capability::{Capability, CapabilityRegistry, CapabilitySource};
use crawrad_actions::Effect;

let mut reg = CapabilityRegistry::new();
let cap = Capability::custom("custom.test", "Test", "Desc", "Group", CapabilitySource::Native, |_| Ok(Effect::none()));
assert!(reg.register(cap).is_ok());
```
