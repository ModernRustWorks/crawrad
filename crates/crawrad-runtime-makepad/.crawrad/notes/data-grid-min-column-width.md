---
id: data-grid-min-column-width
kind: doc
target_file: src/data_grid.rs
line_hint: 438
author: 
created: 
title: "MIN_COLUMN_WIDTH"
tags: []
signature: "pub const MIN_COLUMN_WIDTH : f64 = 60.0"
---

The narrowest a column gets, so a wide table stays legible (it clips).

# Examples

```rust
use crawrad_runtime_makepad::data_grid::MIN_COLUMN_WIDTH;

assert_eq!(MIN_COLUMN_WIDTH, 60.0);
```
