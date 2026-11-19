---
id: string-grid-gridlayout-selectable-at
kind: doc
target_file: src/string_grid.rs
line_hint: 99
author: 
created: 
title: "GridLayout::selectable_at"
tags: []
signature: "pub fn selectable_at (& self , x : f64 , y : f64) -> Option < (usize , usize) >"
---

Finds the selectable (non-fixed) cell under point `(x, y)`.

Returns `None` if the point is outside or on a fixed cell.

# Examples

```rust
use crawrad_runtime::string_grid::GridLayout;

let layout = GridLayout::from_props(4, 8, 1, 1, 80, 20);
assert_eq!(layout.selectable_at(0.0, 0.0), None);
assert_eq!(layout.selectable_at(85.0, 25.0), Some((1, 1)));
```
