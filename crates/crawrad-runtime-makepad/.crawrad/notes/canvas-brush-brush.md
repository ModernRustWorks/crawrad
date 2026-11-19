---
id: canvas-brush-brush
kind: doc
target_file: src/canvas/brush.rs
line_hint: 340
author: 
created: 
title: "Brush"
tags: []
signature: "pub struct Brush < 'a , 'b , 'c >"
---

One frame's painting surface.

It owns nothing — neither the shaders nor the `Cx2d` — and it does not
outlive the frame. And it **decides nothing**: there is no hit-test here, and
no selection choice (ADR 0027 §3).

# Examples

```rust
use crawrad_runtime_makepad::canvas::{Brush, Probe};
use crawrad_runtime_makepad::Transformacao;

let mut probe = Probe::default();
let brush = Brush::probing(&mut probe, Transformacao::new(0.0, 0.0, 1.0));
assert_eq!(brush.medida(10.0), 10.0);
```
