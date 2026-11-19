---
id: session-breakpoint
kind: doc
target_file: src/session.rs
line_hint: 86
author: 
created: 
title: "Breakpoint"
tags: []
signature: "pub struct Breakpoint"
---

A breakpoint as the adapter answered it.

# Examples

```rust
use std::path::PathBuf;
use crawrad_dap::Breakpoint;

let bp = Breakpoint {
    path: PathBuf::from("src/main.rs"),
    requested_line: 10,
    line: Some(10),
    verified: true,
};
assert!(bp.verified);
```
