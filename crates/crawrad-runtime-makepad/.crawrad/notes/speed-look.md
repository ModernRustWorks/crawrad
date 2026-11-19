---
id: speed-look
kind: doc
target_file: src/speed.rs
line_hint: 43
author: 
created: 
title: "look"
tags: []
signature: "pub fn look (cx : & mut Cx , widget : & WidgetRef , down : bool , flat : bool)"
---

Paints the SpeedButton `widget` down or up, flat or framed.

# Examples

```rust,no_run
// Makepad Cx and WidgetRef require the Makepad application runtime.
use makepad_widgets::{Cx, WidgetRef};
use crawrad_runtime_makepad::speed::look;

fn update_speed(cx: &mut Cx, widget: &WidgetRef) {
    look(cx, widget, false, true);
}
```
