---
id: data-grid-column-widths
kind: doc
target_file: src/data_grid.rs
line_hint: 430
author: 
created: 
title: "column_widths"
tags: []
signature: "pub fn column_widths (available : f64 , columns : & [GridColumn]) -> Vec < f64 >"
---

Each column's width (#936).

A column's own `width` as given, the others share what is left, never
narrower than [`MIN_COLUMN_WIDTH`].

# Examples

```rust
use crawrad_runtime::grid::{ColumnAlign, GridColumn};
use crawrad_runtime_makepad::data_grid::column_widths;

let cols = vec![GridColumn::new("a", "A", ColumnAlign::Left), GridColumn::new("b", "B", ColumnAlign::Left)];
assert_eq!(column_widths(600.0, &cols), vec![300.0, 300.0]);
```
