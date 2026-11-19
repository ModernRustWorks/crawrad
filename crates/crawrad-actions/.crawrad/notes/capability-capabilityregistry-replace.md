---
id: capability-capabilityregistry-replace
kind: doc
target_file: src/capability.rs
line_hint: 538
author: 
created: 
title: "CapabilityRegistry::replace"
tags: []
signature: "pub fn replace (& mut self , capability : Capability ,) -> Result < Option < Capability > , CapabilityError >"
---

Replaces or updates an existing capability, or registers it if absent.

Replaces or updates an existing capability (or registers it if it does not exist).

# Errors

Returns [`CapabilityError::InvalidId`] if the capability ID format is invalid.

# Examples

```rust
use crawrad_actions::capability::{Capability, CapabilityRegistry, CapabilitySource};
use crawrad_actions::Effect;

let mut reg = CapabilityRegistry::new();
let cap = Capability::custom("custom.test", "Test", "Desc", "Group", CapabilitySource::Native, |_| Ok(Effect::none()));
assert!(reg.replace(cap).is_ok());
```
