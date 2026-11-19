---
id: props-props-font-size
kind: doc
target_file: src/props.rs
line_hint: 1138
author: 
created: 
title: "Props::font_size"
tags: []
signature: "pub fn font_size (default : f64) -> PropDesc"
---

Font size in points (`FontSize`).

# Examples

```rust
use crawrad_registry::Props;

let p = Props::font_size(12.0);
assert_eq!(p.key, "font_size");
```
