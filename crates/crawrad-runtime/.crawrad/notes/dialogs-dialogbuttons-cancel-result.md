---
id: dialogs-dialogbuttons-cancel-result
kind: doc
target_file: src/dialogs.rs
line_hint: 124
author: 
created: 
title: "DialogButtons::cancel_result"
tags: []
signature: "pub fn cancel_result (& self) -> ModalResult"
---

Cancel response when pressing Escape key or closing.

# Examples

```rust
use crawrad_runtime::dialogs::{DialogButtons, ModalResult};

assert_eq!(DialogButtons::OkCancel.cancel_result(), ModalResult::Cancel);
```
