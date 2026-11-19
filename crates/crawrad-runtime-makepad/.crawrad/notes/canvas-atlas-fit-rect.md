---
id: canvas-atlas-fit-rect
kind: doc
target_file: src/canvas/atlas.rs
line_hint: 269
author: 
created: 
title: "fit_rect"
tags: []
signature: "pub fn fit_rect (control : Rect , image_w : f64 , image_h : f64 , stretch : bool , proportional : bool , center : bool ,) -> Rect"
---

Computes the picture rectangle inside `control` by RAD rules.

Computes the picture rectangle inside `control` following classic RAD rules:
- `proportional`: aspect kept, fits inside control bounds.
- `stretch` only: fills the entire control.
- neither: original pixel size.
- `center`: centered inside control; otherwise aligned top-left.

# Examples

```rust
use makepad_widgets::{dvec2, Rect};
use crawrad_runtime_makepad::canvas::atlas::fit_rect;

let ctrl = Rect { pos: dvec2(0.0, 0.0), size: dvec2(100.0, 100.0) };
let fitted = fit_rect(ctrl, 50.0, 50.0, false, false, false);
assert_eq!(fitted.size, dvec2(50.0, 50.0));
```
