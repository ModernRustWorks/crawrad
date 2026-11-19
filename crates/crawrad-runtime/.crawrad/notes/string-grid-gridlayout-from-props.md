---
id: string-grid-gridlayout-from-props
kind: doc
target_file: src/string_grid.rs
line_hint: 55
author: 
created: 
title: "GridLayout::from_props"
tags: []
signature: "pub fn from_props (col_count : i64 , row_count : i64 , fixed_cols : i64 , fixed_rows : i64 , col_width : i64 , row_height : i64 ,) -> Self"
---

Constructs a grid layout from raw signed properties.

Negatives become 0, and column width / row height are clamped to at least 1.

# Examples

```rust
use crawrad_runtime::string_grid::GridLayout;

let layout = GridLayout::from_props(4, 8, 1, 1, 80, 20);
assert_eq!(layout.col_count, 4);
assert_eq!(layout.row_height, 20.0);
```
