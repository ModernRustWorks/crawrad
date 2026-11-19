---
id: components-non-visual-badge-box
kind: doc
target_file: src/components/non_visual.rs
line_hint: 48
author: 
created: 
title: "badge_box"
tags: []
signature: "pub fn badge_box (r : Rect) -> Rect"
---

The badge box, given the component rectangle **already on screen**.

It is the WHOLE rectangle, and that is the only defensible answer:
`Designer::hit` finds the click through the node's `rect_no_formulario`
(`crawrad-designer/src/lib.rs:402-409`), which is exactly this rectangle before
the [`crate::Transformacao`]. Drawing a square of its own size here — 28
screen points, say — would make the area one SEES disagree with the one that
SELECTS, and the disagreement would grow with every zoom step. The RXP
recorded the same defect in the `selo_nao_visual` comment
(`crawrad-runtime-egui/src/lib.rs:2386-2389`).

The function exists (instead of the drawing using `r` directly) because it
is THIS that the test measures against `Designer::hit`.

# Examples

```rust
use makepad_widgets::{dvec2, Rect};
use crawrad_runtime_makepad::components::non_visual::badge_box;

let r = Rect { pos: dvec2(10.0, 10.0), size: dvec2(28.0, 28.0) };
assert_eq!(badge_box(r), r);
```
