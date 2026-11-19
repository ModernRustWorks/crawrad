---
id: grid-viewline
kind: doc
target_file: src/grid.rs
line_hint: 426
author: 
created: 
title: "ViewLine"
tags: []
signature: "pub enum ViewLine"
---

A line in the grid viewport: either a group summary header or a data row.

# Examples

```rust
use crawrad_runtime::grid::ViewLine;

let line = ViewLine::Row {
    index: 0,
    id: 0,
    cells: vec!["Alice".into()],
    raw_cells: vec![Some("Alice".into())],
    odd: false,
    selected: true,
    marker: "",
};
match line {
    ViewLine::Row { index, .. } => assert_eq!(index, 0),
    _ => unreachable!(),
}
```
