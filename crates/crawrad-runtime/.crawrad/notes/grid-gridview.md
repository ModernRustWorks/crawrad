---
id: grid-gridview
kind: doc
target_file: src/grid.rs
line_hint: 403
author: 
created: 
title: "GridView"
tags: []
signature: "pub struct GridView"
---

Headless view snapshot ready for immediate rendering by any UI runtime.

# Examples

```rust
use std::time::Instant;
use crawrad_runtime::grid::{ColumnAlign, GridColumn, GridState, GridView};

let state = GridState::new(vec![GridColumn::new("id", "ID", ColumnAlign::Right)]);
let view: GridView = state.view(&datagrid::Screen::default(), Instant::now());
assert!(!view.animating);
```
