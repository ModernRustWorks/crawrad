---
id: breakpoints-breakpoints-count
kind: doc
target_file: src/breakpoints.rs
line_hint: 173
author: 
created: 
title: "Breakpoints::count"
tags: []
signature: "pub fn count (& self) -> usize"
---

How many breakpoints there are, in every file.

# Examples

```rust
use crawrad_host::Breakpoints;

let bp = Breakpoints::default();
assert_eq!(bp.count(), 0);
```
