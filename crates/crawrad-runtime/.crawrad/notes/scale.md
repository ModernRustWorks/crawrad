---
id: scale
kind: doc
target_file: src/lib.rs
line_hint: 68
author: 
created: 
title: "Scale"
tags: []
signature: "pub struct Scale (pub f32)"
---

Conversion factor between design unit and device pixel.

# Examples

```rust
use crawrad_runtime::Scale;

let scale = Scale(2.0);
assert_eq!(scale.to_px(10.0), 20);
assert_eq!(scale.to_design(20), 10.0);
```
