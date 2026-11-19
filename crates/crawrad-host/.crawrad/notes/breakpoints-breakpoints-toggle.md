---
id: breakpoints-breakpoints-toggle
kind: doc
target_file: src/breakpoints.rs
line_hint: 97
author: 
created: 
title: "Breakpoints::toggle"
tags: []
signature: "pub fn toggle (& mut self , file : & Path , line : u32) -> bool"
---

Toggles a breakpoint at the given line of `file`.

Adds the breakpoint when the line has none, removes it when it has one.
Returns whether the line carries one afterwards.

# Examples

```rust
use std::path::Path;
use crawrad_host::Breakpoints;

let mut bp = Breakpoints::default();
assert!(bp.toggle(Path::new("src/lib.rs"), 42));
assert!(!bp.toggle(Path::new("src/lib.rs"), 42));
```
