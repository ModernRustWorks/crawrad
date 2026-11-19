---
id: form-formbase-close-with
kind: doc
target_file: src/form.rs
line_hint: 76
author: 
created: 
title: "FormBase::close_with"
tags: []
signature: "pub fn close_with (& mut self , result : ModalResult)"
---

Closes the form setting an explicit `ModalResult`.

# Examples

```rust
use crawrad_component::pieces::ModalResult;
use crawrad_runtime::FormBase;

let mut form = FormBase::default();
form.close_with(ModalResult::Ok);
assert_eq!(form.modal_result, ModalResult::Ok);
```
