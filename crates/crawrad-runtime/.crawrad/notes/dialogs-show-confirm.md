---
id: dialogs-show-confirm
kind: doc
target_file: src/dialogs.rs
line_hint: 333
author: 
created: 
title: "show_confirm"
tags: []
signature: "pub fn show_confirm (message : impl Into < String > , callback : impl FnOnce (ModalResult) + Send + 'static ,)"
---

Shows a confirmation dialog with Yes and No buttons.

Shows a confirmation dialog with Yes and No buttons, invoking `callback` with the user's choice.

# Examples

```rust
use crawrad_runtime::dialogs::{active_dialog, resolve_dialog, show_confirm, ModalResult};

show_confirm("Overwrite?", |_| {});
assert!(active_dialog().is_some());
resolve_dialog(ModalResult::No);
```

# Panics

Panics if the internal mutex is poisoned.
