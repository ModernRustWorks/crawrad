---
id: splash-window-painter-size
kind: doc
target_file: src/splash_window.rs
line_hint: 45
author: 
created: 
title: "Painter::size"
tags: []
signature: "fn size (& self) -> (f32 , f32)"
---

The window's size in 96-DPI pixels.

# Examples

```rust
use crawrad_os::splash_window::{Frame, Painter};

struct MyPainter;
impl Painter for MyPainter {
    fn size(&self) -> (f32, f32) { (200.0, 100.0) }
    fn picture(&self, _w: usize, _h: usize) -> Vec<u32> { Vec::new() }
    fn frame(&mut self, _other_window_up: bool) -> Option<Frame> { None }
}

assert_eq!(MyPainter.size(), (200.0, 100.0));
```
