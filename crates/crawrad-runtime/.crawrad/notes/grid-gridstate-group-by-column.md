---
id: grid-gridstate-group-by-column
kind: doc
target_file: src/grid.rs
line_hint: 131
author: 
created: 
title: "GridState::group_by_column"
tags: []
signature: "pub fn group_by_column (& mut self , column : usize) -> datagrid :: Query"
---

Adds a grouping level for the column at `column` index if not already present.

# Examples

```rust
use crawrad_runtime::grid::{ColumnAlign, GridColumn, GridState};

let mut state = GridState::new(vec![GridColumn::new("category", "Category", ColumnAlign::Left)]);
let q = state.group_by_column(0);
assert_eq!(q.group_by.len(), 1);
```
