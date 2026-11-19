---
id: grid-gridstate-ungroup
kind: doc
target_file: src/grid.rs
line_hint: 142
author: 
created: 
title: "GridState::ungroup"
tags: []
signature: "pub fn ungroup (& mut self , level : usize) -> datagrid :: Query"
---

Removes a grouping level at the specified hierarchy index.

# Examples

```rust
use crawrad_runtime::grid::{ColumnAlign, GridColumn, GridState};

let mut state = GridState::new(vec![GridColumn::new("dept", "Department", ColumnAlign::Left)]);
state.group_by_column(0);
let q = state.ungroup(0);
assert!(q.group_by.is_empty());
```
