---
id: menu-covers
kind: doc
target_file: src/menu.rs
line_hint: 77
author: 
created: 
title: "covers"
tags: []
signature: "pub fn covers (cx : & mut Cx , widget : & WidgetRef , at : DVec2) -> bool"
---

Whether the shown `widget` covers the point `at`.

# Examples

```rust,no_run
// Makepad Cx and WidgetRef require the Makepad application runtime.
use makepad_widgets::{dvec2, Cx, WidgetRef};
use crawrad_runtime_makepad::menu::covers;

fn check_hit(cx: &mut Cx, menu: &WidgetRef) {
    let _hit = covers(cx, menu, dvec2(50.0, 50.0));
}
```
