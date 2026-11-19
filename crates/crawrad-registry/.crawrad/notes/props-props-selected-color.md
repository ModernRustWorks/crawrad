---
id: props-props-selected-color
kind: doc
target_file: src/props.rs
line_hint: 1060
author: 
created: 
title: "Props::selected_color"
tags: []
signature: "pub fn selected_color (default : crawrad_ir :: Color) -> PropDesc"
---

The colour a `ColorBox` has chosen (`Selected`, #500).

# Examples

```rust
use crawrad_ir::Color;
use crawrad_registry::Props;

let p = Props::selected_color(Color::rgb(0, 0, 0));
assert_eq!(p.key, "selected");
```
