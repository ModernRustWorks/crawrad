---
id: capability-capabilityregistry-with-builtin-actions
kind: doc
target_file: src/capability.rs
line_hint: 497
author: 
created: 
title: "CapabilityRegistry::with_builtin_actions"
tags: []
signature: "pub fn with_builtin_actions () -> Result < Self , CapabilityError >"
---

Creates a pre-populated registry with all native catalogue actions.

Creates a pre-populated registry with all native actions from [`crate::catalogue()`].

# Errors

Returns [`CapabilityError`] if a native action has an invalid ID or collides.

# Examples

```rust
use crawrad_actions::capability::CapabilityRegistry;

let reg = CapabilityRegistry::with_builtin_actions().expect("builtins valid");
assert!(!reg.is_empty());
```
