---
id: radio-group-cells
kind: doc
target_file: src/radio_group.rs
line_hint: 72
author: 
created: 
title: "cells"
tags: []
signature: "pub fn cells (w : f32 , h : f32 , n : usize , columns : i64) -> Vec < (f32 , f32 , f32 , f32) >"
---

Computes item cell rectangles laid out in top-down columns.

The cell of each of `n` items in a group `w` x `h`, laid out in
`columns` columns filled top down, in the group's own coordinates.

# Examples

```rust
use crawrad_runtime_makepad::radio_group::cells;

let item_cells = cells(200.0, 100.0, 3, 1);
assert_eq!(item_cells.len(), 3);
```
