---
id: canvas-brush-brush-picture-size
kind: doc
target_file: src/canvas/brush.rs
line_hint: 647
author: 
created: 
title: "Brush::picture_size"
tags: []
signature: "pub fn picture_size (& mut self , bytes : & [u8]) -> Option < (usize , usize) >"
---

The original pixel size `(width, height)` of a decoded picture.

# Examples

```rust
use crawrad_runtime_makepad::canvas::{Brush, Probe};
use crawrad_runtime_makepad::Transformacao;

let mut probe = Probe::default();
let mut brush = Brush::probing(&mut probe, Transformacao::new(0.0, 0.0, 1.0));
assert_eq!(brush.picture_size(&[]), None);
```
