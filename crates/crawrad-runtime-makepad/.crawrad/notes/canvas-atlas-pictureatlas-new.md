---
id: canvas-atlas-pictureatlas-new
kind: doc
target_file: src/canvas/atlas.rs
line_hint: 48
author: 
created: 
title: "PictureAtlas::new"
tags: []
signature: "pub fn new () -> Self"
---

Creates an empty atlas.

# Examples

```rust
use crawrad_runtime_makepad::canvas::atlas::PictureAtlas;

let atlas = PictureAtlas::new();
assert!(atlas.texture().is_none());
```
