---
id: shape-apply
kind: doc
target_file: src/shape.rs
line_hint: 22
author: 
created: 
title: "apply"
tags: []
signature: "pub fn apply (cx : & mut Cx , widget : & WidgetRef , kind : & str , pen_width : i64 , brush : [f32 ; 4] , pen : [f32 ; 4] ,)"
---

Applies kind, pen width, and brush/pen colors to a Shape widget.

Applies the kind, pen width (points) and the brush and pen colours
(`0..1` RGBA) to the Shape `widget`.

# Examples

```rust,no_run
// Makepad Cx and WidgetRef require the Makepad application runtime.
use makepad_widgets::{Cx, WidgetRef};
use crawrad_runtime_makepad::shape::apply;

fn update_shape(cx: &mut Cx, widget: &WidgetRef) {
    apply(cx, widget, "Circle", 2, [1.0, 0.0, 0.0, 1.0], [0.0, 0.0, 0.0, 1.0]);
}
```
