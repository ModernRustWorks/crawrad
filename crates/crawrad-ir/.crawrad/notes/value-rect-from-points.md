---
id: value-rect-from-points
kind: doc
target_file: src/value.rs
line_hint: 849
author: 
created: 
title: "Rect::from_points"
tags: []
signature: "pub fn from_points (x0 : f32 , y0 : f32 , x1 : f32 , y1 : f32) -> Self"
---

Normalized rectangle from two points, in any order.

Normalized rectangle from two points, in any order — which is
what the marquee needs, since one drags in any direction.

# Examples

```rust
use crawrad_ir::Rect;

let r = Rect::from_points(100.0, 50.0, 10.0, 20.0);
assert_eq!(r, Rect::new(10.0, 20.0, 90.0, 30.0));
```
