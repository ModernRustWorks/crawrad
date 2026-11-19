---
id: canvas-atlas-downscale-size
kind: doc
target_file: src/canvas/atlas.rs
line_hint: 200
author: 
created: 
title: "downscale_size"
tags: []
signature: "pub fn downscale_size (w : usize , h : usize , max_side : usize) -> (usize , usize)"
---

Computes target dimensions when scaling to fit `max_side`.

Computes the target dimensions when scaling to fit `max_side` on the longest edge.

# Examples

```rust
use crawrad_runtime_makepad::canvas::atlas::downscale_size;

assert_eq!(downscale_size(1024, 512, 512), (512, 256));
```
