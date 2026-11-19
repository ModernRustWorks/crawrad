---
id: icon-rasterize-oversampled
kind: doc
target_file: src/icon.rs
line_hint: 166
author: 
created: 
title: "rasterize_oversampled"
tags: []
signature: "pub fn rasterize_oversampled (steps : & [IconStep] , half_stroke : f32 , side : u32 , factor : u32 ,) -> Vec < f32 >"
---

Rasterizes the icon sampled `factor` times per pixel on each axis.

The same drawing sampled `factor` times per pixel on each axis, with the
antialias still that of a `side`-pixel icon: `(side * factor)²` values.

It is what keeps a texture of the icon as sharp as the shader was. The shader
evaluated the distance at the real pixel centre, and a list row often sits at
a fractional offset; a texture sampled at one value per pixel would then be
interpolated across two pixels and blur both edges. With quarter-pixel
samples the interpolation runs between points a quarter of a pixel apart,
where the antialias ramp is almost a straight line.

# Examples

```rust
use crawrad_ir::icon::{rasterize_oversampled, IconStep};

let steps = [
    IconStep::Rect { x: 0.0, y: 0.0, w: 16.0, h: 16.0 },
    IconStep::Fill,
];
let pixels = rasterize_oversampled(&steps, 0.5, 16, 2);
assert_eq!(pixels.len(), 1024);
```
