---
id: grid-gridcolumn-new
kind: doc
target_file: src/grid.rs
line_hint: 126
author: 
created: 
title: "GridColumn::new"
tags: []
signature: "pub fn new (name : impl Into < String > , title : impl Into < String > , align : ColumnAlign) -> Self"
---

Creates a new grid column descriptor.

# Examples

```rust
use crawrad_runtime::grid::{ColumnAlign, GridColumn};

let col = GridColumn::new("name", "Customer Name", ColumnAlign::Left);
assert_eq!(col.title, "Customer Name");
assert_eq!(col.align, ColumnAlign::Left);
```
