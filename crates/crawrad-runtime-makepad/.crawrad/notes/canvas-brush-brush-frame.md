---
id: canvas-brush-brush-frame
kind: doc
target_file: src/canvas/brush.rs
line_hint: 783
author: 
created: 
title: "Brush::frame"
tags: []
signature: "pub fn frame (& mut self , r : Rect , c : [f32 ; 4] , thickness : f64)"
---

A frame `thickness` thick inside `r`, in four strokes.

# Examples

```rust
use makepad_widgets::{dvec2, Rect};
use crawrad_runtime_makepad::canvas::{Brush, Probe};
use crawrad_runtime_makepad::Transformacao;

let mut probe = Probe::default();
let mut brush = Brush::probing(&mut probe, Transformacao::new(0.0, 0.0, 1.0));
let r = Rect { pos: dvec2(0.0, 0.0), size: dvec2(20.0, 20.0) };
brush.frame(r, [0.0, 0.0, 0.0, 1.0], 1.0);
assert_eq!(probe.marks().len(), 4);
```
