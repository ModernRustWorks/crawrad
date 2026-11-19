---
id: art-background
kind: doc
target_file: src/art.rs
line_hint: 59
author: 
created: 
title: "background"
tags: []
signature: "pub fn background (width : usize , height : usize) -> Vec < u32 >"
---

The picture, `width` × `height` pixels, row by row from the top.

# Examples

```rust,ignore
// private module art: not exposed in crate root
let pixels = background(10, 10);
assert_eq!(pixels.len(), 100);
```
