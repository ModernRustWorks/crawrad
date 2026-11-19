---
id: components-message-dialog-message-dialog
kind: doc
target_file: src/components/message_dialog.rs
line_hint: 5
author: 
created: 
title: "message_dialog"
tags: []
signature: "pub fn message_dialog () -> ComponentDesc"
---

`MessageDialog`: native modal message dialog component.

# Examples

```rust
use crawrad_registry::message_dialog;

let desc = message_dialog();
assert_eq!(desc.class, "MessageDialog");
```
