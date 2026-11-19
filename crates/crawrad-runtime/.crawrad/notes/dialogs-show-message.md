---
id: dialogs-show-message
kind: doc
target_file: src/dialogs.rs
line_hint: 291
author: 
created: 
title: "show_message"
tags: []
signature: "pub fn show_message (message : impl Into < String >)"
---

Shows an informative message dialog with an OK button.

# Examples

```rust
use crawrad_runtime::dialogs::{active_dialog, resolve_dialog, show_message, ModalResult};

show_message("Saved successfully.");
assert!(active_dialog().is_some());
resolve_dialog(ModalResult::Ok);
```

# Panics

Panics if the internal mutex is poisoned.
