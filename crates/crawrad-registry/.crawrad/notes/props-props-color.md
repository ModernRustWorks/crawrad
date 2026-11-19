---
id: props-props-color
kind: doc
target_file: src/props.rs
line_hint: 1049
author: 
created: 
title: "Props::color"
tags: []
signature: "pub fn color (default : crawrad_ir :: Color) -> PropDesc"
---

Background / foreground color (`Color`).

# Examples

```rust
use crawrad_ir::Color;
use crawrad_registry::Props;

let p = Props::color(Color::rgb(255, 255, 255));
assert_eq!(p.key, "color");
```
