---
id: components-comum-rgb
kind: doc
target_file: src/components/comum.rs
line_hint: 30
author: 
created: 
title: "rgb"
tags: []
signature: "pub const fn rgb (r : u8 , g : u8 , b : u8) -> [f32 ; 4]"
---

Converts an opaque 8-bit-per-channel color to the shader's `[f32; 4]`.

It exists so that a token can be written with the SAME number that RXP
writes (`crawrad-runtime-egui/src/lib.rs:182-185` uses `Color32::from_rgb`) —
comparing `0xD0` with `0.816` by hand invites silent divergence.

# Examples

```rust
use crawrad_runtime_makepad::components::comum::rgb;

let c = rgb(255, 0, 0);
assert_eq!(c, [1.0, 0.0, 0.0, 1.0]);
```
