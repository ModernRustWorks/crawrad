---
id: canvas-brush-point-in-design
kind: doc
target_file: src/canvas/brush.rs
line_hint: 102
author: 
created: 
title: "point_in_design"
tags: []
signature: "pub fn point_in_design (t : & Transformacao , p : DVec2) -> (f32 , f32)"
---

A screen point becomes a point in design units.

# Examples

```rust
use makepad_widgets::dvec2;
use crawrad_runtime_makepad::canvas::point_in_design;
use crawrad_runtime_makepad::Transformacao;

let t = Transformacao::new(10.0, 20.0, 1.0);
let pt = point_in_design(&t, dvec2(10.0, 20.0));
assert_eq!(pt, (0.0, 0.0));
```
