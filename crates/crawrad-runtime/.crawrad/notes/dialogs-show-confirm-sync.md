---
id: dialogs-show-confirm-sync
kind: doc
target_file: src/dialogs.rs
line_hint: 350
author: 
created: 
title: "show_confirm_sync"
tags: []
signature: "pub fn show_confirm_sync (message : impl Into < String >) -> ModalResult"
---

Shows a confirmation dialog and blocks until the user responds.

Shows a confirmation dialog and blocks the current thread until the user responds.

NOTE: Do not call on a single-threaded UI rendering frame unless running on a worker thread.

# Examples

```rust,no_run
// show_confirm_sync blocks the current thread until the dialog is resolved.
use crawrad_runtime::dialogs::{show_confirm_sync, ModalResult};

let res = show_confirm_sync("Save changes?");
assert_eq!(res, ModalResult::Cancel);
```
