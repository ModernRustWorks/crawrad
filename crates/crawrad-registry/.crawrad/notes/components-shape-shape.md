---
id: components-shape-shape
kind: doc
target_file: src/components/shape.rs
line_hint: 5
author: 
created: 
title: "shape"
tags: []
signature: "pub fn shape () -> ComponentDesc"
---

Vector geometric shape component such as rectangle or ellipse.

`Shape`: vector geometric graphic component (rectangle, circle, ellipse, rounded corners).

# Examples

```rust
use crawrad_registry::shape;

let desc = shape();
assert_eq!(desc.class, "Shape");
```
