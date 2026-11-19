---
id: data-grid-datagrid-columns
kind: doc
target_file: src/data_grid.rs
line_hint: 166
author: 
created: 
title: "DataGrid::columns"
tags: []
signature: "pub fn columns (& self) -> & [GridColumn]"
---

Returns the column metadata declarations.

# Examples

```rust
use std::sync::Arc;
use datagrid::{Store, VecSource};
use crawrad_runtime::grid::{ColumnAlign, GridColumn};
use crawrad_runtime_egui::data_grid::DataGrid;

let source = Arc::new(VecSource::from_store(Store::new()));
let columns = vec![GridColumn::new("id", "ID", ColumnAlign::Left)];
let grid = DataGrid::new(source, columns);
assert_eq!(grid.columns().len(), 1);
assert_eq!(grid.columns()[0].name, "id");
```
