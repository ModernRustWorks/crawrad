---
id: props-props-images-data
kind: doc
target_file: src/props.rs
line_hint: 1009
author: 
created: 
title: "Props::images_data"
tags: []
signature: "pub fn images_data () -> PropDesc"
---

Raw image list data (`Images`).

# Examples

```rust
use crawrad_registry::Props;

let p = Props::images_data();
assert_eq!(p.key, "images_data");
```
