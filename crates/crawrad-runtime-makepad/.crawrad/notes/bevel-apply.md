---
id: bevel-apply
kind: doc
target_file: src/bevel.rs
line_hint: 23
author: 
created: 
title: "apply"
tags: []
signature: "pub fn apply (cx : & mut Cx , widget : & WidgetRef , shape : & str , style : & str)"
---

Applies the shape and the style (`Lowered` / `Raised`) to the Bevel widget.

Applies the shape and the style (`Lowered` / `Raised`) to the Bevel
`widget`.

# Examples

```rust,no_run
// Makepad Cx and WidgetRef require the Makepad application runtime.
use makepad_widgets::*;
use crawrad_runtime_makepad::bevel::apply;

fn example(cx: &mut Cx, widget: &WidgetRef) {
    apply(cx, widget, "Box", "Raised");
}
```
