---
id: color-picker-strip-fraction
kind: doc
target_file: src/color_picker.rs
line_hint: 348
author: 
created: 
title: "strip_fraction"
tags: []
signature: "pub fn strip_fraction (top : f64 , height : f64 , y : f64) -> f32"
---

Computes the normalized vertical fraction along a vertical strip.

Computes the vertical fraction in `0.0..=1.0` along a vertical strip
starting at `top` with height `height`.

# Examples

```rust
use crawrad_runtime_makepad::color_picker::strip_fraction;

let frac = strip_fraction(10.0, 100.0, 35.0);
assert!((frac - 0.25).abs() < 1e-5);
```
