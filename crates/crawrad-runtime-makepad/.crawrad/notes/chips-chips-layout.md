---
id: chips-chips-layout
kind: doc
target_file: src/chips.rs
line_hint: 47
author: 
created: 
title: "chips_layout"
tags: []
signature: "pub fn chips_layout (widths : & [f32] , area_width : f32 , chip_height : f32 , gap : f32 ,) -> Vec < (f32 , f32) >"
---

Pure function computing top-left `(x, y)` relative coordinates for each chip.

Chips are laid out left to right, wrapping to the next line when a chip
would exceed `area_width`.

# Examples

```rust
use crawrad_runtime_makepad::chips::chips_layout;

let widths = [40.0, 50.0];
let positions = chips_layout(&widths, 100.0, 24.0, 6.0);
assert_eq!(positions, vec![(0.0, 0.0), (46.0, 0.0)]);
```
