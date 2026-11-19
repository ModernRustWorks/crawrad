---
id: grid-gridstate-click-header
kind: doc
target_file: src/grid.rs
line_hint: 111
author: 
created: 
title: "GridState::click_header"
tags: []
signature: "pub fn click_header (& mut self , column : usize) -> datagrid :: Query"
---

Cycles column sort between ascending, descending, and returns the query.

Header click: not sorted → ascending; ascending → descending; descending → ascending.
It replaces the sort (single-column sort). Returns the new query.

# Examples

```rust
use datagrid::Order;
use crawrad_runtime::grid::{ColumnAlign, GridColumn, GridState};

let mut state = GridState::new(vec![GridColumn::new("id", "ID", ColumnAlign::Right)]);
let q = state.click_header(0);
assert_eq!(q.sort[0].order, Order::Asc);
let q2 = state.click_header(0);
assert_eq!(q2.sort[0].order, Order::Desc);
```
