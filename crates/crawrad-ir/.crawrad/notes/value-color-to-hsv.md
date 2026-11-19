---
id: value-color-to-hsv
kind: doc
target_file: src/value.rs
line_hint: 596
author: 
created: 
title: "Color::to_hsv"
tags: []
signature: "pub fn to_hsv (self) -> (f32 , f32 , f32)"
---

Converts this color to HSV representation.

Converts this color to HSV representation: hue in degrees `0.0..360.0`,
saturation in `0.0..=1.0`, and value in `0.0..=1.0`. Grey (r == g == b)
has hue 0.0 and saturation 0.0.

# Examples

```rust
use crawrad_ir::Color;

let (h, s, v) = Color::rgb(255, 0, 0).to_hsv();
assert_eq!(h, 0.0);
assert_eq!(s, 1.0);
assert_eq!(v, 1.0);
```
