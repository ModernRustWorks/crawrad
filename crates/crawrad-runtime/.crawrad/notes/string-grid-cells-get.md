---
id: string-grid-cells-get
kind: doc
target_file: src/string_grid.rs
line_hint: 27
author: 
created: 
title: "Cells::get"
tags: []
signature: "pub fn get (& self , col : usize , row : usize) -> & str"
---

Returns the text stored at `(col, row)`, or `""` if unset.

# Examples

```rust
use crawrad_runtime::string_grid::Cells;

let mut cells = Cells::default();
cells.set(1, 2, "Val");
assert_eq!(cells.get(1, 2), "Val");
assert_eq!(cells.get(0, 0), "");
```
