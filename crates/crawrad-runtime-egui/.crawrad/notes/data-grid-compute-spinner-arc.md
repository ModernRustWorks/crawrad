---
id: data-grid-compute-spinner-arc
kind: doc
target_file: src/data_grid.rs
line_hint: 98
author: 
created: 
title: "compute_spinner_arc"
tags: []
signature: "pub fn compute_spinner_arc (center : Pos2 , radius : f32 , time : f64 , n_points : usize) -> Vec < Pos2 >"
---

Generates polyline points for the loading spinner rotating circular arc.

Generates the polyline points forming a rotating circular arc for the loading spinner.

# Examples

```rust
use egui::pos2;
use crawrad_runtime_egui::data_grid::compute_spinner_arc;

let points = compute_spinner_arc(pos2(100.0, 100.0), 10.0, 0.0, 16);
assert_eq!(points.len(), 17);
```
