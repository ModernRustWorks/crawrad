---
id: dialogs-resolve-dialog
kind: doc
target_file: src/dialogs.rs
line_hint: 286
author: 
created: 
title: "resolve_dialog"
tags: []
signature: "pub fn resolve_dialog (result : ModalResult) -> Option < ModalResult >"
---

Resolves the active dialog with a ModalResult.

# Examples

```rust
use crawrad_runtime::dialogs::{resolve_dialog, show_message, ModalResult};

show_message("Resolve me");
assert_eq!(resolve_dialog(ModalResult::Ok), Some(ModalResult::Ok));
```

# Panics

Panics if the internal mutex is poisoned.
