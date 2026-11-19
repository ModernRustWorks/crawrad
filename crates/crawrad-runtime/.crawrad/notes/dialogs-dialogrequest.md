---
id: dialogs-dialogrequest
kind: doc
target_file: src/dialogs.rs
line_hint: 187
author: 
created: 
title: "DialogRequest"
tags: []
signature: "pub struct DialogRequest"
---

A request to show a message dialog.

# Examples

```rust
use crawrad_runtime::dialogs::{DialogButtons, DialogKind, DialogRequest};

let req = DialogRequest {
    id: 1,
    title: "Notice".to_string(),
    message: "Ready".to_string(),
    kind: DialogKind::Information,
    buttons: DialogButtons::Ok,
    callback: None,
};
assert_eq!(req.id, 1);
```
