---
id: args-as-bool
kind: doc
target_file: src/lib.rs
line_hint: 423
author: 
created: 
title: "Args::as_bool"
tags: []
signature: "pub fn as_bool (& self , name : & str) -> Option < bool >"
---

A flag; `None` is absence, and the action picks the default.

`component.rename` reads a missing `everywhere` as `true`.

# Examples

```rust
use crawrad_actions::Args;
use crawrad_ir::Value;

let args = Args::one("everywhere", Value::Bool(false));
assert_eq!(args.as_bool("everywhere"), Some(false));
assert_eq!(args.as_bool("other"), None);
```
