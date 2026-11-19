---
id: string-grid-cells-set
kind: doc
target_file: src/string_grid.rs
line_hint: 32
author: 
created: 
title: "Cells::set"
tags: []
signature: "pub fn set (& mut self , col : usize , row : usize , text : impl Into < String >)"
---

Sets the text stored at `(col, row)`.

Setting an empty string removes the entry.

# Examples

```rust
use crawrad_runtime::string_grid::Cells;

let mut cells = Cells::default();
cells.set(0, 0, "test");
assert_eq!(cells.get(0, 0), "test");
cells.set(0, 0, "");
assert_eq!(cells.get(0, 0), "");
```
