---
id: canvas-brush-brush-badge-icon
kind: doc
target_file: src/canvas/brush.rs
line_hint: 746
author: 
created: 
title: "Brush::badge_icon"
tags: []
signature: "pub fn badge_icon (& mut self , r : Rect , steps : & [crawrad_ir :: icon :: IconStep] , ink : [f32 ; 4]) -> bool"
---

Paints a class icon inside a non-visual badge, on the badge layer.

The icon is rasterized on the CPU (`crawrad_ir::icon::rasterize`, the same port
of `Sdf2d` the palette strip uses) and emitted as runs of solid pixels through
the badge shader itself. Same shader, later instances: the icon lands above
the badge box and below the name, and stays in front of every visual
component.

Returns `false`, painting nothing, when there are no steps or the box is too
small — the caller then writes the initials.

# Examples

```rust
use makepad_widgets::{dvec2, Rect};
use crawrad_ir::icon::IconStep;
use crawrad_runtime_makepad::canvas::{Brush, Probe};
use crawrad_runtime_makepad::Transformacao;

let mut probe = Probe::default();
let mut brush = Brush::probing(&mut probe, Transformacao::new(0.0, 0.0, 1.0));
let r = Rect { pos: dvec2(0.0, 0.0), size: dvec2(16.0, 16.0) };
let dot = [IconStep::Circle { x: 8.0, y: 8.0, r: 4.0 }, IconStep::Fill];
assert!(brush.badge_icon(r, &dot, [0.0, 0.0, 0.0, 1.0]));
assert!(!brush.badge_icon(r, &[], [0.0, 0.0, 0.0, 1.0]));
assert_eq!(probe.marks().len(), 1);
```
