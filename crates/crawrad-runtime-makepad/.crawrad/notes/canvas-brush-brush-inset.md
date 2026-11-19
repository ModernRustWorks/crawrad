---
id: canvas-brush-brush-inset
kind: doc
target_file: src/canvas/brush.rs
line_hint: 565
author: 
created: 
title: "Brush::inset"
tags: []
signature: "pub fn inset (& self , r : Rect) -> Rect"
---

A control's inner rectangle — the margin every content respects.

# Examples

```rust
use makepad_widgets::{dvec2, Rect};
use crawrad_runtime_makepad::canvas::{Brush, Probe};
use crawrad_runtime_makepad::Transformacao;

let mut probe = Probe::default();
let brush = Brush::probing(&mut probe, Transformacao::new(0.0, 0.0, 1.0));
let r = Rect { pos: dvec2(0.0, 0.0), size: dvec2(100.0, 100.0) };
let inner = brush.inset(r);
assert!(inner.size.x < r.size.x);
```
