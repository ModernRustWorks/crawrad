---
id: dialogs-show-warning
kind: doc
target_file: src/dialogs.rs
line_hint: 305
author: 
created: 
title: "show_warning"
tags: []
signature: "pub fn show_warning (message : impl Into < String >)"
---

Shows a warning message dialog with an OK button.

# Examples

```rust
use crawrad_runtime::dialogs::{active_dialog, resolve_dialog, show_warning, ModalResult};

show_warning("Low disk space.");
assert!(active_dialog().is_some());
resolve_dialog(ModalResult::Ok);
```

# Panics

Panics if the internal mutex is poisoned.
