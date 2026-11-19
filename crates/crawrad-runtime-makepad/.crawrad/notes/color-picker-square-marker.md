---
id: color-picker-square-marker
kind: doc
target_file: src/color_picker.rs
line_hint: 356
author: 
created: 
title: "square_marker"
tags: []
signature: "pub fn square_marker (rect_pos : (f64 , f64) , rect_size : (f64 , f64) , s : f32 , v : f32) -> (f64 , f64)"
---

Computes the center position of the marker inside the saturation/value square.

Computes the absolute `(x, y)` center position of the marker inside
the saturation/value square at `rect_pos` with size `rect_size`.

# Examples

```rust
use crawrad_runtime_makepad::color_picker::square_marker;

let pos = square_marker((10.0, 20.0), (100.0, 100.0), 0.5, 0.5);
assert_eq!(pos, (60.0, 70.0));
```
