---
id: dialogs-pendinganswer-open
kind: doc
target_file: src/dialogs.rs
line_hint: 160
author: 
created: 
title: "PendingAnswer::open"
tags: []
signature: "pub fn open (& self , title : & str , message : & str , kind : DialogKind , buttons : DialogButtons)"
---

Opens the dialog; the answer lands in this handle.

# Examples

```rust
use crawrad_runtime::dialogs::{
    DialogButtons, DialogKind, ModalResult, PendingAnswer, resolve_dialog,
};

let pending = PendingAnswer::default();
pending.open("Test", "Prompt", DialogKind::Confirmation, DialogButtons::YesNo);
resolve_dialog(ModalResult::Yes);
assert_eq!(pending.take(), Some(ModalResult::Yes));
```
