---
id: capability-capabilityparam-json-schema
kind: doc
target_file: src/capability.rs
line_hint: 94
author: 
created: 
title: "CapabilityParam::json_schema"
tags: []
signature: "pub fn json_schema (& self) -> serde_json :: Value"
---

Generates the corresponding JSON Schema fragment for this parameter.

# Examples

```rust
use crawrad_actions::capability::CapabilityParam;
use crawrad_actions::ParamKind;

let param = CapabilityParam::new("text", ParamKind::String, "Input text");
let schema = param.json_schema();
assert_eq!(schema["type"], "string");
```
