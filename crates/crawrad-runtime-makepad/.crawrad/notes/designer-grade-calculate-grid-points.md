---
id: designer-grade-calculate-grid-points
kind: doc
target_file: src/designer/grade.rs
line_hint: 6
author: 
created: 
title: "calculate_grid_points"
tags: []
signature: "pub fn calculate_grid_points (form_rect : Rect , step : f32) -> Vec < (f32 , f32) >"
---

Calculates grid dot points within the given form rectangle.

# Examples

```rust
use crawrad_ir::Rect;
use crawrad_runtime_makepad::designer::calculate_grid_points;

let r = Rect::new(0.0, 0.0, 30.0, 30.0);
let points = calculate_grid_points(r, 10.0);
assert_eq!(points.len(), 4);
```
