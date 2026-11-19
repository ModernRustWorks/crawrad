---
id: splash-window-spawn
kind: doc
target_file: src/splash_window.rs
line_hint: 55
author: 
created: 
title: "spawn"
tags: []
signature: "pub fn spawn (painter : impl Painter)"
---

Opens the window on its own thread; it lives until the painter ends it.

# Examples

```rust,no_run
// Opens an operating system window on a background thread.
use crawrad_os::splash_window::{Frame, Painter, spawn};

struct MyPainter;
impl Painter for MyPainter {
    fn size(&self) -> (f32, f32) { (300.0, 200.0) }
    fn picture(&self, _w: usize, _h: usize) -> Vec<u32> { Vec::new() }
    fn frame(&mut self, _other_up: bool) -> Option<Frame> { None }
}

spawn(MyPainter);
```
