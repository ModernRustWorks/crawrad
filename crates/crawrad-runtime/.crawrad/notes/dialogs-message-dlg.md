---
id: dialogs-message-dlg
kind: doc
target_file: src/dialogs.rs
line_hint: 359
author: 
created: 
title: "message_dlg"
tags: []
signature: "pub fn message_dlg (title : impl Into < String > , message : impl Into < String > , kind : DialogKind , buttons : DialogButtons , callback : Option < impl FnOnce (ModalResult) + Send + 'static > ,)"
---

Shows a fully configurable message dialog.

# Examples

```rust
use crawrad_runtime::dialogs::{
    active_dialog, message_dlg, resolve_dialog, DialogButtons, DialogKind, ModalResult,
};

message_dlg(
    "Question",
    "Continue?",
    DialogKind::Confirmation,
    DialogButtons::YesNo,
    None::<fn(ModalResult)>,
);
assert!(active_dialog().is_some());
resolve_dialog(ModalResult::Yes);
```

# Panics

Panics if the internal mutex is poisoned.
