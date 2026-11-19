---
id: value-value-describe
kind: doc
target_file: src/value.rs
line_hint: 44
author: 
created: 
title: "Value::describe"
tags: []
signature: "pub fn describe (& self) -> String"
---

Describes the value for display in the Object Inspector.

The text of a value, for the Object Inspector's row that does not
edit it: the value itself (`#RRGGBB`, `10, 10, 80 × 28`, `3 items`),
never the name of its type unless nothing better exists.

# Examples

```rust
use crawrad_ir::Value;

assert_eq!(Value::Int(42).describe(), "42");
assert_eq!(Value::Bool(true).describe(), "true");
```
