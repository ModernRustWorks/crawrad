---
id: components-bar-scrollbargeometry
kind: doc
target_file: src/components/bar.rs
line_hint: 52
author: 
created: 
title: "ScrollBarGeometry"
tags: []
signature: "pub struct ScrollBarGeometry"
---

A bar's geometry: the two buttons and the thumb, in screen points.

**It withstands any rectangle**, the empty one included: whatever is left
comes out as a zero-area rectangle, which [`Brush::rounded_box`] refuses to
draw. No panic and no made-up floor — the whole reason lives in `entre`
([`crate::components::comum::entre`]).

# Examples

```rust
use makepad_widgets::{dvec2, Rect};
use crawrad_runtime_makepad::components::bar::ScrollBarGeometry;

let geom = ScrollBarGeometry {
    start: Rect { pos: dvec2(0.0, 0.0), size: dvec2(10.0, 10.0) },
    end: Rect { pos: dvec2(0.0, 90.0), size: dvec2(10.0, 10.0) },
    thumb: Rect { pos: dvec2(2.0, 20.0), size: dvec2(6.0, 20.0) },
    horizontal: false,
};
assert!(!geom.horizontal);
```
