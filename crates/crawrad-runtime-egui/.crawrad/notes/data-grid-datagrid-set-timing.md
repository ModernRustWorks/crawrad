---
id: data-grid-datagrid-set-timing
kind: doc
target_file: src/data_grid.rs
line_hint: 141
author: 
created: 
title: "DataGrid::set_timing"
tags: []
signature: "pub fn set_timing (& mut self , timing : BusyTiming)"
---

Sets the anti-flicker timing parameters for loading transitions.

# Examples

```rust
use std::sync::Arc;
use std::time::Duration;
use datagrid::{Store, VecSource};
use crawrad_component::BusyTiming;
use crawrad_runtime_egui::data_grid::DataGrid;

let source = Arc::new(VecSource::from_store(Store::new()));
let mut grid = DataGrid::new(source, Vec::new());
let timing = BusyTiming {
    show_after: Duration::from_millis(100),
    min_visible: Duration::from_millis(300),
    fade_out: Duration::from_millis(100),
};
grid.set_timing(timing);
assert_eq!(grid.rec_no(), 0);
```
