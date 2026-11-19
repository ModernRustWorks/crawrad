---
id: look-background
kind: doc
target_file: src/look.rs
line_hint: 34
author: 
created: 
title: "background"
tags: []
signature: "pub fn background (cx : & mut Cx , widget : & WidgetRef , rgba : [f32 ; 4] , show_bg : bool)"
---

Sets the background colour of `widget` in `rgba` (0..1).

The background of `widget` in `rgba` (0..1). Alpha 0 is `ParentColor`:
nothing of its own is painted, so what is behind shows through.
`show_bg`: the widget is a `View` that only paints a background when told,
and is told to stop again when the colour goes back to the parent's.

# Examples

```rust,no_run
// Makepad Cx and WidgetRef require the Makepad application runtime.
use makepad_widgets::{Cx, WidgetRef};
use crawrad_runtime_makepad::look::background;

fn apply_bg(cx: &mut Cx, widget: &WidgetRef) {
    background(cx, widget, [0.1, 0.2, 0.3, 1.0], true);
}
```
