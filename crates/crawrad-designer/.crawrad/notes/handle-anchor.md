---
id: handle-anchor
kind: doc
target_file: src/lib.rs
line_hint: 95
author: 
created: 
title: "Handle::anchor"
tags: []
signature: "pub fn anchor (self , r : Rect) -> (f32 , f32)"
---

Handle center, for drawing and hit-testing.

# Examples

```rust
use crawrad_designer::Handle;
use crawrad_ir::Rect;

// The north-west handle sits just outside the corner.
let (x, y) = Handle::NW.anchor(Rect::new(0.0, 0.0, 100.0, 100.0));
assert!(x < 0.0 && y < 0.0);
assert_eq!(x, y);
```
