---
id: dialogs-dialogmanager-active
kind: doc
target_file: src/dialogs.rs
line_hint: 228
author: 
created: 
title: "DialogManager::active"
tags: []
signature: "pub fn active (& self) -> Option < & DialogRequest >"
---

Returns the currently active dialog request, if any.

# Examples

```rust
use crawrad_runtime::dialogs::DialogManager;

let mgr = DialogManager::default();
assert!(mgr.active().is_none());
```
