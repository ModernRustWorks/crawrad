---
id: components-open-dialog-open-dialog
kind: doc
target_file: src/components/open_dialog.rs
line_hint: 5
author: 
created: 
title: "open_dialog"
tags: []
signature: "pub fn open_dialog () -> ComponentDesc"
---

`OpenDialog`: standard system dialog for choosing a file to open.

# Examples

```rust
use crawrad_registry::open_dialog;

let desc = open_dialog();
assert_eq!(desc.class, "OpenDialog");
```
