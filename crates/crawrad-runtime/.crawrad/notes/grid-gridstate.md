---
id: grid-gridstate
kind: doc
target_file: src/grid.rs
line_hint: 73
author: 
created: 
title: "GridState"
tags: []
signature: "pub struct GridState"
---

Headless view-state for a grid component across user interactions.

# Examples

```rust
use crawrad_runtime::grid::{ColumnAlign, GridColumn, GridState};

let state = GridState::new(vec![GridColumn::new("col", "Col", ColumnAlign::Left)]);
assert_eq!(state.columns.len(), 1);
```
