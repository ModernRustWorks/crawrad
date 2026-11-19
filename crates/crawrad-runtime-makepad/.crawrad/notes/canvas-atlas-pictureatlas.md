---
id: canvas-atlas-pictureatlas
kind: doc
target_file: src/canvas/atlas.rs
line_hint: 33
author: 
created: 
title: "PictureAtlas"
tags: []
signature: "pub struct PictureAtlas"
---

The picture atlas holding a CPU pixel buffer and a GPU texture.

# Examples

```rust
use crawrad_runtime_makepad::canvas::atlas::PictureAtlas;

let atlas = PictureAtlas::new();
assert!(atlas.texture().is_none());
```
