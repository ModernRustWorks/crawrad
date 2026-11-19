---
id: canvas-brush-brush-transformacao
kind: doc
target_file: src/canvas/brush.rs
line_hint: 550
author: 
created: 
title: "Brush::transformacao"
tags: []
signature: "pub fn transformacao (& self) -> & Transformacao"
---

The frame's transform — to READ, never to redo the arithmetic.

# Examples

```rust
use crawrad_runtime_makepad::canvas::{Brush, Probe};
use crawrad_runtime_makepad::Transformacao;

let mut probe = Probe::default();
let brush = Brush::probing(&mut probe, Transformacao::new(1.0, 2.0, 1.0));
assert_eq!(brush.transformacao().origem_x, 1.0);
```
