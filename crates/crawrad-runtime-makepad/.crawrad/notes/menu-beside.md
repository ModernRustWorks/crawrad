---
id: menu-beside
kind: doc
target_file: src/menu.rs
line_hint: 71
author: 
created: 
title: "beside"
tags: []
signature: "pub fn beside (cx : & mut Cx , widget : & WidgetRef) -> DVec2"
---

Computes position for a sub-menu beside its parent menu item.

Where a sub-menu opens beside the item that holds it: at its top-right
corner.

# Examples

```rust,no_run
// Makepad Cx and WidgetRef require the Makepad application runtime.
use makepad_widgets::{Cx, WidgetRef};
use crawrad_runtime_makepad::menu::beside;

fn get_pos(cx: &mut Cx, item: &WidgetRef) {
    let _pos = beside(cx, item);
}
```
