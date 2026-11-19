---
id: drawing-limit-property
kind: doc
target_file: src/drawing.rs
line_hint: 128
author: 
created: 
title: "Limit::property"
tags: []
signature: "pub fn property (& self) -> Option < & str >"
---

The key that this limit reads, when it reads any.

It exists for the gate that demands that every verb point to a property
that exists: without it, the limit would be the only place in the vocabulary where
an invented key would slip through unnoticed.

# Examples

```rust
use crawrad_ir::drawing::Limit;

assert_eq!(Limit::Fixed(0.0).property(), None);
assert_eq!(Limit::Property("value".to_string()).property(), Some("value"));
```
