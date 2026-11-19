---
id: data-grid-max-columns
kind: doc
target_file: src/data_grid.rs
line_hint: 404
author: 
created: 
title: "MAX_COLUMNS"
tags: []
signature: "pub const MAX_COLUMNS : usize = 16"
---

The maximum number of columns the grid shows.

How many columns the grid shows: the cell slots `c0`..`c15` in the DSL of
the header and of the row template. A source with more columns shows the
first [`MAX_COLUMNS`].

# Examples

```rust
use crawrad_runtime_makepad::data_grid::MAX_COLUMNS;

assert_eq!(MAX_COLUMNS, 16);
```
