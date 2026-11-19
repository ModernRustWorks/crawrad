---
id: breakpoints-breakpoints-files
kind: doc
target_file: src/breakpoints.rs
line_hint: 166
author: 
created: 
title: "Breakpoints::files"
tags: []
signature: "pub fn files (& self) -> impl Iterator < Item = (& Path , Vec < u32 >) >"
---

Every file with its breakpoint lines — what a debug session sends.

# Examples

```rust
use crawrad_host::Breakpoints;

let bp = Breakpoints::default();
assert_eq!(bp.files().count(), 0);
```
