---
id: history-panel-origin-badge-colors
kind: doc
target_file: src/history_panel.rs
line_hint: 348
author: 
created: 
title: "origin_badge_colors"
tags: []
signature: "pub fn origin_badge_colors (origin : & str) -> ([f32 ; 4] , [f32 ; 4])"
---

Returns (text_col, bg_col) rgba for an origin badge.

# Examples

```rust
use crawrad_runtime_makepad::history_panel::origin_badge_colors;

let (text, bg) = origin_badge_colors("external");
assert_eq!(text[3], 1.0);
assert_eq!(bg[3], 1.0);
```
