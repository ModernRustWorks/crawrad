---
id: chips-chip-rects
kind: doc
target_file: src/chips.rs
line_hint: 82
author: 
created: 
title: "chip_rects"
tags: []
signature: "pub fn chip_rects (items : & [String] , removable : bool , font_size : f32 , area : Rect ,) -> Vec < (String , Rect , Option < Rect >) >"
---

Calculates rectangles for each chip fitting in `area` (#604).

Each chip that fits in `area` — its text, its rectangle and, when the chips
are removable, the rectangle of its `×` — in `area`'s coordinates (#604).
ONE answer for the click and for what a host reports to MCP (`view.get_dialog`),
so a test clicks exactly where the chip listens.

# Examples

```rust
use makepad_widgets::{dvec2, Rect};
use crawrad_runtime_makepad::chips::chip_rects;

let area = Rect { pos: dvec2(0.0, 0.0), size: dvec2(200.0, 100.0) };
let rects = chip_rects(&["Chip".to_string()], false, 9.0, area);
assert_eq!(rects.len(), 1);
```
