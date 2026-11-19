---
id: canvas-brush-rgba
kind: doc
target_file: src/canvas/brush.rs
line_hint: 142
author: 
created: 
title: "rgba"
tags: []
signature: "pub fn rgba (c : [f32 ; 4]) -> Vec4"
---

Turns a token colour (`componentes::comum`) into the `Vec4` the shader wants.

**A token colour, never a literal**: the whole palette lives in a single
file, and the shell has a test that rejects a hand-written colour.

# Examples

```rust
use makepad_widgets::Vec4;
use crawrad_runtime_makepad::canvas::rgba;

let v = rgba([1.0, 0.0, 0.0, 1.0]);
assert_eq!(v, Vec4 { x: 1.0, y: 0.0, z: 0.0, w: 1.0 });
```
