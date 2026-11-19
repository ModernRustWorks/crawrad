---
id: breakpoints-breakpoints-log-message
kind: doc
target_file: src/breakpoints.rs
line_hint: 137
author: 
created: 
title: "Breakpoints::log_message"
tags: []
signature: "pub fn log_message (& self , file : & Path , line : u32) -> Option < & str >"
---

The message of the logpoint at `line` of `file`, if it is one.

# Examples

```rust
use std::path::Path;
use crawrad_host::Breakpoints;

let bp = Breakpoints::default();
assert_eq!(bp.log_message(Path::new("src/lib.rs"), 1), None);
```
