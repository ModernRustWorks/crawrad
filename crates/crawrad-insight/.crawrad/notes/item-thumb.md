---
id: item-thumb
kind: doc
target_file: src/item.rs
line_hint: 154
author: 
created: 
title: "thumb"
tags: []
signature: "pub fn thumb (total : usize , rows : usize , top : usize , row_h : f64) -> (f64 , f64)"
---

Calculates scrollbar thumb height and offset for list scrolling.

The scrollbar thumb of a list of `total` items showing `rows` of
`row_h` from `top`: its height and its offset; none when all fit.

# Examples

```rust
use crawrad_insight::item::thumb;

let (height, offset) = thumb(10, 5, 0, 20.0);
assert!(height > 0.0);
assert_eq!(offset, 0.0);
```
