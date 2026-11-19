---
id: card-header-height
kind: doc
target_file: src/card.rs
line_hint: 128
author: 
created: 
title: "header_height"
tags: []
signature: "pub fn header_height (subtitle : & str) -> f64"
---

Calculates the header's height based on whether a subtitle is present.

The header's height: 42 with a subtitle, 28 without — the same as the egui
drawing (`paint_card`), so the children sit below it on both targets.

# Examples

```rust
use crawrad_runtime_makepad::card::header_height;

assert_eq!(header_height(""), 28.0);
assert_eq!(header_height("Subtitle"), 42.0);
```
