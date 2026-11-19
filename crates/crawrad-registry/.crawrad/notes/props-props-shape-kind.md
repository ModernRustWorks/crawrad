---
id: props-props-shape-kind
kind: doc
target_file: src/props.rs
line_hint: 1483
author: 
created: 
title: "Props::shape_kind"
tags: []
signature: "pub fn shape_kind (default : & 'static str) -> PropDesc"
---

Geometric shape of `Shape` component (`Shape`).

# Examples

```rust
use crawrad_registry::Props;

let p = Props::shape_kind("Rectangle");
assert_eq!(p.key, "shape");
```
