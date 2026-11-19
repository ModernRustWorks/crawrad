---
id: multi-select-drawmultiselectcell
kind: doc
target_file: src/multi_select.rs
line_hint: 79
author: 
created: 
title: "DrawMultiSelectCell"
tags: []
signature: "pub struct DrawMultiSelectCell"
---

The shader of a cell/box with fill, border and corner radius.

# Examples

```rust,no_run
// Makepad shader structures require the Makepad application runtime.
use crawrad_runtime_makepad::multi_select::DrawMultiSelectCell;

fn check_cell(cell: &DrawMultiSelectCell) {
    let _r = cell.radius;
}
```
