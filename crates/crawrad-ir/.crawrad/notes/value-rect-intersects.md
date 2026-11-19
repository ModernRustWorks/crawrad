---
id: value-rect-intersects
kind: doc
target_file: src/value.rs
line_hint: 859
author: 
created: 
title: "Rect::intersects"
tags: []
signature: "pub fn intersects (& self , o : & Rect) -> bool"
---

Returns whether this rectangle intersects another rectangle.

Touches the other rectangle? The classic RAD IDE marquee selects by
intersection, not by containing entirely.

# Examples

```rust
use crawrad_ir::Rect;

let a = Rect::new(0.0, 0.0, 10.0, 10.0);
let b = Rect::new(5.0, 5.0, 10.0, 10.0);
assert!(a.intersects(&b));
```
