---
id: value-rect
kind: doc
target_file: src/value.rs
line_hint: 831
author: 
created: 
title: "Rect"
tags: []
signature: "pub struct Rect"
---

Rectangle in **design units** (not in device pixels).

Whoever converts to pixels is the runtime, with the window scale factor.

# Examples

```rust
use crawrad_ir::Rect;

let r = Rect::new(10.0, 20.0, 100.0, 50.0);
assert_eq!(r.x, 10.0);
assert_eq!(r.w, 100.0);
```
