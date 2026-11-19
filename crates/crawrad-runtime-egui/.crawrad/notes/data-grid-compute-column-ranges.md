---
id: data-grid-compute-column-ranges
kind: doc
target_file: src/data_grid.rs
line_hint: 64
author: 
created: 
title: "compute_column_ranges"
tags: []
signature: "pub fn compute_column_ranges (available_width : f32 , widths : & [Option < f32 >]) -> Vec < (f32 , f32) >"
---

Computes horizontal coordinate intervals for each column.

Computes the horizontal coordinate intervals `(start_x, end_x)` for each column.

# Examples

```rust
use crawrad_runtime_egui::data_grid::compute_column_ranges;

let ranges = compute_column_ranges(300.0, &[None, None, None]);
assert_eq!(ranges, vec![(0.0, 100.0), (100.0, 200.0), (200.0, 300.0)]);
```
