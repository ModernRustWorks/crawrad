---
id: dialogs-sync-message-dialog
kind: doc
target_file: src/dialogs.rs
line_hint: 122
author: 
created: 
title: "sync_message_dialog"
tags: []
signature: "pub fn sync_message_dialog (cx : & mut Cx , view : & WidgetRef)"
---

Helper to sync the modal message dialog state to a Makepad widget view.

# Examples

```rust,no_run
// Makepad Cx and WidgetRef require the Makepad application runtime.
use makepad_widgets::{Cx, WidgetRef};
use crawrad_runtime_makepad::dialogs::sync_message_dialog;

fn example(cx: &mut Cx, view: &WidgetRef) {
    sync_message_dialog(cx, view);
}
```
