---
id: data-grid-datagrid-new
kind: doc
target_file: src/data_grid.rs
line_hint: 126
author: 
created: 
title: "DataGrid::new"
tags: []
signature: "pub fn new (source : Arc < dyn RowSource > , columns : Vec < GridColumn >) -> Self"
---

Constructs a new data grid bound to a row source and column definitions.

Constructs a new data grid bound to an asynchronous row source and column definitions.

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
