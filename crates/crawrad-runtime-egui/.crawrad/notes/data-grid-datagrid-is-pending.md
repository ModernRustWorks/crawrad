---
id: data-grid-datagrid-is-pending
kind: doc
target_file: src/data_grid.rs
line_hint: 156
author: 
created: 
title: "DataGrid::is_pending"
tags: []
signature: "pub fn is_pending (& self) -> bool"
---

Returns whether the background worker is currently fetching a requested window.

# Examples

```rust
use std::sync::Arc;
use datagrid::{Store, VecSource};
use crawrad_runtime_egui::data_grid::DataGrid;

let source = Arc::new(VecSource::from_store(Store::new()));
let grid = DataGrid::new(source, Vec::new());
assert!(grid.is_pending());
```
