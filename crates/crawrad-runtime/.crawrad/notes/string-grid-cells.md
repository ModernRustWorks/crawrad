---
id: string-grid-cells
kind: doc
target_file: src/string_grid.rs
line_hint: 6
author: 
created: 
title: "Cells"
tags: []
signature: "pub struct Cells"
---

Cell text contents mapped by `(col, row)`.

# Examples

```rust
use crawrad_runtime::string_grid::Cells;

let mut cells = Cells::default();
cells.set(0, 0, "Hello");
assert_eq!(cells.get(0, 0), "Hello");
```
