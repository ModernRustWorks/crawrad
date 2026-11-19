---
id: grid-gridstate-view
kind: doc
target_file: src/grid.rs
line_hint: 382
author: 
created: 
title: "GridState::view"
tags: []
signature: "pub fn view (& self , screen : & datagrid :: Screen , now : std :: time :: Instant) -> GridView"
---

Derives the view projection from the screen snapshot and point in time.

Derives the immutable view projection from the latest screen snapshot and point in time.

# Examples

```rust
use std::time::Instant;
use crawrad_runtime::grid::{ColumnAlign, GridColumn, GridState};

let state = GridState::new(vec![GridColumn::new("id", "ID", ColumnAlign::Right)]);
let screen = datagrid::Screen::default();
let view = state.view(&screen, Instant::now());
assert_eq!(view.header.len(), 1);
```
