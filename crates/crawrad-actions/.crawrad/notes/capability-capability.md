---
id: capability-capability
kind: doc
target_file: src/capability.rs
line_hint: 213
author: 
created: 
title: "Capability"
tags: []
signature: "pub struct Capability"
---

A unified capability registered in the IDE.

# Examples

```rust
use crawrad_actions::capability::{Capability, CapabilitySource};
use crawrad_actions::Effect;

let cap = Capability::custom("test.cmd", "Test", "Test command", "Test", CapabilitySource::Native, |_| Ok(Effect::none()));
assert_eq!(cap.id, "test.cmd");
```
