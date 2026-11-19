---
id: color-picker-square-fractions
kind: doc
target_file: src/color_picker.rs
line_hint: 338
author: 
created: 
title: "square_fractions"
tags: []
signature: "pub fn square_fractions (rect_pos : (f64 , f64) , rect_size : (f64 , f64) , at : (f64 , f64)) -> (f32 , f32)"
---

Computes normalized saturation `s` and value `v` fractions for a point.

Computes normalized saturation `s` and value `v` fractions in `0.0..=1.0`
for point `at` within a rectangle at `rect_pos` with size `rect_size`.

Saturation `s` increases left-to-right from 0.0 to 1.0.
Value `v` is inverted: top is 1.0, bottom is 0.0.

# Examples

```rust
use crawrad_runtime_makepad::color_picker::square_fractions;

let (s, v) = square_fractions((10.0, 20.0), (100.0, 100.0), (60.0, 70.0));
assert!((s - 0.5).abs() < 1e-5);
assert!((v - 0.5).abs() < 1e-5);
```
