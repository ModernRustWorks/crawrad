---
id: components-grid-grid-column
kind: doc
target_file: src/components/grid.rs
line_hint: 39
author: 
created: 
title: "grid_column"
tags: []
signature: "pub fn grid_column () -> ComponentDesc"
---

A column of a `Grid` (#936): which field it shows and how.

Off the palette: a column is born inside a grid, from its columns editor.

# Examples

```rust
let col = crawrad_registry::components::grid_column();
assert_eq!(col.class, "GridColumn");
assert!(!col.emits_code);
```
