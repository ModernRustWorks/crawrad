---
id: string-grid-gridlayout-cell-at
kind: doc
target_file: src/string_grid.rs
line_hint: 89
author: 
created: 
title: "GridLayout::cell_at"
tags: []
signature: "pub fn cell_at (& self , x : f64 , y : f64) -> Option < (usize , usize) >"
---

Finds the cell `(col, row)` under point `(x, y)`.

Finds the cell coordinate `(col, row)` under point `(x, y)` relative to the top-left.

Returns `None` if `(x, y)` is outside the cell bounds.

# Examples

```rust
use crawrad_runtime::string_grid::GridLayout;

let layout = GridLayout::from_props(4, 8, 1, 1, 80, 20);
assert_eq!(layout.cell_at(85.0, 25.0), Some((1, 1)));
assert_eq!(layout.cell_at(-1.0, 0.0), None);
```
