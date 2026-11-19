---
id: canvas-brush-brush-na-tela
kind: doc
target_file: src/canvas/brush.rs
line_hint: 555
author: 
created: 
title: "Brush::na_tela"
tags: []
signature: "pub fn na_tela (& self , r : RectIr) -> Rect"
---

A design rectangle at its place on screen.

# Examples

```rust
use makepad_widgets::dvec2;
use crawrad_ir::Rect as RectIr;
use crawrad_runtime_makepad::canvas::{Brush, Probe};
use crawrad_runtime_makepad::Transformacao;

let mut probe = Probe::default();
let brush = Brush::probing(&mut probe, Transformacao::new(0.0, 0.0, 1.0));
let r = brush.na_tela(RectIr::new(10.0, 20.0, 30.0, 40.0));
assert_eq!(r.pos, dvec2(10.0, 20.0));
```
