---
id: canvas-brush-rect-on-screen
kind: doc
target_file: src/canvas/brush.rs
line_hint: 93
author: 
created: 
title: "rect_on_screen"
tags: []
signature: "pub fn rect_on_screen (t : & Transformacao , r : RectIr) -> Rect"
---

A rectangle in design units becomes a rectangle on screen.

A free function on purpose: it does not depend on `Cx`, so it is measurable
without a window. And it is the canvas's only design -> screen path —
whoever draws calls this, nobody adds the origin by hand.

# Examples

```rust
use makepad_widgets::dvec2;
use crawrad_ir::Rect as RectIr;
use crawrad_runtime_makepad::canvas::rect_on_screen;
use crawrad_runtime_makepad::Transformacao;

let t = Transformacao::new(10.0, 20.0, 1.0);
let screen = rect_on_screen(&t, RectIr::new(0.0, 0.0, 50.0, 30.0));
assert_eq!(screen.pos, dvec2(10.0, 20.0));
```
