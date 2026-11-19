---
id: grid-gridstate-query
kind: doc
target_file: src/grid.rs
line_hint: 102
author: 
created: 
title: "GridState::query"
tags: []
signature: "pub fn query (& self) -> datagrid :: Query"
---

Builds the [`datagrid::Query`] representing current sort and group state.

Constructs the [`datagrid::Query`] representing the current sort and group state.

# Examples

```rust
use crawrad_runtime::grid::{ColumnAlign, GridColumn, GridState};

let state = GridState::new(vec![GridColumn::new("id", "ID", ColumnAlign::Right)]);
let q = state.query();
assert!(q.sort.is_empty());
```
