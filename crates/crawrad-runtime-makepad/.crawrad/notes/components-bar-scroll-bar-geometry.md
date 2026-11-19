---
id: components-bar-scroll-bar-geometry
kind: doc
target_file: src/components/bar.rs
line_hint: 65
author: 
created: 
title: "scroll_bar_geometry"
tags: []
signature: "pub fn scroll_bar_geometry (r : Rect , position : f32 , scale : f64) -> ScrollBarGeometry"
---

Calculates the geometry of the scroll bar that fills `r`.

The geometry of the bar that fills `r`, with the thumb at `position`
(0 to 100).

`scale` is the frame's `Transformacao::medida(1.0)`: the constants above are
in design units and it is what puts them on screen.

# Examples

```rust
use makepad_widgets::{dvec2, Rect};
use crawrad_runtime_makepad::components::bar::scroll_bar_geometry;

let r = Rect { pos: dvec2(0.0, 0.0), size: dvec2(100.0, 16.0) };
let geom = scroll_bar_geometry(r, 0.0, 1.0);
assert!(geom.horizontal);
```
