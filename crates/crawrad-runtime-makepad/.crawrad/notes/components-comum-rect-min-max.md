---
id: components-comum-rect-min-max
kind: doc
target_file: src/components/comum.rs
line_hint: 139
author: 
created: 
title: "rect_min_max"
tags: []
signature: "pub fn rect_min_max (x1 : f32 , y1 : f32 , x2 : f32 , y2 : f32) -> Rect"
---

Builds a rectangle ensuring vertex order to avoid panics.

# Examples

```rust
use crawrad_runtime_makepad::components::comum::rect_min_max;

let r = rect_min_max(10.0, 20.0, 30.0, 40.0);
assert_eq!(r.x, 10.0);
assert_eq!(r.w, 20.0);
```
