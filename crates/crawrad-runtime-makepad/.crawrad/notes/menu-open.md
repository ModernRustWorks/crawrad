---
id: menu-open
kind: doc
target_file: src/menu.rs
line_hint: 55
author: 
created: 
title: "open"
tags: []
signature: "pub fn open (cx : & mut Cx , list : & WidgetRef , body : & WidgetRef , at : DVec2)"
---

Shows and positions a popup menu view at `at` inside `body`.

Shows the `list` with its top-left corner at `at` (window coordinates),
placed in the `body` that holds it.

# Examples

```rust,no_run
// Makepad Cx and WidgetRef require the Makepad application runtime.
use makepad_widgets::{dvec2, Cx, WidgetRef};
use crawrad_runtime_makepad::menu::open;

fn show_menu(cx: &mut Cx, list: &WidgetRef, body: &WidgetRef) {
    open(cx, list, body, dvec2(100.0, 50.0));
}
```
