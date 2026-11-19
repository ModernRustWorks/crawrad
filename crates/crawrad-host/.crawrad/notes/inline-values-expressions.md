---
id: inline-values-expressions
kind: doc
target_file: src/inline_values.rs
line_hint: 80
author: 
created: 
title: "expressions"
tags: []
signature: "pub fn expressions (lines : & [(usize , Vec < String >)]) -> Vec < String >"
---

Collects unique path expressions to ask of the debug session.

Every path once, in the order the lines name them — what is asked of the
session.

# Examples

```rust
use crawrad_host::inline_values::expressions;

let exprs = expressions(&[(0, vec!["x".into(), "y".into()])]);
assert_eq!(exprs, vec!["x", "y"]);
```
