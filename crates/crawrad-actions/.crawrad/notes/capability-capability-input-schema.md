---
id: capability-capability-input-schema
kind: doc
target_file: src/capability.rs
line_hint: 382
author: 
created: 
title: "Capability::input_schema"
tags: []
signature: "pub fn input_schema (& self) -> serde_json :: Value"
---

Produces the complete JSON Schema (`inputSchema`) for the MCP tool definition.

# Examples

```rust
use crawrad_actions::capability::{Capability, CapabilitySource};
use crawrad_actions::Effect;

let cap = Capability::custom("test.cmd", "Test", "Desc", "Group", CapabilitySource::Native, |_| Ok(Effect::none()));
let schema = cap.input_schema();
assert_eq!(schema["type"], "object");
```
