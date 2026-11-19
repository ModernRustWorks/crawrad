---
id: props-props-current-color
kind: doc
target_file: src/props.rs
line_hint: 2139
author: 
created: 
title: "Props::current_color"
tags: []
signature: "pub fn current_color (default : crawrad_ir :: Color) -> PropDesc"
---

Background color of current breadcrumb segment (`CurrentColor`, #500).

Background color of the current chevron segment in a `Breadcrumb` (`CurrentColor`, #500).

# Examples

```rust
use crawrad_ir::Color;
use crawrad_registry::Props;

let p = Props::current_color(Color::rgb(255, 255, 255));
assert_eq!(p.key, "current_color");
```
