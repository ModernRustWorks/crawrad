---
id: canvas-atlas-max-picture-side
kind: doc
target_file: src/canvas/atlas.rs
line_hint: 19
author: 
created: 
title: "MAX_PICTURE_SIDE"
tags: []
signature: "pub const MAX_PICTURE_SIDE : usize = 512"
---

Longest side allowed for a picture in the atlas before downscaling.

# Examples

```rust
use crawrad_runtime_makepad::canvas::atlas::MAX_PICTURE_SIDE;

assert_eq!(MAX_PICTURE_SIDE, 512);
```
