---
id: dialogs-dialogmanager-post
kind: doc
target_file: src/dialogs.rs
line_hint: 216
author: 
created: 
title: "DialogManager::post"
tags: []
signature: "pub fn post (& mut self , mut req : DialogRequest) -> u64"
---

Enqueues a dialog request, activating it immediately if idle.

Enqueues a new dialog request. If no dialog is currently active, activates it immediately.

# Examples

```rust
use crawrad_runtime::dialogs::{DialogButtons, DialogKind, DialogManager, DialogRequest};

let mut mgr = DialogManager::default();
let req = DialogRequest {
    id: 0,
    title: "Hi".into(),
    message: "Body".into(),
    kind: DialogKind::Information,
    buttons: DialogButtons::Ok,
    callback: None,
};
let id = mgr.post(req);
assert_eq!(id, 1);
assert!(mgr.is_open());
```
