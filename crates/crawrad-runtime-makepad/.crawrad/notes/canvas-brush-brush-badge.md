---
id: canvas-brush-brush-badge
kind: doc
target_file: src/canvas/brush.rs
line_hint: 655
author: 
created: 
title: "Brush::badge"
tags: []
signature: "pub fn badge (& mut self , r : Rect , fill : [f32 ; 4] , border : [f32 ; 4] , radius : f64)"
---

Paints the box of a non-visual badge above visual components.

The box of a non-visual BADGE — the same figure as
[`Brush::rounded_box`], **on a layer above every visual component**.

The rule is the owner's: *"the drawings of non-visual components in
designer mode must always sit in front of every visual component"*. In
a classic RAD IDE the non-visual is a designer overlay — it takes no part in the
form's Z order — and an `ActionList` hidden behind a `Panel` is a
component nobody can click to select.

Why a shader just for this: in Makepad the painting order goes by SHADER,
not by call order (the whole note is in the canvas DSL). Reusing
[`Brush::rounded_box`] put the badge in the same call as the components,
where there is no Z between instances.

# Examples

```rust
use makepad_widgets::{dvec2, Rect};
use crawrad_runtime_makepad::canvas::{Brush, Probe};
use crawrad_runtime_makepad::Transformacao;

let mut probe = Probe::default();
let mut brush = Brush::probing(&mut probe, Transformacao::new(0.0, 0.0, 1.0));
let r = Rect { pos: dvec2(0.0, 0.0), size: dvec2(30.0, 20.0) };
brush.badge(r, [1.0, 1.0, 1.0, 1.0], [0.0, 0.0, 0.0, 1.0], 2.0);
assert_eq!(probe.marks().len(), 1);
```
