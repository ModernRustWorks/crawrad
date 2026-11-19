---
id: value-from-json
kind: doc
target_file: src/lib.rs
line_hint: 6041
author: 
created: 
title: "value_from_json"
tags: []
signature: "pub fn value_from_json (v : & serde_json :: Value) -> crawrad_ir :: Value"
---

A JSON value as the IR's: integers stay `Int`, other numbers `Float`.

# Examples

```rust
use crawrad_actions::value_from_json;
use crawrad_ir::Value;

let v = value_from_json(&serde_json::json!(42));
assert_eq!(v, Value::Int(42));
```
