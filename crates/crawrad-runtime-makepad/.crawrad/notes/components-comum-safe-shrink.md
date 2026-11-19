---
id: components-comum-safe-shrink
kind: doc
target_file: src/components/comum.rs
line_hint: 148
author: 
created: 
title: "safe_shrink"
tags: []
signature: "pub fn safe_shrink (r : Rect , d : f32) -> Rect"
---

Shrinks a rectangle safely without inverting it.

Shrinks a rectangle safely: if size is smaller than 2 * d, returns original rectangle.

# Examples

```rust
use crawrad_ir::Rect;
use crawrad_runtime_makepad::components::comum::safe_shrink;

let r = Rect::new(0.0, 0.0, 10.0, 10.0);
let shrunk = safe_shrink(r, 2.0);
assert_eq!(shrunk.w, 6.0);
```
