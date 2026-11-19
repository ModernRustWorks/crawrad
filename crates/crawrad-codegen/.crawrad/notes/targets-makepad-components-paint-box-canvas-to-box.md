---
id: targets-makepad-components-paint-box-canvas-to-box
kind: doc
target_file: src/targets/makepad/components/paint_box.rs
line_hint: 113
author: 
created: 
title: "canvas_to_box"
tags: []
signature: "pub fn canvas_to_box (call : & str , w : f32 , h : f32 , path : & str) -> String"
---

Runs an `OnPaint` handler and hands what it drew to a `CrawradPaintBox`.

Runs an `OnPaint` and hands what it drew to a `CrawradPaintBox` (#459),
for a PaintBox and for the form: `call` is the handler call, `w` x `h`
the canvas size, `path` the box's id path.

# Examples

```rust
use crawrad_codegen::targets::makepad::components::paint_box::canvas_to_box;

let code = canvas_to_box("self.paint(&mut canvas);", 100.0, 50.0, "box1");
assert!(code.contains("let mut canvas = Canvas::new(100.0, 50.0);"));
assert!(code.contains("self.paint(&mut canvas);"));
```
