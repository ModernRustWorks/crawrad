---
id: grid-line-for-item
kind: doc
target_file: src/grid.rs
line_hint: 503
author: 
created: 
title: "line_for_item"
tags: []
signature: "pub fn line_for_item (view : & GridView , requested_first : usize , item : usize) -> ItemLines < '_ >"
---

Selects group headers and data row for portal list item `item`.

Pure projection selecting the group headers and data row for portal list item `item`.

If row `item` is present in `view.lines`, it is returned along with any
group headers immediately preceding it.
If absent (e.g. while a new window is loading), it attempts to return the
row sitting at the same offset in the old screen (`screen.first + (item - requested_first)`),
marking `is_stale = true`.

# Examples

```rust
use std::time::Instant;
use crawrad_runtime::grid::{ColumnAlign, line_for_item, GridColumn, GridState};

let state = GridState::new(vec![GridColumn::new("id", "ID", ColumnAlign::Right)]);
let view = state.view(&datagrid::Screen::default(), Instant::now());
let item_lines = line_for_item(&view, 0, 0);
assert!(item_lines.is_stale);
```
