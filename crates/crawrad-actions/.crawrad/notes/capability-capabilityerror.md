---
id: capability-capabilityerror
kind: doc
target_file: src/capability.rs
line_hint: 434
author: 
created: 
title: "CapabilityError"
tags: []
signature: "pub enum CapabilityError"
---

Possible errors in [`CapabilityRegistry`] operations.

# Examples

```rust
use crawrad_actions::capability::CapabilityError;

let err = CapabilityError::NotFound("unknown.cmd".to_string());
assert!(err.to_string().contains("unknown.cmd"));
```
