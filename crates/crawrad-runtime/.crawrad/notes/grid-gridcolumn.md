---
id: grid-gridcolumn
kind: doc
target_file: src/grid.rs
line_hint: 9
author: 
created: 
title: "GridColumn"
tags: []
signature: "pub struct GridColumn"
---

Column metadata describing how a field is identified and labeled.

# Examples

```rust
use crawrad_runtime::grid::{ColumnAlign, GridColumn};

let col = GridColumn::new("id", "ID", ColumnAlign::Right);
assert_eq!(col.name, "id");
```
