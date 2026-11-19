---
id: string-grid-gridlayout-cell-rect
kind: doc
target_file: src/string_grid.rs
line_hint: 79
author: 
created: 
title: "GridLayout::cell_rect"
tags: []
signature: "pub fn cell_rect (& self , col : usize , row : usize) -> (f64 , f64 , f64 , f64)"
---

Computes the bounding rectangle `(x, y, w, h)` of cell `(col, row)`.

Computes the bounding rectangle `(x, y, w, h)` of cell `(col, row)`
relative to the top-left of the grid.

# Examples

```rust
use crawrad_runtime::string_grid::GridLayout;

let layout = GridLayout::from_props(4, 8, 1, 1, 80, 20);
let (x, y, w, h) = layout.cell_rect(1, 2);
assert_eq!((x, y, w, h), (80.0, 40.0, 80.0, 20.0));
```
