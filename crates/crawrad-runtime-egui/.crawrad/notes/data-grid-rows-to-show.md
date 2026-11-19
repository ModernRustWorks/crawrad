---
id: data-grid-rows-to-show
kind: doc
target_file: src/data_grid.rs
line_hint: 75
author: 
created: 
title: "rows_to_show"
tags: []
signature: "pub fn rows_to_show (total : Option < usize > , shown_total : usize) -> usize"
---

Returns how many rows the scroll area spans.

How many rows the scroll area spans: the source's count once it has one,
the last count while a new query is still being counted.

# Examples

```rust
use crawrad_runtime_egui::data_grid::rows_to_show;

assert_eq!(rows_to_show(Some(42), 100), 42);
assert_eq!(rows_to_show(None, 100), 100);
```
