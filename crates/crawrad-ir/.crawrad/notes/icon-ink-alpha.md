---
id: icon-ink-alpha
kind: doc
target_file: src/icon.rs
line_hint: 170
author: 
created: 
title: "ink_alpha"
tags: []
signature: "pub fn ink_alpha (coverage : f32) -> f32"
---

The ink strength a rasterized icon pixel is painted with, from its coverage.

Doubled and capped at 1, in quarter steps: half a pixel of coverage is full
ink, so a thin stroke reads as strongly as the badge initials did.

# Examples

```rust
use crawrad_ir::icon::ink_alpha;

assert_eq!(ink_alpha(0.5), 1.0);
assert_eq!(ink_alpha(0.25), 0.5);
assert_eq!(ink_alpha(0.0), 0.0);
```
