---
id: splash-window-painter
kind: doc
target_file: src/splash_window.rs
line_hint: 43
author: 
created: 
title: "Painter"
tags: []
signature: "pub trait Painter : Send + 'static"
---

What a splash window shows.

# Examples

```rust
use crawrad_os::splash_window::{Frame, Painter};

struct MyPainter;
impl Painter for MyPainter {
    fn size(&self) -> (f32, f32) { (400.0, 300.0) }
    fn picture(&self, _w: usize, _h: usize) -> Vec<u32> { Vec::new() }
    fn frame(&mut self, _other_window_up: bool) -> Option<Frame> { None }
}

let painter = MyPainter;
assert_eq!(painter.size(), (400.0, 300.0));
```
