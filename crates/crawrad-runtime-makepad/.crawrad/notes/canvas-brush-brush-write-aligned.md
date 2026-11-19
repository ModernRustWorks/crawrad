---
id: canvas-brush-brush-write-aligned
kind: doc
target_file: src/canvas/brush.rs
line_hint: 805
author: 
created: 
title: "Brush::write_aligned"
tags: []
signature: "pub fn write_aligned (& mut self , area : Rect , text : & str , c : [f32 ; 4] , alignment : & str)"
---

Writes inside `area` with the specified horizontal alignment.

Writes inside `area`, vertically centered, with horizontal alignment
(`taLeftJustify`, `taCenter`, `taRightJustify`).

# Examples

```rust
use makepad_widgets::{dvec2, Rect};
use crawrad_runtime_makepad::canvas::{Brush, Probe};
use crawrad_runtime_makepad::Transformacao;

let mut probe = Probe::default();
let mut brush = Brush::probing(&mut probe, Transformacao::new(0.0, 0.0, 1.0));
let r = Rect { pos: dvec2(0.0, 0.0), size: dvec2(100.0, 30.0) };
brush.write_aligned(r, "Center", [0.0, 0.0, 0.0, 1.0], "taCenter");
assert_eq!(probe.marks().len(), 1);
```
