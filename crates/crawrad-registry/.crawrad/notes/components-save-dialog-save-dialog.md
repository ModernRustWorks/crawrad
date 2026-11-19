---
id: components-save-dialog-save-dialog
kind: doc
target_file: src/components/save_dialog.rs
line_hint: 5
author: 
created: 
title: "save_dialog"
tags: []
signature: "pub fn save_dialog () -> ComponentDesc"
---

`SaveDialog`: standard system dialog for choosing a file to save.

# Examples

```rust
use crawrad_registry::save_dialog;

let desc = save_dialog();
assert_eq!(desc.class, "SaveDialog");
```
