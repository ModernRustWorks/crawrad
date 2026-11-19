---
id: value-color-from-hsv
kind: doc
target_file: src/value.rs
line_hint: 624
author: 
created: 
title: "Color::from_hsv"
tags: []
signature: "pub fn from_hsv (h : f32 , s : f32 , v : f32 , a : u8) -> Self"
---

Creates a color from HSV components.

Creates a color from HSV components: `h` in degrees (wraps modulo 360),
`s` and `v` clamped to `0.0..=1.0`, with the given alpha channel `a`.
Channels round to the nearest `u8`.

# Examples

```rust
use crawrad_ir::Color;

let color = Color::from_hsv(0.0, 1.0, 1.0, 255);
assert_eq!(color, Color::rgb(255, 0, 0));
```
