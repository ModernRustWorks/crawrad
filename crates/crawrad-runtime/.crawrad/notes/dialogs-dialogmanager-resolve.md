---
id: dialogs-dialogmanager-resolve
kind: doc
target_file: src/dialogs.rs
line_hint: 244
author: 
created: 
title: "DialogManager::resolve"
tags: []
signature: "pub fn resolve (& mut self , result : ModalResult) -> Option < ModalResult >"
---

Resolves the active dialog and advances to the next queued dialog.

Resolves the currently active dialog with the given ModalResult,
triggers any registered callback, and advances to the next queued dialog.

# Examples

```rust
use crawrad_runtime::dialogs::{
    DialogButtons, DialogKind, DialogManager, DialogRequest, ModalResult,
};

let mut mgr = DialogManager::default();
mgr.post(DialogRequest {
    id: 0,
    title: "T".into(),
    message: "M".into(),
    kind: DialogKind::Information,
    buttons: DialogButtons::Ok,
    callback: None,
});
assert_eq!(mgr.resolve(ModalResult::Ok), Some(ModalResult::Ok));
```
