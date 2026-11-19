---
id: dialogs-dialog-manager
kind: doc
target_file: src/dialogs.rs
line_hint: 271
author: 
created: 
title: "dialog_manager"
tags: []
signature: "pub fn dialog_manager () -> & 'static Mutex < DialogManager >"
---

Returns the global thread-safe dialog manager.

# Examples

```rust
use crawrad_runtime::dialogs::dialog_manager;

let mgr = dialog_manager().lock().unwrap();
let _ = mgr.is_open();
```
