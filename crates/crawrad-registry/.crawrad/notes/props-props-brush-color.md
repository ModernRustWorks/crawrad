---
id: props-props-brush-color
kind: doc
target_file: src/props.rs
line_hint: 1509
author: 
created: 
title: "Props::brush_color"
tags: []
signature: "pub fn brush_color (default : crawrad_ir :: Color) -> PropDesc"
---

The color a Shape is filled with (`Brush.Color`, #460).

Outline pen width (`PenWidth`).
The colour a Shape is filled with (`Brush.Color`, #460).

# Examples

```rust
use crawrad_ir::Color;
use crawrad_registry::Props;

let p = Props::brush_color(Color::rgb(255, 255, 255));
assert_eq!(p.key, "brush_color");
```
