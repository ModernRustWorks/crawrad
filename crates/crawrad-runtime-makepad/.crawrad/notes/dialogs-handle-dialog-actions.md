---
id: dialogs-handle-dialog-actions
kind: doc
target_file: src/dialogs.rs
line_hint: 159
author: 
created: 
title: "handle_dialog_actions"
tags: []
signature: "pub fn handle_dialog_actions (cx : & mut Cx , view : & WidgetRef , actions : & Actions) -> bool"
---

Handles actions / user interactions on the message dialog view.

# Examples

```rust,no_run
// Makepad Cx, WidgetRef and Actions require the Makepad application runtime.
use makepad_widgets::{Actions, Cx, WidgetRef};
use crawrad_runtime_makepad::dialogs::handle_dialog_actions;

fn example(cx: &mut Cx, view: &WidgetRef, actions: &Actions) {
    let _ = handle_dialog_actions(cx, view, actions);
}
```
