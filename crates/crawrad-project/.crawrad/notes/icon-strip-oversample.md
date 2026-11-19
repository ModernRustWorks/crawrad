---
id: icon-strip-oversample
kind: doc
target_file: src/icon_strip.rs
line_hint: 15
author: 
created: 
title: "OVERSAMPLE"
tags: []
signature: "pub const OVERSAMPLE : u32 = 4"
---

Samples per screen pixel, on each axis — see `rasterize_oversampled`.

# Examples

```rust
use crawrad_project::icon_strip::OVERSAMPLE;

assert_eq!(OVERSAMPLE, 4);
```
