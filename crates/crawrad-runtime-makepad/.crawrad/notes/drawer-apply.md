---
id: drawer-apply
kind: doc
target_file: src/drawer.rs
line_hint: 97
author: 
created: 
title: "apply"
tags: []
signature: "pub fn apply (cx : & mut Cx , drawer : & WidgetRef , side : & str)"
---

Applies shadow offset and background to the Drawer widget according to `side`.

# Examples

```rust,no_run
// Makepad Cx and WidgetRef require the Makepad application runtime.
use makepad_widgets::{Cx, WidgetRef};
use crawrad_runtime_makepad::drawer::apply;

fn example(cx: &mut Cx, drawer: &WidgetRef) {
    apply(cx, drawer, "dsLeft");
}
```
