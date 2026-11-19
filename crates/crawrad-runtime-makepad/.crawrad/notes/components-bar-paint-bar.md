---
id: components-bar-paint-bar
kind: doc
target_file: src/components/bar.rs
line_hint: 175
author: 
created: 
title: "paint_bar"
tags: []
signature: "pub fn paint_bar (p : & mut Brush , r : Rect , position : f32 , state : ScrollBarState)"
---

Paints a whole scroll bar inside `r`.

Paints a whole bar inside `r`: sunken track, the two buttons with their
arrow, and the thumb.

**The arrow is what the owner asked for** — *"no arrows at the ends, no
proportional thumb"*. The RXP paints both buttons as plain rectangles
(`crawrad-runtime-egui/src/lib.rs:2711-2712`); here they get the triangle, which
is what Windows and a classic RAD IDE draw and is what says which way the button goes.

Two things, and only two, change in a [`ScrollBarState::Disabled`] bar —
which is what the owner's classic RAD print shows: **the arrows go grey and the
sliding block is not drawn**. The track and the two buttons keep their
face, as Windows keeps them. Neither of the two decisions is taken here:
both are [`ScrollBarState::live`] and [`ScrollBarState::arrow`], which are
measurable without a window — painting needs a `Cx2d`, and a `Cx2d` needs
an open window.

# Examples

```rust
use makepad_widgets::{dvec2, Rect};
use crawrad_runtime_makepad::Transformacao;
use crawrad_runtime_makepad::canvas::{Brush, Probe};
use crawrad_runtime_makepad::components::bar::{paint_bar, ScrollBarState};

let mut probe = Probe::default();
let mut brush = Brush::probing(&mut probe, Transformacao::new(0.0, 0.0, 1.0));
let r = Rect { pos: dvec2(0.0, 0.0), size: dvec2(100.0, 16.0) };
paint_bar(&mut brush, r, 50.0, ScrollBarState::Active);
assert!(!probe.marks().is_empty());
```
