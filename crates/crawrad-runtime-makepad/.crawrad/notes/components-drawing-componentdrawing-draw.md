---
id: components-drawing-componentdrawing-draw
kind: doc
target_file: src/components/drawing.rs
line_hint: 97
author: 
created: 
title: "ComponentDrawing::draw"
tags: []
signature: "fn draw (& self , p : & mut Brush , c : & MakepadCtl , r : Rect , scene : & Scene)"
---

Paints the component. `r` is already the rectangle ON SCREEN.

# Examples

```rust,no_run
// MakepadCtl and Brush require the Makepad canvas runtime.
use makepad_widgets::Rect;
use crawrad_runtime_makepad::MakepadCtl;
use crawrad_runtime_makepad::canvas::Brush;
use crawrad_runtime_makepad::components::{ComponentDrawing, Scene};

fn example(drawing: &dyn ComponentDrawing, p: &mut Brush, c: &MakepadCtl, r: Rect, s: &Scene) {
    drawing.draw(p, c, r, s);
}
```
