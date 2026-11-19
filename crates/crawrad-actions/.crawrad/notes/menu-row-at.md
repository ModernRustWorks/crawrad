---
id: menu-row-at
kind: doc
target_file: src/menu.rs
line_hint: 335
author: 
created: 
title: "row_at"
tags: []
signature: "pub fn row_at (rows : & [Row] , y : f64 , row : f64 , rule : f64) -> Option < usize >"
---

The row `y` pixels below the first row's top falls on, or `None` outside them.

# Examples

```rust
use crawrad_actions::menu::row_at;
use crawrad_actions::Row;

let rows = vec![Row::custom("Run", None), Row::rule(), Row::custom("Step Over", None)];
assert_eq!(row_at(&rows, 30.0, 22.0, 7.0), Some(2));
assert_eq!(row_at(&rows, 60.0, 22.0, 7.0), None);
```
