---
id: data-grid-datagrid-total
kind: doc
target_file: src/data_grid.rs
line_hint: 161
author: 
created: 
title: "DataGrid::total"
tags: []
signature: "pub fn total (& self) -> Option < usize >"
---

Returns the total number of rows matching the active query, if resolved.

# Examples

```rust
use std::sync::Arc;
use datagrid::{Store, VecSource};
use crawrad_runtime_egui::data_grid::DataGrid;

let source = Arc::new(VecSource::from_store(Store::new()));
let grid = DataGrid::new(source, Vec::new());
assert_eq!(grid.total(), None);
```
