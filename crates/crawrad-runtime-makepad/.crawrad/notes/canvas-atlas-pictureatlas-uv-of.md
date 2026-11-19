---
id: canvas-atlas-pictureatlas-uv-of
kind: doc
target_file: src/canvas/atlas.rs
line_hint: 165
author: 
created: 
title: "PictureAtlas::uv_of"
tags: []
signature: "pub fn uv_of (& mut self , bytes : & [u8]) -> Option < [f32 ; 4] >"
---

Returns the cached or newly packed UV rectangle for `bytes`.

# Examples

```rust
use crawrad_runtime_makepad::canvas::atlas::PictureAtlas;

let mut atlas = PictureAtlas::new();
assert_eq!(atlas.uv_of(&[]), None);
```
