---
id: dialogs-show-error
kind: doc
target_file: src/dialogs.rs
line_hint: 319
author: 
created: 
title: "show_error"
tags: []
signature: "pub fn show_error (message : impl Into < String >)"
---

Shows an error message dialog with an OK button.

# Examples

```rust
use crawrad_runtime::dialogs::{active_dialog, resolve_dialog, show_error, ModalResult};

show_error("File not found.");
assert!(active_dialog().is_some());
resolve_dialog(ModalResult::Ok);
```

# Panics

Panics if the internal mutex is poisoned.
