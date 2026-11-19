---
id: drawing-axis
kind: doc
target_file: src/drawing.rs
line_hint: 137
author: 
created: 
title: "Axis"
tags: []
signature: "pub enum Axis"
---

The axis along which an [`Area::Fraction`] grows.

# Examples

```rust
use crawrad_ir::drawing::Axis;

assert_eq!(Axis::X.name(), "x");
assert_eq!(Axis::from_name("y"), Some(Axis::Y));
```
