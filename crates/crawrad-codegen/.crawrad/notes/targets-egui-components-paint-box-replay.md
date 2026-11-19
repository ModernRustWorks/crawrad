---
id: targets-egui-components-paint-box-replay
kind: doc
target_file: src/targets/egui/components/paint_box.rs
line_hint: 61
author: 
created: 
title: "replay"
tags: []
signature: "pub fn replay (canvas : & str , rect : & str) -> String"
---

Paints the commands of a Canvas for OnPaint handlers.

**The commands of a `Canvas`, painted** (#459), for a PaintBox's and the
form's `OnPaint`: `canvas` and `rect` are the names of the canvas and of
the box it fills in the generated code; the painting is clipped to it.

# Examples

```rust
use crawrad_codegen::targets::egui::components::paint_box::replay;

let code = replay("canvas", "box_rect");
assert!(code.contains("painter_at(box_rect)"));
```
