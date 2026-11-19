---
id: split-place
kind: doc
target_file: src/split.rs
line_hint: 9
author: 
created: 
title: "place"
tags: []
signature: "pub fn place (cx : & mut Cx , widget : & WidgetRef , x : f32 , y : f32 , w : f32 , h : f32)"
---

Puts `widget` at `(x, y)` in its parent with size `w` x `h` (points).

# Examples

```rust,no_run
// Makepad Cx and WidgetRef require the Makepad application runtime.
use makepad_widgets::{Cx, WidgetRef};
use crawrad_runtime_makepad::split::place;

fn move_widget(cx: &mut Cx, widget: &WidgetRef) {
    place(cx, widget, 10.0, 20.0, 100.0, 50.0);
}
```
