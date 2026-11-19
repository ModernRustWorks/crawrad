---
id: bevel-shape-index
kind: doc
target_file: src/bevel.rs
line_hint: 10
author: 
created: 
title: "shape_index"
tags: []
signature: "pub fn shape_index (shape : & str) -> f64"
---

The shader's number for a classic `Bevel` shape (`Spacer` draws nothing).

# Examples

```rust
use crawrad_runtime_makepad::bevel::shape_index;

assert_eq!(shape_index("Box"), 0.0);
assert_eq!(shape_index("Frame"), 1.0);
assert_eq!(shape_index("Spacer"), 6.0);
```
