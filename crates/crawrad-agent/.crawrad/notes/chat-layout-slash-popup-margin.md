---
id: chat-layout-slash-popup-margin
kind: doc
target_file: src/chat_layout.rs
line_hint: 558
author: 
created: 
title: "slash_popup_margin"
tags: []
signature: "pub fn slash_popup_margin ((bar_x , bar_y) : (f64 , f64) , (panel_x , panel_y , panel_h) : (f64 , f64 , f64) , popup_h : f64 ,) -> (f64 , f64)"
---

Calculates the margin position of the slash popup menu.

The slash (/) popup's margin `(x, y)`, as [`context_popup_margin`] with no button.

# Examples

```rust
use crawrad_agent::chat_layout::slash_popup_margin;

let (x, y) = slash_popup_margin((0.0, 500.0), (0.0, 0.0, 600.0), 100.0);
assert!(x >= 8.0 && y >= 4.0);
```
