---
id: value-value-kind
kind: doc
target_file: src/value.rs
line_hint: 80
author: 
created: 
title: "Value::kind"
tags: []
signature: "pub fn kind (& self) -> & 'static str"
---

Type name, for error messages and for the inspector to pick an editor.

# Examples

```rust
use crawrad_ir::Value;

assert_eq!(Value::Int(42).kind(), "int");
assert_eq!(Value::Str("hello".into()).kind(), "str");
```
