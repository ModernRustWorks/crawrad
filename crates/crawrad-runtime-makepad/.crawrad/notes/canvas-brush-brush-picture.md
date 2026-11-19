---
id: canvas-brush-brush-picture
kind: doc
target_file: src/canvas/brush.rs
line_hint: 603
author: 
created: 
title: "Brush::picture"
tags: []
signature: "pub fn picture (& mut self , r : Rect , bytes : & [u8]) -> bool"
---

Paints the picture stretched to `r` on `Layer::Picture`.

Paints the picture stretched to `r`, on [`Layer::Picture`].
Returns false when there is nothing to paint (empty/undecodable).

# Examples

```rust
use makepad_widgets::{dvec2, Rect};
use crawrad_runtime_makepad::canvas::{Brush, Probe};
use crawrad_runtime_makepad::Transformacao;

let mut probe = Probe::default();
let mut brush = Brush::probing(&mut probe, Transformacao::new(0.0, 0.0, 1.0));
let r = Rect { pos: dvec2(0.0, 0.0), size: dvec2(50.0, 50.0) };
assert!(!brush.picture(r, &[]));
```
