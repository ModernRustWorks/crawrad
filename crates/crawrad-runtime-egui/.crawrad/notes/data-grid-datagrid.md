---
id: data-grid-datagrid
kind: doc
target_file: src/data_grid.rs
line_hint: 113
author: 
created: 
title: "DataGrid"
tags: []
signature: "pub struct DataGrid"
---

Immediate-mode virtualized data grid widget.

# Examples

```rust
use std::sync::Arc;
use datagrid::{Store, VecSource};
use crawrad_runtime::grid::{ColumnAlign, GridColumn};
use crawrad_runtime_egui::data_grid::DataGrid;

let source = Arc::new(VecSource::from_store(Store::new()));
let columns = vec![GridColumn::new("id", "ID", ColumnAlign::Left)];
let grid = DataGrid::new(source, columns);
assert_eq!(grid.rec_no(), 0);
```
