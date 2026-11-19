---
id: grid-share-widths
kind: doc
target_file: src/grid.rs
line_hint: 99
author: 
created: 
title: "share_widths"
tags: []
signature: "pub fn share_widths (available : f64 , widths : & [Option < f64 >]) -> Vec < f64 >"
---

Fixed widths stay as given; the rest share what is left, never below zero.

# Examples

```rust
use crawrad_runtime::grid::share_widths;

assert_eq!(share_widths(600.0, &[Some(100.0), None, None]), vec![100.0, 250.0, 250.0]);
```
