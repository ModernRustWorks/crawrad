---
id: crop-bgra
kind: doc
target_file: src/lib.rs
line_hint: 324
author: 
created: 
title: "crop_bgra"
tags: []
signature: "pub fn crop_bgra (src : & [u8] , src_width : u32 , src_height : u32 , region : [i32 ; 4]) -> Option < Capture >"
---

Crops a BGRA image buffer to a clamped rectangular region and converts to RGBA.

Coordinates below 0 are clamped to 0, and right/bottom beyond the buffer bounds
clamp to the image size. Returns `None` if the clamped region is empty or if
the source buffer is smaller than `src_width * src_height * 4`.

# Examples

```
// One BGRA pixel: blue 1, green 2, red 3.
let capture = crawrad_os::crop_bgra(&[1, 2, 3, 0], 1, 1, [0, 0, 1, 1]).unwrap();
assert_eq!(capture.rgba, vec![3, 2, 1, 255]);
```
