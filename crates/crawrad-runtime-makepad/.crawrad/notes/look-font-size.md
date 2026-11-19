---
id: look-font-size
kind: doc
target_file: src/look.rs
line_hint: 26
author: 
created: 
title: "font_size"
tags: []
signature: "pub fn font_size (cx : & mut Cx , widget : & WidgetRef , size : f32)"
---

The text of `widget` at `size` points; 0 leaves the theme's.

# Examples

```rust,no_run
// Makepad Cx and WidgetRef require the Makepad application runtime.
use makepad_widgets::{Cx, WidgetRef};
use crawrad_runtime_makepad::look::font_size;

fn apply_size(cx: &mut Cx, widget: &WidgetRef) {
    font_size(cx, widget, 14.0);
}
```
