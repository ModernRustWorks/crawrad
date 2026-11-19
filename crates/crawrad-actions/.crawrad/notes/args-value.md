---
id: args-value
kind: doc
target_file: src/lib.rs
line_hint: 237
author: 
created: 
title: "Args::value"
tags: []
signature: "pub fn value (& self , name : & str) -> Option < & Value >"
---

The raw value, for whoever knows what to expect.

# Examples

```rust
use crawrad_actions::Args;
use crawrad_ir::Value;

let args = Args::text("title", "Hello");
assert_eq!(args.value("title"), Some(&Value::Str("Hello".into())));
```
