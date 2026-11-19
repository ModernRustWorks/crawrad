---
id: dialogs-dialogmanager-snapshot
kind: doc
target_file: src/dialogs.rs
line_hint: 233
author: 
created: 
title: "DialogManager::snapshot"
tags: []
signature: "pub fn snapshot (& self) -> Option < DialogSnapshot >"
---

Returns a lightweight snapshot of the active dialog for renderers.

# Examples

```rust
use crawrad_runtime::dialogs::DialogManager;

let mgr = DialogManager::default();
assert!(mgr.snapshot().is_none());
```
