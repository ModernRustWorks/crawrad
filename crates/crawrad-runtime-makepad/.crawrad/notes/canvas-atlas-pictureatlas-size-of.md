---
id: canvas-atlas-pictureatlas-size-of
kind: doc
target_file: src/canvas/atlas.rs
line_hint: 170
author: 
created: 
title: "PictureAtlas::size_of"
tags: []
signature: "pub fn size_of (& mut self , bytes : & [u8]) -> Option < (usize , usize) >"
---

Returns the original pixel size `(width, height)` of `bytes`.

# Examples

```rust
use crawrad_runtime_makepad::canvas::atlas::PictureAtlas;

let mut atlas = PictureAtlas::new();
assert_eq!(atlas.size_of(&[]), None);
```
