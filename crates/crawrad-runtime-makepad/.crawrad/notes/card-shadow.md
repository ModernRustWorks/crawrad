---
id: card-shadow
kind: doc
target_file: src/card.rs
line_hint: 133
author: 
created: 
title: "shadow"
tags: []
signature: "pub fn shadow (elevation : i64) -> (f32 , f64)"
---

The shadow of an elevation 0..=3: `(alpha, offset)`; 0 draws none.

# Examples

```rust
use crawrad_runtime_makepad::card::shadow;

assert_eq!(shadow(0), (0.0, 0.0));
let (alpha, offset) = shadow(1);
assert!((alpha - 0.2).abs() < 1e-6);
assert_eq!(offset, 2.0);
```
