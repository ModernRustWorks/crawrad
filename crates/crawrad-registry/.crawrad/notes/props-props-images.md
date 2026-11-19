---
id: props-props-images
kind: doc
target_file: src/props.rs
line_hint: 505
author: 
created: 
title: "Props::images"
tags: []
signature: "pub fn images () -> PropDesc"
---

Associated image list (`Images`).

# Examples

```rust
use crawrad_registry::Props;

let p = Props::images();
assert_eq!(p.key, "images");
```
