---
id: data-grid-compute-group-indent
kind: doc
target_file: src/data_grid.rs
line_hint: 80
author: 
created: 
title: "compute_group_indent"
tags: []
signature: "pub fn compute_group_indent (level : usize) -> f32"
---

Computes the left indentation padding in points for a group header line.

# Examples

```rust
use crawrad_runtime_egui::data_grid::compute_group_indent;

assert_eq!(compute_group_indent(0), 8.0);
assert_eq!(compute_group_indent(1), 24.0);
```
