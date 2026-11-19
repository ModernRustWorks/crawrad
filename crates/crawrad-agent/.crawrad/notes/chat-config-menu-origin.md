---
id: chat-config-menu-origin
kind: doc
target_file: src/chat_config.rs
line_hint: 221
author: 
created: 
title: "menu_origin"
tags: []
signature: "pub fn menu_origin (button_x : f64 , panel : [f64 ; 4] , bar_y : f64 , menu_h : f64) -> (f64 , f64)"
---

Calculates the menu origin coordinates relative to the chat panel.

Where the menu goes, relative to the chat panel (`[x, y, w, h]`): anchored at
its button like the `+` menu, right above the prompt bar, and never past the
panel's right edge (the owner caught the effort menu cut off there).

# Examples

```rust
use crawrad_agent::chat_config::menu_origin;

let (x, y) = menu_origin(50.0, [0.0, 0.0, 400.0, 600.0], 550.0, 100.0);
assert!(x > 0.0 && y > 0.0);
```
