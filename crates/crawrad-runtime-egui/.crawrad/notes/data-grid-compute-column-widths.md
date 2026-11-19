---
id: data-grid-compute-column-widths
kind: doc
target_file: src/data_grid.rs
line_hint: 42
author: 
created: 
title: "compute_column_widths"
tags: []
signature: "pub fn compute_column_widths (available_width : f32 , widths : & [Option < f32 >]) -> Vec < f32 >"
---

Each column's width (#936).

Its own `width` as given, the others share the rest evenly.

The widths are not rounded, so the columns add up to the row with no
remainder to hand to the last one.

# Examples

```rust
use crawrad_runtime_egui::data_grid::compute_column_widths;

assert_eq!(compute_column_widths(300.0, &[None, None, None]), vec![100.0, 100.0, 100.0]);
assert_eq!(compute_column_widths(300.0, &[Some(100.0), None]), vec![100.0, 200.0]);
assert!(compute_column_widths(300.0, &[]).is_empty());
```
