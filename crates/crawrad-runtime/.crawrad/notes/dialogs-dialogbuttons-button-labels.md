---
id: dialogs-dialogbuttons-button-labels
kind: doc
target_file: src/dialogs.rs
line_hint: 102
author: 
created: 
title: "DialogButtons::button_labels"
tags: []
signature: "pub fn button_labels (& self) -> Vec < (& 'static str , ModalResult) >"
---

List of button text and corresponding ModalResult.

# Examples

```rust
use crawrad_runtime::dialogs::{DialogButtons, ModalResult};

let labels = DialogButtons::Ok.button_labels();
assert_eq!(labels, vec![("OK", ModalResult::Ok)]);
```
