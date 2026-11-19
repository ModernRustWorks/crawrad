---
id: tabs-focused
kind: doc
target_file: src/tabs.rs
line_hint: 28
author: 
created: 
title: "focused"
tags: []
signature: "pub fn focused (cx : & Cx , tab : & WidgetRef) -> bool"
---

Checks whether the `tab` button widget currently holds keyboard focus.

Whether the `tab` button holds the key focus (#139). Before the first
draw a widget has no area, and an empty area would match a keyboard that
holds no focus at all.

# Examples

```rust,no_run
// Makepad Cx and WidgetRef require the Makepad application runtime.
use makepad_widgets::{Cx, WidgetRef};
use crawrad_runtime_makepad::tabs::focused;

fn check_focus(cx: &Cx, tab: &WidgetRef) {
    let _f = focused(cx, tab);
}
```
