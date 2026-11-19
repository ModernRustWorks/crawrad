---
id: screenrect-bottom
kind: doc
target_file: src/lib.rs
line_hint: 83
author: 
created: 
title: "ScreenRect::bottom"
tags: []
signature: "pub fn bottom (& self) -> f64"
---

The bottom edge — `y + height`.

# Examples

```rust
use crawrad_os::ScreenRect;

let rect = ScreenRect { x: 10.0, y: 20.0, width: 100.0, height: 50.0 };
assert_eq!(rect.bottom(), 70.0);
```
