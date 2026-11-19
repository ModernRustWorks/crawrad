---
id: dialogs-dialogmanager-is-open
kind: doc
target_file: src/dialogs.rs
line_hint: 257
author: 
created: 
title: "DialogManager::is_open"
tags: []
signature: "pub fn is_open (& self) -> bool"
---

Checks if a dialog is currently open.

# Examples

```rust
use crawrad_runtime::dialogs::DialogManager;

let mgr = DialogManager::default();
assert!(!mgr.is_open());
```
