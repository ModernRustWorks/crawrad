---
id: canvas-atlas-pictureatlas-clear
kind: doc
target_file: src/canvas/atlas.rs
line_hint: 53
author: 
created: 
title: "PictureAtlas::clear"
tags: []
signature: "pub fn clear (& mut self)"
---

Resets the atlas packer, buffer, and cached entries.

# Examples

```rust
use crawrad_runtime_makepad::canvas::atlas::PictureAtlas;

let mut atlas = PictureAtlas::new();
atlas.clear();
assert!(atlas.texture().is_none());
```
