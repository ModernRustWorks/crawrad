---
id: list-sync-scroll-bar
kind: doc
target_file: src/list.rs
line_hint: 249
author: 
created: 
title: "sync_scroll_bar"
tags: []
signature: "pub fn sync_scroll_bar (cx : & mut Cx , view : & mut View , bar_hidden : & mut Option < bool > , rows : (usize , f64 , f64) ,) -> f64"
---

Adjusts list scroll bar visibility depending on whether rows fit the view.

Hides the list's scroll bar when every row fits, shows it when they
overflow (#604) — only on a change; `rows` is (count, row height, list
height). Returns the width the bar takes from the rows' right end.

# Examples

```rust,no_run
// Makepad Cx and View require the Makepad application runtime.
use makepad_widgets::{Cx, View};
use crawrad_runtime_makepad::list::sync_scroll_bar;

fn update_bar(cx: &mut Cx, view: &mut View, bar_hidden: &mut Option<bool>) {
    let _w = sync_scroll_bar(cx, view, bar_hidden, (10, 20.0, 300.0));
}
```
