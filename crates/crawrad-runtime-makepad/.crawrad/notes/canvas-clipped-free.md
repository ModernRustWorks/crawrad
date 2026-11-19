---
id: canvas-clipped-free
kind: doc
target_file: src/canvas.rs
line_hint: 531
author: 
created: 
title: "Clipped::free"
tags: []
signature: "pub fn free (item : T) -> Self"
---

Nothing clips this.

# Examples

```rust
use crawrad_runtime_makepad::canvas::Clipped;

let c = Clipped::free(10);
assert_eq!(c.item, 10);
assert_eq!(c.clip, None);
```
