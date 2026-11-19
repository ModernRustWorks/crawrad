---
id: shape-kind-index
kind: doc
target_file: src/shape.rs
line_hint: 10
author: 
created: 
title: "kind_index"
tags: []
signature: "pub fn kind_index (kind : & str) -> f64"
---

The shader's number for a classic `Shape` kind.

# Examples

```rust
use crawrad_runtime_makepad::shape::kind_index;

assert_eq!(kind_index("Square"), 1.0);
assert_eq!(kind_index("Circle"), 5.0);
assert_eq!(kind_index("Unknown"), 0.0);
```
