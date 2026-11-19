---
id: string-grid-gridlayout-is-fixed
kind: doc
target_file: src/string_grid.rs
line_hint: 74
author: 
created: 
title: "GridLayout::is_fixed"
tags: []
signature: "pub fn is_fixed (& self , col : usize , row : usize) -> bool"
---

Returns whether the cell at `(col, row)` is in the fixed header area.

# Examples

```rust
use crawrad_runtime::string_grid::GridLayout;

let layout = GridLayout::from_props(4, 8, 1, 1, 80, 20);
assert!(layout.is_fixed(0, 0));
assert!(!layout.is_fixed(1, 1));
```
