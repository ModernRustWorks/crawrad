---
id: props-props-image-index
kind: doc
target_file: src/props.rs
line_hint: 1024
author: 
created: 
title: "Props::image_index"
tags: []
signature: "pub fn image_index (default : i64) -> PropDesc"
---

Associated image index (`ImageIndex`).

# Examples

```rust
use crawrad_registry::Props;

let p = Props::image_index(0);
assert_eq!(p.key, "image_index");
```
