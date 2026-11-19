---
id: args-and
kind: doc
target_file: src/lib.rs
line_hint: 209
author: 
created: 
title: "Args::and"
tags: []
signature: "pub fn and (mut self , name : impl Into < String > , value : Value) -> Self"
---

One more, chaining: `Args::text("class", "Button").and("x", …)`.

# Examples

```rust
use crawrad_actions::Args;
use crawrad_ir::Value;

let args = Args::text("class", "Button").and("count", Value::Int(1));
assert_eq!(args.as_text("class"), Some("Button"));
```
