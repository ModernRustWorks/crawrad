---
id: props-props-rect
kind: doc
target_file: src/props.rs
line_hint: 118
author: 
created: 
title: "Props::rect"
tags: []
signature: "pub fn rect (default : Rect) -> PropDesc"
---

Position and size rectangle (`x, y, w, h`).

# Examples

```rust
use crawrad_ir::Rect;
use crawrad_registry::Props;

let p = Props::rect(Rect::new(0.0, 0.0, 100.0, 50.0));
assert_eq!(p.key, "rect");
```
