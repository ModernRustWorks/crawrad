---
id: canvas-brush-brush-probing
kind: doc
target_file: src/canvas/brush.rs
line_hint: 368
author: 
created: 
title: "Brush::probing"
tags: []
signature: "pub fn probing (probe : & 'a mut Probe , t : Transformacao) -> Self"
---

Creates a brush that paints on `probe` without the GPU (#114).

A brush that paints on `probe` instead of the GPU (#114), with the same
transform a frame would have.

# Examples

```rust
use crawrad_runtime_makepad::canvas::{Brush, Probe};
use crawrad_runtime_makepad::Transformacao;

let mut probe = Probe::default();
let brush = Brush::probing(&mut probe, Transformacao::new(0.0, 0.0, 1.0));
assert_eq!(brush.medida(1.0), 1.0);
```
