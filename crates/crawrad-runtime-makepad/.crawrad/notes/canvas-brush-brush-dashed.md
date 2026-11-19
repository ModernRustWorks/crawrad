---
id: canvas-brush-brush-dashed
kind: doc
target_file: src/canvas/brush.rs
line_hint: 713
author: 
created: 
title: "Brush::dashed"
tags: []
signature: "pub fn dashed (& mut self , r : Rect , c : [f32 ; 4])"
---

The dashed outline — marquee, lasso and a class with no drawing of its
own.

# Examples

```rust
use makepad_widgets::{dvec2, Rect};
use crawrad_runtime_makepad::canvas::{Brush, Probe};
use crawrad_runtime_makepad::Transformacao;

let mut probe = Probe::default();
let mut brush = Brush::probing(&mut probe, Transformacao::new(0.0, 0.0, 1.0));
let r = Rect { pos: dvec2(0.0, 0.0), size: dvec2(10.0, 10.0) };
brush.dashed(r, [0.0, 0.0, 0.0, 1.0]);
assert_eq!(probe.marks().len(), 1);
```
