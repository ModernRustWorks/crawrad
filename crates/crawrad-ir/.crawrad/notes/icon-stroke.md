---
id: icon-stroke
kind: doc
target_file: src/icon.rs
line_hint: 11
author: 
created: 
title: "STROKE"
tags: []
signature: "pub const STROKE : f32 = 1.25"
---

The stroke width of every class icon, in grid units.

The palette strip and the non-visual badge both rasterize with half of it, so
the two draw the same ink.

# Examples

```rust
use crawrad_ir::icon::STROKE;

assert_eq!(STROKE, 1.25);
```
