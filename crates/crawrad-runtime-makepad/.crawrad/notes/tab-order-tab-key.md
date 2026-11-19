---
id: tab-order-tab-key
kind: doc
target_file: src/tab_order.rs
line_hint: 17
author: 
created: 
title: "tab_key"
tags: []
signature: "pub fn tab_key (cx : & mut Cx , stops : & [TabStop] , event : & Event) -> bool"
---

Traverses focus to the next or previous Tab stop on a Tab key event.

Moves the focus when `event` is a Tab, and says so: the generated
`handle_event` then stops, so the control that just got the focus does
not take the key as well.

# Examples

```rust,no_run
// Makepad Cx and Event require the Makepad application runtime.
use makepad_widgets::{Cx, Event};
use crawrad_runtime_makepad::tab_order::tab_key;

fn handle_tab(cx: &mut Cx, event: &Event) {
    let _handled = tab_key(cx, &[], event);
}
```
