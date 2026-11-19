---
id: canvas-brush-mark
kind: doc
target_file: src/canvas/brush.rs
line_hint: 247
author: 
created: 
title: "Mark"
tags: []
signature: "pub struct Mark"
---

One thing the brush painted, as the [`Probe`] writes it down (#114).

# Examples

```rust
use makepad_widgets::{dvec2, Rect};
use crawrad_runtime_makepad::canvas::{Layer, Mark};

let m = Mark {
    layer: Layer::Box,
    rect: Rect { pos: dvec2(0.0, 0.0), size: dvec2(10.0, 10.0) },
    color: [1.0, 1.0, 1.0, 1.0],
    border: None,
    text: None,
    font_size: None,
};
assert_eq!(m.layer, Layer::Box);
```
