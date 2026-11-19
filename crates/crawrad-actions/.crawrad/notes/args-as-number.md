---
id: args-as-number
kind: doc
target_file: src/lib.rs
line_hint: 254
author: 
created: 
title: "Args::as_number"
tags: []
signature: "pub fn as_number (& self , name : & str) -> Option < f64 >"
---

A number, accepting `Int` **or** `Float`.

This is the line that `Vec<String>` would not let exist: `x` and `y` of
`component.add` are coordinates, and on the other side of the protocol JSON
sends `10` both for integer and for real. Whoever calls converts
to `f32` at the point of use — the document measures in `f32`, and the conversion is
on the caller so as not to hide the loss.

# Examples

```rust
use crawrad_actions::Args;
use crawrad_ir::Value;

let args = Args::one("x", Value::Float(10.5));
assert_eq!(args.as_number("x"), Some(10.5));
```
