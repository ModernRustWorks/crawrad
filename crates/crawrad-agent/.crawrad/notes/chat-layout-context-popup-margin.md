---
id: chat-layout-context-popup-margin
kind: doc
target_file: src/chat_layout.rs
line_hint: 531
author: 
created: 
title: "context_popup_margin"
tags: []
signature: "pub fn context_popup_margin ((btn_x , btn_y) : (f64 , f64) , (bar_x , bar_y) : (f64 , f64) , (panel_x , panel_y , panel_h) : (f64 , f64 , f64) , popup_h : f64 ,) -> (f64 , f64)"
---

Calculates the margin position of the context popup menu.

The context (+) popup's margin `(x, y)` inside the chat panel, above the input
bar. Points are `(x, y)`; `panel` is `(x, y, height)`; unsized rects read 0.

# Examples

```rust
use crawrad_agent::chat_layout::context_popup_margin;

let (x, y) = context_popup_margin((10.0, 10.0), (0.0, 500.0), (0.0, 0.0, 600.0), 100.0);
assert!(x >= 8.0 && y >= 4.0);
```
