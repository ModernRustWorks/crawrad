---
id: chips-chip-width
kind: doc
target_file: src/chips.rs
line_hint: 68
author: 
created: 
title: "chip_width"
tags: []
signature: "pub fn chip_width (text : & str , removable : bool , font_size : f32) -> f32"
---

Approximate chip width based on text length and optional remove button.

# Examples

```rust
use crawrad_runtime_makepad::chips::chip_width;

let w = chip_width("Tag", false, 9.0);
assert!(w > 0.0);
```
