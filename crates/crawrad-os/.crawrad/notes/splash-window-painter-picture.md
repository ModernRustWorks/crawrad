---
id: splash-window-painter-picture
kind: doc
target_file: src/splash_window.rs
line_hint: 48
author: 
created: 
title: "Painter::picture"
tags: []
signature: "fn picture (& self , width : usize , height : usize) -> Vec < u32 >"
---

The background, `width` × `height` physical pixels, `0x00RRGGBB`, row by row.

# Examples

```rust
use crawrad_os::splash_window::{Frame, Painter};

struct MyPainter;
impl Painter for MyPainter {
    fn size(&self) -> (f32, f32) { (10.0, 10.0) }
    fn picture(&self, w: usize, h: usize) -> Vec<u32> { vec![0; w * h] }
    fn frame(&mut self, _other_window_up: bool) -> Option<Frame> { None }
}

assert_eq!(MyPainter.picture(2, 2).len(), 4);
```
