---
id: props-props-pen-color
kind: doc
target_file: src/props.rs
line_hint: 1520
author: 
created: 
title: "Props::pen_color"
tags: []
signature: "pub fn pen_color (default : crawrad_ir :: Color) -> PropDesc"
---

The colour of a Shape's outline (`Pen.Color`, #460).

# Examples

```rust
use crawrad_ir::Color;
use crawrad_registry::Props;

let p = Props::pen_color(Color::rgb(0, 0, 0));
assert_eq!(p.key, "pen_color");
```
