---
id: paint-paint
kind: doc
target_file: src/paint.rs
line_hint: 162
author: 
created: 
title: "paint"
tags: []
signature: "pub fn paint (cx : & mut Cx2d , draw_paint : & mut DrawPaint , draw_text : & mut DrawText , origin : DVec2 , commands : & [Command] ,)"
---

Paints draw commands with their top-left corner at `origin`.

Paints `commands` with their top-left corner at `origin`: the drawing a
PaintBox's OnPaint asked for, and any widget of this crate that draws
itself the same way (#500: the RadioGroup).

# Examples

```rust,no_run
// Makepad Cx2d and shaders require the Makepad application runtime.
use makepad_widgets::{dvec2, Cx2d, DrawText};
use crawrad_runtime_makepad::paint::{paint, DrawPaint};

fn render(cx: &mut Cx2d, draw_paint: &mut DrawPaint, draw_text: &mut DrawText) {
    paint(cx, draw_paint, draw_text, dvec2(0.0, 0.0), &[]);
}
```
