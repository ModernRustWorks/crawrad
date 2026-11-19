---
id: components-paint-box-paint-box
kind: doc
target_file: src/components/paint_box.rs
line_hint: 4
author: 
created: 
title: "paint_box"
tags: []
signature: "pub fn paint_box () -> ComponentDesc"
---

`PaintBox`: custom drawing area for arbitrary user routines via `OnPaint` event.

# Examples

```rust
use crawrad_registry::paint_box;

let desc = paint_box();
assert_eq!(desc.class, "PaintBox");
```
