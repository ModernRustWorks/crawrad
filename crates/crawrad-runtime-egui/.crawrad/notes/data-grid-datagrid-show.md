---
id: data-grid-datagrid-show
kind: doc
target_file: src/data_grid.rs
line_hint: 171
author: 
created: 
title: "DataGrid::show"
tags: []
signature: "pub fn show (& mut self , ui : & mut Ui)"
---

Paints the complete visual data grid onto the target egui context.

# Examples

```rust
use std::sync::Arc;
use datagrid::{Store, VecSource};
use crawrad_runtime_egui::data_grid::DataGrid;

let source = Arc::new(VecSource::from_store(Store::new()));
let mut grid = DataGrid::new(source, Vec::new());
let ctx = egui::Context::default();
let mut out = ctx.run_ui(egui::RawInput::default(), |ui| {
    grid.show(ui);
});
out.textures_delta.clear();
assert_eq!(grid.rec_no(), 0);
```
