---
id: dialogs-dialogsnapshot
kind: doc
target_file: src/dialogs.rs
line_hint: 197
author: 
created: 
title: "DialogSnapshot"
tags: []
signature: "pub struct DialogSnapshot"
---

Snapshot of an active dialog request, safe for UI rendering loops.

# Examples

```rust
use crawrad_runtime::dialogs::{DialogButtons, DialogKind, DialogSnapshot};

let snap = DialogSnapshot {
    id: 1,
    title: "Alert".to_string(),
    message: "Body".to_string(),
    kind: DialogKind::Warning,
    buttons: DialogButtons::Ok,
};
assert_eq!(snap.title, "Alert");
```
