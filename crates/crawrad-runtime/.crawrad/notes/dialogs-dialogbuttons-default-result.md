---
id: dialogs-dialogbuttons-default-result
kind: doc
target_file: src/dialogs.rs
line_hint: 116
author: 
created: 
title: "DialogButtons::default_result"
tags: []
signature: "pub fn default_result (& self) -> ModalResult"
---

Default response when confirming with Enter key.

# Examples

```rust
use crawrad_runtime::dialogs::{DialogButtons, ModalResult};

assert_eq!(DialogButtons::OkCancel.default_result(), ModalResult::Ok);
```
