---
id: string-grid-gridlayout
kind: doc
target_file: src/string_grid.rs
line_hint: 43
author: 
created: 
title: "GridLayout"
tags: []
signature: "pub struct GridLayout"
---

Grid geometry and layout calculation.

# Examples

```rust
use crawrad_runtime::string_grid::GridLayout;

let layout = GridLayout::from_props(5, 10, 1, 1, 64, 24);
assert_eq!(layout.col_count, 5);
```
