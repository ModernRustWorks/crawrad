---
id: dialogs-dialogmanager-clear
kind: doc
target_file: src/dialogs.rs
line_hint: 262
author: 
created: 
title: "DialogManager::clear"
tags: []
signature: "pub fn clear (& mut self)"
---

Clears all dialogs and pending queue.

# Examples

```rust
use crawrad_runtime::dialogs::DialogManager;

let mut mgr = DialogManager::default();
mgr.clear();
assert!(!mgr.is_open());
```
