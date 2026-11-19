---
id: picture-show
kind: doc
target_file: src/picture.rs
line_hint: 52
author: 
created: 
title: "show"
tags: []
signature: "pub fn show (cx : & mut Cx , ui : & WidgetRef , path : & [LiveId] , bytes : & [u8] , shown : & mut Shown)"
---

Loads and displays image bytes at `path`, skipping if already showing them.

Shows `bytes` in the `img` of Image `path`, unless that very picture is on
screen already.

# Examples

```rust,no_run
// Makepad Cx and WidgetRef require the Makepad application runtime.
use makepad_widgets::{live_id, Cx, LiveId, WidgetRef};
use crawrad_runtime_makepad::picture::{show, Shown};

fn update_image(cx: &mut Cx, ui: &WidgetRef, shown: &mut Shown) {
    show(cx, ui, &[live_id!(img_logo)], &[], shown);
}
```
