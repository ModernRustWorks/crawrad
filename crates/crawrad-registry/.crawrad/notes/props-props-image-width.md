---
id: props-props-image-width
kind: doc
target_file: src/props.rs
line_hint: 987
author: 
created: 
title: "Props::image_width"
tags: []
signature: "pub fn image_width (default : i64) -> PropDesc"
---

Width of icons in an image list (`Width`).

# Examples

```rust
use crawrad_registry::Props;

let p = Props::image_width(16);
assert_eq!(p.key, "image_width");
```
