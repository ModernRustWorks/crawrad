---
id: dialogs-active-dialog
kind: doc
target_file: src/dialogs.rs
line_hint: 276
author: 
created: 
title: "active_dialog"
tags: []
signature: "pub fn active_dialog () -> Option < DialogSnapshot >"
---

Gets a snapshot of the active dialog, if one is currently open.

# Examples

```rust
use crawrad_runtime::dialogs::{active_dialog, resolve_dialog, show_message, ModalResult};

show_message("Status");
assert!(active_dialog().is_some());
resolve_dialog(ModalResult::Ok);
```

# Panics

Panics if the internal mutex is poisoned.
