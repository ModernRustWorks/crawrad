---
id: dialogs-is-dialog-open
kind: doc
target_file: src/dialogs.rs
line_hint: 281
author: 
created: 
title: "is_dialog_open"
tags: []
signature: "pub fn is_dialog_open () -> bool"
---

Checks whether a dialog is currently open.

# Examples

```rust
use crawrad_runtime::dialogs::{is_dialog_open, resolve_dialog, show_message, ModalResult};

show_message("Active");
assert!(is_dialog_open());
resolve_dialog(ModalResult::Ok);
```

# Panics

Panics if the internal mutex is poisoned.
