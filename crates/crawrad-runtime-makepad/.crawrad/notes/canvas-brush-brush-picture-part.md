---
id: canvas-brush-brush-picture-part
kind: doc
target_file: src/canvas/brush.rs
line_hint: 608
author: 
created: 
title: "Brush::picture_part"
tags: []
signature: "pub fn picture_part (& mut self , r : Rect , bytes : & [u8] , part : [f32 ; 4]) -> bool"
---

Paints a sub-rectangle of the picture stretched to `r`.

Paints a sub-rectangle `part` (in `0.0..1.0` of the image) of
the picture stretched to `r`, on [`Layer::Picture`].

# Examples

```rust
use makepad_widgets::{dvec2, Rect};
use crawrad_runtime_makepad::canvas::{Brush, Probe};
use crawrad_runtime_makepad::Transformacao;

let mut probe = Probe::default();
let mut brush = Brush::probing(&mut probe, Transformacao::new(0.0, 0.0, 1.0));
let r = Rect { pos: dvec2(0.0, 0.0), size: dvec2(50.0, 50.0) };
assert!(!brush.picture_part(r, &[], [0.0, 0.0, 1.0, 1.0]));
```
