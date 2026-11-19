---
id: data-grid-datagrid-rec-no
kind: doc
target_file: src/data_grid.rs
line_hint: 151
author: 
created: 
title: "DataGrid::rec_no"
tags: []
signature: "pub fn rec_no (& self) -> usize"
---

Returns the active selected record index (0-based).

# Examples

```rust
use std::sync::Arc;
use datagrid::{Store, VecSource};
use crawrad_runtime_egui::data_grid::DataGrid;

let source = Arc::new(VecSource::from_store(Store::new()));
let grid = DataGrid::new(source, Vec::new());
assert_eq!(grid.rec_no(), 0);
```
