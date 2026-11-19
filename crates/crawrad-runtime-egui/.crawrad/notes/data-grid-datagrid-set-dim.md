---
id: data-grid-datagrid-set-dim
kind: doc
target_file: src/data_grid.rs
line_hint: 146
author: 
created: 
title: "DataGrid::set_dim"
tags: []
signature: "pub fn set_dim (& mut self , dim : f64)"
---

Sets the dimming opacity of stale rows during lengthy source queries.

# Examples

```rust
use std::sync::Arc;
use datagrid::{Store, VecSource};
use crawrad_runtime_egui::data_grid::DataGrid;

let source = Arc::new(VecSource::from_store(Store::new()));
let mut grid = DataGrid::new(source, Vec::new());
grid.set_dim(0.5);
assert_eq!(grid.rec_no(), 0);
```
