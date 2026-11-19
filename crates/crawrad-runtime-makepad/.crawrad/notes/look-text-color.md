---
id: look-text-color
kind: doc
target_file: src/look.rs
line_hint: 59
author: 
created: 
title: "text_color"
tags: []
signature: "pub fn text_color (cx : & mut Cx , widget : & WidgetRef , rgba : [u8 ; 4] , states : TextStates)"
---

The text of `widget` in `rgba` bytes; alpha 0 leaves the theme's.

# Examples

```rust,no_run
// Makepad Cx and WidgetRef require the Makepad application runtime.
use makepad_widgets::{Cx, WidgetRef};
use crawrad_runtime_makepad::look::{text_color, TextStates};

fn apply_color(cx: &mut Cx, widget: &WidgetRef) {
    text_color(cx, widget, [255, 255, 255, 255], TextStates::Plain);
}
```
