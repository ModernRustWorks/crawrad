---
id: param-json-schema
kind: doc
target_file: src/lib.rs
line_hint: 444
author: 
created: 
title: "Param::json_schema"
tags: []
signature: "pub fn json_schema (& self) -> serde_json :: Value"
---

Produces the JSON Schema fragment corresponding to this parameter.

If `self.list` is true, the schema is wrapped in `type: "array"`.

# Examples

```rust
use crawrad_actions::{Param, ParamKind};

let p = Param {
    name: "name",
    kind: ParamKind::String,
    description: "desc",
    list: false,
    required: true,
    domain: None,
};
let schema = p.json_schema();
assert_eq!(schema["type"], "string");
```
