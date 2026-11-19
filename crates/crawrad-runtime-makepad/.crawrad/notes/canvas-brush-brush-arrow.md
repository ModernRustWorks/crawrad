---
id: canvas-brush-brush-arrow
kind: doc
target_file: src/canvas/brush.rs
line_hint: 724
author: 
created: 
title: "Brush::arrow"
tags: []
signature: "pub fn arrow (& mut self , r : Rect , direction : DVec2 , c : [f32 ; 4])"
---

A triangle pointing at `direction` (a unit vector), inside `r`.

It serves the `ComboBox` arrow (which points down) and a scroll bar's two
buttons (which point all four ways). It is a vector, and not a
four-armed enum, because rotating in the shader by angle forces the
drawing to know that Makepad's Y axis grows DOWNWARDS — and that is the
kind of detail that comes out wrong once every two months.

# Examples

```rust
use makepad_widgets::{dvec2, Rect};
use crawrad_runtime_makepad::canvas::{Brush, Probe};
use crawrad_runtime_makepad::Transformacao;

let mut probe = Probe::default();
let mut brush = Brush::probing(&mut probe, Transformacao::new(0.0, 0.0, 1.0));
let r = Rect { pos: dvec2(0.0, 0.0), size: dvec2(10.0, 10.0) };
brush.arrow(r, dvec2(0.0, 1.0), [0.0, 0.0, 0.0, 1.0]);
assert_eq!(probe.marks().len(), 1);
```
