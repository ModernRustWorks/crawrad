---
id: anchors-rect
kind: doc
target_file: src/anchors.rs
line_hint: 28
author: 
created: 
title: "rect"
tags: []
signature: "pub fn rect (design : (f64 , f64 , f64 , f64) , a : Anchors , grown : DVec2) -> (f64 , f64 , f64 , f64)"
---

Calculates where an anchored control ends up when the form grew.

Where a control with the design rect `(x, y, w, h)` ends up when the form
grew by `grown`. The floor of 1.0 is the painter's, as in the egui
target: the sum itself has none, so nothing is lost when the window
shrinks and grows back.

# Examples

```rust
use makepad_widgets::dvec2;
use crawrad_runtime_makepad::anchors::{rect, Anchors};

let r = rect((10.0, 10.0, 100.0, 30.0), Anchors::default(), dvec2(50.0, 20.0));
assert_eq!(r, (10.0, 10.0, 100.0, 30.0));
```
