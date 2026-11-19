---
id: data-grid-compute-loading-card-rect
kind: doc
target_file: src/data_grid.rs
line_hint: 85
author: 
created: 
title: "compute_loading_card_rect"
tags: []
signature: "pub fn compute_loading_card_rect (rows_rect : Rect , text_size : Vec2 , spinner_size : f32 , padding : Vec2 , gap : f32 ,) -> Rect"
---

Computes bounds of the loading overlay card centered over the rows area.

Computes the screen bounds of the loading overlay card centered over the rows area.

# Examples

```rust
use egui::{Pos2, Rect, vec2};
use crawrad_runtime_egui::data_grid::compute_loading_card_rect;

let rows = Rect::from_min_size(Pos2::ZERO, vec2(800.0, 600.0));
let card = compute_loading_card_rect(rows, vec2(100.0, 20.0), 20.0, vec2(10.0, 10.0), 10.0);
assert_eq!(card.center(), rows.center());
```
