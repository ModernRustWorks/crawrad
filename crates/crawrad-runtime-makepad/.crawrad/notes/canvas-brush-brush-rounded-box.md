---
id: canvas-brush-brush-rounded-box
kind: doc
target_file: src/canvas/brush.rs
line_hint: 570
author: 
created: 
title: "Brush::rounded_box"
tags: []
signature: "pub fn rounded_box (& mut self , r : Rect , fill : [f32 ; 4] , border : [f32 ; 4] , radius : f64)"
---

Paints a box with fill, border, and corner radius.

A box with a fill, a border and a radius: the primitive almost every
canvas control is made of.

`radius` is the corner radius **as everybody understands it** — half the
side gives a circle. The shader is the one that converts it into what
`Sdf2d` wants, and the reason lives there: `Sdf2d::box` DOUBLES the radius
it receives (`makepad-draw-1.0.0/src/shader/std.rs:394-401`).

# Examples

```rust
use makepad_widgets::{dvec2, Rect};
use crawrad_runtime_makepad::canvas::{Brush, Probe};
use crawrad_runtime_makepad::Transformacao;

let mut probe = Probe::default();
let mut brush = Brush::probing(&mut probe, Transformacao::new(0.0, 0.0, 1.0));
let r = Rect { pos: dvec2(0.0, 0.0), size: dvec2(100.0, 50.0) };
brush.rounded_box(r, [1.0, 1.0, 1.0, 1.0], [0.0, 0.0, 0.0, 1.0], 4.0);
assert_eq!(probe.marks().len(), 1);
```
