---
id: check-grayed
kind: doc
target_file: src/check.rs
line_hint: 29
author: 
created: 
title: "grayed"
tags: []
signature: "pub fn grayed (cx : & mut Cx , widget : & WidgetRef , grayed : bool)"
---

Holds the tick of `widget` halfway while grayed.

Holds the tick of `widget` halfway while `grayed`; the animator owns it
again as soon as the state leaves cbGrayed.

# Examples

```rust,no_run
// Makepad Cx and WidgetRef require the Makepad application runtime.
use makepad_widgets::*;
use crawrad_runtime_makepad::check::grayed;

fn example(cx: &mut Cx, widget: &WidgetRef) {
    grayed(cx, widget, true);
}
```
