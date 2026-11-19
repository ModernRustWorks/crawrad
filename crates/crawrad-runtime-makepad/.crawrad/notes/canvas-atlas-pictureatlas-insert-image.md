---
id: canvas-atlas-pictureatlas-insert-image
kind: doc
target_file: src/canvas/atlas.rs
line_hint: 87
author: 
created: 
title: "PictureAtlas::insert_image"
tags: []
signature: "pub fn insert_image (& mut self , print : u64 , orig_w : usize , orig_h : usize , data : & [u32] ,) -> [f32 ; 4]"
---

Inserts raw pixel data into the atlas under `print`.

# Panics

Panics if a downscaled picture cannot fit in an empty atlas.

# Examples

```rust
use crawrad_runtime_makepad::canvas::atlas::PictureAtlas;

let mut atlas = PictureAtlas::new();
let uv = atlas.insert_image(1, 2, 2, &[0xFFFFFFFF; 4]);
assert_eq!(uv[0], 0.0);
```
