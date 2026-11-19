---
id: form-formbase-close
kind: doc
target_file: src/form.rs
line_hint: 69
author: 
created: 
title: "FormBase::close"
tags: []
signature: "pub fn close (& mut self)"
---

Closes the form, defaulting to `ModalResult::Cancel` if none was set.

# Examples

```rust
use crawrad_component::pieces::ModalResult;
use crawrad_runtime::FormBase;

let mut form = FormBase::default();
form.close();
assert_eq!(form.modal_result, ModalResult::Cancel);
```
