---
id: breakpoints-breakpoints-lines
kind: doc
target_file: src/breakpoints.rs
line_hint: 44
author: 
created: 
title: "Breakpoints::lines"
tags: []
signature: "pub fn lines (& self , file : & Path) -> Vec < u32 >"
---

The lines of `file` that carry a breakpoint, in order.

# Examples

```rust
use std::path::Path;
use crawrad_host::Breakpoints;

let bp = Breakpoints::default();
assert!(bp.lines(Path::new("src/lib.rs")).is_empty());
```
