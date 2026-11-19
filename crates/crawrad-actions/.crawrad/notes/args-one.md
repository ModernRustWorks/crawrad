---
id: args-one
kind: doc
target_file: src/lib.rs
line_hint: 204
author: 
created: 
title: "Args::one"
tags: []
signature: "pub fn one (name : impl Into < String > , value : Value) -> Self"
---

A named argument. It is the general doorway — all others arrive here.

# Examples

```rust
use crawrad_actions::Args;
use crawrad_ir::Value;

let args = Args::one("name", Value::Str("btn".into()));
assert_eq!(args.as_text("name"), Some("btn"));
```
