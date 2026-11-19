---
id: splash-window-painter-frame
kind: doc
target_file: src/splash_window.rs
line_hint: 51
author: 
created: 
title: "Painter::frame"
tags: []
signature: "fn frame (& mut self , other_window_up : bool) -> Option < Frame >"
---

The next frame, or `None` to close the window.

The next frame, or `None` to close the window. `other_window_up` is
whether another visible top-level window of this process exists.

# Examples

```rust
use crawrad_os::splash_window::{Frame, Painter};

struct MyPainter;
impl Painter for MyPainter {
    fn size(&self) -> (f32, f32) { (10.0, 10.0) }
    fn picture(&self, _w: usize, _h: usize) -> Vec<u32> { Vec::new() }
    fn frame(&mut self, _other_window_up: bool) -> Option<Frame> { None }
}

assert!(MyPainter.frame(false).is_none());
```
