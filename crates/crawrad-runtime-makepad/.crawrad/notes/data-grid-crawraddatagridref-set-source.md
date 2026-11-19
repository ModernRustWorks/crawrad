---
id: data-grid-crawraddatagridref-set-source
kind: doc
target_file: src/data_grid.rs
line_hint: 775
author: 
created: 
title: "CrawradDataGridRef::set_source"
tags: []
signature: "pub fn set_source (& self , cx : & mut Cx , source : Arc < dyn datagrid :: RowSource > , columns : Vec < GridColumn > ,)"
---

Binds a row source and column metadata to the visual grid.

# Examples

```rust,no_run
// Makepad Cx and RowSource require the Makepad application runtime.
use std::sync::Arc;
use makepad_widgets::Cx;
use crawrad_runtime_makepad::data_grid::CrawradDataGridRef;

fn example(grid: &CrawradDataGridRef, cx: &mut Cx) {
    // grid.set_source(cx, source, columns);
}
```
