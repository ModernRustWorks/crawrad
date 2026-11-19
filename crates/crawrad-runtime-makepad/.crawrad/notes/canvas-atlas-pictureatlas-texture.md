---
id: canvas-atlas-pictureatlas-texture
kind: doc
target_file: src/canvas/atlas.rs
line_hint: 63
author: 
created: 
title: "PictureAtlas::texture"
tags: []
signature: "pub fn texture (& self) -> Option < & Texture >"
---

The Makepad texture allocated on the GPU, if any.

# Examples

```rust
use crawrad_runtime_makepad::canvas::atlas::PictureAtlas;

let atlas = PictureAtlas::new();
assert!(atlas.texture().is_none());
```
