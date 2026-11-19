---
id: props-props-border-width
kind: doc
target_file: src/props.rs
line_hint: 827
author: 
created: 
title: "Props::border_width"
tags: []
signature: "pub fn border_width (default : i64) -> PropDesc"
---

Border / frame width of containers (`BorderWidth`).

# Examples

```rust
use crawrad_registry::Props;

let p = Props::border_width(1);
assert_eq!(p.key, "border_width");
```
