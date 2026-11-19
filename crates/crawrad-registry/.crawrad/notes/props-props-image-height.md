---
id: props-props-image-height
kind: doc
target_file: src/props.rs
line_hint: 998
author: 
created: 
title: "Props::image_height"
tags: []
signature: "pub fn image_height (default : i64) -> PropDesc"
---

Height of icons in an image list (`Height`).

# Examples

```rust
use crawrad_registry::Props;

let p = Props::image_height(16);
assert_eq!(p.key, "image_height");
```
