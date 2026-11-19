---
id: menu-below
kind: doc
target_file: src/menu.rs
line_hint: 65
author: 
created: 
title: "below"
tags: []
signature: "pub fn below (cx : & mut Cx , widget : & WidgetRef) -> DVec2"
---

Where a list opens under a widget of the bar: at its bottom-left corner.

# Examples

```rust,no_run
// Makepad Cx and WidgetRef require the Makepad application runtime.
use makepad_widgets::{Cx, WidgetRef};
use crawrad_runtime_makepad::menu::below;

fn get_pos(cx: &mut Cx, bar_btn: &WidgetRef) {
    let _pos = below(cx, bar_btn);
}
```
