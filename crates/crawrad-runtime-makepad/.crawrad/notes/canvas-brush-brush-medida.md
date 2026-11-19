---
id: canvas-brush-brush-medida
kind: doc
target_file: src/canvas/brush.rs
line_hint: 560
author: 
created: 
title: "Brush::medida"
tags: []
signature: "pub fn medida (& self , v : f32) -> f64"
---

A design measure in screen pixels.

# Examples

```rust
use crawrad_runtime_makepad::canvas::{Brush, Probe};
use crawrad_runtime_makepad::Transformacao;

let mut probe = Probe::default();
let brush = Brush::probing(&mut probe, Transformacao::new(0.0, 0.0, 2.0));
assert_eq!(brush.medida(10.0), 20.0);
```
