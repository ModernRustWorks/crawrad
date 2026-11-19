---
id: props-props-font
kind: doc
target_file: src/props.rs
line_hint: 587
author: 
created: 
title: "Props::font"
tags: []
signature: "pub fn font (default : impl Into < crawrad_ir :: Font >) -> PropDesc"
---

Typography (`Font`: family, size, style).

# Examples

```rust
use crawrad_ir::Font;
use crawrad_registry::Props;

let p = Props::font(Font::default());
assert_eq!(p.key, "font");
```
