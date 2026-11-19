---
id: icon-rasterize
kind: doc
target_file: src/icon.rs
line_hint: 161
author: 
created: 
title: "rasterize"
tags: []
signature: "pub fn rasterize (steps : & [IconStep] , half_stroke : f32 , side : u32) -> Vec < f32 >"
---

Coverage (0..=1) of each pixel, row-major, `side * side` values.

# Examples

```rust
use crawrad_ir::icon::{rasterize, IconStep};

let steps = [
    IconStep::Rect { x: 0.0, y: 0.0, w: 16.0, h: 16.0 },
    IconStep::Fill,
];
let pixels = rasterize(&steps, 0.5, 16);
assert_eq!(pixels.len(), 256);
```
