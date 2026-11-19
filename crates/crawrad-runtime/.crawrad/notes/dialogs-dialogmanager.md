---
id: dialogs-dialogmanager
kind: doc
target_file: src/dialogs.rs
line_hint: 207
author: 
created: 
title: "DialogManager"
tags: []
signature: "pub struct DialogManager"
---

Central state manager for message dialogs.

# Examples

```rust
use crawrad_runtime::dialogs::DialogManager;

let mgr = DialogManager::default();
assert!(!mgr.is_open());
```
