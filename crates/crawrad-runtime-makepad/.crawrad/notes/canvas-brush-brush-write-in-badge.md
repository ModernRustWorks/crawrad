---
id: canvas-brush-brush-write-in-badge
kind: doc
target_file: src/canvas/brush.rs
line_hint: 672
author: 
created: 
title: "Brush::write_in_badge"
tags: []
signature: "pub fn write_in_badge (& mut self , area : Rect , text : & str , c : [f32 ; 4] , centered : bool)"
---

A badge's text — the initials and the name — on the badge layer.

**The two have to rise together.** If only the box went up a layer, the
badge would sit in front and its initials behind: the canvas's ordinary
text comes out of `DrawText`, which is a lower call than [`DrawSelo`]'s.

# Examples

```rust
use makepad_widgets::{dvec2, Rect};
use crawrad_runtime_makepad::canvas::{Brush, Probe};
use crawrad_runtime_makepad::Transformacao;

let mut probe = Probe::default();
let mut brush = Brush::probing(&mut probe, Transformacao::new(0.0, 0.0, 1.0));
let r = Rect { pos: dvec2(0.0, 0.0), size: dvec2(40.0, 20.0) };
brush.write_in_badge(r, "TMR", [0.0, 0.0, 0.0, 1.0], true);
assert_eq!(probe.marks().len(), 1);
```
