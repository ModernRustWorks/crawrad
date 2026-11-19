---
id: dialogs-handle-dialog-key
kind: doc
target_file: src/dialogs.rs
line_hint: 189
author: 
created: 
title: "handle_dialog_key"
tags: []
signature: "pub fn handle_dialog_key (cx : & mut Cx , view : & WidgetRef , event : & Event) -> bool"
---

Handles keyboard events (Enter to confirm, Escape to cancel).

# Examples

```rust,no_run
// Makepad Cx, WidgetRef and Event require the Makepad application runtime.
use makepad_widgets::{Cx, Event, WidgetRef};
use crawrad_runtime_makepad::dialogs::handle_dialog_key;

fn example(cx: &mut Cx, view: &WidgetRef, event: &Event) {
    let _ = handle_dialog_key(cx, view, event);
}
```
