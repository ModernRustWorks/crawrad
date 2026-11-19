---
id: canvas-brush-brush-measure
kind: doc
target_file: src/canvas/brush.rs
line_hint: 795
author: 
created: 
title: "Brush::measure"
tags: []
signature: "pub fn measure (& mut self , text : & str) -> DVec2"
---

The width and the height `text` takes up, in screen pixels.

# Examples

```rust
use crawrad_runtime_makepad::canvas::{Brush, Probe};
use crawrad_runtime_makepad::Transformacao;

let mut probe = Probe::default();
let mut brush = Brush::probing(&mut probe, Transformacao::new(0.0, 0.0, 1.0));
let size = brush.measure("Hi");
assert!(size.x > 0.0);
```
