---
id: layout-at
kind: doc
target_file: src/layout.rs
line_hint: 25
author: 
created: 
title: "at"
tags: []
signature: "pub fn at (path : & str , base : & str) -> String"
---

Fills the `{}` of a path with the variable's `evaluateName`.

# Examples

```rust
use crawrad_dap_rust::layout::at;

assert_eq!(at("{}.len", "v"), "v.len");
```
