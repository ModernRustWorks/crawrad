---
id: screenrect-right
kind: doc
target_file: src/lib.rs
line_hint: 78
author: 
created: 
title: "ScreenRect::right"
tags: []
signature: "pub fn right (& self) -> f64"
---

The right edge — `x + width`.

# Examples

```rust
use crawrad_os::ScreenRect;

let rect = ScreenRect { x: 10.0, y: 20.0, width: 100.0, height: 50.0 };
assert_eq!(rect.right(), 110.0);
```
