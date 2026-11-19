---
id: canvas-brush-shrink
kind: doc
target_file: src/canvas/brush.rs
line_hint: 107
author: 
created: 
title: "shrink"
tags: []
signature: "pub fn shrink (r : Rect , d : f64) -> Rect"
---

Shrinks a screen rectangle by `d` on each side, without turning it inside out.

Shrinks a screen rectangle by `d` on each side, without turning it inside
out.

# Examples

```rust
use makepad_widgets::{dvec2, Rect};
use crawrad_runtime_makepad::canvas::shrink;

let r = Rect { pos: dvec2(0.0, 0.0), size: dvec2(20.0, 20.0) };
let s = shrink(r, 2.0);
assert_eq!(s.pos, dvec2(2.0, 2.0));
assert_eq!(s.size, dvec2(16.0, 16.0));
```
