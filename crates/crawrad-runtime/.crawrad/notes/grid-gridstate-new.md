---
id: grid-gridstate-new
kind: doc
target_file: src/grid.rs
line_hint: 89
author: 
created: 
title: "GridState::new"
tags: []
signature: "pub fn new (columns : Vec < GridColumn >) -> Self"
---

Constructs a grid state with default dimming (0.45) and standard busy timing.

# Examples

```rust
use crawrad_runtime::grid::{ColumnAlign, GridColumn, GridState};

let state = GridState::new(vec![GridColumn::new("name", "Name", ColumnAlign::Left)]);
assert_eq!(state.dim, 0.45);
```
