---
id: canvas-brush-brush-write
kind: doc
target_file: src/canvas/brush.rs
line_hint: 800
author: 
created: 
title: "Brush::write"
tags: []
signature: "pub fn write (& mut self , area : Rect , text : & str , c : [f32 ; 4] , centered : bool)"
---

Writes inside `area`, always centred vertically and horizontally when asked for.

Writes inside `area`, always centred vertically and horizontally when
asked for.

# Examples

```rust
use makepad_widgets::{dvec2, Rect};
use crawrad_runtime_makepad::canvas::{Brush, Probe};
use crawrad_runtime_makepad::Transformacao;

let mut probe = Probe::default();
let mut brush = Brush::probing(&mut probe, Transformacao::new(0.0, 0.0, 1.0));
let r = Rect { pos: dvec2(0.0, 0.0), size: dvec2(100.0, 30.0) };
brush.write(r, "Label", [0.0, 0.0, 0.0, 1.0], false);
assert_eq!(probe.marks().len(), 1);
```
