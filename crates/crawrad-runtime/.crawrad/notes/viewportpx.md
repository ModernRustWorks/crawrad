---
id: viewportpx
kind: doc
target_file: src/lib.rs
line_hint: 59
author: 
created: 
title: "ViewportPx"
tags: []
signature: "pub struct ViewportPx"
---

Rectangle in device pixels for the viewport.

Rectangle in device pixels — the only place in the system that speaks in
pixels. Above here, everything is in design units.

# Examples

```rust
use crawrad_runtime::ViewportPx;

let vp = ViewportPx { x: 0, y: 0, w: 800, h: 600 };
assert_eq!(vp.w, 800);
```
