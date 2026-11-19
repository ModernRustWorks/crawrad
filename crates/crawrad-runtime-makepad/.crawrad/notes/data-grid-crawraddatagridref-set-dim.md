---
id: data-grid-crawraddatagridref-set-dim
kind: doc
target_file: src/data_grid.rs
line_hint: 809
author: 
created: 
title: "CrawradDataGridRef::set_dim"
tags: []
signature: "pub fn set_dim (& self , dim : f64)"
---

Sets the dimming opacity of stale rows during loading operations.

# Examples

```rust,no_run
// WidgetRef requires the Makepad application runtime.
use crawrad_runtime_makepad::data_grid::CrawradDataGridRef;

fn example(grid: &CrawradDataGridRef) {
    grid.set_dim(0.5);
}
```
