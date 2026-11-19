---
id: canvas-brush-brush-stroke
kind: doc
target_file: src/canvas/brush.rs
line_hint: 769
author: 
created: 
title: "Brush::stroke"
tags: []
signature: "pub fn stroke (& mut self , r : Rect , c : [f32 ; 4])"
---

Paints a solid rectangle with no border and no radius.

A solid rectangle, with no border and no radius: a frame side, the
accelerator underline and the highlight of the chosen line.

# Examples

```rust
use makepad_widgets::{dvec2, Rect};
use crawrad_runtime_makepad::canvas::{Brush, Probe};
use crawrad_runtime_makepad::Transformacao;

let mut probe = Probe::default();
let mut brush = Brush::probing(&mut probe, Transformacao::new(0.0, 0.0, 1.0));
let r = Rect { pos: dvec2(0.0, 0.0), size: dvec2(10.0, 10.0) };
brush.stroke(r, [0.0, 0.0, 0.0, 1.0]);
assert_eq!(probe.marks().len(), 1);
```
