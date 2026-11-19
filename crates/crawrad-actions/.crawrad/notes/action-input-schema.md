---
id: action-input-schema
kind: doc
target_file: src/lib.rs
line_hint: 507
author: 
created: 
title: "Action::input_schema"
tags: []
signature: "pub fn input_schema (& self) -> serde_json :: Value"
---

Produces the complete JSON Schema (`inputSchema`) for the MCP tool definition.

# Examples

```rust
use crawrad_actions::by_id;

let action = by_id("file.save").unwrap();
let schema = action.input_schema();
assert_eq!(schema["type"], "object");
```
