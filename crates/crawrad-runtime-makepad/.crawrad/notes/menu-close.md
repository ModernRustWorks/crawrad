---
id: menu-close
kind: doc
target_file: src/menu.rs
line_hint: 82
author: 
created: 
title: "close"
tags: []
signature: "pub fn close (cx : & mut Cx , list : & WidgetRef)"
---

Hides the `list`, when it is shown.

# Examples

```rust,no_run
// Makepad Cx and WidgetRef require the Makepad application runtime.
use makepad_widgets::{Cx, WidgetRef};
use crawrad_runtime_makepad::menu::close;

fn hide_menu(cx: &mut Cx, list: &WidgetRef) {
    close(cx, list);
}
```
