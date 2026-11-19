---
id: canvas-atlas-downscale-box-filter
kind: doc
target_file: src/canvas/atlas.rs
line_hint: 219
author: 
created: 
title: "downscale_box_filter"
tags: []
signature: "pub fn downscale_box_filter (src : & [u32] , src_w : usize , src_h : usize , dst_w : usize , dst_h : usize ,) -> Vec < u32 >"
---

Downscales a BGRA pixel buffer using a box filter.

# Examples

```rust
use crawrad_runtime_makepad::canvas::atlas::downscale_box_filter;

let pixels = vec![0xFFFFFFFF; 4];
let scaled = downscale_box_filter(&pixels, 2, 2, 1, 1);
assert_eq!(scaled.len(), 1);
```
